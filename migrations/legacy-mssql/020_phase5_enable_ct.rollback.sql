-- Rollback for 020_phase5_enable_ct.sql.
--
-- Reverses in dependency-safe order: DISABLE CT → DROP PK → ALTER COLUMN
-- back to NULLABLE (only for the columns the apply tightened). Every
-- statement is `IF EXISTS`-guarded so re-runs no-op cleanly.
--
-- Database-level CT is left enabled by default — disabling it would also
-- disable any other CT-enabled tables on the DB (e.g. the 6 mirror
-- tables from 021). Operator can run the optional last block manually
-- if the goal is full Phase 5 + 5.5b removal.

SET NOCOUNT ON;
GO

-- 1. HT_Customers
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Customers'))
    ALTER TABLE HT_Customers DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Customers' AND object_id = OBJECT_ID('HT_Customers'))
    ALTER TABLE HT_Customers DROP CONSTRAINT PK_HT_Customers;
GO

-- 2. HT_Rooms
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Rooms'))
    ALTER TABLE HT_Rooms DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Rooms' AND object_id = OBJECT_ID('HT_Rooms'))
    ALTER TABLE HT_Rooms DROP CONSTRAINT PK_HT_Rooms;
GO
ALTER TABLE HT_Rooms ALTER COLUMN id INT NULL;
GO

-- 3. HT_Book_H
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Book_H'))
    ALTER TABLE HT_Book_H DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Book_H' AND object_id = OBJECT_ID('HT_Book_H'))
    ALTER TABLE HT_Book_H DROP CONSTRAINT PK_HT_Book_H;
GO
ALTER TABLE HT_Book_H ALTER COLUMN Book_ID VARCHAR(50) NULL;
GO

-- 4. HT_Book_Ds
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Book_Ds'))
    ALTER TABLE HT_Book_Ds DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Book_Ds' AND object_id = OBJECT_ID('HT_Book_Ds'))
    ALTER TABLE HT_Book_Ds DROP CONSTRAINT PK_HT_Book_Ds;
GO

-- 5. HT_Book_Date
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Book_Date'))
    ALTER TABLE HT_Book_Date DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Book_Date' AND object_id = OBJECT_ID('HT_Book_Date'))
    ALTER TABLE HT_Book_Date DROP CONSTRAINT PK_HT_Book_Date;
GO

-- 6. HT_CheckIn_H
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_CheckIn_H'))
    ALTER TABLE HT_CheckIn_H DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_CheckIn_H' AND object_id = OBJECT_ID('HT_CheckIn_H'))
    ALTER TABLE HT_CheckIn_H DROP CONSTRAINT PK_HT_CheckIn_H;
GO

-- 7. HT_CheckIn_Ds
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_CheckIn_Ds'))
    ALTER TABLE HT_CheckIn_Ds DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_CheckIn_Ds' AND object_id = OBJECT_ID('HT_CheckIn_Ds'))
    ALTER TABLE HT_CheckIn_Ds DROP CONSTRAINT PK_HT_CheckIn_Ds;
GO

-- 8. HT_CheckIn_Pay
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_CheckIn_Pay'))
    ALTER TABLE HT_CheckIn_Pay DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_CheckIn_Pay' AND object_id = OBJECT_ID('HT_CheckIn_Pay'))
    ALTER TABLE HT_CheckIn_Pay DROP CONSTRAINT PK_HT_CheckIn_Pay;
GO

-- 9. HT_Room_Status
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Room_Status'))
    ALTER TABLE HT_Room_Status DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Room_Status' AND object_id = OBJECT_ID('HT_Room_Status'))
    ALTER TABLE HT_Room_Status DROP CONSTRAINT PK_HT_Room_Status;
GO
ALTER TABLE HT_Room_Status ALTER COLUMN id INT NULL;
GO

-- 10. HT_Rooms_Cancel
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Rooms_Cancel'))
    ALTER TABLE HT_Rooms_Cancel DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Rooms_Cancel' AND object_id = OBJECT_ID('HT_Rooms_Cancel'))
    ALTER TABLE HT_Rooms_Cancel DROP CONSTRAINT PK_HT_Rooms_Cancel;
GO
ALTER TABLE HT_Rooms_Cancel ALTER COLUMN id INT NULL;
GO

-- 11. HT_Receipt_H
IF EXISTS (SELECT 1 FROM sys.change_tracking_tables WHERE object_id = OBJECT_ID('HT_Receipt_H'))
    ALTER TABLE HT_Receipt_H DISABLE CHANGE_TRACKING;
GO
IF EXISTS (SELECT 1 FROM sys.indexes WHERE name = 'PK_HT_Receipt_H' AND object_id = OBJECT_ID('HT_Receipt_H'))
    ALTER TABLE HT_Receipt_H DROP CONSTRAINT PK_HT_Receipt_H;
GO

-- OPTIONAL: disable database-level CT entirely. Only safe if NO other
-- table on this DB has CT enabled (e.g. 021's 6 mirror tables also
-- need to be rolled back first). Uncomment to use.
-- IF EXISTS (SELECT 1 FROM sys.change_tracking_databases WHERE database_id = DB_ID())
--     ALTER DATABASE CURRENT SET CHANGE_TRACKING = OFF;
-- GO
