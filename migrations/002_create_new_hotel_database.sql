-- Migration: 002_create_new_hotel_database
-- Version: 2.0.0
-- Date: 2026-02-05
-- Description: Creates the HotelNew database with all tables for the new hotel management system

-- ==============================================================================
-- UP MIGRATION
-- ==============================================================================

-- Create the new database (run this separately as sysadmin if needed)
-- USE master
-- GO
-- IF NOT EXISTS (SELECT name FROM sys.databases WHERE name = N'HotelNew')
-- BEGIN
--     CREATE DATABASE HotelNew
-- END
-- GO

-- Switch to the new database
-- USE HotelNew
-- GO

-- ==============================================================================
-- Table: HT_Customers - Customer master data
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Customers] (
        [Cust_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Cust_Code] NVARCHAR(20) NULL,                    -- Optional customer code
        [Cust_Title] NVARCHAR(20) NULL,                   -- Title (Mr., Mrs., etc.)
        [Cust_FirstName] NVARCHAR(100) NOT NULL,          -- First name
        [Cust_LastName] NVARCHAR(100) NULL,               -- Last name
        [Cust_NickName] NVARCHAR(50) NULL,                -- Nickname
        [Cust_IDCard] NVARCHAR(20) NULL,                  -- National ID card
        [Cust_Passport] NVARCHAR(50) NULL,                -- Passport number
        [Cust_Nationality] NVARCHAR(50) NULL,             -- Nationality
        [Cust_Phone] NVARCHAR(20) NULL,                   -- Phone number
        [Cust_Email] NVARCHAR(100) NULL,                  -- Email
        [Cust_Address] NVARCHAR(500) NULL,                -- Address
        [Cust_Company] NVARCHAR(200) NULL,                -- Company name
        [Cust_TaxID] NVARCHAR(20) NULL,                   -- Tax ID
        [Cust_Notes] NVARCHAR(MAX) NULL,                  -- Additional notes
        [Cust_VIP] BIT DEFAULT 0,                         -- VIP flag
        [Cust_Blacklist] BIT DEFAULT 0,                   -- Blacklist flag
        [Cust_Created_At] DATETIME DEFAULT GETDATE(),     -- Created timestamp
        [Cust_Updated_At] DATETIME DEFAULT GETDATE(),     -- Updated timestamp
        [Cust_Created_By] NVARCHAR(50) NULL,              -- Created by user
        [Cust_Updated_By] NVARCHAR(50) NULL,              -- Updated by user
        [Cust_Active] BIT DEFAULT 1                       -- Soft delete flag
    )

    -- Index on common search fields
    CREATE INDEX IX_HT_Customers_Name ON [dbo].[HT_Customers] ([Cust_FirstName], [Cust_LastName])
    CREATE INDEX IX_HT_Customers_Phone ON [dbo].[HT_Customers] ([Cust_Phone])
    CREATE INDEX IX_HT_Customers_IDCard ON [dbo].[HT_Customers] ([Cust_IDCard])
    CREATE INDEX IX_HT_Customers_Passport ON [dbo].[HT_Customers] ([Cust_Passport])
END
GO

-- ==============================================================================
-- Table: HT_Room_Types - Room type definitions
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Room_Types]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Room_Types] (
        [Type_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Type_Code] NVARCHAR(20) NOT NULL UNIQUE,         -- Type code (STD, DLX, SUI, etc.)
        [Type_Name] NVARCHAR(100) NOT NULL,               -- Display name
        [Type_Name_En] NVARCHAR(100) NULL,                -- English name
        [Type_Description] NVARCHAR(500) NULL,            -- Description
        [Type_Base_Price] DECIMAL(10,2) DEFAULT 0,        -- Base price per night
        [Type_Max_Guests] INT DEFAULT 2,                  -- Maximum guests
        [Type_Bed_Type] NVARCHAR(50) NULL,                -- Bed type (Single, Double, Twin, etc.)
        [Type_Size_SQM] DECIMAL(6,2) NULL,                -- Room size in square meters
        [Type_Amenities] NVARCHAR(MAX) NULL,              -- JSON array of amenities
        [Type_Sort_Order] INT DEFAULT 0,                  -- Display sort order
        [Type_Active] BIT DEFAULT 1,                      -- Active flag
        [Type_Created_At] DATETIME DEFAULT GETDATE(),
        [Type_Updated_At] DATETIME DEFAULT GETDATE()
    )
