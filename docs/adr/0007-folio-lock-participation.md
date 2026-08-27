# ADR 0007 — Participate in iHOTEL's folio lock for `extend_stay`, and only there

**Status:** Accepted — 2026-08-20. Shipped in PR #291
(`fix/cin-work-number-lock-semantics`, commit `7fb300c`). The write is not new: `extend_stay`
has emitted it since the recipe was written, under a wrong name. PR #291 renamed it, corrected
every doc that called it a TM.30 batch number, and left the emitted SQL byte-identical. This
ADR records why it stays, and draws the boundary around it.
**Scope:** `HT_CheckIn_H.Cin_Work_number` — which of our writeback recipes may write it, and
what may and may not be inferred from the fact that iHOTEL checks it. Does **not** cover the
service-layer defect that let a same-date submit enqueue an `ExtendStay` with nothing to
change; that was a separate fix, and it has since landed — the guard is
`hotel-backend/src/service/checkin.rs` `reject_noop_extend`
(`docs/sessions/2026-08-19-folio-lock-token-followups.md` §"Real bugs" "`reject_noop_extend`").
Decision 3 below explains why this ADR never licensed that defect.

## Context

`HT_CheckIn_H.Cin_Work_number` is iHOTEL's **per-folio optimistic-lock token**. Our own docs
called it vestigial ("written `0`, never read") and our code named it a TM.30 batch number.
Both were wrong. The full mechanism, call-site table and caveats now live in
`docs/legacy-app/COMPAT_CHEATSHEET.md` §7.4 "per-folio optimistic-lock token"; the facts this
decision rests on, each re-verified against the `Hotel-2018- V.1.45` decompile on 2026-08-20:

- **Taken on folio LOAD.** `Module1.GET_WORK_NUMBER` (`Module1.cs:1662-1669`) writes a fresh
  `Random.Next(100000, 999999)` into the column and returns it to the form's `WORK_ID`. It is
  called from exactly five sites, all inside a `LoadBill()`: `FrmEditDate.cs:4187`,
  `FrmPayAdd.cs:4753`, `FrmPayAddPro.cs:4649`, `FrmCheckIn.cs:8228`, `FrmCheckOut.cs:5167` —
  extend/shorten stay, add payment, add product, edit folio, check out and settle.
- **Re-checked before every save.** Each of those five forms re-reads the column in its own
  `SAVE_EDIT()` (`FrmCheckIn.cs:9695`, `FrmCheckOut.cs:6130`, `FrmEditDate.cs:4867`,
  `FrmPayAdd.cs:5380`, `FrmPayAddPro.cs:5300` — enclosing method confirmed for each). In four of
  the five it is the method's first statement; `FrmEditDate` puts one Yes/No save-confirm box
  ahead of it, so its operator sees the warning later — but still before any write. On mismatch
  the form shows `มีการแก้ไข … จากเครื่องอื่น`, calls `Close()`, and **issues no writes at
  all**: the receptionist's typed work is discarded and is not recoverable.
- **Nothing else consumes the value.** Across the whole decompile the only references to the
  column are `GET_WORK_NUMBER`, those five guards, and the one-time
  `ALTER TABLE HT_CheckIn_H ADD [Cin_Work_number]` at `frmMain1.cs:6815`. The schema default is
  `((0))` —
  `docs/legacy-app/SCHEMA.sql` §"Table: dbo.HT_CheckIn_H" "[Cin_Work_number] int NOT NULL DEFAULT ((0))"
  — which is where the "written `0`" reading came from: a folio that has never been opened
  simply still holds the default.

The misreading had a specific cause worth recording, because it will recur with the next
capture-derived recipe. The 2026-04 spike captured the write accurately and then attributed it
to the wrong event. In `docs/legacy-spike/raw/extend-20260424-101350/writes.txt` the token write
sits at `writes.txt:2` (`10:15:22`), while the save burst it was folded into is
`writes.txt:3-9` (`10:15:26`); three further token writes sit at `writes.txt:1`, `writes.txt:10`
and `writes.txt:25`, with no save attached to any of them. A capture shows *what* fired, never
*why*; the "why" needed the decompile.
`docs/legacy-spike/findings.md` §"3f. Extend stay" "the form-open lock take" now separates the
two explicitly.

## Decision

### 1. `extend_stay` bumps the token deliberately. No other recipe does.

