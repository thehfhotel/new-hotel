#!/usr/bin/env bash
# check-doc-anchors.sh — enforce the repo's document-citation convention in CI.
#
# This repo cites its own living documents constantly, and until now it did so
# by LINE NUMBER, in at least six different spellings. Editing either document
# silently invalidated every citation pointing into it: PR #291's doc growth
# broke ~40 citations across 25 files, and several of those had already been
# wrong for months without anyone noticing, because nothing ever checked them.
#
# This script is that check. It is pure text over `git ls-files` — no network,
# no database, no cargo, no docker. Modelled on `scripts/check-cardinality-map.sh`
# (same `::error::` output, same `--self-test` flag, same repo-root resolution).
# The matching engine is Perl rather than awk because `§` is a two-byte UTF-8
# sequence and the anchors nest quotes inside quotes — both of which are fragile
# across the awk flavours CI and dev machines actually ship.
#
# ======================================================================
# THE CONVENTION (one form, all three parts on ONE line)
# ======================================================================
#
#     <doc-ref> §<section-anchor> ["<phrase-anchor>"]
#
# e.g.  `docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_CheckIn_Ds` "Refund deposit"
#       `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "R{yyMM}-{4digit}"
#       docs/architecture.md §3.6d
#
# 1. <doc-ref> — a repo-relative path (backticks optional), a unique file
#    BASENAME, or one of the registered aliases in ALIAS_TABLE below. A token
#    that LOOKS like a document (`*.md`, `*.sql`, or a registered alias) but
#    resolves to no git-tracked file is a failure ("target not found").
#
# 2. §<section-anchor> — `§` MEANS SECTION, ALWAYS AND ONLY. Two spellings:
#      a. the heading NUMBER exactly as the doc writes it: §1.6, §3.6d, §J3, §3e
#      b. a backticked-or-quoted HEADING-TEXT SUBSTRING when the heading is
#         unnumbered: §`HT_Round_Bill`, §"Table: dbo.HT_Book_H"
#    The anchor must resolve to EXACTLY ONE heading in the target. That single
#    rule retires the old `§946-956` (which meant LINES) for free: no heading is
#    named "946-956", so it fails to resolve.
#
# 3. "<phrase-anchor>" — optional, but REQUIRED whenever the citation supports a
#    specific factual claim rather than pointing at a whole section. A
#    double-quoted literal that must appear inside the resolved section's body.
#    This is the half that upgrades "the pointer resolves" into "the claim is
#    actually there".
#
# CONTINUATION: a `§` with no doc-ref before it on its own line binds to the
# nearest preceding doc-ref in the SAME contiguous comment block, within 10
# lines. That keeps multi-line doc comments legal as written:
#     //! Per docs/architecture.md §1, §6 and docs/legacy-spike/findings.md
#     //! §3a (walk-in), §3d (check-in to booking)
# Inside a Markdown document, a `§` with no doc-ref in scope is a SELF-reference
# and resolves against the citing document itself ("see §3a" in findings.md).
#
# A SELF-REFERENCE MUST NAME ONE OF THAT DOCUMENT'S OWN HEADINGS, and it is
# resolved with the citing line — and the rest of its paragraph — REMOVED from
# the evidence. Both halves matter, and neither is theoretical:
#   * The whole-file unique-literal fallback (below) used to run for
#     self-references too. Since a citation contains its own anchor text, the
#     fallback found the anchor ON THE CITING LINE and reported a clean resolve.
#     `see §"Heading That Never Existed"` VALIDATED ITSELF. Four live sites in
#     this tree passed exactly that way, and every one of them was a real
#     citation into a DIFFERENT document (architecture.md's §3a means
#     findings.md's; the 2026-06-11 audit's §3.13 means the cheatsheet's) — the
#     guard was not merely lenient, it was confirming the wrong file.
#   * So the literal fallback is OFF for a self-reference into a document that
#     has headings: there, "this names one of my headings" is the only honest
#     evidence. MEASURED before shipping — 106 self-references in this tree
#     resolve by heading and are unaffected; ZERO resolved by a literal
#     elsewhere in their file; the 4 that resolved by literal all resolved on
#     their own line. The fallback bought nothing here and cost soundness.
# A `§` that means another document's section must NAME that document on the
# line. That is the convention, and it is now checkable rather than assumed.
#
# A DOC-REF IS THE PRECONDITION FOR CHECKING A `§`, NOT SOMETHING TO DEMAND.
# Outside Markdown, a `§` with no doc-ref in scope is NOT A CITATION and is not
# checked. `§` is also how this tree writes a section of an external standard
# (`RFC 4122 §4.3`), a section of a CODE file's doc comment (`writeback recipe
# checkout.rs §3e` — and code files are not documents, see below), a section of
# a printed `.html` checklist, and a work-item label in an off-repo plan
# (`wave-4 §A3.1`). None of those name anything this script could resolve, and
# an earlier version reported all 124 of them as "no document named", which is
# the guard being wrong about the file rather than the other way round.
#
# `RFC <number> §<n>` is exempt OUTRIGHT, not merely by falling outside the
# continuation window. No RFC is a git-tracked file here, so that `§` can never
# be a citation this script resolves — but `outbox/idempotency.rs` names
# `docs/architecture.md` 11 lines above its `RFC 4122 §4.3`, one line outside
# the cap, so a single added sentence would otherwise turn a non-citation red.
#
# The 10-line continuation cap is itself load-bearing and MEASURED: removing it
# to recover coverage (a tempting one-character change) immediately fails both
# `RFC 4122 §4.3` sites against architecture.md. Coverage is not worth
# reintroducing the false-positive class this guard was corrected to stop.
#
# The cost is real: some skipped `§`s ARE citations into real repo documents
# whose doc-ref merely sits outside the continuation window, and those are now
# unverified. That gap is counted on the PASS line and enumerable with
# `--report-unbound`, so it stays a visible number. Adding a doc-ref to the
# line is what moves one back under the check. NOTE that this count is NOT
# ratcheted — it is reported, not enforced. A hard ceiling on it was considered
# and rejected: the only fix for a legitimate new `RFC …§` or off-repo plan
# label would be to raise the number, which is the "just bump the count"
# anti-pattern the baseline below refuses on purpose.
#
# `docs/legacy-app/SCHEMA.sql` has no markdown headings but exactly one
# `-- Table: dbo.<Name>` marker per table; those markers ARE its headings.
# A target with no headings at all falls back to whole-file unique-literal: the
# anchor must occur exactly once, and the phrase within 40 lines below it.
#
# NEVER ANCHOR ON A FIGURE. Stop the anchor before any parenthesised count or
# timing: §"3h. Take payment + print invoice", never
# §"3h. Take payment + print invoice (7 statements, ~26s gap...)". Figures get
# corrected; the prose before the "(" is what a citation can rely on.
#
# ======================================================================
# THE TWO RULES THIS SCRIPT ENFORCES
# ======================================================================
#
# RULE A — RESOLVE. Every citation of the canonical form must land: the target
#   file must exist, the section anchor must match exactly one heading, and the
#   phrase (when present) must occur inside that section's extent.
#
# RULE B — BAN. No line numbers into LIVING documents. Five spellings are
#   rejected: `<doc>.md:N`, `<doc> line(s) N[-M]`, `§N` with a bare 3+ digit N,
#   the `COMPAT:N` alias-colon form, and the `#LN` GitHub line anchor.
#
#   Rule A alone would have passed clean over almost every defect in this repo,
#   because in the common broken case the document name is nowhere near the
#   number. Both rules are needed.
#
# LINE NUMBERS INTO FROZEN EVIDENCE ARE CORRECT and are exempt from Rule B —
# but ONLY when the frozen file is NAMED ON THE SAME LINE. That is not
# cosmetic: naming the file is exactly what lets this script tell an allowed
# frozen reference from a banned living one. Frozen = `docs/legacy-spike/raw/**`
# (read-only forever), `docs/legacy-spike/schema/**`, `schema-baseline.txt`, the
# off-repo v1.45 decompile (`*.cs`, `_decompiled_clean`), and the off-repo
# Profiler capture `/tmp/legacy-events-full.log`.
#
# ## Behavior
#
# Exits 0 when every citation resolves and no living-document line number
# remains. Exits 1 with a `::error file=…,line=…::` per violation otherwise.
# Exits 2 when the guard cannot make a trustworthy statement at all — a
# malformed allowlist entry, or a ledger with a row that is not classified and
# justified. Exit 2 is NOT downgraded by `--report-only`: that flag tolerates
# known citation debt, not a broken configuration.
# `--report-only` prints the same diagnostics but exits 0 — used while the
# tree-wide conversion lands package by package. Flags compose:
# `--report-only --report-unbound` is a legitimate triage run.
#
# Doc-refs resolve against `git ls-files`, which is what CI sees. A brand-new
# document that exists on disk but has not been staged yet therefore fails as
# "target not found", and the diagnostic says so in those words.
#
# ## The baseline
#
# This is a NEW lint on an EXISTING codebase, so it ships with a recorded
# baseline of pre-existing debt: `scripts/doc-anchors-baseline.tsv`. It is a
# LEDGER, not a mute, and four separate rules keep it one:
#
#   1. A failure that is not listed still fails the build.
#   2. A listed failure occurring MORE times than its recorded count fails too,
#      so the debt cannot grow by copy-paste.
#   3. EVERY ROW MUST BE CLASSIFIED AND JUSTIFIED — `scope` is `in-scope` or
#      `out-of-scope`, `reason` is a real sentence with no TODO/FIXME/XXX in it.
#      A row that fails either test EXITS 2. This is what stops the ledger being
#      a mute button: `--update-baseline` writes `unclassified` and a TODO
#      reason for any row it has not seen, and those rows are REFUSED, so a
#      regenerate-and-commit cycle cannot make a failure quietly disappear.
#   4. RAISING A COUNT IS AN ERROR. The ledger is compared against its version
#      at the merge base; a row whose count went up fails the build. Rule 2
#      catches the copy-paste; rule 4 catches the "fix" of widening the
#      tolerance afterwards, which is the move the file's own header bans by
#      name and which nothing used to enforce. Adding a row stays legal and is
#      announced with its scope and reason on every run. When no merge base is
#      available (a shallow checkout, or a run outside a git tree) the run SAYS
#      the comparison was skipped and why — a silently skipped check reads as a
#      guarantee that is not being made.
#
# A listed failure that gets FIXED prints a warning asking for the row to be
# deleted (warning, not error: paying down debt must never turn a build red).
# The entry key is path + diagnostic class + citation TEXT and contains NO LINE
# NUMBER — a line-keyed baseline would rot exactly the way the line citations
# this branch deleted did. Regenerate with `--update-baseline`; see the file's
# own header for the full contract.
#
# `--report-unbound` lists every `§` skipped for want of a doc-ref — the
# coverage this guard deliberately does not have. It prints on a FAILING run as
# well as a passing one: the flag exists to keep that gap visible, and the run
# where someone needs to see it is the red one.
#
# ## Self-tests
#
# `--self-test` runs both rules against fixture files in a scratch dir:
#   1. PASS when the cited heading and phrase are both present.
#   2. FAIL when the heading is renamed out from under the citation.
#   3. FAIL when the phrase is deleted from the cited section but left
#      elsewhere in the file (proves section scoping, not whole-file grep).
#   4. FAIL when the anchor matches two headings (ambiguous).
#   5. FAIL on a bare `foo.md:42` line-number citation.
#   6. PASS on a line number into frozen capture evidence.
#   7. FAIL when the doc-ref names a file that does not exist.
#   8. PASS on a multi-line comment block using the continuation rule.
#   9. PASS on a `§` that names no document (RFC section, off-repo plan label).
#  10. PASS when a Markdown table row's self-reference is NOT hijacked by a
#      doc-ref named in an earlier row of the same table.
#  11. FAIL on a broken citation with no baseline.
#  12. PASS on that same failure once baselined — and says it is tolerating it.
#  13. FAIL on a BRAND-NEW broken citation while the old one stays baselined.
#      This is the property that makes the baseline a ledger and not a mute.
#  14. FAIL when a baselined citation is copy-pasted to a second site (the
#      recorded count is part of the entry, so the debt cannot grow).
#  15. PASS when a baselined citation is fixed, with a warning asking for the
#      row to be removed.
#  16. PASS on a line number into the off-repo frozen Profiler capture log.
#  17. PASS on an `RFC NNNN §N` whose block names a real document INSIDE the
#      continuation window — the standard's section marker is never a citation.
#  18. PASS on a self-reference to a REAL heading of the citing document — the
#      shape 106 citations in this tree use, which the soundness fix must keep.
#  19. FAIL on an INVENTED self-reference whose anchor text appears only on the
#      citation's own line. This is the self-validation hole; before the fix it
#      passed.
#  20. FAIL on the same invention when the anchor text is also echoed elsewhere
#      in the citing paragraph — excluding one line is not enough.
#  21. FAIL when a heading is renamed out from under a SELF-reference. The
#      property the branch exists for, applied to the citation form that used to
#      be unfalsifiable.
#  22. EXIT 2 on a ledger row whose scope is `unclassified`.
#  23. EXIT 2 on a ledger row whose reason is still a TODO placeholder.
#  24. FAIL when a ledger row's count is RAISED against the base version.
#  25. PASS when a row is legitimately added — and announce it, with scope and
#      reason, so growth is never quiet.
#  26. `--report-unbound` prints its listing on a FAILING run.
#  27. The allowlist's rules column: a session record's line-number specimen is
#      exempt from Rule B, and a broken anchor on the next line of the same file
#      still FAILS Rule A.
#  28. EXIT 2 on a malformed allowlist entry — a typo must never widen an
#      exemption to both rules.
#  29. FAIL when a heading's bare numeric label is renamed by PREPENDING a
#      digit (`9k.` -> `19k.`) — the old label is still a SUBSTRING of the new
#      one, which is exactly how `### 3e.` -> `### 13e.` stayed green in the
#      real tree until this case was added (PR #292 peer review).
#  30. PASS when a bare anchor is not an exact heading label but the heading
#      text STARTS with it as a whole token (`7g` against `7g-note. …`) — the
#      cure is a whole-token PREFIX match, not a bare exact-label match with
#      the substring fallback deleted outright.
# CI invokes the same script with no arguments.