END
GO

-- ==============================================================================
-- Table: HT_Rooms_New - Room inventory
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Rooms_New] (
        [Room_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Room_No] NVARCHAR(10) NOT NULL UNIQUE,           -- Room number (101, 102, etc.)
        [Room_Type_ID] INT NULL,                          -- FK to HT_Room_Types
        [Room_Floor] NVARCHAR(10) NULL,                   -- Floor number
        [Room_Building] NVARCHAR(50) NULL,                -- Building name
        [Room_View] NVARCHAR(50) NULL,                    -- View type (Sea, Garden, City, etc.)
        [Room_Status] NVARCHAR(20) DEFAULT 'available',   -- available, occupied, maintenance, cleaning
        [Room_Notes] NVARCHAR(500) NULL,                  -- Room-specific notes
        [Room_Features] NVARCHAR(MAX) NULL,               -- JSON array of special features
        [Room_Active] BIT DEFAULT 1,                      -- Active flag (soft delete)
        [Room_Created_At] DATETIME DEFAULT GETDATE(),
        [Room_Updated_At] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_Rooms_Type FOREIGN KEY ([Room_Type_ID])
            REFERENCES [dbo].[HT_Room_Types]([Type_ID])
    )

    CREATE INDEX IX_HT_Rooms_Status ON [dbo].[HT_Rooms_New] ([Room_Status])
    CREATE INDEX IX_HT_Rooms_Type ON [dbo].[HT_Rooms_New] ([Room_Type_ID])
END
GO

-- ==============================================================================
-- Table: HT_Bookings - Booking records
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Bookings] (
        [Book_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Book_No] NVARCHAR(20) NOT NULL UNIQUE,           -- Booking reference number
        [Book_Date] DATETIME DEFAULT GETDATE(),           -- Date booking was made
        [Book_Cust_ID] INT NOT NULL,                      -- FK to HT_Customers
        [Book_CheckIn] DATE NOT NULL,                     -- Expected check-in date
        [Book_CheckOut] DATE NOT NULL,                    -- Expected check-out date
        [Book_Adults] INT DEFAULT 1,                      -- Number of adults
        [Book_Children] INT DEFAULT 0,                    -- Number of children
        [Book_Nights] AS DATEDIFF(DAY, [Book_CheckIn], [Book_CheckOut]) PERSISTED, -- Computed nights
        [Book_Status] NVARCHAR(20) DEFAULT 'confirmed',   -- confirmed, checked_in, checked_out, cancelled, no_show
        [Book_Source] NVARCHAR(50) NULL,                  -- Booking source (Walk-in, Phone, Online, OTA, etc.)
        [Book_Channel] NVARCHAR(50) NULL,                 -- Specific channel (Agoda, Booking.com, etc.)
        [Book_Total_Price] DECIMAL(12,2) DEFAULT 0,       -- Total booking price
        [Book_Deposit] DECIMAL(12,2) DEFAULT 0,           -- Deposit amount
        [Book_Deposit_Date] DATETIME NULL,                -- Deposit payment date
        [Book_Special_Requests] NVARCHAR(MAX) NULL,       -- Special requests
        [Book_Internal_Notes] NVARCHAR(MAX) NULL,         -- Internal staff notes
        [Book_Cancelled_At] DATETIME NULL,                -- Cancellation timestamp
        [Book_Cancel_Reason] NVARCHAR(500) NULL,          -- Cancellation reason
        [Book_Created_At] DATETIME DEFAULT GETDATE(),
        [Book_Updated_At] DATETIME DEFAULT GETDATE(),
        [Book_Created_By] NVARCHAR(50) NULL,
        [Book_Updated_By] NVARCHAR(50) NULL,

        CONSTRAINT FK_HT_Bookings_Customer FOREIGN KEY ([Book_Cust_ID])
            REFERENCES [dbo].[HT_Customers]([Cust_ID]),
        CONSTRAINT CK_HT_Bookings_Dates CHECK ([Book_CheckOut] > [Book_CheckIn])
    )

    CREATE INDEX IX_HT_Bookings_Customer ON [dbo].[HT_Bookings] ([Book_Cust_ID])
    CREATE INDEX IX_HT_Bookings_CheckIn ON [dbo].[HT_Bookings] ([Book_CheckIn])
    CREATE INDEX IX_HT_Bookings_CheckOut ON [dbo].[HT_Bookings] ([Book_CheckOut])
    CREATE INDEX IX_HT_Bookings_Status ON [dbo].[HT_Bookings] ([Book_Status])
    CREATE INDEX IX_HT_Bookings_DateRange ON [dbo].[HT_Bookings] ([Book_CheckIn], [Book_CheckOut])
