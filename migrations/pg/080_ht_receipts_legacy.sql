-- Migration: 080_ht_receipts_legacy
-- Version: vNext
-- Date: 2026-07-28
-- Description: Per-PK ack cache for the Phase 6-A `payments` reconcile arm
--              (legacy `HT_Receipt_H` ↔ canonical `ht_payments`), plus the
--              supporting `ht_payments.pay_reference` index and the
--              `sync_status` row the arm reports into.
--
-- ## What the arm is
--
-- `scheduler/sync.rs::sync_payments` hashes every legacy receipt that carries
-- a `Receipt_ref` (the parent `Cin_no`) and compares it against the canonical
-- `ht_payments` row, recording divergence into `ht_reconcile_log` under
-- `table_name = 'payments'`, `legacy_pk = Receipt_no`.
--
-- Hashed inputs (`payment_canonical_hash`): `receipt_no`, amount at 2dp,
-- the void bit (legacy `status_name = 'ยกเลิก'` vs canonical `pay_voided`),
-- and `legacy_cin_no`. `pay_date` and `pay_method` are deliberately EXCLUDED —
-- the receipt UPSERT COALESCEs `pay_date` (so an app-originated row keeps its
-- own creation instant forever) and `pay_method` is defaulted to 'cash', never
-- mirrored. Hashing either would be permanent, unfixable false sync lag.
--
-- **SHIPPED DARK.** The arm only runs when `RECONCILE_PAYMENTS_ARM_ENABLED=true`
-- (compose default false on every service). With the flag off this table simply
-- stays empty; nothing reads or writes it. Rollout is Ville-first → 48h soak →
-- HF Hotel, in an announced window (the first enabled tick re-hashes every
-- historical receipt, so a backlog landing at once is expected).
--
-- ## Why an ack table at all
--
-- Same role as `ht_rooms_legacy` / `ht_customers_legacy` / `ht_checkins_legacy`
-- post-v2.63.0: a per-PK record of the last reconciled `mssql_hash` so the next
-- tick short-circuits a stable receipt instead of re-querying canonical for it.
-- Unlike those tables this one has NO data columns — it was born after the
-- Phase 5.5 cutover, so there is no pre-5.5 mirror to preserve and
-- `LEGACY_SYNC_RECONCILE_MODE=upsert` has nothing to write here (the arm is
-- diff-only by construction).
--
-- Cache-only: it NEVER holds canonical state. Truncating it is safe — the next
-- tick simply re-compares every receipt from scratch.
--
-- ## The `ht_payments.pay_reference` index
--
-- The arm's canonical probe mirrors `sync/mappers/payment.rs::apply_receipt_upsert`:
--
--     WHERE p.legacy_receipt_no = $1 OR p.pay_reference = $1
--
-- `legacy_receipt_no` already has a partial index (migration 030 / init-db);
-- `pay_reference` had none, so the OR degraded to a sequential scan per
-- non-acked receipt. On the first enabled tick that is one seq scan PER
-- receipt. The partial index lets PG plan a BitmapOr of two index scans.
-- Partial (`WHERE pay_reference IS NOT NULL`) to match the sibling index and
-- keep it off the rows that never carry a receipt number.
--
-- Strictly additive: new table, new index, one `sync_status` row. No existing
-- row is touched, no legacy DDL, no new legacy write (the arm READS legacy
-- only). Applies to both `hotelnew` and `hotelville` via `scripts/migrate.sh --site`.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_receipts_legacy (
    id          SERIAL PRIMARY KEY,
    receipt_no  VARCHAR(50) NOT NULL UNIQUE,
    sync_hash   VARCHAR(64),
    synced_at   TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ix_receipts_legacy_synced ON ht_receipts_legacy(synced_at);

COMMENT ON TABLE ht_receipts_legacy IS
  'Per-PK ack cache for the payments reconcile arm (Phase 6-A): the last '
  'mssql_hash scheduler/sync.rs::sync_payments reconciled for a legacy '
  'HT_Receipt_H.Receipt_no. Cache ONLY — never canonical state; truncating it '
  'just makes the next tick re-compare every receipt. No data columns by '
  'design (born after the Phase 5.5 mirror cutover; the arm is diff-only). '
  'Populated only while RECONCILE_PAYMENTS_ARM_ENABLED=true. Migration 080.';

-- Supports the arm's `legacy_receipt_no = $1 OR pay_reference = $1` probe —
-- see the header note. Partial, matching ix_ht_payments_legacy_receipt_no.
CREATE INDEX IF NOT EXISTS ix_ht_payments_pay_reference
    ON ht_payments (pay_reference) WHERE pay_reference IS NOT NULL;

-- `record_success` / `record_error` UPDATE this row by entity_type; without it
-- the arm's per-tick counters would silently no-op.
INSERT INTO sync_status (entity_type) VALUES ('payments')
ON CONFLICT (entity_type) DO NOTHING;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_ht_payments_pay_reference;
-- DROP TABLE IF EXISTS ht_receipts_legacy;
-- DELETE FROM sync_status WHERE entity_type = 'payments';
-- DELETE FROM schema_migrations WHERE version = '080';
