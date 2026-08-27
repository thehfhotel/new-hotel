-- Migration: 078_reseed_ct_state_per_table
-- Version: vNext
-- Date: 2026-07-27
-- Description: Force-reseed EVERY `legacy_ct_state_per_table` row from the
--              CURRENT global `legacy_ct_state.last_seen_version`, so the
--              per-table watermark path (Resilience PR R3, migration 050)
--              can be switched on with `SYNC_PER_TABLE_WATERMARK=true`
--              without a retention-overflow page storm followed by an
--              unbounded Change Tracking replay.
--
-- =============================================================================
-- WHY THIS MIGRATION EXISTS — the per-table rows froze at their apply date
-- =============================================================================
--
-- Both existing seeds of `legacy_ct_state_per_table` use
-- `ON CONFLICT (table_name) DO NOTHING`:
--
--   * `migrations/pg/050_legacy_ct_state_per_table.sql` §"INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version)" "ON CONFLICT (table_name) DO NOTHING"  (18 tables)
--   * `migrations/pg/056_legacy_mirror_ht_book_pro.sql` §"SELECT 'HT_Book_Pro'," "ON CONFLICT (table_name) DO NOTHING"  (HT_Book_Pro)
--
-- `DO NOTHING` was the CORRECT choice there and remains correct for those
-- files: both are one-shot seeds whose job is "create a row if one does not
-- exist yet", and a re-apply must never trample per-table progress that the
-- watcher has since made. Their side effect, however, is that re-running them
-- is a total no-op — so once a row exists it is pinned forever at whatever
-- the global watermark happened to be on the day the migration landed, while
-- the global row keeps advancing every second under the (still default)
-- global-watermark mode.
--
-- Live state on evergreen, verified 2026-07-27 (pre-reseed):
--
--   hotelville   per-table: 9060  for every table except HT_Book_Pro (23849)
--                           (stamped 2026-05-14, the day 050 was applied)
--                global:    37843
--
--   hotelnew     per-table: 17209 for every table except HT_Book_Pro (43891)
--                           (stamped 2026-05-14)
--                global:    67515
--
-- Flipping `SYNC_PER_TABLE_WATERMARK=true` against rows that stale means
-- every table fails `check_retention`'s `min_valid > last_seen` test on the
-- very first tick (SQL Server CT retention here is 2 days; these watermarks
-- are ~2.5 months old and long since aged out). That fires an UNCOOLED,
-- UN-DEDUPED `:rotating_light: CT retention overflow` Slack message per
-- table — 19 pages in one tick — and then, on the following tick, tries to
-- replay an unbounded CT backlog from a version the server can no longer
-- resolve. So the reseed is a hard prerequisite of the flag flip, not a
-- tidy-up.
--
-- =============================================================================
-- WHY `DO UPDATE` IS SAFE HERE (and why GREATEST, not a plain assignment)
-- =============================================================================
--
-- The global watermark is, by construction, a version that EVERY CT-enabled
-- table has already been consumed up to: in global mode the watcher advances
-- `legacy_ct_state` only after all per-table mappers in that tick committed.
-- Moving a per-table row forward to the global value therefore skips nothing
-- — it re-states progress that already happened.
--
-- `GREATEST(existing, EXCLUDED)` guarantees the reseed can only ever move a
-- row FORWARD:
--
--   * `HT_Book_Pro` is ahead of its siblings (seeded later, by 056) but still
--     behind global, so it lands on global too.
--   * If per-table mode is already live somewhere and a table has advanced
--     PAST the (then frozen) global row, its own higher value wins and the
--     reseed leaves it untouched. A plain `SET last_seen_version = EXCLUDED…`
--     would roll such a table BACKWARD and force it to re-consume — hence
--     GREATEST rather than assignment.
--
-- `last_polled_at` is refreshed to `now()` because the column means "when did
-- the watcher last touch this row", and the per-table watchdog that migration
-- 050 added `ix_legacy_ct_state_per_table_polled_at` for queries
-- `last_polled_at < now() - interval`. Leaving it at 2026-05-14 would make
-- all 19 rows look wedged to that watchdog for the first tick after the flip.
--
-- `last_error` / `last_error_at` are deliberately NOT cleared. They are NULL
-- today (the per-table write path has never executed in production), so
-- clearing them would be a no-op now — but on a future re-apply it would
-- erase a live diagnostic. Reseeding a watermark is not the same operation as
-- acknowledging an error.
--
-- =============================================================================
-- ORDERING — why reseed-then-flip is safe
-- =============================================================================
--
-- `scripts/deploy/run-deploy.sh` runs `scripts/migrate.sh` (both sites)
-- BEFORE it starts the compose workers. So on the deploy that ships this
-- file, the reseed is committed while the old sync container is still the
-- one running, and the new container starts against already-current rows.
-- The flag flip is a SEPARATE, later operator action (`gh variable set
-- SYNC_PER_TABLE_WATERMARK -b true` + redeploy) — this migration only makes
-- that flip survivable, it does not perform it. Both `SYNC_PER_TABLE_WATERMARK`
-- and `HFVILLE_SYNC_PER_TABLE_WATERMARK` ship `false`.
--
-- The table list below MUST stay in lock-step with `CT_ENABLED_TABLES` in
-- `hotel-backend/src/bin/sync.rs` (19 entries as of 2026-07-27 — the same
-- list `sync --print-ct-tables` prints for the deploy's CT gate).
--
-- @transactional true

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version, last_polled_at)
SELECT t.name,
       COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0),
       now()
FROM (VALUES
    -- Phase 5 — canonical sync (10 tables, CT enabled 2026-04-25)
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H'),
    -- Phase 5.5b — legacy_mirror.* opaque pass-through (6 tables)
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds'),
    -- Phase 5/E1 — Track E1 sync-gap closure (2 tables)
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel'),
    -- Phase 5/E2 — coexistence audit 2026-06-11 P2 gap closure (1 table)
    ('HT_Book_Pro')
) AS t(name)
ON CONFLICT (table_name) DO UPDATE
    SET last_seen_version = GREATEST(
            legacy_ct_state_per_table.last_seen_version,
            EXCLUDED.last_seen_version
        ),
        last_polled_at = now();

-- Fresh-install note: on a brand-new database `legacy_ct_state` is seeded at
-- `last_seen_version = 0` (migration 013), so the COALESCE yields 0, GREATEST
-- yields 0, and this statement is a semantic no-op on top of 050 + 056 — the
-- watcher's normal `--bootstrap` still owns first-run seeding. Idempotent and
-- safe to re-run at any time.

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- There is NO value-restoring rollback: the pre-reseed per-table versions are
-- not recorded anywhere by this migration, and restoring them would be
-- actively harmful (it would re-create exactly the aged-out watermarks that
-- trigger the retention-overflow page storm). The observed pre-reseed values
-- are captured in the header comment above for forensics only.
--
-- The only meaningful rollback is to stop using the per-table path — i.e.
-- keep `SYNC_PER_TABLE_WATERMARK=false` (the shipped default), under which
-- these rows are inert and the single-row `legacy_ct_state` remains the
-- source of truth.
--
-- DELETE FROM schema_migrations WHERE version = '078';