END
GO

-- ==============================================================================
-- Table: HT_Booking_Rooms - Junction table for booking-room assignments
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Booking_Rooms]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Booking_Rooms] (
        [BR_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [BR_Book_ID] INT NOT NULL,                        -- FK to HT_Bookings
        [BR_Room_ID] INT NOT NULL,                        -- FK to HT_Rooms_New
        [BR_Room_Type_ID] INT NULL,                       -- FK to HT_Room_Types (requested type)
        [BR_Rate_Per_Night] DECIMAL(10,2) DEFAULT 0,      -- Rate per night for this room
        [BR_Assigned_At] DATETIME NULL,                   -- When room was assigned
        [BR_Notes] NVARCHAR(500) NULL,                    -- Room-specific booking notes

        CONSTRAINT FK_HT_BR_Booking FOREIGN KEY ([BR_Book_ID])
            REFERENCES [dbo].[HT_Bookings]([Book_ID]) ON DELETE CASCADE,
        CONSTRAINT FK_HT_BR_Room FOREIGN KEY ([BR_Room_ID])
            REFERENCES [dbo].[HT_Rooms_New]([Room_ID]),
        CONSTRAINT FK_HT_BR_RoomType FOREIGN KEY ([BR_Room_Type_ID])
            REFERENCES [dbo].[HT_Room_Types]([Type_ID]),
        CONSTRAINT UQ_HT_BR_BookRoom UNIQUE ([BR_Book_ID], [BR_Room_ID])
    )

    CREATE INDEX IX_HT_BR_Room ON [dbo].[HT_Booking_Rooms] ([BR_Room_ID])
END
GO

-- ==============================================================================
-- Table: HT_CheckIns - Check-in records
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_CheckIns] (
        [Cin_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Cin_No] NVARCHAR(20) NOT NULL UNIQUE,            -- Check-in reference number
        [Cin_Book_ID] INT NULL,                           -- FK to HT_Bookings (optional, for walk-ins)
        [Cin_Cust_ID] INT NOT NULL,                       -- FK to HT_Customers
        [Cin_Room_ID] INT NOT NULL,                       -- FK to HT_Rooms_New
        [Cin_CheckIn_Time] DATETIME NOT NULL,             -- Actual check-in timestamp
        [Cin_CheckOut_Time] DATETIME NULL,                -- Actual check-out timestamp
        [Cin_Expected_Out] DATE NOT NULL,                 -- Expected check-out date
        [Cin_Adults] INT DEFAULT 1,                       -- Number of adults
        [Cin_Children] INT DEFAULT 0,                     -- Number of children
        [Cin_Status] NVARCHAR(20) DEFAULT 'active',       -- active, checked_out, extended
        [Cin_Rate_Per_Night] DECIMAL(10,2) DEFAULT 0,     -- Rate per night
        [Cin_Total_Amount] DECIMAL(12,2) DEFAULT 0,       -- Total amount
        [Cin_Paid_Amount] DECIMAL(12,2) DEFAULT 0,        -- Amount paid
        [Cin_Payment_Method] NVARCHAR(50) NULL,           -- Cash, Credit Card, Transfer, etc.
        [Cin_Key_Card_No] NVARCHAR(20) NULL,              -- Key card number issued
        [Cin_Vehicle_Plate] NVARCHAR(20) NULL,            -- Vehicle plate number
        [Cin_Notes] NVARCHAR(MAX) NULL,                   -- Check-in notes
        [Cin_Created_At] DATETIME DEFAULT GETDATE(),
        [Cin_Updated_At] DATETIME DEFAULT GETDATE(),
        [Cin_Created_By] NVARCHAR(50) NULL,
        [Cin_Updated_By] NVARCHAR(50) NULL,

        CONSTRAINT FK_HT_CheckIns_Booking FOREIGN KEY ([Cin_Book_ID])
            REFERENCES [dbo].[HT_Bookings]([Book_ID]),
        CONSTRAINT FK_HT_CheckIns_Customer FOREIGN KEY ([Cin_Cust_ID])
            REFERENCES [dbo].[HT_Customers]([Cust_ID]),
        CONSTRAINT FK_HT_CheckIns_Room FOREIGN KEY ([Cin_Room_ID])
            REFERENCES [dbo].[HT_Rooms_New]([Room_ID])
    )

    CREATE INDEX IX_HT_CheckIns_Booking ON [dbo].[HT_CheckIns] ([Cin_Book_ID])
    CREATE INDEX IX_HT_CheckIns_Customer ON [dbo].[HT_CheckIns] ([Cin_Cust_ID])
    CREATE INDEX IX_HT_CheckIns_Room ON [dbo].[HT_CheckIns] ([Cin_Room_ID])
    CREATE INDEX IX_HT_CheckIns_Status ON [dbo].[HT_CheckIns] ([Cin_Status])
    CREATE INDEX IX_HT_CheckIns_CheckIn ON [dbo].[HT_CheckIns] ([Cin_CheckIn_Time])
    CREATE INDEX IX_HT_CheckIns_ExpectedOut ON [dbo].[HT_CheckIns] ([Cin_Expected_Out])
