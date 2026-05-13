-- Migration: 038_seed_vat_percent
-- Version: 2.63.15
-- Date: 2026-05-13
-- Description: Seed `ht_settings.vat_percent` so the payment writeback recipe
--              reads the VAT rate from PG instead of the hardcoded
--              `RECEIPT_VAT_PERCENT` constant.
--
-- Wave 5c (`docs/coexistence/audit-2026-05-13.md` Decision #2): the legacy
-- `HT_Receipt_H.Receipt_VatPer` column was previously stamped with the value
-- of the in-binary constant `writeback::constants::RECEIPT_VAT_PERCENT = 7`.
-- That forced a code-change-+-deploy cycle to flip between 0% and 7% VAT —
-- inconvenient for hotels that need a maintenance toggle.
--
-- This migration seeds a single row keyed by `setting_key = 'vat_percent'`
-- with value `'7.0'` (the hotel's standard Thai VAT today). The route layer
-- (`hotel-backend/src/routes/new_payments.rs`) reads it via
-- `repository::settings::get_vat_percent`, threads it through to the writeback
-- recipe via `WritebackIntent::RecordPayment.vat_percent`, and the recipe
-- stamps it into `HT_Receipt_H.Receipt_VatPer` + uses it for the VAT split.
--
-- To flip the rate at runtime:
--   UPDATE ht_settings SET setting_value = '0' WHERE setting_key = 'vat_percent';
-- (Or '7', '7.0', etc — the repository accepts both integer and decimal shapes.)
--
-- Idempotent: `ON CONFLICT (setting_key) DO NOTHING` preserves any existing
-- value (e.g. a hotel that already manually inserted the row). The
-- `vat_percent_seeded` migrations entry is appended below.

-- UP MIGRATION
INSERT INTO ht_settings (setting_key, setting_value, setting_type, setting_description)
VALUES (
    'vat_percent',
    '7.0',
    'string',
    'Receipt VAT percent (Wave 5c). Read at payment time; threaded through '
    || 'WritebackIntent::RecordPayment.vat_percent and stamped into '
    || 'HT_Receipt_H.Receipt_VatPer. Accepts integer or decimal strings.'
)
ON CONFLICT (setting_key) DO NOTHING;

-- DOWN MIGRATION (commented — intentional, deletion would orphan receipts
-- mid-flight that already serialized the value into queued writeback intents):
-- DELETE FROM ht_settings WHERE setting_key = 'vat_percent';