set -euo pipefail

# Byte semantics everywhere: `§` is multi-byte UTF-8 and the docs carry Thai
# text. This script never counts characters, only bytes, so C is correct and
# identical on every runner.
export LC_ALL=C

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Allow override for self-tests.
SCAN_ROOT="${SCAN_ROOT:-${REPO_ROOT}}"

REPORT_ONLY=0
REPORT_UNBOUND=""
MODE=check

# Path of the tolerated-debt ledger, RELATIVE to the scanned tree.
BASELINE_REL="scripts/doc-anchors-baseline.tsv"

# ----------------------------------------------------------------------
# Alias table. Registered short names that may stand in for a full path, so
# existing healthy prose is not churned. Anything NOT listed here (and not a
# resolvable path or unique basename) is a guard failure.
# ----------------------------------------------------------------------
alias_table() {
    cat <<'ALIAS'
spike=docs/legacy-spike/findings.md
findings=docs/legacy-spike/findings.md
cheatsheet=docs/legacy-app/COMPAT_CHEATSHEET.md
COMPAT_CHEATSHEET=docs/legacy-app/COMPAT_CHEATSHEET.md
COMPAT=docs/legacy-app/COMPAT_CHEATSHEET.md
FEATURE_MAP=docs/legacy-app/FEATURE_MAP.md
architecture=docs/architecture.md
ROOM_GRID_REFRESH=docs/legacy-app/ROOM_GRID_REFRESH.md
ROOM_STATUS_PALETTE=docs/legacy-app/ROOM_STATUS_PALETTE.md
ALIAS
}

# ----------------------------------------------------------------------
# Paths never SCANNED. (Several of them may still be CITED — raw/** is the
# canonical frozen evidence and citing it by line is correct and expected.)
#
#   docs/legacy-spike/raw/     primary captured evidence, read-only forever
#   docs/legacy-spike/schema/  frozen schema capture (twin of schema-baseline)
#   hotel-backend/schema-baseline.txt   ditto, the backend's copy
#   .decompile/                vendored decompile working area
#   CHANGELOG.md               release-please owns it; its historical citations
#                              are release history and must not be rewritten
#   .sqlx/                     generated sqlx query cache
#   target/, node_modules/, lockfiles   build output
# ----------------------------------------------------------------------
EXCLUDE_RE='^(docs/legacy-spike/raw/|docs/legacy-spike/schema/|hotel-backend/schema-baseline\.txt$|\.decompile/|\.sqlx/|target/|node_modules/|CHANGELOG\.md$|pnpm-lock\.yaml$|package-lock\.json$)'

# ----------------------------------------------------------------------
# Allowlist — `path-glob<TAB>rules<TAB>perl-regex<TAB>reason`.
#
# Entries are path + PATTERN, never path + LINE, so the allowlist itself cannot
# rot when a file is edited. Every entry carries a reason.
#
# The RULES column says WHICH rule the entry suppresses, and it is not
# decoration: an exemption must be no wider than the reason that justifies it.
#   A   — Rule A only (anchor resolution). The line's `§` is not a citation.
#   B   — Rule B only (line numbers). The line's `line N` is not a file line.
#   AB  — both. Only for lines that are not citations of any kind.
#
# `docs/sessions/*` is the entry that motivated the column. It used to read
# pattern `.` with no rules column, i.e. BOTH rules off on EVERY line of every
# session record. Its stated reason — session records quote the DEFECTIVE
# line-number spellings verbatim as specimens — justifies exactly Rule B and
# nothing more. It is now `B`, so a session record's `§` anchors resolve like
# anywhere else, and the handful that do not are recorded in the ledger by name
# rather than hidden behind a wildcard.
#
# This set is MEASURED, not guessed — it is every genuine non-citation `line N`
# in the tree as of this branch. Same device `check-write-pool-routing.sh`
# already uses for its handler exceptions.
# ----------------------------------------------------------------------
allowlist_entries() {
    cat <<'ALLOW'
hotel-backend/src/render/thai_id_card.rs	B	[Ll]ines? [12](\D|$)	printed Thai-ID-card address lines 1/2, not file lines
hotel-backend/src/scheduler/payment_ledger_probe.rs	B	line 55(121|149)	HT_CheckIn_Pay folio ledger row ids, not file lines
thai-id-middleware-tauri/src-tauri/src/mrz.rs	B	[Ll]ines? [12](\D|$)	passport MRZ TD3 line 1/2, not file lines
thai-id-middleware-tauri/src-tauri/src/ihotel/*	B	[Ll]ines? [12](\D|$)	MRZ/ID-card line 1/2, not file lines
scripts/test-migrate-parse.sh	B	line 20(\D|$)	a parser rule about migration-file line 20, not a citation
scripts/deploy/run-deploy.sh	B	\(line 519\)	self-reference inside the same file; flagged as a follow-up, not a citation
docs/sessions/*	B	.	session handoff records quote the DEFECTIVE line-number spellings verbatim as specimens; they are history, not guidance — same rationale as CHANGELOG.md. Rule B ONLY: an anchor in a session record still has to resolve.
scripts/check-doc-anchors.sh	AB	.	this script documents every banned spelling, and every § spelling, in its own header
scripts/doc-anchors-baseline.tsv	AB	.	the tolerated-debt ledger quotes each broken citation verbatim as its identity key; it is the RECORD of the defects, not an instance of them
ALLOW
}

# ----------------------------------------------------------------------
# The check itself.
# ----------------------------------------------------------------------
run_check() {
    local tmp
    tmp="$(mktemp -d)"
    # shellcheck disable=SC2064
    trap "rm -rf '${tmp}'" RETURN

    # The baseline lives beside this script, inside whatever tree is being
    # scanned — so the self-tests get their own fixture baseline for free.
    local baseline="${SCAN_ROOT}/${BASELINE_REL}"

    # Every tracked path, for doc-ref resolution (raw/** included — frozen
    # evidence is a legitimate citation TARGET even though it is never scanned).
    list_tracked > "${tmp}/tracked.txt"
    if [ ! -s "${tmp}/tracked.txt" ]; then
        echo "::error::check-doc-anchors: no files to scan under ${SCAN_ROOT}" >&2
        return 2
    fi

    # The subset actually scanned: tracked, not excluded, not binary.
    grep -vE "${EXCLUDE_RE}" "${tmp}/tracked.txt" > "${tmp}/candidates.txt" || true
    ( cd "${SCAN_ROOT}" && tr '\n' '\0' < "${tmp}/candidates.txt" \
        | xargs -0 grep -Il '' 2>/dev/null ) > "${tmp}/scan.txt" || true

    alias_table       > "${tmp}/alias.txt"
    allowlist_entries > "${tmp}/allow.txt"
    write_engine      > "${tmp}/engine.pl"

    local status=0
    if [ "${MODE}" = "write" ]; then
        # Never truncate the baseline in place: on an engine crash that would
        # destroy the ledger and turn the next run green for the wrong reason.
        DA_SCAN_ROOT="${SCAN_ROOT}" \
        DA_TRACKED="${tmp}/tracked.txt" \
        DA_SCAN="${tmp}/scan.txt" \
        DA_ALIAS="${tmp}/alias.txt" \
        DA_ALLOW="${tmp}/allow.txt" \
        DA_BASELINE="${baseline}" \
        DA_MODE=write \
            perl "${tmp}/engine.pl" > "${tmp}/baseline.new" || status=$?
        if [ "${status}" -eq 0 ]; then
            mkdir -p "$(dirname "${baseline}")"
            mv "${tmp}/baseline.new" "${baseline}"
            echo "check-doc-anchors: wrote ${baseline}"
        fi
    else
        local base_info base_state base_desc
        base_info="$(fetch_base_baseline "${tmp}/baseline.base")"
        base_state="${base_info%%|*}"
        base_desc="${base_info#*|}"

        DA_SCAN_ROOT="${SCAN_ROOT}" \
        DA_TRACKED="${tmp}/tracked.txt" \
        DA_SCAN="${tmp}/scan.txt" \
        DA_ALIAS="${tmp}/alias.txt" \
        DA_ALLOW="${tmp}/allow.txt" \
        DA_BASELINE="${baseline}" \
        DA_BASELINE_BASE="${tmp}/baseline.base" \
        DA_BASE_STATE="${base_state}" \
        DA_BASE_DESC="${base_desc}" \
        DA_REPORT_UNBOUND="${REPORT_UNBOUND}" \
        DA_MODE=check \
            perl "${tmp}/engine.pl" || status=$?
    fi

    if [ "${status}" -gt 1 ]; then
        # Exit 2 is a CONFIGURATION failure, not a citation failure: a malformed
        # allowlist or an invalid ledger. The engine has already said which.
        echo "::error::check-doc-anchors: could not run a trustworthy check (engine exit ${status}) — see the diagnostics above; the tree was NOT given a clean bill of health." >&2
        return 2
    fi
    return "${status}"
}

# ----------------------------------------------------------------------
# The ledger's PREVIOUS version, for the growth check.
#
# Row-level validation (classified scope, real reason) makes a row impossible to
# add silently. It does not, on its own, stop someone RAISING THE COUNT on a row
# that already exists — the one move the ledger's own header bans by name ("The
# fix is to remove the new occurrence, never to raise the count"). Comparing
# against the merge base is what turns that sentence into an enforced rule.
#
# Availability is not assumed. A shallow CI checkout has no merge base, and the
# self-test fixtures are not a repo at all; in those cases the comparison is
# SKIPPED WITH A STATED REASON printed on the run, never silently.
# `DOC_ANCHORS_BASE_REF` overrides the ref; `DOC_ANCHORS_BASE_FILE` overrides
# the whole lookup with a file (that is how the self-tests drive it).
# ----------------------------------------------------------------------
doc_anchors_base_ref() {
    local r
    if [ -n "${DOC_ANCHORS_BASE_REF:-}" ]; then echo "${DOC_ANCHORS_BASE_REF}"; return 0; fi
    if [ -n "${GITHUB_BASE_REF:-}" ]; then
        if git -C "${SCAN_ROOT}" rev-parse --verify -q "origin/${GITHUB_BASE_REF}" >/dev/null 2>&1; then
            echo "origin/${GITHUB_BASE_REF}"; return 0
        fi
    fi
    for r in origin/master origin/main master main; do
        if git -C "${SCAN_ROOT}" rev-parse --verify -q "${r}" >/dev/null 2>&1; then
            echo "${r}"; return 0
        fi
    done
    echo ""
}

# Writes the base version of the ledger to $1 and echoes "<state>|<description>".
# state: available | absent-at-base | unavailable
fetch_base_baseline() {
    local out="$1" ref mb
    if [ -n "${DOC_ANCHORS_BASE_FILE:-}" ]; then
        if [ -e "${DOC_ANCHORS_BASE_FILE}" ]; then
            cp "${DOC_ANCHORS_BASE_FILE}" "${out}"
            echo "available|${DOC_ANCHORS_BASE_DESC:-DOC_ANCHORS_BASE_FILE}"
        else
            echo "absent-at-base|${DOC_ANCHORS_BASE_DESC:-DOC_ANCHORS_BASE_FILE}"
        fi
        return 0
    fi
    if ! git -C "${SCAN_ROOT}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        echo "unavailable|${SCAN_ROOT} is not a git checkout"
        return 0
    fi
    ref="$(doc_anchors_base_ref)"
    if [ -z "${ref}" ]; then
        echo "unavailable|no base branch ref found (tried \$DOC_ANCHORS_BASE_REF, \$GITHUB_BASE_REF, origin/master, origin/main, master, main)"
        return 0
    fi
    mb="$(git -C "${SCAN_ROOT}" merge-base HEAD "${ref}" 2>/dev/null || true)"
    if [ -z "${mb}" ]; then
        echo "unavailable|no merge base between HEAD and ${ref} (a shallow clone has none — set fetch-depth: 0)"
        return 0
    fi
    if git -C "${SCAN_ROOT}" cat-file -e "${mb}:${BASELINE_REL}" 2>/dev/null; then
        if git -C "${SCAN_ROOT}" show "${mb}:${BASELINE_REL}" > "${out}" 2>/dev/null; then
            echo "available|merge base $(printf '%.12s' "${mb}") with ${ref}"
        else
            echo "unavailable|could not read ${BASELINE_REL} at ${mb}"
        fi
    else
        echo "absent-at-base|merge base $(printf '%.12s' "${mb}") with ${ref}"
    fi
}

# Tracked-file list. Uses `git ls-files` in a real checkout (so untracked
# scratch files are invisible to the guard, same as every other lint here) and
# falls back to `find` for the self-test fixtures, which are not a repo.
list_tracked() {
    if git -C "${SCAN_ROOT}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        git -C "${SCAN_ROOT}" ls-files
    else
        ( cd "${SCAN_ROOT}" && find . -type f | sed 's|^\./||' | sort )
    fi
}

# ----------------------------------------------------------------------
# The engine. Emitted to a temp file and run with perl.
# ----------------------------------------------------------------------
write_engine() {
    cat <<'ENGINE'
use strict;
use warnings;

my $ROOT    = $ENV{DA_SCAN_ROOT};
my $SECT    = "\xc2\xa7";               # UTF-8 bytes for the section marker

# ---------------------------------------------------------------- inputs
sub slurp_lines {
    my ($p) = @_;
    open my $fh, '<', $p or die "check-doc-anchors: cannot read $p: $!\n";
    my @l = <$fh>;
    close $fh;
    chomp @l;
    return @l;
}

my (%tracked, %by_base);
for my $p (slurp_lines($ENV{DA_TRACKED})) {
    next unless length $p;
    $tracked{$p} = 1;
    (my $b = $p) =~ s{.*/}{};
    push @{ $by_base{$b} }, $p;
}

