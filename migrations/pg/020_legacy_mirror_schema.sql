-- Migration: 020_legacy_mirror_schema
-- Version: 2.50.0
-- Date: 2026-04-28
-- Phase 5.5a — opaque pass-through mirror for legacy-only features.
--
-- Per docs/architecture.md §11, the legacy .NET app implements a
-- number of features our app does not (and likely never will, until
-- decommission): coupons, standalone deposits, in-stay POS / minibar,
-- mid-stay room moves, credit-sales ledger, hourly-extension pricing,
-- per-customer-type pricing tiers. During co-existence (State A / B),
-- receptionists may still author these in the .NET app; our app must
-- (a) never corrupt those rows on writeback and (b) ideally display
-- them so receptionists see a complete picture in our UI without
-- switching apps.
--
-- The strategy is **opaque pass-through**: Phase 5.5 mirrors the 11
-- relevant tables into a `legacy_mirror` PG schema as read-only
-- copies. UI surfaces them as informational panels. We never write
-- here from the application path (with one documented exception,
-- HT_Rooms_Cancel, which the writeback adapter already handles via
-- `checkin_cancel.rs`; the mirror row is informational duplication).
-- On decommission, the mirror schema is dropped.
--
-- This migration creates the schema and all 11 mirror tables. Phase
-- 5.5a additionally wires the 4 slow-changing dimension tables
-- (HT_ContinueTime, HT_Rooms_Price, HT_Order_Up, HT_Order_Down) into
-- the existing reconcile job for full-table reload. Phase 5.5b/5.5c
-- enable CT on the 6 transactional tables and add `MirrorMapper`s for
-- them — until then those tables exist but stay empty.
--
-- ## Type mapping (MSSQL → PG)
--
-- | MSSQL                | PG                  | Why |
-- |----------------------|---------------------|-----|
-- | `int`                | `INTEGER`           | exact match |
-- | `varchar(N)`         | `TEXT`              | PG idiom; varchar length limits rarely useful |
-- | `float` (53-prec)    | `DOUBLE PRECISION`  | MSSQL float == IEEE 754 double == PG double precision |
-- | `datetime`           | `TIMESTAMP`         | no TZ — values are Thai local per CLAUDE.md |
-- | `text`               | `TEXT`              | exact match |
--
-- ## PK strategy
--
-- Each mirror table uses the legacy natural key as its PK (declared
-- NOT NULL even when the legacy column is nullable — we reject NULL
-- PKs at the mapper boundary loud-and-clear rather than carrying
-- nullability into the mirror).
--
-- ## Bookkeeping columns
--
-- Two PG-side columns added per mirror table:
-- * `mirrored_at TIMESTAMPTZ` — last time the mirror row was
--   written/refreshed. Used by the dashboard to show staleness.
-- * `mirror_source TEXT` — `'reconcile'` (full-table reload by
--   `scheduler::sync`) or `'ct'` (incremental tick from the CT
--   watcher). Useful for debugging which path populated a given row.

-- UP MIGRATION

CREATE SCHEMA IF NOT EXISTS legacy_mirror;

