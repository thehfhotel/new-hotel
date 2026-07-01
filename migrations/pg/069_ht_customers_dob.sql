-- Migration: 069_ht_customers_dob
-- Version: vNext
-- Date: 2026-07-01
-- Check-in registration — date of birth capture.
--
-- ## Background
--
-- The check-in registration flow (Thai ID chip read + passport MRZ parse)
-- surfaces a guest date of birth. There is no home for it on the canonical
-- customer row today, so it was dropped. This adds `cust_dob DATE`.
--
-- ## Canonical-only (NOT mirrored to legacy)
--
-- The legacy `HT_Customers` table has NO date-of-birth column, so this field
-- is PG-canonical-only — there is nothing to write it back to and the customer
-- sync mapper does not read it. It rides the existing customer resave/enrichment
-- path (dynamic `sqlx::query` COALESCE update) with no legacy counterpart.

-- UP MIGRATION
ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS cust_dob DATE;

COMMENT ON COLUMN ht_customers.cust_dob IS
  'Guest date of birth captured at check-in registration (Thai ID chip / '
  'passport MRZ). PG-canonical-only — legacy HT_Customers has no DOB column, '
  'so this is never mirrored to MSSQL. Migration 069.';

-- DOWN MIGRATION (commented)
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS cust_dob;
-- DELETE FROM schema_migrations WHERE version = '069';
