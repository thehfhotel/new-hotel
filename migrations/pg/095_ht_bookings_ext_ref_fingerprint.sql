-- Migration: 095_ht_bookings_ext_ref_fingerprint
-- Version: vNext
-- Date: 2026-09-11
-- Description: B8d follow-up (issue #305) — bind the caller-idempotency key
--              stored on a booking to the REQUEST that minted it. Adds
--              nullable `ht_bookings.book_ext_ref_fingerprint`, the SHA-256 of
--              the canonicalised create request, written in the same statement
--              as `book_ext_ref` (migration 076).
--
-- ## Why this exists
--
-- B8d moved the loyalty hold's dedupe onto migration 076's
-- `(book_channel, book_ext_ref)` UNIQUE index so it lives INSIDE the booking's
-- transaction and therefore survives a crash between the booking write and the
-- `ht_channel_idempotency` write (migration 093). That closed the double-hold,
-- but it left the booking holding only half of what migration 093 holds:
--
--   * 093 stores `idem_fingerprint` and answers **422** when a key is reused
--     for a materially DIFFERENT request, rather than silently replaying
--     someone else's stay;
--   * the booking stored the key alone, so once the 093 row was gone — after a
--     crash, or after its 24 h TTL — a reused key with a different request
--     matched on `book_ext_ref` and replayed the OLD booking as a fresh 201.
--
-- The two records must therefore agree on what "the same request" means. This
-- column carries the same SHA-256 over the same canonicalised fields
-- (`routes::channel::create_booking_fingerprint`), so the booking-side replay
-- applies the identical 422 rule with no 093 row present.
--
-- ## Retention asymmetry, made explicit
--
-- `ht_channel_idempotency` expires at 24 h; a booking does not. So a key is
-- now **one-shot for the life of the booking it created**, NOT for 24 h — a
-- retry a week later replays (or is refused) rather than minting a second
-- hold. That is the intended semantic and `docs/loyalty-channel.md` states it;
-- the 093 TTL is now just a cache in front of the durable record, not the
-- record itself. Deliberately NOT fixed by making 093 rows TTL-free: a crash
-- leaves no 093 row at all, so only the booking can answer.
--
-- ## Generality
--
-- Named for `book_ext_ref` rather than for the loyalty channel because it
-- fingerprints whatever request minted that external reference. The OTA create
-- path (which supplies a channel-native booking id and has no fingerprint
-- today) simply leaves it NULL, and a NULL on either side is treated as
-- "no opinion" by the comparison — existing OTA behaviour is unchanged.
--
-- ## NOT mirrored to legacy
--
-- PG-CANONICAL ONLY, exactly like `book_ext_ref` itself: no `HT_Book_H`
-- counterpart, no sync mapper, no writeback recipe, no `WritebackIntent`, no
-- dark flag. iHOTEL has no notion of a request key or its fingerprint. This
-- migration triggers no new legacy write of any kind.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row; annotates the existing
-- `ht_bookings` row's note instead).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_ext_ref_fingerprint TEXT;

COMMENT ON COLUMN ht_bookings.book_ext_ref_fingerprint IS
    'SHA-256 over the CANONICALISED request that minted book_ext_ref — migration '
    '095 / issue #305 B8d. Written in the same statement (and therefore the same '
    'transaction) as book_ext_ref, so a crash can never leave a key without the '
    'request it is bound to. Lets the booking-side idempotency replay apply the '
    'same "same key + different request = 422" rule ht_channel_idempotency '
    '(migration 093) applies, once that row is gone — after a crash, or after its '
    '24 h TTL. Consequence, stated deliberately: a key is one-shot for the LIFE OF '
    'THE BOOKING, not for 24 h. NULL = no fingerprint recorded (the OTA create '
    'path), which the comparison treats as "no opinion". PG-CANONICAL ONLY — never '
    'mirrored to legacy.';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- -- Dropping this re-opens the "reused key, different request, replayed as a
-- -- fresh 201" hole for any booking whose 093 row has expired. Harmless only
-- -- while the loyalty channel is dark.
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_ext_ref_fingerprint;
-- DELETE FROM schema_migrations WHERE version = '095';
