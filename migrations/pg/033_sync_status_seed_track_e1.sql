-- Migration: 033_sync_status_seed_track_e1
-- Version: 2.63.12
-- Date: 2026-05-13
-- Track E1 (audit 2026-05-13) — seed `legacy_sync_status` for two
-- additional CT-watched tables wired up in this release:
--
-- * `HT_CheckIn_Other_People` — T2 HIGH-3. CT newly enabled by
--   `migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql`;
--   mapper added in `sync/mappers/guest_registry.rs` to UPSERT into
--   `ht_guest_registry` (TM.30 immigration registry).
--
-- * `HT_Rooms_Cancel` — T2 HIGH-5. CT was already enabled in Phase 5
--   (migration 020) but the watcher had no mapper — a dangling
--   subscription. New mirror mapper in `sync/mappers/mirror.rs`
--   populates `legacy_mirror.ht_rooms_cancel`.
--
-- The watcher only iterates tables that have a corresponding
-- `legacy_sync_status` row — adding a mapper to `CT_ENABLED_TABLES`
-- without seeding here would mean every tick's status UPDATE matches
-- zero rows (silent observability gap), so the seed migration ships
-- alongside the binary change.

-- UP MIGRATION

INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel')
ON CONFLICT (table_name) DO NOTHING;

-- DOWN MIGRATION (commented — destructive)
-- DELETE FROM legacy_sync_status WHERE table_name IN (
--     'HT_CheckIn_Other_People', 'HT_Rooms_Cancel');
