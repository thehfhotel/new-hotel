-- Migration: 014_legacy_id_columns
-- Version: 2.31.0
-- Date: 2026-04-25
--
-- Adds the columns the writeback worker's resolver
-- (`hotel-backend/src/bin/writeback.rs::resolve_legacy_ids`) reads and
-- back-populates after a successful recipe run.
--
-- ## What was missing
--
-- The Phase 4b writeback worker was merged with a resolver that does
--
--     SELECT legacy_book_id FROM ht_bookings WHERE aggregate_id = $1
--
-- but the canonical PG tables (`ht_bookings`, `ht_checkins`, `ht_rooms_new`)
-- never had `legacy_*` or `aggregate_id` columns. As a result every non-create
-- writeback intent (modify, cancel, extend, checkout, payment, mark_clean)
-- failed at the resolver step. Only `CreateBooking` and walk-in `CreateCheckIn`
-- worked end-to-end (their dispatcher branches don't need the resolver).
--
-- ## Design
--
-- * `aggregate_id UUID UNIQUE` — matches the UUID the service layer puts in
--   `writeback_jobs.aggregate_id` (computed via `service::ids::aggregate_uuid`,
--   v5 over the canonical SERIAL `*_id`). Lets the resolver join PG row → UUID
--   in O(1).
-- * `legacy_*` columns — populated by the worker's `mark_done` callback after
--   each successful writeback so subsequent intents (modify, cancel) on the
--   same aggregate can resolve immediately.
--
-- ## Existing rows
--
-- The columns are nullable — existing rows get NULL and are skipped by the
-- resolver (already-loud failure mode: "ModifyBooking requires resolved
-- legacy_book_id"). Production has no rows yet (the receptionist test data was
-- cleared on 2026-04-24); a separate backfill binary can populate `aggregate_id`
-- for any pre-existing rows once data accumulates.
--
-- For `ht_rooms_new` the backfill binary
-- (`hotel-backend/src/bin/backfill_rooms.rs`) is updated in the same release
-- to populate `legacy_room_no`/`legacy_room_id_int` directly from the MSSQL
-- mirror.

-- UP MIGRATION

ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS legacy_book_id  VARCHAR(20);
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS legacy_cust_no  VARCHAR(20);
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS aggregate_id    UUID;

ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS legacy_cin_no         VARCHAR(20);
ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS legacy_room_no        VARCHAR(10);
ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS legacy_cust_no        VARCHAR(20);
ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS legacy_checkin_ds_id  INTEGER;
ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS aggregate_id          UUID;

ALTER TABLE ht_rooms_new ADD COLUMN IF NOT EXISTS legacy_room_no      VARCHAR(10);
ALTER TABLE ht_rooms_new ADD COLUMN IF NOT EXISTS legacy_room_id_int  INTEGER;
ALTER TABLE ht_rooms_new ADD COLUMN IF NOT EXISTS aggregate_id        UUID;

-- UNIQUE constraints on aggregate_id (created as separate indexes so they're
-- partial / nullable-aware: NULL is permitted for pre-migration rows but no
-- two non-NULL values may match).
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_bookings_aggregate_id
    ON ht_bookings (aggregate_id) WHERE aggregate_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_checkins_aggregate_id
    ON ht_checkins (aggregate_id) WHERE aggregate_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_rooms_new_aggregate_id
    ON ht_rooms_new (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- Lookup indexes for the inverse direction (legacy → PG row), used by
-- subscribers and any future drift-reconcile job.
CREATE INDEX IF NOT EXISTS ix_ht_bookings_legacy_book_id
    ON ht_bookings (legacy_book_id) WHERE legacy_book_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_ht_checkins_legacy_cin_no
    ON ht_checkins (legacy_cin_no) WHERE legacy_cin_no IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_ht_rooms_new_legacy_room_no
    ON ht_rooms_new (legacy_room_no) WHERE legacy_room_no IS NOT NULL;

-- DOWN MIGRATION (commented for reference)
-- DROP INDEX IF EXISTS ix_ht_rooms_new_legacy_room_no;
-- DROP INDEX IF EXISTS ix_ht_checkins_legacy_cin_no;
-- DROP INDEX IF EXISTS ix_ht_bookings_legacy_book_id;
-- DROP INDEX IF EXISTS ux_ht_rooms_new_aggregate_id;
-- DROP INDEX IF EXISTS ux_ht_checkins_aggregate_id;
-- DROP INDEX IF EXISTS ux_ht_bookings_aggregate_id;
-- ALTER TABLE ht_rooms_new DROP COLUMN IF EXISTS aggregate_id;
-- ALTER TABLE ht_rooms_new DROP COLUMN IF EXISTS legacy_room_id_int;
-- ALTER TABLE ht_rooms_new DROP COLUMN IF EXISTS legacy_room_no;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS aggregate_id;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS legacy_checkin_ds_id;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS legacy_cust_no;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS legacy_room_no;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS legacy_cin_no;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS aggregate_id;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS legacy_cust_no;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS legacy_book_id;
