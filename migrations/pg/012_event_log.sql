-- Migration: 012_event_log
-- Version: 2.8.2
-- Date: 2026-04-25
-- Description: Durable domain-event bus. Every state-mutating action emits one
--              row; subscribers (SSE broadcaster, writeback worker, audit
--              logger) maintain their own cursor and replay missed events from
--              this table. Per docs/architecture.md §3.6, §4d-bis.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS event_log (
    id                  UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type          TEXT         NOT NULL,         -- 'BookingCreated', 'CheckInCancelled', etc.
    aggregate_id        UUID,                          -- the entity this event is about (nullable for system events)
    payload             JSONB        NOT NULL,
    source_kind         TEXT         NOT NULL,         -- 'our_app' | 'legacy_app' | 'system'
    source_user_id      UUID,
    source_request_id   UUID,
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS ix_event_log_created_at
    ON event_log (created_at DESC);

CREATE INDEX IF NOT EXISTS ix_event_log_aggregate_created
    ON event_log (aggregate_id, created_at DESC);

CREATE INDEX IF NOT EXISTS ix_event_log_type_created
    ON event_log (event_type, created_at DESC);

-- Retention policy (architecture.md §4d-bis): drop events older than 30 days.
-- A nightly cron job (Phase 6) will run:
--   DELETE FROM event_log WHERE created_at < now() - INTERVAL '30 days';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here — duplicates in the same TX violate
-- schema_migrations_version_key.

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_event_log_type_created;
-- DROP INDEX IF EXISTS ix_event_log_aggregate_created;
-- DROP INDEX IF EXISTS ix_event_log_created_at;
-- DROP TABLE IF EXISTS event_log;
-- DELETE FROM schema_migrations WHERE version = '012';
