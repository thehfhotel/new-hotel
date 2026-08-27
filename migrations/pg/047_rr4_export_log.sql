-- Migration: 047_rr4_export_log
-- Version: 2.64.0
-- Date: 2026-05-14
-- Track G8 (audit 2026-05-13 T4 CRIT-2) — RR.4 Thai immigration export
-- audit trail.
--
-- Thai law (RR.4 / ตม.30) requires every hotel to file foreign-guest
-- check-ins with the district immigration office on a fixed schedule.
-- The legacy iHOTEL app produces the report via `FrmReportRR4`
-- (Crystal Reports), bound to `Datalocal.ReportRR4` and built from
-- `HT_CheckIn_H` + `HT_CheckIn_Other_People` + `HT_Customers` +
-- `TB_SETTINGS` — see `docs/legacy-app/REPORTS_INVENTORY.md` §"3.9 Government".
--
-- Track G8 closes the standalone-readiness gap (audit T4 CRIT-2): we
-- emit the report from the canonical `ht_guest_registry` +
-- `ht_checkins` + `ht_customers` + `ht_rooms_new` join. The regulator
-- may re-request previously filed periods, so we need a durable audit
-- trail of every export attempt (range, format, row count, SHA-256
-- of the emitted bytes, and the operator who pressed the button).
--
-- Column inventory:
--
-- * `id`            — SERIAL primary key.
-- * `site`          — `'hfhotel'` / `'hfville'` (operator-selected on
--                     the export form). Free-text varchar(50) so future
--                     sites don't need a schema change.
-- * `range_from`    — start of the export window (DATE, inclusive).
-- * `range_to`      — end of the export window (DATE, inclusive).
-- * `format`        — `'csv'` or `'xlsx'`. CHECK constraint enforces.
-- * `row_count`     — foreign-guest row count in the emitted file
--                     (one row per (guest, room-night-cluster) — today
--                     keyed on `ht_checkins.cin_room_id` until Track B2
--                     lands the `ht_checkin_rooms` junction).
-- * `exported_by`   — username (or `'system'`). varchar(100).
-- * `exported_at`   — defaults to NOW().
-- * `file_hash`     — hex SHA-256 of the generated file bytes (64
--                     chars). Required for audit non-repudiation: if
--                     the immigration office disputes a filing we can
--                     re-derive the same bytes and prove they match.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_rr4_exports (
    id           SERIAL PRIMARY KEY,
    site         VARCHAR(50)  NOT NULL,
    range_from   DATE         NOT NULL,
    range_to     DATE         NOT NULL,
    format       VARCHAR(10)  NOT NULL,
    row_count    INTEGER      NOT NULL DEFAULT 0,
    exported_by  VARCHAR(100) NOT NULL DEFAULT 'system',
    exported_at  TIMESTAMP    NOT NULL DEFAULT NOW(),
    file_hash    VARCHAR(64)  NOT NULL,

    CONSTRAINT ck_ht_rr4_exports_format
        CHECK (format IN ('csv', 'xlsx')),
    CONSTRAINT ck_ht_rr4_exports_range
        CHECK (range_to >= range_from)
);

CREATE INDEX IF NOT EXISTS ix_ht_rr4_exports_site_range
    ON ht_rr4_exports (site, range_from, range_to);
CREATE INDEX IF NOT EXISTS ix_ht_rr4_exports_exported_at
    ON ht_rr4_exports (exported_at DESC);

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_rr4_exports_exported_at;
-- DROP INDEX IF EXISTS ix_ht_rr4_exports_site_range;
-- DROP TABLE IF EXISTS ht_rr4_exports;
