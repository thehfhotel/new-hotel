-- Migration: 084_reconcile_era_floor_id
-- Version: vNext
-- Date: 2026-08-01
-- Description: Give `ht_reconcile_era_floor` an ID basis (`era_floor_id
--              BIGINT`) alongside its existing TIMESTAMP basis, so the Phase
--              6-D payment-ledger probe can clamp its coverage floor to a
--              persisted, non-decreasing watermark the way the Phase 6-B
--              `guest_registry` arm already does.
--
-- ## The incident this closes (2026-07-30 → 2026-08-01, live)
--
-- `scheduler/payment_ledger_probe.rs` floors its legacy scan at
-- `PG_FLOOR_SQL = SELECT MIN(ledger_legacy_id) FROM ht_payment_ledger` — the
-- mirror's own coverage boundary, derived, never configured. That is only a
-- valid coverage boundary if coverage is an ID-CONTIGUOUS SUFFIX of the legacy
-- table. It is not, and a date-windowed backfill is what breaks the assumption:
--
--   * `backfill_payment_ledger --days=212` (run 2026-07-30) selects folios by
--     DATE and mirrors each one WHOLE, because `mirror_payment_ledger` DELETEs
--     a `Cin_No`'s lines and re-INSERTs the loader's current set. Coverage is
--     therefore "these folios", not "ids >= N".
--   * One monthly-billed long-stay at HF Ville (`CH25-000076`) carried a
--     payment line from 2025-08. Mirroring that folio whole dragged
--     `MIN(ledger_legacy_id)` from ~40470 back to 39113 — roughly seven months.
--   * The next probe tick's `HAVING MIN(id) >= floor` then admitted 404 legacy
--     folios the mirror had NEVER covered and reported every one of them as
--     `missing_pg`. Zero were genuine gaps.
--
-- Remediated the same day at the affected site by running Ville's `--all`
-- backfill so its coverage really is complete (Ville's floor is now 39014, the
-- absolute legacy minimum). HF Hotel's `--all` has NOT run yet — it is
-- scheduled for the night of 2026-08-01, and the probe stays dark there until
-- it has. This migration removes the CLASS: a floor that can only ratchet
-- FORWARD cannot be dragged back by a later whole-folio mirror of a
-- pre-coverage row.
--
-- ## Enable-order constraint the ratchet creates
--
-- Monotonicity makes the FIRST enabled tick's reading durable, so seeding it
-- from narrow coverage is its own failure mode. Do not enable
-- `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` at a site before that site's
-- `backfill_payment_ledger --all` has completed: a floor seeded from a
-- date-windowed window keeps holding after a later `--all` widens coverage,
-- permanently excluding the folios that backfill lands. If that happens, the
-- only way down is to delete the row —
--   DELETE FROM ht_reconcile_era_floor WHERE table_name = 'payment_ledger_probe';
-- on that site's canonical database; the next tick re-derives it. A hand
-- UPDATE can only move a floor FORWARD (the upsert clamps with GREATEST). A
-- watermark that holds for ~1h raises the `era_floor_held:` Slack alert.
--
-- ## Why a new column instead of reusing `era_floor`
--
-- `era_floor` is `TIMESTAMP`. The obvious "just switch the probe to a date
-- basis" is the WRONG fix: `ht_payment_ledger.ledger_pay_date` is `TIMESTAMPTZ`
-- and the legacy side stores naive local Thai time, so a date-based floor
-- reintroduces exactly the Thai→UTC boundary reasoning the integer IDENTITY
-- basis was chosen to avoid (module docs, `PG_FLOOR_SQL`). The two bases
-- coexist in one table, one column each, one row per arm:
--
--   | table_name              | era_floor           | era_floor_id |
--   |-------------------------|---------------------|--------------|
--   | `guest_registry`        | 2026-05-13 00:00:00 | NULL         |
--   | `payment_ledger_probe`  | NULL                | 39014        |
--
-- The row identity is the arm's own key — the SAME literal it reports under in
-- `ht_reconcile_log.table_name` and `sync_status.entity_type` — so one operator
-- query joins all three and the two arms can never collide on the PK.
--
-- ## `era_floor` becomes NULLABLE
--
-- An ID-basis arm has no meaningful timestamp to write, and a sentinel
-- (epoch, `-infinity`) would be a lie an operator could read as a real floor.
-- Dropping the NOT NULL is safe for the existing arm: `guest_registry` writes
-- `era_floor` on every tick through the GREATEST upsert and never writes NULL,
-- and it reads its own row by key. The CHECK below keeps a row from being
-- entirely floorless in BOTH bases.
--
-- Idempotent: ADD COLUMN IF NOT EXISTS; DROP NOT NULL is a no-op when already
-- dropped; the CHECK is guarded by an existence probe in `pg_constraint`
-- (the 048 idiom — `ADD CONSTRAINT IF NOT EXISTS` is not available).
--
-- STRICTLY ADDITIVE. No legacy DDL, no legacy write, no data change: the
-- payment-ledger probe seeds its own watermark from the derived
-- `MIN(ledger_legacy_id)` on its first tick after deploy, which is the honest
-- absolute minimum at Ville today and at HF Hotel after the `--all` backfill.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_reconcile_era_floor
    ADD COLUMN IF NOT EXISTS era_floor_id BIGINT;

