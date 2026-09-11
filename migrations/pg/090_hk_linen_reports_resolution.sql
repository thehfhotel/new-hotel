-- Migration: 090_hk_linen_reports_resolution
-- Version: vNext
-- Date: 2026-09-01
-- Make the maid's linen-shortage (ขาดผ้า) report COMPLETABLE — owner request
-- 2026-09-01. `POST /api/hk/rooms/{room_id}/linen-shortage/resolve`.
--
-- ## What changes, and what it corrects
--
-- Migration 088 shipped `ht_hk_linen_reports` as RECORD-ONLY: rows existed so
-- housekeeping could see what ran out where, and the `/hk` room shapes badged a
-- room from a DAY-SCOPED `EXISTS` (`linenShortageToday`, the Bangkok civil day).
-- Both of those statements are now WRONG as the whole story, and this migration
-- is what makes the corrected one representable:
--
-- * a linen-shortage report is OPEN until a maid marks the room restocked. It
--   is no longer a record; it is a small piece of work with a completion.
-- * the ขาดผ้า indication therefore means "this room has OPEN reports", of ANY
--   age. Completion SUPERSEDES the old day-rollover rule — a shortage reported
--   at 23:55 and not restocked is still the thing that needs doing at 00:05,
--   exactly the "visible until done, whatever the day" convention ADR 0008's
--   room signals already follow (CONTEXT.md §Housekeeping).
--
-- Completion is ROOM-LEVEL, not per-kind: a maid restocks a room in one trip,
-- so ONE เติมผ้าแล้ว resolves every open row for that room. Per-kind taps would
-- be busywork with no operational meaning — she does not carry the pillowcases
-- up and leave the towels for a second tap.
--
-- HISTORY STAYS APPEND-ONLY. Nothing is deleted and no row's reported data is
-- rewritten: a resolved row keeps `hklr_kind` / `hklr_qty` / `hklr_badge` /
-- `hklr_created_at` exactly as filed and merely GAINS who resolved it and when.
-- Grouping by `hklr_kind` still recovers consumption over any window, which is
-- the reason the table exists at all.
--
-- ## Still PG-CANONICAL ONLY, still no event
--
-- Unchanged from 088 and not a fill-in-the-blank: iHOTEL has no linen-inventory
-- counterpart, so there is NO sync mapper, NO writeback recipe, NO
-- `WritebackIntent` and no dark flag waiting to enable one — coexistence
-- invariant #6 is upheld by there being nothing legacy-coupled to gate. The
-- resolve endpoint publishes NO domain event either: unlike ADR 0008's signals
-- (whose events are the SSE plumbing behind two live boards), linen stays
-- record-domain and both surfaces pick the change up on their next poll/reload.
--
-- ## Shape of the columns
--
-- All three are NULLABLE with no default, so every existing row is `NULL` =
-- OPEN, which is the correct reading of the rows already in both databases: no
-- maid has marked any of those rooms restocked. There is deliberately NO
-- `hklr_status` enum — "resolved" is exactly `hklr_resolved_at IS NOT NULL`,
-- one fact in one column that cannot disagree with itself.
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` logical DBs
-- carry the table) — `scripts/migrate.sh --site hfville` runs this same file
-- against `hotelville`, and `init-db/01-create-hotelville-database.sh` replays
-- `init-hotelnew.sql` there on a fresh cluster, so no per-site file is needed.
--
-- ADDITIVE AND SAFE TO APPLY AHEAD OF THE CODE: adding nullable columns leaves
-- every existing row valid and every existing statement working (the 088
-- INSERT names its columns), and until the new binary ships nothing writes
-- them, so the migration is inert on its own.

-- UP MIGRATION

-- When the room was marked restocked. NULL = OPEN, and that IS the status —
-- there is no separate status column to fall out of step with it.
ALTER TABLE ht_hk_linen_reports
    ADD COLUMN IF NOT EXISTS hklr_resolved_at TIMESTAMPTZ;

-- HF ID badge of the maid who marked it restocked (verified identity from the
-- Cloudflare Access assertion, never client-typed) plus the display-name
-- snapshot — the same convention, and the same deliberate absence of an FK to
-- `ht_users`, as `hklr_badge` / `hklr_name` on the reporting side.
ALTER TABLE ht_hk_linen_reports
    ADD COLUMN IF NOT EXISTS hklr_resolved_badge TEXT;

ALTER TABLE ht_hk_linen_reports
    ADD COLUMN IF NOT EXISTS hklr_resolved_name TEXT;

-- The hot path of the new read surface: the room list's `linenShortageOpen`
-- EXISTS (correlated per room, one statement for the whole list), the detail's
-- open per-kind totals, and the resolve UPDATE's own predicate. PARTIAL on
-- exactly `hklr_resolved_at IS NULL` — same reasoning as
-- `ix_ht_hk_room_signals_live` (migration 089): resolved rows are history that
-- only the audit reads, and they are the ones that grow without bound, so
-- keeping them out keeps this index the size of the live backlog forever.
CREATE INDEX IF NOT EXISTS ix_ht_hk_linen_reports_open
    ON ht_hk_linen_reports (hklr_room_id)
    WHERE hklr_resolved_at IS NULL;

-- Refresh 088's table comment, which still says RECORD-ONLY and still describes
-- the day-scoped read.
COMMENT ON TABLE ht_hk_linen_reports IS
    'Maid-reported linen shortages (ขาดผ้า) from the /hk surface, migration 088; '
    'COMPLETABLE since migration 090. A report is OPEN until a maid marks the room '
    'restocked (เติมผ้าแล้ว), which is ROOM-LEVEL: one tap resolves every open row for '
    'that room. hklr_resolved_at IS NULL is the status — no separate status column. '
    'The room''s ขาดผ้า indication means "has OPEN reports" of ANY age; completion '
    'supersedes the old day-rollover rule (visible until done, the same convention as '
    'ht_hk_room_signals). Still APPEND-ONLY: a resolved row keeps everything it was '
    'filed with and only gains who/when resolved. One row per (submission, kind); '
    'hklr_report_uuid groups a submission. PG-CANONICAL ONLY: no legacy counterpart, no '
    'sync mapper, no writeback, no domain event, no notification. hklr_kind is TEXT with '
    'NO CHECK on purpose — the kind allowlist lives in routes::hk::VALID_LINEN_KINDS so a '
    'new kind needs no migration. Per-site (connection-level scoping).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented — destructive)
-- =============================================================================
-- -- Dropping these columns DISCARDS every completion a maid recorded, and with
-- -- them the only evidence that a room was restocked. Re-open the rows
-- -- deliberately if that is really what is wanted; do not run this to "undo a
-- -- deploy".
-- DROP INDEX IF EXISTS ix_ht_hk_linen_reports_open;
-- ALTER TABLE ht_hk_linen_reports DROP COLUMN IF EXISTS hklr_resolved_name;
-- ALTER TABLE ht_hk_linen_reports DROP COLUMN IF EXISTS hklr_resolved_badge;
-- ALTER TABLE ht_hk_linen_reports DROP COLUMN IF EXISTS hklr_resolved_at;
-- DELETE FROM schema_migrations WHERE version = '090';
