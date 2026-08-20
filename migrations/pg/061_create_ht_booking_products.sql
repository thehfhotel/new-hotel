-- Migration: 061_create_ht_booking_products
-- Version: vNext
-- Date: 2026-06-27
-- Task #52 (reservation enhancements) — pre-ordered products on a booking.
--
-- ## Background
--
-- iHOTEL lets a receptionist attach pre-booked products to a booking via
-- FrmAddBook2 (COMPAT_CHEATSHEET line 725-730, §3.4 step 3.5). Those rows
-- live in legacy `HT_Book_Pro` and are mirrored READ-ONLY into
-- `legacy_mirror.ht_book_pro` (migration 056). Until this migration our app
-- had no CANONICAL place to record a product a guest pre-orders at the moment
-- the reservation is taken, so the feature was iHOTEL-only.
--
-- This adds the canonical 1:N line table linking a booking to a product. It
-- mirrors the `ht_pos_sales` conventions (migration 052 — the in-stay POS
-- analog that links a product to a *check-in* folio) but keyed on the booking
-- instead of the folio, because a pre-order happens before check-in.
--
-- ## Schema notes
--
-- - `bp_book_id ON DELETE CASCADE` — pre-orders belong to the booking; if the
--   booking row is purged the pre-order lines go with it (same cascade
--   direction as `ht_pos_sales.sale_cin_id`).
-- - `bp_product_id` references `ht_products(prod_id)` (canonical product
--   master, migration 041) but NOT cascading: products are durable inventory;
--   a hard-deleted product keeps the audit line referencing the (dangling) id
--   so the reservation history stays intact. Soft-delete (`prod_active=FALSE`)
--   is the supported path.
-- - `bp_total` is a STORED generated column (`qty × unit_price`) — same
--   stale-total-proofing as `ht_pos_sales.sale_total`.
-- - `bp_legacy_id` is the (future) back-link to `HT_Book_Pro.id`. The legacy
--   write-back of booking pre-orders is NOT shipped in task #52 (the
--   `HT_Book_Pro` INSERT shape is unverified — `docs/legacy-spike/findings.md`
--   only captures a DELETE; the table is empty in both DBs). Column + partial
--   UNIQUE index exist now so a follow-up can stamp it without a schema change.
-- - `source` distinguishes our-app rows (`'canonical'`) from any future
--   reverse-synced iHOTEL rows (`'legacy'`), matching `ht_pos_sales.source`.
-- - `aggregate_id UUID` (nullable, partial UNIQUE) follows the
--   `ht_products.aggregate_id` shape so a downstream event/writeback path can
--   correlate without a back-link round-trip; minted at INSERT by the service.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_booking_products (
    bp_id           BIGSERIAL    PRIMARY KEY,
    bp_book_id      INTEGER      NOT NULL REFERENCES ht_bookings(book_id) ON DELETE CASCADE,
    bp_product_id   BIGINT       NOT NULL REFERENCES ht_products(prod_id),
    bp_qty          NUMERIC(10, 3) NOT NULL CHECK (bp_qty > 0),
    bp_unit_price   NUMERIC(12, 2) NOT NULL CHECK (bp_unit_price >= 0),
    -- Materialized line total: durable evidence of the pre-order at the moment
    -- it was taken (qty × unit_price), even if the catalog price drifts later.
    bp_total        NUMERIC(14, 2) GENERATED ALWAYS AS (bp_qty * bp_unit_price) STORED,
    bp_note         VARCHAR(500),
    bp_legacy_id    INTEGER,
    source          VARCHAR(20)  NOT NULL DEFAULT 'canonical',
    aggregate_id    UUID,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_booking_products_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_booking_products_book_id_idx
    ON ht_booking_products (bp_book_id);

CREATE INDEX IF NOT EXISTS ht_booking_products_product_id_idx
    ON ht_booking_products (bp_product_id);

CREATE UNIQUE INDEX IF NOT EXISTS ht_booking_products_legacy_id_uq
    ON ht_booking_products (bp_legacy_id) WHERE bp_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_booking_products_aggregate_id_uq
    ON ht_booking_products (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ht_booking_products_aggregate_id_uq;
-- DROP INDEX IF EXISTS ht_booking_products_legacy_id_uq;
-- DROP INDEX IF EXISTS ht_booking_products_product_id_idx;
-- DROP INDEX IF EXISTS ht_booking_products_book_id_idx;
-- DROP TABLE IF EXISTS ht_booking_products;
-- DELETE FROM schema_migrations WHERE version = '061';
