-- Migration: 054_comment_ht_reconcile_log_semantic
-- Version: documentation-only
-- Date: 2026-05-20

-- Clarify the semantic of ht_reconcile_log for future operators. The
-- table name reads as "durable divergence ledger" but the actual
-- semantic is "sync-lag observation queue": rows are snapshots of
-- moments when the diff-only sweep noticed two sides hadn't converged
-- YET. The auto-resolve sweep at every reconcile tick re-hashes both
-- sides and closes converged rows. Only rows that resist multiple
-- sweep cycles are durable divergence (and those are what the >4h
-- alert is calibrated to surface).
--
-- The rename ht_reconcile_log -> ht_sync_lag_log would have been more
-- honest but the blast radius (queries, alerts, runbooks, dashboards,
-- historical incident links) was disproportionate to the one-time
-- conceptual-clarity benefit. A schema comment + operator-facing
-- alert wording rewrite (this migration + sync.rs templates) achieves
-- ~80% of the value at ~1% of the cost.

-- UP MIGRATION
COMMENT ON TABLE ht_reconcile_log IS
  'Sync-lag observations. Not durable divergence. Rows are logged when '
  'the diff-only reconcile sweep notices the legacy and canonical hashes '
  'do not yet match; the auto-resolve sweep re-hashes both sides every '
  'tick and closes rows where they have converged. Only rows that resist '
  'multiple sweep cycles (>4h unresolved) represent actual durable '
  'divergence requiring operator action. See docs/architecture.md §3.6d.';

-- DOWN MIGRATION (commented)
-- COMMENT ON TABLE ht_reconcile_log IS NULL;
