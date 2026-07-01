# Database Migrations

This directory contains SQL migration scripts for the hotel management database.

## IMPORTANT: Database Architecture Change (v2.13.0)

As of v2.13.0, the HotelNew database has been **migrated from SQL Server to PostgreSQL**. The migration files 002-007 in this directory are **historical T-SQL scripts** that were used with the original SQL Server setup. They are kept for reference but are no longer applied directly.

The canonical schema source for HotelNew is now **`init-db/init-hotelnew.sql`** (PostgreSQL syntax), which is auto-run by Docker on first startup.

## Migration Naming Convention

```
NNN_description.sql
```

- `NNN`: Sequential number (001, 002, etc.)
- `description`: Brief description using underscores (e.g., `create_booking_notes_table`)

## How to Apply Migrations

### Legacy Database (SQL Server)

1. **Review the migration** - Check if the changes are compatible with other applications
2. **Backup the database** - Always backup before applying migrations
3. **Run manually** - Execute the SQL file against the database using SSMS or sqlcmd:

```bash
# Using sqlcmd
sqlcmd -S <legacy-mssql-host> -d HotelDB -U username -P password -i migrations/001_create_booking_notes_table.sql
```

### HotelNew Database (PostgreSQL) — Automated Migrations

As of v2.14.0, HotelNew schema changes are **automatically applied** by the CI/CD pipeline via `scripts/migrate.sh`. Migration files live in `migrations/pg/`.

#### Creating a New Migration

1. **Create a migration file** in `migrations/pg/`:
   - Name format: `NNN_description.sql` (e.g., `001_add_customer_notes.sql`)
   - Use `IF NOT EXISTS` to make it idempotent
   - Include commented rollback SQL at the bottom

2. **Update `init-db/init-hotelnew.sql`** with the same changes (for fresh deployments)

3. **Update this README** — add entry to the migrations table below

4. **Push to master** — the pipeline will:
   - Create a `pg_dump` backup before applying
   - Run each pending migration in a transaction
   - Record it in the `schema_migrations` tracking table
   - Restart the backend to pick up schema changes

#### Manual Migration (development)

```bash
# Run all pending migrations locally
./scripts/migrate.sh

# Or apply a single file manually
docker exec -i new-hotel-db psql -U postgres -p 5439 -d hotelnew < migrations/pg/001_description.sql
```

#### Per-migration pragmas

Migration files MAY declare a header pragma to opt out of the default
per-migration `BEGIN`/`COMMIT` atomic wrap. The pragma is a SQL comment
that must appear in the first 20 lines of the file:

```sql
-- @transactional false
```

When detected, `migrate.sh` runs the migration body directly (no
transaction wrap) so statements like `CREATE INDEX CONCURRENTLY`,
`VACUUM`, `REINDEX CONCURRENTLY`, and similar
forbidden-inside-a-transaction operations are allowed.

**Caveat.** A non-transactional migration that fails halfway through
leaves the database in a partially-applied state. The
`schema_migrations` row is recorded in a separate follow-up statement
only after the body succeeds, so a partial failure will be re-attempted
on the next run rather than silently skipped — but the migration body
itself MUST be idempotent (e.g. `CREATE INDEX CONCURRENTLY IF NOT
EXISTS`).

Use the pragma only when the default transactional wrap is genuinely
incompatible with what the migration needs to do; otherwise leave it
off and benefit from automatic rollback on failure.

#### Drift check (init-db ↔ migrations/pg/)

Every CI run includes an `init-db-migrations-drift-check` job that
spins up a throwaway Postgres, runs `init-db/init-hotelnew.sql`, then
runs `scripts/migrate.sh`. The contract is **zero pending migrations**
on top of a fresh seed.

If the job fails with `Drift detected: init-db/init-hotelnew.sql is
out of sync with migrations/pg/.`, you forgot one of:

1. The DDL change in your new migration didn't get mirrored into
   `init-db/init-hotelnew.sql`.
2. The `INSERT INTO schema_migrations VALUES ('NNN', ...)` seed row
   didn't get added to `init-db/init-hotelnew.sql` for the new
   migration version.

Both are required for fresh deployments to land on the same schema as
upgraded ones.

#### Schema Migration Tracking

Applied migrations are tracked in the `schema_migrations` table:

| Column | Type | Description |
|--------|------|-------------|
| `version` | VARCHAR(10) UNIQUE | Migration version (e.g., `000`, `001`) |
| `filename` | VARCHAR(255) | Migration file name |
| `checksum` | VARCHAR(64) | SHA-256 of the migration file |
| `applied_at` | TIMESTAMP | When it was applied |
| `applied_by` | VARCHAR(100) | `init-script` or `migrate-script` |

Check applied migrations:
```bash
docker exec new-hotel-db psql -U postgres -p 5439 -d hotelnew -c "SELECT * FROM schema_migrations ORDER BY version;"
```

## Migration Guidelines

1. **Always use IF NOT EXISTS** - Migrations should be idempotent (safe to run multiple times)
2. **Include rollback scripts** - Add commented rollback SQL at the bottom of each file
3. **Document dependencies** - Note which tables/columns are required
4. **Test on staging first** - Never apply untested migrations to production

## Current Migrations

### Legacy Migrations (T-SQL, historical)

| # | File | Description | Applied |
|---|------|-------------|---------|
| 001 | `001_create_booking_notes_table.sql` | Creates HT_Booking_Notes table for booking annotations | v1.16.0 (deprecated) |
| 002 | `002_create_new_hotel_database.sql` | Creates new HotelNew database with all application-owned tables (T-SQL, historical) | Superseded by PG init |
| 003 | `003_alter_ht_rates_table.sql` | Alters HT_Rates table to support multiplier/fixed rate types (T-SQL, historical) | Superseded by PG init |
| 004 | `004_create_inventory_tables.sql` | Creates inventory management tables (T-SQL, historical) | Superseded by PG init |
| 005 | `005_move_booking_notes_to_hotelnew.sql` | Moves HT_Booking_Notes to HotelNew database (T-SQL, historical) | Superseded by PG init |
| 006 | `006_payment_tracking.sql` | Adds HT_Payments table for multiple payments per check-in (T-SQL, historical) | Superseded by PG init |
| 007 | `007_maintenance_system.sql` | Creates maintenance request system tables (T-SQL, historical) | Superseded by PG init |

### PostgreSQL Migrations (`migrations/pg/`)

These are automatically applied by `scripts/migrate.sh` during deployment.

