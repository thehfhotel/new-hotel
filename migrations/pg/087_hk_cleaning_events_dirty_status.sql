-- Migration: 087_hk_cleaning_events_dirty_status
-- Version: vNext
-- Date: 2026-08-14
-- Wave-4 housekeeping stream (B1) — allow a maid to report "ห้องยังไม่สะอาด"
-- from the /hk surface, alongside the existing เริ่มทำความสะอาด / เสร็จแล้ว.
--
-- ## What changes
--
-- `ht_hk_cleaning_events.hkev_status` accepted exactly ('started','done') since
-- migration 077. This widens the CHECK to ('started','done','dirty').
--
-- ADDITIVE AND SAFE TO APPLY AHEAD OF THE FEATURE: widening a CHECK cannot make
-- an existing row invalid (both sites hold ZERO rows in this table as of
-- 2026-08-14), and nothing writes 'dirty' until `HK_MARK_DIRTY_ENABLED` is
-- flipped on — the route answers 403 while it is off (invariant #6). So this
-- migration ships with the code and stays inert until its own verification
-- window (PENDING-VERIFICATIONS V12).
--
-- ## Correcting migration 077's stated invariant
--
-- 077's header claims this table "deliberately does NOT flip
-- `ht_rooms_new.room_clean`". That has been FALSE since 2026-08-11
-- (housekeeping-ops): a `done` event delegates to
-- `service::housekeeping::report_cleaning_progress`, which flips `room_clean`
-- to true, enqueues the proven `MarkRoomClean` writeback and publishes
-- `RoomMarkedClean` in ONE transaction — that is the whole point (reception
-- must see the finished room on iHOTEL's board without the maid touching the
-- legacy app). From this migration, `dirty` does the mirror image: flips
-- `room_clean` to false and enqueues `MarkRoomDirty`.
--
-- The TABLE itself remains PG-canonical-only: no legacy twin, no sync mapper,
-- no CT enablement. Only the cleanliness FLAG on `ht_rooms_new` crosses to
-- legacy, through the existing, byte-pinned recipes
-- (`writeback/recipes/mark_clean.rs`, `writeback/recipes/mark_dirty.rs` — the
-- single statement `update HT_Rooms set Room_Clean='yes' where id=<id>`, plain
-- literals, no N'…', no HT_Housewife row). NO new recipe was written for this.
--
-- `started` stays legacy-inert on purpose: iHOTEL's `Room_Clean_Time` feeds its
-- room-power countdown, so mirroring "in progress" is parity risk for no gain.
-- It publishes the PG-only `RoomCleaningStarted` domain event instead, which is
-- what makes progress visible to reception in real time over SSE. `event_log`
-- needs no migration for it — `event_type` is TEXT with no CHECK (migration 012).
--
-- Constraint name below was read from the live databases (both `hotelnew` and
-- `hotelville` carry `ht_hk_cleaning_events_hkev_status_check`), so the DROP
-- resolves rather than silently no-opping. `IF EXISTS` keeps it re-runnable.

-- UP MIGRATION

ALTER TABLE ht_hk_cleaning_events
    DROP CONSTRAINT IF EXISTS ht_hk_cleaning_events_hkev_status_check;

ALTER TABLE ht_hk_cleaning_events
    ADD CONSTRAINT ht_hk_cleaning_events_hkev_status_check
        CHECK (hkev_status IN ('started', 'done', 'dirty'));

-- Refresh 077's table comment, which still asserted the no-flip invariant.
COMMENT ON TABLE ht_hk_cleaning_events IS
    'Maid-reported room-cleaning progress (employee-login plan Phase 4, migration 077; '
    '''dirty'' added by migration 087). Append-only event log; latest event per room per '
    'Thai day = current progress. The TABLE is PG-CANONICAL ONLY (no legacy counterpart, '
    'no sync mapper). Since 2026-08-11 the ''done'' status DOES flip ht_rooms_new.room_clean '
    'and enqueue the MarkRoomClean writeback in the same transaction; ''dirty'' (migration '
    '087, gated by HK_MARK_DIRTY_ENABLED) does the mirror image via MarkRoomDirty. '
    '''started'' stays legacy-inert and publishes RoomCleaningStarted for reception''s live '
    'board. Per-site (connection-level scoping).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- -- Refuses to apply while any 'dirty' row exists — that is intentional: the
-- -- rows are a maid's audit trail and must not be silently deleted to satisfy
-- -- a narrowed constraint. Delete or re-state them explicitly first.
-- ALTER TABLE ht_hk_cleaning_events
--     DROP CONSTRAINT IF EXISTS ht_hk_cleaning_events_hkev_status_check;
-- ALTER TABLE ht_hk_cleaning_events
--     ADD CONSTRAINT ht_hk_cleaning_events_hkev_status_check
--         CHECK (hkev_status IN ('started', 'done'));
-- DELETE FROM schema_migrations WHERE version = '087';
