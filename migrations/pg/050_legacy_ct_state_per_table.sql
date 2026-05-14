-- Migration: 050_legacy_ct_state_per_table
-- Version: vNext
-- Date: 2026-05-14
-- Description: Resilience PR R3 — decouple per-table CT watermarks so a
--              wedge on one hot table (canonical: HT_Book_H row lock on
--              Book_ID='R015142', 74-min global stall observed
--              2026-05-14) freezes only that table instead of gating all
--              16-18 CT-enabled tables.
--
--              Adds a SIBLING table `legacy_ct_state_per_table` keyed on
--              `table_name`. The legacy single-row `legacy_ct_state`
--              stays in place (operational on the global path) and is
--              read/written by the existing code when the feature flag
--              `SYNC_PER_TABLE_WATERMARK` is OFF (default). When ON, the
--              watcher loads + advances per-row watermarks from this
--              new table instead.
--
--              Backfill: one row per CT-tracked table seeded with the
--              CURRENT `legacy_ct_state.last_seen_version`, so flipping
--              the flag at runtime resumes from the same point the
--              global path was at — no replay, no leap.
--
-- @transactional true

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS legacy_ct_state_per_table (
    table_name        TEXT         PRIMARY KEY,
    last_seen_version BIGINT       NOT NULL DEFAULT 0,
    last_polled_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    last_error        TEXT,
    last_error_at     TIMESTAMPTZ
);

COMMENT ON TABLE legacy_ct_state_per_table IS
    'Per-table Change Tracking watermark (Resilience PR R3, 2026-05-14). '
    'Sibling to legacy_ct_state; consulted only when '
    'SYNC_PER_TABLE_WATERMARK=true. Decouples each tracked legacy MSSQL '
    'table''s SYS_CHANGE_VERSION horizon so a wedge on one hot table '
    '(e.g. HT_Book_H row lock) freezes only that row instead of '
    'gating every CT-enabled table.';

COMMENT ON COLUMN legacy_ct_state_per_table.last_error IS
    'Most recent per-table sync failure message (truncated to first '
    '1024 chars in code). NULL on success — cleared by the watcher when '
    'a tick advances the watermark or completes with zero rows.';

COMMENT ON COLUMN legacy_ct_state_per_table.last_error_at IS
    'Timestamp of the most recent failure. Paired with last_error so a '
    'per-table watchdog can age stuck rows independently of the global '
    'last_polled_at field on legacy_ct_state.';

-- Backfill — seed every CT-tracked table from the current global
-- watermark. List MUST stay in lock-step with `CT_ENABLED_TABLES` in
-- `hotel-backend/src/bin/sync.rs` and the seeds in migrations 017 +
-- 022 (legacy_sync_status). Idempotent via ON CONFLICT DO NOTHING so
-- re-applying after `legacy_ct_state` has advanced doesn't trample
-- per-table progress.
INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version)
SELECT t.name, COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0)
FROM (VALUES
    -- Phase 5 canonical sync (10 tables)
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H'),
    -- Phase 5.5b legacy_mirror.* opaque pass-through (6 tables)
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds'),
    -- Track E1 sync-gap closure (2 tables)
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel')
) AS t(name)
ON CONFLICT (table_name) DO NOTHING;

-- Per-table watchdog index (Track D / T7 follow-up). The R3 watcher
-- queries `WHERE last_polled_at < now() - interval` to find tables
-- that haven't ticked recently. Tiny table (~18 rows), but the index
-- keeps the watchdog plan stable as future rebuilds add columns.
CREATE INDEX IF NOT EXISTS ix_legacy_ct_state_per_table_polled_at
    ON legacy_ct_state_per_table (last_polled_at);

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_legacy_ct_state_per_table_polled_at;
-- DROP TABLE IF EXISTS legacy_ct_state_per_table;
-- DELETE FROM schema_migrations WHERE version = '050';
--
-- Safe to drop while SYNC_PER_TABLE_WATERMARK is OFF (default) — the
-- global `legacy_ct_state` row is the source of truth in that mode.
-- DO NOT drop while the flag is ON anywhere in the fleet; the watcher
-- will lose every per-table watermark and re-sync from the global
-- value, which may be hours behind for tables that had advanced past
-- the global horizon during the per-table run.
