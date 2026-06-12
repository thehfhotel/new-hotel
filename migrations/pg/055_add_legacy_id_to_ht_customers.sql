-- Migration: 055_add_legacy_id_to_ht_customers
-- Version: vNext
-- Date: 2026-06-11

-- Customer hard-delete handling (iHOTEL coexistence audit 2026-06-11,
-- P1 finding #6). SQL Server Change Tracking D rows for HT_Customers
-- carry ONLY the table's PK — the legacy SERIAL `id` — because the
-- LEFT JOIN against the deleted row yields NULL for every projected
-- column (including Cust_no). The customer mapper's apply_soft_delete
-- resolved exclusively by Cust_no, so every FrmManageCustomersNew
-- delete was a silent no-op: the canonical ht_customers row stayed
-- live forever.
--
-- This migration persists the legacy `HT_Customers.id` on the canonical
-- row so D events can resolve by it. The customer mapper UPSERT now
-- stores `legacy_id` on every I/U apply (and on the eager-mirror path),
-- and apply_soft_delete resolves by legacy_id first, falling back to
-- Cust_no for rows mirrored before this column existed.
--
-- Rows mirrored before this migration carry NULL legacy_id until their
-- next CT touch backfills it — a delete of such a row still falls back
-- to the (usually NULL on D) Cust_no and is logged loudly as
-- unresolvable rather than silently skipped.
--
-- One-shot backfill for those pre-055 rows: run
-- `hotel-backend/src/bin/backfill_customer_legacy_ids.rs`
-- (`cargo run --release --bin backfill_customer_legacy_ids -- --dry-run`
-- first) once per site after this migration deploys. It reads
-- `(id, Cust_no)` from legacy HT_Customers (read-only) and stamps
-- `legacy_id` onto canonical rows where it is still NULL — idempotent,
-- chunked, never overwrites a CT-mapper-stamped value.

-- UP MIGRATION
ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS legacy_id INTEGER;

CREATE INDEX IF NOT EXISTS idx_ht_customers_legacy_id
    ON ht_customers (legacy_id) WHERE legacy_id IS NOT NULL;

COMMENT ON COLUMN ht_customers.legacy_id IS
  'Legacy HT_Customers.id (MSSQL SERIAL PK). CT D-rows carry only this '
  'key, so customer hard-deletes resolve by it. NULL for rows mirrored '
  'before migration 055 that have not been re-touched by CT since.';

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS idx_ht_customers_legacy_id;
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS legacy_id;
