# PG canonical ↔ legacy MSSQL cardinality map

Single source of truth for cross-system table mapping. Required by
[PROCESS.md P1 + P5](PROCESS.md). Every `ht_*` canonical table MUST have
a row here.

**Status legend:**
- ✅ — implemented and verified correct
- ⚠️ — implemented but with known divergence (cite finding)
- ❌ — missing implementation for this layer
- N/A — does not apply at this layer

**Cardinality legend:**
- `1:1` — exactly one PG row per legacy row
- `1:N` — one PG row corresponds to N legacy rows (legacy is denormalized)
- `N:1` — N PG rows per legacy row (PG is normalized)
- `N:N` — many-to-many (rare; usually via a junction)
- `merged` — PG row aggregates multiple legacy rows (e.g. header + detail collapsed)

## Domain entity tables

| PG table | Legacy counterpart | Cardinality | Source of truth | Sync mapper | Read path | Write path | Notes |
|---|---|---|---|---|---|---|---|
| `ht_customers` | `HT_Customers` | `1:1` | shared | `sync/mappers/customer.rs` | `routes/new_customers.rs` | `writeback/recipes/booking_create.rs` + walkin/checkin_to_booking | ✅ column-for-column parity (Track E2 / migration 035 — all 33 legacy columns mirrored; `cust_price_over` writeback deferred to Track G) |
| `ht_room_types` | none (derived) | N/A | PG canonical | none | `routes/new_rooms.rs` | manual / settings | T1: verify type literals match legacy free-text where surfaced |
| `ht_rooms_new` | `HT_Rooms` | `1:1` | shared | `sync/mappers/room.rs` | `routes/rooms.rs` | maintenance + housekeeping | ✅ column-for-column parity (Track E2 / migration 036 — `Room_Use_Count` / `Room_X/Y` / `Room_Group` / `Room_Power_*` / `Room_Polity` all mirrored); ⚠️ `room_price_*` axis (weekday/weekend/special vs legacy A/B/C per customer-type) deferred to Track F |
| `ht_bookings` | `HT_Book_H` | `1:1` | shared | `sync/mappers/booking.rs` | `routes/new_bookings.rs` | `writeback/recipes/booking_create.rs` + modify/cancel | T1: stay-range derivation |
| `ht_booking_rooms` | `HT_Book_Ds` | `N:1` (junction) | PG canonical | `sync/mappers/booking.rs` (per-line) | joined into `routes/new_bookings.rs` | `writeback/recipes/booking_create.rs` per-night INSERT | ✅ correctly modeled as junction |
| **`ht_checkins`** | **`HT_CheckIn_H` (header)** | **`1:1` AT HEADER LEVEL** | shared | `sync/mappers/checkin.rs::apply_checkin_aggregate` | `routes/rooms.rs`, `routes/stats.rs`, calendar | walkin/checkin_to_booking recipes | ❌ **BUG: collapses multi-room folios into a single canonical row** (see [multi-room finding](audit-2026-05-13.md) — pending) |
| **(missing)** | **`HT_CheckIn_Ds`** | **expected `N:1`** | **expected PG canonical** | **none today** | **none today** | **walkin recipe emits one Ds row** | ❌ **MISSING: no `ht_checkin_rooms` junction table exists; T1 must verify and propose** |
| `ht_guest_registry` | `HT_CheckIn_Other_People` | `N:1`? | shared? | `sync/mappers/checkin.rs`? | TM.30 reports | walkin/checkin_to_booking | T1: confirm cardinality + whether "primary guest" is also stored |
| `ht_room_calendar` | `HT_Room_Status` | `1:1` (per room+date) | shared | `sync/mappers/room_calendar.rs` | (deferred — current readers use ht_bookings+ht_checkins reconstruction) | writeback recipes (mark_clean / walkin / extend_stay / checkout) write to MSSQL; CT hydrates PG | ✅ Track F1 / T1 HIGH-4 — canonical per-night ledger added 2026-05-13. Read path migration deferred to follow-up track |
| `ht_rates` | none (DEPRECATED) | N/A | PG-only legacy | none | none (POST/PUT/DELETE only) | `routes/new_rates.rs` (deprecated write-only) | ⚠️ DEPRECATED post-F4 — `(weekday/weekend/special)` axis is structurally wrong; superseded by `ht_rate_tiers`. Table left in place so the existing CRUD form does not error; removal follows once frontend retires |
| `ht_rate_tiers` | `HT_Rooms_Price` | `1:1` (composite key on `Room_Type` × `Cust_Type`) | shared | `sync/mappers/rate_tiers.rs` (periodic-poll, 15-min reconcile cadence) | `routes/new_rates.rs` (`GET /api/new/rates` + `GET /api/new/rate-tiers`) | none (iHOTEL is source-of-truth for pricing edits; writeback deferred to a later track) | ✅ Track F4 / T1 CRIT-4. Canonical pricing matrix mirrors legacy `HT_Rooms_Price` with `Room_Price` / `Room_Price_H` / `Room_Price_M` columns |
| `ht_settings` | `TB_SETTINGS` (or similar)? | TBD | TBD | TBD | various | manual | T1: verify settings sync |
| `ht_payments` | `HT_CheckIn_Pay` + `HT_Receipt_H` | `1:N` (one payment → one pay row + one receipt row)? | shared | TBD | `routes/new_payments.rs` | `writeback/recipes/payment.rs` | T1: VAT/transfer split + multi-payment-per-checkin |
| `ht_booking_notes` | `HT_Invoice_Note`? | TBD | TBD | TBD | TBD | TBD | T1: verify |
| `ht_inventory_*` (4 tables) | `HT_CheckIn_Product`? + housekeeping/stock tables? | TBD | TBD | TBD | TBD | TBD | T1: full mapping |
| `ht_products` | `HT_Products` | `1:1` on `prod_legacy_no = Pro_no` | shared | `sync/mappers/products.rs` (periodic poll — CT enablement on `HT_Products` deferred to a sibling `migrations/legacy-mssql/` migration) | `routes/new_products.rs` | `writeback/recipes/adjust_product_stock.rs` (additive `Pro_Amt = Pro_Amt + delta` — closes stock invariant from our app's writes; legacy continues to maintain Pro_Amt for its own sales) | ✅ Track F3 / T1 CRIT-3 (migration 041) — `ht_inventory_items.inv_product_id` FK links housekeeping/POS items to canonical product master |
| `ht_maintenance_*` (2 tables) | none (likely PG-only)? | TBD | PG canonical? | none? | `routes/new_maintenance.rs` | PG-only? | T1: confirm policy |
| `ht_users` | none | N/A | PG canonical | none | auth routes | manual / admin | ✅ PG-only by design |
| `ht_sessions` | none | N/A | PG canonical | none | session middleware | auth flow | ✅ PG-only by design |
| `ht_shifts` | `HT_Round_Bill` | `1:1` | PG canonical | none (Track G follow-up) | `routes/new_shifts.rs` | gate at `service/payment.rs::record_payment` | Track F2 / T1 HIGH-5 — migration 040 adds the canonical table + one-open-per-site partial UNIQUE index. PG is source-of-truth for now; legacy `HT_Round_Bill` mirror writeback + CT sync deferred to Track G ("round-bill shift discipline" feature). Payment gate active: `record_payment` refuses unless `current_open_shift().is_some()` |

## Legacy mirror tables (read-only snapshots, no canonical equivalent)

These exist for reconciliation/observability — they mirror legacy state
1:1 without canonicalization:

| Mirror table | Legacy source | Purpose |
|---|---|---|
| `legacy_mirror.ht_cupon` | `HT_Cupon` | Coupon print history |
| `legacy_mirror.ht_checkin_product` | `HT_CheckIn_Product` | Product orders during stay |
| `legacy_mirror.ht_deposit` | `HT_Deposit` | Deposits |
| `legacy_mirror.ht_continuetime` | `HT_ContinueTime` | Short-stay extensions |
| `legacy_mirror.ht_changed_room` | `HT_Changed_Room` | Mid-stay room changes |
| `legacy_mirror.ht_rooms_cancel` | `HT_Rooms_Cancel` | Cancelled-room ledger |
| `legacy_mirror.ht_rooms_price` | `HT_Rooms_Price` | Per-room rate snapshots |
| `legacy_mirror.ht_bill_debt_h` | `HT_Bill_Debt_H` | Debt invoice header |
| `legacy_mirror.ht_bill_debt_ds` | `HT_Bill_Debt_Ds` | Debt invoice detail |
| `legacy_mirror.ht_order_up` | `HT_Order_Up` | Order ledger (up) |
| `legacy_mirror.ht_order_down` | `HT_Order_Down` | Order ledger (down) |

T4 (feature parity) should check whether any of these mirror-only tables
need to be promoted to canonical (e.g. if our app's UI is expected to
write deposits, coupons, or product orders — today only legacy writes
them).

## Operational / app-internal tables (no legacy counterpart)

| Table | Purpose |
|---|---|
| `schema_migrations` | Migration version tracking |
| `sync_status` | Per-aggregate sync watermark (newer style) |
| `legacy_ct_state` | MSSQL Change Tracking watermark |
| `legacy_sync_status` | Per-entity sync health |
| `writeback_jobs` | Outbox queue |
| `event_log` | Event sourcing log |
| `ht_reconcile_log` | Reconcile-job divergence findings |

## Open questions for the audits to resolve

1. **T1:** Is there a `ht_checkin_rooms` junction needed? (Almost certainly yes — finding from 2026-05-12 diagnostic.) Are there other `HT_*_Ds` legacy tables that should have PG junctions?
2. **T1:** What's the canonical model for `HT_CheckIn_Pay` + `HT_Receipt_H` + `HT_Invoice_Ds` (the payment+receipt+invoice trio)? Single `ht_payments` may collapse three legacy tables.
3. **T1:** `HT_Book_H2`, `HT_Book_Ds2` — legacy secondary tables. COMPAT_CHEATSHEET says these are unused; verify.
4. **T2:** Does every CT-tracked legacy table have a sync mapper? Cross-reference with `migrations/legacy-mssql/` PK+CT migrations.
5. **T3:** Do all dashboard queries respect cardinality? `routes/rooms.rs:49-54` definitely doesn't (multi-room blind spot). What else?
6. **T4:** Reports inventory in `docs/legacy-app/REPORTS_INVENTORY.md` — which iHOTEL reports are not implemented in our app, and is that by policy?
7. **T5:** When iHOTEL writes a room status change while our writeback is mid-recipe, what conflicts? Spike covered booking-edit concurrency; broader scenarios?
