-- Migration: 035_track_e2_customer_columns
-- Version: 2.63.13
-- Date: 2026-05-13
-- Track E2 / T1 HIGH-2 (`docs/coexistence/audit-2026-05-13.md`) —
-- widen `ht_customers` so the CT mapper can mirror every column legacy
-- `HT_Customers` carries. The previous schema only captured ~8 of the
-- 35 legacy columns; this migration adds the missing 25+:
--
--   * `Cust_Price_Over` — running debt balance (business-critical;
--     legacy `Module1.UPDATE_MONEY` mutates this on every check-in/out
--     and audits via `HT_Log_Debt`). For Track E2 we only READ this
--     column — writeback of debt mutations is deferred to Track G.
--   * Address tuple (8 component fields) — legacy renders each part
--     separately in invoices. Previously collapsed to single
--     `cust_address` field; we keep that column populated from
--     `Cust_Add_no` for backwards-compat with single-line readers.
--   * Work-address tuple (12 fields including `Cust_Work_Tax`) —
--     required for corporate invoices.
--   * `Cust_name2` (English/secondary name) — used by FrmReportRR4.
--   * `Cust_Type` rate-tier label (kept distinct from existing
--     `cust_type` which mirrors the `Cust_Type_Main` category).
--   * `Cust_sex`, `Cust_Last_Change`, `Cust_Contry`.
--
-- All columns are nullable (matching legacy `NULL`-tolerant schema)
-- except `cust_price_over` which defaults to `0` to match legacy
-- `float NOT NULL DEFAULT 0` — but we use `NULL` here so the mapper's
-- `COALESCE` semantics work cleanly when legacy reports NULL.
--
-- Companion: `036_track_e2_room_columns.sql` for `ht_rooms_new`.

-- UP MIGRATION

ALTER TABLE ht_customers
    ADD COLUMN IF NOT EXISTS cust_name2          VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_sex            VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_price_tier     VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_add_no         VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_moo        VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_add_soi        VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_road       VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_tambon     VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_ampore     VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_province   VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_add_code       VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_add_fax        VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_name      VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_no        VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_moo       VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_work_soi       VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_road      VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_tambon    VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_ampore    VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_province  VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_code      VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_work_tel       VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_fax       VARCHAR(250),
    ADD COLUMN IF NOT EXISTS cust_work_tax       VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_last_change    TIMESTAMP,
    ADD COLUMN IF NOT EXISTS cust_contry         VARCHAR(50),
    ADD COLUMN IF NOT EXISTS cust_price_over     DOUBLE PRECISION;

-- DOWN MIGRATION (commented — destructive)
-- ALTER TABLE ht_customers
--     DROP COLUMN IF EXISTS cust_name2,
--     DROP COLUMN IF EXISTS cust_sex,
--     DROP COLUMN IF EXISTS cust_price_tier,
--     DROP COLUMN IF EXISTS cust_add_no,
--     DROP COLUMN IF EXISTS cust_add_moo,
--     DROP COLUMN IF EXISTS cust_add_soi,
--     DROP COLUMN IF EXISTS cust_add_road,
--     DROP COLUMN IF EXISTS cust_add_tambon,
--     DROP COLUMN IF EXISTS cust_add_ampore,
--     DROP COLUMN IF EXISTS cust_add_province,
--     DROP COLUMN IF EXISTS cust_add_code,
--     DROP COLUMN IF EXISTS cust_add_fax,
--     DROP COLUMN IF EXISTS cust_work_name,
--     DROP COLUMN IF EXISTS cust_work_no,
--     DROP COLUMN IF EXISTS cust_work_moo,
--     DROP COLUMN IF EXISTS cust_work_soi,
--     DROP COLUMN IF EXISTS cust_work_road,
--     DROP COLUMN IF EXISTS cust_work_tambon,
--     DROP COLUMN IF EXISTS cust_work_ampore,
--     DROP COLUMN IF EXISTS cust_work_province,
--     DROP COLUMN IF EXISTS cust_work_code,
--     DROP COLUMN IF EXISTS cust_work_tel,
--     DROP COLUMN IF EXISTS cust_work_fax,
--     DROP COLUMN IF EXISTS cust_work_tax,
--     DROP COLUMN IF EXISTS cust_last_change,
--     DROP COLUMN IF EXISTS cust_contry,
--     DROP COLUMN IF EXISTS cust_price_over;
