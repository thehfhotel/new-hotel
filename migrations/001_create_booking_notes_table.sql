-- Migration: 001_create_booking_notes_table
-- Description: Creates the HT_Booking_Notes table for storing booking annotations
-- Version: 1.16.0
-- Date: 2026-01-29
-- Author: new-hotel web app
--
-- Dependencies: None (new table)
-- Breaking Changes: None (additive only)
--
-- This table stores notes/annotations that staff can add to bookings.
-- It references Book_No from View_Booking_Ds but does not have a foreign key
-- constraint to allow flexibility with the legacy system.

-- ============================================================================
-- UP MIGRATION
-- ============================================================================

-- Create the booking notes table if it doesn't exist
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='HT_Booking_Notes' AND xtype='U')
BEGIN
    CREATE TABLE HT_Booking_Notes (
        Note_ID INT IDENTITY(1,1) PRIMARY KEY,
        Book_No NVARCHAR(50) NOT NULL,           -- References View_Booking_Ds.Book_No
        Note_Text NVARCHAR(MAX) NOT NULL,        -- The note content
        Created_At DATETIME DEFAULT GETDATE(),   -- When the note was created
        Updated_At DATETIME DEFAULT GETDATE()    -- When the note was last updated
    );

    PRINT 'Created table HT_Booking_Notes';
END
ELSE
BEGIN
    PRINT 'Table HT_Booking_Notes already exists, skipping creation';
END
GO

-- Create index on Book_No for faster lookups
IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name='IX_Booking_Notes_BookNo' AND object_id = OBJECT_ID('HT_Booking_Notes'))
BEGIN
    CREATE INDEX IX_Booking_Notes_BookNo ON HT_Booking_Notes(Book_No);
    PRINT 'Created index IX_Booking_Notes_BookNo';
END
ELSE
BEGIN
    PRINT 'Index IX_Booking_Notes_BookNo already exists, skipping creation';
END
GO

-- ============================================================================
-- DOWN MIGRATION (ROLLBACK)
-- Uncomment and run these statements to undo this migration
-- ============================================================================

-- WARNING: This will DELETE ALL NOTES DATA!
--
-- DROP INDEX IF EXISTS IX_Booking_Notes_BookNo ON HT_Booking_Notes;
-- DROP TABLE IF EXISTS HT_Booking_Notes;
-- PRINT 'Dropped table HT_Booking_Notes and index IX_Booking_Notes_BookNo';
