-- Migration: 076_ht_bookings_ota_provenance
-- Version: vNext
-- Date: 2026-07-09
-- Description: OTA Desk → PMS booking write-back, Phase 0 — a caller-side
--              idempotency natural key on `ht_bookings`. Adds `book_channel`
--              (the OTA source channel, e.g. 'bookingcom') + `book_ext_ref`
--              (the channel's own booking id, e.g. Booking.com's
--              `channel_booking_id`) and a partial UNIQUE index over the pair.
--
-- ## Why this exists
--
-- `POST /api/bookings` (→ `BookingService::create`) writes the canonical
-- `ht_bookings` row, enqueues a byte-parity legacy write-back, and publishes a
-- domain event — all in one PG transaction. The write-back's idempotency key is
-- derived server-side from the freshly-minted `book_id` SERIAL, so it only
-- protects the PG→MSSQL leg (a crashed/retried WORKER never double-writes).
-- It does NOT protect a duplicate CREATE REQUEST: two POSTs for the same OTA
-- reservation (double-click, HTTP retry after a timeout, OTA Desk
-- crash-after-commit) each mint a new `book_id` → new key → TWO `ht_bookings`
-- rows → TWO real iHOTEL bookings.
--
-- This migration gives the create path a natural key to dedupe on. When a
-- create carries both `book_channel` and `book_ext_ref`, the service looks the
-- pair up first and returns the existing booking instead of inserting; the
-- partial UNIQUE index below is the serializer-of-last-resort for a tight
-- concurrent race that slips two creates past the lookup.
--
-- ## `book_channel` already exists on fresh seeds
--
-- `init-db/init-hotelnew.sql` already declares `book_channel VARCHAR(50)` (a
-- previously-unwired column). The `ADD COLUMN IF NOT EXISTS` below is therefore
-- a no-op on a fresh-seeded database and only materialises the column on an
-- older production DB that predates it — which the UNIQUE index requires to
-- exist before it can be built. `text` and `varchar(50)` are index-compatible,
-- so the type divergence between the two provisioning paths is harmless.
--
-- ## Strictly additive — no behaviour change for existing callers
--
-- Both columns are nullable; every existing (walk-in / manual) caller passes
-- neither and is unaffected. The index predicate `WHERE book_ext_ref IS NOT
-- NULL` exempts all existing rows (they are all NULL), so the index builds even
-- if historical data had collisions (it cannot — the column is brand new). This
-- is a PG-canonical provenance/idempotency key ONLY: it is NOT mirrored to
-- legacy iHOTEL and the `booking_create` write-back recipe is unchanged, so
-- this migration triggers no new legacy write.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row; annotates the existing
-- `ht_bookings` row's note instead).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

-- `book_channel` is present on fresh seeds as VARCHAR(50); IF NOT EXISTS makes
-- this a no-op there and materialises it (as text) only on an older DB missing
-- it, so the UNIQUE index below always has a column to reference.
ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_channel text;

ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_ext_ref text;

COMMENT ON COLUMN ht_bookings.book_ext_ref IS
  'OTA channel-native booking id (e.g. Booking.com channel_booking_id). Paired '
  'with book_channel as the caller-idempotency natural key for '
  'BookingService::create so a double-POST of one OTA reservation cannot spawn '
  'two ht_bookings rows. PG-canonical only — not mirrored to legacy. '
  'Migration 076.';

-- Caller-idempotency backstop: at most one booking per (channel, ext_ref).
-- Partial predicate keeps every existing (NULL ext_ref) row exempt.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_bookings_channel_ext_ref
    ON ht_bookings (book_channel, book_ext_ref)
    WHERE book_ext_ref IS NOT NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ux_ht_bookings_channel_ext_ref;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_ext_ref;
-- -- book_channel is left in place (it predates this migration on fresh seeds).
-- DELETE FROM schema_migrations WHERE version = '076';
