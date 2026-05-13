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
| 040 | `040_create_ht_shifts.sql` | Track F2 / T1 HIGH-5 (`audit-2026-05-13.md`) — new canonical `ht_shifts` table so the receptionist app can gate `record_payment` behind an open cashier round (mirrors iHOTEL's `HT_Round_Bill` discipline). One-open-shift-per-site enforced via partial UNIQUE index `ht_shifts_one_open_per_site WHERE shift_closed_at IS NULL`. Service layer (`service/shifts.rs`) wraps the open/close/lookup API; the gate sits in `service/payment.rs::record_payment`. Routes mounted at `/api/new/shifts/{open,close,current}` + list. Legacy `HT_Round_Bill` mirror writeback + CT-side sync deferred to Track G | v2.63.16 |
| 041 | `041_create_ht_products.sql` | Track F3 (`audit-2026-05-13.md` T1 CRIT-3) — new `ht_products` canonical table mirroring legacy `HT_Products` (keyed on `prod_legacy_no = Pro_no`). Periodic-poll sync mapper at `hotel-backend/src/sync/mappers/products.rs` (CT enablement on `HT_Products` deferred to a sibling `migrations/legacy-mssql/` migration). New `WritebackIntent::AdjustProductStock` recipe issues additive `UPDATE HT_Products SET Pro_Amt = Pro_Amt + delta WHERE Pro_no=…` — closes the stock invariant from our app's writes (legacy continues to maintain Pro_Amt for its own sales). `ht_inventory_items.inv_product_id` FK links housekeeping/POS items to canonical product master | vNext |
| 042 | `042_create_ht_rate_tiers.sql` | Track F4 / T1 CRIT-4 (`audit-2026-05-13.md`) — introduce canonical `ht_rate_tiers` table keyed on `(rate_tier_room_type, rate_tier_cust_type)` mirroring legacy `HT_Rooms_Price`'s composite axis (per-customer-type pricing). Replaces the structurally wrong `(weekday/weekend/special)` axis in `ht_rates`. `ht_rates` is left in place (deprecated) so the existing /rates CRUD form keeps working; canonical reads now go through `ht_rate_tiers` via `sync::mappers::rate_tiers` (periodic-poll, 15-min reconcile cadence). | v2.63.16 |
| 043 | `043_create_ht_checkin_rooms.sql` | Track B1 / T1 CRIT-1 + T2 CRIT-1 (`audit-2026-05-13.md`) — new canonical `ht_checkin_rooms` junction table mirroring legacy `HT_CheckIn_Ds` cardinality (one row per room per check-in folio). `UNIQUE (cr_cin_id, cr_room_id)`. All operational columns from the legacy `Ds` row: `cr_room_in/out`, `cr_room_status`, `cr_rate_per_night`, `cr_nights`, `cr_room_total`, deposit fields, coupon count, note, `cr_legacy_ds_id`. Foundation only — no app behavior change. Follow-on sub-waves B2 (mapper) / B3 (dashboard) / B4 (writeback) / B5 (backfill) layer behavior on top; `ht_checkins.cin_room_id` deprecated but retained until B5 completes. | vNext |
| 044 | `044_ht_payments_refund_columns.sql` | Track G2 / T4 CRIT-1 (`audit-2026-05-13.md`) — adds `refund_of_payment_id` (self-referential FK ON DELETE SET NULL) + `refund_reason` (`VARCHAR(500)`) to `ht_payments`. Partial index on `refund_of_payment_id WHERE NOT NULL`. Enables the new `service::payment::refund_payment` + `WritebackIntent::RefundPayment` flow which inserts a negative `ht_payments` row linked back to the original. Same legacy mapping as `record_payment` (no new CARDINALITY_MAP row needed). | vNext |
| 048 | `048_round_bill_shift_id.sql` | Track G9 / T4 HIGH-8 (`audit-2026-05-13.md`) — adds `cin_round_bill_shift_id BIGINT` to `ht_checkins` with FK to `ht_shifts(shift_id)` ON DELETE SET NULL + partial index `ix_ht_checkins_round_bill_shift_id` on non-NULL rows. Stamped by `service::checkin::check_out` with the resolved open-shift id so the daily cashier report can attribute revenue per shift. Extends Track F2's payment-gate (`service::payment::record_payment`) to the round-bill (fold) path — `check_out` now refuses unless `ShiftService::current_open_shift()` returns `Some`. Legacy `HT_Round_Bill` / `HT_Receipt_H` have no linking column, so the shift attribution lives entirely in canonical PG (no writeback recipe change). Slots 045 / 046 / 047 reserved for parallel-track migrations | vNext |

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
| `ht_guest_registry` | Guest registry for TM30 compliance | v2.0.0 |
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
| `ht_reconcile_log` | Phase 5.5 drift tripwire — rows where MSSQL hash != canonical PG hash (logged by the demoted `scheduler::sync::run_sync` 15-min diff-only safety net) | v2.45.0 |
| `ht_users` | Local username + Argon2id password hash + role (`admin` / `receptionist`). Backs Phase 4 cookie-session authentication (PR1 introduces the schema; PR2 adds HTTP login routes) | v2.60.0 |
| `ht_sessions` | Active server-side sessions keyed by HttpOnly cookie token. `ON DELETE CASCADE` from `ht_users`; `expires_at` index drives periodic cleanup. (Phase 4 PR1) | v2.60.0 |
| `scheduler_notification_state` | Persisted Slack watermark per (site, notification type) consumed by `scheduler::jobs::{poll_checkins, poll_checkouts, poll_new_bookings}`. Restored across redeploy so the polling jobs don't re-page historical events on container restart | v2.63.14 |
| `ht_shifts` | Cashier-shift canonical (Track F2 / T1 HIGH-5) — gates `service::payment::record_payment` so no payment is accepted unless an open shift exists for the binary's `SITE_ID`. One-open-shift-per-site enforced by the partial UNIQUE index `ht_shifts_one_open_per_site`. Legacy `HT_Round_Bill` mirror writeback + CT sync deferred to Track G | v2.63.16 |

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
