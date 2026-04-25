-- Migration: 016_writeback_notify_trigger
-- Version: 2.19.0
-- Date: 2026-04-26
--
-- Wire `NOTIFY writeback_channel` to fire on every `writeback_jobs` INSERT
-- so the worker wakes immediately instead of waiting on the 30-second poll
-- fallback.
--
-- ## Why a DB trigger (not app-level pg_notify)
--
-- The `bin/writeback.rs` doc comment §3 already anticipates this:
--   > Future migration will wire a trigger to fire this on every INSERT
--   > into `writeback_jobs` so the worker doesn't depend on
--   > application-level NOTIFY.
--
-- Trigger-based:
-- - Doesn't depend on every `OutboxRepository::enqueue` caller remembering
--   to issue the NOTIFY.
-- - Catches any future direct INSERT (operator manual recovery, future
--   batch-import binary, etc.).
-- - Fires inside the SAME transaction as the INSERT so the notification
--   only goes out if the canonical write commits — no spurious wakeups
--   on rollback.
--
-- ## Why the worker still polls every 30s
--
-- NOTIFY is best-effort: PostgreSQL drops queued notifications if the
-- listener disconnects (e.g. transient PG conn drop, worker restart). The
-- poll fallback covers that gap. With this trigger in place, the poll
-- becomes the rare slow-path; sub-second latency is the new default.

-- UP MIGRATION

CREATE OR REPLACE FUNCTION writeback_jobs_notify() RETURNS trigger AS $$
BEGIN
    -- Payload is the row id as text. The worker reads `claim_next_job`
    -- against the queue rather than parsing the payload, so the value is
    -- informational only — but useful in `pg_stat_activity` for debugging
    -- "who fired this notification?".
    PERFORM pg_notify('writeback_channel', NEW.id::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS writeback_jobs_notify_trigger ON writeback_jobs;
CREATE TRIGGER writeback_jobs_notify_trigger
    AFTER INSERT ON writeback_jobs
    FOR EACH ROW
    EXECUTE FUNCTION writeback_jobs_notify();

-- DOWN MIGRATION (commented for reference)
-- DROP TRIGGER IF EXISTS writeback_jobs_notify_trigger ON writeback_jobs;
-- DROP FUNCTION IF EXISTS writeback_jobs_notify();
