-- Migration: 057_create_ht_payment_ledger
-- Version: x.x.x
-- Date: 2026-06-27
--
-- Track J7a — canonical per-line mirror of legacy `HT_CheckIn_Pay` so the
-- round-close reconciliation (income by tender) and the iHOTEL-equivalent
-- shift report can be computed from canonical PG WITHOUT touching the legacy
-- DB from the API layer (docs/architecture.md "critical rule").
--
-- WHY a new table (not `ht_payments`): `ht_payments` is single-tender
-- (`pay_amount` + one `pay_method`) and the legacy→canonical payment sync only
-- refreshes a check-in's *paid total* (`ht_checkins.cin_paid_amount`) — it does
-- NOT carry the per-tender split (cash/credit/transfer/free/web). iHOTEL's
-- ReportShipCash sums those split columns from `HT_CheckIn_Pay`. This ledger
-- mirrors that ledger per-line so the report sees BOTH apps' payments (our
-- app's payments also write `HT_CheckIn_Pay` via the writeback recipe and
-- CT-sync back), with no double counting.
--
-- Site scoping is connection-level (this table exists in both `hotelnew` and
-- `hotelville` logical DBs; each site's `sync` worker populates its own DB) —
-- same model as `ht_shifts` (see docs/architecture.md canonical PG split). No
-- site column.
--
-- `HT_CheckIn_Pay` is a LINE-ITEM ledger: one row per charge line, each with
-- its own tender split, sharing a `Pay_no` across lines of one payment.
-- `Cin_Pay_Ds_Price = cash + credit + free + tran + web` per line (legacy
-- invariant). Cancellation is a status flip (`Cin_Status='ยกเลิก'`), not a
-- delete — we mirror status so the report can exclude cancelled lines.

-- UP MIGRATION
CREATE TABLE IF NOT EXISTS ht_payment_ledger (
    ledger_id        BIGSERIAL PRIMARY KEY,
    -- `HT_CheckIn_Pay.id` (IDENTITY on the legacy side) — the idempotent
    -- upsert key. UNIQUE within this logical DB (one site's legacy server).
    ledger_legacy_id INTEGER     NOT NULL,
    ledger_pay_no    VARCHAR(50),          -- Pay_no  (R{yyMM}-{4digit}; shared across a payment's lines)
    ledger_cin_no    VARCHAR(50),          -- Cin_No  (check-in / receipt / bill / book id the line belongs to)
    ledger_cust_no   VARCHAR(50),          -- Cin_Cust_no
    -- Line descriptor + category. `ledger_ds_id` is the product code
    -- (`'P001'` = room-rent line) used for the sales-by-category section.
    ledger_ds_label  VARCHAR(500),         -- Cin_Pay_Ds       (short label, often room no)
    ledger_ds_name   VARCHAR(500),         -- Cin_Pay_Ds_Name  (human description, e.g. ค่าห้อง)
    ledger_ds_id     VARCHAR(50),          -- Cin_Pay_Ds_ID    (category code; P001 = room)
    ledger_ds_num    NUMERIC(12,2),        -- Cin_Pay_Ds_Num   (qty / nights)
    -- Tender + amount columns are NUMERIC(14,2) (not 12,2) for headroom: a
    -- corrupt/garbage legacy float that overflowed (12,2)'s ~10^10 ceiling
    -- would error the cast and POISON the shared sync batch tx (wedging the
    -- whole payment tick — review J7a P2). The `mirror_payment_ledger`
    -- projection additionally coerces non-finite (NaN/±Inf) tenders to 0.
    -- Split tenders (baht). Can be NEGATIVE — refunds use negation.
    ledger_cash      NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_Cash
    ledger_credit    NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_Credit (card)
    ledger_free      NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_Free   (discount/comp)
    ledger_tran      NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_Tran   (bank transfer)
    ledger_web       NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_web    (online)
    ledger_amount    NUMERIC(14,2) NOT NULL DEFAULT 0,  -- Cin_Pay_Ds_Price (line total = sum of tenders)
    ledger_status    VARCHAR(50)   NOT NULL DEFAULT '1',-- Cin_Status ('1' active / 'ยกเลิก' cancelled)
    ledger_branch    VARCHAR(50),          -- Branch  (cashier-selected branch label)
    ledger_pay_by    VARCHAR(100),         -- Pay_by  (cashier login)
    ledger_note      VARCHAR(500),         -- Cin_Pay_Note
    -- Cin_Pay_Date — legacy datetime is Thai local (tz-naive); the mapper
    -- converts to a true UTC instant for this TIMESTAMPTZ (same convention as
    -- ht_shifts / the checkin mapper). Round-window scans compare against
    -- ht_shifts.shift_opened_at..shift_closed_at (also TIMESTAMPTZ).
    ledger_pay_date  TIMESTAMPTZ,
    ledger_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_payment_ledger_legacy_id_key UNIQUE (ledger_legacy_id)
);

-- Round-window range scan (the hot path for the close summary / report).
CREATE INDEX IF NOT EXISTS ix_ht_payment_ledger_pay_date
    ON ht_payment_ledger (ledger_pay_date);
-- Replace-for-check-in upkeep (the mapper re-mirrors all lines of a changed Cin_No).
CREATE INDEX IF NOT EXISTS ix_ht_payment_ledger_cin_no
    ON ht_payment_ledger (ledger_cin_no);

COMMENT ON TABLE ht_payment_ledger IS
    'Track J7a — canonical per-line mirror of legacy HT_CheckIn_Pay (all tender '
    'splits + category + status), populated by the sync worker''s PaymentMapper '
    'coalesced path. Source for round-close income-by-tender reconciliation and '
    'the iHOTEL-equivalent shift report. Per-site (connection-level scoping).';

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS ix_ht_payment_ledger_cin_no;
-- DROP INDEX IF EXISTS ix_ht_payment_ledger_pay_date;
-- DROP TABLE IF EXISTS ht_payment_ledger;
