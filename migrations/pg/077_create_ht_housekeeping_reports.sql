-- Migration: 077_create_ht_housekeeping_reports
-- Version: vNext
-- Date: 2026-07-09
-- Employee-login plan Phase 4 (HF-erp docs/employee-login-plan.md) — maid-facing
-- housekeeping surface at /hk (behind its own Cloudflare Access app, silent HF ID,
-- grant key `housekeeping`).
--
-- ## Background
--
-- Maids report room-cleaning progress and broken/damaged items from their phones
-- (LINE in-app browser). Two kinds of "Housekeeping Report" (CONTEXT.md term in
-- HF-erp): a cleaning-progress event and a broken-item report. Both are NEW
-- PG-canonical data:
--
-- - PG-CANONICAL ONLY. iHOTEL has NO counterpart for maid-reported cleaning
--   progress (its `HT_Rooms.Room_Clean` boolean is front-desk state, mirrored by
--   the EXISTING housekeeping service, untouched here) — so there is NO sync
--   mapper and NO legacy writeback (like `ht_verification_responses`). The maid's
--   progress log deliberately does NOT flip `ht_rooms_new.room_clean`: that flag
--   drives the front-desk board + the legacy mirror, and v1 of this surface is
--   verification/progress-reporting only (coexistence invariant #6 — no new
--   legacy writes).
-- - `ht_hk_cleaning_events` is APPEND-ONLY: the latest event per room per (Thai)
--   day is the room's current progress; yesterday's events age out of the "today"
--   view naturally, giving the daily reset for free plus a full audit trail.
-- - `ht_hk_broken_reports` carries an optional photo (`BYTEA` + mime — same
--   storage pattern as `ht_guest_documents.doc_image`).
-- - Identity columns store the HF ID badge (`sub` of the verified identity) plus
--   the display name snapshot; there is deliberately NO FK to `ht_users` — maids
--   are identified via Cloudflare Access + HF ID claims, not PMS accounts.
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` logical DBs
-- get these tables; each site's pool holds its own reports) — same model as
-- `ht_verification_responses` / `ht_notes`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_hk_cleaning_events (
    hkev_id         BIGSERIAL    PRIMARY KEY,
    -- Room the maid reported on (site-local canonical room).
    hkev_room_id    INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    -- Progress: 'started' (เริ่มทำความสะอาด) | 'done' (เสร็จแล้ว).
    hkev_status     TEXT         NOT NULL CHECK (hkev_status IN ('started', 'done')),
    -- HF ID badge of the reporting maid (verified identity, never client-typed).
    hkev_badge      TEXT         NOT NULL,
    -- Display-name snapshot from the identity claims (may lag renames; audit only).
    hkev_name       TEXT,
    hkev_created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Latest-event-per-room lookup for the /hk room list ("today" view).
CREATE INDEX IF NOT EXISTS ix_ht_hk_cleaning_events_room_created
    ON ht_hk_cleaning_events (hkev_room_id, hkev_created_at DESC);

COMMENT ON TABLE ht_hk_cleaning_events IS
    'Maid-reported room-cleaning progress (employee-login plan Phase 4, migration 077). '
    'Append-only event log; latest event per room per Thai day = current progress. '
    'PG-CANONICAL ONLY (no legacy counterpart, no sync, no writeback — does NOT touch '
    'ht_rooms_new.room_clean). Per-site (connection-level scoping).';

CREATE TABLE IF NOT EXISTS ht_hk_broken_reports (
    hkbr_id          BIGSERIAL    PRIMARY KEY,
    hkbr_room_id     INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    -- Free-text description of the broken/damaged item (required).
    hkbr_description TEXT         NOT NULL,
    -- HF ID badge of the reporting maid (verified identity).
    hkbr_badge       TEXT         NOT NULL,
    hkbr_name        TEXT,
    -- Optional photo (same BYTEA+mime storage pattern as ht_guest_documents).
    hkbr_photo       BYTEA,
    hkbr_photo_mime  TEXT,
    -- Triage state; v1 only creates 'open' rows (follow-up surface may resolve).
    hkbr_status      TEXT         NOT NULL DEFAULT 'open'
                     CHECK (hkbr_status IN ('open', 'acknowledged', 'resolved')),
    hkbr_created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    hkbr_updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Per-room recent-first listing (room detail screen).
CREATE INDEX IF NOT EXISTS ix_ht_hk_broken_reports_room_created
    ON ht_hk_broken_reports (hkbr_room_id, hkbr_created_at DESC);
-- Open-report triage listing (future reception/maintenance view).
CREATE INDEX IF NOT EXISTS ix_ht_hk_broken_reports_status
    ON ht_hk_broken_reports (hkbr_status, hkbr_created_at DESC);

COMMENT ON TABLE ht_hk_broken_reports IS
    'Maid-submitted broken-item reports (employee-login plan Phase 4, migration 077). '
    'A report, not a maintenance ticket — may be promoted to ht_maintenance_requests '
    'by staff later. PG-CANONICAL ONLY (no legacy counterpart, no sync, no writeback). '
    'Optional photo stored as BYTEA. Per-site (connection-level scoping).';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_hk_broken_reports_status;
-- DROP INDEX IF EXISTS ix_ht_hk_broken_reports_room_created;
-- DROP TABLE IF EXISTS ht_hk_broken_reports;
-- DROP INDEX IF EXISTS ix_ht_hk_cleaning_events_room_created;
-- DROP TABLE IF EXISTS ht_hk_cleaning_events;
-- DELETE FROM schema_migrations WHERE version = '077';
