-- Migration: 096_ht_bookings_hold_auto_released_at
-- Version: vNext
-- Date: 2026-09-12
-- Description: B13 prerequisite — make a loyalty-channel hold expiry TYPED.
--              Adds nullable `ht_bookings.book_hold_auto_released_at TIMESTAMPTZ`,
--              stamped by the scheduler's expiry sweep in the SAME `UPDATE`
--              that cancels the hold, so an expired hold is machine-countable
--              instead of being inferable only from English prose.
--
-- ## Why this exists
--
-- B13 is "measure the expired-hold rate, then take an explicit HOLD_TTL
-- decision". That measurement is not currently possible. The expiry sweep
-- (`service::channel::sweep_expired_holds` →
-- `repository::channel::release_hold`) records the reason a hold died as
-- free-text `book_cancel_reason` only, and lands the booking in exactly the
-- same terminal shape as every other cancellation:
--
--     book_status      = 'cancelled'
--     book_cancelled_at = NOW()
--     book_cancel_reason = 'loyalty hold expired (auto-release)'
--
-- So "was this hold killed by the clock, or by a human / the app?" can only be
-- answered by string-matching the reason — and that string match is not merely
-- fragile, it is WRONG TODAY. The channel's own release endpoint
-- (`routes::channel::release`) writes:
--
--     book_cancel_reason = 'loyalty payment window lapsed (channel release)'
--
-- Both strings say the payment window ran out, but only ONE of them is an
-- expiry we control the TTL of; the other is the loyalty app deciding, for its
-- own reasons, to hand the room back. Counting them together would inflate the
-- very rate B13 exists to read, and would make a HOLD_TTL change look
-- effective (or ineffective) for reasons that have nothing to do with TTL.
-- A typed marker removes the guesswork: `book_hold_auto_released_at IS NOT NULL` is
-- the expiry, full stop.
--
-- ## Why a column and not a status value
--
-- The obvious alternative — a new `book_status` literal such as `'expired'` —
-- was rejected. `book_status` carries NO check constraint (there is no
-- `chk_booking_status`; the only CHECK on this table is `ck_ht_bookings_dates`),
-- so it is not an enumerated, validated vocabulary that a new member could be
-- added to safely. It is instead a string read by a long tail of consumers —
-- the sweep's own `'pending'` guard, `service::reports::loyalty_reconcile`'s
-- `CANCELLED_STATUS`, the F5 writeback-stall tripwire, the CT sync mapper and
-- the `booking_cancel` writeback recipe. Introducing a terminal status they do
-- not know would silently drop expired holds out of every "cancelled" rollup
-- and reconcile query that exists, which is a behaviour change — precisely
-- what a ship-dark preparation task must not make. A nullable timestamp is
-- purely additive: every existing predicate keeps matching exactly the rows it
-- matched before, and the new fact rides alongside.
--
-- The timestamp (rather than a boolean) is free — the sweep knows WHEN it
-- fired — and it is what makes a RATE computable: expiries can be bucketed by
-- the instant the hold died, independently of the stay dates the booking
-- covers.
--
-- ## Naming — it names the EVENT, not the verdict
--
-- Called `book_hold_auto_released_at` rather than `..._expired_at` for three
-- reasons, all of which outlived the first draft of this migration:
--
--   1. **It is what actually happened.** Exactly one thing stamps this column:
--      the scheduler sweep auto-releasing a hold. "Expired" describes a
--      conclusion the reader has to trust; "auto-released" describes the act
--      the row is a record of.
--   2. **It cannot be typo-confused.** The alternative sat ONE letter from
--      migration 086's `book_hold_expires_at` and was also `TIMESTAMPTZ`, so a
--      mistyped query would still compile and still return plausible
--      timestamps — silently measuring the deadline instead of the event, in
--      the one column whose whole purpose is measurement.
--   3. **It stays honest if the rule changes.** B13 exists to revisit
--      `HOLD_TTL`. If a later change alters what "expired" means — a grace
--      period, a second chance before release, a per-rate TTL — a column named
--      for expiry would quietly start lying, while a column named for the
--      auto-release keeps describing the event it records.
--
-- The metric built on it is still called `holdsExpired` on the rollup: that is
-- the business question ("how many holds did we lose to the clock?"), and it
-- is deliberately allowed to differ from the mechanism's name. The rollup's
-- field carries an explicit `serde(rename)` so the two cannot drift apart by
-- accident.
--
-- Still worth stating plainly, because the pair remains easy to misread:
--
--   * `book_hold_expires_at` (086) — FUTURE. The deadline stamped at creation.
--     NOT cleared on confirmation, so it is meaningful only while
--     `book_status='pending'`; reading it alone labels a PAID booking expired.
--   * `book_hold_auto_released_at` (this migration) — PAST. The instant the
--     sweep released the hold. NULL on everything else, including a confirmed
--     booking, so it is safe to read WITHOUT a status guard.
--
-- ## No index
--
-- Deliberately none, unlike migration 094. The only reader is the channel
-- rollup, which already restricts to a property and a period over a row set it
-- reaches by other predicates; counting a nullable column across rows that are
-- already being scanned adds no access path. Migration 095 set the same
-- precedent. If the rollup is ever run over unbounded history, revisit — a
-- partial `WHERE book_hold_auto_released_at IS NOT NULL` index is the answer then.
--
-- ## NOT mirrored to legacy
--
-- PG-CANONICAL ONLY. `HT_Book_H` has no counterpart column and iHOTEL has no
-- notion of a hold, a payment window or a TTL — a channel hold exists there as
-- an ordinary `จอง`, and its expiry reaches legacy as the ordinary
-- `CancelBooking` writeback it already rides. This migration therefore changes
-- NO writeback recipe, NO `WritebackIntent`, NO byte-parity literal, and adds
-- no new legacy write of any kind, so invariant #6's dark-flag requirement has
-- nothing to attach to. It likewise needs no CT sync mapper change: nothing in
-- iHOTEL can author this fact, the booking mapper writes an explicit column
-- list that does not name it, and a legacy-driven booking update therefore
-- leaves an already-stamped value intact.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row; annotates the existing
-- `ht_bookings` row's note instead).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_hold_auto_released_at TIMESTAMPTZ;

