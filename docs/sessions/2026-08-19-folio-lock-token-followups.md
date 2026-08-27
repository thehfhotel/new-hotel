# Follow-ups from the folio-lock-token branch (2026-08-19/20)

Branch: `fix/cin-work-number-lock-semantics`. These were found while fixing it, deliberately
left out of scope, and each needed a decision or its own ticket. Two have since been settled in
`chore/doc-anchor-hardening` — the no-op extend (item 1) and the citation sweep (item 3) — and
each of those entries records that outcome in its own text, so a reader is never left believing
a closed defect is still open.

## Real bugs

1. **No-op extend bumped the folio lock for nothing — FIXED in `chore/doc-anchor-hardening`.**
   `hotel-backend/src/service/checkin.rs` `extend()` used to accept a `new_end` equal to the
   folio's current `cin_expected_checkout`, so a same-date submit via
   `PUT /api/checkins/{id}/change-dates` enqueued an `ExtendStay` that bumped
   `Cin_Work_number` and kicked a receptionist out of her open folio for no change at all.
   It was the correct layer and a real bug, and it was deliberately left out of the
   folio-lock branch; it is fixed in this tree instead. This entry is now the record of a
   closed defect — do not read it as outstanding work.

   **What landed.** `reject_noop_extend` — an equality-only guard called from `extend()`
   immediately after that method re-reads the folio inside the writing transaction with
   `SELECT cin_expected_checkout FROM ht_checkins WHERE cin_id = $1 FOR UPDATE`. It
   **rejects**; it does not silently swallow the submit. The error is
   `ServiceError::Validation` carrying `check-in <id> already ends on <date> — nothing to
   change`, which reaches the receptionist as a 400 naming the date her folio already ends
   on, so the message tells her what to do next. `Validation` rather than `Conflict` is
   deliberate: both extend routes funnel every `Conflict` through `map_extend_error`, which
   replaces the message with the fixed string "Check-in is not active" — accurate for the
   status guard, actively misleading here. Direction remains the route layer's business: a
   later date still extends, an earlier one still shortens, and only equality is refused.

   **The row lock is the load-bearing half.** Both HTTP callers already refuse this shape up
   front (`validate_new_checkout_after_existing` rejects `<=` on the one-more-night endpoint,
   `validate_checkout_date_editable` rejects `==` on change-dates), so reaching the service
   guard at all means the folio moved between the route's read and the write. Re-reading the
   current date under `FOR UPDATE` in the same transaction closes that window instead of
   narrowing it. The guard runs before the `ht_checkins` UPDATE and before the outbox insert,
   so on the no-op path nothing is written to PG and nothing is enqueued.

   **Covered by four tests — `cargo test --lib` now lists 1670, four more than before this
   fix.** Two are pure guard tests (an equal date rejects with a message naming that date; a
   later and an earlier date both pass). Two are DB-backed: the no-op case asserts zero
   `writeback_jobs` rows and an
   untouched `updated_at` sentinel — proving nothing at all was written, not merely that the
   same date was written back — and the control case asserts that a genuine extend and a
   genuine shorten each still land and each still enqueue their own job. The two DB-backed
   tests skip themselves when PostgreSQL is unreachable, so a green suite on a machine
   without PG proves the pure guard only.

2. **`extend_stay` has already run in production once**, at HF Hotel:
   `writeback_jobs` id 47, folio `CH26-006392`, 2026-07-31 11:41:55Z. That folio still carries
   `Cin_Work_number = 183833154` — the only value outside iHOTEL's own generator range among
   HF Hotel's 20,730 folios. Under the branch's decision that write was correct behaviour,
   merely misnamed, so it is **not** to be "repaired": any value we write is itself a lock-token
   write, and it self-heals the next time a receptionist opens that folio. Whether anyone
   actually lost an edit that evening is **unverifiable** — iHOTEL logs nothing when the modal
   fires. Guard against a well-meaning cleanup script.

## Structural