`writeback/recipes/extend_stay.rs` emits, as its first statement, one
`update HT_CheckIn_H set Cin_Work_number=<fresh> where Cin_No=…`. The token is generated in
`execute()` by `new_folio_lock_token()` so `build_statements` stays pure, and
`folio_lock_token_leads_and_is_emitted_once` pins three properties: the bump leads the recipe,
there is exactly one of it, and it only ships alongside the real `Total_Price_Room` mutation —
we never invalidate a receptionist's open form without actually changing the folio underneath
her.

Everything else deliberately leaves the column alone, and that is a decision rather than an
oversight:

| Recipe | What it mutates | Bumps the token? |
|---|---|---|
| `extend_stay` | `HT_CheckIn_H` totals + dates, `HT_CheckIn_Ds`, `HT_Room_Status` | **Yes** |
| `payment`, `checkout`, `checkin_cancel` | the folio header `HT_CheckIn_H` itself | No |
| `refund_payment`, `room_change`, `pos_sale` | folio child rows (`HT_CheckIn_Pay`, `HT_CheckIn_Ds`, `HT_CheckIn_Product`) that the same five forms display | No |
| `walkin`, `checkin_to_booking` | INSERT a brand-new folio | No — and correctly so: the folio did not exist a moment ago, so no iHOTEL form can be holding a token for it. Both pin `[Cin_Work_number]` out of their INSERT column lists by test. |

The middle two rows are a **knowing gap**, not a claim of safety. A receptionist sitting on an
open `FrmPayAdd` while our `payment` recipe commits will still save over it. Widening the bump
to those recipes is a separate decision (see Consequences).

### 2. The bump is for concurrency safety, not byte-parity

Byte-parity does **not** require this write. The capture proves it: the token statement fired
four seconds before the save burst, as `FrmEditDate`'s form-open side effect, and the burst we
replay contains none of it. If parity were the only value we optimised, this statement would be
deleted tomorrow. It stays because of what happens without it.

Without the bump, a receptionist holding any of the five guarded forms — loaded before our
extend, still open after it — saves, her `SAVE_EDIT()` finds a matching token, and it proceeds.
What it then writes is **absolute literals read off her stale in-form labels**, not a delta.
`FrmEditDate.SAVE_EDIT` (`FrmEditDate.cs:4880-4886`, verified 2026-08-20) concatenates all five
`Total_Price_*` columns straight from `LabelTroom` / `LabelTpro` / `Labelroompro` / `LabelPayed`
/ `Tpay`. Legacy is now back at the pre-extend numbers.

The reversion is then **laundered into canonical, silently**. `sync/mappers/checkin.rs`'s
`update_existing` writes `cin_total_amount`, `cin_room_amount` and `cin_paid_amount` as plain,
un-COALESCEd assignments (`cin_total_amount = $8::float8` and siblings) — deliberately, because
the legacy header is the truth for those columns and a COALESCE guard would pin a stale basis
forever. So Change Tracking carries the reverted number straight into PostgreSQL, both sides
converge on it, and the reconcile sweep — which compares a fresh legacy hash against a fresh
canonical hash — finds them in agreement. **No reconcile row is recorded and no alert can
fire.** The guest's extended stay is gone from both databases and nothing anywhere says so.

Bumping the token converts that into a loud, safe failure: her form fails closed, tells her the
folio was edited from another machine, and writes nothing.

### 3. Why ADR 0006's notify-don't-automate rule does not forbid this write

ADR 0006 refuses to let our app cause writes into legacy MSSQL at moments nobody chose
(`docs/adr/0006-legacy-stale-notification.md` §"1. Notify, don't automate" "at moments nobody chose").
That principle is intact here, and the distinction is not a loophole:

- ADR 0006 was refusing writes that are a **side effect of automation nobody asked for** — a
  room-power toggle and `HT_Rooms`/`HT_Book_Date` UPDATEs fired because *we* decided to drive
  iHOTEL's refresh button on a screen the receptionist was not looking at.
- This write is the **direct consequence of a receptionist's own deliberate extend on that exact
  folio**. Someone chose the moment: she did. And invalidating open folio forms is the vendor
  app's own designed response to that folio being edited — we are conforming to iHOTEL's
  protocol, not inventing traffic.

