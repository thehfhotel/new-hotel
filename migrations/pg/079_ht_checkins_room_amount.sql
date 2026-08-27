-- Migration: 079_ht_checkins_room_amount
-- Version: vNext
-- Date: 2026-07-28
-- Description: Persist the ROOM-ONLY half of the legacy folio total on
--              `ht_checkins`, so the server-authoritative checkout folio
--              (`routes::new_checkins::folio_breakdown`) has an exact room
--              basis instead of re-using the product-INCLUSIVE net.
--
-- ## The bug this closes
--
-- `ht_checkins.cin_total_amount` mirrors legacy `HT_CheckIn_H.Total_Price_Net`
-- (`sync/mappers/checkin.rs::project_aggregate`), which is **Room + Product by
-- definition** — iHOTEL rewrites the whole `Total_Price_*` family on EVERY
-- payment/sale change (`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_H`" "`Total_Price_*`: aggregated totals; old app updates these on every payment/sale change."
-- — was a bare line-range citation (359-362) that had drifted onto
-- `HT_Room_Status`), so by
-- the time a folio reaches checkout its `Total_Price_Net` already folds in any
-- POS line iHOTEL knows about.
--
-- `folio_breakdown` computed `net_total = cin_total_amount + product_total`,
-- i.e. it re-added POS lines that were already inside the room basis —
-- a DOUBLE-COUNT. And because `room_price_total` is threaded into
-- `CheckOutCommand` → `writeback/recipes/checkout.rs`, the inflated number
-- would be stamped back into the SHARED legacy DB as `Total_Price_Room`,
-- which the next Change-Tracking tick reads straight back into
-- `cin_total_amount` — a self-reinforcing corruption loop.
--
-- Not reachable in production today (`ht_pos_sales` and `ht_products` are both
-- empty on BOTH sites, so `product_total` is always 0.00), but
-- `CHECKOUT_SERVER_TOTAL_ENABLED=true` is live, so the first POS row would have
-- charged real guests wrong.
--
-- ## The fix (Route A — persist the split)
--
-- Mirror legacy `HT_CheckIn_H.Total_Price_Room` into its own canonical column
-- and use THAT as the room basis:
--
--     room_total = cin_room_amount
--     net_total  = room_total + product_total
--
-- Exact and state-free: a POS line iHOTEL has already folded is excluded from
-- the room basis by construction (it lives in `Total_Price_Product`, not
-- `Total_Price_Room`); a line we originated that iHOTEL has not folded yet is
-- added via `product_total`. Every line is counted exactly once regardless of
-- which app originated it or how far the sync has progressed.
--
-- ## Nullable on purpose — NULL is load-bearing
--
-- NO `DEFAULT 0`. `cin_room_amount IS NULL` means "the sync has never projected
-- this folio's `Total_Price_Room`" (rows that predate this migration and have
-- not had a CT event since; app-originated check-ins before their first
-- read-back tick; historical checked-out folios that may never get another CT
-- event). The read path treats NULL as "fall back to `cin_total_amount`" — i.e.
-- exactly today's behaviour, which is CORRECT whenever no POS line exists, and
-- that is every live folio.
--
-- A `DEFAULT 0` would be actively dangerous: it makes "never projected"
-- indistinguishable from "legitimately a zero room charge", and the read path
-- would then zero out the room charge on every historical folio — a
-- catastrophic UNDERcharge on a live money path. `Some(0.00)` must keep meaning
-- a genuine zero, because legacy `Total_Price_Room` is `float NOT NULL
-- DEFAULT 0` (`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_H`" "Total_Price_Room float NOT NULL DEFAULT 0,"
-- — was a bare line-375 citation that had drifted onto `HT_Room_Status`)
-- and a product-only folio legitimately has
-- a zero room component.
--
-- ## Strictly additive
--
-- Nullable, no default, no backfill, no index. Existing rows are untouched and
-- keep today's behaviour via the NULL fallback; each folio self-heals to the
-- exact basis on its next CT tick (the check-in gate `existing_matches` now
-- compares the column, so a `Total_Price_Room`-only edit in iHOTEL re-applies
-- instead of being idempotency-skipped).
--
-- READ-ONLY with respect to legacy: `Total_Price_Room` is ALREADY in the
-- `HT_CheckIn_H` projection (`sync/parent_loader.rs::CHECKIN_H_PROJECTION`),
-- so there is no legacy DDL, no new CT table, and no `migrations/legacy-mssql/`
-- prerequisite. This migration triggers NO new legacy write; the checkout
-- writeback already wrote `Total_Price_Room` — it just now writes the CORRECT
-- value.
--
-- ALTER-only (no new table → no new CARDINALITY_MAP.md row; annotates the
-- existing `ht_checkins` row's note instead).
-- Applies to both `hotelnew` and `hotelville` via `scripts/migrate.sh --site`.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_checkins
    ADD COLUMN IF NOT EXISTS cin_room_amount DECIMAL(12,2);

COMMENT ON COLUMN ht_checkins.cin_room_amount IS
  'Room-only folio total, mirrored from legacy HT_CheckIn_H.Total_Price_Room by '
  'sync/mappers/checkin.rs (hard overwrite, no COALESCE — same write semantics '
  'as cin_total_amount). Distinct from cin_total_amount, which mirrors '
  'Total_Price_Net = Room + Product. Used as the ROOM BASIS by '
  'routes::new_checkins::folio_breakdown so net_total = room + product counts '
  'each POS line exactly once. NULL = never projected (pre-079 rows, or an '
  'app-originated check-in before its first CT read-back) → the read path falls '
  'back to cin_total_amount, which is exact whenever no POS line exists. NULL '
  'is deliberately NOT 0: 0 is a legitimate room charge on a product-only '
  'folio. Migration 079.';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- ALTER TABLE ht_checkins DROP COLUMN IF EXISTS cin_room_amount;
-- DELETE FROM schema_migrations WHERE version = '079';
