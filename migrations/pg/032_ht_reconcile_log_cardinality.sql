-- Migration: 032_ht_reconcile_log_cardinality
-- Version: 2.63.11
-- Date: 2026-05-13
-- Description: Track D / T7 CRIT-1 — cardinality-aware reconcile.
--
-- The `ht_reconcile_log.pg_hash IS NULL` + `divergence_kind='cardinality'`
-- paths must NEVER be ack-silenced. Pre-Track-D the reconciler treated
-- every divergence as value-only and acked the MSSQL hash into
-- `ht_*_legacy.sync_hash`, so a multi-room folio (e.g. PG canonical
-- denormalises 1 row while MSSQL has 3 detail rows) would hash-mismatch
-- forever AND get silently silenced after the first detection. Add a
-- discriminator column + row-count columns so the worker can choose
-- whether to re-fire every tick (cardinality, canonical-missing) or
-- once (value drift, repaired by operator action).
--
-- ## Columns
--
-- * `divergence_kind TEXT` — one of:
--     - `'value'`         — same row count, content drift (acked by hash)
--     - `'cardinality'`   — row counts differ between PG and MSSQL (never silenced)
--     - `'missing_pg'`    — canonical row missing entirely (`pg_hash IS NULL`)
--     - `'missing_mssql'` — legacy row missing (`mssql_hash IS NULL`)
-- * `legacy_row_count INT` — number of MSSQL rows aggregated into the hash
--   (1 for flat tables; N for `View_CheckIn_Ds` / `View_Booking_Ds` groups)
-- * `pg_row_count INT` — number of canonical rows for the same PK. Today
--   the canonical fetch returns 0 or 1 row per PK; the column is sized
--   for future schema growth (Track B multi-room junction tables will
--   make this >1).
--
-- All three columns are nullable for backward compatibility with rows
-- inserted before this migration. New rows are populated by
-- `scheduler::sync::record_divergence`.

-- UP MIGRATION

ALTER TABLE ht_reconcile_log
    ADD COLUMN IF NOT EXISTS divergence_kind  TEXT;

ALTER TABLE ht_reconcile_log
    ADD COLUMN IF NOT EXISTS legacy_row_count INT;

ALTER TABLE ht_reconcile_log
    ADD COLUMN IF NOT EXISTS pg_row_count     INT;

-- Diagnostic index — Track D HIGH-1 level-triggered drift digest scans
-- by `(table_name, resolved_at IS NULL, detected_at)`. Already covered by
-- `idx_ht_reconcile_log_table_unresolved` from migration 019; the new
-- discriminator column is read alongside but doesn't change index shape.

-- DOWN MIGRATION (commented — apply manually for rollback)
-- ALTER TABLE ht_reconcile_log DROP COLUMN IF EXISTS pg_row_count;
-- ALTER TABLE ht_reconcile_log DROP COLUMN IF EXISTS legacy_row_count;
-- ALTER TABLE ht_reconcile_log DROP COLUMN IF EXISTS divergence_kind;
-- DELETE FROM schema_migrations WHERE version = '032';