The honest caveat: that argument holds only while every `ExtendStay` corresponds to a real
change. When this ADR was written it did not — a same-date submit through
`PUT /api/checkins/{id}/change-dates` enqueued a no-op `ExtendStay` that bumped the token and
evicted a receptionist for nothing. That was a **bug in the service layer**, which this ADR
explicitly did not license, and it has since been fixed: `service/checkin.rs` re-reads the
folio's `cin_expected_checkout` under a `FOR UPDATE` row lock inside the writing transaction
and `reject_noop_extend` refuses an equal date with a validation error the routes render as a
400 naming the date the folio already ends on, so on that path nothing is written to PostgreSQL
and no `ExtendStay` is enqueued
(`docs/sessions/2026-08-19-folio-lock-token-followups.md` §"Real bugs" "nothing is written to PG and nothing is enqueued").
The row lock is what makes the premise hold rather than merely usually hold: both HTTP callers
already reject an equal date up front, so the only way to reach the service guard is for the
folio to move between the route's read and the write — precisely the case that would otherwise
have slipped an eviction-for-nothing past this ADR's reasoning. Were that guard ever removed,
ADR 0006's objection would start to apply to the no-op path fairly.

### 4. The lock is ADVISORY. Do not build anything on it.

Recorded here because the natural next thought — "we participate in the lock, so we have mutual
exclusion with iHOTEL" — is false, and three independent reasons make it false:

1. **The check and the writes are not in one transaction.** `SAVE_EDIT()` reads the token, then
   issues its UPDATEs as separate batches. `grep BeginTransaction` over the entire decompile
   returns **zero** hits (verified 2026-08-20), which is the same finding
   `docs/legacy-spike/findings.md` §4a "is a separate batch" reached from the capture side. The
   window between compare and first UPDATE is unguarded.
2. **`new Random()` is clock-seeded.** `GET_WORK_NUMBER` constructs a fresh `Random` per call,
   so two workstations loading the same folio within the same tick can draw the same token and
   each see the other's stamp as its own. (`Random.Next(min, max)` also excludes its upper
   bound, so iHOTEL's reachable range is `100000..=999998`.)
3. **`FrmCheckIn_EditOnly` bypasses the lock entirely.** Reached from `FrmPayDebt.cs:2168`, its
   `SAVE_EDIT()` does `delete from HT_CheckIn_H where Cin_no=…` and then re-INSERTs the header
   (`FrmCheckIn_EditOnly.cs:8006` and `:8111`). The file contains **zero** `WORK_ID` /
   `GET_WORK_NUMBER` / `Cin_Work_number` references — it neither takes nor checks the token.
   iHOTEL can still silently destroy a folio another iHOTEL form is holding.

So the bump is a **best-effort courtesy that turns the common single-receptionist race into a
visible failure**. It is not a mutex, it is not a barrier, and it must never be cited as the
reason some other invariant holds.

### 5. We invalidate; we do not take or check

Our recipe writes the token. It never reads one before writing, and never refuses to write
because the token moved. Full participation — take on read, verify on write — would buy nothing
given Decision 4, and would add a legacy round-trip plus a new failure mode (what does a
writeback job do when it loses the race? it cannot ask anyone) to a worker whose own
multi-statement mutations are already transactional in a way iHOTEL's never are.

## Consequences

**Gained.** The one recipe that rewrites an existing folio's money and dates makes every stale
iHOTEL form on that folio fail closed instead of silently reverting it. The failure the bump
prevents was specifically an *invisible* one — un-COALESCEd plain writes make both databases
agree on the wrong number — so this is the difference between a caught error and no error at
all. The column is covered by the startup schema fingerprint
(`hotel-backend/src/writeback/fingerprint.rs`, `EXPECTED_SCHEMA_BASELINE`, entry
`("HT_CheckIn_H", 22, "Cin_Work_number", "int")`), so a rename or retype stops the workers
rather than quietly changing what the write means.

**Given up.** Six folio-mutating recipes knowingly do not bump the token, so the same
stale-form reversion remains possible for payments, check-out, check-in cancellation, refunds,
room changes and POS charges. Widening it needs its own decision record **and** a
reception-coordinated live test, for a concrete reason: the bump's cost is a receptionist losing
typed work at an unpredictable moment, and six recipes' worth of that is a UX change to
reception's day, not a code change. Do not widen it by inference from this ADR.

