# RUNBOOK — round-bill coexistence: backfill, surface the report, flip writes

Operator reference for taking the round-bill coexistence work from "shipped
dark" to live. Three independent milestones, in order of risk:

1. **Backfill the payment ledger** (J7e) — read-only, no coordination.
2. **Surface the read-only income report** to reception — no writes, no coordination.
3. **Flip round writes on** (`ROUND_WRITEBACK_ENABLED`) — needs a reception-coordinated live test.

Background: `docs/coexistence/ville-coequal-writes-plan.md` (round sections) and
the `[[round-bill-coexistence]]` memory. All commands run on **evergreen**
(`ssh evergreen`); the PG container is `new-hotel-db`, workers are
`new-hotel-production-{backend,sync,writeback}[-hfville]-1`.

---

## 1. Backfill `ht_payment_ledger` (read-only on legacy — safe anytime)

The live `PaymentMapper` mirror is forward-only (it populates a check-in's
lines when its payments next change). The round report (`GET
/api/shifts/{id}/report`) sums `ht_payment_ledger`, so until a backfill it
under-reports rounds with pre-deploy paid-in-full check-ins. The bin reads
`HT_CheckIn_Pay` READ-ONLY and mirrors via the exact live code path; idempotent.

```bash
# On evergreen, from the compose dir, as root (reads deploy-owned .env):
cd /home/deploy/new-hotel-production
# NOTE: pass flags by OVERRIDING the command (a bare `-- --dry-run` is eaten by
# `docker compose run`'s arg parser). Dry-run first, then live, per site:
sudo docker compose --profile backfill run --rm backfill-payment-ledger        ./backfill_payment_ledger --dry-run
sudo docker compose --profile backfill run --rm backfill-payment-ledger        ./backfill_payment_ledger
sudo docker compose --profile backfill run --rm backfill-payment-ledger-hfville ./backfill_payment_ledger --dry-run
sudo docker compose --profile backfill run --rm backfill-payment-ledger-hfville ./backfill_payment_ledger
# Widen the window: append `--days=180` or `--all` to the command.
```

> **Done 2026-06-27** (first pass): HF Hotel 1289 lines, HF Ville 1056 lines
> mirrored (90-day window, 0 errors). Re-run anytime (idempotent) to extend the
> window or refresh.

Verify (per site):
```sql
-- expect a non-trivial row count + recent ledger_pay_date values
SELECT count(*), max(ledger_pay_date) FROM ht_payment_ledger;
-- spot-check a closed round's report adds up to iHOTEL's ReportShipCash
```
Re-runnable safely (delete-then-insert per `Cin_No`). No legacy writes ever.

---

## 2. Surface the read-only income report (no writes)

Once the ledger is backfilled, the **read-only** round report can be shown to
reception WITHOUT enabling round writes — it just reads `ht_payment_ledger`.
This is gated separately from the write flag in `RoundControl.tsx` (the
`ดูรายงานรอบ` view button); un-gate it once §1 is done and verified, then deploy
via the normal pipeline (push to master). The close/denomination flow stays
gated on `ROUND_WRITEBACK_ENABLED`.

---

## 3. Flip round writes on (`ROUND_WRITEBACK_ENABLED`) — reception-coordinated

This is the only step that writes the **live shared** `HT_Round_Bill`. A cashier
round is the real shift session (one open per site; iHOTEL books the shift's
real payments to it) — there is no throwaway round.

**Verification basis (why reception coordination is light, not real-time).**
iHOTEL reads round state purely from the DB — verified against the decompile
(`FrmDueBill.cs`): the gate is `SELECT id FROM HT_Round_Bill WHERE round_end IS
NULL`; the round display/report reads the `View_RBill_H` view (computed from the
payment tables by round window). Our writeback writes the **exact** rows iHOTEL
writes — open `INSERT INTO [HT_Round_Bill] ([id],[round_start],[round_price],
[round_by])` (`:1653`), close `update HT_Round_Bill set round_end=…,round_by=…
where round_end is null` (`:1670`). So a correct DB write is **provably** seen
by iHOTEL — **no human watching iHOTEL's screen is required** to confirm it.
Technical verification = the operator smoke test (Phase 1) + querying
`HT_Round_Bill` / `View_RBill_H` directly. The ONE reason a dead window + a
reception heads-up still matter: our open/close toggles iHOTEL's **live payment
gate** (a closed round blocks iHOTEL payments until one is open), so don't do it
while reception is taking payments. Phase 2 (reception driving the buttons) is
then **workflow/UX confidence**, not a correctness gate.

### Prerequisites
- HF **Hotel only** until the Ship-B per-site write bundle (task 20) lands —
  `branch=hfville` open/close is still blocked by the `ville_write_guard`.
- §1 backfill done so the close report is accurate.
- The flag is wired in `docker-compose.yml` (`ROUND_WRITEBACK_ENABLED`, default
  false); enable by setting it `true` via the same mechanism as `AUTH_ENABLED`
  (GH Actions variable / evergreen `.env`) + redeploy.

### Phase 1 — operator smoke test (no reception, dead-of-night)
Pick a window with no transactions (~03:00). Flag on → confirm current iHOTEL
round closed → `POST /api/shifts/open` → watch: `writeback` log `open_round`
job done; a new `HT_Round_Bill` row (id, round_start, round_price, round_by),
`round_end` NULL, exactly one open; `ht_shifts` matching; iHOTEL UI shows it
open. Then `POST /api/shifts/close` → `round_end` set, iHOTEL shows closed,
`ht_shifts.shift_closed_at` set. Reverse: open in iHOTEL → `sync_round_bills`
mirrors into `ht_shifts` within a tick. Any anomaly → flag back to false + redeploy.

### Phase 2 — reception-driven (real shift boundary)
One person / one app at a time (the `get_id` MAX+1 race). At a quiet boundary:
reception closes the outgoing round, **opens the new round in our app**,
operator confirms it landed in `HT_Round_Bill` + iHOTEL shows it open; reception
does a normal payment that shift → confirm iHOTEL's round/cash-drawer report
includes it; at the next boundary, reception **closes from our app** → confirm
in iHOTEL.

### Abort / rollback (tell reception verbatim)
> "If the new app's open/close button errors, or iHOTEL shows a save error,
> just use iHOTEL as normal — it's still the source of truth."

Operator then sets `ROUND_WRITEBACK_ENABLED=false` + redeploys. No state to undo
(the legacy PK prevents duplicates; `sync_round_bills` self-heals `ht_shifts`).

### Sign-off
Both directions verified, exactly one open round per site throughout, zero
`writeback_jobs` failures, iHOTEL reports correct → leave the flag on (live) or
flip off and schedule go-live.
