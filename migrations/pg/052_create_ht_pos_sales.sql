-- Migration: 052_create_ht_pos_sales
-- Version: vNext
-- Date: 2026-05-14
-- Track G6 — POS / sales-to-room module (MVP).
--
-- ## Background
--
-- iHOTEL has a POS module that lets a cashier ring up F&B / laundry /
-- minibar / amenity products and charge them to a guest's current
-- folio. The legacy app inserts one row per line item into
-- `HT_CheckIn_Product` (see `docs/legacy-app/SCHEMA.sql` §"Table: dbo.HT_CheckIn_Product" "[id] int IDENTITY NOT NULL"
-- — was a bare SCHEMA.sql line-17 citation, which landed on the closing
-- paren of `HT_Bank_Accounts` after the 2026-06-11 regeneration — and
-- `docs/legacy-spike/schema/01-baseline-schema.txt` lines 275-286 for
-- the verified column list — 12 cols, `id` IDENTITY).
--
-- Until G6 our app had no equivalent: receptionists fell back to iHOTEL
-- whenever a guest wanted a coke or a laundry charge added to the
-- folio. Track F3 (migration 041) shipped the `ht_products` catalog;
-- this migration adds the sale-line table that links a product to a
-- check-in folio and to the stock-adjust write path.
--
-- ## Schema notes
--
-- - `sale_cin_id ON DELETE CASCADE` — if a folio is purged the sale
--   lines go with it. Matches the cascade direction the legacy app
--   uses (DELETE FROM HT_CheckIn_Product WHERE Cin_no=... on every
--   check-in cancel / re-issue).
-- - `sale_product_id` references `ht_products(prod_id)` (canonical
--   product master, F3) but NOT cascading: products are durable
--   inventory; if a product row is hard-deleted we keep the audit
--   line referencing the (now-dangling) id so revenue history stays
--   intact. Soft-delete via `prod_active = FALSE` is the supported
--   path.
-- - `sale_total` is a STORED generated column (`qty × unit_price`).
--   Computing it at write time inside the DB eliminates the "stale
--   total" drift that would creep in if a future UPDATE only touched
--   qty or unit_price.
-- - `sale_legacy_id` is the back-link to `HT_CheckIn_Product.id`
--   IDENTITY. Partial UNIQUE index excludes NULLs so canonically-
--   originated rows stay distinct from legacy-origin rows until the
--   writeback worker stamps the legacy id. Matches the pattern from
--   `ht_room_changes.rc_legacy_id` (migration 045) and
--   `ht_checkin_rooms.cr_legacy_ds_id` (migration 043).
-- - `aggregate_id UUID UNIQUE NOT NULL` — every line gets a stable
--   v5 UUID minted at INSERT time so downstream subscribers (events,
--   reconcile, future audit feeds) can correlate without round-trip-
--   ing through the SERIAL PK. Same convention as `ht_payments` /
--   `ht_room_changes`.
-- - `source VARCHAR(20) NOT NULL DEFAULT 'canonical'` — distinguishes
--   rows that originated in our app (`'canonical'`) from rows mirrored
--   in from iHOTEL via the sync mapper (`'legacy'`). Lets the
--   reconcile job differentiate one-sided rows that haven't yet been
--   written back.
-- - `sale_status` — 'posted' or 'voided'. Voids are out of scope for
--   the MVP (see `### Deferred` in CHANGELOG); the column exists so
--   a follow-up PR can ship the void flow without a schema migration.
--
-- ## Deferred (NOT in this migration)
--
-- See the matching CHANGELOG block. Captured as TODO comments in the
-- service / route code, not as migration TODOs.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_pos_sales (
    sale_id          BIGSERIAL    PRIMARY KEY,
    sale_cin_id      INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    sale_product_id  BIGINT       NOT NULL REFERENCES ht_products(prod_id),
    sale_qty         NUMERIC(10, 3) NOT NULL CHECK (sale_qty > 0),
    sale_unit_price  NUMERIC(12, 2) NOT NULL CHECK (sale_unit_price >= 0),
    -- Total is materialized inside the row: row is durable evidence of
    -- the sale at the moment it was posted (qty × unit_price), even if
    -- the catalog price drifts later.
    sale_total       NUMERIC(14, 2) GENERATED ALWAYS AS (sale_qty * sale_unit_price) STORED,
    sale_sold_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    sale_sold_by     VARCHAR(64),
    sale_note        VARCHAR(500),
    sale_status      VARCHAR(20)  NOT NULL DEFAULT 'posted',
    sale_legacy_id   INTEGER,
    source           VARCHAR(20)  NOT NULL DEFAULT 'canonical',
    aggregate_id     UUID         NOT NULL,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_pos_sales_status_check
        CHECK (sale_status IN ('posted', 'voided')),
    CONSTRAINT ht_pos_sales_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_pos_sales_cin_id_idx
    ON ht_pos_sales (sale_cin_id);

CREATE INDEX IF NOT EXISTS ht_pos_sales_product_id_idx
    ON ht_pos_sales (sale_product_id);

CREATE INDEX IF NOT EXISTS ht_pos_sales_sold_at_idx
    ON ht_pos_sales (sale_sold_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_sales_legacy_id_uq
    ON ht_pos_sales (sale_legacy_id) WHERE sale_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_sales_aggregate_id_uq
    ON ht_pos_sales (aggregate_id);

-- Track G7 / migration 046 — extend the permission grid with `pos.sell`
-- so the cashier and admin roles can ring up POS sales. Receptionists
-- and housekeepers do NOT get the permission by default (cashier is
-- the canonical role for till operations, per iHOTEL convention).
-- ON CONFLICT is required because migration 046 may not yet have a row
-- for `pos.sell` (introduced here for the first time).
INSERT INTO ht_permissions (permission_key, description)
VALUES (
    'pos.sell',
    'Ring up a POS sale and charge it to a check-in folio (Track G6)'
)
ON CONFLICT (permission_key) DO NOTHING;

-- Grant to admin (already has the cross-join "all permissions" rule in
-- migration 046, but new permission keys added after the migration ran
-- need an explicit grant to keep admin universal).
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key = 'pos.sell'
WHERE r.role_key = 'admin'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Grant to cashier — the role that runs the till in iHOTEL.
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key = 'pos.sell'
WHERE r.role_key = 'cashier'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- DOWN MIGRATION (commented — destructive)
-- DELETE FROM ht_role_permissions WHERE permission_id IN
--     (SELECT permission_id FROM ht_permissions WHERE permission_key = 'pos.sell');
-- DELETE FROM ht_permissions WHERE permission_key = 'pos.sell';
-- DROP INDEX IF EXISTS ht_pos_sales_aggregate_id_uq;
-- DROP INDEX IF EXISTS ht_pos_sales_legacy_id_uq;
-- DROP INDEX IF EXISTS ht_pos_sales_sold_at_idx;
-- DROP INDEX IF EXISTS ht_pos_sales_product_id_idx;
-- DROP INDEX IF EXISTS ht_pos_sales_cin_id_idx;
-- DROP TABLE IF EXISTS ht_pos_sales;