my %alias;
for my $l (slurp_lines($ENV{DA_ALIAS})) {
    next unless $l =~ /\S/;
    my ($k, $v) = split /=/, $l, 2;
    $alias{ lc $k } = $v;            # case-insensitive: `Spike` == `spike`
}

# glob -> regex, `*` never crossing a `/`
sub glob2re {
    my ($g) = @_;
    my $re = '';
    for my $c (split //, $g) {
        if    ($c eq '*') { $re .= '[^/]*' }
        elsif ($c eq '?') { $re .= '[^/]'  }
        else              { $re .= quotemeta $c }
    }
    return qr/\A$re\z/;
}

# [ path-regex, rules, line-regex, reason ]. `rules` is 'A', 'B' or 'AB' and is
# checked here so a typo can never silently widen an exemption to both rules.
my @allow;
for my $l (slurp_lines($ENV{DA_ALLOW})) {
    next unless $l =~ /\S/;
    next if $l =~ /^#/;
    my ($g, $rules, $re, $why) = split /\t/, $l, 4;
    unless (defined $why && length $why && defined $rules && $rules =~ /\A(?:A|B|AB)\z/) {
        printf "::error::check-doc-anchors: malformed allowlist entry (want: path-glob<TAB>A|B|AB<TAB>regex<TAB>reason): %s\n", $l;
        exit 2;
    }
    push @allow, [ glob2re($g), $rules, qr/$re/, $why ];
}

my @scan = grep { length } slurp_lines($ENV{DA_SCAN});

# ---------------------------------------------------------------- frozen
# A line-number citation is legitimate ONLY when the frozen file it points into
# is named on the same line. These are the frozen families.
my $FROZEN_RE = qr{
      docs/legacy-spike/raw/
    | docs/legacy-spike/schema/
    | writes\.txt
    | events\.txt
    | \d\d-[A-Za-z0-9][A-Za-z0-9._-]*\.txt
    | schema-baseline\.txt
    | _decompiled_clean
    | \b[A-Za-z_][A-Za-z0-9_]*\.cs\b
    | legacy-events-full\.log
}x;
# NOTE on that last branch: `/tmp/legacy-events-full.log` is the off-repo raw
# Profiler capture the writeback recipes were derived from. It is frozen in the
# same sense as the `*.cs` decompile — nothing in this repo can regenerate or
# renumber it — so a line number into it is the ONLY citation form available.
# Named exactly, NOT generalised to `\.log\b`: a wildcard would let a genuine
# living-document line number ride in beside any mention of any log file.

# ---------------------------------------------------------------- doc-refs
# A token "looks like" a document reference if it is a registered alias, an
# `ADR NNNN` reference, or carries a `.md`/`.sql` extension. Code files are
# deliberately NOT documents: `§` addresses a heading, and a citation into
# `page.tsx` or `sync.rs` gets a SYMBOL name, never a section marker.
# Distinguishing "looks like" from "resolves" is what lets us say "target not
# found" instead of silently skipping a citation whose document was deleted.
sub looks_like_docref {
    my ($t) = @_;
    return 1 if exists $alias{ lc $t };
    return 1 if $t =~ /\Aadr:\d{4}\z/;
    return 1 if $t =~ /\.(md|sql)\z/i;
    return 0;
}

sub resolve_docref {
    my ($t) = @_;
    return $alias{ lc $t } if exists $alias{ lc $t };
    if ($t =~ /\Aadr:(\d{4})\z/) {
        my @m = grep { m{\Adocs/adr/$1-.*\.md\z} } sort keys %tracked;
        return @m == 1 ? $m[0] : undef;
    }
    return $t if $tracked{$t};
    (my $b = $t) =~ s{.*/}{};
    if ($by_base{$b} && @{ $by_base{$b} } == 1) { return $by_base{$b}[0] }
    return undef;
}

# Walk candidates right-to-left; the RIGHTMOST one that LOOKS like a doc-ref
# decides. Returns ($resolved_path, $display_token) — $resolved_path undef
# means the token named a document that does not exist.
sub docref_in {
    my ($s) = @_;
    my @cand;                            # [ position, token ]
    # `ADR 0006 §5` is an established spelling across components/, the backend
    # and docs/; it resolves to the unique docs/adr/0006-*.md.
    while ($s =~ /(?<![A-Za-z0-9_])(?:ADR|adr)[ _-]?(\d{4})(?!\d)/g) {
        push @cand, [ $-[0], "adr:$1" ];
    }
    while ($s =~ /([A-Za-z0-9_][A-Za-z0-9_.\/-]*)/g) {
        # Read the match POSITION before touching $c: a substitution resets @-,
        # so the original `push @cand, [ $-[1], ... ]` after an in-place s///
        # recorded undef for every candidate. That silently disabled the
        # rightmost-wins sort below (undef <=> undef is 0, so the list stayed in
        # left-to-right match order) and emitted ~176k perl warnings per run,
        # which is most of what CI would have printed.
        my $p = $-[1];
        (my $c = $1) =~ s/[.-]+\z//;     # trailing sentence period / dash
        push @cand, [ $p, $c ];
    }
    for my $c (sort { $b->[0] <=> $a->[0] } @cand) {
        next unless looks_like_docref($c->[1]);
        my $disp = $c->[1];
        $disp =~ s/\Aadr:/ADR /;
        return (resolve_docref($c->[1]), $disp);
    }
    return (undef, undef);
}

