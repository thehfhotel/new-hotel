-- Migration: 088_create_ht_hk_linen_reports
-- Version: vNext
-- Date: 2026-08-31
-- Maid "linen shortage" (ขาดผ้า) report on the `/hk` surface —
-- `POST /api/hk/rooms/{room_id}/linen-shortage`.
--
-- ## Background
--
-- A maid servicing a room finds she is short of linen (ปลอกหมอน / ปลอกผ้านวม /
-- ผ้าเช็ดตัว / ผ้าเช็ดหน้า / ผ้าเช็ดเท้า) and reports how many pieces of each
-- kind she is missing. RECORD-ONLY: the rows exist so housekeeping can see what
-- ran out where. Nothing else happens.
--
-- - PG-CANONICAL ONLY, and more narrowly so than 077's tables. iHOTEL has NO
--   linen-inventory counterpart at all, so there is NO sync mapper and NO
--   legacy writeback (coexistence invariant #6 — no new legacy writes). Unlike
--   `ht_hk_cleaning_events` (whose `done` phase grew a `MarkRoomClean`
--   writeback on 2026-08-11), this table has no path to legacy by design and
--   is not a candidate for one: there is nothing on the iHOTEL side to write.
-- - Also NO notification, NO Slack, NO domain event / outbox row. The endpoint
--   commits PG rows and returns; the whole feature is these rows plus the
--   handler that writes them.
-- - APPEND-ONLY, one row per (submission, kind). `hklr_report_uuid` groups the
--   rows of ONE submission (generated server-side in
--   `service::housekeeping::report_linen_shortage`, NOT client-supplied), so a
--   maid who reports 3 pillowcases + 2 bath towels in one tap produces two rows
--   sharing one uuid. Grouping by uuid recovers the submission; grouping by
--   kind recovers consumption. Neither view needs a header table.
-- - Identity columns store the HF ID badge (`sub` of the verified Cloudflare
--   Access identity) plus the display-name snapshot — same convention as
--   `ht_hk_cleaning_events.hkev_badge` / `hkev_name`, and deliberately NO FK to
--   `ht_users`: maids are CF Access + HF ID identities, not PMS accounts.
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` logical
-- DBs get this table; each site's pool holds its own reports) — same model as
-- `ht_hk_cleaning_events` / `ht_verification_responses`. `scripts/migrate.sh
-- --site hfville` runs this same file against `hotelville`, and
-- `init-db/01-create-hotelville-database.sh` replays `init-hotelnew.sql`
-- there on a fresh cluster, so no per-site file is needed.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_hk_linen_reports (
    hklr_id          BIGSERIAL    PRIMARY KEY,
    -- Groups the rows of ONE submission. Server-generated (uuid v4) in the
    -- service; never accepted from the client.
    hklr_report_uuid UUID         NOT NULL,
    -- Room the maid reported on (site-local canonical room). FK + CASCADE
    -- mirrors `ht_hk_cleaning_events.hkev_room_id` — the route 404s an unknown
    -- or inactive room before the insert, so this is a backstop, not the gate.
    hklr_room_id     INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    -- Which linen ran short. DELIBERATELY plain TEXT with NO CHECK enumerating
    -- the kinds: the allowlist lives ONLY in the app
    -- (`routes::hk::VALID_LINEN_KINDS` = pillowcase | duvet_cover | bath_towel
    -- | face_towel | foot_towel), which is what the 400 for an unknown kind is
    -- served from. Adding a sixth kind later (ผ้าปูที่นอน, say) is then a
    -- one-line constant change that ships with the frontend — no migration, no
    -- ALTER on a live table at two sites, and no window where the deployed
    -- binary and the deployed CHECK disagree about the valid set (exactly the
    -- coupling migration 087 had to unpick for `hkev_status`). The cost is that
    -- the DB will accept a kind the app would refuse; the app is the only
    -- writer, so that cost is not paid.
    hklr_kind        TEXT         NOT NULL,
    -- Pieces missing. Bounded here as well as in the app: 1..=20 is a maid
    -- reporting a shortage, not a stockroom order, and a fat-fingered 200 is
    -- worth refusing at both layers. This CHECK is a real invariant of the
    -- data (unlike the kind set, which is a product decision that moves).
    hklr_qty         INTEGER      NOT NULL CHECK (hklr_qty >= 1 AND hklr_qty <= 20),
    -- HF ID badge of the reporting maid (verified identity, never client-typed).
    hklr_badge       TEXT         NOT NULL,
    -- Display-name snapshot from the identity claims (may lag renames; audit only).
    hklr_name        TEXT,
    hklr_created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Per-room recent-first listing (room detail / housekeeping follow-up view).
CREATE INDEX IF NOT EXISTS ix_ht_hk_linen_reports_room_created
    ON ht_hk_linen_reports (hklr_room_id, hklr_created_at DESC);

COMMENT ON TABLE ht_hk_linen_reports IS
    'Maid-reported linen shortages (ขาดผ้า) from the /hk surface, migration 088. '
    'Append-only, one row per (submission, kind); hklr_report_uuid groups a submission. '
    'RECORD-ONLY and PG-CANONICAL ONLY: no legacy counterpart, no sync mapper, no '
    'writeback, no domain event, no notification. hklr_kind is TEXT with NO CHECK on '
    'purpose — the kind allowlist lives in routes::hk::VALID_LINEN_KINDS so a new kind '
    'needs no migration. Per-site (connection-level scoping).';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_hk_linen_reports_room_created;
-- DROP TABLE IF EXISTS ht_hk_linen_reports;
-- DELETE FROM schema_migrations WHERE version = '088';
