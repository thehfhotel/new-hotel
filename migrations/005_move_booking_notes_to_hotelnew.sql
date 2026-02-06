-- =============================================================================
-- Migration 005: Move HT_Booking_Notes to HotelNew Database
-- =============================================================================
-- Version: 2.2.0
-- Date: 2026-02-06
--
-- Purpose:
-- This migration ensures HT_Booking_Notes is created in the HotelNew database
-- to enforce the "legacy database is read-only" principle.
--
-- Background:
-- The legacy database is shared with another application and should NEVER be
-- modified by this application. Previously, HT_Booking_Notes was being created
-- in whatever database the connection pool pointed to, which could be the
-- legacy database. This migration ensures the table exists in HotelNew.
--
-- NOTE: This script should be run against the HotelNew database.
-- The init-db/init-hotelnew.sql script already includes this table definition.
-- =============================================================================

USE HotelNew;
GO

-- UP MIGRATION: Create HT_Booking_Notes in HotelNew
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Booking_Notes]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Booking_Notes] (
        [Note_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Book_No] NVARCHAR(50) NOT NULL,
        [Note_Text] NVARCHAR(MAX) NOT NULL,
        [Created_At] DATETIME DEFAULT GETDATE(),
        [Updated_At] DATETIME DEFAULT GETDATE()
    )

    CREATE INDEX IX_Booking_Notes_BookNo ON [dbo].[HT_Booking_Notes]([Book_No])

    PRINT 'Created HT_Booking_Notes table in HotelNew'
END
ELSE
BEGIN
    PRINT 'HT_Booking_Notes table already exists in HotelNew'
END
GO

-- =============================================================================
-- DOWN MIGRATION (Rollback) - COMMENTED OUT
-- =============================================================================
-- WARNING: Only run this if you need to rollback. Data will be lost!
--
-- USE HotelNew;
-- GO
-- DROP TABLE IF EXISTS [dbo].[HT_Booking_Notes];
-- GO

-- =============================================================================
-- DATA MIGRATION (Optional) - Run this ONCE if migrating from legacy
-- =============================================================================
-- If there are existing notes in the legacy database that need to be migrated,
-- run this script to copy them over. Adjust the server/database names as needed.
--
-- NOTE: This requires a linked server or direct access to both databases.
-- Uncomment and modify as needed:
--
-- INSERT INTO HotelNew.dbo.HT_Booking_Notes (Book_No, Note_Text, Created_At, Updated_At)
-- SELECT Book_No, Note_Text, Created_At, Updated_At
-- FROM [LegacyServerName].db.dbo.HT_Booking_Notes
-- WHERE NOT EXISTS (
--     SELECT 1 FROM HotelNew.dbo.HT_Booking_Notes n
--     WHERE n.Book_No = HT_Booking_Notes.Book_No
--       AND n.Note_Text = HT_Booking_Notes.Note_Text
--       AND n.Created_At = HT_Booking_Notes.Created_At
-- );
-- GO
