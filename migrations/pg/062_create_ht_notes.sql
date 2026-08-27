-- Migration: 062_create_ht_notes
-- Version: vNext
-- Date: 2026-06-27
-- Task #47 (room & staff sticky notes — โน้ตห้อง / โน้ตพนักงาน).
--
-- ## Background
--
-- iHOTEL keeps two "sticky note" tables with an identical shape and a simple
-- read-flag flow — was a bare cheatsheet line-range citation (946-956), which
-- had drifted onto the `HT_Log` section:
--   `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_EMP_SMS`" "`SMS_Readed`: `'no'` (default on insert) / `'yes'` (read)"
--   `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Room_SMS`" "Same shape as HT_EMP_SMS but keyed on `SMS_Room`"
--   `docs/legacy-app/COMPAT_CHEATSHEET.md` §"3.22 Add Sticky Note to Room / Employee" "INSERT HT_Room_SMS (SMS_Room=<r>"
--   `docs/legacy-app/SCHEMA.sql` §"Table: dbo.HT_EMP_SMS" "[SMS_Readed]"
--   `docs/legacy-app/SCHEMA.sql` §"Table: dbo.HT_Room_SMS" "[SMS_Room]"
--   `docs/legacy-app/FEATURE_MAP.md` §"3.3 Room Cell Click Handlers" "Per-room sticky notes (`HT_Room_SMS`)"
--   `docs/legacy-app/FEATURE_MAP.md` §"3.3 Room Cell Click Handlers" "Inter-employee sticky notes (`HT_EMP_SMS`)"
--
--   * `HT_Room_SMS`  — a sticky note pinned to a ROOM (keyed `SMS_Room` = room_no).
--   * `HT_EMP_SMS`   — an inter-employee note (keyed `SMS_TO` = staff username).
--
-- Both have `SMS_ID int IDENTITY` PK, `SMS_Details text` (the body),
-- `SMS_By varchar(250)` (author login) and `SMS_Readed varchar(50)` holding the
-- lowercase literal `'no'` (default on insert) / `'yes'` (read). iHOTEL's
-- `frmMain1` polls `SELECT SMS_Room,SMS_ID FROM HT_Room_SMS WHERE SMS_Readed='no'`
-- (and a `GROUP BY SMS_TO` count for staff notes) to surface unread badges.
--
-- These had ZERO presence in the new app (no table, no sync, no writeback, no
-- board UI). `ht_booking_notes` is unrelated — it has no read flag and is keyed
-- to a booking, not a room/staff target.
--
-- ## Schema notes
--
-- One canonical table covers BOTH legacy tables, discriminated by
-- `note_target_kind` ('room' | 'staff'), so a single sync poll, a single API
-- read and a single board UI cover the whole feature — the same collapse-by-
-- discriminator approach `ht_cash_categories` (migration 059) uses for the
-- three legacy account trees.
--
-- - `note_target_key` is the legacy target business key VERBATIM: the room
--   number (`HT_Room_SMS.SMS_Room`) for room notes, the staff username
--   (`HT_EMP_SMS.SMS_TO`) for staff notes. The two SMS tables are keyed on raw
--   strings, so this column IS the legacy key (no FK to ht_rooms_new — a note
--   may reference a room/login that isn't modeled canonically).
-- - `note_is_read BOOLEAN` is the normalized `SMS_Readed` ('yes' ⇒ TRUE).
-- - `note_legacy_id` is the `SMS_ID` IDENTITY back-pointer. NULL for an entry
--   created in our app that has not yet been written back to legacy (PG allows
--   multiple NULLs under a UNIQUE constraint). The two SMS tables have
--   INDEPENDENT IDENTITY sequences, so the idempotent upsert key for the
--   inbound poll is the (kind, legacy id) PAIR — not the legacy id alone.
-- - `note_source` distinguishes our-app rows ('app') from rows mirrored in from
--   iHOTEL ('legacy'), matching `ht_cash_ledger.cash_source` / `ht_pos_sales.source`.
-- - `aggregate_id UUID` (nullable, partial UNIQUE) is the writeback correlation
--   key: minted v4 at INSERT by the create handler and used by the writeback
--   worker to back-populate `note_legacy_id` after the legacy INSERT returns its
--   IDENTITY (same role as `ht_coupons.aggregate_id`).
--
-- Site scoping is connection-level (these tables exist in both the `hotelnew`
-- and `hotelville` logical DBs; each site's `sync` worker populates its own DB)
-- — same model as `ht_shifts` / `ht_cash_ledger`. No site column.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_notes (
    note_id          BIGSERIAL    PRIMARY KEY,
    -- Which legacy table this note mirrors: 'room' = HT_Room_SMS,
    -- 'staff' = HT_EMP_SMS.
    note_target_kind VARCHAR(10)  NOT NULL
                     CHECK (note_target_kind IN ('room', 'staff')),
    -- Room number (SMS_Room) or staff username (SMS_TO) — verbatim legacy key.
    note_target_key  VARCHAR(50)  NOT NULL,
    note_body        TEXT         NOT NULL,         -- SMS_Details
    note_created_by  VARCHAR(250),                  -- SMS_By (author login)
    -- Normalized SMS_Readed ('yes' ⇒ TRUE, 'no'/default ⇒ FALSE).
    note_is_read     BOOLEAN      NOT NULL DEFAULT FALSE,
    -- HT_Room_SMS.SMS_ID / HT_EMP_SMS.SMS_ID (IDENTITY). NULL until written
    -- back. Idempotent upsert key for the inbound poll is (kind, legacy id).
    note_legacy_id   INTEGER,
    -- Provenance: 'legacy' = mirrored from the SMS table by the sync poll,
    -- 'app' = created via POST /api/notes.
    note_source      VARCHAR(20)  NOT NULL DEFAULT 'legacy'
                     CHECK (note_source IN ('legacy', 'app')),
    -- Writeback correlation id (v4, minted at INSERT for app rows).
    aggregate_id     UUID,
    note_created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    note_updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    note_synced_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_notes_kind_legacy_id_key UNIQUE (note_target_kind, note_legacy_id)
);

-- Hot read path: notes for one room/staff target (the board panel), and the
-- unread-first ordering.
CREATE INDEX IF NOT EXISTS ix_ht_notes_target
    ON ht_notes (note_target_kind, note_target_key);

-- Unread badge / "open notes" filter — partial so it stays tiny.
CREATE INDEX IF NOT EXISTS ix_ht_notes_unread
    ON ht_notes (note_target_kind, note_target_key)
    WHERE note_is_read = FALSE;

CREATE UNIQUE INDEX IF NOT EXISTS ix_ht_notes_aggregate_id
    ON ht_notes (aggregate_id) WHERE aggregate_id IS NOT NULL;

COMMENT ON TABLE ht_notes IS
    'Room & staff sticky notes (โน้ตห้อง / โน้ตพนักงาน) — canonical mirror of '
    'legacy HT_Room_SMS + HT_EMP_SMS, collapsed into one table via '
    'note_target_kind. Populated read-only by the sync worker''s '
    'sync_sticky_notes per-tick poll (note_source=legacy) and by '
    'POST /api/notes (note_source=app). Writeback to the legacy SMS tables is '
    'SHIPPED DARK behind NOTES_WRITEBACK_ENABLED (default off) — see '
    'writeback/recipes/sticky_note.rs. Per-site (connection-level scoping). '
    'Migration 062 (task #47).';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_notes_aggregate_id;
-- DROP INDEX IF EXISTS ix_ht_notes_unread;
-- DROP INDEX IF EXISTS ix_ht_notes_target;
-- DROP TABLE IF EXISTS ht_notes;
-- DELETE FROM schema_migrations WHERE version = '062';
