#!/usr/bin/env bash
# check-cardinality-map.sh — enforce PROCESS.md P1 in CI.
#
# Every `CREATE TABLE ht_*` introduced under `migrations/pg/` MUST have a
# matching row in `docs/coexistence/CARDINALITY_MAP.md`. This script is the
# CI hook proposed by PROCESS.md "CI hook proposal" — was reviewer
# discipline only, now enforced.
#
# ## Scope
#
# Looks at `CREATE TABLE` (optionally `IF NOT EXISTS`) statements that
# create tables in the bare-`ht_` namespace inside `migrations/pg/*.sql`.
# `legacy_mirror.ht_*` and `ville.ht_*` and `*_legacy` tables are out of
# scope — those mirror legacy state 1:1 and live in dedicated sections of
# CARDINALITY_MAP.md that don't require a per-row cardinality decision.
#
# ## Behavior
#
# Exits 0 when every `ht_*` table found in migrations is mapped.
# Exits 1 (with a `::error::` diagnostic listing each gap) otherwise.
#
# ## Self-tests
#
# Run with `--self-test` to execute two assertions in a scratch dir:
#   1. PASSES when every CREATE TABLE has a matching map row.
#   2. FAILS when a CREATE TABLE has no matching map row.
# CI invokes the same script without `--self-test`.

set -euo pipefail

# Resolve repo root by climbing from this script's directory.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

MIGRATIONS_DIR_DEFAULT="${REPO_ROOT}/migrations/pg"
MAP_FILE_DEFAULT="${REPO_ROOT}/docs/coexistence/CARDINALITY_MAP.md"

# Allow override for self-tests.
MIGRATIONS_DIR="${MIGRATIONS_DIR:-${MIGRATIONS_DIR_DEFAULT}}"
MAP_FILE="${MAP_FILE:-${MAP_FILE_DEFAULT}}"

# Extract unique bare-namespace `ht_*` table names from CREATE TABLE
# statements in migrations/pg/*.sql. Excludes:
#   - schema-qualified names (legacy_mirror.ht_*, ville.ht_*)
#   - `*_legacy` mirror tables (covered by the mirror section, not the
#     canonical entity table, of CARDINALITY_MAP.md)
extract_migration_tables() {
    local dir="$1"
    # No migrations directory at all is a hard error — the script's whole
    # purpose is to scan it.
    if [ ! -d "${dir}" ]; then
        echo "::error::migrations directory not found: ${dir}" >&2
        exit 2
    fi
    # Step 1: regex grep all CREATE TABLE statements emitting ht_* into a buffer.
    # -h: hide filenames. -E: extended regex. -o: only the match.
    # The regex matches an optional schema-qualifier so we can filter it out
    # in step 2 with `awk`.
    grep -rhEo 'CREATE TABLE[[:space:]]+(IF NOT EXISTS[[:space:]]+)?([a-zA-Z_][a-zA-Z0-9_]*\.)?ht_[a-z0-9_]+' \
        "${dir}" 2>/dev/null \
    | awk '{
        # Last whitespace-separated token is the table name (possibly
        # schema-qualified). Strip any `schema.` prefix.
        n = split($0, parts, /[[:space:]]+/)
        name = parts[n]
        if (index(name, ".") > 0) {
            # Skip schema-qualified names (legacy_mirror.ht_*, ville.ht_*).
            next
        }
        # Skip *_legacy mirrors — they belong to the mirror section.
        if (name ~ /_legacy$/) next
        print name
      }' \
    | sort -u
}

# Check that `table` has a backtick-quoted entry on at least one line of
# the cardinality map. The map uses the convention `` `ht_foo` `` in the
# PG-table column.
table_is_mapped() {
    local table="$1"
    local map="$2"
    grep -qF "\`${table}\`" "${map}"
}

run_check() {
    if [ ! -f "${MAP_FILE}" ]; then
        echo "::error::CARDINALITY_MAP.md not found: ${MAP_FILE}" >&2
        return 2
    fi

    local tables
    tables="$(extract_migration_tables "${MIGRATIONS_DIR}")"

    local checked=0
    local missing=()

    if [ -z "${tables}" ]; then
        echo "No ht_* tables found in ${MIGRATIONS_DIR} — nothing to check."
        return 0
    fi

    while IFS= read -r table; do
        [ -z "${table}" ] && continue
        checked=$((checked + 1))
        if ! table_is_mapped "${table}" "${MAP_FILE}"; then
            missing+=("${table}")
        fi
    done <<< "${tables}"

    if [ ${#missing[@]} -gt 0 ]; then
        echo "::error::CARDINALITY_MAP.md is missing rows for the following ht_* tables introduced in migrations/pg/:"
        local gap
        for gap in "${missing[@]}"; do
            echo "::error::  - ${gap}"
        done
        echo "::error::See docs/coexistence/PROCESS.md P1 — every new ht_* table MUST have a row in CARDINALITY_MAP.md."
        return 1
    fi

    echo "PASS: ${checked} ht_* table(s) from migrations/pg/ are all mapped in CARDINALITY_MAP.md."
    return 0
}

# ----------------------------------------------------------------------
# Self-tests (--self-test). Two assertions in a scratch directory: the
# PASS case (every table mapped) and the FAIL case (a table unmapped).
# ----------------------------------------------------------------------
run_self_tests() {
    local tmp
    tmp="$(mktemp -d)"
    # shellcheck disable=SC2064
    trap "rm -rf '${tmp}'" EXIT

    local fixtures_dir="${tmp}/migrations"
    local map="${tmp}/CARDINALITY_MAP.md"
    mkdir -p "${fixtures_dir}"

    # Fixture migration containing two ht_* tables AND ignorable ones.
    cat > "${fixtures_dir}/100_fixture.sql" <<'SQL'
-- Self-test fixture: two canonical ht_* tables, plus two ignorable forms.
CREATE TABLE IF NOT EXISTS ht_foo (id INT);
CREATE TABLE ht_bar (id INT);
-- Schema-qualified — should be ignored.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_qux (id INT);
-- *_legacy mirror — should be ignored.
CREATE TABLE IF NOT EXISTS ht_baz_legacy (id INT);
SQL

    # PASS-case map: both bare ht_* names present.
    cat > "${map}" <<'MD'
| PG table | Legacy | Cardinality |
|---|---|---|
| `ht_foo` | HT_Foo | 1:1 |
| `ht_bar` | HT_Bar | 1:1 |
MD

    echo "[self-test] case 1 — all tables mapped → expect PASS"
    if MIGRATIONS_DIR="${fixtures_dir}" MAP_FILE="${map}" run_check >/dev/null; then
        echo "[self-test] case 1 OK"
    else
        echo "[self-test] case 1 FAILED — expected exit 0 when all tables mapped" >&2
        return 1
    fi

    # FAIL-case map: drop `ht_bar`.
    cat > "${map}" <<'MD'
| PG table | Legacy | Cardinality |
|---|---|---|
| `ht_foo` | HT_Foo | 1:1 |
MD

    echo "[self-test] case 2 — ht_bar unmapped → expect FAIL"
    local status=0
    MIGRATIONS_DIR="${fixtures_dir}" MAP_FILE="${map}" run_check >/dev/null 2>&1 || status=$?
    if [ "${status}" -eq 1 ]; then
        echo "[self-test] case 2 OK"
    else
        echo "[self-test] case 2 FAILED — expected exit 1 when ht_bar unmapped, got ${status}" >&2
        return 1
    fi

    echo "[self-test] all passed"
    return 0
}

if [ "${1:-}" = "--self-test" ]; then
    run_self_tests
    exit $?
fi

run_check
exit $?
