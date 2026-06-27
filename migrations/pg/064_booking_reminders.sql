-- Migration: 064_booking_reminders
-- Version: vNext
-- Date: 2026-06-27
-- Task #53 (availability calendar grid + booking reminders).
--
-- Adds the PG-canonical reminder state to ht_bookings that backs the in-shell
-- notification bell (GET /api/calendar/reminders) and the balance-due filter on
-- the bookings list. The availability calendar grid itself is a pure READ feature
-- over the existing GET /api/calendar payload and needs no schema change.
--
-- ## Columns (all PG-CANONICAL ONLY — no legacy writeback)
--
-- - book_notify_day INTEGER — reminder lead time in days before check-in at
--   which the "arriving soon" reminder starts surfacing for THIS booking.
--   NULL ⇒ the endpoint's default window (DEFAULT_NOTIFY_LEAD_DAYS, 3 days). A
--   per-booking override lets a deposit-pending VIP be flagged earlier.
-- - book_notify_note TEXT — free-form receptionist reminder copy (e.g.
--   "โทรยืนยันมัดจำ"), shown in the bell dropdown. Distinct from book_notes /
--   book_internal_notes / book_special_requests which are guest/ops booking
--   notes, not reminder copy.
-- - book_notify_dismissed_at TIMESTAMPTZ — when a receptionist dismisses /
--   suppresses the reminder (NULL ⇒ active). The reminders endpoint excludes
--   dismissed rows; POST /api/calendar/reminders/{id}/dismiss stamps NOW().
--
-- iHOTEL has NO equivalent stored reminder/dismiss state (the .NET app surfaces
-- arrivals via an ad-hoc dashboard query, not a persisted flag), so there is
-- nothing to mirror — these columns are canonical-only by design.
-- TODO(coexistence): if a legacy reminder/notify column is ever identified in
-- the iHOTEL schema, wire a writeback recipe; until then dismiss state lives
-- only here (matches the round-bill / sticky-note "PG-first" precedent).
--
-- Per-site: these columns exist in BOTH the hotelnew (HF Hotel) and hotelville
-- (HF Ville) logical DBs — connection-level site scoping, no site column.

-- UP MIGRATION
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS book_notify_day INTEGER;
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS book_notify_note TEXT;
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS book_notify_dismissed_at TIMESTAMPTZ;

COMMENT ON COLUMN ht_bookings.book_notify_day IS
  'Reminder lead time in days before check-in (NULL => endpoint default, 3). Task #53.';
COMMENT ON COLUMN ht_bookings.book_notify_note IS
  'Free-form reminder note shown in the notification bell. PG-canonical only. Task #53.';
COMMENT ON COLUMN ht_bookings.book_notify_dismissed_at IS
  'When the upcoming/balance-due reminder was dismissed (NULL => active). PG-canonical only. Task #53.';

-- Active-reminder lookup: not-yet-dismissed bookings ordered by check-in.
-- Partial so it stays tiny (the reminders endpoint only reads non-dismissed rows).
CREATE INDEX IF NOT EXISTS ix_ht_bookings_active_reminders
    ON ht_bookings (book_checkin)
    WHERE book_notify_dismissed_at IS NULL;

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS ix_ht_bookings_active_reminders;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_notify_dismissed_at;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_notify_note;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_notify_day;
-- DELETE FROM schema_migrations WHERE version = '064';