# ---------------------------------------------------------------- targets
my %doc;
sub load_doc {
    my ($p) = @_;
    return $doc{$p} if exists $doc{$p};

    my @lines;
    if (open my $fh, '<', "$ROOT/$p") { @lines = <$fh>; close $fh; chomp @lines }
    else { return $doc{$p} = undef }

    my @head;                            # [ line0, level, text, label ]
    my $fence = 0;
    for my $i (0 .. $#lines) {
        # Fenced code blocks hold shell comments that start with `#`; they are
        # NOT headings. architecture.md's State A/B/C bash fence is exactly this.
        if ($lines[$i] =~ /^\s*(?:```|~~~)/) { $fence = !$fence; next }
        next if $fence;

        my ($lvl, $txt);
        if ($p =~ /\.md\z/i && $lines[$i] =~ /^(\#{1,6})\s+(.*?)\s*\z/) {
            ($lvl, $txt) = (length $1, $2);
            $txt =~ s/\s*\#+\s*\z//;     # closed ATX headings
        }
        elsif ($p =~ /\.sql\z/i && $lines[$i] =~ /^--\s*(Table:\s*\S.*?)\s*\z/) {
            ($lvl, $txt) = (1, $1);
        }
        next unless defined $txt;

        # Leading heading LABEL: `3a.`, `1.6`, `3g-bis.`, `J3.`, `11a.`
        my $label;
        if ($txt =~ /\A([A-Za-z]{0,2}\d[0-9A-Za-z.-]*?)\.?(?=\s|\z)/) { $label = $1 }
        push @head, [ $i, $lvl, $txt, $label ];
    }
    return $doc{$p} = { lines => \@lines, head => \@head };
}

# Extent of heading #$k: its own line through the line before the next heading
# of the same or shallower level.
sub extent {
    my ($d, $k) = @_;
    my $H = $d->{head};
    my $from = $H->[$k][0];
    my $to   = $#{ $d->{lines} };
    for my $j ($k + 1 .. $#$H) {
        if ($H->[$j][1] <= $H->[$k][1]) { $to = $H->[$j][0] - 1; last }
    }
    return ($from, $to);
}

# Returns ($from, $to, $err, $class). $err set means the anchor did not resolve.
#
# $excl is a set of 0-based line indices that MUST NOT be used as evidence. It
# is non-empty only when the citation is inside the document it cites, and it
# then holds the citing line and the rest of its paragraph/comment block. See
# the long note at the call site: without it a citation resolves against its own
# text and validates itself.
#
# $no_literal suppresses the whole-file unique-literal fallback. Set for a
# self-citation into a document that HAS headings: there, "the anchor names a
# heading" is the only evidence worth anything, and the fallback is precisely
# what let an invented heading name pass.
sub resolve_anchor {
    my ($d, $anchor, $kind, $excl, $no_literal) = @_;
    $excl ||= {};
    my $H = $d->{head};
    my @cand = grep { !$excl->{ $H->[$_][0] } } 0 .. $#$H;
    my @hit;

    if (@cand) {
        if ($kind eq 'bare') {
            # A bare anchor is a heading NUMBER: match the heading's own label
            # exactly, so §3 cannot swallow §3a/§3.6d.
            @hit = grep { defined $H->[$_][3] && $H->[$_][3] eq $anchor } @cand;
            # No label match: fall back to a WHOLE-TOKEN prefix match — the
            # heading text must START with the anchor, immediately followed by
            # a token boundary (`.`, whitespace, `)`, `-`, or end-of-string).
            # That is still enough to turn architecture.md's `§4d` (headings
            # "4d-bis. …" and "4d-tris. …", labels "4d-bis"/"4d-tris", neither
            # an exact match) into "ambiguous" rather than a bare "not found" —
            # both headings' text literally starts with "4d-". It is NOT a bare
            # substring match: a heading renamed from `3e.` to `13e.` no longer
            # STARTS with "3e" (it starts with "13e"), so §3e citations into it
            # correctly fail instead of matching "3e" wherever it happens to
            # occur inside "13e". A prior substring-anywhere fallback let a
            # heading rename silently invalidate every citation pointing at it
            # without CI noticing.
            @hit = grep { $H->[$_][2] =~ /\A\Q$anchor\E(?:[.\s)-]|\z)/ } @cand unless @hit;
        } else {
            @hit = grep { index($H->[$_][2], $anchor) >= 0 } @cand;
            if (@hit > 1) {
                my @exact = grep { $H->[$_][2] eq $anchor } @hit;
                @hit = @exact if @exact;
            }
        }
    }

    if (@hit == 1) { my ($f, $t) = extent($d, $hit[0]); return ($f, $t, undef, undef) }
    if (@hit > 1) {
        my $where = join ', ', map { 'line ' . ($H->[$_][0] + 1) } @hit;
        return (undef, undef, sprintf('ambiguous section anchor — matches %d headings (%s)', scalar @hit, $where), 'ambiguous-heading');
    }

    return (undef, undef, 'section anchor matches no heading', 'anchor-not-found')
        if $no_literal;

    # A bare anchor names a heading NUMBER and nothing else (see the `bare`
    # branch above) — so once the target document has headings to check a bare
    # anchor against and none matched, that is a genuine miss, not license to
    # fall through to raw-text search below. Without this, the literal
    # fallback re-opens the exact hole the whole-token prefix match above was
    # written to close: a heading renamed from `3e.` to `13e.` still contains
    # "3e" as a substring of its OWN raw source line ("### 13e. …"), so a
    # naive whole-file search can re-validate a `§3e` citation against the
    # renamed heading's own line whenever that happens to be the only
    # occurrence of "3e" left in the file — silently reproducing the bug this
    # fix exists to close. Quoted/backtick anchors (`kind ne 'bare'`) are
    # unaffected: those legitimately mean "this literal text", headings or not.
    return (undef, undef, 'section anchor matches no heading', 'anchor-not-found')
        if $kind eq 'bare' && @cand;

    # No heading matched (or the target has no headings at all): fall back to a
    # whole-file UNIQUE literal, with a 40-line window as the section extent.
    my $L = $d->{lines};
    my @occ = grep { !$excl->{$_} && index($L->[$_], $anchor) >= 0 } 0 .. $#$L;
    if (@occ > 1) {
        my @whole = grep { $L->[$_] =~ /\A\s*\Q$anchor\E\s*\z/ } @occ;
        @occ = @whole if @whole == 1;
    }
    return (undef, undef, 'section anchor not found', 'anchor-not-found') if @occ == 0;
    if (@occ > 1) {
        return (undef, undef, sprintf('ambiguous anchor — %d occurrences and no heading match', scalar @occ), 'ambiguous-literal');
    }
    my $to = $occ[0] + 40; $to = $#$L if $to > $#$L;
    return ($occ[0], $to, undef, undef);
}

# ---------------------------------------------------------------- reporting
my @problems;

# $class is a stable, machine-readable code for WHAT went wrong. It is half of
# the baseline key (the other half is the citation text), so it must never
# contain a line number, a path, or anything else that changes when a document
# is edited — that is the whole point. $ident is the citation text itself: the
# `§anchor "phrase"` for Rule A, the matched literal for Rule B.
sub problem {
    my ($file, $line, $why, $cite, $class, $ident) = @_;
    $ident = defined $ident ? $ident : (defined $cite ? $cite : '');
    $ident =~ s/\s+/ /g;                 # tabs would break the TSV baseline
    $ident =~ s/\A\s+|\s+\z//g;
    push @problems, {
        file => $file, line => $line, why => $why, cite => $cite,
        class => $class, ident => $ident,
    };
}

# Every `§` skipped because no doc-ref was in scope. Not failures — see the
# long comment at the skip site — but counted so the coverage this guard does
# NOT have stays a visible number instead of an invisible one.
my @unbound;

sub report_unbound {
    printf "\n--- unbound %s-marks (%d; not citations to this guard, listed so the gap stays visible) ---\n",
        $SECT, scalar @unbound;
    for my $u (@unbound) { printf "%s:%d: %s\n", $u->{file}, $u->{line}, $u->{cite} }
    print "--- end unbound ---\n\n";
}

# ---------------------------------------------------------------- Rule B
# Each entry: [ name, regex, hint, class ].
my @BAN = (
    [ 'line-number citation into a living document',
      qr{[A-Za-z0-9_.-]+\.(?:md|sql):\d+},
      'cite the section instead: `path/to/doc.md` §"Heading text" "phrase"',
      'doc-colon-line' ],
    [ 'line-number citation into a living document',
      qr{(?<![A-Za-z0-9_-])(?:lines?|ln)[ \t]+\d\d?\d?\d?\d?(?!\d)},
      'cite the section instead: `path/to/doc.md` §"Heading text" "phrase"',
      'prose-line-number' ],
    [ 'bare line number after § (§ means SECTION, never a line)',
      qr{\Q$SECT\E\s?\d\d\d+},
      'a heading is never named "946-956" — use §"Heading text"',
      'section-marker-line-number' ],
    [ 'dead alias:number citation form',
      qr{(?<![A-Za-z0-9_])(?:COMPAT|SPIKE|FINDINGS|CHEATSHEET):\d+},
      'the bare-alias-colon-number form is banned outright',
      'alias-colon-line' ],
    [ 'GitHub #L line anchor',
      qr{\#L\d+(?:-L?\d+)?},
      'line anchors rot the same way; cite the section',
      'github-line-anchor' ],
);

# "command line 5" is prose, not a citation. Every other idiom (newline,
# multi-line, deadline, pipeline, line-height, …) is already excluded by the
# non-word lookbehind above. `baseline` is deliberately NOT excluded: it would
# swallow the legitimate `schema-baseline.txt lines 119-529` family.
my $IDIOM_RE = qr{(?:command|cmd|shell|kernel|boot)[ \t]+(?:lines?)[ \t]+\d};

sub rule_b {
    my ($file, $lineno, $text) = @_;
    return if $text =~ $FROZEN_RE;       # frozen target named on this line
    for my $b (@BAN) {
        next unless $text =~ $b->[1];
        my $m = $&;
        next if $b->[0] =~ /living document/ && $text =~ $IDIOM_RE;
        problem($file, $lineno, "$b->[0]: `$m` — $b->[2]", undef, $b->[3], $m);
        return;                          # one diagnostic per line is enough
    }
}

# ---------------------------------------------------------------- Rule A
sub comment_class {
    my ($t, $is_md) = @_;
    return ''     unless $t =~ /\S/;
    return 'md'   if $is_md;
    $t =~ s/\A\s+//;
    return 'slash' if $t =~ m{\A//};
    return 'star'  if $t =~ m{\A[*]} || $t =~ m{\A/\*};
    return 'dash'  if $t =~ m{\A--};
    return 'hash'  if $t =~ m{\A\#};
    return 'code';
}

for my $file (@scan) {
    my (@allow_a, @allow_b);
    for my $a (@allow) {
        next unless $file =~ $a->[0];
        push @allow_a, $a->[2] if index($a->[1], 'A') >= 0;
        push @allow_b, $a->[2] if index($a->[1], 'B') >= 0;
    }

    open my $fh, '<', "$ROOT/$file" or next;
    my @L = <$fh>;
    close $fh;
    chomp @L;

    my $is_md = ($file =~ /\.md\z/i) ? 1 : 0;

    # Block ids: a doc-ref only carries to a later `§` inside the same
    # contiguous comment block (or, in Markdown, the same paragraph).
    #
    # A Markdown TABLE ROW is its own block. Without this, a table with no
    # blank lines between rows is ONE paragraph, so a doc-ref named in one
    # cell hijacks every later `§` in unrelated rows — the real defect at
    # docs/legacy-spike/findings.md, where three "See §3i."-style
    # SELF-references eight rows below a "COMPAT_CHEATSHEET §7.4" cell were
    # resolved against the cheatsheet and failed there. Cells are
    # self-contained; a citation is never deliberately split across rows.
    my (@block, $prev, $bid);
    $prev = "\0"; $bid = 0;
    for my $i (0 .. $#L) {
        my $c = comment_class($L[$i], $is_md);
        my $row = ($is_md && $L[$i] =~ /^\s*\|/) ? 1 : 0;
        $bid++ if $c ne $prev || $c eq '' || $c eq 'code' || $row;
        $block[$i] = ($c eq '' || $c eq 'code') ? -1 : $bid;
        $prev = $row ? "\0" : $c;    # a row never continues into the next line
    }

    my ($last_doc, $last_tok, $last_line, $last_block);

    LINE: for my $i (0 .. $#L) {
        my $text   = $L[$i];
        my $lineno = $i + 1;

        # Rule-scoped exemptions. An entry that only justifies "this `line 2` is
        # a printed address line" must not also switch off anchor resolution.
        my $skip_a = 0;
        my $skip_b = 0;
        for my $re (@allow_a) { if ($text =~ $re) { $skip_a = 1; last } }
        for my $re (@allow_b) { if ($text =~ $re) { $skip_b = 1; last } }

        rule_b($file, $lineno, $text) unless $skip_b;

        # A Rule-A-exempt line is not a citation at all, so it must not seed the
        # continuation rule either — same behaviour the old blanket `next LINE`
        # had for these entries.
        next LINE if $skip_a;

        my $pos = 0;
        while ((my $sp = index($text, $SECT, $pos)) >= 0) {
            my $prefix = substr($text, 0, $sp);
            my $rest   = substr($text, $sp + length $SECT);
            $pos = $sp + length $SECT;

            # ---- anchor
            my $lead = ($rest =~ s/\A[ ]//) ? 1 : 0;
            my ($anchor, $kind, $used);
            if    ($rest =~ /\A"([^"]+)"/)  { ($anchor, $kind, $used) = ($1, 'quoted', length $&) }
            elsif ($rest =~ /\A`([^`]+)`/)  { ($anchor, $kind, $used) = ($1, 'quoted', length $&) }
            elsif ($rest =~ /\A([0-9A-Za-z][0-9A-Za-z._-]*)/) {
                ($anchor, $kind, $used) = ($1, 'bare', length $&);
                $anchor =~ s/[._-]+\z//;   # §7.4. §5-§6 §3.6d,
            }
            next unless defined $anchor && length $anchor;
            $pos += $lead + $used;

            # ---- optional phrase, immediately after the anchor.
            # If the anchor ENDS the line, the phrase may open the next line of
            # the same comment block — Rust doc comments wrap at ~100 columns
            # and a silently-unchecked phrase is the one failure mode this
            # guard must not have.
            my $phrase;
            my $after = substr($rest, $used);
            if ($after =~ /\A[ ]?"([^"]+)"/) { $phrase = $1 }
            elsif ($after =~ /\A\s*\z/ && $i < $#L
                   && $block[$i] >= 0 && $block[ $i + 1 ] == $block[$i]) {
                (my $nxt = $L[ $i + 1 ]) =~ s{\A\s*(?://[/!]?|\#|--|[*])?\s*}{};
                if ($nxt =~ /\A"([^"]+)"/) { $phrase = $1 }
            }

            my $cite = $SECT . ($kind eq 'quoted' ? "\"$anchor\"" : $anchor)
                     . (defined $phrase ? " \"$phrase\"" : '');

            # ---- `RFC <n> §<n>` is an IETF standard, never a repo document.
            # This is belt-and-braces over the continuation cap, and it is NOT
            # hypothetical: hotel-backend/src/outbox/idempotency.rs names
            # `docs/architecture.md` on line 3 and writes `RFC 4122 §4.3` on
            # line 14 of the SAME `//!` block — a distance of 11, one line
            # outside the cap. Adding a single sentence to that paragraph would
            # pull the RFC section marker into architecture.md's continuation
            # range and fail it as "§4.3 not found in docs/architecture.md",
            # which is the exact false-positive class this guard was corrected
            # to stop emitting. (Measured: removing the cap entirely does fail
            # both RFC sites, so the cap is load-bearing and stays.)
            #
            # Zero risk of hiding a real defect: no RFC is a git-tracked file
            # here, so a `§` introduced by `RFC <number>` can never be a
            # citation this script could resolve. Deliberately anchored to the
            # RFC idiom alone rather than generalised to "any external
            # standard" — a broader rule would start swallowing real citations.
            if ($prefix =~ /(?:RFC|rfc)[ \t]*\d+[ \t]*\z/) {
                push @unbound, { file => $file, line => $lineno, cite => $cite };
                next;
            }

            # ---- which document?
            my ($path, $tok) = docref_in($prefix);
            if (defined $tok && !defined $path) {
                # Resolution is against `git ls-files`, deliberately: that is
                # what CI will see. A brand-new document that exists on disk but
                # has not been staged yet fails here, and "not a git-tracked
                # file" alone reads like a typo. Say which it is.
                my $hint = ($tok !~ /\AADR /  && -e "$ROOT/$tok")
                    ? ' — the file EXISTS on disk but is NOT tracked by git yet, so `git add` it; this guard resolves citations against `git ls-files`, the same view CI has'
                    : ' and is not a registered alias';
                problem($file, $lineno, "target not found: `$tok` is not a git-tracked file$hint", $cite, 'target-not-found', "$tok $cite");
                next;
            }
            if (!defined $tok) {
                # Continuation: nearest preceding doc-ref, same block, <=10 lines.
                if (defined $last_doc && defined $last_block
                    && $last_block >= 0 && $block[$i] == $last_block
                    && $lineno - $last_line <= 10) {
                    $path = $last_doc;
                    $tok  = $last_tok;
                }
                elsif ($is_md) {
                    $path = $file;       # self-reference inside a document
                    $tok  = $file;
                }
                else {
                    # NOT A CITATION. A doc-ref is the PRECONDITION for checking
                    # a `§`, never something to complain about the absence of.
                    # `§` is also how this tree writes a section of an external
                    # standard (`RFC 4122 §4.3`), a section of a CODE file's doc
                    # comment (`writeback recipe checkout.rs §3e` — and the
                    # header above states outright that code files are not
                    # documents), a section of a printed `.html` checklist, and
                    # a work-item label in an off-repo plan (`wave-4 §A3.1`).
                    # None of those name a document this guard could ever
                    # resolve, so reporting them was the guard being wrong.
                    #
                    # This narrows Rule A's reach and nothing else. The two
                    # properties the guard exists for are untouched: Rule B
                    # still bans living-document line numbers EVERYWHERE, and a
                    # `§` that DOES name a document still hard-fails when that
                    # document or heading is missing (the branch above).
                    #
                    # The cost is real and deliberately made visible rather than
                    # hidden: some of these are genuine citations whose doc-ref
                    # merely sits outside the continuation window, and they are
                    # now unverified. `--report-unbound` lists every one, and
                    # the PASS line carries the count so it cannot quietly grow.
                    push @unbound, { file => $file, line => $lineno, cite => $cite };
                    next;
                }
            }

            my $d = load_doc($path);
            if (!defined $d) {
                problem($file, $lineno, "target not found: $path", $cite, 'target-not-found', "$path $cite");
                next;
            }

            # ---- A CITATION MAY NEVER SATISFY ITSELF.
            #
            # When the target IS the citing file — a Markdown self-reference, or
            # a document that names its own path — the citation's own text is
            # part of the document being searched. The whole-file unique-literal
            # fallback then finds the anchor on the CITING LINE and reports a
            # clean resolve, so `see §"Heading That Never Existed"` validated
            # itself. Four live sites in this tree passed exactly that way
            # (architecture.md §3a, the 2026-06-11 audit's §3.13,
            # legacy-spike/tables.md §3j, writeback-audit-2026-05-12 §3k — every
            # one of them a real citation into a DIFFERENT document, resolved
            # against the wrong file and passing on its own line). A guard that
            # green-lights a fabricated anchor manufactures false confidence,
            # which is worse than having no guard.
            #
            # Two locks, both needed:
            #   1. %excl removes the citing line and the rest of its
            #      paragraph/comment block from every kind of evidence — heading
            #      matches, the literal fallback, and the phrase body.
            #   2. $no_literal drops the literal fallback altogether for a
            #      self-citation into a document that has headings. `§` means
            #      SECTION; inside your own document the only honest evidence is
            #      "this names one of my headings". MEASURED before shipping:
            #      106 self-references in this tree resolve by heading and keep
            #      working; ZERO resolve by a literal somewhere else in the file;
            #      the 4 that resolved by literal all resolved on their own line.
            #      So the fallback bought nothing here and cost soundness.
            # The fallback is untouched for cross-document citations, where it
            # earns its keep on targets with no headings (SCHEMA.sql, migrations).
            my $self_cite = ($path eq $file) ? 1 : 0;
            my %excl;
            if ($self_cite) {
                $excl{$i} = 1;
                if (defined $block[$i] && $block[$i] >= 0) {
                    for (my $j = $i - 1; $j >= 0 && $block[$j] == $block[$i]; $j--) { $excl{$j} = 1 }
                    for (my $j = $i + 1; $j <= $#L && $block[$j] == $block[$i]; $j++) { $excl{$j} = 1 }
                }
            }
            my $no_literal = ($self_cite && @{ $d->{head} }) ? 1 : 0;

            my ($from, $to, $err, $eclass) = resolve_anchor($d, $anchor, $kind, \%excl, $no_literal);
            if (defined $err) {
                $eclass = "self-$eclass" if $self_cite;
                problem($file, $lineno,
                    $self_cite
                      ? "$err in $path — this is a SELF-reference (no document named before the $SECT), so it must name one of this file's own headings; if it points at another document, name that document on this line"
                      : "$err in $path",
                    $cite, $eclass, "$path $cite");
                next;
            }

            if (defined $phrase) {
                my $body = join "\n", map { $d->{lines}[$_] }
                           grep { !$excl{$_} } $from .. $to;
                if (index($body, $phrase) < 0) {
                    problem($file, $lineno,
                        sprintf('phrase not found in section: "%s" is absent from %s lines %d-%d%s',
                                $phrase, $path, $from + 1, $to + 1,
                                ($self_cite ? ' (the citing line itself does not count)' : '')),
                        $cite, ($self_cite ? 'self-phrase-not-found' : 'phrase-not-found'), "$path $cite");
                }
            }
        }

        # Remember the last doc-ref named anywhere on this line, for the
        # continuation rule.
        my ($d2, $t2) = docref_in($text);
        if (defined $d2) {
            ($last_doc, $last_tok, $last_line, $last_block) = ($d2, $t2, $lineno, $block[$i]);
        }
    }
}

# ---------------------------------------------------------------- baseline
# A checked-in ledger of citations that are KNOWN to be broken and are tolerated
# for now — pre-existing debt in areas a focused branch never claimed. It is a
# ledger, not a mute: see the file's own header for the contract.
#
# KEY = path + class + citation-identity, NEVER a line number. A baseline keyed
# on line numbers would rot the instant a file was edited, which is the exact
# failure this whole branch exists to delete; it would also silently re-point at
# whatever citation happened to slide into that line.
my $BASELINE = $ENV{DA_BASELINE} || '';
my $MODE     = $ENV{DA_MODE} || 'check';

sub bkey { my ($p) = @_; return join "\t", $p->{file}, $p->{class} // '?', $p->{ident} }

# A row is only a ledger entry if it is CLASSIFIED and JUSTIFIED. Before this,
# `--update-baseline` emitted `unclassified` + `TODO: state why this is
# tolerated` and nothing ever rejected it, so the ledger could grow silently by
# regenerating it — the mute button the design says it must not be. These are
# exit-2 (configuration) failures, not exit-1 citation failures: the ledger
# itself is broken, so the guard cannot say anything trustworthy about the tree.
my %VALID_SCOPE = ('in-scope' => 1, 'out-of-scope' => 1);
my $MIN_REASON  = 20;    # a reason is a sentence, not a word

sub bad_row {
    my ($ln, $msg) = @_;
    # In write mode STDOUT *is* the new ledger — diagnostics must never land in
    # it. In check mode they go to STDOUT so GitHub renders the annotation.
    my $fh = ($MODE eq 'write') ? \*STDERR : \*STDOUT;
    printf {$fh} "::error file=%s,line=%d::%s\n", $BASELINE, $ln, $msg;
    return 1;
}

sub parse_baseline {
    # $label: 'baseline' or 'base version'. $validate is off for the base
    # version: those rows are already merged history, and re-litigating them
    # would fail a build for a row nobody in this change touched.
    my ($path_to, $label, $validate) = @_;
    my (%rows, @order, $bad);
    open my $bfh, '<', $path_to or return (\%rows, \@order, 0);
    my $ln = 0;
    while (my $l = <$bfh>) {
        $ln++;
        chomp $l;
        next unless $l =~ /\S/;
        next if $l =~ /^\s*#/;
        my ($path, $count, $class, $ident, $scope, $reason, $info) = split /\t/, $l, 7;
        unless (defined $reason && defined $ident && defined $class && defined $count
                && length $path && length $class && length $ident
                && $count =~ /\A\d+\z/ && $count > 0) {
            # A malformed row in the BASE version is history, not this change's
            # problem — and reporting it would quote the wrong file and line.
            $bad += bad_row($ln, "malformed $label entry (want: path<TAB>count<TAB>class<TAB>citation<TAB>scope<TAB>reason, count >= 1)")
                if $validate;
            next;
        }
        $scope  = defined $scope  ? $scope  : '';
        $reason = defined $reason ? $reason : '';
        $reason =~ s/\A\s+|\s+\z//g;
        if (!$validate) {
            my $k0 = join "\t", $path, $class, $ident;
            next if exists $rows{$k0};   # first wins; the base version is history
            $rows{$k0} = { line => $ln, count => $count + 0, path => $path, class => $class,
                           ident => $ident, scope => $scope, reason => $reason, seen => 0 };
            push @order, $k0;
            next;
        }
        unless ($VALID_SCOPE{$scope}) {
            $bad += bad_row($ln, sprintf(
                "%s row is not classified: scope `%s` — every row must be `in-scope` or `out-of-scope`. `unclassified` is what --update-baseline writes for a row nobody has triaged; triage it or delete the row.",
                ucfirst $label, $scope));
        }
        if ($reason =~ /\b(?:TODO|FIXME|XXX)\b/i || $reason =~ /\A(?:-|n\/a|tbd|\?)\z/i) {
            $bad += bad_row($ln, sprintf(
                "%s row has a placeholder reason (`%s`). A tolerated failure must say WHY it is tolerated and what would fix it; a TODO is not a justification.",
                ucfirst $label, $reason));
        }
        elsif (length $reason < $MIN_REASON) {
            $bad += bad_row($ln, sprintf(
                "%s row's reason is %d character(s) (`%s`); at least %d are required. State why the failure is tolerated and what the cure is.",
                ucfirst $label, length $reason, $reason, $MIN_REASON));
        }
        my $k = join "\t", $path, $class, $ident;
        if (exists $rows{$k}) {
            $bad += bad_row($ln, "duplicate $label entry for $ident — merge it into one row and raise the count");
            next;
        }
        $rows{$k} = { line => $ln, count => $count + 0, path => $path, class => $class,
                      ident => $ident, scope => $scope, reason => $reason, seen => 0 };
        push @order, $k;
    }
    close $bfh;
    return (\%rows, \@order, $bad || 0);
}

my (%bl, @bl_order);
if (length $BASELINE && -e $BASELINE) {
    my ($rows, $order, $bad) = parse_baseline($BASELINE, 'baseline', 1);
    if ($bad && $MODE ne 'write') {
        printf "::error::check-doc-anchors: %s is not a valid ledger (%d bad row(s) above). Fix the rows — a ledger that is not classified and justified is a mute button, not a record.\n",
            $BASELINE, $bad;
        exit 2;
    }
    %bl = %$rows;
    @bl_order = @$order;
}

# Actual occurrences, grouped by key.
my (%actual, %actual_lines);
for my $p (@problems) {
    my $k = bkey($p);
    push @{ $actual{$k} }, $p;
    push @{ $actual_lines{$k} }, $p->{line};
}

# ---------------------------------------------------------------- --update-baseline
if ($MODE eq 'write') {
    # This header is regenerated verbatim on every --update-baseline, so the
    # file round-trips to itself and the prose can never drift from the code
    # that enforces it. Edit it HERE, not in the .tsv.
    print <<'HDR';
# doc-anchors baseline — citations this guard KNOWS are broken and tolerates.
#
# WHY THIS FILE EXISTS. `scripts/check-doc-anchors.sh` is a NEW lint landing on
# an existing codebase. A new lint lands with a recorded baseline and burns down;
# it does not hold a focused branch hostage to a tree-wide sweep. Everything
# listed here is pre-existing debt in an area the doc-anchor-hardening branch
# never claimed. Nothing here is a citation this branch wrote.
#
# CONTRACT — enforced by the guard, not by convention:
#   * The guard exits 0 when every remaining failure is listed here.
#   * A failure NOT listed here still fails the build. This is a ledger, not a mute.
#   * A listed failure occurring MORE times than its count FAILS the build, so the
#     debt cannot grow by copy-paste. The fix is to remove the new occurrence,
#     never to raise the count.
#   * EVERY ROW MUST BE CLASSIFIED AND JUSTIFIED. `scope` must be `in-scope` or
#     `out-of-scope`, and `reason` must be a real sentence (>= 20 chars, and a
#     TODO/FIXME/XXX placeholder is refused outright). A row that fails either
#     test makes the guard EXIT 2: an untriaged row is not a ledger entry, it is
#     a mute button, and a ledger that mutes silently is worse than no ledger.
#     This is why `--update-baseline` writes `unclassified` + a TODO reason for
#     any row it has not seen before — the regenerated file does not pass until a
#     human has decided each new row is tolerable and written down why.
#   * RAISING A COUNT IS AN ERROR, not an edit. The guard compares this file
#     against its version at the merge base; a row whose count went up fails the
#     build, quoting the before and after. (When no merge base is available — a
#     shallow CI checkout, or a run outside a git tree — the run says so out loud
#     rather than pretending the comparison happened.)
#   * ADDING a row is allowed and never quiet: every row that is new since the
#     merge base is printed with its scope and reason on every run.
#   * A listed failure that is FIXED (or occurs fewer times) prints a WARNING
#     asking for the row to be deleted, so the file shrinks toward zero instead of
#     rotting. Warning and not error, deliberately: paying down debt must never
#     turn someone's build red, and this branch landed while a sibling package was
#     still converting citations in the same tree — a hard failure there would
#     have made every partial fix look like a regression.
#
# HOW TO REGENERATE:  ./scripts/check-doc-anchors.sh --update-baseline
#   (preserves the scope and reason of every row it already knows; new rows get
#   `unclassified` and a TODO reason, both of which the guard REFUSES, so a new
#   row cannot be added silently — triage it or delete it.)
#
# COLUMNS, tab-separated:
#   1 path      the citing file
#   2 count     how many times this exact failure occurs in that file
#   3 class     stable diagnostic code — never contains a line number
#   4 citation  the resolved target + the citation text: the identity that
#               survives the file being edited
#   5 scope     out-of-scope | in-scope   (`unclassified` is REJECTED)
#   6 reason    why it is tolerated, and what would fix it
#   7 lines=    INFORMATIONAL ONLY, for humans navigating to the site. Never
#               matched against; stale numbers here are harmless by construction.
#
# THE KEY IS (path, class, citation) AND DELIBERATELY CONTAINS NO LINE NUMBER.
# Keying a baseline on line numbers would reproduce exactly the rot this guard
# was built to delete: edit the file, and every entry either points at the wrong
# citation or stops matching. Keyed on the citation TEXT, an entry follows its
# citation around the file and cannot be inherited by a different one.
#
HDR
    my (%emitted, @needs_triage);
    for my $k (sort keys %actual) {
        my $p  = $actual{$k}[0];
        my $n  = scalar @{ $actual{$k} };
        my $old = $bl{$k};
        my $scope  = $old ? $old->{scope}  : 'unclassified';
        my $reason = $old ? $old->{reason} : 'TODO: state why this is tolerated and what would fix it';
        push @needs_triage, $p unless $old && $VALID_SCOPE{$scope}
                                   && $reason !~ /\b(?:TODO|FIXME|XXX)\b/i
                                   && length $reason >= $MIN_REASON;
        printf "%s\t%d\t%s\t%s\t%s\t%s\tlines=%s\n",
            $p->{file}, $n, $p->{class} // '?', $p->{ident},
            $scope, $reason, join(',', @{ $actual_lines{$k} });
        $emitted{$k} = 1;
    }
    printf STDERR "check-doc-anchors: baseline rewritten — %d entr(ies), %d occurrence(s).\n",
        scalar keys %emitted, scalar @problems;
    for my $k (@bl_order) {
        next if $emitted{$k};
        my $b = $bl{$k};
        printf STDERR "  dropped (now passing): %s  %s\n", $b->{path}, $b->{ident};
    }
    # A regenerated ledger is a DRAFT until every new row is triaged. The guard
    # refuses an `unclassified` scope and a TODO reason (see parse_baseline), so
    # these rows fail the very next run — that is deliberate. Regenerating must
    # never be a way to make a failure disappear without a human deciding it is
    # tolerable and writing down why.
    if (@needs_triage) {
        printf STDERR "\ncheck-doc-anchors: %d row(s) need triage before this ledger is valid.\n",
            scalar @needs_triage;
        printf STDERR "  Set scope (in-scope|out-of-scope) and replace the TODO reason on each of:\n";
        for my $p (@needs_triage) {
            printf STDERR "    %s:%d  %s  %s\n", $p->{file}, $p->{line}, $p->{class} // '?', $p->{ident};
        }
        printf STDERR "  Until then `%s` EXITS 2 — an untriaged row is not a ledger entry.\n", 'check-doc-anchors.sh';
    }
    exit 0;
}

# ------------------------------------------------- ledger growth vs the base
# Row validation above makes an untriaged row impossible. This makes a QUIET row
# impossible: every difference from the merge-base version of the ledger is
# named on the run, and the one change the ledger's own contract forbids —
# raising a row's count so a copy-pasted failure stops failing — is an error,
# not a note.
my $BASE_FILE  = $ENV{DA_BASELINE_BASE} || '';
my $BASE_STATE = $ENV{DA_BASE_STATE}    || 'unavailable';
my $BASE_DESC  = $ENV{DA_BASE_DESC}     || '';
my (@ledger_notes, @ledger_errors);

if ($BASE_STATE eq 'available' && length $BASE_FILE && -e $BASE_FILE) {
    my ($base, $border) = parse_baseline($BASE_FILE, 'base version', 0);
    my (@added, @raised, @lowered, @removed);
    for my $k (@bl_order) {
        my $b = $base->{$k};
        if (!$b) { push @added, $bl{$k}; next }
        push @raised,  [ $bl{$k}, $b ] if $bl{$k}{count} > $b->{count};
        push @lowered, [ $bl{$k}, $b ] if $bl{$k}{count} < $b->{count};
    }
    for my $k (@$border) { push @removed, $base->{$k} unless exists $bl{$k} }

    for my $r (@raised) {
        push @ledger_errors, sprintf(
            "::error file=%s,line=%d::ledger count RAISED: `%s` in %s went %d -> %d since %s. The count is not a dial. A baselined failure occurring more often means a new occurrence was written — remove it; do not widen the tolerance.",
            $BASELINE, $r->[0]{line}, $r->[0]{ident}, $r->[0]{path},
            $r->[1]{count}, $r->[0]{count}, $BASE_DESC);
    }
    if (@added) {
        push @ledger_notes, sprintf(
            "::warning file=%s,line=%d::the ledger GREW by %d row(s) since %s. Growth is allowed but never quiet — each row below is a failure someone decided to tolerate:",
            $BASELINE, $added[0]{line}, scalar @added, $BASE_DESC);
        push @ledger_notes, sprintf("::warning file=%s,line=%d::  + [%s] %s  %s — %s",
            $BASELINE, $_->{line}, $_->{scope}, $_->{path}, $_->{ident}, $_->{reason}) for @added;
    }
    push @ledger_notes, sprintf(
        "::notice::check-doc-anchors: ledger shrank by %d row(s) since %s — debt paid down.",
        scalar @removed, $BASE_DESC) if @removed;
    push @ledger_notes, sprintf(
        "::notice::check-doc-anchors: %d ledger row(s) lowered their count since %s — debt paid down.",
        scalar @lowered, $BASE_DESC) if @lowered;
}
elsif ($BASE_STATE eq 'absent-at-base') {
    push @ledger_notes, sprintf(
        "::notice::check-doc-anchors: %s does not exist at %s — this change INTRODUCES the ledger with %d row(s). Every one is validated (classified scope, stated reason); growth is compared from here on.",
        $BASELINE, ($BASE_DESC || 'the merge base'), scalar @bl_order)
        if @bl_order;
}
else {
    # Say so out loud. A growth check nobody can tell has been skipped is worse
    # than no growth check: it reads as a guarantee that is not being made.
    push @ledger_notes, sprintf(
        "::notice::check-doc-anchors: ledger growth NOT compared against a base version (%s). Row-level validation still applied to all %d row(s).",
        ($BASE_DESC || $BASE_STATE), scalar @bl_order)
        if @bl_order;
}

# ---------------------------------------------------------------- output
my (@new, @tolerated, @warnings);
for my $k (sort keys %actual) {
    my $n = scalar @{ $actual{$k} };
    my $b = $bl{$k};
    if (!$b) { push @new, @{ $actual{$k} }; next }
    $b->{seen} = $n;
    if ($n > $b->{count}) {
        # The debt grew. We cannot tell WHICH occurrence is the new one, so all
        # of them are reported — the fix is to remove the new one, not to bump
        # the count.
        push @new, @{ $actual{$k} };
        push @warnings, sprintf(
            "::error file=%s,line=%d::baseline entry grew: `%s` now occurs %d time(s) in %s, baseline records %d. Do NOT raise the count — remove the new occurrence.",
            $BASELINE, $b->{line}, $b->{ident}, $n, $b->{path}, $b->{count});
        next;
    }
    push @tolerated, @{ $actual{$k} };
    if ($n < $b->{count}) {
        push @warnings, sprintf(
            "::warning file=%s,line=%d::baseline entry shrank: `%s` now occurs %d time(s) in %s (baseline says %d). Lower the count so it cannot grow back.",
            $BASELINE, $b->{line}, $b->{ident}, $n, $b->{path}, $b->{count});
    }
}

# Entries that no longer fail at all — the debt was paid. Say so and ask for the
# row to be deleted, so the file shrinks toward zero instead of rotting.
my @fixed = grep { $bl{$_}{seen} == 0 } @bl_order;
for my $k (@fixed) {
    my $b = $bl{$k};
    push @warnings, sprintf(
        "::warning file=%s,line=%d::baselined citation is FIXED: `%s` in %s no longer fails. Delete this baseline row (or run --update-baseline).",
        $BASELINE, $b->{line}, $b->{ident}, $b->{path});
}

for my $w (@ledger_notes)  { print "$w\n" }
for my $w (@ledger_errors) { print "$w\n" }
for my $w (@warnings)      { print "$w\n" }

# `--report-unbound` exists to keep the coverage gap VISIBLE, which means it has
# to print when the guard is red — that is when someone is triaging. It used to
# sit after the `exit 1` below, so the one flag whose job is visibility was
# silent in exactly the run that needed it.
report_unbound() if $ENV{DA_REPORT_UNBOUND};

if (@new) {
    my %files;
    for my $p (@new) {
        printf "::error file=%s,line=%d::%s%s\n",
            $p->{file}, $p->{line},
            (defined $p->{cite} ? "$p->{cite} — " : ''),
            $p->{why};
        $files{ $p->{file} } = 1;
    }
    printf "::error::check-doc-anchors: %d citation problem(s) across %d file(s)%s. Convention: `path/to/doc.md` %s\"Heading text\" \"verbatim phrase\" — see the header of scripts/check-doc-anchors.sh.\n",
        scalar @new, scalar keys %files,
        (@tolerated ? sprintf(' (a further %d are baselined in %s)', scalar @tolerated, $BASELINE) : ''),
        $SECT;
    exit 1;
}

if (@ledger_errors) {
    printf "::error::check-doc-anchors: the ledger %s changed in a way its own contract forbids (%d finding(s) above). Every citation in the tree is otherwise accounted for.\n",
        $BASELINE, scalar @ledger_errors;
    exit 1;
}

# ---- green.
if (@tolerated) {
    my %scopes;
    for my $k (sort keys %actual) { next unless $bl{$k}; $scopes{ $bl{$k}{scope} } += $bl{$k}{seen} }
    printf "::warning::check-doc-anchors: %d known-broken citation(s) tolerated from %s (%s). This is recorded debt, not a pass — it must burn down, and it cannot grow.\n",
        scalar @tolerated, $BASELINE,
        join(', ', map { "$_: $scopes{$_}" } sort keys %scopes);
}
if (@unbound) {
    printf "check-doc-anchors: %d %s-mark(s) skipped — no document named, so nothing to verify (RFC sections, code-file doc comments, off-repo plans, and genuine citations whose doc-ref sits outside the continuation window). Run --report-unbound to list them.\n",
        scalar @unbound, $SECT;
}
printf "PASS: check-doc-anchors — %d file(s) scanned, every checked document citation resolves and no living-document line numbers remain.\n",
    scalar @scan;
exit 0;
ENGINE
}

# ----------------------------------------------------------------------
# Self-tests (--self-test).
# ----------------------------------------------------------------------
# expect: pass (exit 0) | fail (exit 1, a citation problem) | error (exit 2, a
# configuration problem — a malformed allowlist or an invalid ledger).
self_test_case() {
    local name="$1" expect="$2" root="$3" needle="${4:-}"
    local out status=0 want
    case "${expect}" in
        pass)  want=0 ;;
        fail)  want=1 ;;
        error) want=2 ;;
        *) echo "[self-test] ${name} FAILED — bad expectation '${expect}'" >&2; return 1 ;;
    esac
    out="$(SCAN_ROOT="${root}" run_check 2>&1)" || status=$?
    if [ "${status}" -ne "${want}" ]; then
        echo "[self-test] ${name} FAILED — expected exit ${want}, got ${status}:" >&2
        echo "${out}" >&2
        return 1
    fi
    # A needle is checked on PASS too: the baseline cases must prove they
    # passed for the RIGHT reason (a tolerated entry, a "now fixed" warning)
    # and not because the guard simply stopped looking.
    if [ -n "${needle}" ] && ! printf '%s' "${out}" | grep -qF -- "${needle}"; then
        echo "[self-test] ${name} FAILED — exit ${status} as expected, but output did not mention '${needle}':" >&2
        echo "${out}" >&2
        return 1
    fi
    echo "[self-test] ${name} OK"
    return 0
}