END
GO

-- ==============================================================================
-- Table: HT_Guest_Registry - Guest registration (multiple guests per check-in)
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Guest_Registry]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Guest_Registry] (
        [Guest_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Guest_Cin_ID] INT NOT NULL,                      -- FK to HT_CheckIns
        [Guest_Cust_ID] INT NULL,                         -- FK to HT_Customers (if registered customer)
        [Guest_FirstName] NVARCHAR(100) NOT NULL,         -- Guest first name
        [Guest_LastName] NVARCHAR(100) NULL,              -- Guest last name
        [Guest_IDCard] NVARCHAR(20) NULL,                 -- National ID
        [Guest_Passport] NVARCHAR(50) NULL,               -- Passport number
        [Guest_Nationality] NVARCHAR(50) NULL,            -- Nationality
        [Guest_Is_Primary] BIT DEFAULT 0,                 -- Primary guest flag
        [Guest_Created_At] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_GuestReg_CheckIn FOREIGN KEY ([Guest_Cin_ID])
            REFERENCES [dbo].[HT_CheckIns]([Cin_ID]) ON DELETE CASCADE,
        CONSTRAINT FK_HT_GuestReg_Customer FOREIGN KEY ([Guest_Cust_ID])
            REFERENCES [dbo].[HT_Customers]([Cust_ID])
    )

    CREATE INDEX IX_HT_GuestReg_CheckIn ON [dbo].[HT_Guest_Registry] ([Guest_Cin_ID])