-- ── 1. HT_Cupon ──────────────────────────────────────────────────
-- Food/breakfast vouchers generated per check-in. ~17,894 rows at
-- HF Hotel as of the 2026-04-24 snapshot — the most-active legacy-
-- only table. Coupons are generated/printed in `FrmCuponMain`; our
-- UI surfaces them as a "coupons attached" panel on the check-in
-- detail page.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_cupon (
    cupon_no       INTEGER          PRIMARY KEY,
    cupon_cin_no   TEXT,
    cupon_cin_room TEXT,
    cupon_date     TIMESTAMP,
    cupon_gen_date TIMESTAMP,
    cupon_by       TEXT,
    cupon_print    INTEGER          NOT NULL DEFAULT 0,
    mirrored_at    TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source  TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_cupon_cin_no_idx
    ON legacy_mirror.ht_cupon (cupon_cin_no);

-- ── 2. HT_CheckIn_Product ────────────────────────────────────────
-- In-stay POS / minibar charges per room. 0 rows at HF Hotel
-- (feature unused) but mirrored defensively. UI shows on folio.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_checkin_product (
    id                  INTEGER          PRIMARY KEY,
    cin_no              TEXT,
    cin_room_no         TEXT,
    cin_ds_date         TIMESTAMP,
    cin_pro_id          TEXT,
    cin_pro_name        TEXT,
    cin_pro_unit        TEXT,
    cin_pro_num         DOUBLE PRECISION,
    cin_pro_price       DOUBLE PRECISION,
    cin_pro_pricetotal  DOUBLE PRECISION,
    cin_pro_pay         DOUBLE PRECISION,
    cin_pro_note        TEXT,
    mirrored_at         TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source       TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_checkin_product_cin_no_idx
    ON legacy_mirror.ht_checkin_product (cin_no);

-- ── 3. HT_Deposit ────────────────────────────────────────────────
-- Standalone deposit ledger + refunds (legacy form FormShowDEPBack).
-- 0 rows at HF Hotel (this hotel uses booking-attached deposits
-- instead). UI shows "deposit on file" indicator.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_deposit (
    id            INTEGER          PRIMARY KEY,
    dep_no        TEXT,
    dep_date      TIMESTAMP,
    dep_room      TEXT,
    dep_name      TEXT,
    dep_price     DOUBLE PRECISION,
    dep_status    TEXT,
    dep_ref       TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

-- ── 4. HT_ContinueTime ───────────────────────────────────────────
-- Hourly extension price master. Tiny dimension table; eligible for
-- 5.5a full-table reload (no CT needed). Used by the legacy
-- "extend stay by N hours" flow.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_continuetime (
    id            INTEGER          PRIMARY KEY,
    con_name      TEXT,
    con_minute    INTEGER,
    con_price     DOUBLE PRECISION,
    con_type      TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

-- ── 5. HT_Changed_Room ───────────────────────────────────────────
-- Mid-stay room-move audit. ~3,866 rows at HF Hotel. UI shows in
-- stay history panel ("guest moved from 201 to 305 on …").
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_changed_room (
    id                INTEGER          PRIMARY KEY,
    cin_no            TEXT             NOT NULL,
    room_before       TEXT,
    room_after        TEXT,
    change_date       TIMESTAMP,
    room_before_price DOUBLE PRECISION NOT NULL DEFAULT 0,
    note              TEXT,
    toprice           TEXT,
    mirrored_at       TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source     TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_changed_room_cin_no_idx
    ON legacy_mirror.ht_changed_room (cin_no);

-- ── 6. HT_Rooms_Cancel ───────────────────────────────────────────
-- Per-room cancel audit (multi-room cancellation). 297 rows at
-- HF Hotel. NOTE: this is the documented exception — the writeback
-- adapter (`writeback::recipes::checkin_cancel`) DOES write here
-- when our app cancels a multi-room check-in. The mirror row exists
-- so the UI can read cancellation history without a MSSQL round-
-- trip; either the writeback path or a CT tick will populate it.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_rooms_cancel (
    id            INTEGER          PRIMARY KEY,
    room_no       TEXT,
    cin_no        TEXT,
    cancel_date   TIMESTAMP,
    cancel_by     TEXT,
    cancel_note   TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_rooms_cancel_cin_no_idx
    ON legacy_mirror.ht_rooms_cancel (cin_no);

-- ── 7. HT_Rooms_Price ────────────────────────────────────────────
-- Per-customer-type room price overrides (e.g. corporate rate vs
-- walk-in). 32 rows. Eligible for 5.5a full-table reload.
-- `bin/backfill_rooms.rs` already reads this for the canonical
-- ht_rates seed; this mirror row is for UI surfacing.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_rooms_price (
    id              INTEGER          PRIMARY KEY,
    room_type       TEXT,
    room_custtype   TEXT,
    room_price      DOUBLE PRECISION,
    room_price_h    DOUBLE PRECISION,
    room_price_m    DOUBLE PRECISION,
    mirrored_at     TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source   TEXT             NOT NULL
);

-- ── 8. HT_Bill_Debt_H ────────────────────────────────────────────
-- Credit-sales ledger header. 0 rows at HF Hotel (feature unused)
-- but mirrored defensively in case a vendor enables it.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_bill_debt_h (
    bill_no            TEXT             PRIMARY KEY,
    bill_cust_id       TEXT,
    bill_cust_name     TEXT,
    bill_cust_address  TEXT,
    bill_cust_tel      TEXT,
    bill_cust_fax      TEXT,
    bill_date          TIMESTAMP,
    bill_ref           TEXT,
    bill_price_type    TEXT,
    bill_type          TEXT,
    bill_total         DOUBLE PRECISION,
    bill_pay           DOUBLE PRECISION,
    bill_debt          DOUBLE PRECISION,
    bill_pay_cash      DOUBLE PRECISION,
    bill_pay_credit    DOUBLE PRECISION,
    bill_status        TEXT,
    bill_by            TEXT,
    bill_note          TEXT,
    mirrored_at        TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source      TEXT             NOT NULL
);

-- ── 9. HT_Bill_Debt_Ds ───────────────────────────────────────────
-- Credit-sales ledger detail. 0 rows at HF Hotel.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_bill_debt_ds (
    id              INTEGER          PRIMARY KEY,
    bill_no         TEXT,
    ds_id           INTEGER,
    ds_no           TEXT,
    ds_name         TEXT,
    ds_unit         TEXT,
    ds_num          DOUBLE PRECISION,
    ds_price        DOUBLE PRECISION,
    ds_price_total  DOUBLE PRECISION,
    mirrored_at     TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source   TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_bill_debt_ds_bill_no_idx
    ON legacy_mirror.ht_bill_debt_ds (bill_no);

-- ── 10. HT_Order_Up ──────────────────────────────────────────────
-- Per-customer-type pricing tier (price-up bracket). Tiny dimension
-- table; eligible for 5.5a full-table reload.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_order_up (
    id            INTEGER          PRIMARY KEY,
    cust_type     TEXT,
    cust_month    INTEGER,
    cast_type     TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

-- ── 11. HT_Order_Down ────────────────────────────────────────────
-- Per-customer-type pricing tier (price-down bracket). Tiny
-- dimension table; eligible for 5.5a full-table reload.
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_order_down (
    id            INTEGER          PRIMARY KEY,
    cust_type     TEXT,
    cust_month    INTEGER,
    cast_type     TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

-- DOWN MIGRATION (commented — destructive)
-- DROP SCHEMA legacy_mirror CASCADE;
