# iHOTEL Coexistence Audit — 2026-06-11

Five parallel audits verified every legacy-touching aspect of the codebase against the
decompile-derived references (`docs/legacy-app/*`, `docs/legacy-spike/findings.md`):
(1) writeback recipes, (2) CT sync mappers, (3) reconcile/backfill/migrations SQL,
(4) cross-cutting invariants (fingerprint, encoding, locking, timezone, echo
suppression), (5) functional coverage (FEATURE_MAP × REPORTS_INVENTORY).

## Overall verdict

**Conditional pass — coexistence-safe in daily operation, but partly by accident.**
Column/key/literal fidelity is excellent (byte-parity tests against live captures,
projection-guard locks against `schema-baseline.txt`, spike-validated TABLOCKX ID
allocation, correct Thai-wall-clock datetime discipline). The systemic weakness is the
**defer-on-missing-FK ↔ watermark-advance contract** in the CT watcher — the proven
root cause of the 2026-06-03 lost customer+booking — plus three writeback recipes that
can corrupt iHOTEL-entered data, and an echo-suppression design that was never actually
wired to the SQL Server primitive that implements it.

## Root cause of the 2026-06-03 silent drop (C22209 / R015290) — CONFIRMED

Two independent audits converged on the same mechanism:

1. `mappers/booking.rs:328-340` — when `resolve_customer_id` misses (customer row not
   yet in PG), the booking apply returns `Ok(None)` ("defer"). The watcher counts this
   as `skipped`, NOT `errored` (`bin/sync.rs:2550-2553`), so the watermark advances
   past the booking's CT version. Nothing ever re-fires a dependent aggregate — the
   `resolve.rs:8-14` contract doc claiming "the next CT tick will surface it" is false.
2. Default **global-watermark mode** (`SYNC_PER_TABLE_WATERMARK` off): the `HT_Book_H`
   poll advancing the shared watermark to V2 strands the customer's not-yet-polled V1
   row in `HT_Customers` — which is why BOTH rows vanished with zero errors.
3. Echo suppression was provably NOT involved (see invariant #6 below — the filter is
   inert; nothing produces `SYS_CHANGE_CONTEXT = 0x4E48`).

The check-in mapper already has the fix for this exact class
(`resolve_customer_or_eager_mirror`, `checkin.rs:618-673`); booking, payment-receipt,
guest-registry, and mirror dual-write call sites never received it.

## Prioritized fix list

### P0 — data loss / corruption in production paths

| # | Finding | Where | Fix |
|---|---------|-------|-----|
| 1 | **Defer/watermark silent-drop class** (June 3 root cause). 7 `Ok(None)` defer paths consume CT rows while the watermark advances: booking→customer (`booking.rs:328-340`), receipt→checkin (`payment.rs:267-278`, receipts have NO re-fire source), guest-registry→checkin (`guest_registry.rs:148-155`, TM.30 undercount), checkin→room/booking (`checkin.rs:342-380`), mirror G4/G6 (`mirror.rs:285-287,593-599`) | sync mappers | (a) eager-mirror customers in `apply_booking_aggregate` like checkin does; (b) `errored=true` on FK-defer everywhere; (c) enable `SYNC_PER_TABLE_WATERMARK`; (d) fix the false `resolve.rs` contract doc |
| 2 | **`payment.rs` blindly `DELETE HT_CheckIn_Product`** without the mandatory `Pro_Amt` stock-restore pairing (cheatsheet §6.3) and without re-insert — every writeback payment on a folio with iHOTEL minibar/product lines erases the charges AND corrupts stock | `recipes/payment.rs:209-211` | Drop the cart-clear statement (Phase-1 artifact) |
| 3 | **`extend_stay.rs` clobbers totals**: sets `Total_Price_Product=0` (hardcoded) and `Total_Price_Pay` from stale intent snapshot — columns the §3f capture never touches; races concurrent payments | `recipes/extend_stay.rs:130-142,226` | Drop both columns from the UPDATE (match capture) |
| 4 | **Room change leaves iHOTEL occupancy stale**: skips §3.17 caller duties (`Cin_Room_No`, `HT_Rooms` flags both rooms); code comment claims "sync mappers" converge it — wrong direction. iHOTEL shows guest in old room; double-assignment risk | `recipes/room_change.rs:25-43`, `service/checkin.rs:697-720` | Emit the companion statements in the same transaction |

### P1 — correctness bugs with bounded blast radius