run_self_tests() {
    local tmp
    tmp="$(mktemp -d)"
    # shellcheck disable=SC2064
    trap "rm -rf '${tmp}'" EXIT

    local root="${tmp}/fx"
    mkdir -p "${root}/docs/legacy-spike/raw/cap-1" "${root}/src"

    write_fixture_doc() {
        cat > "${root}/docs/target.md" <<'MD'
# Fixture target

## 1. First section

Body of the first section.

## 2.4 ID generation patterns

The Pay_no format is R{yyMM}-{4digit} allocated per month.

## 3. Later section

Nothing to see. Mentions R{yyMM}-{4digit} again, outside section 2.4.
MD
    }
    write_fixture_doc

    cat > "${root}/docs/legacy-spike/raw/cap-1/writes.txt" <<'TXT'
2026-04-24 10:23:02  update HT_Customers set [Cust_Name]=...
TXT

    cat > "${root}/src/ok.rs" <<'RS'
//! Fixture source.
//!
//! Pay_no allocation mirrors `docs/target.md` §"2.4 ID generation patterns"
//! "R{yyMM}-{4digit}".
//! Frozen evidence is citable by line: raw/cap-1/writes.txt:1.
fn main() {}
RS

    echo "[self-test] case 1 — anchor + phrase both present → expect PASS"
    self_test_case "case 1" pass "${root}" || return 1

    # Case 2: rename the heading out from under the citation.
    sed 's/## 2.4 ID generation patterns/## 2.4 ID generation/' \
        "${root}/docs/target.md" > "${root}/docs/target.md.new"
    mv "${root}/docs/target.md.new" "${root}/docs/target.md"
    echo "[self-test] case 2 — heading renamed → expect FAIL"
    self_test_case "case 2" fail "${root}" "section anchor not found" || return 1
    write_fixture_doc

    # Case 3: phrase deleted from the cited section but still present elsewhere
    # in the file — proves the phrase check is scoped to the section.
    sed 's/^The Pay_no format is R{yyMM}-{4digit} allocated per month\./The Pay_no format is allocated per month./' \
        "${root}/docs/target.md" > "${root}/docs/target.md.new"
    mv "${root}/docs/target.md.new" "${root}/docs/target.md"
    echo "[self-test] case 3 — phrase left in the file but not in the section → expect FAIL"
    self_test_case "case 3" fail "${root}" "phrase not found in section" || return 1
    write_fixture_doc

    # Case 4: two headings share a leading NUMBER, so the bare `§2.4` cannot
    # pick one. This is docs/architecture.md's real `§4e` defect in miniature
    # (`### 4e. Reconcile …` and `### 4e. Schema fingerprint guard`); the cure
    # is a heading-text anchor, not surgery on the document.
    cat >> "${root}/docs/target.md" <<'MD'

## 2.4 Allocation strategy

Body.
MD
    cat > "${root}/src/ambig.rs" <<'RS'
// Allocation follows `docs/target.md` §2.4 for both formats.
RS
    echo "[self-test] case 4 — bare anchor matches two headings → expect FAIL"
    self_test_case "case 4" fail "${root}" "ambiguous section anchor" || return 1
    rm -f "${root}/src/ambig.rs"
    write_fixture_doc

    # Case 5: a bare line-number citation into a living document.
    cat > "${root}/src/bad.rs" <<'RS'
// See docs/target.md:42 for the allocation rule.
RS
    echo "[self-test] case 5 — living-document line number → expect FAIL"
    self_test_case "case 5" fail "${root}" "line-number citation" || return 1
    rm -f "${root}/src/bad.rs"

    # Case 6: the same shape into frozen capture evidence → allowed.
    cat > "${root}/src/frozen.rs" <<'RS'
// Verbatim from docs/legacy-spike/raw/cap-1/writes.txt:1 (frozen capture).
// Same thing spelled the other way: writes.txt line 1.
RS
    echo "[self-test] case 6 — line number into frozen evidence → expect PASS"
    self_test_case "case 6" pass "${root}" || return 1
    rm -f "${root}/src/frozen.rs"

    # Case 7: doc-ref naming a file that does not exist.
    cat > "${root}/src/missing.rs" <<'RS'
// Per `docs/nope.md` §"1. First section" the answer is 42.
RS
    echo "[self-test] case 7 — unresolvable doc-ref → expect FAIL"
    self_test_case "case 7" fail "${root}" "target not found" || return 1
    rm -f "${root}/src/missing.rs"

    # Case 8: the continuation rule — a `§` on its own line binding to the
    # doc-ref that ended the previous line of the same comment block.
    cat > "${root}/src/cont.rs" <<'RS'
//! Recipes follow docs/target.md
//! §"1. First section" and §"3. Later section".
fn f() {}
RS
    echo "[self-test] case 8 — continuation across a comment block → expect PASS"
    self_test_case "case 8" pass "${root}" || return 1
    rm -f "${root}/src/cont.rs"

    # ------------------------------------------------------------------
    # Case 9: a `§` that names NO document is not a citation. This is the
    # false-positive class that made the guard unshippable: `RFC 4122 §4.3`,
    # `writeback recipe checkout.rs §3e`, a printed `.html` checklist's §5, and
    # work-item labels in off-repo plans (`wave-4 §A3.1`). None name a document
    # this guard could ever resolve.
    cat > "${root}/src/nodoc.rs" <<'RS'
//! Ids are UUIDv4 per RFC 4122 §4.3.
//! Phase 4 wave-4 §A3.1 covers the branch picker; see also §B6.
// The `bin/writeback.rs` doc comment §3 already anticipates this.
RS
    echo "[self-test] case 9 — § with no doc-ref in scope → expect PASS (not a citation)"
    self_test_case "case 9" pass "${root}" "no document named" || return 1
    rm -f "${root}/src/nodoc.rs"

    # Case 10: a Markdown TABLE ROW is its own continuation block. Without that,
    # a doc-ref in one cell hijacks a `§` in an unrelated row several rows later
    # and resolves it against the wrong document. Here row 3's `§1. First
    # section` is a SELF-reference and must resolve inside doc.md, NOT against
    # the `docs/target.md` named two rows above it.
    cat > "${root}/docs/tbl.md" <<'MD'
# Table doc

## 1. First section

Body.

| Item | Note |
|---|---|
| a | See `docs/target.md` §"1. First section". |
| b | Unrelated row. |
| c | See §"1. First section". |
MD
    echo "[self-test] case 10 — table rows do not leak the continuation → expect PASS"
    self_test_case "case 10" pass "${root}" || return 1

    # ------------------------------------------------------------------
    # Baseline cases. The fixture baseline lives at the same repo-relative path
    # the real one does, so these exercise the production code path exactly.
    mkdir -p "${root}/scripts"
    local bl="${root}/${BASELINE_REL}"

    cat > "${root}/src/debt.rs" <<'RS'
// Pre-existing debt: `docs/target.md` §"9. No such section" is broken.
RS

    echo "[self-test] case 11 — a broken citation with NO baseline → expect FAIL"
    self_test_case "case 11" fail "${root}" "section anchor not found" || return 1

    # Case 12: the same failure, now recorded. Tolerated → exit 0, but LOUDLY.
    # The row is CLASSIFIED and JUSTIFIED because the guard now refuses rows that
    # are not (cases 21 and 22 below pin that).
    local fixture_reason='fixture debt: a deliberately broken citation used by the self-test'
    printf 'src/debt.rs\t1\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tout-of-scope\t%s\tlines=1\n' \
        "${fixture_reason}" > "${bl}"
    echo "[self-test] case 12 — the same failure, baselined → expect PASS"
    self_test_case "case 12" pass "${root}" "known-broken citation(s) tolerated" || return 1

    # Case 13: THE PROPERTY THAT MAKES A BASELINE A LEDGER AND NOT A MUTE — a
    # brand-new broken citation, with the old one still baselined, still fails.
    cat > "${root}/src/fresh.rs" <<'RS'
// Brand new: `docs/target.md` §"8. Also missing" was just written.
RS
    echo "[self-test] case 13 — a NEW broken citation alongside the baseline → expect FAIL"
    self_test_case "case 13" fail "${root}" "src/fresh.rs" || return 1
    rm -f "${root}/src/fresh.rs"

    # Case 14: the debt GROWS by copy-paste — a second instance of an already
    # baselined citation in the same file. Count is part of the entry, so this
    # fails too. Raising the count is explicitly not the fix.
    cat >> "${root}/src/debt.rs" <<'RS'
// Copy-pasted: `docs/target.md` §"9. No such section" again.
RS
    echo "[self-test] case 14 — a baselined citation copy-pasted → expect FAIL"
    self_test_case "case 14" fail "${root}" "baseline entry grew" || return 1

    # Case 15: the debt is PAID. The guard stays green but says so and asks for
    # the row to be deleted, so the ledger shrinks to zero instead of rotting.
    rm -f "${root}/src/debt.rs"
    echo "[self-test] case 15 — a baselined citation now fixed → expect PASS + 'remove it'"
    self_test_case "case 15" pass "${root}" "is FIXED" || return 1
    rm -f "${bl}"

    # Case 16: a line number into the off-repo frozen Profiler capture. Same
    # class as the `*.cs` decompile: nothing in this repo can renumber it, so a
    # line number is the only citation form that exists.
    cat > "${root}/src/capture.rs" <<'RS'
// Verbatim from /tmp/legacy-events-full.log (line 3988) — frozen capture.
RS
    echo "[self-test] case 16 — line number into the frozen capture log → expect PASS"
    self_test_case "case 16" pass "${root}" || return 1
    rm -f "${root}/src/capture.rs"

    # Case 17: an RFC section marker is never a repo citation, even when a real
    # doc-ref is named EARLIER IN THE SAME COMMENT BLOCK and well inside the
    # 10-line continuation window. Without the RFC guard the continuation binds
    # `§4.3` to docs/target.md and fails it. This is the real shape of
    # hotel-backend/src/outbox/idempotency.rs, where the doc-ref and the RFC sit
    # 11 lines apart — one line from the cap, i.e. one edit from a red build.
    cat > "${root}/src/rfc.rs" <<'RS'
//! Keys are described in `docs/target.md` §"1. First section".
//! They are UUIDv5 values (RFC 4122 §4.3, name-based hashing with SHA-1)
//! over a tiny payload.
RS
    echo "[self-test] case 17 — RFC § with a doc-ref in continuation range → expect PASS"
    self_test_case "case 17" pass "${root}" || return 1
    rm -f "${root}/src/rfc.rs"

    # ==================================================================
    # SELF-REFERENCE SOUNDNESS. A citation must never be able to satisfy
    # itself. Cases 18-21 are the four corners of that.
    # ==================================================================

    # Case 18: the legitimate shape, which must keep working — a `§` with no
    # doc-ref inside a document, naming one of that document's OWN headings.
    # 106 citations in the real tree resolve exactly this way; the fix for the
    # self-validation hole must not cost any of them.
    cat > "${root}/docs/selfref.md" <<'MD'
# Self-reference fixture

## 1. First section

Body of the first section.

## 2. Second section

Handled the same way as §"1. First section" above.
MD
    echo "[self-test] case 18 — self-reference to a REAL own heading → expect PASS"
    self_test_case "case 18" pass "${root}" || return 1

    # Case 19: THE HOLE. An invented heading name, cited from inside the
    # document. Every occurrence of the anchor text in the file is on the
    # CITING LINE, so the old whole-file unique-literal fallback found it there
    # and reported a clean resolve — the citation validated itself.
    cat > "${root}/docs/selfref.md" <<'MD'
# Self-reference fixture

## 1. First section

Body of the first section.

## 2. Second section

Handled per §"Heading That Never Existed" above.
MD
    echo "[self-test] case 19 — invented self-anchor, text present only on its own line → expect FAIL"
    self_test_case "case 19" fail "${root}" "SELF-reference" || return 1

    # Case 20: the same invention, with the anchor text ALSO written elsewhere
    # in the citing paragraph. Excluding just the one line would not be enough:
    # the whole citing block is barred from being its own evidence.
    cat > "${root}/docs/selfref.md" <<'MD'
# Self-reference fixture

## 1. First section

Body of the first section.

## 2. Second section

The Heading That Never Existed rule is subtle,
so read §"Heading That Never Existed" before changing it.
MD
    echo "[self-test] case 20 — invented self-anchor echoed in its own paragraph → expect FAIL"
    self_test_case "case 20" fail "${root}" "matches no heading" || return 1

    # Case 21: the property the branch exists for, applied to self-references —
    # rename the heading and the citation goes red. Same shape as case 2, but
    # for the citation form that used to be unfalsifiable.
    cat > "${root}/docs/selfref.md" <<'MD'
# Self-reference fixture

## 1. First heading, since renamed

Body of the first section.

## 2. Second section

Handled the same way as §"1. First section" above.
MD
    echo "[self-test] case 21 — renaming a heading breaks its self-reference → expect FAIL"
    self_test_case "case 21" fail "${root}" "matches no heading" || return 1
    rm -f "${root}/docs/selfref.md"

    # ==================================================================
    # THE LEDGER IS A LEDGER. Cases 22-25.
    # ==================================================================

    cat > "${root}/src/debt.rs" <<'RS'
// Pre-existing debt: `docs/target.md` §"9. No such section" is broken.
RS

    # Case 22: `--update-baseline` writes `unclassified` for a row nobody has
    # triaged. That row must NOT quietly suppress the failure it names. Exit 2
    # (configuration failure) — the ledger itself is not usable.
    printf 'src/debt.rs\t1\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tunclassified\t%s\tlines=1\n' \
        "${fixture_reason}" > "${bl}"
    echo "[self-test] case 22 — ledger row with scope 'unclassified' → expect ERROR (exit 2)"
    self_test_case "case 22" error "${root}" "is not classified" || return 1

    # Case 23: same for the TODO reason the writer emits. A placeholder is not a
    # justification, and a row without one is a mute button.
    printf 'src/debt.rs\t1\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tout-of-scope\tTODO: state why this is tolerated and what would fix it\tlines=1\n' > "${bl}"
    echo "[self-test] case 23 — ledger row with a TODO reason → expect ERROR (exit 2)"
    self_test_case "case 23" error "${root}" "placeholder reason" || return 1

    # Case 24: the count is a RECORD, not a dial. Raising it against the base
    # version of the ledger is the one edit the contract forbids by name, and it
    # is invisible to every other check because the run is otherwise green.
    printf 'src/debt.rs\t1\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tout-of-scope\t%s\tlines=1\n' \
        "${fixture_reason}" > "${tmp}/base-ledger.tsv"
    printf 'src/debt.rs\t9\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tout-of-scope\t%s\tlines=1\n' \
        "${fixture_reason}" > "${bl}"
    echo "[self-test] case 24 — ledger count raised above the base version → expect FAIL"
    DOC_ANCHORS_BASE_FILE="${tmp}/base-ledger.tsv" DOC_ANCHORS_BASE_DESC="the fixture base" \
        self_test_case "case 24" fail "${root}" "ledger count RAISED" || return 1

    # Case 25: a legitimately added row still passes — growth is allowed — but
    # it is ANNOUNCED, with its scope and reason, so it cannot land unnoticed.
    : > "${tmp}/base-ledger-empty.tsv"
    printf 'src/debt.rs\t1\tanchor-not-found\tdocs/target.md \302\247"9. No such section"\tout-of-scope\t%s\tlines=1\n' \
        "${fixture_reason}" > "${bl}"
    echo "[self-test] case 25 — a new, classified ledger row → expect PASS + 'the ledger GREW'"
    DOC_ANCHORS_BASE_FILE="${tmp}/base-ledger-empty.tsv" DOC_ANCHORS_BASE_DESC="the fixture base" \
        self_test_case "case 25" pass "${root}" "ledger GREW by 1 row" || return 1

    # Case 26: `--report-unbound` is the flag that keeps the coverage gap
    # visible. It has to print when the guard is RED, which is when someone is
    # triaging; it used to sit after the failing exit and print nothing.
    rm -f "${bl}"
    cat > "${root}/src/unbound.rs" <<'RS'
//! Ids are UUIDv4 per RFC 4122 §4.3.
RS
    echo "[self-test] case 26 — --report-unbound prints while the guard FAILS"
    local ub_out ub_status=0
    ub_out="$(SCAN_ROOT="${root}" REPORT_UNBOUND=1 run_check 2>&1)" || ub_status=$?
    if [ "${ub_status}" -ne 1 ]; then
        echo "[self-test] case 26 FAILED — expected the guard to fail (exit 1), got ${ub_status}" >&2
        echo "${ub_out}" >&2
        return 1
    fi
    if ! printf '%s' "${ub_out}" | grep -qF -- "unbound §-marks"; then
        echo "[self-test] case 26 FAILED — the guard failed but --report-unbound printed nothing:" >&2
        echo "${ub_out}" >&2
        return 1
    fi
    if ! printf '%s' "${ub_out}" | grep -qF -- "src/unbound.rs:1"; then
        echo "[self-test] case 26 FAILED — the unbound listing did not name the skipped § site:" >&2
        echo "${ub_out}" >&2
        return 1
    fi
    echo "[self-test] case 26 OK"
    rm -f "${root}/src/unbound.rs" "${root}/src/debt.rs"

    # Case 27: the allowlist's RULES column. A session record quotes a defective
    # line-number spelling verbatim as a specimen (Rule B exempt, by the entry's
    # own stated reason) — and that exemption must NOT also switch off anchor
    # resolution on the same line, which is what the old `.` entry did.
    mkdir -p "${root}/docs/sessions"
    cat > "${root}/docs/sessions/fixture-session.md" <<'MD'
# Session record fixture

We used to write `docs/target.md line 42`, which is the banned spelling.
MD
    echo "[self-test] case 27a — session record's line-number specimen → expect PASS (Rule B exempt)"
    self_test_case "case 27a" pass "${root}" || return 1
    cat >> "${root}/docs/sessions/fixture-session.md" <<'MD'

The converted form points at `docs/target.md` §"9. No such section".
MD
    echo "[self-test] case 27b — a BROKEN anchor in the same session record → expect FAIL (Rule A applies)"
    self_test_case "case 27b" fail "${root}" "section anchor not found" || return 1
    rm -rf "${root}/docs/sessions"

    # Case 28: a malformed allowlist entry is a configuration failure, not a
    # silently wider exemption.
    echo "[self-test] case 28 — malformed allowlist entry → expect ERROR (exit 2)"
    local saved_allow bad_status=0 bad_out
    saved_allow="$(allowlist_entries)"
    allowlist_entries() { printf 'docs/*\tZ\t.\tbogus rules column\n'; }
    bad_out="$(SCAN_ROOT="${root}" run_check 2>&1)" || bad_status=$?
    eval "allowlist_entries() { cat <<'RESTORED'
${saved_allow}
RESTORED
}"
    if [ "${bad_status}" -ne 2 ] || ! printf '%s' "${bad_out}" | grep -qF -- "malformed allowlist entry"; then
        echo "[self-test] case 28 FAILED — expected exit 2 and a malformed-allowlist diagnostic, got ${bad_status}:" >&2
        echo "${bad_out}" >&2
        return 1
    fi
    # The override above is the only mutation the self-tests make to the script's
    # own behaviour; prove it was undone rather than assuming it.
    if [ "$(allowlist_entries)" != "${saved_allow}" ]; then
        echo "[self-test] case 28 FAILED — allowlist_entries was not restored after the override" >&2
        return 1
    fi
    echo "[self-test] case 28 OK"

    # ==================================================================
    # BARE-ANCHOR WHOLE-TOKEN MATCH (PR #292 peer review). The bare-anchor
    # fallback used to accept ANY substring of a heading's text, so renaming
    # `### 3e.` to `### 13e.` in the real tree left every `§3e` citation into
    # it silently green — CI never noticed dozens of citations had gone
    # stale. Cases 29-30 pin the cure: a whole-TOKEN prefix match, not a
    # substring match anywhere in the label.
    # ==================================================================

    # Case 29: renaming a heading by PREPENDING a digit must break every bare
    # citation into it, not keep passing because the old label is still a
    # substring of the new one ("9k" inside "19k").
    cat >> "${root}/docs/target.md" <<'MD'

