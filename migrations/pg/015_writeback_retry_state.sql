-- Migration: 015_writeback_retry_state
-- Version: 2.33.0
-- Date: 2026-04-25
--
-- Operational hardening for the writeback worker before the first live test:
--
-- 1. **Stuck `in_progress` recovery** — a worker process that crashes
--    mid-recipe (or a `mark_done`/`mark_failed` PG failure) leaves the
--    `writeback_jobs` row in `status='in_progress'` forever. The previous
--    `claim_next_job` only picked `pending` or `failed` rows, so stuck
--    jobs required manual SQL intervention. Add `claimed_at` so the worker
--    can detect a stale claim (claimed > 5 min ago) and re-claim it.
--
-- 2. **Retry backoff** — a `failed` job was immediately re-claimable on the
--    next loop tick. If MSSQL was briefly down, the worker would burn
--    through `WRITEBACK_MAX_ATTEMPTS` in seconds. Add `next_retry_at` so
--    `mark_failed` can schedule the next retry with exponential backoff
--    (30s, 2min, 10min by default) and the claim can filter accordingly.
--
-- 3. **`exhausted` terminal state** — once `attempts >= WRITEBACK_MAX_ATTEMPTS`,
--    the worker now transitions the row to `status='exhausted'` (instead of
--    leaving it as `failed` with attempts==max, which was indistinguishable
--    from "currently retrying"). This makes operator triage one query:
--    `SELECT * FROM writeback_jobs WHERE status='exhausted'`. The Slack
--    notifier fires on the transition.
--
-- 4. **Index updates** — the existing partial index on `(status,
--    created_at) WHERE status IN ('pending','failed')` no longer matches
--    the claim query exactly. Replace with one covering all retry-eligible
--    states.

-- UP MIGRATION

ALTER TABLE writeback_jobs
    ADD COLUMN IF NOT EXISTS claimed_at    TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS next_retry_at TIMESTAMPTZ;

-- Replace the old partial index — it filtered by hardcoded status set that
-- doesn't match the new claim query (which adds `in_progress` with stale
-- claimed_at).
DROP INDEX IF EXISTS ix_writeback_jobs_pending_created;

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_claim
    ON writeback_jobs (status, next_retry_at, created_at)
    WHERE status IN ('pending', 'failed', 'in_progress');

-- Quick "any exhausted jobs?" query for the operator + healthcheck endpoint.
CREATE INDEX IF NOT EXISTS ix_writeback_jobs_exhausted
    ON writeback_jobs (created_at DESC)
    WHERE status = 'exhausted';

-- DOWN MIGRATION (commented for reference)
-- DROP INDEX IF EXISTS ix_writeback_jobs_exhausted;
-- DROP INDEX IF EXISTS ix_writeback_jobs_claim;
-- CREATE INDEX IF NOT EXISTS ix_writeback_jobs_pending_created
--     ON writeback_jobs (status, created_at)
--     WHERE status IN ('pending', 'failed');
-- ALTER TABLE writeback_jobs DROP COLUMN IF EXISTS next_retry_at;
-- ALTER TABLE writeback_jobs DROP COLUMN IF EXISTS claimed_at;
