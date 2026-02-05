-- Migration: 003_alter_ht_rates_table
-- Version: 2.1.0
-- Date: 2026-02-05
-- Description: Alters HT_Rates table to support multiplier/fixed rate types for Phase 3 Financial features

-- ==============================================================================
-- UP MIGRATION
-- ==============================================================================

-- Add new columns for rate type and value
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Type')
BEGIN
    ALTER TABLE [dbo].[HT_Rates]
    ADD [Rate_Type] VARCHAR(20) DEFAULT 'fixed' NOT NULL
END
GO

IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Value')
BEGIN
    ALTER TABLE [dbo].[HT_Rates]
    ADD [Rate_Value] DECIMAL(10,2) NULL
END
GO

-- Migrate data: copy Rate_Price to Rate_Value for existing records
UPDATE [dbo].[HT_Rates]
SET [Rate_Value] = [Rate_Price]
WHERE [Rate_Value] IS NULL AND [Rate_Price] IS NOT NULL
GO

-- Rename columns for consistency with API
-- Rate_Start_Date -> Rate_Valid_From
-- Rate_End_Date -> Rate_Valid_To
-- Rate_Day_Of_Week -> Rate_Days_Of_Week

-- Note: SQL Server doesn't support RENAME COLUMN directly. We use sp_rename
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Start_Date')
BEGIN
    EXEC sp_rename 'HT_Rates.Rate_Start_Date', 'Rate_Valid_From', 'COLUMN'
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_End_Date')
BEGIN
    EXEC sp_rename 'HT_Rates.Rate_End_Date', 'Rate_Valid_To', 'COLUMN'
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Day_Of_Week')
BEGIN
    EXEC sp_rename 'HT_Rates.Rate_Day_Of_Week', 'Rate_Days_Of_Week', 'COLUMN'
END
GO

-- Rename Rate_Created_At to Rate_Created for consistency
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Created_At')
BEGIN
    EXEC sp_rename 'HT_Rates.Rate_Created_At', 'Rate_Created', 'COLUMN'
END
GO

-- Add Rate_Updated column if not exists
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND name = 'Rate_Updated')
BEGIN
    ALTER TABLE [dbo].[HT_Rates]
    ADD [Rate_Updated] DATETIME DEFAULT GETDATE()
END
GO

-- Make Rate_Room_Type_ID nullable (to support rates not tied to a specific room type)
-- Note: This requires dropping the FK constraint first if it exists
IF EXISTS (SELECT * FROM sys.foreign_keys WHERE name = 'FK_HT_Rates_RoomType')
BEGIN
    ALTER TABLE [dbo].[HT_Rates]
    DROP CONSTRAINT FK_HT_Rates_RoomType
END
GO

-- Alter column to allow NULL
ALTER TABLE [dbo].[HT_Rates]
ALTER COLUMN [Rate_Room_Type_ID] INT NULL
GO

-- Re-add the FK constraint
ALTER TABLE [dbo].[HT_Rates]
ADD CONSTRAINT FK_HT_Rates_RoomType FOREIGN KEY ([Rate_Room_Type_ID])
    REFERENCES [dbo].[HT_Room_Types]([Type_ID])
GO

-- Update indexes for the new date columns
IF EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Rates_Dates' AND object_id = OBJECT_ID('HT_Rates'))
BEGIN
    DROP INDEX IX_HT_Rates_Dates ON [dbo].[HT_Rates]
END
GO

CREATE INDEX IX_HT_Rates_ValidDates ON [dbo].[HT_Rates] ([Rate_Valid_From], [Rate_Valid_To])
GO

-- ==============================================================================
-- DOWN MIGRATION (Rollback - commented out for safety)
-- ==============================================================================

/*
-- WARNING: This will revert schema changes!

-- Drop new columns
ALTER TABLE [dbo].[HT_Rates] DROP COLUMN [Rate_Type]
ALTER TABLE [dbo].[HT_Rates] DROP COLUMN [Rate_Value]
ALTER TABLE [dbo].[HT_Rates] DROP COLUMN [Rate_Updated]

-- Rename columns back
EXEC sp_rename 'HT_Rates.Rate_Valid_From', 'Rate_Start_Date', 'COLUMN'
EXEC sp_rename 'HT_Rates.Rate_Valid_To', 'Rate_End_Date', 'COLUMN'
EXEC sp_rename 'HT_Rates.Rate_Days_Of_Week', 'Rate_Day_Of_Week', 'COLUMN'
EXEC sp_rename 'HT_Rates.Rate_Created', 'Rate_Created_At', 'COLUMN'

-- Recreate original index
DROP INDEX IX_HT_Rates_ValidDates ON [dbo].[HT_Rates]
CREATE INDEX IX_HT_Rates_Dates ON [dbo].[HT_Rates] ([Rate_Start_Date], [Rate_End_Date])
*/
