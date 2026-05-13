-- Migration: 039_create_ht_room_calendar
-- Version: vNext (Track F1 — version bump deferred to merge coordination)
-- Date: 2026-05-13
-- Track F1 (audit 2026-05-13 / T1 HIGH-4) — canonical for `HT_Room_Status`.
--
-- ## Why this exists
--
-- `HT_Room_Status` is the legacy iHOTEL booking-calendar's per-(room, date)
-- ledger — one row per night per room with `room_status` ∈ {`จอง`
-- (reserved), `เข้าพัก` (occupying), `Check Out`, …}. The legacy app's
-- calendar grid reads from this table directly; our dashboard today
-- reconstructs availability from `ht_bookings` + `ht_checkins` joins,
-- which silently miss any direct iHOTEL edit to `HT_Room_Status`
-- (mark-clean, walk-in, extend-stay, mid-stay room moves, …).
--
-- This migration adds the canonical PG table. The Track F1 sync mapper
-- (`hotel-backend/src/sync/mappers/room_calendar.rs`) hydrates it from
-- the CT-enabled `HT_Room_Status` source (CT enablement landed in
-- Phase 5, migration 017). Read-path migration is deferred to a
-- follow-up track — existing routes keep working off the reconstruction
-- model until then.
--
-- ## Columns
--
--   rcal_id              BIGSERIAL  PK
--   rcal_room_id         INTEGER    FK → ht_rooms_new.room_id
--   rcal_date            DATE       Per-night ledger date
--   rcal_status          VARCHAR(50) `จอง` / `เข้าพัก` / `Check Out` / …
--   rcal_book_id         INTEGER    FK → ht_bookings.book_id (NULL for walk-ins)
--   rcal_cin_id          INTEGER    FK → ht_checkins.cin_id (NULL pre-checkin)
--   rcal_customer_label  VARCHAR(500) Legacy `room_Details` free-text
--                                    (customer name string the legacy app
--                                    renders on the grid tile).
--   rcal_legacy_id       INTEGER    Legacy `HT_Room_Status.id` for
--                                    drift-detection + idempotency.
--   rcal_created_at      TIMESTAMPTZ DEFAULT NOW()
--   rcal_updated_at      TIMESTAMPTZ DEFAULT NOW()
--
--   UNIQUE (rcal_room_id, rcal_date) — one ledger row per room per
--     night. Mirrors the legacy table's logical PK (legacy `id` is an
--     allocator surrogate; `(room_no, room_date)` is the business key).
--
-- ## Indices
--
-- * `ux_ht_room_calendar_room_date` — covers the UNIQUE; named index for
--   explicit observability.
-- * `ix_ht_room_calendar_date_status` — supports the future calendar
--   grid query `WHERE rcal_date BETWEEN $1 AND $2 AND rcal_status IN (…)`.
-- * `ix_ht_room_calendar_legacy_id` — partial unique on the legacy id
--   for fast UPSERT / reconcile lookups.
--
-- ## DOWN
--
-- Commented to preserve canonical history. `DROP TABLE` would cascade
-- through the FKs — never run on production.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_room_calendar (
    rcal_id              BIGSERIAL    PRIMARY KEY,
    rcal_room_id         INTEGER      NOT NULL,
    rcal_date            DATE         NOT NULL,
    rcal_status          VARCHAR(50)  NOT NULL,
    rcal_book_id         INTEGER,
    rcal_cin_id          INTEGER,
    rcal_customer_label  VARCHAR(500),
    rcal_legacy_id       INTEGER,
    rcal_created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rcal_updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_ht_room_calendar_room    FOREIGN KEY (rcal_room_id)
        REFERENCES ht_rooms_new(room_id),
    CONSTRAINT fk_ht_room_calendar_booking FOREIGN KEY (rcal_book_id)
        REFERENCES ht_bookings(book_id),
    CONSTRAINT fk_ht_room_calendar_checkin FOREIGN KEY (rcal_cin_id)
        REFERENCES ht_checkins(cin_id),
    CONSTRAINT uq_ht_room_calendar_room_date UNIQUE (rcal_room_id, rcal_date)
);

CREATE INDEX IF NOT EXISTS ix_ht_room_calendar_date_status
    ON ht_room_calendar (rcal_date, rcal_status);

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_room_calendar_legacy_id
    ON ht_room_calendar (rcal_legacy_id) WHERE rcal_legacy_id IS NOT NULL;

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ux_ht_room_calendar_legacy_id;
-- DROP INDEX IF EXISTS ix_ht_room_calendar_date_status;
-- DROP TABLE IF EXISTS ht_room_calendar;
-- DELETE FROM schema_migrations WHERE version = '039';
