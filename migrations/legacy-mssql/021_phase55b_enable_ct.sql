-- Phase 5.5b — enable Change Tracking on the 6 transactional
-- legacy-only tables that the CT-mirror mappers (5.5c) will populate
-- into legacy_mirror.* incrementally.
--
-- Database-level CT was already enabled in Phase 5 (2026-04-25).
-- Here we add per-table tracking for:
--   HT_Cupon, HT_CheckIn_Product, HT_Deposit,
--   HT_Changed_Room, HT_Bill_Debt_H, HT_Bill_Debt_Ds
--
-- Pre-flight (verified 2026-04-28 against <legacy-mssql-host>):
--   * No existing PRIMARY KEY on any of the 6 tables
--   * No existing per-table CT on any
--   * Zero NULLs and zero duplicates on every candidate PK column
--
-- Tables with empty data sets at HF Hotel (Deposit, Bill_Debt_*,
-- CheckIn_Product) get the same treatment as live ones — adding a
-- PK + enabling CT on an empty table is microseconds and de-risks
-- future data appearing without observability.
--
-- Rollback: see 021_phase55b_enable_ct.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/021_phase55b_enable_ct.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'

SET NOCOUNT ON;
GO

-- IMPORTANT: GO separators between each statement so SQL Server
-- commits each metadata change before parsing the next. Without GO,
-- ALTER COLUMN ... NOT NULL and the subsequent ADD CONSTRAINT PK
-- run in the same batch and the PK validation reads the OLD column
-- nullability — Msg 8111 "Cannot define PRIMARY KEY constraint on
-- nullable column" even though the ALTER COLUMN succeeded. Learned
-- the hard way 2026-04-28.

-- 1. HT_Cupon — most active table (17,894 rows). Tighten NOT NULL on
--    the natural key, add PK, enable CT.
ALTER TABLE HT_Cupon ALTER COLUMN cupon_no INT NOT NULL;
GO
ALTER TABLE HT_Cupon ADD CONSTRAINT PK_HT_Cupon PRIMARY KEY CLUSTERED (cupon_no);
GO
ALTER TABLE HT_Cupon ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 2. HT_CheckIn_Product — empty table; identity id is already NOT NULL.
ALTER TABLE HT_CheckIn_Product ADD CONSTRAINT PK_HT_CheckIn_Product PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_CheckIn_Product ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 3. HT_Deposit — empty table; tighten id NOT NULL then PK + CT.
ALTER TABLE HT_Deposit ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_Deposit ADD CONSTRAINT PK_HT_Deposit PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Deposit ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 4. HT_Changed_Room — 3,872 rows; identity id is already NOT NULL.
ALTER TABLE HT_Changed_Room ADD CONSTRAINT PK_HT_Changed_Room PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Changed_Room ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 5. HT_Bill_Debt_H — empty table; tighten Bill_No NOT NULL then PK + CT.
ALTER TABLE HT_Bill_Debt_H ALTER COLUMN Bill_No VARCHAR(20) NOT NULL;
GO
ALTER TABLE HT_Bill_Debt_H ADD CONSTRAINT PK_HT_Bill_Debt_H PRIMARY KEY CLUSTERED (Bill_No);
GO
ALTER TABLE HT_Bill_Debt_H ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 6. HT_Bill_Debt_Ds — empty table; identity id is already NOT NULL.
ALTER TABLE HT_Bill_Debt_Ds ADD CONSTRAINT PK_HT_Bill_Debt_Ds PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Bill_Debt_Ds ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

PRINT '== Phase 5.5b apply complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_Cupon','HT_CheckIn_Product','HT_Deposit','HT_Changed_Room','HT_Bill_Debt_H','HT_Bill_Debt_Ds')
 ORDER BY t.name;
