-- Migration: 017_legacy_sync_status
-- Version: 2.43.0
-- Date: 2026-04-26
-- Description: Phase 5.1 — observability for the Change Tracking watcher
--              (`bin/sync.rs`). Introduces a per-table progress / health
--              table consumed by the watcher each tick, and adds the
--              soft-delete column on `ht_customers` that the upcoming
--              HT_Customers `D` mapper (Phase 5.2) will populate.
--
-- Per docs/architecture.md §3.6d, §3.7. The watcher's main loop reads
-- `legacy_ct_state.last_seen_version`, queries `CHANGETABLE(CHANGES …)`
-- per CT-enabled table, applies the changes, and updates this table
-- (`rows_ingested`/`rows_skipped`/`last_error`) so operators can see
-- per-table health without parsing logs.
--
-- The 10 CT-enabled tables are pre-seeded so the watcher can rely on
-- `UPDATE legacy_sync_status WHERE table_name=$1` instead of UPSERT
-- in the steady-state hot path.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_customers
    ADD COLUMN IF NOT EXISTS cust_deleted_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS legacy_sync_status (
    table_name           TEXT        PRIMARY KEY,
    last_processed_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    rows_ingested        BIGINT      NOT NULL DEFAULT 0,
    rows_skipped         BIGINT      NOT NULL DEFAULT 0,
    last_error           TEXT,
    last_error_at        TIMESTAMPTZ,
    consecutive_failures INT         NOT NULL DEFAULT 0
);

-- Seed the 10 CT-enabled tables (per docs/architecture.md §10 #8). The
-- watcher only iterates tables that exist in this seed — adding a new
-- mapper later means inserting another row here as part of its migration.
INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H')
ON CONFLICT (table_name) DO NOTHING;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here — duplicates in the same TX violate
-- schema_migrations_version_key.

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP TABLE IF EXISTS legacy_sync_status;
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS cust_deleted_at;
-- DELETE FROM schema_migrations WHERE version = '017';