**Already in production, once.** `extend_stay` has run against legacy exactly once at HF Hotel,
on 2026-07-31, and that folio carries a token outside iHOTEL's own generator range —
`docs/sessions/2026-08-19-folio-lock-token-followups.md` §"Real bugs" "id 47, folio `CH26-006392`".
Under this decision the write was correct behaviour that was merely misnamed, so it is **not to
be repaired**: any value written to "clean it up" is itself a lock-token write with the same
eviction effect, and the folio self-heals the next time a receptionist opens it. Whether anyone
lost an edit that evening is **unverifiable** — iHOTEL logs nothing when the modal fires. This
paragraph exists to stop a well-meaning cleanup script.

**A standing rule, not just a record.** Both of these now carry the prohibition, and this ADR is
the decision record the second of them refers to:

- `CLAUDE.md` §"Legacy Database" "NEVER write `HT_CheckIn_H.Cin_Work_number` casually"
- `docs/legacy-app/COMPAT_CHEATSHEET.md` §7.4 "No other recipe may write it without its own decision record."

## Alternatives considered

- **Drop the write for byte-parity.** Rejected. Parity is a real invariant but it does not
  demand this statement — the capture shows it outside the save burst — and dropping it buys
  back nothing while re-opening a silent data-loss path. Recorded because "the capture doesn't
  have it in the burst, so delete it" is the obvious future PR.
- **Bump the token in every folio-mutating recipe.** Rejected *for now*, not on principle. It is
  probably where this ends up; it needs its own decision and a reception-coordinated test first,
  because the cost lands on receptionists rather than on us.
- **Take and check the token the way iHOTEL does.** Rejected — see Decision 5. It cannot deliver
  mutual exclusion (Decision 4) and it gives a headless worker a race it has no way to resolve.
- **Rely on the token for mutual exclusion between our writeback and iHOTEL.** Rejected as
  unsound. Decision 4 exists so this is refused with evidence rather than by taste.
- **Change nothing and detect the reversion afterwards via reconcile.** Not viable: Decision 2 is
  precisely the case reconcile cannot see, because both sides end up holding the same wrong
  value.

## References

- `docs/legacy-app/COMPAT_CHEATSHEET.md` §7.4 "the lock is advisory, not sound" — the
  decompile-derived mechanism, the five-form take/check table, and the caveats. Read that first
  for anything claimed above about iHOTEL's own behaviour.
- `docs/legacy-spike/findings.md` §"3f. Extend stay" "the form-open lock take" — the capture,
  and the separation of the form-open side effect from the save burst.
- `docs/legacy-spike/raw/extend-20260424-101350/writes.txt:1-2`, `writes.txt:10`, `writes.txt:25`
  — the four form-open token writes; `writes.txt:3-9` is the save burst, which contains none of
  them.
- `docs/legacy-spike/raw/checkout2-20260424-101023/07-events.txt:9` and `07-events.txt:14` — the
  take at folio open and the matching check 2.4 s later at save, without needing the decompile.
- `Module1.cs:1662-1669` (`_decompiled_clean/iHOTEL2025/`) — `GET_WORK_NUMBER`, the generator and
  the write.
- `hotel-backend/src/writeback/recipes/extend_stay.rs` — `build_statements`,
  `new_folio_lock_token`, and the `folio_lock_token_leads_and_is_emitted_once` test that pins
  position, count and co-occurrence with the totals UPDATE.
- `hotel-backend/src/sync/mappers/checkin.rs` — `update_existing`, whose plain un-COALESCEd
  `cin_total_amount` / `cin_room_amount` / `cin_paid_amount` writes are why the reversion in
  Decision 2 is invisible.
- `hotel-backend/src/service/checkin.rs` — `extend` and `reject_noop_extend`: the `FOR UPDATE`
  re-read and the equality guard that keep Decision 3's premise true, plus the four tests that
  pin the rejection, the empty outbox and the untouched row.
- `docs/adr/0006-legacy-stale-notification.md` §"1. Notify, don't automate" "at moments nobody chose"
  — the principle Decision 3 reconciles with.
- `docs/adr/0002-indefinite-coexistence.md` §"Decision" "Permanent co-existence is the target end-state."
  — why a lock we share with a vendor app is a permanent design concern, not a
  migration-window one.
- `docs/sessions/2026-08-19-folio-lock-token-followups.md` §"Real bugs" — the no-op-extend bug
  (found on the folio-lock branch, fixed in `chore/doc-anchor-hardening`, and recorded there as
  closed) and the one production run, which this ADR points at rather than resolves.
