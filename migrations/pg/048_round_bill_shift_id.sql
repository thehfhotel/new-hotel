-- Migration: 048_round_bill_shift_id
-- Version: 2.63.18
-- Date: 2026-05-14
-- Description: Track G9 / T4 HIGH-8 — record the cashier shift that
--              folded each round-bill (checkout) so the daily cashier
--              report can attribute revenue per shift.
--
-- Why: Track F2 (migration 040) added `ht_shifts` + a payment gate on
-- `service::payment::record_payment`. The audit's T5 race-safety finding
-- noted that the same gate was missing on the round-bill (final
-- check-out) path. Without it:
--   - A cashier can fold a folio with no shift open → end-of-shift
--     reconciliation can't tell who closed which folio.
--   - Folding into a closed shift corrupts the daily cashier report.
--
-- G9 extends F2's gate to `service::checkin::check_out` AND records the
-- `ht_shifts.shift_id` on the `ht_checkins` row so per-shift revenue
-- attribution is possible from canonical PG (the legacy `HT_Receipt_H`
-- and `HT_Round_Bill` have no column linking them — see
-- `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Round_Bill`" and
-- `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Receipt_H`"). So the column is canonical-only; the
-- writeback recipe for check-out (`writeback/recipes/checkout.rs`) is
-- unchanged — we don't invent legacy columns.
--
-- Cardinality: `ht_checkins.cin_round_bill_shift_id` -> `ht_shifts.shift_id`
-- is `N:1` (many checkouts can be folded inside one shift). NULL until
-- the folio is folded; the FK uses ON DELETE SET NULL so retiring an
-- erroneously-recorded shift doesn't cascade away check-in history.
--
-- Idempotent: ADD COLUMN IF NOT EXISTS + ADD CONSTRAINT guarded by an
-- existence check in pg_constraint.

-- UP MIGRATION

ALTER TABLE ht_checkins
    ADD COLUMN IF NOT EXISTS cin_round_bill_shift_id BIGINT;

-- FK guarded so a re-run on a schema that already added the constraint
-- (e.g. after a partial migration / manual fix) is a no-op.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
         WHERE conname = 'fk_ht_checkins_round_bill_shift'
    ) THEN
        ALTER TABLE ht_checkins
            ADD CONSTRAINT fk_ht_checkins_round_bill_shift
            FOREIGN KEY (cin_round_bill_shift_id)
            REFERENCES ht_shifts(shift_id)
            ON DELETE SET NULL;
    END IF;
END
$$;

-- Per-shift revenue attribution read pattern: "give me every folio
-- folded in shift N". Partial index — most rows stay NULL until the
-- final round-bill, so a partial index is dramatically smaller than a
-- full-table one.
CREATE INDEX IF NOT EXISTS ix_ht_checkins_round_bill_shift_id
    ON ht_checkins (cin_round_bill_shift_id)
    WHERE cin_round_bill_shift_id IS NOT NULL;

-- DOWN MIGRATION (commented — would lose per-shift attribution for any
-- check-out already folded under the new gate):
-- DROP INDEX IF EXISTS ix_ht_checkins_round_bill_shift_id;
-- ALTER TABLE ht_checkins DROP CONSTRAINT IF EXISTS fk_ht_checkins_round_bill_shift;
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS cin_round_bill_shift_id;
