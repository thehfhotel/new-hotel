-- Migration: 068_ht_payments_legacy_receipt_unique
-- Version: vNext
-- Date: 2026-06-30
-- Payment-dedup hardening (GitHub issue #203).
--
-- ## Background
--
-- A legacy receipt (HT_Receipt_H / iHOTEL bill) maps to exactly one canonical
-- payment per check-in. Without a uniqueness guard, a retried sync/writeback or
-- a crash-after-commit replay can land the SAME legacy receipt twice against the
-- same check-in, double-counting in round reports / payment ledgers. This adds a
-- partial UNIQUE index on (pay_cin_id, legacy_receipt_no) so a duplicate insert
-- hard-fails at the DB instead of silently double-counting.
--
-- The existing non-unique ix_ht_payments_legacy_receipt_no stays (covers lookups
-- where pay_cin_id is not in the predicate); this partial UNIQUE adds the
-- per-check-in dedup constraint. NULL legacy_receipt_no rows are exempt (partial
-- WHERE) — app-native payments carry no legacy receipt.
--
-- ## Pre-flight (MANDATORY before deploy)
--
-- The index creation will FAIL if any existing (pay_cin_id, legacy_receipt_no)
-- non-NULL duplicates already exist. Pre-flight BOTH hotelnew + hotelville:
--   SELECT pay_cin_id, legacy_receipt_no, COUNT(*)
--     FROM ht_payments
--    WHERE legacy_receipt_no IS NOT NULL
--    GROUP BY pay_cin_id, legacy_receipt_no
--   HAVING COUNT(*) > 1;
-- Resolve any duplicates before applying.

-- UP MIGRATION

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_payments_cin_legacy_receipt_no
    ON ht_payments (pay_cin_id, legacy_receipt_no) WHERE legacy_receipt_no IS NOT NULL;

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS ux_ht_payments_cin_legacy_receipt_no;
-- DELETE FROM schema_migrations WHERE version = '068';
