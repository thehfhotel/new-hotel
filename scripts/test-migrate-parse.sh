#!/usr/bin/env bash
# =============================================================================
# Unit tests for migrate.sh's filename parser + applied-set membership check.
# =============================================================================
# These tests do NOT spin up PostgreSQL — they exercise the pure-bash
# regex + associative-array logic that decides which migration files are
# pending. The full migrate.sh against a real PG is verified end-to-end
# by every successful CI deploy.
#
# Usage: ./scripts/test-migrate-parse.sh
# Exits 0 on success, 1 on any failed assertion.
# =============================================================================

set -euo pipefail

PASS=0
FAIL=0

assert_eq() {
    local label="$1"
    local expected="$2"
    local actual="$3"
    if [ "$expected" = "$actual" ]; then
        echo "  PASS: $label"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $label"
        echo "        expected: '$expected'"
        echo "        actual:   '$actual'"
        FAIL=$((FAIL + 1))
    fi
}

# ---------------------------------------------------------------------------
# Regex contract: only `NNN_<name>.sql` (1-3 digits) is a valid migration name.
# ---------------------------------------------------------------------------
echo "Test group: filename regex"
match_or_empty() {
    local filename="$1"
    if [[ "$filename" =~ ^([0-9]{1,3})_.+\.sql$ ]]; then
        echo "${BASH_REMATCH[1]}"
    else
        echo "INVALID"
    fi
}

assert_eq "001_baseline.sql -> 001"          "001"     "$(match_or_empty "001_baseline.sql")"
assert_eq "019_ht_reconcile_log.sql -> 019"  "019"     "$(match_or_empty "019_ht_reconcile_log.sql")"
assert_eq "999_max.sql -> 999"               "999"     "$(match_or_empty "999_max.sql")"
assert_eq "1_short.sql -> 1"                 "1"       "$(match_or_empty "1_short.sql")"
assert_eq "no_prefix.sql -> INVALID"         "INVALID" "$(match_or_empty "no_prefix.sql")"
assert_eq "001-dash.sql -> INVALID"          "INVALID" "$(match_or_empty "001-dash.sql")"
assert_eq "001_no_ext -> INVALID"            "INVALID" "$(match_or_empty "001_no_ext")"
assert_eq "0001_too_long.sql -> INVALID"     "INVALID" "$(match_or_empty "0001_too_long.sql")"
assert_eq "abc_letters.sql -> INVALID"       "INVALID" "$(match_or_empty "abc_letters.sql")"

# ---------------------------------------------------------------------------
# Set-membership check (the bug the old `grep -q` had: substring matches).
# ---------------------------------------------------------------------------
echo "Test group: applied-set membership"
declare -A APPLIED_SET=([001]=1 [010]=1 [019]=1)

is_applied() {
    [ -n "${APPLIED_SET[$1]:-}" ] && echo "yes" || echo "no"
}

assert_eq "001 is applied"                    "yes" "$(is_applied 001)"
assert_eq "010 is applied"                    "yes" "$(is_applied 010)"
assert_eq "019 is applied"                    "yes" "$(is_applied 019)"
assert_eq "002 is NOT applied"                "no"  "$(is_applied 002)"
assert_eq "100 is NOT applied (was substring of 010 with old grep)" "no" "$(is_applied 100)"
assert_eq "1 is NOT applied (used to substring-match 001)" "no" "$(is_applied 1)"

# ---------------------------------------------------------------------------
# Pragma detection: `-- @transactional false` opt-out for CREATE INDEX
# CONCURRENTLY etc. Mirror migrate.sh::detect_transactional_pragma so we
# can assert the same behaviour without invoking the full runner.
# ---------------------------------------------------------------------------
echo "Test group: @transactional pragma detection"

detect_transactional_pragma_test() {
    local file="$1"
    if head -n 20 "$file" | grep -Eiq '^[[:space:]]*--[[:space:]]*@transactional[[:space:]]+false[[:space:]]*$'; then
        echo "false"
    else
        echo "true"
    fi
}

# Each fixture lives in a temp file so `head -n 20` and the grep see real
# newlines (not embedded `\n` in a string).
PRAGMA_TMP=$(mktemp -d)
trap 'rm -rf "$PRAGMA_TMP"' EXIT

