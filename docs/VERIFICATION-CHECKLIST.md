# Consolidated Verification & Flip Plan

**Updated:** 2026-06-27. Single source of truth for what still needs human/operator
verification across every recent effort. Everything below is **implemented and
deployed**; this is the verification + flag-flip checklist, not a build list.

## ⚠ Reframing (2026-06-27): app is verification-only, not live for reception

Established this session: the new app is **not deployed to reception** — it is
**verification/shadow-only and not feature-complete** for day-to-day use. Daily
ops (bookings, check-ins, check-outs, invoices, housekeeping) still run entirely
in **iHOTEL**; real data flows iHOTEL → sync → canonical PG. Two consequences
reprioritize everything below:

1. **Flag flips are the LAST mile, not the next step.** The coexistence
   write-back flags only matter once reception actually operates the app and
   dual-writes. Until the app is feature-complete *and adopted*, they stay dark.
   **Priority shifts from "flip flags" → "reach feature-completeness."**
2. **Shadow-log validation is unreliable here.** The app's own mutating paths are
   barely exercised (checkouts/bookings ride iHOTEL), so `shadow.*` logs accrue
   almost nothing. Validate via the **read-only quote/validate endpoints + direct
   canonical-PG↔iHOTEL diff** (as done for the folio on 2026-06-27), not by
   waiting on shadow data.

**Revised track order:**
- **A. Correctness of what exists** — fix what verification surfaces: folio basis
  (#34), `cin_rate_per_night = 0` data-completeness, read-only feature eyeball
  (#29). Cheap, no reception scheduling.
- **B. Feature-completeness for reception adoption** *(new core work, #35)* — audit
  new app vs iHOTEL (`docs/legacy-app/FEATURE_MAP.md`) + daily reception
  workflows → gap list → close by daily-ops frequency. This is the real gate.
- **C. Coexistence write-back (DEFERRED)** — the flags below (#20/#30/#31 + round
  writeback) wait until B is done and reception is using the app. Re-validate via
  read-only diff, not shadow logs. The Jul-6 "flip BOOKING_VALIDATION after
  shadow review" reminder rests on a false premise (no shadow data accrues) —
  revise or cancel.

The original sections A (live) / B (dark flags) / C (implementation) below are
unchanged in content; only their **priority** is now A-correctness → B-features →
C-flags.

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

- [ ] **Checkout total — `CHECKOUT_SERVER_TOTAL_ENABLED` (spike Phase 2). ⛔ BLOCKED.**
      **2026-06-27 live-data check (via PG on evergreen): the folio is broken — DO
      NOT FLIP.** Room = `cin_rate_per_night × nights`, but `cin_rate_per_night = 0`
      for 100% of check-ins (hotelnew 19,827 zero/2 null/0 positive; hotelville
      1,873 zero/0 positive) and there's no fallback. `ht_pos_sales` is empty (no
      POS data). Flipping would record/writeback **room=0, net=0, balance=−paid**,
      zeroing room revenue + corrupting legacy `Total_Price_*`. Real amount lives
      in `cin_total_amount` (= `cin_paid_amount`, synced from iHOTEL); `ht_payments`
      populated (706 rows). Evidence + worked table:
      `docs/coexistence/checkout-folio-comparison.md` (top).
      **Fix first:** base room/net on `cin_total_amount` (`rate>0 ? rate×nights :
      cin_total_amount`; derive display rate = total÷nights), re-run comparison.
      Then: 1) collect `shadow.checkout_total`; 2) decide nights policy w/ reception;
      3) flip. (Note: shadow log fires only on new-app checkouts — rare, since
      checkouts still ride iHOTEL and sync in, so little shadow data accrues.)
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
