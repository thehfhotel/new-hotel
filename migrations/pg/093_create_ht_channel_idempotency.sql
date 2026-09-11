-- Migration: 093_create_ht_channel_idempotency
-- Version: vNext
-- Date: 2026-09-11
-- Description: Caller-side request idempotency for the loyalty-app booking
--              channel (`docs/loyalty-channel.md`). Stores one row per
--              (caller identity, `Idempotency-Key`) carrying the response that
--              request produced, so a client retry REPLAYS it instead of
--              creating a second hold.
--
-- ## Why this exists
--
-- `POST /api/channel/bookings` (→ `ChannelService::create_hold` →
-- `BookingService::create`) mints a `book_id` SERIAL, enqueues a byte-parity
-- legacy write-back and publishes a domain event, all in one PG transaction.
-- The write-back's own idempotency key is derived server-side from that fresh
-- `book_id`, so it only protects the PG→MSSQL leg: a crashed/retried WORKER
-- never double-writes. It does NOT protect a duplicate CREATE REQUEST.
--
-- A loyalty-app client whose HTTP request hangs and is retried therefore gets
-- TWO holds → two `ht_bookings` rows → two real iHOTEL `จอง` bookings against
-- (usually) two different rooms, one of which nobody will ever release before
-- its 2h deadline. The loyalty app works around this today with a 20 s Redis
-- lock, which is a timing heuristic, not a guarantee.
--
-- Migration 076 gave the OTA create path a natural key to dedupe on
-- (`book_channel`, `book_ext_ref`). That does not fit here: a loyalty hold has
-- no channel-native booking id at request time — the id the app knows is the
-- one WE mint. What the app CAN supply is a client-generated idempotency key,
-- which is what this table is keyed on.
--
-- ## Shape
--
-- Keyed on `(idem_caller, idem_key)` with a UNIQUE constraint, and that
-- constraint is load-bearing twice over:
--
--   1. It is the REPLAY lookup — a second request with the same key finds the
--      first request's stored status + body and returns them verbatim.
--   2. It is the CONCURRENCY SERIALIZER. The reserving INSERT runs inside a
--      transaction that stays open for the whole create; a concurrent second
--      INSERT of the same key BLOCKS on the uncommitted unique-index entry
--      (PostgreSQL speculative insertion) until the winner commits, then finds
--      zero rows inserted and replays the winner's stored response. That is
--      why only ONE hold can be created for one key even when two identical
--      requests arrive at the same instant — no advisory lock, no Redis.
--
-- `idem_caller` is the SHA-256 hex digest of the presented bearer
-- (`LOYALTY_CHANNEL_TOKEN`), never the token itself: it scopes the key space to
-- the client that owns it, so two callers cannot collide on a key like `"1"`,
-- and rotating the token deliberately starts a fresh key space (a rotated token
-- is a different client). A digest is stored rather than a label because the
-- column must be safe to read in a psql session, a dump, or a support ticket.
--
-- `idem_fingerprint` is a SHA-256 over the CANONICALISED request (normalised
-- field by field, not raw bytes — so whitespace and JSON key order do not
-- produce a false mismatch). Reusing one key for a DIFFERENT request is a
-- client bug that must be loud: the API answers 422 rather than silently
-- replaying an unrelated booking's response.
--
-- ## Retention
--
-- 24 h TTL via `idem_expires_at`, swept opportunistically (a bounded DELETE of
-- expired rows before each fresh reservation) rather than by a scheduler job —
-- the volume is a handful of rows a day and a stray sweep failure must never
-- fail a booking. An expired row is also treated as absent on lookup, so a
-- key replayed after 24 h behaves exactly like a first-time request.
--
-- ## PG-canonical only
--
-- Nothing here is mirrored to legacy: iHOTEL has no notion of a request key,
-- there is NO sync mapper, NO writeback recipe, NO `WritebackIntent` and no
-- dark flag waiting to enable one — coexistence invariant #6 is upheld by
-- there being nothing legacy-coupled in this table at all. It does not change
-- what `booking_create` writes, and a request WITHOUT a key behaves exactly as
-- it does today (no row is written).
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` get the
-- table; each site's pool holds its own keys) — the same model as every other
-- channel table. A key is therefore scoped per PROPERTY as well as per caller,
-- which is correct: the two properties are two databases and a retry always
-- targets the same property it originally did.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_channel_idempotency (
    idem_id           BIGSERIAL   PRIMARY KEY,
    -- SHA-256 hex of the presented channel bearer. Scopes the key space to one
    -- caller; never the token itself (see the header block).
    idem_caller       TEXT        NOT NULL,
    -- The client-generated `Idempotency-Key` header value, verbatim. Length and
    -- charset are validated in the app (1..=255 printable ASCII) — a CHECK here
    -- would be a second place to change when that policy moves, and the app is
    -- the only writer.
    idem_key          TEXT        NOT NULL,
    -- Which operation the key was spent on, e.g. `channel_create_booking`.
    -- Recorded for debugging and for a future second idempotent endpoint; it is
    -- deliberately NOT part of the uniqueness key, because idempotency means
    -- "this client's logical operation runs once", and reusing one key across
    -- two endpoints is the same client bug as reusing it with a different body
    -- (and is caught the same way, by the fingerprint).
    idem_endpoint     TEXT        NOT NULL,
    -- SHA-256 hex over the canonicalised request. A mismatch under the same key
    -- is answered 422.
    idem_fingerprint  TEXT        NOT NULL,
    -- The stored response. NULL on a row that is still in flight; a row is only
    -- ever COMMITTED with these set, because the reserving transaction commits
    -- at the same moment it records them. A committed row is therefore always
    -- complete, and an in-flight row is invisible to everyone but its owner.
    idem_status       SMALLINT    NULL,
    -- The response body as it was SENT, stored as text rather than jsonb so the
    -- replay is byte-identical: jsonb normalises key order and would hand the
    -- client a different document than the original request received.
    idem_body         TEXT        NULL,
    -- The booking the request created, for support/debugging ("which hold did
    -- key X produce?") and so an operator can answer that without parsing the
    -- body. No FK: the row outlives nothing and must not block a booking
    -- cleanup, and this table is swept on its own TTL.
    idem_book_id      INTEGER     NULL,
    idem_created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    idem_completed_at TIMESTAMPTZ NULL,
    -- TTL horizon. Defaulted here so the app never has to compute it, and read
    -- on every lookup — an expired row is treated as absent even if the sweep
    -- has not reached it yet.
    idem_expires_at   TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '24 hours'),
    CONSTRAINT ux_ht_channel_idempotency_caller_key UNIQUE (idem_caller, idem_key)
);

-- The TTL sweep's index. Plain (not partial): every row is eligible eventually,
-- so there is no stable predicate to narrow on, and the table is small.
CREATE INDEX IF NOT EXISTS ix_ht_channel_idempotency_expires
    ON ht_channel_idempotency (idem_expires_at);

COMMENT ON TABLE ht_channel_idempotency IS
    'Caller-side request idempotency for the loyalty-app booking channel, migration 093. '
    'One row per (idem_caller, idem_key): the SHA-256 of the presented channel bearer '
    'plus the client''s Idempotency-Key header. Stores the response status + body that '
    'key produced so a retry REPLAYS it instead of creating a second hold, and the '
    'UNIQUE constraint doubles as the concurrency serializer — the reserving INSERT '
    'holds the uncommitted index entry for the whole create, so a simultaneous duplicate '
    'blocks and then replays instead of racing. idem_fingerprint is a SHA-256 over the '
    'CANONICALISED request; the same key with a different request is answered 422, never '
    'silently replayed. 24 h TTL (idem_expires_at), swept opportunistically before each '
    'fresh reservation; an expired row reads as absent. Requests without a key write NO '
    'row and behave exactly as before. PG-CANONICAL ONLY: no legacy counterpart, no sync '
    'mapper, no writeback, no domain event. Per-site (connection-level scoping).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- -- Dropping this table loses in-flight idempotency protection: a retry that
-- -- arrives after the drop creates a second hold. Harmless once the channel is
-- -- quiet; never run it while the loyalty app is live.
-- DROP INDEX IF EXISTS ix_ht_channel_idempotency_expires;
-- DROP TABLE IF EXISTS ht_channel_idempotency;
-- DELETE FROM schema_migrations WHERE version = '093';