write_fixture() {
    local name="$1"
    local body="$2"
    printf '%s\n' "$body" > "$PRAGMA_TMP/$name"
    echo "$PRAGMA_TMP/$name"
}

f_no_pragma=$(write_fixture "no_pragma.sql" "-- normal migration
CREATE TABLE foo (id INT);")
assert_eq "no pragma -> transactional=true"   "true" "$(detect_transactional_pragma_test "$f_no_pragma")"

f_lower=$(write_fixture "lower.sql" "-- @transactional false
CREATE INDEX CONCURRENTLY ix_foo ON foo(id);")
assert_eq "-- @transactional false -> false"  "false" "$(detect_transactional_pragma_test "$f_lower")"

f_upper=$(write_fixture "upper.sql" "-- @TRANSACTIONAL FALSE
CREATE INDEX CONCURRENTLY ix_foo ON foo(id);")
assert_eq "case-insensitive (UPPER) -> false" "false" "$(detect_transactional_pragma_test "$f_upper")"

f_mixed=$(write_fixture "mixed.sql" "-- @Transactional False
CREATE INDEX CONCURRENTLY ix_foo ON foo(id);")
assert_eq "case-insensitive (Mixed) -> false" "false" "$(detect_transactional_pragma_test "$f_mixed")"

f_extra_ws=$(write_fixture "extra_ws.sql" "  --   @transactional   false
CREATE INDEX CONCURRENTLY ix_foo ON foo(id);")
assert_eq "extra whitespace -> false"         "false" "$(detect_transactional_pragma_test "$f_extra_ws")"

f_after_header=$(write_fixture "after_header.sql" "-- Migration: 020_demo
-- Version: 2.46.1
-- Date: 2026-04-26
-- @transactional false
CREATE INDEX CONCURRENTLY ix_foo ON foo(id);")
assert_eq "pragma after header lines -> false" "false" "$(detect_transactional_pragma_test "$f_after_header")"

# Pragma BELOW line 20 must NOT trigger the opt-out (defends against a stray
# comment lower in the file accidentally disabling transactional safety).
deep_body=""
for i in $(seq 1 25); do
    deep_body="${deep_body}-- filler line $i
"
done
deep_body="${deep_body}-- @transactional false
CREATE TABLE foo (id INT);"
f_deep=$(write_fixture "deep.sql" "$deep_body")
assert_eq "pragma below line 20 -> true (default)" "true" "$(detect_transactional_pragma_test "$f_deep")"

# A non-comment line that happens to contain the literal text must not match
# (regex anchors require leading `--`).
f_bogus=$(write_fixture "bogus.sql" "SELECT 'this row mentions @transactional false in a string';")
assert_eq "non-comment match -> true (default)" "true" "$(detect_transactional_pragma_test "$f_bogus")"

# `-- @transactional true` is a no-op (default is true). Make sure we don't
# misread it as "false".
f_explicit_true=$(write_fixture "explicit_true.sql" "-- @transactional true
CREATE TABLE foo (id INT);")
assert_eq "explicit true -> true"             "true" "$(detect_transactional_pragma_test "$f_explicit_true")"

# ---------------------------------------------------------------------------
# Real migrations directory: every existing file must parse cleanly.
# ---------------------------------------------------------------------------
echo "Test group: real migrations directory parses"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MIGRATIONS_DIR="$PROJECT_ROOT/migrations/pg"
if [ -d "$MIGRATIONS_DIR" ]; then
    bad=0
    for f in "$MIGRATIONS_DIR"/*.sql; do
        [ -f "$f" ] || continue
        name=$(basename "$f")
        if [ "$(match_or_empty "$name")" = "INVALID" ]; then
            echo "  FAIL: real migration $name does not match regex"
            bad=$((bad + 1))
        fi
    done
    if [ $bad -eq 0 ]; then
        echo "  PASS: all real migrations under $MIGRATIONS_DIR parse"
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + bad))
    fi
else
    echo "  SKIP: $MIGRATIONS_DIR not found"
fi

# ---------------------------------------------------------------------------
echo ""
echo "Results: $PASS passed, $FAIL failed"
[ $FAIL -eq 0 ]