## 9k. Token heading

Distinguishing body text for the token-match fixture.
MD
    cat > "${root}/src/token.rs" <<'RS'
// Cites `docs/target.md` §9k for the token-match fixture.
RS
    echo "[self-test] case 29a — bare anchor matches a heading's OWN label → expect PASS"
    self_test_case "case 29a" pass "${root}" || return 1
    sed 's/## 9k\. Token heading/## 19k. Token heading/' \
        "${root}/docs/target.md" > "${root}/docs/target.md.new"
    mv "${root}/docs/target.md.new" "${root}/docs/target.md"
    echo "[self-test] case 29b — heading renamed by prepending a digit (9k → 19k) → expect FAIL, not a silent substring pass"
    self_test_case "case 29b" fail "${root}" "section anchor matches no heading" || return 1
    rm -f "${root}/src/token.rs"
    write_fixture_doc

    # Case 30: a bare anchor that is not an EXACT heading label still resolves
    # when the heading text STARTS with the anchor as a whole token
    # (immediately followed by `.`, whitespace, `)`, or `-`) — the same
    # mechanism that turns architecture.md's real `§4d` into "ambiguous"
    # rather than "not found" against its `4d-bis`/`4d-tris` headings. This
    # proves the fix is a whole-token PREFIX match, not merely an exact-label
    # match with the substring fallback deleted outright.
    cat >> "${root}/docs/target.md" <<'MD'