END
GO

-- ==============================================================================
-- Table: HT_Rates - Room rate configurations
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Rates] (
        [Rate_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Rate_Room_Type_ID] INT NOT NULL,                 -- FK to HT_Room_Types
        [Rate_Name] NVARCHAR(100) NOT NULL,               -- Rate name (Standard, Rack, Corporate, etc.)
        [Rate_Code] NVARCHAR(20) NULL,                    -- Rate code
        [Rate_Price] DECIMAL(10,2) NOT NULL,              -- Price per night
        [Rate_Start_Date] DATE NULL,                      -- Valid from date
        [Rate_End_Date] DATE NULL,                        -- Valid until date
        [Rate_Day_Of_Week] NVARCHAR(50) NULL,             -- Specific days (Mon,Tue,Wed,etc.)
        [Rate_Min_Nights] INT DEFAULT 1,                  -- Minimum nights
        [Rate_Active] BIT DEFAULT 1,
        [Rate_Created_At] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_Rates_RoomType FOREIGN KEY ([Rate_Room_Type_ID])
            REFERENCES [dbo].[HT_Room_Types]([Type_ID])
    )

    CREATE INDEX IX_HT_Rates_RoomType ON [dbo].[HT_Rates] ([Rate_Room_Type_ID])
    CREATE INDEX IX_HT_Rates_Dates ON [dbo].[HT_Rates] ([Rate_Start_Date], [Rate_End_Date])
END
GO

-- ==============================================================================
-- Table: HT_Settings - System settings
-- ==============================================================================
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Settings]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Settings] (
        [Setting_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Setting_Key] NVARCHAR(100) NOT NULL UNIQUE,      -- Setting key
        [Setting_Value] NVARCHAR(MAX) NULL,               -- Setting value (JSON supported)
        [Setting_Type] NVARCHAR(20) DEFAULT 'string',     -- string, number, boolean, json
        [Setting_Description] NVARCHAR(500) NULL,         -- Description
        [Setting_Updated_At] DATETIME DEFAULT GETDATE(),
        [Setting_Updated_By] NVARCHAR(50) NULL
    )

    -- Insert default settings
    INSERT INTO [dbo].[HT_Settings] ([Setting_Key], [Setting_Value], [Setting_Type], [Setting_Description])
    VALUES
        ('hotel.name', 'The HF Hotel', 'string', 'Hotel name'),
        ('hotel.check_in_time', '14:00', 'string', 'Default check-in time'),
        ('hotel.check_out_time', '12:00', 'string', 'Default check-out time'),
        ('booking.prefix', 'BK', 'string', 'Booking number prefix'),
        ('checkin.prefix', 'CI', 'string', 'Check-in number prefix'),
        ('customer.prefix', 'CU', 'string', 'Customer code prefix')
END
GO

-- ==============================================================================
-- Sequence Generators (for generating reference numbers)
-- ==============================================================================

-- Create sequence for booking numbers
IF NOT EXISTS (SELECT * FROM sys.sequences WHERE name = 'SQ_Booking_No')
BEGIN
    CREATE SEQUENCE [dbo].[SQ_Booking_No]
        AS INT
        START WITH 1
        INCREMENT BY 1
        MINVALUE 1
        NO MAXVALUE
        NO CYCLE
        CACHE 10
END
GO

-- Create sequence for check-in numbers
IF NOT EXISTS (SELECT * FROM sys.sequences WHERE name = 'SQ_CheckIn_No')
BEGIN
    CREATE SEQUENCE [dbo].[SQ_CheckIn_No]
        AS INT
        START WITH 1
        INCREMENT BY 1
        MINVALUE 1
        NO MAXVALUE
        NO CYCLE
        CACHE 10
END
GO