-- An ID-basis arm has no timestamp to write. See the header.
ALTER TABLE ht_reconcile_era_floor
    ALTER COLUMN era_floor DROP NOT NULL;

-- A row must carry a floor in at least ONE basis; a row with neither is a
-- write bug (an arm that upserted a key without a value), and silently
-- admitting it would read downstream as "no coverage floor" — the unbounded
-- scan this table exists to prevent.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
         WHERE conname = 'ck_ht_reconcile_era_floor_basis'
    ) THEN
        ALTER TABLE ht_reconcile_era_floor
            ADD CONSTRAINT ck_ht_reconcile_era_floor_basis
            CHECK (era_floor IS NOT NULL OR era_floor_id IS NOT NULL);
    END IF;
END
$$;

COMMENT ON COLUMN ht_reconcile_era_floor.era_floor IS
  'TIMESTAMP-basis floor: the oldest PARENT timestamp the arm will admit. Used '
  'by arms whose coverage boundary is a date on a canonical column compared '
  'against that same column (no cross-DB clock). NULL for an ID-basis arm. '
  'First arm: guest_registry (migration 081).';

COMMENT ON COLUMN ht_reconcile_era_floor.era_floor_id IS
  'ID-basis floor: the lowest legacy IDENTITY the arm will admit. Used by arms '
  'whose mirror is keyed on an integer legacy id, where a date basis would '
  'have to cross the naive-Thai/TIMESTAMPTZ boundary. NULL for a '
  'TIMESTAMP-basis arm. Ratcheted FORWARD only, by the same GREATEST upsert as '
  'era_floor. First arm: payment_ledger_probe (migration 084) — a raw '
  'MIN(ledger_legacy_id) is only a valid coverage boundary while coverage is an '
  'id-contiguous SUFFIX, and a date-windowed backfill that mirrors WHOLE folios '
  'violates that (2026-07-30: one 2025-08 line on a monthly-billed long-stay '
  'dragged the floor back ~7 months and swept 404 never-mirrored folios into '
  'the scan as missing_pg).';

COMMENT ON TABLE ht_reconcile_era_floor IS
  'Per-reconcile-arm scope watermark: the oldest row an arm will admit, in '
  'whichever basis that arm uses — era_floor (TIMESTAMP, parent time) or '
  'era_floor_id (BIGINT, legacy IDENTITY). Exactly one basis per arm; the row '
  'key is the arm''s own ht_reconcile_log.table_name / sync_status.entity_type '
  'literal. Written ONLY through a GREATEST upsert, so it is monotonically '
  'non-decreasing and an arm''s scope can only ever narrow — a derived MIN() is '
  'a LOW-water mark, and one pre-coverage row acquiring a mirrored counterpart '
  'must not drag it backwards and expand the scan by years. Operators may move '
  'a floor FORWARD by hand; the clamp makes that stick. Deleting a row is safe '
  'but re-derives the floor from live data on the next tick. updated_at is '
  'touched by every tick that confirms a floor, not only by the ticks that '
  'raise one — the clamp and the read are one statement so the arm can never '
  'act on a stale value. Arms: guest_registry (081, TIMESTAMP), '
  'payment_ledger_probe (084, BIGINT).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- Restoring NOT NULL requires the ID-basis rows to be gone first, or the
-- ALTER fails on them — hence the DELETE ahead of it.
--
-- ALTER TABLE ht_reconcile_era_floor DROP CONSTRAINT IF EXISTS ck_ht_reconcile_era_floor_basis;
-- DELETE FROM ht_reconcile_era_floor WHERE era_floor IS NULL;
-- ALTER TABLE ht_reconcile_era_floor DROP COLUMN IF EXISTS era_floor_id;
-- ALTER TABLE ht_reconcile_era_floor ALTER COLUMN era_floor SET NOT NULL;
-- DELETE FROM schema_migrations WHERE version = '084';
