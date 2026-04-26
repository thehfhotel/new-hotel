-- Migration: 018_ht_customers_aggregate_keys
-- Version: 2.43.1
-- Date: 2026-04-26
-- Description: Phase 5.2 — adds the legacy / aggregate identifier columns
--              that the Customer CT mapper (`sync::mappers::customer`) uses
--              to map MSSQL `HT_Customers` rows to canonical PG rows and
--              back. Mirrors the pattern established by migration 014 for
--              `ht_bookings` / `ht_checkins` / `ht_rooms_new`.
--
-- ## Why this exists
--
-- Migration 014 added `legacy_*_id` + `aggregate_id` columns to the three
-- tables the writeback worker resolves UUID → row through. `ht_customers`
-- was excluded at the time because no writeback intent operates on a
-- customer aggregate directly (customer rows are minted as a side-effect
-- of `CreateBooking` / `CreateCheckIn`). With Phase 5.2 the *reverse*
-- direction lights up: the CT watcher mapper for `HT_Customers` needs to
--
--   1. find the canonical row for an MSSQL `Cust_no` to UPSERT against, and
--   2. publish a `DomainEvent::CustomerCreated` / `CustomerModified` whose
--      `id` is a stable, deterministic `aggregate_id` (so subscribers and
--      `event_log (aggregate_id, created_at)` queries align with the rest
--      of the system).
--
-- Both needs are met with the same column shape used by 014: a nullable
-- legacy reference column + a nullable `aggregate_id UUID`, each with a
-- partial unique index so non-NULL values are forced unique while
-- pre-migration rows can keep `NULL`.
--
-- ## Existing rows
--
-- Existing `ht_customers` rows get NULL for both new columns. The mapper's
-- first run will populate them via UPSERT (legacy_cust_no resolved from
-- MSSQL `Cust_no`, aggregate_id derived deterministically by
-- `service::ids::aggregate_uuid(AggregateKind::Customer, cust_id)`).
-- The columns are nullable so this migration is safe on a populated
-- table; the mapper is the single writer.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS legacy_cust_no VARCHAR(20);
ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS aggregate_id   UUID;

-- Partial unique indexes — match the pattern in migration 014. NULL is
-- permitted (legacy rows that the mapper hasn't touched yet), but no two
-- non-NULL values may collide.
CREATE UNIQUE INDEX IF NOT EXISTS ht_customers_legacy_cust_no_uniq
    ON ht_customers (legacy_cust_no) WHERE legacy_cust_no IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_customers_aggregate_id_uniq
    ON ht_customers (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here — duplicates in the same TX violate
-- schema_migrations_version_key.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ht_customers_aggregate_id_uniq;
-- DROP INDEX IF EXISTS ht_customers_legacy_cust_no_uniq;
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS aggregate_id;
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS legacy_cust_no;
-- DELETE FROM schema_migrations WHERE version = '018';
