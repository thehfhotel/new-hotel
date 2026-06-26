-- Migration: 058_add_shift_cash_count
-- Version: x.x.x
-- Date: 2026-06-27
--
-- Track J7c — cash-drawer reconciliation at round close. iHOTEL's
-- ReportShipCash includes a physical cash-count-by-denomination with a
-- counted-vs-expected variance. We store the cashier's count on the shift row
-- so the round report (GET /api/shifts/{id}/report) can show:
--   expected_cash = shift_opening_float + SUM(ledger_cash over the round window)
--   counted_cash  = SUM(denomination × count)   [server-computed, not trusted]
--   variance      = counted_cash - expected_cash
-- Only the CASH tender feeds expected (credit/transfer/web don't touch the
-- drawer). Per-site (these columns live in both hotelnew + hotelville, like
-- the rest of ht_shifts).
--
-- `shift_cash_count` keeps the raw {denomination: count} map (JSONB) for the
-- audit breakdown; `shift_counted_cash` is the server-computed total so the
-- report needn't re-sum the map. Both NULL until a close supplies a count
-- (a close MAY omit it — the report then shows expected only, variance NULL).

-- UP MIGRATION
ALTER TABLE ht_shifts
    ADD COLUMN IF NOT EXISTS shift_counted_cash NUMERIC(14,2),
    ADD COLUMN IF NOT EXISTS shift_cash_count   JSONB;

COMMENT ON COLUMN ht_shifts.shift_counted_cash IS
    'Track J7c — server-computed physical cash counted at close '
    '(SUM denomination×count from shift_cash_count). NULL when the close '
    'supplied no drawer count.';
COMMENT ON COLUMN ht_shifts.shift_cash_count IS
    'Track J7c — raw {denomination: count} map the cashier entered at close, '
    'kept for the audit breakdown in the round report.';

-- DOWN MIGRATION (commented)
-- ALTER TABLE ht_shifts
--     DROP COLUMN IF EXISTS shift_cash_count,
--     DROP COLUMN IF EXISTS shift_counted_cash;
