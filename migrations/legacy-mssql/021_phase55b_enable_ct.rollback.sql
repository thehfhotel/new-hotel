-- Phase 5.5b rollback — disable CT, drop PKs, revert NOT NULL on the
-- 6 tables touched by 021_phase55b_enable_ct.sql. Reverts to the
-- pre-2026-04-28 schema state.
--
-- Use only if the .NET app misbehaves after the apply OR if we need
-- to back out for an unrelated reason. Rollback is order-sensitive:
-- DISABLE CT before DROP PK before ALTER COLUMN nullable.
--
-- Safe to run multiple times — every statement is guarded by a
-- pre-check so re-running just no-ops. (The IF block style mirrors
-- the original Phase-5 rollback at /tmp/disable-ct-rollback.sql on
-- evergreen.)
--
-- Application:
--   ssh evergreen 'cat <path>/021_phase55b_enable_ct.rollback.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'

SET NOCOUNT ON;

-- 1. HT_Cupon
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Cupon'))
    ALTER TABLE HT_Cupon DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Cupon' AND object_id=OBJECT_ID('HT_Cupon'))
    ALTER TABLE HT_Cupon DROP CONSTRAINT PK_HT_Cupon;
IF EXISTS (SELECT 1 FROM sys.columns WHERE name='cupon_no' AND object_id=OBJECT_ID('HT_Cupon') AND is_nullable=0)
    ALTER TABLE HT_Cupon ALTER COLUMN cupon_no INT NULL;

-- 2. HT_CheckIn_Product
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_CheckIn_Product'))
    ALTER TABLE HT_CheckIn_Product DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_CheckIn_Product' AND object_id=OBJECT_ID('HT_CheckIn_Product'))
    ALTER TABLE HT_CheckIn_Product DROP CONSTRAINT PK_HT_CheckIn_Product;
-- (id is identity NOT NULL by original schema — no nullable revert)

-- 3. HT_Deposit
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Deposit'))
    ALTER TABLE HT_Deposit DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Deposit' AND object_id=OBJECT_ID('HT_Deposit'))
    ALTER TABLE HT_Deposit DROP CONSTRAINT PK_HT_Deposit;
IF EXISTS (SELECT 1 FROM sys.columns WHERE name='id' AND object_id=OBJECT_ID('HT_Deposit') AND is_nullable=0)
    ALTER TABLE HT_Deposit ALTER COLUMN id INT NULL;

-- 4. HT_Changed_Room
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Changed_Room'))
    ALTER TABLE HT_Changed_Room DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Changed_Room' AND object_id=OBJECT_ID('HT_Changed_Room'))
    ALTER TABLE HT_Changed_Room DROP CONSTRAINT PK_HT_Changed_Room;
-- (id is identity NOT NULL by original schema — no nullable revert)

-- 5. HT_Bill_Debt_H
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Bill_Debt_H'))
    ALTER TABLE HT_Bill_Debt_H DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Bill_Debt_H' AND object_id=OBJECT_ID('HT_Bill_Debt_H'))
    ALTER TABLE HT_Bill_Debt_H DROP CONSTRAINT PK_HT_Bill_Debt_H;
IF EXISTS (SELECT 1 FROM sys.columns WHERE name='Bill_No' AND object_id=OBJECT_ID('HT_Bill_Debt_H') AND is_nullable=0)
    ALTER TABLE HT_Bill_Debt_H ALTER COLUMN Bill_No VARCHAR(20) NULL;

-- 6. HT_Bill_Debt_Ds
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id=OBJECT_ID('HT_Bill_Debt_Ds'))
    ALTER TABLE HT_Bill_Debt_Ds DISABLE CHANGE_TRACKING;
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name='PK_HT_Bill_Debt_Ds' AND object_id=OBJECT_ID('HT_Bill_Debt_Ds'))
    ALTER TABLE HT_Bill_Debt_Ds DROP CONSTRAINT PK_HT_Bill_Debt_Ds;
-- (id is identity NOT NULL by original schema — no nullable revert)

PRINT '== Phase 5.5b rollback complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_Cupon','HT_CheckIn_Product','HT_Deposit','HT_Changed_Room','HT_Bill_Debt_H','HT_Bill_Debt_Ds')
 ORDER BY t.name;
