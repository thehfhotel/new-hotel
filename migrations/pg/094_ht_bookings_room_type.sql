-- Migration: 094_ht_bookings_room_type
-- Version: vNext
-- Date: 2026-09-11
-- Description: B8c (issue #304 follow-up) — give a PARKED (roomless) booking a
--              room-type attribution. Adds nullable
--              `ht_bookings.book_room_type_id` (FK → `ht_room_types`) so the
--              loyalty-channel availability math can subtract a parked claim
--              from the TYPE it actually claims instead of only capping
--              property-wide.
--
-- ## Why this exists
--
-- B8a taught the channel that a parked booking — a live `ht_bookings` row with
-- ZERO `ht_booking_rooms` rows — consumes inventory. But canonical recorded no
-- room type for one: `ht_bookings` had no type column, and
-- `ht_booking_rooms.br_room_type_id` only exists on a row that already carries
-- `br_room_id NOT NULL`. `repository::channel` therefore had to use the honest
-- but blunt property-wide rule
--
--     surplus         = max(free rooms property-wide − parked claims, 0)
--     available(type) = min(free rooms of the type, surplus)
--
-- which never oversells but also never blocks the RIGHT type: a parked claim on
-- a Deluxe let the channel sell the property's last Deluxe as long as a Standard
-- was free somewhere. This column is the missing fact. With it:
--
--     available(type) = min( max(free(type) − parked_typed(type), 0), surplus )
--
-- and the property-wide `surplus` term is UNCHANGED, so a parked claim whose
-- type is still unknown (NULL) keeps exactly the #304 behaviour.
--
-- ## Who writes it
--
-- * The desk / OTA create + edit path (`routes::new_bookings` →
--   `service::booking`): an optional `roomTypeId` on the request. When rooms
--   ARE assigned the value must AGREE with the first assigned room's type, or
--   it is DERIVED from that room — the two facts can never disagree in a
--   committed row.
-- * The CT sync mapper (`sync::mappers::booking`): a legacy booking taken in
--   iHOTEL's "ระบุประเภทห้อง" mode (`HT_Book_H.Book_room_type = 1`,
--   cheatsheet §3.3) carries a room-TYPE code in `HT_Book_Ds.Book_Room_Type`
--   rather than a room number; that code is resolved against
--   `ht_room_types.type_code` / `type_name`. An unresolvable code leaves NULL
--   (logged once per distinct unknown value) — never an error, because a
--   data-quality value must not hold the CT watermark.
--
-- ## NOT mirrored to legacy
--
-- PG-CANONICAL ONLY. There is no new legacy write of any kind: `HT_Book_H` has
-- no column this maps onto (its own `Book_room_type` is a MODE discriminator —
-- 1 = no specific rooms, 2 = with specific rooms — not a room type), the
-- byte-parity `booking_create` / `booking_modify` recipes are untouched, there
-- is no `WritebackIntent` and no dark flag waiting to enable one. Coexistence
-- invariant #6 is upheld by there being nothing legacy-coupled here: this
-- migration reads FROM legacy via the existing mapper and writes only PG.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row; annotates the existing
-- `ht_bookings` row's note instead).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_room_type_id INTEGER;

-- FK so a parked claim can never name a type that does not exist. Added
-- separately (and idempotently) because `ADD COLUMN IF NOT EXISTS` cannot carry
-- a named constraint conditionally. `ON DELETE SET NULL`: deleting a room type
-- must not block, and a claim whose type vanished is exactly the NULL-type case
-- the property-wide cap already handles correctly.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'fk_ht_bookings_room_type'
    ) THEN
        ALTER TABLE ht_bookings
            ADD CONSTRAINT fk_ht_bookings_room_type
            FOREIGN KEY (book_room_type_id)
            REFERENCES ht_room_types(type_id)
            ON DELETE SET NULL;
    END IF;
END
$$;

COMMENT ON COLUMN ht_bookings.book_room_type_id IS
    'Room type this booking claims, for a PARKED (roomless) booking above all — '
    'migration 094 / issue #304 B8c. Written by the desk create+edit path '
    '(request field roomTypeId; when rooms are assigned it must AGREE with the '
    'first assigned room''s type or is DERIVED from it) and by the CT sync '
    'mapper from HT_Book_Ds.Book_Room_Type when HT_Book_H.Book_room_type = 1 '
    '(iHOTEL''s "no specific rooms" mode, where that column holds a room-TYPE '
    'code rather than a room number). NULL means "type not known": '
    'repository::channel then falls back to the property-wide parked-claim cap. '
    'PG-CANONICAL ONLY — HT_Book_H has no counterpart column (its own '
    'Book_room_type is a 1/2 MODE discriminator) and nothing writes this back to '
    'legacy.';

-- Parked-claim aggregation: `repository::channel::inventory_ctes` groups live
-- ROOMLESS bookings by this column over a stay-date range. Partial predicate
-- keeps the index to the rows that carry a type (every pre-094 row is NULL),
-- and it doubles as the FK's referencing-side index so deleting a room type
-- does not seq-scan ht_bookings.
CREATE INDEX IF NOT EXISTS ix_ht_bookings_room_type
    ON ht_bookings (book_room_type_id)
    WHERE book_room_type_id IS NOT NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_ht_bookings_room_type;
-- ALTER TABLE ht_bookings DROP CONSTRAINT IF EXISTS fk_ht_bookings_room_type;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_room_type_id;
-- DELETE FROM schema_migrations WHERE version = '094';