COMMENT ON COLUMN ht_bookings.book_hold_auto_released_at IS
    'The instant the scheduler sweep AUTO-RELEASED this loyalty-channel hold '
    'because its payment window lapsed (migration 096, B13). Written by '
    'repository::channel::release_hold in the SAME UPDATE that sets '
    'book_status=''cancelled'', so the marker and the cancellation can never '
    'disagree. NULL on every other booking — including one the loyalty app '
    'released itself through the channel release endpoint, and including a '
    'confirmed (paid) one — so `book_hold_auto_released_at IS NOT NULL` is the '
    'exact, guard-free predicate for "the clock killed this hold" and is what the '
    'channel rollup reports as holdsExpired. Named for the EVENT (an auto-release) '
    'rather than the verdict (an expiry) so it cannot be typo-confused with '
    'book_hold_expires_at (migration 086) — the FUTURE deadline, also TIMESTAMPTZ, '
    'which is meaningless without a book_status=''pending'' guard — and so it stays '
    'accurate if a later HOLD_TTL change alters what "expired" means. '
    'PG-CANONICAL ONLY — no HT_Book_H counterpart, no sync mapper, no writeback '
    'recipe; the release''s legacy leg is the ordinary CancelBooking writeback.';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- -- Dropping this does not change how any hold behaves — the sweep still
-- -- cancels it and still writes the free-text reason — but it re-blinds B13:
-- -- the expired-hold rate becomes unmeasurable again, and the channel rollup's
-- -- holdsExpired must be removed with it.
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_hold_auto_released_at;
-- DELETE FROM schema_migrations WHERE version = '096';
