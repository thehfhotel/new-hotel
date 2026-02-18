-- Migration: 009_widen_legacy_varchar_columns
-- Version: 2.17.1
-- Date: 2026-02-18
--
-- Widens varchar columns in legacy sync tables to prevent truncation errors.
-- SQL Server data often has longer values than initially expected.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

-- ht_customers_legacy: widen columns that may exceed 50 chars
ALTER TABLE ht_customers_legacy ALTER COLUMN cust_no TYPE VARCHAR(100);
ALTER TABLE ht_customers_legacy ALTER COLUMN cust_type TYPE VARCHAR(100);
ALTER TABLE ht_customers_legacy ALTER COLUMN cust_phone TYPE VARCHAR(200);
ALTER TABLE ht_customers_legacy ALTER COLUMN cust_idcard TYPE VARCHAR(100);

-- =============================================================================
-- DOWN MIGRATION (commented - would truncate data)
-- =============================================================================
-- ALTER TABLE ht_customers_legacy ALTER COLUMN cust_no TYPE VARCHAR(50);
-- ALTER TABLE ht_customers_legacy ALTER COLUMN cust_type TYPE VARCHAR(50);
-- ALTER TABLE ht_customers_legacy ALTER COLUMN cust_phone TYPE VARCHAR(50);
-- ALTER TABLE ht_customers_legacy ALTER COLUMN cust_idcard TYPE VARCHAR(50);
