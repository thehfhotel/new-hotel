-- Migration: 022_legacy_mirror_sync_status_seed
-- Version: 2.51.0
-- Date: 2026-04-29
-- Phase 5.5c — seed `legacy_sync_status` rows for the 6 legacy_mirror.*
-- tables so the CT watcher (`bin/sync.rs`) tracks them on each tick
-- and the dashboard surfaces their per-table observability.
--
-- The watcher iterates `CT_ENABLED_TABLES` in the binary AND requires a
-- corresponding `legacy_sync_status` row to update with rows_ingested /
-- rows_skipped / consecutive_failures / last_error. Without these rows
-- the table-name lookup at end-of-tick fails silently (zero rows
-- updated) and the dashboard never sees the table.
--
-- Migration 017 seeded the 10 canonical-sync tables. This adds the 6
-- legacy_mirror tables CT was enabled on in Phase 5.5b
-- (migrations/legacy-mssql/021).

-- UP MIGRATION

INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds')
ON CONFLICT (table_name) DO NOTHING;

-- DOWN MIGRATION (commented — destructive)
-- DELETE FROM legacy_sync_status WHERE table_name IN (
--     'HT_Cupon','HT_CheckIn_Product','HT_Deposit',
--     'HT_Changed_Room','HT_Bill_Debt_H','HT_Bill_Debt_Ds');