## 7g-note. Prefix token heading

Distinguishing body text for the whole-token prefix fixture.
MD
    cat > "${root}/src/prefix.rs" <<'RS'
// Cites `docs/target.md` §7g for the whole-token prefix fixture.
RS
    echo "[self-test] case 30 — bare anchor matches a heading's whole-token PREFIX (not its exact label) → expect PASS"
    self_test_case "case 30" pass "${root}" || return 1
    rm -f "${root}/src/prefix.rs"
    write_fixture_doc

    echo "[self-test] all passed"
    return 0
}

# Flags compose: `--report-only --report-unbound` is a legitimate triage run.
# (The old single-`case` form silently ignored every argument after the first.)
while [ $# -gt 0 ]; do
    case "$1" in
        --self-test)
            run_self_tests
            exit $?
            ;;
        --report-only)
            REPORT_ONLY=1
            ;;
        --report-unbound)
            REPORT_UNBOUND=1
            ;;
        --update-baseline)
            MODE='write'   # quoted: bare `write` reads as the write(1) command (SC2209)
            ;;
        *)
            echo "usage: $(basename "$0") [--self-test] [--report-only] [--report-unbound] [--update-baseline]" >&2
            exit 2
            ;;
    esac
    shift
done

status=0
run_check || status=$?
if [ "${status}" -eq 1 ] && [ "${REPORT_ONLY}" -eq 1 ]; then
    echo "check-doc-anchors: --report-only, exiting 0 despite the violations above."
    status=0
fi
exit "${status}"