3. **Line-number citations into living documents.** This repo cited
   `COMPAT_CHEATSHEET.md` and `findings.md` by line number in at least six spellings
   (`<doc>:N`, `<doc> line N`, `<doc> lines N, M, P-Q`, a section sign followed by a
   line RANGE rather than a heading, `COMPAT:N`, and `spike <section> line N`).
   Any edit to either document silently invalidated them all: this branch's doc growth broke
   ~40 of them across 25 files, and repairing that consumed more effort than the change itself.
   Several were **already wrong before this branch**. The durable fix is to convert them to
   section anchors (`` `docs/legacy-app/COMPAT_CHEATSHEET.md` §7.4 ``) repo-wide, plus a CI
   check that every citation resolves. Deliberately not attempted in the folio-lock branch —
   it would have buried a one-column change under an unrelated sweep.

   **Done in `chore/doc-anchor-hardening`.** The canonical form is
   `` <doc-ref> §<section-anchor> ["<phrase-anchor>"] `` — a repo-relative path (or a
   registered alias), `§` meaning a heading and only a heading, and an optional verbatim
   phrase that must appear inside that heading's section. Line numbers survive only into
   frozen evidence (`docs/legacy-spike/raw/**`, `docs/legacy-spike/schema/**`,
   `hotel-backend/schema-baseline.txt`, the off-repo `*.cs` decompile), and those must name
   the frozen file on the same line. Enforced by `scripts/check-doc-anchors.sh` in CI.

   **How it landed — three buckets, three different treatments.** The guard's first full
   run reported **204 citation problems across 65 files**. That number was never one
   problem. Triaging it split cleanly into three groups that needed opposite handling,
   and the governing decision was to **ship the guard green with the legacy debt
   explicitly baselined** — a new lint on an existing codebase lands with a recorded
   baseline and burns down, rather than holding a focused branch hostage to a 204-item
   sweep.

   - **(a) Guard false positives — fixed in the guard, never in the files.** The guard
     treated every section marker as a document citation. Plenty of them in this tree address
     something else: a section of an IETF standard (`RFC 4122 §4.3`), a section of a
     *code* file's own doc comment (`bin/writeback.rs` numbers its header sections and
     `migrations/pg/016_writeback_notify_trigger.sql` points at one of them), the printed
     Thai reception checklist (`docs/coexistence/reception-verification-TH.html`, which the
     guard deliberately does not treat as a document — HTML has no `#` headings), and
     off-repo plans that exist nowhere in git (the employee-login wave-4 A/B/C section
     labels). A doc-ref is now a **precondition** for checking a `§` rather than
     something to complain about the absence of, so a migration that numbers its own
     internal sections — `migrations/pg/066_create_ht_verification_responses.sql` labels
     five of them — stays exactly as written.
     Two narrower fixes shipped alongside: a Markdown **table row** is now its own
     continuation block (a doc-ref in one cell had been hijacking `§`s eight rows down,
     which is why three self-references in `findings.md`'s summary table were being
     resolved against `COMPAT_CHEATSHEET.md`), and the off-repo capture log
     `legacy-events-full.log` joined the frozen-evidence set. **No prose was rewritten to
     satisfy a buggy linter.**

     One document did get rewritten afterwards, for the opposite reason: *this* one. It is
     a record *about* citation spellings, so its specimens were themselves being read as
     citations — and one of them was passing only because a section marker that names no
     document used to fall back to the citing file, i.e. the specimen was validating
     itself. The specimens now **describe** the retired spellings instead of reproducing
     them, exactly as item 6 did for the audit snapshot's `was:` parentheticals, and every
     section marker left in this file names a document and resolves against a real heading
     in it.

     What that fix costs, stated plainly: roughly 70 of the silenced findings *were*
     real citations into real repo documents that simply sit further than the
     10-line/one-comment-block continuation window from their doc-ref (cheatsheet §3.17
     in `recipes/room_change.rs`, cheatsheet §1.9 in `writeback/dispatcher.rs`, ADR 0005
     §3a/§4/§5 in `scheduler/sync.rs`, and others). Those are no longer verified — the
     guard will not notice if `COMPAT_CHEATSHEET.md` renumbers §3.17. Adding a doc-ref
     next to the `§` re-arms them.

   - **(b) In-scope debt — converted.** Real citations under `hotel-backend/src/**`,
     `hotel-backend/tests/**`, `docs/**` and `migrations/**` were converted to the
     canonical form. Four repair patterns recur, and the distinction between them
     matters more than the count:
     - a *line range wearing a section marker* (the retired spellings aimed a section
       marker at `960-970` and at `946-956`) is not a section at all — it becomes one
       anchor per heading, e.g.
       `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Round_Bill` (A)" plus
       §"3.20 Open Round-Bill" plus §"3.21 Close Round-Bill";
     - *ambiguity* is cured by lengthening the anchor, **never** by renumbering the
       target: an anchor of just `HT_Rooms` matches five headings in the cheatsheet,
       while `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Rooms` (A)" matches
       exactly one. Same device for `HT_CheckIn_Pay` (3→1), `HT_Book_Ds` (2→1),
       `HT_Products` (2→1), and for the duplicate `4e` heading in
       `docs/architecture.md` §"4e. Reconcile";
     - a *wrong document* is the commonest silent defect — the cheatsheet's 3.15/3.16/3.17
       and the spike's 3e/3f/3j had drifted onto whichever document the continuation
       happened to name;
     - a *frozen-evidence citation that wrapped across two lines* needs no anchor at all,
       only a reflow, because the exemption requires the frozen file on the same line.
     No conversion in `docs/**` or `migrations/**` was left unfinished and parked in the
     baseline, and the DDL in every touched `.sql` file is byte-identical (only `--`
     comment lines changed).

   - **(c) Out-of-scope debt — recorded, not fixed.** It is listed in
     **`scripts/doc-anchors-baseline.tsv`**, which the guard reads on every run and which
     is the authority on the current row set — this paragraph describes it, it does not
     duplicate it. **Every row carries scope `out-of-scope`; no in-scope citation is
     baselined, and none may be** — an in-scope row would be the baseline working as a
     mute button, which is exactly what it must not be. Two families are in it as this
     branch lands: pre-existing citations under `thai-id-middleware-tauri/**` and `app/**`,
     areas this branch never claimed (three of them the same defect — a heading RANGE the
     anchor grammar cannot express, cured by writing two anchors; one naming a section no
     one could identify, which needs a decision rather than a rename); and section markers
     elsewhere in `docs/**` that name no document and so read as self-references, which the
     guard began failing once it stopped resolving a self-reference against a unique
     literal in the citing file itself. Every one of those is cured by naming, on the same
     line, the document it always meant — they are debt to burn down, not a class of
     citation the guard is wrong about. The
     ledger cannot silently grow: a failure that is *not* listed still fails the build,
     and a listed failure that occurs **more** times than its recorded count also fails
     ("do NOT raise the count — remove the new occurrence"). Fixing debt is never a red
     build — a shrunken or repaired entry prints a warning asking for the row to be
     lowered or deleted. Its key is `path + class + citation TEXT`, deliberately **never
     a line number**: keying a baseline on line numbers would reproduce exactly the rot
     this guard exists to delete. The `lines=` column is informational only. Regenerate
     with `./scripts/check-doc-anchors.sh --update-baseline`; burn down by converting a
     citation and deleting its row.

   **The guard immediately earned its keep.** `docs/runbook-sync.md` cited `§3` three
   times for the Slack alert catalogue. There is no `## 3.` heading in that file:
   commit `d04db24` (2026-07-29) replaced the `## 3. Slack alert meanings` heading with
   `### 2c. SSE pool-exhaustion fix` while inserting the 2c knobs, deleting the section
   header and leaving its body orphaned inside §2. All three citations had been silently
   resolving to `### Scenario 3 — Check-in` in the receptionist test plan ever since —
   the wrong section, in the wrong half of the document, with nothing to notice it. Only
   the citation carrying a **phrase** anchor ("CT watcher LAG sustained") failed loudly;
   the two bare `§3`s resolved and would have stayed wrong indefinitely. That is the
   argument for the phrase half of the convention in one example. The heading has been
   restored rather than the citations re-pointed.

