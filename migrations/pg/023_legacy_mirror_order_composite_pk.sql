-- Migration 023: legacy_mirror.ht_order_up / ht_order_down composite PK
--
-- Problem (diagnosed 2026-04-29 during Phase 5 Ville bootstrap):
-- The legacy `HT_Order_Up` / `HT_Order_Down` tables at Ville carry
-- 8 rows each, and `id` is a tier-number column (1, 2, 3) — NOT
-- the unique key. The real composite key is `(id, cust_type, cast_type)`
-- where `id` is the tier, `cust_type` is the customer-class label,
-- and `cast_type` is the pricing-class label.
--
-- The original `legacy_mirror.ht_order_up/_down` schema (init-hotelnew.sql
-- migration 020) declared `id INTEGER PRIMARY KEY` (single-column),
-- which blew up the dimension reload TX with `duplicate key value
-- violates unique constraint` on every reconcile tick. HF Hotel never
-- hit this because both legacy tables are EMPTY at HF Hotel — the
-- bug only surfaces when a site (Ville) actually populates them.
--
-- Fix: change PK to composite (id, cust_type, cast_type). Mirrors
-- the legacy app's actual semantics. Both DBs converge on the correct
-- schema.
--
-- Pre-flight verified 2026-04-29:
--   * legacy_mirror.ht_order_up at hotelnew: 0 rows
--   * legacy_mirror.ht_order_up at hotelville: 0 rows (TX rolled back)
--   * legacy_mirror.ht_order_down at hotelnew: 0 rows
--   * legacy_mirror.ht_order_down at hotelville: 0 rows
-- Both empty → ALTER COLUMN NOT NULL is trivial, no row-data conversion.
--
-- This migration also gets folded into init-hotelnew.sql so fresh
-- deploys reach the corrected baseline directly.

BEGIN;

-- HT_Order_Up
ALTER TABLE legacy_mirror.ht_order_up DROP CONSTRAINT IF EXISTS ht_order_up_pkey;
ALTER TABLE legacy_mirror.ht_order_up ALTER COLUMN cust_type SET NOT NULL;
ALTER TABLE legacy_mirror.ht_order_up ALTER COLUMN cast_type SET NOT NULL;
ALTER TABLE legacy_mirror.ht_order_up ADD CONSTRAINT ht_order_up_pkey
    PRIMARY KEY (id, cust_type, cast_type);

-- HT_Order_Down
ALTER TABLE legacy_mirror.ht_order_down DROP CONSTRAINT IF EXISTS ht_order_down_pkey;
ALTER TABLE legacy_mirror.ht_order_down ALTER COLUMN cust_type SET NOT NULL;
ALTER TABLE legacy_mirror.ht_order_down ALTER COLUMN cast_type SET NOT NULL;
ALTER TABLE legacy_mirror.ht_order_down ADD CONSTRAINT ht_order_down_pkey
    PRIMARY KEY (id, cust_type, cast_type);

COMMIT;

-- Rollback (manual, not run by migrate.sh):
-- BEGIN;
-- ALTER TABLE legacy_mirror.ht_order_up DROP CONSTRAINT IF EXISTS ht_order_up_pkey;
-- ALTER TABLE legacy_mirror.ht_order_up ALTER COLUMN cust_type DROP NOT NULL;
-- ALTER TABLE legacy_mirror.ht_order_up ALTER COLUMN cast_type DROP NOT NULL;
-- ALTER TABLE legacy_mirror.ht_order_up ADD CONSTRAINT ht_order_up_pkey
--     PRIMARY KEY (id);
-- (and same for ht_order_down)
-- COMMIT;
