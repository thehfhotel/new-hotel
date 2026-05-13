-- Migration: 045_create_ht_room_changes
-- Version: vNext
-- Date: 2026-05-13
-- Track G4 / T4 HIGH-3 (`docs/coexistence/audit-2026-05-13.md`) — introduce
-- the canonical `ht_room_changes` audit table that mirrors legacy
-- `HT_Changed_Room` cardinality (one row per mid-stay room move).
--
-- ## Background
--
-- A receptionist moves an active guest from room A to room B mid-stay
-- (room failure, noise complaint, upgrade). Legacy iHOTEL writes the
-- audit row to `HT_Changed_Room` (`docs/legacy-app/COMPAT_CHEATSHEET.md`
-- §`HT_Changed_Room`, 3.17 Change Room Mid-Stay). Until G4 our app had
-- no service path, no UI, and no writeback recipe — receptionists
-- fell back to iHOTEL for this common operation.
--
-- This migration lands the canonical PG table that:
-- 1. Records every room move our app initiates (write path —
--    `service::checkin::change_room` → outbox →
--    `writeback/recipes/room_change.rs`).
-- 2. Captures rows authored on the legacy side as well (reverse-sync
--    via `sync/mappers/mirror.rs::RoomChangeCanonicalMirrorMapper`), so
--    a move done in iHOTEL also lands canonically.
--
-- ## Schema notes
--
-- - `rc_cin_id` ON DELETE CASCADE: if a folio is purged the audit row
--   goes with it. `rc_from_room_id` / `rc_to_room_id` do not cascade —
--   rooms are durable inventory; if a room row is deleted we keep the
--   audit reference dangling (matches `ht_checkin_rooms.cr_room_id`).
-- - `rc_legacy_id` is the back-link to `HT_Changed_Room.id` IDENTITY.
--   Partial UNIQUE index excludes NULLs so canonically-originated rows
--   stay distinct from legacy-originated rows until the writeback
--   back-populates the legacy id. Mirrors the pattern used by
--   `ht_checkin_rooms.cr_legacy_ds_id` (migration 043) and
--   `ht_guest_registry.guest_legacy_id` (migration 034).
-- - `rc_room_before_price NUMERIC(12,2)` mirrors the legacy
--   `room_before_price float NOT NULL DEFAULT 0` column — captured so
--   the audit row can answer "how much did this stay charge before the
--   move?" without joining back through `ht_checkin_rooms` history.
-- - `rc_to_price VARCHAR(20)` mirrors the legacy `ToPrice varchar(20)`
--   column verbatim (legacy stores it as text; we preserve the shape).
-- - `rc_changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()` — wall-clock of
--   the move. Matches `HT_Changed_Room.change_date` (`datetime` in
--   legacy, Thai local time per CLAUDE.md timezone convention; the
--   sync mapper stores it as `TIMESTAMP` without offset to preserve
--   the legacy semantics).
-- - `cr_cin_id` index — for the stay-history panel which lists every
--   room change for a check-in folio.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_room_changes (
    rc_id                BIGSERIAL    PRIMARY KEY,
    rc_cin_id            INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    rc_from_room_id      INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    rc_to_room_id        INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    rc_reason            VARCHAR(500),
    rc_changed_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rc_changed_by        VARCHAR(64),
    rc_room_before_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    rc_to_price          VARCHAR(20),
    rc_legacy_id         INTEGER,
    rc_created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rc_updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ht_room_changes_cin_id_idx
    ON ht_room_changes (rc_cin_id);

CREATE UNIQUE INDEX IF NOT EXISTS ht_room_changes_legacy_id_uq
    ON ht_room_changes (rc_legacy_id) WHERE rc_legacy_id IS NOT NULL;

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ht_room_changes_legacy_id_uq;
-- DROP INDEX IF EXISTS ht_room_changes_cin_id_idx;
-- DROP TABLE IF EXISTS ht_room_changes;
