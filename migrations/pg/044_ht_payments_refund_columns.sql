-- Migration: 044_ht_payments_refund_columns
-- Version: vNext
-- Date: 2026-05-13
-- Track G2 / T4 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`) —
-- introduce refund tracking columns on `ht_payments` so a refund row can
-- be unambiguously traced back to the payment it offsets.
--
-- ## Background
--
-- Today `service::payment::record_payment` rejects `amount_satang <= 0`,
-- so a guest complaint forces the receptionist back into iHOTEL just to
-- record a refund (negative `Cin_Pay_Cash` / `Cin_Pay_Credit` per
-- `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay`" "can be negative (refunds use negation)").
-- Track G2 adds a separate
-- `refund_payment` service method that inserts a negative `ht_payments`
-- row plus a `WritebackIntent::RefundPayment` outbox enqueue. To keep
-- refunds queryable / auditable on the canonical side, the new row
-- needs a back-pointer to the original payment.
--
-- ## Columns
--
-- - `refund_of_payment_id INTEGER` — FK to `ht_payments(pay_id)` of the
--   original payment this row refunds. NULL for non-refund rows
--   (preserves the existing INSERT shape). Self-referential FK; ON
--   DELETE SET NULL so audit-purging an original payment leaves the
--   refund row standing (still useful for the legacy invoice audit).
-- - `refund_reason VARCHAR(500)` — operator-supplied free text captured
--   from the refund UI. NULL when the row isn't a refund. NOT written
--   to legacy MSSQL (no audit column on `HT_CheckIn_Pay`); kept here so
--   the canonical row reflects the full operator context.
--
-- ## Indexes
--
-- - Partial index on `refund_of_payment_id` for the inverse-lookup
--   ("show me every refund against payment X"). Partial WHERE NOT NULL
--   matches the existing convention from migration 030.
--
-- ## Out of scope (deferred)
--
-- - Multi-room refund apportionment. Track B's payment_lines schema
--   (Track B1's CRIT-2 follow-on) will give us per-room refund
--   resolution; until then a refund applies to the whole check-in
--   folio.

-- UP MIGRATION

ALTER TABLE ht_payments
    ADD COLUMN IF NOT EXISTS refund_of_payment_id INTEGER;

ALTER TABLE ht_payments
    ADD COLUMN IF NOT EXISTS refund_reason        VARCHAR(500);

-- Add the FK only if not already present (idempotency for re-runs).
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'fk_ht_payments_refund_of'
    ) THEN
        ALTER TABLE ht_payments
            ADD CONSTRAINT fk_ht_payments_refund_of
            FOREIGN KEY (refund_of_payment_id)
            REFERENCES ht_payments(pay_id)
            ON DELETE SET NULL;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS ix_ht_payments_refund_of_payment_id
    ON ht_payments (refund_of_payment_id)
    WHERE refund_of_payment_id IS NOT NULL;

-- DOWN MIGRATION (commented for reference)
-- DROP INDEX IF EXISTS ix_ht_payments_refund_of_payment_id;
-- ALTER TABLE ht_payments DROP CONSTRAINT IF EXISTS fk_ht_payments_refund_of;
-- ALTER TABLE ht_payments DROP COLUMN IF EXISTS refund_reason;
-- ALTER TABLE ht_payments DROP COLUMN IF EXISTS refund_of_payment_id;