-- Create sequence for customer codes
IF NOT EXISTS (SELECT * FROM sys.sequences WHERE name = 'SQ_Customer_Code')
BEGIN
    CREATE SEQUENCE [dbo].[SQ_Customer_Code]
        AS INT
        START WITH 1
        INCREMENT BY 1
        MINVALUE 1
        NO MAXVALUE
        NO CYCLE
        CACHE 10
END
GO

-- ==============================================================================
-- Stored Procedures for generating reference numbers
-- ==============================================================================

-- Generate next booking number
IF EXISTS (SELECT * FROM sys.objects WHERE type = 'P' AND name = 'SP_Generate_Booking_No')
    DROP PROCEDURE [dbo].[SP_Generate_Booking_No]
GO
CREATE PROCEDURE [dbo].[SP_Generate_Booking_No]
    @BookingNo NVARCHAR(20) OUTPUT
AS
BEGIN
    DECLARE @NextVal INT
    DECLARE @Prefix NVARCHAR(10)
    DECLARE @YearMonth NVARCHAR(6)

    SELECT @Prefix = [Setting_Value] FROM [dbo].[HT_Settings] WHERE [Setting_Key] = 'booking.prefix'
    SET @Prefix = ISNULL(@Prefix, 'BK')
    SET @YearMonth = FORMAT(GETDATE(), 'yyMM')

    SET @NextVal = NEXT VALUE FOR [dbo].[SQ_Booking_No]
    SET @BookingNo = @Prefix + @YearMonth + RIGHT('0000' + CAST(@NextVal AS NVARCHAR), 4)
END
GO

-- Generate next check-in number
IF EXISTS (SELECT * FROM sys.objects WHERE type = 'P' AND name = 'SP_Generate_CheckIn_No')
    DROP PROCEDURE [dbo].[SP_Generate_CheckIn_No]
GO
CREATE PROCEDURE [dbo].[SP_Generate_CheckIn_No]
    @CheckInNo NVARCHAR(20) OUTPUT
AS
BEGIN
    DECLARE @NextVal INT
    DECLARE @Prefix NVARCHAR(10)
    DECLARE @YearMonth NVARCHAR(6)

    SELECT @Prefix = [Setting_Value] FROM [dbo].[HT_Settings] WHERE [Setting_Key] = 'checkin.prefix'
    SET @Prefix = ISNULL(@Prefix, 'CI')
    SET @YearMonth = FORMAT(GETDATE(), 'yyMM')

    SET @NextVal = NEXT VALUE FOR [dbo].[SQ_CheckIn_No]
    SET @CheckInNo = @Prefix + @YearMonth + RIGHT('0000' + CAST(@NextVal AS NVARCHAR), 4)
END
GO

-- ==============================================================================
-- DOWN MIGRATION (Rollback - commented out for safety)
-- ==============================================================================

/*
-- WARNING: This will delete all data!

DROP PROCEDURE IF EXISTS [dbo].[SP_Generate_CheckIn_No]
DROP PROCEDURE IF EXISTS [dbo].[SP_Generate_Booking_No]

DROP SEQUENCE IF EXISTS [dbo].[SQ_Customer_Code]
DROP SEQUENCE IF EXISTS [dbo].[SQ_CheckIn_No]
DROP SEQUENCE IF EXISTS [dbo].[SQ_Booking_No]

DROP TABLE IF EXISTS [dbo].[HT_Settings]
DROP TABLE IF EXISTS [dbo].[HT_Rates]
DROP TABLE IF EXISTS [dbo].[HT_Guest_Registry]
DROP TABLE IF EXISTS [dbo].[HT_CheckIns]
DROP TABLE IF EXISTS [dbo].[HT_Booking_Rooms]
DROP TABLE IF EXISTS [dbo].[HT_Bookings]
DROP TABLE IF EXISTS [dbo].[HT_Rooms_New]
DROP TABLE IF EXISTS [dbo].[HT_Room_Types]
DROP TABLE IF EXISTS [dbo].[HT_Customers]
*/
