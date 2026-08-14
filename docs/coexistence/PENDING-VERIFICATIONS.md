# Pending verifications — the record (opened 2026-07-09)

Standing record of everything that is BUILT but not yet live-verified, or verified-once but
awaiting its permanent-enable decision. Each item names the gate and the runbook. This file
originally rode an intentionally-open PR to stay visible; that PR was lost, and the file
sat unmerged on `docs/pending-verifications` until recovered to master on 2026-08-11
(referenced as "V11" by PR #237 and the compose flip checklist). Strike items as they
complete (with date + evidence link) rather than deleting them.

## Reception-coordinated (need a scheduled window with reception)

| # | Verification | Gate / flag | Runbook / ref |
|---|---|---|---|
| V1 | **TM30 companion mirror retest** — convergent-delta fix (e23a5ad) is UNTESTED live after the 2026-07-01 echo-loop incident: verify add mirrors once (verbatim, no title prefix), delete propagates, primary preserved, NO escalating echo across ≥2 sync ticks | `TM30_COMPANION_WRITEBACK_ENABLED` (off) | `docs/coexistence/legacy-writeback-test-runbook.md` + reception form `writeback_test` |
| V2 | **Thai-ID card text quality** — generated card's font/placement ruled "not acceptable"; fix + re-verify in iHOTEL ReportReg_1 before the permanent-enable decision | `GUEST_DOCUMENT_STORAGE_ENABLED` (on, nominally mid-test) | task8 runsheet; template ref in `thai-id-middleware-tauri/reference/` |
| V3 | **HF Hotel round-writeback Phase-1 smoke** (Ville passed 2026-07-01; HF Hotel identical code but untested) + Phase-2 reception adoption of RoundControl | `ROUND_WRITEBACK_ENABLED` (on) | `docs/coexistence/RUNBOOK-round-writeback-flip.md` |
| V4 | **OTA parked-booking promote, first live run** — park a real OTA booking roomless → assign first room → verify byte-parity CreateBooking lands in iHOTEL (#224, live since 2026-07-09) | coordinates with ota-desk `PMS_WRITEBACK_ENABLED` flip (Phase 0 = #223, merged) | `docs/pms-booking-writeback-design.md` (ota-desk) |
| V5 | **Zero-training UX gate** (ADR 0003) — per shift-loop surface as it ships, starting with the spatial grid + guest-move drag build: a receptionist who knows iHOTEL completes the task with no instruction | per-surface | ADR 0003 §acceptance; gap matrix §6b |
| V11 | **HF Hotel `/hk` end-to-end, branch-correct** (wave-4 housekeeping A). A maid opens `/hk`, taps เริ่มทำความสะอาด then เสร็จแล้ว on ONE empty room. Assert: `ht_hk_cleaning_events` rows land in `hotelnew` and **not** `hotelville`; `hotelnew.ht_rooms_new.room_clean` flips `f→t`; the `mark_room_clean` job reaches `done`; legacy `HT_Rooms.Room_Clean` for that `id` goes `'yes'→'no'`; reception's แผนกแม่บ้าน board updates **without a manual refresh** and names the maid | `HK_BRANCHES` (unset ⇒ `hfhotel`); no flag flip needed — this path is LIVE on deploy | baseline/verify SQL per `docs/coexistence/phase3-mark-dirty-runsheet.md` §"Live read path" |
| V12 | **Mark-dirty from the maid's phone** (wave-4 housekeeping B5). Set `HK_MARK_DIRTY_ENABLED=true`, redeploy, then run the EXISTING `phase3-mark-dirty-runsheet.md` (P3 baseline → dirty → **echo test at T+6..T+10, the whole point of the window** → mark clean → rollback criteria) but triggered from `/hk` instead of `/housekeeping`. Companion columns `Room_Use` / `Room_Use_Count` / `Room_Clean_Time` must be byte-identical to baseline, and **no** `HT_Housewife` row may appear (issue #276 absence check). Blast radius: one `Room_Clean` varchar on one row. The write shape is already proven — what is new is the TRIGGER (a maid's phone, unattended, able to flag an occupied room mid-stay), which is why this is not "just config". **2026-08-14: the flip is now real** — `HK_MARK_DIRTY_ENABLED` is plumbed through `docker-compose.yml` + the deploy payload (`.github/workflows/docker-build.yml`); `gh variable set HK_MARK_DIRTY_ENABLED -b true -R thehfhotel/new-hotel` + redeploy actually reaches the container (previously a silent no-op — neither var was wired past the app's own `std::env::var` read) | `HK_MARK_DIRTY_ENABLED` (default off ⇒ 403) | `docs/coexistence/phase3-mark-dirty-runsheet.md` |
| V13 | **HF Ville admission to `/hk`** — SEPARATE, LATER window; do not combine with V11/V12. HF Ville's canonical `legacy_room_id_int` is WRONG for 30 of 34 rooms (legacy `HT_Rooms` interleaves `id`↔`Room_no`; observable today: legacy says Ville 203 is dirty, canonical says 205). Sequence: run `repair_room_legacy_keys --dry-run` → **check the report header names the right databases** (it prints PG `select current_database()` and `DB_SERVER:MSSQL_PORT/DB_NAME`; for Ville that must read `hotelville` and `…:1436/HOTEL` — `--apply` refuses non-zero if the PG database contradicts `SITE_ID`, but the dry run only warns) → review the before/after table → **resolve any unmatched/ambiguous rooms first**: `--apply` aborts non-zero and writes NOTHING while `Legacy without canonical` / `Canonical without legacy` / `Ambiguous legacy Room_no` are non-empty, because an unmatched canonical room keeps a stale `legacy_room_id_int` that can collide with a repaired one (`--allow-partial` overrides this — only after reading WHY each room is unmatched) → **either stop the `sync-hfville` worker for the ~1 s the apply takes, or rely on the bin's built-in re-verify** (a successful `--apply` re-reads both sides, re-plans, and exits non-zero if `Rooms still to repair` ≠ 0 — the live CT room mapper reads outside the bin's lock and can re-stamp a row microseconds after the commit; the fix is simply to re-run) → `--apply` until it exits 0 → **re-run the dry run and confirm `Rooms to repair: 0`** → restart the sync worker if it was stopped → wait two clean sync ticks → re-verify `count(*) FILTER (WHERE room_no IS DISTINCT FROM legacy_room_no) = 0` and that legacy `id=21`/`Room_no='203'` now shows `room_clean=f` on canonical `'203'` (and `'205'` returns to `t`) → only THEN add `hfville` to `HK_BRANCHES` → repeat V11/V12 at Ville. Ville has **never** run a room writeback (`writeback_jobs` has zero) — treat it as a first-deploy event. **2026-08-14: the flip is now real** — `HK_BRANCHES` is plumbed through `docker-compose.yml` + the deploy payload; `gh variable set HK_BRANCHES -b "hfhotel,hfville" -R thehfhotel/new-hotel` + redeploy actually reaches the container (previously a silent no-op) | `HK_BRANCHES` (add `hfville`), gated on `repair_room_legacy_keys --apply` | `migrations/README.md` "Companion utility bins"; bin header |

## Code-gated (fix/verify before flipping or enabling)

| # | Verification | Blocking what |
|---|---|---|
| V6 | **Checkout POS double-count landmine**: `net = cin_total_amount + product_total` double-counts once new-app POS populates `ht_pos_sales` for a stay being checked out — fix + folio-parity re-verification BEFORE using new-app POS at checkout | new-app POS at checkout |
| V7 | **Sticky-note mirror** round-trip idempotency (same echo class as V1) before `NOTES_WRITEBACK_ENABLED` | notes visible to iHOTEL shift |
| V8 | **Cash-entry outbound** — progressed 2026-08-10 (be6bbb4): recipe `execute()` + allocator + `cash_legacy_id` back-population landed, #202 echo guard CLOSED. Still unwired (no route emits the intent) and byte-shape still unverified vs `FrmAddPay.cs:638` — one coordinated capture, then wire + flip | petty-cash parity in iHOTEL reports |
| V9 | **Deposit refund on legacy-origin folios** — `cr_legacy_ds_id` backfill, then verify the WARN-no-op class is gone | deposit refunds for iHOTEL-created stays |
| V10 | **Walk-up receipt VAT attribution scope** (finance decision + verify either app's tax report against `HT_Receipt_*`) — elevated: 4.3k receipts/yr | trusting tax reports |



## Standing monitors (no action until they fire)

- Reconcile sweep: rooms + future-entity projection changes still need manual ack (customers/
  bookings/checkins self-heal; force-converge covers customers/rooms only).
- Alert playbook + incident ledger: `docs/coexistence/sync-incident-log.md`.

Done items move to the bottom with evidence:

## Completed

- ~~V11 Layout-edit writeback echo round-trip~~ — PASSED 2026-08-11 (self-service, at night):
  room 301 moved (683,704)→(683,744) via the real `PUT /api/rooms/layout` route from an
  authenticated session; writeback job 49 `move_room_tiles` succeeded; `HT_Rooms.Room_X/Room_y`
  byte-parity both directions; CT echo converged into canonical <1 min each way; ZERO
  `ht_reconcile_log` rows at any point; board restored to the exact original state.
  `LAYOUT_WRITEBACK_ENABLED=true` (GH var) is now the permanent state — จัดผัง is LIVE.

- ~~Checkin idempotency-gate comparator class~~ — fixed d09e756, verified live, incident logged (2026-07-06).
- ~~PR #223/#224 OTA writeback Phase 0 + parked queue~~ — merged + deployed 2026-07-09 (CI + container restart verified; live promote test = V4).
