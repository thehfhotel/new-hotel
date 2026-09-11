-- Migration: 056_legacy_mirror_ht_book_pro
-- Version: vNext
-- Date: 2026-06-12
--
-- Phase 5/E2 — iHOTEL coexistence audit 2026-06-11 (P2 gap closure):
-- `HT_Book_Pro` (pre-booked products attached to a booking by
-- FrmAddBook2; `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Book_Pro`" "insert in loop on FrmAddBook2 save",
-- step 3.5 of `docs/legacy-app/COMPAT_CHEATSHEET.md` §"3.4 Create Booking (with specific rooms, FrmAddBook2)" "INSERT HT_Book_Pro (B_NO=Book_ID") had no
-- CT mapper, so iHOTEL-entered booking products were invisible to the
-- new app and were silently dropped when such a booking was checked
-- in via the new app.
--
-- Three pieces, matching the established new-table rollout pattern:
--
-- 1. Mirror table `legacy_mirror.ht_book_pro` — opaque pass-through,
--    same shape as migration 020's tables (`mirrored_at` +
--    `mirror_source` bookkeeping columns; `mirror_source='ct'` from
--    the incremental mapper, `'reconcile'` from --bootstrap snapshots).
--
-- 2. `legacy_sync_status` seed (migration 033 pattern) — the watcher's
--    per-tick observability UPDATE matches zero rows without it.
--
-- 3. `legacy_ct_state_per_table` seed (migration 050 backfill pattern)
--    — seeded from the CURRENT global `legacy_ct_state` watermark, NOT
--    0. A new CT table's `CHANGE_TRACKING_MIN_VALID_VERSION` is the
--    DB version at CT-enable time, which is far ahead of 0; an
--    unseeded per-table row (default 0) would trip `check_retention`'s
--    `min_valid > last_seen` overflow page on the first per-table-mode
--    tick. Seeding from the global watermark makes the table enter the
--    stream "from now" — pre-existing rows arrive via `--bootstrap`
--    snapshot, never via CT replay. (HT_Book_Pro has 0 rows at HF
--    Hotel per schema-baseline.txt, so there is no history to seed
--    today.)
--
-- Companion changes: `migrations/legacy-mssql/023_book_pro_ct.sql`
-- (PK + CT enable — apply BEFORE the deploy that ships this
-- migration), `BookProMirrorMapper` in `sync/mappers/mirror.rs`,
-- `CT_ENABLED_TABLES` + `build_mappers` in `bin/sync.rs`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_book_pro (
    id            INTEGER          PRIMARY KEY,
    b_no          TEXT,
    b_room        TEXT,
    b_name        TEXT,
    b_unit        TEXT,
    b_num         DOUBLE PRECISION,
    b_price       DOUBLE PRECISION,
    b_price_total DOUBLE PRECISION,
    b_pro_id      INTEGER,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

-- `b_no` is the booking ref (`HT_Book_H.Book_ID`) — the lookup key for
-- "products attached to booking X" reads, same role as the cin_no
-- indexes on the other transactional mirrors.
CREATE INDEX IF NOT EXISTS ht_book_pro_b_no_idx
    ON legacy_mirror.ht_book_pro (b_no);

-- Watcher observability row (migration 033 pattern).
INSERT INTO legacy_sync_status (table_name)
VALUES ('HT_Book_Pro')
ON CONFLICT (table_name) DO NOTHING;

-- Per-table watermark seeded from the current global watermark
-- (migration 050 backfill pattern). ON CONFLICT DO NOTHING so
-- re-applying never tramples per-table progress.
INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version)
SELECT 'HT_Book_Pro',
       COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0)
ON CONFLICT (table_name) DO NOTHING;

-- DOWN MIGRATION (commented — destructive)
-- DELETE FROM legacy_ct_state_per_table WHERE table_name = 'HT_Book_Pro';
-- DELETE FROM legacy_sync_status WHERE table_name = 'HT_Book_Pro';
-- DROP INDEX IF EXISTS legacy_mirror.ht_book_pro_b_no_idx;
-- DROP TABLE IF EXISTS legacy_mirror.ht_book_pro;
-- DELETE FROM schema_migrations WHERE version = '056';
