-- Migration: 037_scheduler_notification_state
-- Version: 2.63.14
-- Date: 2026-05-13
-- Description: Persist per-(site, notification type) Slack watermark for
--              the scheduler polling jobs (poll_checkins / poll_checkouts /
--              poll_new_bookings in `src/scheduler/jobs.rs`).
--
-- Problem: The scheduler tracked `last_<event>_timestamp` in process
-- memory only. Two failure modes followed:
--
--   1. Backend container redeploy reset the watermarks to None. The
--      "first run" branch then seeded the watermark with
--      `chrono::Utc::now().naive_utc()` — but legacy MSSQL stores
--      Cin_Room_In/Cin_Room_Out/Book_Date in Thai local time (GMT+7,
--      see CLAUDE.md "Timezone Handling"). The 7-hour offset between
--      the UTC-seeded watermark and the Thai-stored event time meant
--      every event from the previous 7 hours looked "new" on the
--      first post-deploy poll — ~45 Slack pages of replayed checkouts
--      per redeploy (production-verified 2026-05-13).
--
--   2. Even without the timezone bug, the in-memory watermark loses
--      anything that happened during a deploy/crash window of >2 min
--      between restarts — those events never page at all.
--
-- Fix: a tiny singleton-per-pair table the scheduler reads at
-- first-poll and writes after every successful event ingestion. The
-- table is intentionally narrow:
--
--   - `site_id`           — the multi-tenant key (hfhotel, hfville).
--                           Same key as `current_site_id()` / SITE_ID.
--   - `notification_type` — one of 'checkin', 'checkout', 'booking'.
--                           Free-form text so future polling jobs can
--                           reuse the same table without a migration.
--   - `last_event_at`     — the most-recent event timestamp the
--                           scheduler has already paged on. The next
--                           poll uses `WHERE event_ts > $1` against
--                           this value (stored as Thai local time to
--                           match the source columns).
--   - `updated_at`        — set by trigger on every UPDATE so
--                           operators can spot a stalled watermark
--                           ("nothing for 2h means polling is broken
--                           or the legacy app is idle").
--
-- Volume: 6 rows total (3 notification types × 2 sites). Lookup is by
-- the composite PK so no extra index is required.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS scheduler_notification_state (
    site_id           TEXT        NOT NULL,
    notification_type TEXT        NOT NULL,
    last_event_at     TIMESTAMP   NOT NULL,
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (site_id, notification_type)
);

COMMENT ON TABLE scheduler_notification_state IS
    'Persisted last-paged event timestamp per (site, notification type) '
    'for the scheduler polling jobs. Prevents the post-redeploy Slack '
    'replay storm caused by the in-memory watermark reset (see migration '
    'docstring + scheduler/jobs.rs 2026-05-13 fixes).';

COMMENT ON COLUMN scheduler_notification_state.last_event_at IS
    'Most-recent event timestamp the scheduler has already paged on. '
    'Stored as TIMESTAMP (no timezone) to match the legacy MSSQL columns '
    'which store Thai local time without a tz tag (see CLAUDE.md).';

-- Trigger to bump `updated_at` on every UPDATE. Keeps the row's
-- last-touch visible without forcing every caller to remember the SET.
CREATE OR REPLACE FUNCTION scheduler_notification_state_touch_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_scheduler_notification_state_touch_updated_at
    ON scheduler_notification_state;

CREATE TRIGGER trg_scheduler_notification_state_touch_updated_at
    BEFORE UPDATE ON scheduler_notification_state
    FOR EACH ROW
    EXECUTE FUNCTION scheduler_notification_state_touch_updated_at();

-- =============================================================================
-- DOWN MIGRATION (commented — production rollback runbook only)
-- =============================================================================
-- DROP TRIGGER IF EXISTS trg_scheduler_notification_state_touch_updated_at
--     ON scheduler_notification_state;
-- DROP FUNCTION IF EXISTS scheduler_notification_state_touch_updated_at();
-- DROP TABLE IF EXISTS scheduler_notification_state;
