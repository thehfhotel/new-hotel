-- Migration: 065_create_ht_pos_receipts
-- Version: vNext
-- Date: 2026-06-27
-- Task #45 — POS walk-up (roomless) sale + standalone receipt.
--
-- ## Background
--
-- Track G6 (migration 052) shipped folio POS: a cashier rings up a product
-- against an ACTIVE check-in and the line lands in `ht_pos_sales`
-- (→ legacy `HT_CheckIn_Product`). That path hard-rejects a sale with no
-- active check-in (`service/pos.rs`).
--
-- iHOTEL also supports a roomless / walk-up sale (`FrmAddSale` /
-- `FrmReceiptMain`): the cashier sells products to a customer who is NOT
-- staying, and the sale is recorded as a standalone receipt — one
-- `HT_Receipt_H` header + N `HT_Receipt_Ds` lines, joined by
-- `HT_Receipt_Ds.S_Sale_id = HT_Receipt_H.id` (NO `HT_CheckIn_Product`
-- write — see `docs/legacy-app/COMPAT_CHEATSHEET.md` §3.8 note). This
-- migration adds the canonical receipt header + line tables that mirror
-- that pair so the new app can ring up a walk-up sale and write it back.
--
-- ## Why a NEW table rather than reusing `ht_pos_sales`
--
-- `ht_pos_sales.sale_cin_id` is `NOT NULL REFERENCES ht_checkins(cin_id)`
-- — every folio sale is anchored to a check-in. A walk-up sale has no
-- check-in, so it cannot live there. The receipt model (header + lines)
-- also maps 1:1 onto the legacy `HT_Receipt_H` / `HT_Receipt_Ds` shape
-- the writeback recipe targets.
--
-- ## Schema notes
--
-- - `ht_pos_receipts` = canonical mirror of `HT_Receipt_H` (header).
--   `receipt_legacy_id` (HT_Receipt_H.id) + `receipt_legacy_no`
--   (HT_Receipt_H.Receipt_no, `B{yyMM}-{4digit}`) are back-links the
--   writeback worker stamps after the legacy INSERT lands. Partial UNIQUE
--   on `receipt_legacy_id` excludes NULLs (canonical-origin rows stay
--   distinct from legacy-origin rows until back-population).
-- - `ht_pos_receipt_lines` = canonical mirror of `HT_Receipt_Ds`. Keyed
--   on the header (`line_receipt_id` FK + CASCADE). `line_product_id`
--   references `ht_products(prod_id)` (no cascade — revenue history
--   survives a product hard-delete). `line_total` is computed at write
--   time (qty × unit_price − discount) — NOT a generated column, because
--   the per-line discount makes the arithmetic caller-driven.
-- - `receipt_status` mirrors `ht_pos_sales.sale_status` ('posted' /
--   'voided'); a void flips it + sets legacy `status_name='ยกเลิก'`.
-- - `source` distinguishes our-app rows ('canonical') from rows mirrored
--   in from iHOTEL ('legacy') — same convention as `ht_pos_sales`.
-- - Site-scoping is connection-level (the `hotelnew` / `hotelville`
--   logical DBs), NOT a row column — same as every other `ht_*` table.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_pos_receipts (
    receipt_id            BIGSERIAL    PRIMARY KEY,
    -- Customer header — all default to the legacy "no customer" sentinels
    -- so a quick walk-up sale needs no customer record.
    receipt_customer_no   VARCHAR(50)  NOT NULL DEFAULT 'C0000',
    receipt_customer_name VARCHAR(500) NOT NULL DEFAULT '',
    receipt_customer_addr VARCHAR(500) NOT NULL DEFAULT '',
    receipt_customer_tel  VARCHAR(50)  NOT NULL DEFAULT '',
    receipt_tax_id        VARCHAR(50)  NOT NULL DEFAULT '',
    -- Money. `receipt_total` is the VAT-inclusive grand total the printed
    -- receipt headlines; `receipt_before_vat` + `receipt_vat` are the
    -- split (legacy `Receipt_BeforeVat` / `Receipt_Vat`).
    receipt_subtotal      NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_discount      NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_total         NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_before_vat    NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_vat           NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_vat_percent   INTEGER        NOT NULL DEFAULT 0,
    -- Direct-pay (legacy `Cin_Pro_pay` analog at receipt level). A walk-up
    -- sale is settled on the spot, so `receipt_paid` defaults to the total
    -- in the service when the caller doesn't override.
    receipt_paid          NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_payment_method VARCHAR(20)   NOT NULL DEFAULT 'cash',
    receipt_note          VARCHAR(500)   NOT NULL DEFAULT '',
    receipt_status        VARCHAR(20)    NOT NULL DEFAULT 'posted',
    receipt_sold_by       VARCHAR(64),
    receipt_sold_at       TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    receipt_legacy_id     INTEGER,
    receipt_legacy_no     VARCHAR(50),
    source                VARCHAR(20)    NOT NULL DEFAULT 'canonical',
    aggregate_id          UUID           NOT NULL,
    created_at            TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_pos_receipts_status_check
        CHECK (receipt_status IN ('posted', 'voided')),
    CONSTRAINT ht_pos_receipts_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_pos_receipts_sold_at_idx
    ON ht_pos_receipts (receipt_sold_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_receipts_legacy_id_uq
    ON ht_pos_receipts (receipt_legacy_id) WHERE receipt_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_receipts_aggregate_id_uq
    ON ht_pos_receipts (aggregate_id);

CREATE TABLE IF NOT EXISTS ht_pos_receipt_lines (
    line_id          BIGSERIAL    PRIMARY KEY,
    line_receipt_id  BIGINT       NOT NULL REFERENCES ht_pos_receipts(receipt_id) ON DELETE CASCADE,
    line_product_id  BIGINT       REFERENCES ht_products(prod_id),
    line_product_no  VARCHAR(50)  NOT NULL DEFAULT '',
    line_product_name VARCHAR(255) NOT NULL DEFAULT '',
    line_unit_name   VARCHAR(50)  NOT NULL DEFAULT '',
    line_qty         NUMERIC(10, 3) NOT NULL CHECK (line_qty > 0),
    line_unit_price  NUMERIC(12, 2) NOT NULL CHECK (line_unit_price >= 0),
    line_discount    NUMERIC(12, 2) NOT NULL DEFAULT 0,
    -- Computed at write time (qty × unit_price − discount). Not a generated
    -- column because the per-line discount is caller-driven.
    line_total       NUMERIC(14, 2) NOT NULL DEFAULT 0,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ht_pos_receipt_lines_receipt_id_idx
    ON ht_pos_receipt_lines (line_receipt_id);

CREATE INDEX IF NOT EXISTS ht_pos_receipt_lines_product_id_idx
    ON ht_pos_receipt_lines (line_product_id);

-- DOWN MIGRATION (commented)
-- DROP TABLE IF EXISTS ht_pos_receipt_lines;
-- DROP TABLE IF EXISTS ht_pos_receipts;
-- DELETE FROM schema_migrations WHERE version = '065';
