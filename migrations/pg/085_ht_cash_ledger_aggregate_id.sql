-- Migration: 085_ht_cash_ledger_aggregate_id
-- Version: vNext
-- Date: 2026-08-10
-- Description: Add `aggregate_id UUID` (nullable, partial UNIQUE) to
--              `ht_cash_ledger` — the writeback correlation key
--              `back_populate_legacy_ids` needs to find "this app-originated
--              row" after the legacy `TB_Pay_History` INSERT commits.
--
-- ## Issue #202 — the precondition for ever enabling cash-outbound writeback
--
-- `WritebackIntent::CreateCashEntry` (this release) allocates
-- `TB_Pay_History.id` (MAX+1 TABLOCKX — app-side, not IDENTITY) and the
-- worker's `back_populate_legacy_ids` must stamp it onto the canonical
-- `ht_cash_ledger` row's `cash_legacy_id` so the inbound `sync_cash_history`
-- poll's `ON CONFLICT (cash_legacy_id)` UPSERT recognizes our own write on
-- re-import instead of inserting a duplicate (`bin/sync.rs::
-- CASH_HISTORY_UPSERT_SQL`). Every other back-population arm that keys off
-- "the row this job is about" does so via `WHERE aggregate_id = $1`
-- (`ht_bookings`, `ht_checkins`, `ht_payments`, `ht_coupons`, `ht_notes` —
-- migrations 014/030/051/062) — `ht_cash_ledger` (migration 059) never got
-- this column because outbound writeback wasn't wired yet. This migration
-- closes exactly that gap, following the `ht_notes` (062) precedent:
-- nullable, v4-minted by the future create-handler, partial UNIQUE index so
-- multiple NULLs (legacy-mirrored rows, and app rows pre-dating this
-- migration) coexist under the constraint.
--
-- ## What this does NOT do
--
-- Nothing writes `aggregate_id` yet — no service/route call site enqueues
-- `CreateCashEntry` (see `writeback::recipes::cash_entry`'s module doc).
-- Existing rows (both `cash_source` values) get NULL, which is correct: a
-- legacy-mirrored row has no writeback of its own to correlate, and no
-- app row has been minted with a UUID before this column existed.
--
-- STRICTLY ADDITIVE. No legacy DDL, no legacy write, no data change.
-- Applies to both `hotelnew` and `hotelville` via `scripts/migrate.sh --site`
-- (per-site connection-level scoping, same model as the rest of
-- `ht_cash_ledger` — no site column).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_cash_ledger
    ADD COLUMN IF NOT EXISTS aggregate_id UUID;

CREATE UNIQUE INDEX IF NOT EXISTS ix_ht_cash_ledger_aggregate_id
    ON ht_cash_ledger (aggregate_id) WHERE aggregate_id IS NOT NULL;

COMMENT ON COLUMN ht_cash_ledger.aggregate_id IS
  'Writeback correlation id (v4, minted at INSERT for app rows) — the '
  'back-population WHERE key `back_populate_legacy_ids` uses to stamp '
  '`cash_legacy_id` after a CreateCashEntry writeback commits (issue #202). '
  'NULL for legacy-mirrored rows and for app rows created before this '
  'migration. Same role as ht_coupons.aggregate_id / ht_notes.aggregate_id. '
  'Migration 085.';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_ht_cash_ledger_aggregate_id;
-- ALTER TABLE ht_cash_ledger DROP COLUMN IF EXISTS aggregate_id;
-- DELETE FROM schema_migrations WHERE version = '085';
