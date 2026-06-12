-- Phase 5/E2 rollback — disable CT, drop PK on `HT_Book_Pro`. Reverts
-- to the pre-2026-06-12 schema state.
--
-- Use only if the .NET app misbehaves after apply or if we need to
-- back out for an unrelated reason. Rollback is order-sensitive:
-- DISABLE CT before DROP PK.
--
-- Safe to run multiple times — every statement is guarded by a
-- pre-check so re-running just no-ops. Pattern mirrors
-- 022_phase5e_other_people_rooms_cancel.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/023_book_pro_ct.rollback.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'

SET NOCOUNT ON;

-- HT_Book_Pro
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Book_Pro'))
    ALTER TABLE HT_Book_Pro DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Book_Pro' AND object_id=OBJECT_ID('HT_Book_Pro'))
    ALTER TABLE HT_Book_Pro DROP CONSTRAINT PK_HT_Book_Pro;
-- `id` is IDENTITY — it was already NOT NULL before the apply (no
-- ALTER COLUMN was issued), so there is no nullability to revert.

PRINT '== Phase 5/E2 rollback complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_Book_Pro')
 ORDER BY t.name;
