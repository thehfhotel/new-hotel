-- Migration: 042_create_ht_rate_tiers
-- Version: 2.63.16
-- Date: 2026-05-13
-- Track F4 / T1 CRIT-4 (`docs/coexistence/audit-2026-05-13.md`) —
-- introduce the canonical `(Room_Type, Cust_Type)` rate matrix.
--
-- ## Background
--
-- The legacy iHOTEL prices rooms via `HT_Rooms_Price`, a
-- `(Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M)`
-- matrix where `Room_CustType` is a pricing tier label (e.g. `ราคาปกติ`,
-- corporate, walk-in). The existing PG `ht_rates` table modeled the axis
-- as `weekday/weekend/special` — completely orthogonal to the legacy
-- shape. T1 CRIT-4 flagged this as a structural mismatch that prevented
-- the canonical write path from re-priced any non-standard customer
-- tier without surgery.
--
-- This migration introduces `ht_rate_tiers`, keyed exactly on
-- `(rate_tier_room_type, rate_tier_cust_type)`, mirroring the legacy
-- composite. `Room_Price` (per-night), `Room_Price_H` (hourly extension),
-- and `Room_Price_M` (monthly) all land here so callers can pull the
-- right column for the booking shape (overnight / hourly / long-stay).
--
-- ## Scope of F4
--
-- F4 lands the table + sync mapper + read path. Writeback stays with
-- iHOTEL (pricing edits go through the legacy app today). `HT_ContinueTime`
-- and `HT_Order_Up/Down` (hourly + monthly variants the legacy app
-- composes on top of `HT_Rooms_Price`) are deferred to Track G's
-- hourly-stay / monthly-tier features.
--
-- ## `ht_rates` deprecation
--
-- `ht_rates` is NOT dropped here — frontend callers and
-- `routes/new_rates.rs` (now redirected to `ht_rate_tiers`) need to be
-- audited first. A follow-on migration removes it once we're sure no
-- reader remains.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_rate_tiers (
    rate_tier_id            BIGSERIAL       PRIMARY KEY,
    rate_tier_room_type     VARCHAR(100)    NOT NULL,
    rate_tier_cust_type     VARCHAR(100)    NOT NULL,
    rate_tier_price         NUMERIC(12, 2)  NOT NULL,
    rate_tier_price_hourly  NUMERIC(12, 2),
    rate_tier_price_monthly NUMERIC(12, 2),
    rate_tier_legacy_id     INTEGER,
    rate_tier_active        BOOLEAN         NOT NULL DEFAULT TRUE,
    rate_tier_created_at    TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    rate_tier_updated_at    TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_ht_rate_tiers_room_cust UNIQUE (rate_tier_room_type, rate_tier_cust_type)
);

CREATE INDEX IF NOT EXISTS ix_ht_rate_tiers_room_type
    ON ht_rate_tiers (rate_tier_room_type);

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_rate_tiers_room_type;
-- DROP TABLE IF EXISTS ht_rate_tiers;
