-- Migration: 053_level_drift_alert_cooldowns
-- Date: 2026-05-16
-- Description: Persist the level-triggered reconcile-drift digest
--              cooldown across backend restarts.
--
-- Pre-fix `scheduler::sync::level_alert_cooldowns()` was a
-- process-local `Mutex<HashMap>`. Every backend container restart
-- (typical: 2-5x/day during active deploys) wiped the map, and the
-- next reconcile tick saw "this table has >4h stale drift and no
-- cooldown" → fired the :warning: alert again. Operators got the
-- same alert multiple times per day for the same backlog.
--
-- Production incident 2026-05-16 — alert re-fired 5+ times within
-- 16h despite the documented 24h cooldown, because backend restarts
-- during the v2.69.x deploy series cleared the in-memory state each
-- time. The unresolved-drift backlog itself was unchanged (same
-- 7,626 stale rows).
--
-- Same shape + lifecycle as `scheduler_notification_state`
-- (migration 037). Composite PK on (site_id, table_name); UPSERT on
-- alert fire; SELECT on every reconcile tick to test eligibility.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_level_drift_alert_cooldowns (
    site_id          TEXT        NOT NULL,
    table_name       TEXT        NOT NULL,
    last_alerted_at  TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (site_id, table_name)
);

COMMENT ON TABLE ht_level_drift_alert_cooldowns IS
    'Persisted last-alerted-at per (site, table_name) for the '
    'level-triggered reconcile-drift digest. Replaces the process-local '
    'in-memory cooldown HashMap that was zeroed on every backend restart '
    '— see scheduler::sync::level_alert_cooldowns_persistent (2026-05-16).';

COMMENT ON COLUMN ht_level_drift_alert_cooldowns.last_alerted_at IS
    'When the level-triggered drift digest was most recently emitted '
    'for this (site, table). The cooldown evaluator compares now() - '
    'this against LEVEL_DRIFT_COOLDOWN_HOURS before emitting again.';

-- Re-use the touch-updated-at trigger pattern from migration 037.
CREATE OR REPLACE FUNCTION ht_level_drift_alert_cooldowns_touch_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_ht_level_drift_alert_cooldowns_touch_updated_at
    ON ht_level_drift_alert_cooldowns;

CREATE TRIGGER trg_ht_level_drift_alert_cooldowns_touch_updated_at
    BEFORE UPDATE ON ht_level_drift_alert_cooldowns
    FOR EACH ROW EXECUTE FUNCTION ht_level_drift_alert_cooldowns_touch_updated_at();

-- DOWN MIGRATION (commented — apply manually for rollback)
-- DROP TRIGGER IF EXISTS trg_ht_level_drift_alert_cooldowns_touch_updated_at
--     ON ht_level_drift_alert_cooldowns;
-- DROP FUNCTION IF EXISTS ht_level_drift_alert_cooldowns_touch_updated_at();
-- DROP TABLE IF EXISTS ht_level_drift_alert_cooldowns;
-- DELETE FROM schema_migrations WHERE version = '053';
