-- Migration: 043_create_ht_checkin_rooms
-- Version: vNext
-- Date: 2026-05-13
-- Track B1 / T1 CRIT-1 + T2 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`) —
-- introduce the canonical `ht_checkin_rooms` junction table that mirrors
-- legacy `HT_CheckIn_Ds` cardinality (one row per room per check-in folio).
--
-- ## Background
--
-- `ht_checkins` has a single `cin_room_id NOT NULL` column, but legacy
-- iHOTEL's `HT_CheckIn_Ds` is one row per room — a single `Cin_no` header
-- (`HT_CheckIn_H`) can fan out to N detail rows when a guest books
-- multiple rooms under one folio. The audit (`audit-2026-05-13.md` Theme
-- 1) traced the multi-room blind spot through every layer:
--
-- - the writeback recipe emits one Ds row;
-- - the sync mapper picks `first_room_no`;
-- - the dashboard joins on `cin_room_id` so secondary rooms are invisible;
-- - the calendar's `seenIds` dedup drops rooms 2..N of multi-room bookings.
--
-- The fix is a new junction table keyed on `(cin_id, room_id)`. B1 lands
-- the schema only — no app behavior change yet.
--
-- ## Scope of B1
--
-- - migration 043 (this file): new `ht_checkin_rooms` table with all
--   operational columns from the legacy `Ds` row.
-- - init-db mirror + CARDINALITY_MAP entries + CHANGELOG.
--
-- ## Out of scope (follow-on sub-waves)
--
-- - B2: sync mapper rewrite (`sync::mappers::checkin`) to emit per-room
--   rows under the canonical junction.
-- - B3: dashboard queries (`routes::rooms`, `routes::stats`, calendar)
--   join through `ht_checkin_rooms` instead of `ht_checkins.cin_room_id`.
-- - B4: writeback per-room apportionment — emit N `HT_CheckIn_Ds` rows
--   matching the junction state.
-- - B5: backfill bin populates the junction for existing folios. Only
--   after B5 lands does the existing `ht_checkins.cin_room_id` column
--   become safe to drop.
--
-- ## Design notes
--
-- - `UNIQUE (cr_cin_id, cr_room_id)` — a single check-in folio cannot
--   double-book the same room. Matches legacy `HT_CheckIn_Ds` natural
--   key shape.
-- - `cr_cin_id` cascades on delete — when a folio is purged the junction
--   rows go with it. `cr_room_id` does not cascade (rooms are durable
--   inventory).
-- - All NUMERIC(12,2) money columns mirror the legacy DECIMAL precision
--   the writeback recipe writes in `HT_CheckIn_Ds.Room_Total` etc.
-- - `cr_legacy_ds_id` is the legacy `HT_CheckIn_Ds.id` IDENTITY — partial
--   index excludes NULLs (rows authored locally before B4 writeback fans
--   out per-room legacy ids). Mirrors the pattern used by Track E1's
--   `ht_guest_registry.guest_legacy_id`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_checkin_rooms (
    cr_id              BIGSERIAL    PRIMARY KEY,
    cr_cin_id          INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    cr_room_id         INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    cr_room_in         TIMESTAMPTZ,
    cr_room_out        TIMESTAMPTZ,
    cr_room_status     VARCHAR(50)  NOT NULL,
    cr_rate_per_night  NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_nights          INTEGER      NOT NULL DEFAULT 1,
    cr_room_total      NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_dep_amount      NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_dep_status      VARCHAR(50),
    cr_dep_returned_at TIMESTAMPTZ,
    cr_dep_returned_by VARCHAR(50),
    cr_cupon_count     INTEGER      NOT NULL DEFAULT 0,
    cr_note            VARCHAR(500),
    cr_legacy_ds_id    INTEGER,
    cr_created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    cr_updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_ht_checkin_rooms_folio_room UNIQUE (cr_cin_id, cr_room_id)
);

CREATE INDEX IF NOT EXISTS ht_checkin_rooms_room_status
    ON ht_checkin_rooms (cr_room_id, cr_room_status);

CREATE INDEX IF NOT EXISTS ht_checkin_rooms_legacy_ds_id
    ON ht_checkin_rooms (cr_legacy_ds_id) WHERE cr_legacy_ds_id IS NOT NULL;

-- DOWN MIGRATION (commented — destructive; only run after a verified
-- backfill rollback of B5 and an audit that no reader has cut over)
-- DROP INDEX IF EXISTS ht_checkin_rooms_legacy_ds_id;
-- DROP INDEX IF EXISTS ht_checkin_rooms_room_status;
-- DROP TABLE IF EXISTS ht_checkin_rooms;
