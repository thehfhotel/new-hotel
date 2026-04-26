-- Migration: 019_ht_reconcile_log
-- Version: 2.45.0
-- Date: 2026-04-26
-- Description: Phase 5.5 — drift-detection tripwire for the demoted
--              `scheduler::sync::run_sync` job. With the CT watcher
--              now authoritative for canonical PG state, the legacy
--              5-min full sync is demoted to a 15-min diff-only safety
--              net: it computes per-row hashes for every MSSQL row,
--              compares against the canonical PG row, and LOGS divergent
--              rows here instead of UPSERTing them. Operators (or a
--              future alerting cron) can `SELECT * FROM ht_reconcile_log
--              WHERE resolved_at IS NULL` to surface CT-watcher gaps.
--
-- ## When does a row land here?
--
-- * `LEGACY_SYNC_RECONCILE_MODE=diff_only` (the default in Phase 5.5+)
-- * MSSQL row hash != canonical PG row hash for the same legacy PK
-- * Rows with no canonical counterpart (PG miss) — `pg_hash` is NULL
-- * Rows that exist in PG but no longer in MSSQL — `mssql_hash` is NULL
--
-- ## When does a row clear?
--
-- Manual operator action — set `resolved_at = now()` after investigation.
-- The partial index on unresolved rows keeps the steady-state read cheap.
-- Rows are intentionally NOT auto-deleted: long-running drift patterns
-- are themselves a signal worth preserving for post-mortem.
--
-- ## Schema choices
--
-- * `legacy_pk TEXT` — varies per table (`Cust_no`, `Book_ID`, `Cin_no`,
--   composite `book_no|book_room_type`, etc.). Stored as the same
--   string the reconcile job builds for hash input — no parsing needed
--   when investigating.
-- * `mssql_row_json` / `pg_row_json` — full row payload at detection
--   time, JSONB for ad-hoc queries. Bounded by row width (well under
--   8KB for every legacy entity we care about).
-- * No FK to `legacy_sync_status` — that table is per-MSSQL-table; the
--   reconcile job uses the entity-name shape (`customers`, `bookings`,
--   …) which doesn't 1:1 map to MSSQL table names.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_reconcile_log (
    id              BIGSERIAL    PRIMARY KEY,
    detected_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
    table_name      TEXT         NOT NULL,
    legacy_pk       TEXT         NOT NULL,
    pg_hash         TEXT,
    mssql_hash      TEXT,
    mssql_row_json  JSONB,
    pg_row_json     JSONB,
    resolved_at     TIMESTAMPTZ
);

-- Hot-path index: dashboards / alerting cron only ever query unresolved
-- divergence. Partial index keeps the steady-state read cheap even after
-- a long drift history accumulates.
CREATE INDEX IF NOT EXISTS idx_ht_reconcile_log_unresolved
    ON ht_reconcile_log (detected_at)
    WHERE resolved_at IS NULL;

-- Diagnostic index for per-table investigations: "what rows of customers
-- are currently divergent?" — used during cutover soak.
CREATE INDEX IF NOT EXISTS idx_ht_reconcile_log_table_unresolved
    ON ht_reconcile_log (table_name, detected_at)
    WHERE resolved_at IS NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here — duplicates in the same TX violate
-- schema_migrations_version_key.

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP INDEX IF EXISTS idx_ht_reconcile_log_table_unresolved;
-- DROP INDEX IF EXISTS idx_ht_reconcile_log_unresolved;
-- DROP TABLE IF EXISTS ht_reconcile_log;
-- DELETE FROM schema_migrations WHERE version = '019';
