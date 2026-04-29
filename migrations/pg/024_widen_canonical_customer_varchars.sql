-- Migration 024: widen canonical ht_customers VARCHAR columns
--
-- Backfill of the gap left by migration 009 (which widened the cache
-- table ht_customers_legacy but didn't touch canonical ht_customers).
-- Discovered 2026-04-29 during Phase 5 Ville backfill_legacy run —
-- one Ville customer's `Cust_Add_tel` is 21 chars, blowing past
-- canonical `cust_phone VARCHAR(20)`.
--
-- Mirrors the same widths migration 009 applied to the cache:
--   cust_code:     VARCHAR(20) → VARCHAR(100)
--   cust_phone:    VARCHAR(20) → VARCHAR(200)
--   cust_idcard:   VARCHAR(20) → VARCHAR(100)
--   cust_title:    VARCHAR(20) → VARCHAR(100)
--   cust_taxid:    VARCHAR(20) → VARCHAR(100)
--   source:        VARCHAR(20) → VARCHAR(50)   (operator-supplied tag, plenty of room)
--   legacy_cust_no:VARCHAR(20) → VARCHAR(100)
--
-- ALTER COLUMN TYPE on PG only forbids non-trivial conversions; widening
-- VARCHAR is a fast metadata-only change (no row rewrite), safe under load.

BEGIN;

ALTER TABLE ht_customers ALTER COLUMN cust_code      TYPE VARCHAR(100);
ALTER TABLE ht_customers ALTER COLUMN cust_phone     TYPE VARCHAR(200);
ALTER TABLE ht_customers ALTER COLUMN cust_idcard    TYPE VARCHAR(100);
ALTER TABLE ht_customers ALTER COLUMN cust_title     TYPE VARCHAR(100);
ALTER TABLE ht_customers ALTER COLUMN cust_taxid     TYPE VARCHAR(100);
ALTER TABLE ht_customers ALTER COLUMN source         TYPE VARCHAR(50);
ALTER TABLE ht_customers ALTER COLUMN legacy_cust_no TYPE VARCHAR(100);

COMMIT;

-- Rollback (manual; only safe if no values exceed the narrower widths):
-- ALTER TABLE ht_customers ALTER COLUMN cust_code      TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN cust_phone     TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN cust_idcard    TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN cust_title     TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN cust_taxid     TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN source         TYPE VARCHAR(20);
-- ALTER TABLE ht_customers ALTER COLUMN legacy_cust_no TYPE VARCHAR(20);
