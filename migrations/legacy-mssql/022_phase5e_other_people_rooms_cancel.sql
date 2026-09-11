-- Phase 5/E1 — Track E1 (audit 2026-05-13) sync-gap closure on the
-- legacy MSSQL side. Two CT additions:
--
-- 1. `HT_CheckIn_Other_People` — T2 HIGH-3. The TM.30 immigration
--    registry table is silently stale on our side because the legacy
--    .NET app inserts companion-guest rows there (FrmCheckIn.cs lines
--    9490 / 9975 — INSERT on save, DELETE-then-reinsert on edit) but
--    the table has no PK and no CT subscription. Add a PK on the
--    identity `id` column and enable per-table CT so the new
--    `GuestRegistryMapper` (added in sync/mappers/guest_registry.rs)
--    can mirror rows into `ht_guest_registry`.
--
-- (HT_Rooms_Cancel was already CT-enabled in Phase 5 / migration 020
-- but had no mapper — the mapper is shipped without a new migration
-- because the CT subscription already exists. See sync/mappers/mirror.rs
-- for the new `RoomsCancelMirrorMapper`.)
--
-- Pre-flight (to be verified against <legacy-mssql-host> at apply time):
--   * No existing PRIMARY KEY on `HT_CheckIn_Other_People`
--   * No existing per-table CT
--   * `id` is the IDENTITY column (live-verified IDENTITY per
--     `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Other_People`" "schema says NOT IDENTITY but live verified IDENTITY"
--     — was a bare cheatsheet line-585 citation, which had already drifted
--     onto the `HT_CheckIn_Product` schema fence),
--     so an existing IDENTITY column is already NOT NULL on every
--     row but the column nullability may still be marked NULL in
--     the system catalog; the ALTER COLUMN forces it loud.
--
-- Rollback: see 022_phase5e_other_people_rooms_cancel.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/022_phase5e_other_people_rooms_cancel.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'

SET NOCOUNT ON;
GO

-- HT_CheckIn_Other_People — IDENTITY id. Tighten NOT NULL (no-op if
-- already non-null per the live verification), add PK, enable CT.
-- GO separators are MANDATORY between ALTER COLUMN ... NOT NULL and
-- ADD CONSTRAINT PRIMARY KEY — see migration 020's docstring for the
-- Msg 8111 trap learned 2026-04-28.
ALTER TABLE HT_CheckIn_Other_People ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_CheckIn_Other_People ADD CONSTRAINT PK_HT_CheckIn_Other_People PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_CheckIn_Other_People ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

PRINT '== Phase 5/E1 apply complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_CheckIn_Other_People')
 ORDER BY t.name;
