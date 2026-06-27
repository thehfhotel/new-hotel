-- Migration: 059_create_ht_cash_ledger
-- Version: x.x.x
-- Date: 2026-06-27
--
-- Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — net-new feature parity with
-- iHOTEL's FrmPayMain / FrmAddPay (docs/legacy-app/FEATURE_MAP.md:296,535,560;
-- docs/adoption/feature-gap-audit.md "Cash In/Out Ledger"). Two canonical
-- tables:
--
--   * `ht_cash_ledger`     — per-line mirror of legacy `TB_Pay_History`
--                            (the general income/expense ledger).
--   * `ht_cash_categories` — mirror of the 3-level account taxonomy
--                            (`TB_SET_MyType2` / `TB_SET_MyType2_2` /
--                            `TB_SET_MyType3`) used to classify each entry.
--
-- The shift round report mirrors `HT_CheckIn_Pay` (canonical `ht_payment_ledger`,
-- migration 057) — that is room/folio payments, NOT petty cash, so this ledger
-- is genuinely separate and cannot be derived from it.
--
-- Site scoping is connection-level (these tables exist in both the `hotelnew`
-- and `hotelville` logical DBs; each site's `sync` worker populates its own DB)
-- — same model as `ht_shifts` / `ht_payment_ledger` (docs/architecture.md
-- canonical PG split). No site column.
--
-- Legacy shape (docs/legacy-app/SCHEMA.sql:777, COMPAT_CHEATSHEET.md §1051):
-- `TB_Pay_History` is a 10-col table — `id int` (NOT IDENTITY; allocated
-- app-side via get_id MAX+1), `Pay_Date float` (an OLE-Automation date serial,
-- NOT datetime — `DateTime.ToOADate()`), `Pay_Bill`, `Pay_Cust`, `Pay_Type`
-- (income/expense marker), `Pay_Total float`, `Pay_Note`, `Pay_Program float`
-- (a second OADate), `Pay_Group`, `Pay_Account` (the category-tree id_full
-- codes). The sync mapper converts both OADate floats back to TIMESTAMPTZ and
-- preserves the raw `Pay_Type` verbatim in `cash_legacy_type`.

-- UP MIGRATION

-- ── Account taxonomy (TB_SET_MyType2 / _2_2 / 3) ────────────────────────────
-- Each legacy tree table has the SAME shape (`id IDENTITY, id_full, name`); we
-- collapse all three into one table with a `cat_level` discriminator
-- ('2' | '2_2' | '3') so a single mapper + a single API read covers the tree.
-- `cat_legacy_id` is per-table IDENTITY, so the uniqueness key is the
-- (level, legacy id) pair.
CREATE TABLE IF NOT EXISTS ht_cash_categories (
    cash_cat_id   BIGSERIAL PRIMARY KEY,
    -- Which legacy tree this row came from: '2' = TB_SET_MyType2,
    -- '2_2' = TB_SET_MyType2_2, '3' = TB_SET_MyType3.
    cat_level     VARCHAR(8)   NOT NULL,
    -- The legacy IDENTITY id within that tree table (idempotent upsert key).
    cat_legacy_id INTEGER      NOT NULL,
    cat_id_full   VARCHAR(50),         -- id_full (the code stored in ledger Pay_Group/Pay_Account)
    cat_name      VARCHAR(100),        -- human label
    cat_synced_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_cash_categories_level_legacy_id_key UNIQUE (cat_level, cat_legacy_id)
);

CREATE INDEX IF NOT EXISTS ix_ht_cash_categories_level
    ON ht_cash_categories (cat_level);

COMMENT ON TABLE ht_cash_categories IS
    'Cash-ledger account taxonomy — canonical mirror of legacy TB_SET_MyType2 / '
    'TB_SET_MyType2_2 / TB_SET_MyType3 (3-level account tree), collapsed into one '
    'table via cat_level. Populated read-only by the sync worker''s '
    'sync_cash_categories per-tick poll. Per-site (connection-level scoping). '
    'Migration 059.';

-- ── The income/expense ledger (TB_Pay_History) ─────────────────────────────
CREATE TABLE IF NOT EXISTS ht_cash_ledger (
    cash_id          BIGSERIAL PRIMARY KEY,
    -- `TB_Pay_History.id` (legacy MAX+1 app-allocated id). The idempotent
    -- upsert key for the sync mirror. NULL for an entry created in our app
    -- that has not yet been written back to the legacy DB (PG allows multiple
    -- NULLs under a UNIQUE constraint). `cash_source` disambiguates.
    cash_legacy_id   INTEGER,
    -- Normalized direction. Derived in the mapper from the raw legacy
    -- `Pay_Type` (best-effort: contains 'รับ' → income, 'จ่าย' → expense,
    -- else 'unknown'); set directly for app-created rows. The raw legacy
    -- string is preserved verbatim in `cash_legacy_type` (authoritative).
    cash_kind        VARCHAR(20)  NOT NULL DEFAULT 'unknown'
                     CHECK (cash_kind IN ('income', 'expense', 'unknown')),
    cash_legacy_type VARCHAR(50),         -- Pay_Type  (raw legacy marker, verbatim)
    -- Pay_Date — legacy float OADate (days since 1899-12-30); the mapper
    -- converts the Thai-local wall-clock it encodes to a true UTC instant
    -- (same convention as ht_shifts / ht_payment_ledger). Range scans on the
    -- list endpoint compare against this column.
    cash_entry_date  TIMESTAMPTZ,
    cash_bill_no     VARCHAR(255),        -- Pay_Bill     (document / bill number)
    cash_payee       VARCHAR(500),        -- Pay_Cust     (payer / payee name)
    cash_amount      NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Pay_Total
    cash_note        VARCHAR(500),        -- Pay_Note
    -- Pay_Program — a second legacy OADate float of uncertain semantics
    -- (posting date?); converted to a UTC instant when present, else NULL.
    cash_program_date TIMESTAMPTZ,
    cash_group       VARCHAR(50),         -- Pay_Group    (account-tree id_full)
    cash_account     VARCHAR(50),         -- Pay_Account  (account-tree id_full)
    -- App-side provenance (no legacy column). 'legacy' = mirrored from
    -- TB_Pay_History by the sync poll; 'app' = created via POST /api/cash/*.
    cash_source      VARCHAR(20)  NOT NULL DEFAULT 'legacy'
                     CHECK (cash_source IN ('legacy', 'app')),
    cash_created_by  VARCHAR(100),        -- who entered it in our app (app rows only)
    cash_synced_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_cash_ledger_legacy_id_key UNIQUE (cash_legacy_id)
);

-- Date-range list (the hot path for the income/expense page).
CREATE INDEX IF NOT EXISTS ix_ht_cash_ledger_entry_date
    ON ht_cash_ledger (cash_entry_date);

COMMENT ON TABLE ht_cash_ledger IS
    'Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — canonical mirror of legacy '
    'TB_Pay_History. Populated by the sync worker''s sync_cash_history per-tick '
    'poll (cash_source=legacy) and by POST /api/cash/{income,expense} '
    '(cash_source=app). Writeback to TB_Pay_History is pending byte-shape '
    'verification (see writeback/recipes/cash_entry.rs). Per-site '
    '(connection-level scoping). Migration 059.';

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS ix_ht_cash_ledger_entry_date;
-- DROP TABLE IF EXISTS ht_cash_ledger;
-- DROP INDEX IF EXISTS ix_ht_cash_categories_level;
-- DROP TABLE IF EXISTS ht_cash_categories;
