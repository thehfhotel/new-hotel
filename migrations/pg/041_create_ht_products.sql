-- Migration: 041_create_ht_products
-- Version: vNext
-- Date: 2026-05-13
-- Description: Track F3 — `ht_products` canonical table mirroring legacy
--              `HT_Products` + `ht_inventory_items.inv_product_id` FK linkage.
--
-- Source: `docs/coexistence/audit-2026-05-13.md` Track F — T1 CRIT-3.
--
-- ## Problem statement
--
-- `HT_Products` is the legacy product master (room-rent line `'P001'`,
-- minibar SKUs, amenity charges) with `Pro_Amt` as the running stock
-- counter. Per `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Product`" "Stock change cascade", every
-- `HT_CheckIn_Product` INSERT/UPDATE/DELETE is paired with
-- `UPDATE HT_Products SET Pro_Amt = Pro_Amt ± num WHERE Pro_no='<id>'`
-- so the stock counter stays consistent with consumption.
--
-- Our app today has `ht_inventory_items` (housekeeping/POS items —
-- categories Minibar/Amenities/Linens/Equipment) which **does not link
-- to any legacy product row**. Stock changes our app makes never
-- decrement `Pro_Amt`, and stock movements iHOTEL makes never
-- propagate to `ht_inventory_items`. The two tables drift apart on
-- every product consumption event.
--
-- ## What this migration does
--
-- 1. Creates `ht_products` mirroring `HT_Products` keyed on
--    `prod_legacy_no = Pro_no` (1:1 cardinality). The mapper
--    `hotel-backend/src/sync/mappers/products.rs` populates this from
--    a periodic-poll path (CT enablement on `HT_Products` is deferred
--    to a sibling legacy-mssql migration — see mapper docstring).
--
-- 2. Adds `ht_inventory_items.inv_product_id` FK so housekeeping /
--    POS items can OPTIONALLY link to the canonical product master.
--    Existing rows (Minibar / Amenities / Linens / Equipment seeds)
--    stay un-linked; the column is nullable so present semantics are
--    preserved. New items that represent a legacy product carry the
--    FK and can be reconciled via the `AdjustProductStock` writeback.
--
-- Idempotent: every CREATE / ALTER uses `IF NOT EXISTS`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_products (
    prod_id           BIGSERIAL PRIMARY KEY,
    -- `HT_Products.Pro_no` — business key (e.g. `'B-001'`, `'P001'` for
    -- the sentinel "room rent line"). 1:1 with the canonical row.
    prod_legacy_no    VARCHAR(50)  NOT NULL UNIQUE,
    -- `HT_Products.Pro_Name` — display name.
    prod_name         VARCHAR(250) NOT NULL,
    -- `HT_Products.Pro_Unit` — unit of measure (bottle, piece, set, …).
    prod_unit         VARCHAR(50),
    -- Default selling price. Mirrors `HT_Products.Pro_PriceA` (rate A —
    -- normal-customer price). Per-customer-type overrides
    -- (`Pro_PriceB/C`) are read-only mirrored for now; rate-axis work
    -- belongs in Track F's `ht_rate_tiers` (T1 CRIT-4).
    prod_price        NUMERIC(12,2) NOT NULL DEFAULT 0,
    -- `HT_Products.Pro_Amt` — running stock counter. Stamped by the
    -- sync mapper and adjusted by the `AdjustProductStock` writeback
    -- recipe (closes the invariant from our app's writes).
    prod_current_stock NUMERIC(12,3) NOT NULL DEFAULT 0,
    -- `HT_Products.Pro_Type` — category (e.g. `'B'` for beverage).
    -- Free-text legacy literal, preserved as-is.
    prod_category     VARCHAR(100),
    -- Soft-active flag. Legacy doesn't have this; we default to TRUE
    -- and let admins archive items locally without affecting MSSQL.
    prod_active       BOOLEAN NOT NULL DEFAULT TRUE,
    -- Aggregate UUID (v5 over `prod_id`). Pinned on first INSERT by
    -- the sync mapper so subscribers can deduplicate stock-adjust
    -- events.
    aggregate_id      UUID,
    prod_created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    prod_updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_products_aggregate_id
    ON ht_products (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- The UNIQUE on prod_legacy_no already implies a btree index, but the
-- mapper's hot path (`WHERE prod_legacy_no = $1`) benefits from the
-- IF NOT EXISTS sentinel for forward-compat with deduplication tools.
CREATE INDEX IF NOT EXISTS ht_products_legacy_no ON ht_products(prod_legacy_no);

-- Track F3 linkage: housekeeping / POS items optionally point to a
-- canonical product row. NULL means "this inventory item has no legacy
-- counterpart" (the existing Minibar / Amenities / Linens / Equipment
-- seeds default to NULL — their semantics are unchanged).
ALTER TABLE ht_inventory_items
    ADD COLUMN IF NOT EXISTS inv_product_id INTEGER
        REFERENCES ht_products(prod_id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_product
    ON ht_inventory_items(inv_product_id)
    WHERE inv_product_id IS NOT NULL;

-- DOWN MIGRATION (commented — intentional, deletion would orphan
-- writeback rows mid-flight that already referenced prod_legacy_no):
--
-- ALTER TABLE ht_inventory_items DROP COLUMN IF EXISTS inv_product_id;
-- DROP INDEX IF EXISTS ix_ht_inventory_items_product;
-- DROP TABLE IF EXISTS ht_products;
