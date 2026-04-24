-- Migration: 011_writeback_jobs
-- Version: 2.8.2
-- Date: 2026-04-25
-- Description: Outbox queue table for legacy MSSQL writeback jobs.
--              Service layer INSERTs in the same TX as canonical writes;
--              writeback worker (bin/writeback.rs, Phase 4b) LISTENs
--              'writeback_channel' and dequeues. Per docs/architecture.md §4c.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS writeback_jobs (
    id                BIGSERIAL    PRIMARY KEY,
    intent            TEXT         NOT NULL,           -- 'create_booking', 'cancel_checkin', etc.
    payload           JSONB        NOT NULL,           -- domain data needed to replay
    aggregate_id      UUID         NOT NULL,           -- the PG row this is about
    idempotency_key   UUID         NOT NULL UNIQUE,
    status            TEXT         NOT NULL DEFAULT 'pending',  -- pending|in_progress|done|failed
    attempts          INT          NOT NULL DEFAULT 0,
    last_error        TEXT,
    legacy_ids        JSONB,                           -- e.g. {"book_id": "R014812", "cust_no": "C21613"}
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    completed_at      TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_pending_created
    ON writeback_jobs (status, created_at)
    WHERE status IN ('pending', 'failed');

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_aggregate
    ON writeback_jobs (aggregate_id);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('011', '011_writeback_jobs.sql', 'migrate-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_writeback_jobs_aggregate;
-- DROP INDEX IF EXISTS ix_writeback_jobs_pending_created;
-- DROP TABLE IF EXISTS writeback_jobs;
-- DELETE FROM schema_migrations WHERE version = '011';
