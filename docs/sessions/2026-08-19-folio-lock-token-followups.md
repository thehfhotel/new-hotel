# Follow-ups from the folio-lock-token branch (2026-08-19/20)

Branch: `fix/cin-work-number-lock-semantics`. These were found while fixing it, deliberately
left out of scope, and each needs a decision or its own ticket.

## Real bugs

1. **No-op extend bumps the folio lock for nothing.**
   `hotel-backend/src/service/checkin.rs` `extend()` does not reject
   `new_end == current expected checkout`, so a same-date submit via
   `PUT /api/checkins/{id}/change-dates` enqueues an `ExtendStay` that bumps
   `Cin_Work_number` and kicks a receptionist out of her open folio for no change at all.
   Correct layer, real bug, not this branch's job.

2. **`extend_stay` has already run in production once**, at HF Hotel:
   `writeback_jobs` id 47, folio `CH26-006392`, 2026-07-31 11:41:55Z. That folio still carries
   `Cin_Work_number = 183833154` — the only value outside iHOTEL's own generator range among
   HF Hotel's 20,730 folios. Under the branch's decision that write was correct behaviour,
   merely misnamed, so it is **not** to be "repaired": any value we write is itself a lock-token
   write, and it self-heals the next time a receptionist opens that folio. Whether anyone
   actually lost an edit that evening is **unverifiable** — iHOTEL logs nothing when the modal
   fires. Guard against a well-meaning cleanup script.

## Structural

3. **Line-number citations into living documents.** This repo cites
   `COMPAT_CHEATSHEET.md` and `findings.md` by line number in at least six spellings
   (`file.md:513`, `file.md line 534`, `file.md lines 420, 438, 611-640`, `file.md §946-956`
   — where `§` means a line, not a section — `COMPAT:534`, and `spike §3c line 28`).
   Any edit to either document silently invalidates them all: this branch's doc growth broke
   ~40 of them across 25 files, and repairing that consumed more effort than the change itself.
   Several were **already wrong before this branch**. The durable fix is to convert them to
   section anchors (`COMPAT_CHEATSHEET.md §7.4`) repo-wide, plus a CI check that every
   `<doc>:<line>` citation resolves. Deliberately not attempted here — it would have buried a
   one-column change under an unrelated sweep.

4. **Unresolvable citations** — two the repair passes correctly refused to guess at:
   - `writeback/allocate.rs:324`, `allocate.rs:369`, `writeback-audit-2026-05-12.md:28` and
     `:46` cite "`findings.md` §2 line 129/130" for the `Pay_no` `R{yyMM}-{4digit}` and
     `Receipt_no` `B{yyMM}-{4digit}` formats. `findings.md` documents those formats nowhere,
     at any revision — the citation appears to have always pointed at the wrong document.
   - `writeback/recipes/mod.rs:37` cites "spike §3c line 28" for `update_customer`. Line 28 of
     `findings.md` is blank and §3c starts far below, so "line 28" most likely means line 28 of
     a capture file. Ambiguous; needs whoever wrote it.

5. **Pre-existing wrong anchors left alone.** Where a citation was already wrong at HEAD and
   this branch did not make it inconsistent, it was catalogued rather than fixed, per the scope
   rule above. Item 3's sweep is where these get cleaned up.

## Judgement calls made, easy to reverse

6. `docs/legacy-spike/writeback-audit-2026-05-12.md` is a **dated audit snapshot**, and its
   citations were re-pointed so they resolve today. If dated audits are treated as frozen
   records, revert those five.

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
