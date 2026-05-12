-- Migration: 029_normalize_cin_status_terminal
-- Version: 2.63.x
-- Date: 2026-05-12
-- Description: Canonicalize the post-checkout terminal value of
--              `ht_checkins.cin_status` so all writers and readers agree
--              on `'checkedout'` (no underscore).
--
-- Background. Three forms accumulated in production:
--   - `'checked_out'`   — written by the CT mapper (sync/mappers/checkin.rs).
--   - `'completed'`     — written by `bin/migrate_legacy.rs` during the
--                         one-time PG bootstrap.
--   - `'checkedout'`    — written by routes/new_checkins via
--                         repository/checkin.rs (current production form).
--
-- The route-layer readers in `routes/new_reports.rs` and the calendar route
-- in `routes/rooms.rs::get_room_status_pg` already filter for `'checkedout'`.
-- Reports + calendar were missing every CT-mapper-written row + the entire
-- 1410-row Ville bootstrap pool.
--
-- This migration flips the two divergent terminal forms to `'checkedout'`,
-- leaving `'active'` and `'cancelled'` untouched. The CT mapper code change
-- in the same release commits to writing `'checkedout'` going forward.
--
-- Writeback contract preserved. The legacy MSSQL contract uses LEGACY
-- literals — `Cin_status` is `'ปกติ'` / `'ยกเลิก'`, `Cin_Room_Status` is
-- `'เข้าพัก'` / `'Check-Out'`. None of those are touched by this migration;
-- writeback maps from the new app's canonical state via its own constants
-- in `hotel-backend/src/writeback/constants.rs` and is unaffected.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

UPDATE ht_checkins
   SET cin_status = 'checkedout',
       updated_at = NOW()
 WHERE cin_status IN ('checked_out', 'completed');

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- There is no clean rollback. Once `'checked_out'` and `'completed'` rows are
-- coalesced into `'checkedout'`, the distinction between CT-mapper-written and
-- bootstrap-written rows is lost. If a rollback is required, the operator must
-- decide which form to restore based on whether the row's `legacy_cin_no` shape
-- matches the bootstrap pattern.
