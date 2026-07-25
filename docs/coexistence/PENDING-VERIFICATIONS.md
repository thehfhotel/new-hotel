# Pending verifications — the record (opened 2026-07-09)

Standing record of everything that is BUILT but not yet live-verified, or verified-once but
awaiting its permanent-enable decision. Each item names the gate and the runbook. This file
rides an intentionally-open PR so the list stays visible; strike items as they complete
(with date + evidence link) rather than deleting them.

## Reception-coordinated (need a scheduled window with reception)

| # | Verification | Gate / flag | Runbook / ref |
|---|---|---|---|
| V1 | **TM30 companion mirror retest** — convergent-delta fix (e23a5ad) is UNTESTED live after the 2026-07-01 echo-loop incident: verify add mirrors once (verbatim, no title prefix), delete propagates, primary preserved, NO escalating echo across ≥2 sync ticks | `TM30_COMPANION_WRITEBACK_ENABLED` (off) | `docs/coexistence/legacy-writeback-test-runbook.md` + reception form `writeback_test` |
| V2 | **Thai-ID card text quality** — generated card's font/placement ruled "not acceptable"; fix + re-verify in iHOTEL ReportReg_1 before the permanent-enable decision | `GUEST_DOCUMENT_STORAGE_ENABLED` (on, nominally mid-test) | task8 runsheet; template ref in `thai-id-middleware-tauri/reference/` |
| V3 | **HF Hotel round-writeback Phase-1 smoke** (Ville passed 2026-07-01; HF Hotel identical code but untested) + Phase-2 reception adoption of RoundControl | `ROUND_WRITEBACK_ENABLED` (on) | `docs/coexistence/RUNBOOK-round-writeback-flip.md` |
| V4 | **OTA parked-booking promote, first live run** — park a real OTA booking roomless → assign first room → verify byte-parity CreateBooking lands in iHOTEL (#224, live since 2026-07-09) | coordinates with ota-desk `PMS_WRITEBACK_ENABLED` flip (Phase 0 = #223, merged) | `docs/pms-booking-writeback-design.md` (ota-desk) |
| V5 | **Zero-training UX gate** (ADR 0003) — per shift-loop surface as it ships, starting with the spatial grid + guest-move drag build: a receptionist who knows iHOTEL completes the task with no instruction | per-surface | ADR 0003 §acceptance; gap matrix §6b |

## Code-gated (fix/verify before flipping or enabling)

| # | Verification | Blocking what |
|---|---|---|
| V6 | **Checkout POS double-count landmine**: `net = cin_total_amount + product_total` double-counts once new-app POS populates `ht_pos_sales` for a stay being checked out — fix + folio-parity re-verification BEFORE using new-app POS at checkout | new-app POS at checkout |
| V7 | **Sticky-note mirror** round-trip idempotency (same echo class as V1) before `NOTES_WRITEBACK_ENABLED` | notes visible to iHOTEL shift |
| V8 | **Cash-entry outbound** (`cash_entry.rs` deliberately unwired; byte-shape unverified; #202 echo guard) — one coordinated capture, then wire + flip | petty-cash parity in iHOTEL reports |
| V9 | **Deposit refund on legacy-origin folios** — `cr_legacy_ds_id` backfill, then verify the WARN-no-op class is gone | deposit refunds for iHOTEL-created stays |
| V10 | **Walk-up receipt VAT attribution scope** (finance decision + verify either app's tax report against `HT_Receipt_*`) — elevated: 4.3k receipts/yr | trusting tax reports |

| V11 | **Layout-edit writeback echo round-trip** (#236) — move one tile in จัดผัง mode at night, verify `HT_Rooms.Room_X/Room_y` updated + iHOTEL board shows it + CT echo converges (no reconcile row survives a sweep tick), move it back, then flip `LAYOUT_WRITEBACK_ENABLED` | self-service — NO reception coordination needed (tile positions aren't guest-facing) | CONTEXT.md "Layout-edit drag"; issue #236 |

## Standing monitors (no action until they fire)

- Reconcile sweep: rooms + future-entity projection changes still need manual ack (customers/
  bookings/checkins self-heal; force-converge covers customers/rooms only).
- Alert playbook + incident ledger: `docs/coexistence/sync-incident-log.md`.

Done items move to the bottom with evidence:

## Completed

- ~~Checkin idempotency-gate comparator class~~ — fixed d09e756, verified live, incident logged (2026-07-06).
- ~~PR #223/#224 OTA writeback Phase 0 + parked queue~~ — merged + deployed 2026-07-09 (CI + container restart verified; live promote test = V4).