| # | Finding | Where | Fix |
|---|---------|-------|-----|
| 5 | **Echo suppression is inert**: `SET CONTEXT_INFO` never populates `SYS_CHANGE_CONTEXT` (only per-statement `WITH CHANGE_TRACKING_CONTEXT` does); the watcher filter matches nothing; loop-safety is accidental (idempotent mappers). **WARNING:** adding the clause while keeping the filter would CREATE June-3-style loss (CT coalesces per-PK to latest change's context). | `mssql_session.rs:54`, `dispatcher.rs:364`, `bin/sync.rs:2846-2850` | Either delete the filter + rely on idempotent mappers (current de-facto mechanism, made honest), or move suppression into Rust (context AND projected==canonical). Fix false comments + replace grep-test with live probe |
| 6 | **Customer hard-deletes are a no-op**: D rows carry only `pk_id`; `apply_soft_delete` resolves only by `Cust_no` → every FrmManageCustomersNew delete silently skipped; `C0000` cascade also skipped because `existing_matches` in booking/checkin ignores `legacy_cust_no` | `customer.rs:671-708`, `booking.rs:604-611`, `checkin.rs:1141-1146` | Persist legacy `id` on `ht_customers`, resolve D by it; add `legacy_cust_no` to both `existing_matches` |
| 7 | **`update_room.rs` writes `N'…'` to varchar** (cheatsheet §1.8 violation; comment misattributes convention) + 10 read-side `N'…'` sites (checkout/payment/refund/mark_clean/backfills) | `recipes/update_room.rs:66,82,92` etc. | Strip `N` prefixes (one-char deletions); or amend cheatsheet with a verified carve-out |
| 8 | **Fingerprint guard lags recipes**: `HT_Products`, `HT_Housewife`, `Tb_Save_Image` written but verified nowhere; `HT_CheckIn_Product`/`HT_CheckIn_Other_People` CT-side only | `writeback/fingerprint.rs` | Extend `EXPECTED_SCHEMA_BASELINE`, regenerate hashes, fix stale comments |
| 9 | **`checkin_to_booking.rs` blanks ~25 customer-profile fields** on re-save (payload doesn't carry them; iHOTEL re-saves loaded values) — erases receptionist-entered email/address/ID-card | `recipes/checkin_to_booking.rs:147-158` | Hydrate re-save from existing `HT_Customers` row |
| 10 | **`booking_modify.rs`**: caption cleared but only rewritten when dates change (notes-only edit blanks iHOTEL room caption); `Room_Book` can dangle at a never-inserted `HT_Book_Date.id` | `recipes/booking_modify.rs:101-104,209,297-298` | Rewrite caption whenever step 0b cleared; point `Room_Book` at `MAX(id)` |

### P2 — coverage gaps & doc/contract tensions

- **Customer edits in the new app never write back** to `HT_Customers` (no intent exists; only `CustomerResave` inside ModifyBooking). iHOTEL shows stale phone/address. (`routes/new_customers.rs:205`)
- **POS sales / refunds bypass `HT_Receipt_*`** → iHOTEL `ReportSaleVat`/`ReportTax` lose product-line VAT attribution and overstate after refunds. Decide deliberately whether receipt emission is in scope; flag to finance meanwhile.
- **Maintenance + mark-dirty are PG-only** — a room taken out of service in the new app looks bookable in iHOTEL; mark-dirty never reaches the grid. (`new_maintenance.rs`, `housekeeping.rs:120`)
- **`HT_Book_Pro` has no CT mapper** — iHOTEL booking add-on products are dropped if the booking is checked in via the new app. Also un-ingested: `HT_Products_Price` (per-tier product pricing), `HT_Receipt_Ds`.
- **`Total_Price_vat` written as Σ(tender)** conflicts with cheatsheet §3.8/§6.2 accumulator semantics — verify with a fresh capture (POS sale + receipt cancel on same folio) before trusting.
- **Round-bill gate not honored** (cheatsheet §1.9): payments written outside an open `HT_Round_Bill` round miss iHOTEL shift reports. Add at least a WARN check.
- **`'Check Out'` (space) vs cheatsheet §8.11 hyphen** on `HT_Room_Status` — capture vs cheatsheet conflict; record a deliberate decision in `constants.rs`.
- **Timezone mislabels**: DomainEvent snapshots label Bangkok wall-clock as UTC (`booking.rs:805-809`, `checkin.rs:1535-1544`); `payment.rs:285-286` NULL-receipt-date fallback writes UTC into a Bangkok-wall-clock column.
- **`allocate_cust_no`** derives from `MAX(Cust_no suffix)` while iHOTEL uses `MAX(id)+1` — aligned today; derive from the same `MAX(id)+1` to prevent divergence.
- **Legacy-MSSQL migrations 020/021/022 exceed "CT-only"**: NOT NULL + clustered PKs on 18 legacy tables. CT requires PKs, mitigations were real — but amend CLAUDE.md with an explicit carve-out, and note the new failure mode: iHOTEL's MAX+1 dup-id races now hard-fail on the PK instead of silently succeeding.
- **`docs/legacy-app/SCHEMA.sql` is truncated** (~256 bytes/table, 29/61 tables incomplete) — regenerate from decompile artifacts; real authority is `hotel-backend/schema-baseline.txt` + projection-guard tests.
- **`migrate_legacy.rs` projections diverge from canonical mappers** (rate-tier `Cust_Type`, collapsed `C_Address`, split names) — retire or route through `sync::mappers`.
- Minor: vestigial `AppState.legacy_pool` (never queried — remove to make the routes/MSSQL boundary structural); `jobs.rs:410` datetime-as-string bind; `room_notes` COALESCE never converges a legacy NULL transition; coupon `printed→redeemed` semantic stretch; checkin `derive_stay_range` NULL-date poison-pill wedges the table watermark (loud, but one malformed folio halts checkin sync).

## What verified clean

- **ID allocation**: all 10 allocators use the spike-validated `TABLOCKX, HOLDLOCK` MAX+1 inside worker-owned transactions with retry. Collision-safe against live iHOTEL.
- **Datetime discipline** on the legacy-bound path: `M/D/YYYY h:mm:ss tt` no-leading-zeros, OLE serials, UTC→Bangkok conversion, capture-pinned tests.
- **Thai literals** byte-exact everywhere checked (`'จอง'`, `'ยกเลิก'`, `'เข้าพัก'`, power-note templates, `'Check-Out'` hyphen on CheckIn_Ds).
- **Transactionality**: every multi-statement recipe is atomic; iHOTEL never sees the partial states it would produce itself.
- **Reconcile sweep**: read-only, like-for-like hash projections both sides (checkins even hash through the production mapper itself).
- **Backfills**: zero legacy writes, idempotency claims hold.
- **Architecture boundary**: zero tiberius usage in routes/repository/service — the decommission boundary holds.
- **Plumbing**: 16/16/16 intents↔dispatcher arms↔recipes, no orphans, no dead recipes; `Tb_Version` untouched.
- **booking_create / booking_cancel / walkin / checkout / checkin_cancel / mark_clean / pos_sale / adjust_product_stock / coupon**: compliant, several byte-parity-tested against captures.

## Suggested execution order

1. P0-1 (defer/watermark class) — recurrence of June 3 is otherwise guaranteed.
2. P0-2, P0-3 (payment cart-clear, extend-stay totals) — active corruption of iHOTEL folios.
3. P0-4 (room change) — operational double-assignment risk.
4. P1-5 (echo suppression) — make the de-facto mechanism honest before anyone "fixes" it naively.
5. P1-6..10, then P2 as a planned track with receptionist-coordinated verification on HF Ville.

---

## Remediation status (same day, 2026-06-11)

**All P0 and P1 items above are FIXED and merged to master**, verified by
5/5 consecutive full-suite runs (1106 tests) against a live PG:

- P0-1 → merge `ac4fda4` (`fix(sync)`): booking eager-mirror (June 3 fix),
  ALL FK-defer paths now eager-mirror or hold the watermark, `resolve.rs`
  contract corrected, customer hard-deletes via migration 055 `legacy_id`,
  C0000-cascade idempotency, type-1 bookings, NULL-date poison pill,
  inert echo filter removed honestly, Bangkok-as-UTC event labels.
- P0-2/3/4 + P1-7/8/9/10 → merge `90bef62` (`fix(writeback)`): payment
  cart-clear deleted, extend-stay matches the §3f capture with live
  Balance re-aggregation, room-change emits the full §3.17 companion set
  (+ HT_Room_Status re-point), all N'…' stripped, fingerprint baseline
  extended to all 20 written tables, checkin-to-booking re-save narrowed
  to payload-carried fields, booking-modify caption + MAX(id) Room_Book,
  Cust_no derived from MAX(id)+1.
- Boundary hardening → `a558d82`: `AppState.legacy_pool` removed.
- Docs → `7fc8b59`: SCHEMA.sql regenerated complete from live prod;
  CLAUDE.md CT-DDL carve-out.
- Follow-ups found while verifying → `3a8d358`: **NEW finding** — coupon
  legacy-id-reuse poison pill (Delete orphans the canonical row; MAX+1
  reuse then errors on `ht_coupons_coupon_code_key` every retry; same
  class as the v2.66.3 room-calendar rebind). Fixed by re-attaching the
  orphan pre-INSERT. Plus Bangkok `pay_date` fallback and
  Conflict-on-concurrent-enqueue mapping.
- Test determinism → `d96a4ed`: weak fixture suffixes + shared-fixture
  races across 8 integration files.

**P2 implementation wave (2026-06-12, user-approved):**

- ~~Customer-edit + mark-dirty + maintenance writeback intents~~ — DONE
  (`6ed2018`): `UpdateCustomer` (31-field re-save hydrated from canonical
  so iHOTEL-entered fields are never blanked; deletes deliberately NOT
  written back — iHOTEL's delete is a destructive C0000 cascade),
  `MarkRoomDirty` (§3.13 + HT_Housewife start row), `SetRoomMaintenance`
  (`Room_Manternace` flip only, on actual change), plus the round-bill
  gate warning (`writeback_no_open_round`, log-only, never blocks).
- ~~`HT_Book_Pro` ingestion~~ — DONE (`7c8a5d8`): legacy migration 023
  (PK + CT enable, apply at the next maintenance window per the
  established mapper-ships-with-DDL pattern), PG migration 056
  (`legacy_mirror.ht_book_pro`), `BookProMirrorMapper`, watcher seeds
  following the 033/050 new-table pattern. The booking→check-in product
  TRANSFER (Book_Pro → HT_CheckIn_Product at conversion) is documented
  as a TODO in the mapper module doc — the `B_PRO_ID`→`Pro_no` mapping
  needs decompile/capture verification first.
- ~~`legacy_id` backfill~~ — DONE (`7c8a5d8`):
  `bin/backfill_customer_legacy_ids` (chunked, idempotent, `--dry-run`;
  run per site post-deploy).
- ~~Room-FK silent-loss family~~ — DONE (`7e76e9b`): room master mapper
  auto-creates unknown rooms; calendar + booking-line misses now hold
  the watermark. Gated on a production data-quality check (2026-06-12,
  both sites: zero orphan room references; PG mirrors 58/58 + 34/34).
- ~~Repeatable-intent idempotency keys~~ — DONE (`2453776`, found during
  this wave): ModifyBooking/ExtendStay/RoomChange/MarkRoomClean/
  UpdateRoom switched to per-event discriminator keys; the deterministic
  key + permanently-retained done jobs would have 409'd the second
  occurrence per aggregate.

**Still open (genuinely needs a human decision / external action):**
- POS/refund `HT_Receipt_*` emission (VAT scope — decide with finance).
- `Total_Price_vat` + extend-stay `Total_Price_Net` semantics — need ONE
  fresh capture on a product-bearing folio (receptionist-coordinated).
- `SYNC_PER_TABLE_WATERMARK=true` env flip — defense-in-depth only now;
  flip at a monitored deploy window (suggest hfville first).
- Apply `migrations/legacy-mssql/023` at both sites (Sch-M window), then
  run `backfill_customer_legacy_ids --dry-run` → live, per site.
- `HT_Products_Price` / `HT_Receipt_Ds` ingestion (023/056 pair is the
  template).

### Adversarial re-verification (independent reviewer, same day)

An independent adversarial review of the merged diff confirmed every
P0/P1 fix against the raw captures (`docs/legacy-spike/raw/`),
cheatsheet, and `schema-baseline.txt` — including the suspicious-looking
`Cin_note=''` in extend-stay, which IS in the raw capture (findings.md
abridged it). **No new violations introduced by the fixes.** It refuted
one absolute claim and surfaced residuals:

- ~~`bin/sync.rs` D-event orphan-recovery `lookup_query_errored` arm
  advanced the watermark on a TRANSIENT PG error~~ — fixed same day
  (sets `errored=true`; `no_matching_pg_row` stays warn-only since a
  retry can never learn more).
- **Room-FK family still silently skips** (bounded, rooms are
  operator-managed, but it's a coherent loss scenario when a room is
  created in iHOTEL and immediately sold): `room.rs:342` (unknown room
  warn-skip), `room_calendar.rs:157` (tile dropped), `booking.rs:849`
  (room line excluded from idempotency count — converges to the dropped
  state; checkin errors on the same condition). Decision needed: error
  symmetrically (one new room wedges sync loudly until `backfill_rooms`)
  vs today's quiet skip. Leaning loud, but legacy data quality
  (orphan room_no strings in old rows) needs a check first.
- **Extend-stay `Total_Price_Net` value source**: `new_checkins.rs:706`
  sets Net = room total only (no product plumbing) — on a folio carrying
  iHOTEL product charges an extend understates Net (and thus Balance).
  Same "needs a fresh capture on a product-bearing folio" status as
  `Total_Price_vat`.
- Pure-D-only batches on `HT_Book_Ds`/`HT_Book_Date`/`HT_CheckIn_Pay`
  rely on sibling-table CT rows re-firing the aggregate (every known
  iHOTEL flow does emit one) — cross-table reliance, not a guarantee;
  `force_coalesce_for_orphan_recovery` covers only `HT_CheckIn_Ds`.
- Migration 055 `legacy_id` index is non-UNIQUE; legacy id-reuse could
  pair two rows (bounded: the older is already soft-deleted in every
  reachable sequence).
