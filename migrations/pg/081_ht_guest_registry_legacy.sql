-- Migration: 081_ht_guest_registry_legacy
-- Version: vNext
-- Date: 2026-07-28
-- Description: Per-folio ack cache for the Phase 6-B `guest_registry`
--              reconcile arm (legacy `HT_CheckIn_Other_People` ↔ canonical
--              `ht_guest_registry`), plus the `sync_status` row the arm
--              reports into.
--
-- ## What the arm is
--
-- `scheduler/sync.rs::sync_guest_registry` compares the companion SET of each
-- check-in — the FOLIO — against canonical, recording divergence into
-- `ht_reconcile_log` under `table_name = 'guest_registry'`,
-- `legacy_pk = Cin_no`.
--
-- **The folio, not the row, is the unit — and that is the whole design.**
-- iHOTEL edits companions by DELETE-then-REINSERT (`FrmCheckIn.cs:9975`), so
-- every edit mints a fresh `HT_CheckIn_Other_People.id` and the CT mapper
-- faithfully mints a fresh canonical row. An arm keyed on that id would report
-- TWO divergences on every correctly-applied edit: the retired id (which can
-- never converge) and the new one. Hashing the whole companion set of a
-- `Cin_no` is invariant under that churn and moves only when the companion
-- CONTENT does.
--
-- Hashed body: `cin_no | <sorted "{name}|{country}" lines joined by \n>`. Ids
-- on both sides are excluded. The canonical name is re-concatenated as
-- `first [+ ' ' + last]` using the SAME SQL expression the CT mapper's
-- echo-adoption match uses (`sync/mappers/guest_registry.rs`,
-- `canonical_companion_name_sql!`) — if those diverged, a companion OUR app
-- created and the writeback echoed back would be adopted correctly by the
-- mapper yet hash differently here, forever.
--
-- **SHIPPED DARK.** The arm only runs when
-- `RECONCILE_GUEST_REGISTRY_ARM_ENABLED=true` (compose default false on every
-- service). With the flag off this table simply stays empty; nothing reads or
-- writes it. Rollout is Ville-first → 48h soak → HF Hotel, in an announced
-- window.
--
-- ## Scope — the canonical-era floor
--
-- `ht_guest_registry` is CT-populated with no historical backfill (Track E1
-- enabled CT on `HT_CheckIn_Other_People` in May 2026), while `ht_checkins` IS
-- backfilled to 2021. So every pre-CT folio has a canonical parent check-in and
-- no canonical companions. The arm therefore scans only folios whose parent
-- check-in is at or after the mirror's own coverage floor, derived (never
-- configured) from `MIN(ht_checkins.cin_checkin_time)` over check-ins that
-- carry a mirrored companion.
--
-- Live counts 2026-07-28 — HF Hotel: 20,434 legacy companion rows / 20,423
-- folios vs 819 canonical companions (oldest parent check-in 2026-05-13);
-- HF Ville: 2,185 / 2,184 vs 545 (oldest 2026-05-13). Unfloored the first
-- enabled tick would open ~19.6k + ~1.6k rows that can NEVER close, re-log them
-- every tick, pin the 4h digest and the >72h escalation tier on
-- `guest_registry`, and starve every other entity out of the auto-resolve
-- sweep's age-only 500-row batch. Floored, the same data yields 830 in-era
-- legacy folios vs 818 canonical at HF Hotel (~12 actionable finds) and 574 vs
-- 545 at Ville (~29).
--
-- "Mirrored" means STAMPED (`guest_legacy_id IS NOT NULL`). Canonical also
-- holds non-primary companions with no legacy counterpart at all — the
-- check-in guest endpoint and the migration-070 registration capture write
-- them, and `TM30_COMPANION_WRITEBACK_ENABLED` is compose-default false — and
-- counting those as coverage would claim an era the mirror never covered.
--
-- ## Why the floor is PERSISTED (`ht_reconcile_era_floor`)
--
-- A raw `MIN()` is a low-water mark on the PARENT's check-in time, and ONE row
-- can drag it backwards by years: iHOTEL edits companions by DELETE+REINSERT,
-- the CT mapper resolves the parent by `legacy_cin_no` with no era restriction,
-- and `ht_checkins` is backfilled to 2021 — so a companion edit on a 2023 folio
-- mirrors one canonical row whose parent check-in is 2023. That would move the
-- floor to 2023, admit ~all 20,423 legacy folios instead of 830, and make the
-- next single tick enqueue ~19.6k permanently-open rows: exactly the flood the
-- floor exists to prevent. `ht_reconcile_era_floor` clamps the derived value to
-- a non-decreasing watermark (`GREATEST` in the upsert, so the guarantee holds
-- even with the backend scheduler and `bin/sync` ticking the same database), so
-- scope can only ever NARROW. An operator can move a floor FORWARD by hand and
-- the clamp makes the edit stick; that is the documented remedy if a bootstrap
-- reading ever comes out too low.
--
-- Second belt, in code not schema: `scheduler/sync.rs` compares both sides
-- fully in memory and ABORTS the tick — writing nothing at all, no reconcile
-- rows and no ack rows — if it would enqueue more than
-- `RECONCILE_GUEST_REGISTRY_MAX_DIVERGENCES` (default 500 = the auto-resolve
-- sweep's own per-tick LIMIT) findings, paging under the cooldown key
-- `reconcile_cap:guest_registry` instead.
--
-- ## Why an ack table at all
--
-- Same role as `ht_rooms_legacy` / `ht_customers_legacy` / `ht_checkins_legacy`
-- post-v2.63.0: a per-PK record of the last reconciled `mssql_hash`. But note
-- the deliberate difference — in THIS arm the ack suppresses WRITES only, it
-- never gates detection. Both sides are already resident in memory (two batched
-- PG reads + one bulk MSSQL scan), so short-circuiting the comparison on the
-- ack would buy nothing and would blind the arm to any canonical-side change
-- that leaves the legacy hash untouched (a companion deleted from
-- `ht_guest_registry`, a dropped CT delete) — on a table that exists to satisfy
-- a legal reporting obligation (TM.30 foreign-guest registration).
--
-- No data columns: like `ht_receipts_legacy` (migration 080) this table was
-- born after the Phase 5.5 cutover, so `LEGACY_SYNC_RECONCILE_MODE=upsert` has
-- nothing to write here and the arm is diff-only by construction.
--
-- Cache-only: it NEVER holds canonical state. Truncating it is safe — the next
-- tick simply re-writes an ack row per folio.
--
-- ## No new index
--
-- Every lookup the arm performs is already covered: `ix_ht_guestreg_checkin`
-- on `ht_guest_registry(guest_cin_id)`, `ix_ht_checkins_legacy_cin_no` on
-- `ht_checkins(legacy_cin_no)` and `ix_ht_checkins_checkin` on
-- `ht_checkins(cin_checkin_time)`.
--
-- Strictly additive: two new tables, one `sync_status` row. No existing row is
-- touched, no legacy DDL, no new legacy write (the arm READS legacy only).
-- Applies to both `hotelnew` and `hotelville` via `scripts/migrate.sh --site`.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_guest_registry_legacy (
    id         SERIAL PRIMARY KEY,
    cin_no     VARCHAR(50) NOT NULL UNIQUE,
    sync_hash  VARCHAR(64),
    synced_at  TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ix_guest_registry_legacy_synced
    ON ht_guest_registry_legacy(synced_at);

COMMENT ON TABLE ht_guest_registry_legacy IS
  'Per-FOLIO ack cache for the guest-registry reconcile arm (Phase 6-B): the '
  'last mssql_hash scheduler/sync.rs::sync_guest_registry reconciled for the '
  'companion set of a legacy Cin_no. Cache ONLY — never canonical state; '
  'truncating it just re-writes one ack row per folio next tick. The ack '
  'suppresses WRITES only and never gates detection, so a canonical-side '
  'change that leaves the legacy hash untouched stays observable. No data '
  'columns by design (born after the Phase 5.5 mirror cutover; the arm is '
  'diff-only). Populated only while RECONCILE_GUEST_REGISTRY_ARM_ENABLED=true. '
  'Migration 081.';

-- Durable, non-decreasing scope floor per reconcile arm. Seeded lazily by the
-- arm itself on its first tick (nothing to insert here — the correct value is
-- whatever the mirror's own coverage is at that moment), then only ever moved
-- FORWARD by the GREATEST clamp in
-- `scheduler/sync.rs::RECONCILE_ERA_FLOOR_UPSERT_SQL`.
CREATE TABLE IF NOT EXISTS ht_reconcile_era_floor (
    table_name  VARCHAR(50)  PRIMARY KEY,
    era_floor   TIMESTAMP    NOT NULL,
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE ht_reconcile_era_floor IS
  'Per-reconcile-arm scope watermark: the oldest parent timestamp an arm will '
  'admit. Written ONLY through a GREATEST upsert, so it is monotonically '
  'non-decreasing and an arm''s scope can only ever narrow — one historical row '
  'gaining a mirrored counterpart must not drag a derived MIN() floor backwards '
  'and expand the scan by years. Operators may move a floor FORWARD by hand; '
  'the clamp makes that stick. Deleting a row is safe but re-derives the floor '
  'from live data on the next tick. updated_at is touched by every tick that '
  'confirms a floor, not only by the ticks that raise one — the clamp and the '
  'read are one statement so the arm can never act on a stale value. First arm: '
  'guest_registry (migration 081).';

-- `record_success` / `record_error` UPDATE this row by entity_type; without it
-- the arm's per-tick counters would silently no-op.
INSERT INTO sync_status (entity_type) VALUES ('guest_registry')
ON CONFLICT (entity_type) DO NOTHING;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP TABLE IF EXISTS ht_guest_registry_legacy;
-- DROP TABLE IF EXISTS ht_reconcile_era_floor;
-- DELETE FROM sync_status WHERE entity_type = 'guest_registry';
-- DELETE FROM schema_migrations WHERE version = '081';
