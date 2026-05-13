-- Migration: 036_track_e2_room_columns
-- Version: 2.63.13
-- Date: 2026-05-13
-- Track E2 / T1 HIGH-3 (`docs/coexistence/audit-2026-05-13.md`) —
-- widen `ht_rooms_new` so the CT mapper can mirror the rest of legacy
-- `HT_Rooms`. Previously only ~7 columns made it through; this adds
-- the missing 8:
--
--   * `Room_Use_Count` — running nights total (`int NOT NULL DEFAULT
--     0` in legacy). Mutated on every checkout by `Module1.UPDATE_
--     ROOM_USE`; writeback Wave 6 already increments the legacy
--     counter — this column captures it canonical-side so utilization
--     reports work without round-tripping to MSSQL.
--   * `Room_X` / `Room_Y` — grid coordinates from iHOTEL's drag-drop
--     layout (`int NOT NULL DEFAULT 0` in legacy). Lost on every CT
--     sync today; mirror so a future canonical room-layout UI has the
--     source data.
--   * `Room_Group` — room grouping (floor / wing, `varchar(50)`).
--   * `Room_Power_OPEN` / `Room_Power_CLOSE` / `Room_Power_STATUS` —
--     electricity relay timestamps + state. Legacy stores all three
--     as `varchar(50)`; preserved as text so the literal the legacy
--     app wrote (e.g. status `'off'` / `'on'`) stays intact.
--   * `Room_Polity` — policy id (`int NOT NULL DEFAULT 1` in legacy).
--     Semantics opaque from the cheatsheet but captured for parity so
--     reconcile doesn't drift indefinitely on it.
--
-- **Known wrongness — NOT fixed in this migration.** The existing
-- `room_price_weekday/weekend/special` axis on `ht_rooms_new` does
-- not match the legacy `Room_PriceA/B/C` model (which indexes prices
-- by customer-type per `HT_SET_CusType_Main`). Track F (canonical
-- rate-table model) will reconcile the rate-shape mismatch — Track
-- E2 leaves the price columns untouched.

-- UP MIGRATION

ALTER TABLE ht_rooms_new
    ADD COLUMN IF NOT EXISTS room_use_count     INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS room_x             INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS room_y             INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS room_group         VARCHAR(50),
    ADD COLUMN IF NOT EXISTS room_power_open    VARCHAR(50),
    ADD COLUMN IF NOT EXISTS room_power_close   VARCHAR(50),
    ADD COLUMN IF NOT EXISTS room_power_status  VARCHAR(50) NOT NULL DEFAULT 'off',
    ADD COLUMN IF NOT EXISTS room_polity        INTEGER NOT NULL DEFAULT 1;

-- DOWN MIGRATION (commented — destructive)
-- ALTER TABLE ht_rooms_new
--     DROP COLUMN IF EXISTS room_use_count,
--     DROP COLUMN IF EXISTS room_x,
--     DROP COLUMN IF EXISTS room_y,
--     DROP COLUMN IF EXISTS room_group,
--     DROP COLUMN IF EXISTS room_power_open,
--     DROP COLUMN IF EXISTS room_power_close,
--     DROP COLUMN IF EXISTS room_power_status,
--     DROP COLUMN IF EXISTS room_polity;
