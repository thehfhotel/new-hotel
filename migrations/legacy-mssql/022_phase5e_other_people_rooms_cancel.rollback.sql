-- Phase 5/E1 rollback — disable CT, drop PK, revert NOT NULL on
-- `HT_CheckIn_Other_People`. Reverts to the pre-2026-05-13 schema state.
--
-- Use only if the .NET app misbehaves after apply or if we need to
-- back out for an unrelated reason. Rollback is order-sensitive:
-- DISABLE CT before DROP PK before ALTER COLUMN nullable.
--
-- Safe to run multiple times — every statement is guarded by a
-- pre-check so re-running just no-ops. Pattern mirrors
-- 021_phase55b_enable_ct.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/022_phase5e_other_people_rooms_cancel.rollback.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'

SET NOCOUNT ON;

-- HT_CheckIn_Other_People
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_CheckIn_Other_People'))
    ALTER TABLE HT_CheckIn_Other_People DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_CheckIn_Other_People' AND object_id=OBJECT_ID('HT_CheckIn_Other_People'))
    ALTER TABLE HT_CheckIn_Other_People DROP CONSTRAINT PK_HT_CheckIn_Other_People;
-- `id` is IDENTITY — already NOT NULL by IDENTITY semantics; we leave
-- the post-rollback nullability as the catalog already records it.

PRINT '== Phase 5/E1 rollback complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_CheckIn_Other_People')
 ORDER BY t.name;
