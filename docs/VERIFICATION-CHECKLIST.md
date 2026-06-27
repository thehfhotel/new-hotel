# Consolidated Verification & Flip Plan

**Updated:** 2026-06-27. Single source of truth for what still needs human/operator
verification across every recent effort. Everything below is **implemented and
deployed**; this is the verification + flag-flip checklist, not a build list.

How to read the shadow logs (prod):
```
docker logs new-hotel-production-backend-1 | grep 'shadow.checkout_total'
docker logs new-hotel-production-backend-1 | grep 'shadow.booking_validation'
```

---

## A. LIVE & active — verify by eyeball (no flag)

- [ ] **Room status single-source (spike Phase 1).** Open `/v2/rooms` AND the
      classic dashboard next to iHOTEL; confirm occupied/available/booked/
      checkout/maintenance all match iHOTEL (the original bug was 402–405 shown
      free while occupied). Both UIs now render the same server-derived status.
- [ ] **Round summary, both sites ("ทั้งหมด").** `/v2/rounds` with ทั้งหมด selected
      shows HF Hotel **and** HF Ville rounds (HF badge per row), drill-down opens
      the right site's report.
- [ ] **Round report + A4 print + occupancy.** Open a round report, hit พิมพ์ →
      clean one-page A4; income-by-tender + deposits + ห้องพัก/ผู้เข้าพัก counts
      match iHOTEL `View_RBill_H` (spot-checked: round 4778 cash 2797.20 ≡).
- [ ] **Dead-code removal (spike Phase 4).** Auto-verified (CI build/tests green;
      removed `/api/rooms/board`, `/api/rooms/checkouts-today`, `/api/occupancy`,
      `/api/new/stats` had zero consumers). Optional: smoke the dashboard +
      reports pages once in prod.
- [x] **CI build perf (cargo-chef fix).** Verified — warm backend build ~220s
      (was ~340s); deps cached, build layer slimmed.

## B. DARK — collect shadow data → decide → flip

- [ ] **Checkout total — `CHECKOUT_SERVER_TOTAL_ENABLED` (spike Phase 2).**
      Folio parity DONE (2026-06-27): the quote + flag-on checkout now compute the
      full folio — room + POS products + VAT (inclusive, from settings) + deposit
      line; net = room+product, balance = net−pay (matches iHOTEL HT_CheckIn_H).
      1. Collect `shadow.checkout_total` over ~1–2 weeks of real checkouts; size
         the client-vs-server delta (driven by actual- vs expected-nights basis).
      2. **Decide the nights policy** with reception (match iHOTEL `FrmCheckOut`).
      3. Flip `CHECKOUT_SERVER_TOTAL_ENABLED=true` once the delta is understood +
         validated against real folios. (Implementation gap closed.)
- [ ] **Booking validation — `BOOKING_VALIDATION_ENABLED` (spike Phase 3).**
      1. Collect `shadow.booking_validation` over real bookings; confirm it never
         flags a booking reception actually wants to make (zero false-rejects).
      2. Flip `BOOKING_VALIDATION_ENABLED=true` (fails open even on, so low risk).
- [ ] **Round writeback — `ROUND_WRITEBACK_ENABLED` (round coexistence step 2).**
      1. **Implementation gap:** HF Ville needs the per-site write bundle
         (Ship-B, Section C) before All/Ville can open/close rounds.
      2. Reception-coordinated **shift-boundary live test** (opening via our app
         requires closing reception's live round first — one-open-per-site).
      3. Runbook: `docs/coexistence/RUNBOOK-round-writeback-flip.md`. Flip per-site.

## C. Remaining IMPLEMENTATION (large, separately scoped — not yet built)

- [x] **Phase 2 folio parity** — DONE 2026-06-27 (products/VAT/deposits plumbed,
      ship-dark). See Section B Phase-2.
- [ ] **HF Ville Ship-B** — per-site write bundle so open/close/round/checkout
      writes route to `ville_pool` co-equally (task #20; gates Ville round
      writeback + co-equal Ville writes generally).
- [x] **Phase 4 second pass** — DONE 2026-06-27. Removed (proven zero-consumer):
      `db/dual_pool.rs`, `auth::router()`, and 6 unmounted read/list handlers
      (`list_shifts`, `get_checkin`, `get_customer`, `get_product`, `list_coupons`,
      `list_rate_tiers`) — ~357 lines. **Kept** (review verdict: intended/partial
      features with services or writeback recipes — not dead, just unwired):
      `update_customer`/`delete_customer`, `redeem_coupon`, `void_payment`,
      `adjust_stock`, inventory `create_*`/`get_low_stock`, legacy
      `bookings::list_bookings`. `KNOWN_SYNC_EVENT_NAMES` kept (used by tests).

---

Related: `docs/spikes/2026-06-27-frontend-backend-encapsulation.md`,
`docs/coexistence/RUNBOOK-round-writeback-flip.md`,
`docs/coexistence/ville-coequal-writes-plan.md`.