| Version | File | Description | Since |
|---------|------|-------------|-------|
| 000 | `000_baseline.sql` | Baseline marker for initial schema from `init-hotelnew.sql` | v2.14.0 |
| 008 | `008_legacy_sync_tables.sql` | Legacy sync staging tables + sync_status + source columns | v2.16.0 |
| 009 | `009_widen_legacy_varchar_columns.sql` | Widen varchar columns in ht_customers_legacy to prevent truncation | v2.17.1 |
| 010 | `010_ville_cache_schema.sql` | Create `ville` schema with cached HF Ville tables for local reads | v2.22.0 |
| 011 | `011_writeback_jobs.sql` | Outbox queue for legacy MSSQL writeback (Phase 3a / 4b) | v2.8.2 |
| 012 | `012_event_log.sql` | Durable domain-event bus (Phase 3a / 4a) | v2.8.2 |
| 013 | `013_legacy_ct_state.sql` | Change Tracking watermark for the CT watcher (Phase 5) | v2.8.2 |
| 014 | `014_legacy_id_columns.sql` | Add `legacy_*` + `aggregate_id` columns to `ht_bookings`/`ht_checkins`/`ht_rooms_new` so the writeback worker's resolver can map UUID→row and back-populate allocated MSSQL identifiers | v2.31.0 |
| 015 | `015_writeback_retry_state.sql` | Add `claimed_at`/`next_retry_at` to `writeback_jobs`; introduce `exhausted` terminal status. Enables stuck-in-progress recovery, retry backoff, and Slack alerting on retry exhaustion | v2.33.0 |
| 016 | `016_writeback_notify_trigger.sql` | Auto-fire `NOTIFY writeback_channel` on every `writeback_jobs` INSERT so the worker wakes sub-second instead of waiting on the 30-second poll fallback | v2.38.0 |
| 017 | `017_legacy_sync_status.sql` | Phase 5.1 — per-table CT-watcher observability table (`legacy_sync_status`) seeded for the 10 CT-enabled tables, plus `ht_customers.cust_deleted_at` soft-delete column for upcoming HT_Customers `D` mapper | v2.43.0 |
| 018 | `018_ht_customers_aggregate_keys.sql` | Phase 5.2 — adds `legacy_cust_no` + `aggregate_id` to `ht_customers` (with partial unique indexes) so the new HT_Customers CT mapper can map MSSQL `Cust_no` → canonical PG row and emit `DomainEvent::Customer{Created,Modified}` with stable aggregate UUIDs | v2.43.1 |
| 019 | `019_ht_reconcile_log.sql` | Phase 5.5 — drift-detection tripwire for the demoted `scheduler::sync::run_sync` job. CT watcher is now authoritative; `run_sync` is downgraded to a 15-min diff-only safety net that LOGS divergent MSSQL/PG rows here (resolved manually) instead of UPSERTing canonical state | v2.45.0 |
| 020 | `020_legacy_mirror_schema.sql` | Phase 5.5a — `legacy_mirror.*` opaque pass-through schema mirroring 11 legacy-only tables (`ht_cupon`, `ht_checkin_product`, `ht_deposit`, `ht_continuetime`, `ht_changed_room`, `ht_rooms_cancel`, `ht_rooms_price`, `ht_bill_debt_h`, `ht_bill_debt_ds`, `ht_order_up`, `ht_order_down`). UI surfaces these as informational panels so receptionists don't switch to the .NET app for coupons / deposits / minibar / room moves / pricing tiers. 5.5a populates the 4 dimension tables via reconcile; 5.5c adds CT mappers for the 6 transactional tables | v2.50.0 |
| 025 | `025_drop_ville_schema.sql` | Task #77 — drop the obsolete `ville` schema in `hotelnew`. Migration 010 created it as a local cache for the FreeTDS-based `ville_sync` worker; Phase 5 Ville cutover (#76) repointed `ville_pool` to the new `hotelville` PG database, and after a 1-week soak window the schema is no longer read by any backend code path | v2.55.1 |
| 026 | `026_phase1_soak_no_op.sql` | Pure no-op migration to exercise the Phase 1 CI/CD modernization pipeline (drift-check + migrate.sh + build-backend recreate). Substitutes for the 2-week soak window; safe to leave or remove later | v2.57.2 |
| 027 | `027_create_ht_users.sql` | Phase 4 PR1 — `ht_users` table with `(user_id, username, password_hash, role, active, created_at, last_login_at)`. Argon2id PHC password hashes; role enum `admin`/`receptionist` enforced by CHECK. Foundation for local cookie-session auth (no SSO, no JWT) | v2.60.0 |
| 028 | `028_create_ht_sessions.sql` | Phase 4 PR1 — `ht_sessions` table keyed by 32-byte hex session token (`session_id` PK). FK to `ht_users` with `ON DELETE CASCADE`; tracks `expires_at` (PR1 default = login + 24h), client `ip`, `user_agent`. Index on `expires_at` for the periodic cleanup pass. PR2 will add HTTP routes + Axum middleware that read this table | v2.60.0 |
| 029 | `029_normalize_cin_status_terminal.sql` | Canonicalize `ht_checkins.cin_status` post-checkout terminal value. Flips legacy `'checked_out'` (CT-mapper output) and `'completed'` (bootstrap output) to `'checkedout'` (route-layer convention) so all readers/writers agree. Writeback contract unaffected — legacy MSSQL still receives `'Check-Out'` per the cheatsheet | v2.63.x |
| 030 | `030_add_ht_payments_legacy_columns.sql` | Wave 5a writeback audit item 3 — adds `legacy_pay_no` + `legacy_receipt_no` to `ht_payments` so the worker's `back_populate_legacy_ids` step can stamp the legacy `HT_CheckIn_Pay.Pay_no` / `HT_Receipt_H.Receipt_no` onto the canonical payment row after a successful `record_payment` recipe run. Partial unique indexes keep NULL-tolerant for pre-migration rows | v2.63.6 |
| 032 | `032_ht_reconcile_log_cardinality.sql` | Track D / T7 CRIT-1 — cardinality-aware reconcile. Adds `divergence_kind` (one of `value` / `cardinality` / `missing_pg` / `missing_mssql`) + `legacy_row_count` + `pg_row_count` to `ht_reconcile_log` so the worker can compare row counts before acking the MSSQL hash. `cardinality` and `missing_pg` divergences are never silenced — every reconcile tick re-fires until an operator repairs canonical state | v2.63.11 |
| 033 | `033_sync_status_seed_track_e1.sql` | Track E1 — seed `legacy_sync_status` for `HT_CheckIn_Other_People` (T2 HIGH-3, newly CT-enabled by legacy-mssql migration 022) and `HT_Rooms_Cancel` (T2 HIGH-5, CT enabled since Phase 5 but dangling — mapper added in 2.63.12). Without the seed the watcher's per-tick observability UPDATE would silently match zero rows | v2.63.12 |
| 034 | `034_ht_guest_registry_legacy_id.sql` | Track E1 — adds `guest_legacy_id INTEGER UNIQUE` to `ht_guest_registry` so the new `GuestRegistryMapper` can UPSERT companion-guest rows keyed on the legacy IDENTITY column. iHOTEL's DELETE-then-REINSERT edit pattern would otherwise accumulate duplicate canonical rows | v2.63.12 |
| 035 | `035_track_e2_customer_columns.sql` | Track E2 / T1 HIGH-2 — widens `ht_customers` to mirror the full legacy `HT_Customers` schema (27 new columns: `cust_price_over` running debt balance, address tuple, work-address tuple, `cust_name2`, `cust_sex`, `cust_contry`, `cust_last_change`, `cust_price_tier`, `cust_work_tax`, …). Read-only sync — writeback of `Cust_Price_Over` debt mutations deferred to Track G | v2.63.13 |
| 036 | `036_track_e2_room_columns.sql` | Track E2 / T1 HIGH-3 — widens `ht_rooms_new` with 8 legacy `HT_Rooms` columns: `room_use_count` (running nights total), `room_x` / `room_y` (drag-drop grid layout), `room_group` (floor/wing), `room_power_open` / `_close` / `_status` (electricity relay), `room_polity` (policy id). Defaults match legacy NOT NULL / DEFAULT clauses | v2.63.13 |
| 037 | `037_scheduler_notification_state.sql` | Persisted per-(site, notification type) watermark for the scheduler polling jobs (`poll_checkins` / `poll_checkouts` / `poll_new_bookings`). Fixes the post-redeploy Slack replay storm where ~45 historical events were re-paged because the in-memory watermark reset to UTC-now on every container restart (and MSSQL stores Thai local time, so UTC-now was effectively 7h behind any real event time) | v2.63.14 |
| 038 | `038_seed_vat_percent.sql` | Wave 5c (`audit-2026-05-13.md` Decision #2) — seed `ht_settings.vat_percent='7.0'` so the payment writeback recipe reads VAT rate from PG instead of the hardcoded `RECEIPT_VAT_PERCENT` constant. Threaded through `WritebackIntent::RecordPayment.vat_percent` and stamped into `HT_Receipt_H.Receipt_VatPer`. Flip rate at runtime with `UPDATE ht_settings SET setting_value='0' WHERE setting_key='vat_percent'` | v2.63.15 |
| 039 | `039_create_ht_room_calendar.sql` | Track F1 (`audit-2026-05-13.md` T1 HIGH-4) — new canonical `ht_room_calendar` table mirroring legacy `HT_Room_Status` (per-night booking-calendar ledger). `UNIQUE (rcal_room_id, rcal_date)` keyed on the business pair; FKs to `ht_rooms_new` / `ht_bookings` / `ht_checkins`; `rcal_legacy_id` captures the legacy allocator id. `sync/mappers/room_calendar.rs` hydrates the table from CT. Read-path migration deferred to a follow-up track | vNext |
| 040 | `040_create_ht_shifts.sql` | Track F2 / T1 HIGH-5 (`audit-2026-05-13.md`) — new canonical `ht_shifts` table so the receptionist app can gate `record_payment` behind an open cashier round (mirrors iHOTEL's `HT_Round_Bill` discipline). One-open-shift-per-site enforced via partial UNIQUE index `ht_shifts_one_open_per_site WHERE shift_closed_at IS NULL`. Service layer (`service/shifts.rs`) wraps the open/close/lookup API; the gate sits in `service/payment.rs::record_payment`. Routes mounted at `/api/new/shifts/{open,close,current}` + list. Legacy `HT_Round_Bill` coexistence later landed on this schema with **no further migration**: a per-tick READ-ONLY poll (`bin/sync.rs::sync_round_bills`, NOT Change-Tracking — `HT_Round_Bill` is not CT-enabled) mirrors iHOTEL's rounds into `ht_shifts` (live), and co-equal open/close writeback ships behind `ROUND_WRITEBACK_ENABLED` (default off, not yet enabled) | v2.63.16 |
| 041 | `041_create_ht_products.sql` | Track F3 (`audit-2026-05-13.md` T1 CRIT-3) — new `ht_products` canonical table mirroring legacy `HT_Products` (keyed on `prod_legacy_no = Pro_no`). Periodic-poll sync mapper at `hotel-backend/src/sync/mappers/products.rs` (CT enablement on `HT_Products` deferred to a sibling `migrations/legacy-mssql/` migration). New `WritebackIntent::AdjustProductStock` recipe issues additive `UPDATE HT_Products SET Pro_Amt = Pro_Amt + delta WHERE Pro_no=…` — closes the stock invariant from our app's writes (legacy continues to maintain Pro_Amt for its own sales). `ht_inventory_items.inv_product_id` FK links housekeeping/POS items to canonical product master | vNext |
| 042 | `042_create_ht_rate_tiers.sql` | Track F4 / T1 CRIT-4 (`audit-2026-05-13.md`) — introduce canonical `ht_rate_tiers` table keyed on `(rate_tier_room_type, rate_tier_cust_type)` mirroring legacy `HT_Rooms_Price`'s composite axis (per-customer-type pricing). Replaces the structurally wrong `(weekday/weekend/special)` axis in `ht_rates`. `ht_rates` is left in place (deprecated) so the existing /rates CRUD form keeps working; canonical reads now go through `ht_rate_tiers` via `sync::mappers::rate_tiers` (periodic-poll, 15-min reconcile cadence). | v2.63.16 |
| 043 | `043_create_ht_checkin_rooms.sql` | Track B1 / T1 CRIT-1 + T2 CRIT-1 (`audit-2026-05-13.md`) — new canonical `ht_checkin_rooms` junction table mirroring legacy `HT_CheckIn_Ds` cardinality (one row per room per check-in folio). `UNIQUE (cr_cin_id, cr_room_id)`. All operational columns from the legacy `Ds` row: `cr_room_in/out`, `cr_room_status`, `cr_rate_per_night`, `cr_nights`, `cr_room_total`, deposit fields, coupon count, note, `cr_legacy_ds_id`. Foundation only — no app behavior change. Follow-on sub-waves B2 (mapper) / B3 (dashboard) / B4 (writeback) / B5 (backfill) layer behavior on top; `ht_checkins.cin_room_id` deprecated but retained until B5 completes. | vNext |
| 044 | `044_ht_payments_refund_columns.sql` | Track G2 / T4 CRIT-1 (`audit-2026-05-13.md`) — adds `refund_of_payment_id` (self-referential FK ON DELETE SET NULL) + `refund_reason` (`VARCHAR(500)`) to `ht_payments`. Partial index on `refund_of_payment_id WHERE NOT NULL`. Enables the new `service::payment::refund_payment` + `WritebackIntent::RefundPayment` flow which inserts a negative `ht_payments` row linked back to the original. Same legacy mapping as `record_payment` (no new CARDINALITY_MAP row needed). | vNext |
| 045 | `045_create_ht_room_changes.sql` | Track G4 / T4 HIGH-3 (`audit-2026-05-13.md`) — new canonical `ht_room_changes` audit table mirroring legacy `HT_Changed_Room` (mid-stay room-move audit, 1:1 cardinality). Columns: `rc_cin_id` (FK + cascade), `rc_from_room_id`, `rc_to_room_id`, `rc_reason`, `rc_changed_at`, `rc_changed_by`, `rc_room_before_price`, `rc_to_price`, `rc_legacy_id` (partial UNIQUE on the back-link). Service path `service::checkin::change_room` validates the move + INSERTs the audit row + UPDATEs the `ht_checkin_rooms` junction in one PG transaction; outbox `WritebackIntent::RoomChange` drives `writeback::recipes::room_change` which INSERTs `HT_Changed_Room` with `OUTPUT INSERTED.id` then back-populates `rc_legacy_id`. `ChangedRoomMirrorMapper` extended to also reverse-sync canonical so moves initiated via iHOTEL land canonically. HTTP route `POST /api/new/checkins/:id/change-room`. UI: `ChangeRoomModal.tsx` + occupied-room button on dashboard / rooms page. | vNext |
| 046 | `046_user_roles_and_permissions.sql` | Track G7 / T4 HIGH-9 (`audit-2026-05-13.md`) — replaces the binary `ht_users.role` enum (admin/receptionist) with a full role + permission grid mirroring iHOTEL's Admin / Cashier / Housekeeper / Receptionist split. Adds `ht_roles`, `ht_permissions`, `ht_role_permissions`, `ht_user_roles` and a `display_name` column on `ht_users`. Relaxes the legacy single-role CHECK to also accept `'cashier'` + `'housekeeper'`. Seeds 4 roles, 6 permission keys (`payment.refund`, `checkin.room_change`, `checkin.round_bill`, `inventory.consume`, `reports.rr4`, `admin.users`), the audit T4 grant grid, plus 3 throwaway test accounts (`housekeeper_test` / `cashier_test` / `receptionist_test`, password `temp_password_2026`). Backwards-compatible: pre-existing users get a junction row backfilled from their legacy single-role column. Drives the new `middleware::permissions::require_permission` gate. | vNext |
| 047 | `047_rr4_export_log.sql` | Track G8 / T4 CRIT-2 (audit 2026-05-13) — RR.4 Thai immigration foreign-guest export audit trail. New table `ht_rr4_exports` with `(id, site, range_from, range_to, format, row_count, exported_by, exported_at, file_hash)`. `file_hash` is hex SHA-256 of the emitted bytes so regulator re-requests of a prior filing can be answered with byte-exact fidelity. CHECK constraints enforce `format IN ('csv', 'xlsx')` and `range_to >= range_from`. Indexed on `(site, range_from, range_to)` and `exported_at DESC` for the audit UI | v2.64.0 |
| 048 | `048_round_bill_shift_id.sql` | Track G9 / T4 HIGH-8 (`audit-2026-05-13.md`) — adds `cin_round_bill_shift_id BIGINT` to `ht_checkins` with FK to `ht_shifts(shift_id)` ON DELETE SET NULL + partial index `ix_ht_checkins_round_bill_shift_id` on non-NULL rows. Stamped by `service::checkin::check_out` with the resolved open-shift id so the daily cashier report can attribute revenue per shift. Extends Track F2's payment-gate (`service::payment::record_payment`) to the round-bill (fold) path — `check_out` now refuses unless `ShiftService::current_open_shift()` returns `Some`. Legacy `HT_Round_Bill` / `HT_Receipt_H` have no linking column, so the shift attribution lives entirely in canonical PG (no writeback recipe change). | vNext |
| 050 | `050_legacy_ct_state_per_table.sql` | Resilience PR R3 (2026-05-14) — adds sibling table `legacy_ct_state_per_table (table_name PK, last_seen_version, last_polled_at, last_error, last_error_at)` so the CT watcher can hold per-table watermarks. Decouples per-table progress: a row-lock wedge on one hot legacy MSSQL table (canonical: `HT_Book_H` on `Book_ID='R015142'`, 74-min global stall observed 2026-05-14) now freezes only that row instead of gating every CT-enabled table. Backfills one row per table seeded from the current global `legacy_ct_state.last_seen_version`. Consulted only when `SYNC_PER_TABLE_WATERMARK=true` (default false); the single-row `legacy_ct_state` stays operational on the global path so the migration can land without changing runtime behaviour. Migration applies to both `hotelnew` (HF Hotel) and `hotelville` (HF Ville) via `scripts/migrate.sh --site`. | vNext |
| 051 | `051_create_ht_coupons.sql` | Track G5 — canonical `ht_coupons` mirror of legacy `HT_Cupon` food/breakfast coupon entitlement table. BIGSERIAL PK, partial UNIQUE on `legacy_cupon_no WHERE NOT NULL`, status enum (issued/redeemed/expired/cancelled), `aggregate_id UUID UNIQUE` for outbox / event-bus contracts, `source` enum (canonical/legacy) to distinguish iHOTEL-issued from new-app-issued. Seeds Track G7 permissions `coupon.issue` (admin + receptionist) and `coupon.redeem` (admin + cashier + receptionist). Service path `service::coupon::{issue_coupon, redeem_coupon}` → outbox `WritebackIntent::{IssueCoupon, RedeemCoupon}` → recipe `INSERT/UPDATE [HT_Cupon]` with TABLOCKX+HOLDLOCK MAX+1 cupon_no allocator. Mirror mapper (`CuponMirrorMapper`) extended to dual-write canonical via `apply_canonical_cupon_event` (Track G4 `ChangedRoomMirrorMapper` pattern). HTTP routes: `POST /api/new/coupons`, `POST /api/new/coupons/{code}/redeem`, `GET /api/new/coupons`. UI: `IssueCouponModal.tsx`. | vNext |
| 054 | `054_comment_ht_reconcile_log_semantic.sql` | Documentation-only — adds a PostgreSQL `COMMENT ON TABLE ht_reconcile_log` clarifying that rows are sync-lag observations (transient until the auto-resolve sweep converges them), not durable divergence. The "drift" framing in the historical table name and earlier Slack templates misled operators; this comment + the reworded Slack templates (sync.rs) capture the actual semantic without a costly rename | vNext |
| 055 | `055_add_legacy_id_to_ht_customers.sql` | Customer hard-delete handling (audit 2026-06-11 P1 #6) — adds `legacy_id INTEGER` + partial index to `ht_customers`, mirroring the legacy `HT_Customers.id` SERIAL PK. CT D-rows carry ONLY that key (the LEFT JOIN against the deleted row nulls `Cust_no`), so `apply_soft_delete` resolved nothing and every iHOTEL FrmManageCustomersNew delete was a silent no-op. The customer mapper now persists `legacy_id` on every I/U apply (and via the eager-mirror path) and resolves D-rows by it first, falling back to `Cust_no`. Rows mirrored pre-055 carry NULL until their next CT touch backfills it — deletes of those rows log a loud WARN instead of failing. One-shot closure of the pre-055 NULL population: `backfill_customer_legacy_ids` bin (see Companion utility bins below) | vNext |
| 056 | `056_legacy_mirror_ht_book_pro.sql` | Phase 5/E2 (coexistence audit 2026-06-11 P2) — new `legacy_mirror.ht_book_pro` opaque pass-through mirror of legacy `HT_Book_Pro` (pre-booked products attached to a booking by FrmAddBook2; cheatsheet §3.4 step 3.5). Companion to `migrations/legacy-mssql/023_book_pro_ct.sql` (PK + CT enable) and the new `BookProMirrorMapper`. Also seeds the watcher plumbing for the new table: a `legacy_sync_status` row (migration 033 pattern) and a `legacy_ct_state_per_table` row seeded from the CURRENT global `legacy_ct_state` watermark (migration 050 backfill pattern) so per-table-watermark mode enters the stream "from now" instead of tripping the `check_retention` min-valid overflow at watermark 0. Pre-existing rows arrive via `--bootstrap`, never CT replay (HT_Book_Pro has 0 rows at HF Hotel per schema-baseline.txt) | vNext |
| 059 | `059_create_ht_cash_ledger.sql` | Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — net-new feature parity with iHOTEL's FrmPayMain / FrmAddPay (the shift round report mirrors `HT_CheckIn_Pay`, which is folio payments, NOT petty cash). Two new canonical tables: `ht_cash_ledger` (per-line mirror of legacy `TB_Pay_History`; `Pay_Date`/`Pay_Program` are legacy OADate floats converted to `TIMESTAMPTZ`; raw `Pay_Type` preserved in `cash_legacy_type`, normalized `cash_kind` income/expense/unknown; `cash_source` legacy/app; keyed `UNIQUE (cash_legacy_id)`) and `ht_cash_categories` (the 3-level account taxonomy `TB_SET_MyType2`/`_2_2`/`3` collapsed via a `cat_level` discriminator, keyed `UNIQUE (cat_level, cat_legacy_id)`). Inbound sync is a READ-ONLY per-tick poll (`bin/sync.rs::sync_cash_history` + `sync_cash_categories` — like `sync_round_bills`, NOT Change-Tracking, so NO legacy DDL). HTTP routes `GET /api/cash`, `POST /api/cash/{income,expense}`, `GET /api/cash/categories` (branch-aware). Writeback to `TB_Pay_History` is captured as a PURE recipe (`writeback::recipes::cash_entry`) but left UNWIRED behind a TODO pending byte-shape verification of `Pay_Type`/`Pay_Group`/`Pay_Account`/`Pay_Program` against the FrmAddPay decompile. Per-site (connection-level scoping). | vNext |
| 058 | `058_add_shift_cash_count.sql` | Track J7c (round reconciliation) — add `shift_counted_cash NUMERIC(14,2)` + `shift_cash_count JSONB` to `ht_shifts` for the cash-drawer count at round close. The round report (`GET /api/shifts/{id}/report`) computes `expected_cash = shift_opening_float + SUM(ledger_cash over the window)`, `counted_cash` (server-computed from the denomination map), and the variance. ALTER only (no new table). Per-site. | vNext |
| 057 | `057_create_ht_payment_ledger.sql` | Track J7a (round reconciliation) — new canonical `ht_payment_ledger`, a per-line mirror of legacy `HT_CheckIn_Pay` carrying every tender split (`cash`/`credit`/`free`/`tran`/`web`) + line category (`ledger_ds_id`, `P001`=room) + `Cin_Status`. The existing payment sync only refreshes a check-in's *paid total* (`cin_paid_amount`); this captures the per-tender detail iHOTEL's `ReportShipCash`/`ReportIncome2` sum. Populated per-line by the `PaymentMapper` coalesced path (re-mirrors all lines of a changed `Cin_No`); keyed `UNIQUE (ledger_legacy_id)` = `HT_CheckIn_Pay.id`. Source for the round-close income-by-tender summary + shift report. Per-site (connection-level scoping, like `ht_shifts`); NO legacy schema change (HT_CheckIn_Pay is already CT-enabled). | vNext |
| 052 | `052_create_ht_pos_sales.sql` | Track G6 / `audit-2026-05-13.md` standalone-readiness — new canonical `ht_pos_sales` table mirroring legacy `HT_CheckIn_Product` cardinality (1:N — one row per line item per check-in folio). Columns: `sale_cin_id` (FK + CASCADE), `sale_product_id` (FK to `ht_products`, no cascade), `sale_qty` / `sale_unit_price` / `sale_total` (STORED generated `qty × unit_price`), `sale_sold_at`, `sale_sold_by`, `sale_note`, `sale_status` (`posted`/`voided`), `sale_legacy_id` (partial UNIQUE on back-link), `source` (`canonical`/`legacy`), `aggregate_id UUID UNIQUE`. Service path `service::pos::record_sale` validates folio + decrements `ht_products.prod_current_stock` + INSERTs the audit row + emits `WritebackIntent::RecordPosSale` in one PG transaction; recipe `writeback::recipes::pos_sale` INSERTs `HT_CheckIn_Product` with `OUTPUT INSERTED.id` and back-populates `sale_legacy_id` plus a paired additive `Pro_Amt` UPDATE so stock invariant holds across both DBs. Reverse-sync via extended `CheckinProductMirrorMapper` so sales rung up in iHOTEL land canonically. Also seeds new `pos.sell` permission (admin + cashier grants) so Track G7 middleware can gate the route. HTTP routes `POST /api/new/checkins/:id/pos-sale` + `GET /api/new/checkins/:id/pos-sales`. | v2.66.1 |
| 061 | `061_create_ht_booking_products.sql` | Task #52 (reservation enhancements) — new canonical `ht_booking_products` table for products pre-ordered at the moment a booking is taken. Canonical analog of legacy `HT_Book_Pro` (FrmAddBook2 pre-booked products; mirrored read-only in migration 056). Keyed on the booking (`bp_book_id` FK + CASCADE) rather than the check-in folio (the `ht_pos_sales` analog) because a pre-order precedes check-in. Columns mirror `ht_pos_sales`: `bp_product_id` (FK to `ht_products`, no cascade), `bp_qty` / `bp_unit_price` / `bp_total` (STORED generated `qty × unit_price`), `bp_note`, `bp_legacy_id` (partial UNIQUE back-link — reserved for a future write-back), `source` (`canonical`/`legacy`), `aggregate_id UUID` (nullable partial UNIQUE). Write path: `routes::new_bookings::create_booking` → `service::BookingService::create` inserts the lines in the same TX as the booking (runtime `sqlx::query`, no `.sqlx` cache). **Legacy write-back deferred** — the `HT_Book_Pro` INSERT shape is unverified (`docs/legacy-spike/findings.md` captures only a DELETE; the table is empty in both DBs), so pre-orders are canonical-only and the legacy mirror remains read-only for now. | vNext |
| 062 | `062_create_ht_notes.sql` | Task #47 (room & staff sticky notes — โน้ตห้อง / โน้ตพนักงาน) — new canonical `ht_notes` table mirroring legacy `HT_Room_SMS` + `HT_EMP_SMS` (identical shape, `SMS_Readed` read-flag flow; cheatsheet §932-942 / §3.22). One table covers both, discriminated by `note_target_kind` ('room' = HT_Room_SMS keyed `SMS_Room`/room_no; 'staff' = HT_EMP_SMS keyed `SMS_TO`/username) — same collapse-by-discriminator as `ht_cash_categories`. Columns: `note_target_key` (verbatim legacy key), `note_body` (`SMS_Details`), `note_created_by` (`SMS_By`), `note_is_read` (normalized `SMS_Readed`), `note_legacy_id` (`SMS_ID` IDENTITY back-pointer; UNIQUE per `(note_target_kind, note_legacy_id)` since the two tables have independent IDENTITY sequences), `note_source` (legacy/app), `aggregate_id UUID` (partial UNIQUE — writeback correlation). Inbound sync is a READ-ONLY per-tick poll (`bin/sync.rs::sync_sticky_notes` — like `sync_round_bills`/`sync_cash_history`, NOT Change-Tracking, so NO legacy DDL). HTTP routes `GET /api/notes`, `POST /api/notes`, `POST /api/notes/{id}/read` (branch-aware). Writeback to the legacy SMS tables is a verified recipe (`writeback::recipes::sticky_note` — `INSERT … OUTPUT INSERTED.SMS_ID` + `UPDATE … SET SMS_Readed='yes'`) wired through `WritebackIntent::{CreateNote, MarkNoteRead}` but **SHIPPED DARK** behind `NOTES_WRITEBACK_ENABLED` (default off — app notes persist canonical-only until a reception-coordinated live test). Per-site (connection-level scoping). | vNext |
| 065 | `065_create_ht_pos_receipts.sql` | Task #45 (POS walk-up sale + standalone receipt) — two new canonical tables mirroring the legacy receipt pair: `ht_pos_receipts` (header, 1:1 with `HT_Receipt_H`) + `ht_pos_receipt_lines` (lines, N:1 with `HT_Receipt_Ds`, FK + CASCADE). A walk-up / roomless sale (no active check-in) writes here instead of `ht_pos_sales` (whose `sale_cin_id` is `NOT NULL`). Header columns mirror `HT_Receipt_H`: customer block (`receipt_customer_no/name/addr/tel`, `receipt_tax_id`), money (`receipt_subtotal/discount/total/before_vat/vat/vat_percent`), `receipt_paid` + `receipt_payment_method` (direct-pay), `receipt_status` (`posted`/`voided`), `receipt_legacy_id` (`HT_Receipt_H.id`, partial UNIQUE) + `receipt_legacy_no` (`B{yyMM}-{4digit}`), `source`, `aggregate_id UUID UNIQUE`. Line columns mirror `HT_Receipt_Ds`: `line_product_id` (FK to `ht_products`, no cascade), `line_product_no/name/unit_name`, `line_qty`/`line_unit_price`/`line_discount`/`line_total` (computed `qty × unit_price − discount`). Write path: `service::pos::record_walkup_sale` validates + decrements `ht_products.prod_current_stock` per line + INSERTs header + lines + emits `WritebackIntent::RecordReceipt` in one PG TX; recipe `writeback::recipes::receipt` allocates `HT_Receipt_H.id` (TABLOCKX MAX+1) + `Receipt_no`, INSERTs `HT_Receipt_H` + N `HT_Receipt_Ds` + a paired additive `HT_Products.Pro_Amt` decrement per line. Void path: `service::pos::void_sale` flips `ht_pos_sales.sale_status='voided'` + restores stock + `WritebackIntent::VoidPosSale` (`writeback::recipes::pos_void` — guarded DELETE `HT_CheckIn_Product` + restore `Pro_Amt`). HTTP routes `POST /api/new/pos/walkup-sale` + `POST /api/new/pos/sales/:id/void` (branch-aware via `routes::new_pos::service_for`). Per-site (connection-level scoping). | vNext |
| 064 | `064_booking_reminders.sql` | Task #53 (availability calendar grid + booking reminders) — adds three PG-CANONICAL-ONLY reminder columns to `ht_bookings`: `book_notify_day INTEGER` (per-booking reminder lead time in days before check-in; NULL ⇒ endpoint default of 3), `book_notify_note TEXT` (free-form reminder copy for the bell, distinct from `book_notes`/`book_internal_notes`), and `book_notify_dismissed_at TIMESTAMPTZ` (NULL ⇒ active; stamped by the dismiss endpoint). Plus the partial index `ix_ht_bookings_active_reminders ON (book_checkin) WHERE book_notify_dismissed_at IS NULL`. Backs `GET /api/calendar/reminders` (upcoming arrivals + balance-due bookings, branch-aware) and `POST /api/calendar/reminders/{id}/dismiss` (`routes::new_calendar`), the in-shell notification bell (`components/v2/ReminderBell`), and the balance-due filter on the reservations list (`?balanceDue=true` → `repository::booking::list_with_count`). The availability grid itself (`app/v2/calendar`) is a pure READ over the existing `GET /api/calendar` and needs no schema change. **No legacy writeback** — iHOTEL has no equivalent stored reminder/dismiss flag; TODO(coexistence) noted in the migration if one is ever identified. ALTER-only (no new table → no `CARDINALITY_MAP.md` row). Per-site (connection-level scoping). | vNext |
| 066 | `066_create_ht_verification_responses.sql` | Task #58 (in-app reception verification form — คู่มือตรวจสอบระบบใหม่ สำหรับฝ่ายต้อนรับ) — new canonical `ht_verification_responses` table, the ONLINE equivalent of the printed checklist `docs/coexistence/reception-verification-TH.html` (§1 screen-compare, §2 policy questions, §3 last-5-bills total check, §4 coordinated live tests, §5 overall readiness). Answers land in PostgreSQL so IT can query them instead of reading photos of paper forms. Columns: `vr_submitted_at`, `vr_site` (branch the form was submitted for), `vr_inspector` (ผู้ตรวจ — session username when auth is on, else typed), `vr_answers JSONB NOT NULL` (EVERY choice/text answer keyed by question id — `q1_1`, `q1_2_reasons`, `q3_bills`, … — so the checklist can gain/lose questions without a migration and IT can query `vr_answers->>'q2_1'`), `vr_overall` (§5 verdict denormalized for a ready/not-ready filter). Plus the recent-first index `ix_ht_verification_responses_submitted_at`. HTTP routes `GET /api/verification` (recent list for IT review) + `POST /api/verification` (submit one checklist) — branch-aware via `state.write_pool(branch)` (`routes::new_verification`). **PG-CANONICAL ONLY** — iHOTEL has no counterpart (legacy = none, like `ht_users`), so there is **NO sync mapper and NO legacy writeback**. Per-site (connection-level scoping). | vNext |
| 067 | `067_create_ht_feedback_forms.sql` | Tier 1 data-driven feedback forms (follow-on to #58/#71) — new canonical `ht_feedback_forms` table so reception feedback / re-verification form DEFINITIONS live in data, not code: a generic frontend renderer fetches a form's `form_schema` JSONB (the question list) and renders whatever it says, so editing/adding a question is a DB write (no frontend image rebuild + CI/CD). Columns: `form_key` (stable lookup id, UNIQUE), `form_site`, `form_kind` (groups responses, written into `vr_answers.kind`), `form_title`/`form_intro`, `form_schema JSONB NOT NULL` (`{"questions":[{id,type,label,required,options,showIf}]}`), `form_active`, `form_sort`. Seeds the two per-site re-verify forms (`reverify_hfhotel`/`reverify_hfville`). Read: `GET /api/feedback/forms` (+ `/:key`) — `routes::new_feedback`, primary pool. Answers still land in `ht_verification_responses.vr_answers` via `POST /api/verification` (tagged `kind=form_kind`). **PG-CANONICAL ONLY** (no legacy, no sync, no writeback). Per-site (connection-level scoping). | vNext |
| 068 | `068_ht_payments_legacy_receipt_unique.sql` | Payment-dedup hardening (issue #203) — partial UNIQUE index `ux_ht_payments_cin_legacy_receipt_no` on `ht_payments (pay_cin_id, legacy_receipt_no) WHERE legacy_receipt_no IS NOT NULL`. A legacy receipt maps to exactly one canonical payment per check-in; this blocks retried-sync / crash-after-commit replays from landing the same legacy receipt twice on the same check-in (double-counting in round reports / payment ledgers). NULL `legacy_receipt_no` rows (app-native payments) are exempt via the partial predicate; the existing non-unique `ix_ht_payments_legacy_receipt_no` stays for `pay_cin_id`-less lookups. **INDEX only (no new table, no CARDINALITY_MAP entry).** **Pre-flight MANDATORY**: creation FAILS if existing `(pay_cin_id, legacy_receipt_no)` non-NULL duplicates exist — check BOTH `hotelnew` + `hotelville` before deploy. Per-site (connection-level scoping). | vNext |
| 069 | `069_ht_customers_dob.sql` | Check-in registration — guest date of birth. Adds `cust_dob DATE` to `ht_customers` (idempotent `ADD COLUMN IF NOT EXISTS`). Captured from the Thai ID chip read / passport MRZ parse during check-in registration. **PG-CANONICAL ONLY** — legacy `HT_Customers` has NO date-of-birth column, so this field is never mirrored to MSSQL and the customer sync mapper does not read it; it rides the existing customer resave/enrichment path (dynamic `sqlx::query` COALESCE update). ALTER-only (no new table → no CARDINALITY_MAP row). Per-site (connection-level scoping). | vNext |
| 070 | `070_ht_guest_documents.sql` | Check-in registration — guest identity document / photo storage. New canonical `ht_guest_documents` table (1:N canonical home for legacy `Tb_Save_Image`) holding the captured artifacts (Thai ID card chip image, passport page, webcam face photo) as `doc_image BYTEA` + `doc_mime`, discriminated by `doc_type` (`thai_id_card` / `passport` / `face_photo`) and `doc_source` (`chip` / `scanner` / `webcam`), with optional FKs to `ht_customers(cust_id)` + `ht_checkins(cin_id)` and indexes on both (`idx_ht_guest_documents_cin` / `_cust`). Legacy-linkage columns: `doc_legacy_tmp_no` is the provisional `Tb_Save_Image.tmp_no` minted here and echoed to the check-in POST as `photoTmpNo` (the existing check-in writeback links the legacy image row by `tmp_no`); `doc_legacy_id` is the `Tb_Save_Image.id` back-populated after the mirror runs. `doc_type` → `Tb_Save_Image.ttype`: `thai_id_card`=บัตรประชาชน, `face_photo`=รูปลูกค้า, `passport`=หนังสือเดินทาง. Read path `routes/guest_documents.rs`; legacy mirror `writeback/recipes/save_image.rs` **SHIPPED DARK** behind `GUEST_DOCUMENT_STORAGE_ENABLED` (default off — documents persist canonical-only until the flag is on). Per-site (connection-level scoping). | vNext |

## Tables Owned by This Application

### Legacy Database (db)

**IMPORTANT**: The legacy database should be treated as READ-ONLY. Do not create or modify any tables in the legacy database.

Previously, `HT_Booking_Notes` was created in the legacy database. As of v2.2.0, it has been moved to HotelNew to enforce the read-only principle.

### New Database (HotelNew - PostgreSQL)

All table and column names are **lowercase** (PostgreSQL convention). The canonical schema is defined in `init-db/init-hotelnew.sql`.

| Table | Description | Since |
|-------|-------------|-------|
| `ht_customers` | Customer information (replaces View_Customers) | v2.0.0 |
| `ht_room_types` | Room type definitions (Standard, Deluxe, Suite, etc.) | v2.0.0 |
| `ht_rooms_new` | Room information (replaces HT_Rooms) | v2.0.0 |
| `ht_bookings` | Booking records (replaces View_Booking_Ds) | v2.0.0 |
| `ht_booking_rooms` | Junction table linking bookings to rooms | v2.0.0 |
| `ht_checkins` | Check-in records (replaces View_CheckIn_Ds — header level only; `cin_room_id` DEPRECATED post-Track B1 in favor of `ht_checkin_rooms` junction, retained until B5 backfill completes) | v2.0.0 |
| `ht_checkin_rooms` | Per-room junction mirroring legacy `HT_CheckIn_Ds` cardinality (one row per room per check-in folio). Track B1 / migration 043. Schema-only landing — mapper (B2) / dashboard (B3) / writeback (B4) / backfill (B5) follow on | vNext |
| `ht_room_changes` | Mid-stay room-move audit (1:1 with legacy `HT_Changed_Room`). Track G4 / migration 045. Write path: `service::checkin::change_room` → outbox `WritebackIntent::RoomChange` → `writeback::recipes::room_change`. Reverse-sync via extended `ChangedRoomMirrorMapper` (legacy-origin rows also land canonically). | vNext |
| `ht_pos_sales` | POS / sales-to-room line items (1:N with legacy `HT_CheckIn_Product`). Track G6 / migration 052. Write path: `service::pos::record_sale` → outbox `WritebackIntent::RecordPosSale` → `writeback::recipes::pos_sale` (single recipe issues both the `HT_CheckIn_Product` INSERT and a paired additive `HT_Products.Pro_Amt` decrement). Reverse-sync via extended `CheckinProductMirrorMapper` so legacy-origin sales also land canonically. | vNext |
| `ht_booking_products` | Products pre-ordered against a booking (1:N). Task #52 / migration 061. Canonical analog of legacy `HT_Book_Pro` (read-only mirror in migration 056), keyed on the booking instead of the check-in folio. Write path: `routes::new_bookings::create_booking` → `service::BookingService::create` inserts the lines in the booking's TX via `repository::booking::insert_booking_product` (runtime `sqlx::query`). Legacy write-back DEFERRED (HT_Book_Pro INSERT shape unverified) — canonical-only for now. | vNext |
| `ht_pos_receipts` | POS walk-up / standalone receipt header (1:1 with legacy `HT_Receipt_H`). Task #45 / migration 065. Write path: `service::pos::record_walkup_sale` → outbox `WritebackIntent::RecordReceipt` → `writeback::recipes::receipt`. | vNext |
| `ht_pos_receipt_lines` | POS standalone receipt lines (N:1 with legacy `HT_Receipt_Ds`, FK + CASCADE to `ht_pos_receipts`). Task #45 / migration 065. | vNext |
| `ht_notes` | Room & staff sticky notes (โน้ตห้อง / โน้ตพนักงาน). Task #47 / migration 062. Canonical mirror of legacy `HT_Room_SMS` + `HT_EMP_SMS` collapsed via `note_target_kind` ('room'/'staff'). Inbound: `bin/sync.rs::sync_sticky_notes` per-tick poll (NOT CT). Read/write: `routes::new_notes` (`GET /api/notes`, `POST /api/notes`, `POST /api/notes/{id}/read`, branch-aware — canonical insert/update + in-tx outbox enqueue on the resolved per-site pool). Writeback to the legacy SMS tables via `WritebackIntent::{CreateNote, MarkNoteRead}` + `writeback::recipes::sticky_note` is SHIPPED DARK behind `NOTES_WRITEBACK_ENABLED` (default off). Per-site (connection-level scoping). | vNext |
| `ht_guest_registry` | Guest registry for TM30 compliance | v2.0.0 |
| `ht_guest_documents` | Guest identity documents / photos captured at check-in registration (Thai ID card chip image, passport page, webcam face photo). Migration 070. Canonical home for legacy `Tb_Save_Image` (1:N) — `doc_image BYTEA` + `doc_mime`, discriminated by `doc_type` (`thai_id_card`/`passport`/`face_photo`) and `doc_source` (`chip`/`scanner`/`webcam`). `doc_legacy_tmp_no` is the provisional `Tb_Save_Image.tmp_no` echoed to the check-in POST as `photoTmpNo`; `doc_legacy_id` is the `Tb_Save_Image.id` back-populated after the mirror runs. Read: `routes::guest_documents`. Legacy mirror `writeback::recipes::save_image` SHIPPED DARK behind `GUEST_DOCUMENT_STORAGE_ENABLED` (default off). Per-site (connection-level scoping). | vNext |
| `ht_rates` | Room rates by type and date range — DEPRECATED post-F4; superseded by `ht_rate_tiers` (canonical axis is `Room_Type` × `Cust_Type`, not weekday/weekend/special). Left in place so the existing /rates CRUD form does not break; removal in a follow-on once the frontend retires | v2.0.0 |
| `ht_rate_tiers` | Canonical `(Room_Type, Cust_Type)` pricing matrix mirrored from legacy `HT_Rooms_Price` (per-night, per-hour, per-month columns). Read path for booking/check-in price resolution post-Track F4 | v2.63.16 |
| `ht_settings` | Application settings key-value store | v2.0.0 |
| `ht_inventory_categories` | Inventory categories (Minibar, Amenities, Linens, Equipment) | v2.1.0 |
| `ht_inventory_items` | Inventory items with stock tracking | v2.1.0 |
| `ht_room_inventory` | Items assigned to each room | v2.1.0 |
| `ht_inventory_transactions` | Stock movement transactions (IN, OUT, ADJUST, MOVE) | v2.1.0 |
| `ht_booking_notes` | Booking notes (moved from legacy DB in v2.2.0) | v2.2.0 |
| `ht_payments` | Payment records for check-ins (multiple payments per stay) | v2.10.0 |
| `ht_maintenance_categories` | Maintenance categories (Electrical, Plumbing, AC, Furniture, General) | v2.11.0 |
| `ht_maintenance_requests` | Maintenance request records with status tracking | v2.11.0 |
| `ht_room_calendar` | Canonical per-night booking-calendar ledger (mirrors legacy `HT_Room_Status`). Track F1 / migration 039 | vNext |
| `ht_verification_responses` | In-app reception verification checklist responses (task #58 / migration 066). Online equivalent of `docs/coexistence/reception-verification-TH.html`; every answer in the `vr_answers` JSONB column keyed by question id. PG-CANONICAL ONLY (no legacy counterpart, no sync, no writeback). Read/write: `routes::new_verification` (`GET` / `POST /api/verification`, branch-aware). Per-site (connection-level scoping). | vNext |
| `ht_feedback_forms` | Data-driven reception feedback / re-verification form definitions (Tier 1 / migration 067). `form_schema` JSONB drives a generic renderer so question edits are a DB write, not a frontend rebuild. PG-CANONICAL ONLY (no legacy, no sync, no writeback). Read: `routes::new_feedback` (`GET /api/feedback/forms`). Responses land in `ht_verification_responses`. Per-site (connection-level scoping). | vNext |
| `schema_migrations` | Migration version tracking (applied by migrate.sh) | v2.14.0 |
| `ht_rooms_legacy` | Legacy rooms mirror (synced from HT_Rooms) | v2.16.0 |
| `ht_bookings_legacy` | Legacy bookings mirror (synced from View_Booking_Ds) | v2.16.0 |
| `ht_checkins_legacy` | Legacy check-ins mirror (synced from View_CheckIn_Ds) | v2.16.0 |
| `ht_customers_legacy` | Legacy customers mirror (synced from View_Customers) | v2.16.0 |
| `sync_status` | Background sync health tracking per entity type | v2.16.0 |
| `writeback_jobs` | Outbox queue for legacy MSSQL writeback (one row per pending intent) | v2.8.2 |
| `event_log` | Durable domain-event bus (every state-mutating action emits one row) | v2.8.2 |
| `legacy_ct_state` | Single-row Change Tracking watermark consumed by `bin/sync.rs` | v2.8.2 |
| `legacy_sync_status` | Per-table CT-watcher observability — rows ingested/skipped, last error, consecutive failure count | v2.43.0 |
| `ht_reconcile_log` | Sync-lag observation queue (despite the name). Rows are snapshots of moments when the diff-only reconcile sweep noticed the legacy and canonical hashes had not yet converged; the auto-resolve sweep re-hashes both sides at every reconcile tick and closes converged rows. Only rows that resist multiple sweep cycles (>4h unresolved) represent actual durable divergence. Migration 054 adds a `COMMENT ON TABLE` capturing this semantic | v2.45.0 |
| `ht_users` | Local username + Argon2id password hash + role (`admin` / `cashier` / `housekeeper` / `receptionist` post-Track G7 / migration 046; widened from binary `admin/receptionist`) + optional `display_name`. Backs Phase 4 cookie-session authentication (PR1 introduces the schema; PR2 adds HTTP login routes; G7 adds role + permission grid) | v2.60.0 |
| `ht_roles` | Catalog of canonical user roles. Track G7 / migration 046 — seeded with `admin`, `cashier`, `housekeeper`, `receptionist` mirroring iHOTEL's role split. PG-only (no legacy MSSQL counterpart) | vNext |
| `ht_permissions` | Catalog of permission keys (e.g. `payment.refund`, `checkin.room_change`). Track G7 / migration 046. Routes / UI gate on permission keys, not roles directly, so the grant grid can be re-tuned without code changes once an admin UI lands. PG-only | vNext |
| `ht_role_permissions` | Role → permission junction (many-to-many). Track G7 / migration 046. Seeded grid follows audit T4. PG-only | vNext |
| `ht_user_roles` | User → role junction (many-to-many). Track G7 / migration 046 — a user may hold several roles (e.g. an admin covering cashier shifts). Existing single-role users backfilled from `ht_users.role` at migration time. PG-only | vNext |
| `ht_sessions` | Active server-side sessions keyed by HttpOnly cookie token. `ON DELETE CASCADE` from `ht_users`; `expires_at` index drives periodic cleanup. (Phase 4 PR1) | v2.60.0 |
| `ht_rr4_exports` | Track G8 audit trail — one row per RR.4 / ตม.30 Thai immigration export attempt with `(site, range_from, range_to, format, row_count, exported_by, exported_at, file_hash)`. `file_hash` is hex SHA-256 of the emitted bytes for regulator-side non-repudiation | v2.64.0 |
| `scheduler_notification_state` | Persisted Slack watermark per (site, notification type) consumed by `scheduler::jobs::{poll_checkins, poll_checkouts, poll_new_bookings}`. Restored across redeploy so the polling jobs don't re-page historical events on container restart | v2.63.14 |
| `ht_shifts` | Cashier-shift canonical (Track F2 / T1 HIGH-5) — gates `service::payment::record_payment` so no payment is accepted unless an open shift exists for the binary's `SITE_ID`. One-open-shift-per-site enforced by the partial UNIQUE index `ht_shifts_one_open_per_site`. Legacy `HT_Round_Bill` coexistence now rides on this same schema (no new migration): a READ-ONLY per-tick poll (`bin/sync.rs::sync_round_bills` — `HT_Round_Bill` is polled, NOT CT-enabled) upserts iHOTEL's rounds keyed `ON CONFLICT (shift_site_id, shift_no)` with `shift_no = shift_legacy_round_id = HT_Round_Bill.id` (live); co-equal open/close writeback to `HT_Round_Bill` is shipped behind `ROUND_WRITEBACK_ENABLED` (default off, not yet enabled) | v2.63.16 |
| `ht_coupons` | Canonical food/breakfast/promo coupons (mirror of legacy `HT_Cupon`). Track G5 / migration 051. BIGSERIAL PK + partial UNIQUE on `legacy_cupon_no`. Write path: `service::coupon::{issue_coupon, redeem_coupon}` → outbox `WritebackIntent::{IssueCoupon, RedeemCoupon}` → `writeback::recipes::coupon` (`HT_Cupon` INSERT + `cupon_print=1` UPDATE). Reverse-sync via extended `CuponMirrorMapper` (legacy-issued coupons hydrate canonically via `apply_canonical_cupon_event`). | vNext |
| `ht_payment_ledger` | Track J7a — per-line canonical mirror of legacy `HT_CheckIn_Pay` (all tender splits cash/credit/free/tran/web + category + `Cin_Status`), keyed `UNIQUE (ledger_legacy_id)`. Read-only mirror (no writeback): populated per-line by the sync worker's `PaymentMapper` coalesced path, which re-mirrors all lines of a changed `Cin_No`. Source for the round-close income-by-tender reconciliation + the iHOTEL-equivalent shift report; contains BOTH apps' payments (our app's payments also write `HT_CheckIn_Pay` and CT-sync back). Per-site (connection-level scoping). Migration 057. | vNext |
| `ht_cash_ledger` | Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — canonical mirror of legacy `TB_Pay_History` (income/expense general ledger; separate from folio payments). Columns mirror the 10 legacy cols with OADate floats (`Pay_Date`/`Pay_Program`) converted to `TIMESTAMPTZ`, raw `Pay_Type` preserved in `cash_legacy_type` + normalized `cash_kind` (income/expense/unknown), and `cash_source` (legacy/app). Keyed `UNIQUE (cash_legacy_id)` (NULL for app-created-not-yet-written rows). Populated by `bin/sync.rs::sync_cash_history` (READ-ONLY per-tick poll, NOT CT) and `POST /api/cash/{income,expense}`. Writeback to `TB_Pay_History` is a PURE-but-UNWIRED recipe (`writeback::recipes::cash_entry`, TODO pending byte-shape verification). Per-site (connection-level scoping). Migration 059. | vNext |
| `ht_cash_categories` | Cash-ledger account taxonomy — canonical mirror of legacy `TB_SET_MyType2` / `TB_SET_MyType2_2` / `TB_SET_MyType3` (the 3-level account tree), collapsed into one table via a `cat_level` discriminator ('2'/'2_2'/'3'), keyed `UNIQUE (cat_level, cat_legacy_id)`. Read-only mirror: populated by `bin/sync.rs::sync_cash_categories` (per-tick poll, NOT CT). Used by `GET /api/cash/categories` to populate the income/expense entry-form dropdowns. Per-site (connection-level scoping). Migration 059. | vNext |

## Tables Used (Read-Only or Shared)

These tables and views are owned by the legacy application.

| Table/View | Usage |
|------------|-------|
| `HT_Rooms` | Room information |
| `View_Booking_Ds` | Booking records |
| `View_CheckIn_Ds` | Check-in records |
| `View_Customers` | Customer information |

### PROHIBITED Actions on Legacy Tables/Views

**DO NOT** perform any of the following on tables/views not owned by this app:

- `ALTER TABLE` - Do not add, modify, or drop columns
- `ALTER VIEW` - Do not modify view definitions
- `DROP TABLE` / `DROP VIEW` - Do not delete
- `CREATE INDEX` on legacy tables - May affect legacy app performance
- `ADD CONSTRAINT` - Do not add foreign keys or other constraints

### What To Do Instead

If you need additional data or relationships:

1. **Create a new table** - Store additional data in a new table owned by this app
   ```sql
   -- Example: Need to add notes to bookings
   -- DON'T: ALTER TABLE View_Booking_Ds ADD Note_Text NVARCHAR(MAX)
   -- DO: CREATE TABLE ht_booking_notes (book_no, note_text, ...)
   ```

2. **Create a new view** - If you need a different data shape, create your own view
   ```sql
   -- Example: Need a combined view
   -- DON'T: ALTER VIEW View_Booking_Ds AS ...
   -- DO: CREATE VIEW ht_booking_summary AS SELECT ... FROM ht_bookings JOIN ...
   ```

3. **Use application-level joins** - Join data in your API code instead of modifying the database

## Companion utility bins

A small set of one-shot bins under `hotel-backend/src/bin/` exist for
operations that the automated `scripts/migrate.sh` pipeline cannot
cover — typically tasks that need both legacy MSSQL READ access AND
canonical PG WRITE access in the same process. None of these bins run
automatically; an operator invokes them via `docker compose --profile
backfill run --rm <bin>` (or `cargo run --bin <name>` locally) when the
companion runbook says to.

| Bin | Track | Purpose | `--dry-run`? | Runbook |
|-----|-------|---------|--------------|---------|
| `backfill_rooms` | Phase 5 (Ville bootstrap) | Mirror legacy `HT_Rooms` + `HT_SET_RoomType` into `ht_rooms_new` + `ht_room_types`. Run once per site at first cutover, then again only if a room is added/edited on the legacy side and the CT room mapper hasn't caught up yet. Idempotent. | No | (none — bootstrap-time only) |
| `backfill_checkin_rooms` | Track B5 (`audit-2026-05-13.md` T1 CRIT-1 follow-on) | Materialise `ht_checkin_rooms` rows for every still-active legacy folio that hasn't been re-synced through the post-B2 mapper. Closes the cardinality gap for folios untouched since B2 deploy. Reuses `sync::mappers::checkin::apply_checkin_aggregate` so behaviour is identical to the CT watcher's per-row path. Idempotent (re-runs short-circuit via `existing_matches` + `rooms_match`). | **Yes** (`-- --dry-run` rolls back the tx and reports would-Apply count without writing) | `docs/coexistence/RUNBOOK-b5-backfill.md` |
| `backfill_customer_legacy_ids` | Migration 055 follow-on (coexistence audit 2026-06-11 P1 #6) | Stamp `ht_customers.legacy_id` (the legacy `HT_Customers.id` SERIAL — the only key a CT D-row carries) onto the ~21-22k canonical rows mirrored before migration 055, so iHOTEL hard-deletes of never-re-touched customers resolve instead of WARN-skipping. Reads `(id, Cust_no)` from legacy MSSQL (read-only), `UPDATE … SET legacy_id=$1 WHERE legacy_cust_no=$2 AND legacy_id IS NULL`. Idempotent; never overwrites a CT-mapper-stamped or mismatching value (mismatches are counted + warned for operator review). Run once per site post-055-deploy. | **Yes** (`-- --dry-run` classifies + reports without writing; also `--chunk=N`, default 500) | (none — single-pass, summary printed to stdout) |

Conventions every bin in this list follows:

- Reads `SITE_ID` from env (`hfhotel` / `hfville`) and picks the
  matching MSSQL connection via `DbConfig::from_env()`. Same routing
  as `bin/sync.rs` and `bin/writeback.rs`.
- Connects to PG via `DATABASE_URL` (or `NEW_DATABASE_URL`).
- Prints a stdout summary report at the end so the operator can paste
  it into the runbook's verification step.
- Exits cleanly (status 0) when done. NOT a long-running watcher.
- Bounded concurrency: a tokio semaphore caps in-flight per-row work
  so a large input set never overwhelms either DB.