4. **Unresolvable citations** — two the repair passes correctly refused to guess at.
   Both were settled in `chore/doc-anchor-hardening` and are recorded here for the audit
   trail:
   - `writeback/allocate.rs`, and `writeback-audit-2026-05-12.md`'s C1/C2 entries, cited
     section 2 of `findings.md` for the `Pay_no` `R{yyMM}-{4digit}` and `Receipt_no`
     `B{yyMM}-{4digit}` formats. `findings.md` documents those formats nowhere, at any
     revision — the **document name** was wrong and the numbers were right: they belong to
     `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns". Primary sources
     `Module1.GetSIR_PAY` (`Module1.cs:1756`) and `FrmAddSale.GetSIR`
     (`FrmAddSale.cs:3818`) confirm both formats.
   - `writeback/recipes/mod.rs` cited spike section 3c, line N, for `update_customer`. It means
     the CAPTURE, not `findings.md`:
     `docs/legacy-spike/raw/booking-checkin-20260424-101838/writes.txt:28`. The section
     letter was also wrong — that line is timestamped 10:23:02, which the spike notes
     assign to section 3d, not 3c. Caveat carried into the fix: the capture is truncated at
     `[Cust_Add_ampore]=`, so it evidences the statement's shape but **not** the
     byte-pinned field count or the `where` clause.

5. **Pre-existing wrong anchors left alone.** Where a citation was already wrong at HEAD and
   this branch did not make it inconsistent, it was catalogued rather than fixed, per the scope
   rule above. Item 3's sweep is where these get cleaned up — and what the sweep did not
   reach is now a row in `scripts/doc-anchors-baseline.tsv` rather than an uncatalogued
   defect. "Catalogued rather than fixed" is now machine-checked: the ledger is read on
   every CI run and cannot grow.

## Judgement calls made, easy to reverse

6. `docs/legacy-spike/writeback-audit-2026-05-12.md` is a **dated audit snapshot**, and its
   citations were re-pointed so they resolve today. If dated audits are treated as frozen
   records, revert those five.

   Its C1/C2 entries also carried `(was: … <a line number> …)` parentheticals that quoted
   the *retired* defective spelling verbatim as a specimen — which trips Rule B, since a
   banned line number reads the same whether it is being used or being exhibited. Those
   parentheticals now **describe** the retired form instead of reproducing it. The
   alternative — an allowlist regex for anything prefixed `was:` — was rejected as a
   trivially abusable escape hatch from Rule B. (`docs/sessions/*`, this file included, is
   allowlisted from Rule B's line-number ban, on the narrower ground that session records
   are history, not guidance — the same rationale as `CHANGELOG.md`. That exemption is not
   a licence for broken anchors, and this file does not lean on one: every `§` in it names
   a document and resolves against a real heading.)

7. `CHANGELOG.md` still calls the write a "TM.30 touch" in four historical entries. Left
   deliberately — release-please owns that file and rewriting past entries is its own hazard.
   Flagged so a future grep-driven cleanup does not "re-fix" the recipe from them.

8. `docs/legacy-app/COMPAT_CHEATSHEET.md` §7.2 grew beyond the one entry the decision record
   authorised: all seven of its entries were re-verified against the decompile and rewritten,
   and the heading was renamed because five of them turned out **not** to be vestigial. The new
   claims were spot-checked and hold, but it is more surface than the change advertised.

## Known-imprecise, deliberately not guessed

9. `findings.md` §3e Phase 1 says "~12 statements" (the fence lists 13) and §3a's "~30ms" was
   corrected to a measured 61ms. Other timing figures in that document were not re-measured;
   treat them as approximate until they are.

## Known risk handed to the next session

10. **`docs/coexistence/task8-writeback-test-runsheet.md` is UNTRACKED, and its citations
    fail the guard the moment it is committed.** It belongs to another session, so
    `chore/doc-anchor-hardening` deliberately did not touch it — not an oversight, and not
    a claim that it is clean. The guard scans `git ls-files` only, so an untracked file is
    invisible to it: the branch is green with that runsheet sitting in the working tree,
    and the failure appears in whichever CI run first sees it committed.

    Measured 2026-08-20 by copying the whole working tree into a throwaway non-git
    directory (where the guard's file list falls back to `find`, so untracked files are
    scanned) and running the guard over that copy — the original was neither modified nor
    staged: **all three of its citations fail.** It carries three section markers, each
    introduced by the bare word `runbook`, which is not a registered alias and names no
    file, so every one of them is read as a self-reference into the runsheet: the anchor
    `Step` matches five of the runsheet's own headings and is ambiguous, and the anchors
    `Where the flags live` and `Rollback` match none of them. All three are really
    citations into `docs/runbook-sync.md` and none is being verified against it.

    What its owner should do before committing: put an explicit doc-ref in front of each
    section marker, lengthen the `Step` anchor until it matches exactly one heading in the
    document it actually means, and run `./scripts/check-doc-anchors.sh` locally. **Do not
    baseline it** — `scripts/doc-anchors-baseline.tsv` records pre-existing out-of-scope
    debt, and a citation written today is neither.
