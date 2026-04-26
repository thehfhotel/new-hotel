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
sqlcmd -S 192.168.100.222 -d HotelDB -U username -P password -i migrations/001_create_booking_notes_table.sql
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
| `ht_checkins` | Check-in records (replaces View_CheckIn_Ds) | v2.0.0 |
| `ht_guest_registry` | Guest registry for TM30 compliance | v2.0.0 |
| `ht_rates` | Room rates by type and date range | v2.0.0 |
| `ht_settings` | Application settings key-value store | v2.0.0 |
| `ht_inventory_categories` | Inventory categories (Minibar, Amenities, Linens, Equipment) | v2.1.0 |
| `ht_inventory_items` | Inventory items with stock tracking | v2.1.0 |
| `ht_room_inventory` | Items assigned to each room | v2.1.0 |
| `ht_inventory_transactions` | Stock movement transactions (IN, OUT, ADJUST, MOVE) | v2.1.0 |
| `ht_booking_notes` | Booking notes (moved from legacy DB in v2.2.0) | v2.2.0 |
| `ht_payments` | Payment records for check-ins (multiple payments per stay) | v2.10.0 |
| `ht_maintenance_categories` | Maintenance categories (Electrical, Plumbing, AC, Furniture, General) | v2.11.0 |
| `ht_maintenance_requests` | Maintenance request records with status tracking | v2.11.0 |
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
