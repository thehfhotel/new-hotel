-- Migration: 030_add_ht_payments_legacy_columns
-- Version: 2.63.6
-- Date: 2026-05-12
--
-- Adds `legacy_pay_no` + `legacy_receipt_no` columns to `ht_payments` so the
-- writeback worker's `back_populate_legacy_ids` step can persist the legacy
-- `HT_CheckIn_Pay.Pay_no` / `HT_Receipt_H.Receipt_no` allocated by the
-- `record_payment` recipe onto the canonical payment row.
--
-- ## Why
--
-- Wave 5a writeback-audit item 3: the worker's back-population path only
-- stamped `ht_checkins.legacy_*` fields after a successful payment writeback.
-- The newly-minted `Pay_no` (`R{yyMM}-{4digit}`) and `Receipt_no`
-- (`B{yyMM}-{4digit}`) were dropped on the floor. Operators tracing a
-- canonical `ht_payments` row back to its legacy invoice had to query
-- `writeback_jobs.legacy_ids` JSONB — a slow lookup that the cache pattern
-- was specifically introduced to avoid (see migration 014).
--
-- ## Design
--
-- * `legacy_pay_no VARCHAR(20)` — matches the column width used for the
--   sibling `legacy_*` columns on `ht_checkins` / `ht_bookings`. The
--   allocator emits 11-character strings today (`R2604-0250`), so 20 is
--   ample headroom.
-- * `legacy_receipt_no VARCHAR(20)` — same shape and reasoning.
-- * Nullable with `IF NOT EXISTS` — existing rows get NULL and the resolver
--   (when added) will treat NULL as "look up via the audit log", matching
--   the established pattern for `ht_bookings.legacy_book_id` etc.

-- UP MIGRATION

ALTER TABLE ht_payments ADD COLUMN IF NOT EXISTS legacy_pay_no     VARCHAR(20);
ALTER TABLE ht_payments ADD COLUMN IF NOT EXISTS legacy_receipt_no VARCHAR(20);
-- `aggregate_id` mirrors migration 014's convention for ht_bookings /
-- ht_checkins / ht_rooms_new: the v5 UUID derived from the SERIAL pay_id
-- by `service::ids::aggregate_uuid(Payment, pay_id)`. The writeback
-- worker's `back_populate_legacy_ids` joins on this so the back-pop step
-- targets the right row without having to round-trip the SERIAL through
-- the intent payload (PG side is the source of truth).
ALTER TABLE ht_payments ADD COLUMN IF NOT EXISTS aggregate_id      UUID;

-- Lookup indexes for the inverse direction (legacy → PG row), matching the
-- partial-index convention from migration 014.
CREATE INDEX IF NOT EXISTS ix_ht_payments_legacy_pay_no
    ON ht_payments (legacy_pay_no) WHERE legacy_pay_no IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_ht_payments_legacy_receipt_no
    ON ht_payments (legacy_receipt_no) WHERE legacy_receipt_no IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_payments_aggregate_id
    ON ht_payments (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- DOWN MIGRATION (commented for reference)
-- DROP INDEX IF EXISTS ux_ht_payments_aggregate_id;
-- DROP INDEX IF EXISTS ix_ht_payments_legacy_receipt_no;
-- DROP INDEX IF EXISTS ix_ht_payments_legacy_pay_no;
-- ALTER TABLE ht_payments DROP COLUMN IF EXISTS aggregate_id;
-- ALTER TABLE ht_payments DROP COLUMN IF EXISTS legacy_receipt_no;
-- ALTER TABLE ht_payments DROP COLUMN IF EXISTS legacy_pay_no;
