-- =============================================================================
-- HotelNew Database Initialization Script
-- =============================================================================
-- This script creates the HotelNew database and all required tables.
-- It combines migrations 002, 003, and 004 into a single bootstrap script.
--
-- Usage: This script runs automatically when the SQL Server container starts
-- for the first time (via docker-entrypoint-initdb.d volume mount).
--
-- Note: SQL Server's Docker image doesn't auto-run scripts from initdb.d.
-- You need to run this manually on first deploy:
--   docker exec -it new-hotel-db /opt/mssql-tools18/bin/sqlcmd -S localhost -U sa -P "***REMOVED***" -C -i /docker-entrypoint-initdb.d/init-hotelnew.sql
-- =============================================================================

-- Create database if not exists
IF NOT EXISTS (SELECT name FROM sys.databases WHERE name = N'HotelNew')
BEGIN
    CREATE DATABASE HotelNew;
END
GO

USE HotelNew;
GO

-- Required for tables with computed columns and indexes
SET QUOTED_IDENTIFIER ON;
SET ANSI_NULLS ON;
GO

-- =============================================================================
-- Migration 002: Core Tables
-- =============================================================================

-- HT_Customers - Customer master data
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Customers] (
        [Cust_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Cust_Code] NVARCHAR(20) NULL,
        [Cust_Title] NVARCHAR(20) NULL,
        [Cust_FirstName] NVARCHAR(100) NOT NULL,
        [Cust_LastName] NVARCHAR(100) NULL,
        [Cust_NickName] NVARCHAR(50) NULL,
        [Cust_IDCard] NVARCHAR(20) NULL,
        [Cust_Passport] NVARCHAR(50) NULL,
        [Cust_Nationality] NVARCHAR(50) NULL,
        [Cust_Phone] NVARCHAR(20) NULL,
        [Cust_Email] NVARCHAR(100) NULL,
        [Cust_Address] NVARCHAR(500) NULL,
        [Cust_Company] NVARCHAR(200) NULL,
        [Cust_TaxID] NVARCHAR(20) NULL,
        [Cust_Notes] NVARCHAR(MAX) NULL,
        [Cust_Type] NVARCHAR(50) NULL,                   -- Customer type (Individual, Corporate, etc.)
        [Cust_VIP] BIT DEFAULT 0,
        [Cust_Blacklist] BIT DEFAULT 0,
        [Created_At] DATETIME DEFAULT GETDATE(),
        [Updated_At] DATETIME DEFAULT GETDATE(),
        [Cust_Created_By] NVARCHAR(50) NULL,
        [Cust_Updated_By] NVARCHAR(50) NULL,
        [Cust_Active] BIT DEFAULT 1
    )

    CREATE INDEX IX_HT_Customers_Name ON [dbo].[HT_Customers] ([Cust_FirstName], [Cust_LastName])
    CREATE INDEX IX_HT_Customers_Phone ON [dbo].[HT_Customers] ([Cust_Phone])
    CREATE INDEX IX_HT_Customers_IDCard ON [dbo].[HT_Customers] ([Cust_IDCard])
    CREATE INDEX IX_HT_Customers_Passport ON [dbo].[HT_Customers] ([Cust_Passport])
END
GO

-- HT_Room_Types - Room type definitions
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Room_Types]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Room_Types] (
        [Type_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Type_Code] NVARCHAR(20) NOT NULL UNIQUE,
        [Type_Name] NVARCHAR(100) NOT NULL,
        [Type_Name_En] NVARCHAR(100) NULL,
        [Type_Description] NVARCHAR(500) NULL,
        [Type_Base_Price] DECIMAL(10,2) DEFAULT 0,
        [Type_Max_Guests] INT DEFAULT 2,
        [Type_Bed_Type] NVARCHAR(50) NULL,
        [Type_Size_SQM] DECIMAL(6,2) NULL,
        [Type_Amenities] NVARCHAR(MAX) NULL,
        [Type_Sort_Order] INT DEFAULT 0,
        [Type_Active] BIT DEFAULT 1,
        [Type_Created_At] DATETIME DEFAULT GETDATE(),
        [Type_Updated_At] DATETIME DEFAULT GETDATE()
    )
END
GO

-- HT_Rooms_New - Room inventory
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Rooms_New] (
        [Room_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Room_No] NVARCHAR(10) NOT NULL UNIQUE,
        [Room_Type_ID] INT NULL,
        [Room_Floor] NVARCHAR(10) NULL,
        [Room_Building] NVARCHAR(50) NULL,
        [Room_View] NVARCHAR(50) NULL,
        [Room_Status] NVARCHAR(20) DEFAULT 'available',
        [Room_Clean] BIT DEFAULT 1,                      -- Room cleanliness status
        [Room_Maintenance] BIT DEFAULT 0,                -- Room maintenance status
        [Room_Notes] NVARCHAR(500) NULL,
        [Room_Features] NVARCHAR(MAX) NULL,
        [Room_Active] BIT DEFAULT 1,
        [Created_At] DATETIME DEFAULT GETDATE(),
        [Updated_At] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_Rooms_Type FOREIGN KEY ([Room_Type_ID])
            REFERENCES [dbo].[HT_Room_Types]([Type_ID])
    )

    CREATE INDEX IX_HT_Rooms_Status ON [dbo].[HT_Rooms_New] ([Room_Status])
    CREATE INDEX IX_HT_Rooms_Type ON [dbo].[HT_Rooms_New] ([Room_Type_ID])
END
GO

-- HT_Bookings - Booking records
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Bookings] (
        [Book_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Book_No] NVARCHAR(20) NOT NULL UNIQUE,
        [Book_Date] DATETIME DEFAULT GETDATE(),
        [Book_Cust_ID] INT NOT NULL,
        [Book_CheckIn] DATE NOT NULL,
        [Book_CheckOut] DATE NOT NULL,
        [Book_Adults] INT DEFAULT 1,
        [Book_Children] INT DEFAULT 0,
        [Book_Nights] AS DATEDIFF(DAY, [Book_CheckIn], [Book_CheckOut]) PERSISTED,
        [Book_Status] NVARCHAR(20) DEFAULT 'confirmed',
        [Book_Source] NVARCHAR(50) NULL,
        [Book_Channel] NVARCHAR(50) NULL,
        [Book_Total_Amount] DECIMAL(12,2) DEFAULT 0,
        [Book_Deposit] DECIMAL(12,2) DEFAULT 0,
        [Book_Deposit_Date] DATETIME NULL,
        [Book_Special_Requests] NVARCHAR(MAX) NULL,
        [Book_Internal_Notes] NVARCHAR(MAX) NULL,
        [Book_Cancelled_At] DATETIME NULL,
        [Book_Cancel_Reason] NVARCHAR(500) NULL,
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

-- HT_Booking_Rooms - Junction table for booking-room assignments
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Booking_Rooms]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Booking_Rooms] (
        [BR_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [BR_Book_ID] INT NOT NULL,
        [BR_Room_ID] INT NOT NULL,
        [BR_Room_Type_ID] INT NULL,
        [BR_Rate_Per_Night] DECIMAL(10,2) DEFAULT 0,
        [BR_Assigned_At] DATETIME NULL,
        [BR_Notes] NVARCHAR(500) NULL,

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

-- HT_CheckIns - Check-in records
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_CheckIns] (
        [Cin_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Cin_No] NVARCHAR(20) NOT NULL UNIQUE,
        [Cin_Book_ID] INT NULL,
        [Cin_Cust_ID] INT NOT NULL,
        [Cin_Room_ID] INT NOT NULL,
        [Cin_CheckIn_Time] DATETIME NOT NULL,
        [Cin_CheckOut_Time] DATETIME NULL,
        [Cin_Expected_Out] DATE NOT NULL,
        [Cin_Adults] INT DEFAULT 1,
        [Cin_Children] INT DEFAULT 0,
        [Cin_Status] NVARCHAR(20) DEFAULT 'active',
        [Cin_Rate_Per_Night] DECIMAL(10,2) DEFAULT 0,
        [Cin_Total_Amount] DECIMAL(12,2) DEFAULT 0,
        [Cin_Paid_Amount] DECIMAL(12,2) DEFAULT 0,
        [Cin_Payment_Method] NVARCHAR(50) NULL,
        [Cin_Key_Card_No] NVARCHAR(20) NULL,
        [Cin_Vehicle_Plate] NVARCHAR(20) NULL,
        [Cin_Notes] NVARCHAR(MAX) NULL,
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

-- HT_Guest_Registry - Guest registration (multiple guests per check-in)
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Guest_Registry]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Guest_Registry] (
        [Guest_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Guest_Cin_ID] INT NOT NULL,
        [Guest_Cust_ID] INT NULL,
        [Guest_FirstName] NVARCHAR(100) NOT NULL,
        [Guest_LastName] NVARCHAR(100) NULL,
        [Guest_IDCard] NVARCHAR(20) NULL,
        [Guest_Passport] NVARCHAR(50) NULL,
        [Guest_Nationality] NVARCHAR(50) NULL,
        [Guest_Is_Primary] BIT DEFAULT 0,
        [Guest_Created_At] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_GuestReg_CheckIn FOREIGN KEY ([Guest_Cin_ID])
            REFERENCES [dbo].[HT_CheckIns]([Cin_ID]) ON DELETE CASCADE,
        CONSTRAINT FK_HT_GuestReg_Customer FOREIGN KEY ([Guest_Cust_ID])
            REFERENCES [dbo].[HT_Customers]([Cust_ID])
    )

    CREATE INDEX IX_HT_GuestReg_CheckIn ON [dbo].[HT_Guest_Registry] ([Guest_Cin_ID])
END
GO

-- HT_Rates - Room rate configurations
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rates]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Rates] (
        [Rate_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Rate_Room_Type_ID] INT NULL,
        [Rate_Name] NVARCHAR(100) NOT NULL,
        [Rate_Code] NVARCHAR(20) NULL,
        [Rate_Price] DECIMAL(10,2) NOT NULL,
        [Rate_Type] VARCHAR(20) DEFAULT 'fixed' NOT NULL,
        [Rate_Value] DECIMAL(10,2) NULL,
        [Rate_Valid_From] DATE NULL,
        [Rate_Valid_To] DATE NULL,
        [Rate_Days_Of_Week] NVARCHAR(50) NULL,
        [Rate_Min_Nights] INT DEFAULT 1,
        [Rate_Active] BIT DEFAULT 1,
        [Rate_Created] DATETIME DEFAULT GETDATE(),
        [Rate_Updated] DATETIME DEFAULT GETDATE(),

        CONSTRAINT FK_HT_Rates_RoomType FOREIGN KEY ([Rate_Room_Type_ID])
            REFERENCES [dbo].[HT_Room_Types]([Type_ID])
    )

    CREATE INDEX IX_HT_Rates_RoomType ON [dbo].[HT_Rates] ([Rate_Room_Type_ID])
    CREATE INDEX IX_HT_Rates_ValidDates ON [dbo].[HT_Rates] ([Rate_Valid_From], [Rate_Valid_To])
END
GO

-- HT_Settings - System settings
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[HT_Settings]') AND type = 'U')
BEGIN
    CREATE TABLE [dbo].[HT_Settings] (
        [Setting_ID] INT IDENTITY(1,1) PRIMARY KEY,
        [Setting_Key] NVARCHAR(100) NOT NULL UNIQUE,
        [Setting_Value] NVARCHAR(MAX) NULL,
        [Setting_Type] NVARCHAR(20) DEFAULT 'string',
        [Setting_Description] NVARCHAR(500) NULL,
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

-- =============================================================================
-- Sequences for generating reference numbers
-- =============================================================================

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

-- =============================================================================
-- Stored Procedures
-- =============================================================================

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

-- =============================================================================
-- Migration 004: Inventory Tables
-- =============================================================================

-- Inventory Categories
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'HT_Inventory_Categories')
BEGIN
    CREATE TABLE HT_Inventory_Categories (
        Cat_ID INT IDENTITY(1,1) PRIMARY KEY,
        Cat_Name NVARCHAR(100) NOT NULL,
        Cat_Description NVARCHAR(255),
        Cat_Active BIT DEFAULT 1,
        Cat_Created DATETIME DEFAULT GETDATE()
    );

    -- Insert default categories
    INSERT INTO HT_Inventory_Categories (Cat_Name, Cat_Description) VALUES
    ('Minibar', N'เครื่องดื่มและของว่างในมินิบาร์'),
    ('Amenities', N'อุปกรณ์อำนวยความสะดวก'),
    ('Linens', N'ผ้าและเครื่องนอน'),
    ('Equipment', N'อุปกรณ์ในห้องพัก');
END
GO

-- Inventory Items
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'HT_Inventory_Items')
BEGIN
    CREATE TABLE HT_Inventory_Items (
        Item_ID INT IDENTITY(1,1) PRIMARY KEY,
        Item_Code NVARCHAR(50) NOT NULL UNIQUE,
        Item_Name NVARCHAR(200) NOT NULL,
        Item_Category_ID INT FOREIGN KEY REFERENCES HT_Inventory_Categories(Cat_ID),
        Item_Unit NVARCHAR(50) NOT NULL,
        Item_Min_Stock INT DEFAULT 0,
        Item_Current_Stock INT DEFAULT 0,
        Item_Cost DECIMAL(10,2),
        Item_Active BIT DEFAULT 1,
        Item_Created DATETIME DEFAULT GETDATE(),
        Item_Updated DATETIME
    );
END
GO

-- Room Inventory
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'HT_Room_Inventory')
BEGIN
    CREATE TABLE HT_Room_Inventory (
        RI_ID INT IDENTITY(1,1) PRIMARY KEY,
        RI_Room_ID INT NOT NULL,
        RI_Item_ID INT FOREIGN KEY REFERENCES HT_Inventory_Items(Item_ID),
        RI_Quantity INT DEFAULT 1,
        RI_Last_Checked DATETIME
    );
END
GO

-- Inventory Transactions
IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'HT_Inventory_Transactions')
BEGIN
    CREATE TABLE HT_Inventory_Transactions (
        Trans_ID INT IDENTITY(1,1) PRIMARY KEY,
        Trans_Item_ID INT FOREIGN KEY REFERENCES HT_Inventory_Items(Item_ID),
        Trans_Type VARCHAR(20) NOT NULL,
        Trans_Quantity INT NOT NULL,
        Trans_Room_ID INT,
        Trans_Notes NVARCHAR(500),
        Trans_Date DATETIME DEFAULT GETDATE(),
        Trans_By NVARCHAR(100)
    );
END
GO

-- Create indexes for inventory tables
IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Inventory_Items_Category')
    CREATE INDEX IX_HT_Inventory_Items_Category ON HT_Inventory_Items(Item_Category_ID);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Inventory_Items_Code')
    CREATE INDEX IX_HT_Inventory_Items_Code ON HT_Inventory_Items(Item_Code);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Room_Inventory_Room')
    CREATE INDEX IX_HT_Room_Inventory_Room ON HT_Room_Inventory(RI_Room_ID);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Room_Inventory_Item')
    CREATE INDEX IX_HT_Room_Inventory_Item ON HT_Room_Inventory(RI_Item_ID);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Inventory_Transactions_Item')
    CREATE INDEX IX_HT_Inventory_Transactions_Item ON HT_Inventory_Transactions(Trans_Item_ID);
GO

IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name = 'IX_HT_Inventory_Transactions_Date')
    CREATE INDEX IX_HT_Inventory_Transactions_Date ON HT_Inventory_Transactions(Trans_Date);
GO

-- =============================================================================
-- Schema Updates (for existing databases)
-- =============================================================================

-- Add Cust_Type column if it doesn't exist
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND name = 'Cust_Type')
BEGIN
    ALTER TABLE [dbo].[HT_Customers] ADD [Cust_Type] NVARCHAR(50) NULL;
END
GO

-- Add Room_Clean column if it doesn't exist
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Clean')
BEGIN
    ALTER TABLE [dbo].[HT_Rooms_New] ADD [Room_Clean] BIT DEFAULT 1;
END
GO

-- =============================================================================
-- Column Renames (backend expects Created_At/Updated_At without table prefix)
-- =============================================================================

-- HT_Customers: Rename Cust_Created_At -> Created_At, Cust_Updated_At -> Updated_At
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND name = 'Cust_Created_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND name = 'Created_At')
BEGIN
    EXEC sp_rename 'HT_Customers.Cust_Created_At', 'Created_At', 'COLUMN';
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND name = 'Cust_Updated_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Customers]') AND name = 'Updated_At')
BEGIN
    EXEC sp_rename 'HT_Customers.Cust_Updated_At', 'Updated_At', 'COLUMN';
END
GO

-- HT_Rooms_New: Rename Room_Created_At -> Created_At, Room_Updated_At -> Updated_At
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Created_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Created_At')
BEGIN
    EXEC sp_rename 'HT_Rooms_New.Room_Created_At', 'Created_At', 'COLUMN';
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Updated_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Updated_At')
BEGIN
    EXEC sp_rename 'HT_Rooms_New.Room_Updated_At', 'Updated_At', 'COLUMN';
END
GO

-- Add Room_Maintenance column if it doesn't exist
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Maintenance')
BEGIN
    ALTER TABLE [dbo].[HT_Rooms_New] ADD [Room_Maintenance] BIT DEFAULT 0;
END
GO

-- HT_Bookings: Rename Book_Total_Price -> Book_Total_Amount
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Total_Price')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Total_Amount')
BEGIN
    EXEC sp_rename 'HT_Bookings.Book_Total_Price', 'Book_Total_Amount', 'COLUMN';
END
GO

-- HT_Bookings: Rename Book_Deposit -> Book_Deposit_Amount
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Deposit')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Deposit_Amount')
BEGIN
    EXEC sp_rename 'HT_Bookings.Book_Deposit', 'Book_Deposit_Amount', 'COLUMN';
END
GO

-- HT_Bookings: Add Book_Notes if it doesn't exist
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Notes')
BEGIN
    ALTER TABLE [dbo].[HT_Bookings] ADD [Book_Notes] NVARCHAR(MAX) NULL;
END
GO

-- HT_Bookings: Rename timestamp columns
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Created_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Created_At')
BEGIN
    EXEC sp_rename 'HT_Bookings.Book_Created_At', 'Created_At', 'COLUMN';
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Book_Updated_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Bookings]') AND name = 'Updated_At')
BEGIN
    EXEC sp_rename 'HT_Bookings.Book_Updated_At', 'Updated_At', 'COLUMN';
END
GO

-- HT_Rooms_New: Add pricing columns
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Price_Weekday')
BEGIN
    ALTER TABLE [dbo].[HT_Rooms_New] ADD [Room_Price_Weekday] DECIMAL(10,2) NULL;
END
GO

IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Price_Weekend')
BEGIN
    ALTER TABLE [dbo].[HT_Rooms_New] ADD [Room_Price_Weekend] DECIMAL(10,2) NULL;
END
GO

IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_Rooms_New]') AND name = 'Room_Price_Special')
BEGIN
    ALTER TABLE [dbo].[HT_Rooms_New] ADD [Room_Price_Special] DECIMAL(10,2) NULL;
END
GO

-- HT_CheckIns: Rename Cin_Expected_Out -> Cin_Expected_Checkout
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Cin_Expected_Out')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Cin_Expected_Checkout')
BEGIN
    EXEC sp_rename 'HT_CheckIns.Cin_Expected_Out', 'Cin_Expected_Checkout', 'COLUMN';
END
GO

-- HT_CheckIns: Rename timestamp columns
IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Cin_Created_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Created_At')
BEGIN
    EXEC sp_rename 'HT_CheckIns.Cin_Created_At', 'Created_At', 'COLUMN';
END
GO

IF EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Cin_Updated_At')
    AND NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Updated_At')
BEGIN
    EXEC sp_rename 'HT_CheckIns.Cin_Updated_At', 'Updated_At', 'COLUMN';
END
GO

-- HT_CheckIns: Add Cin_Payment_Status if it doesn't exist
IF NOT EXISTS (SELECT * FROM sys.columns WHERE object_id = OBJECT_ID(N'[dbo].[HT_CheckIns]') AND name = 'Cin_Payment_Status')
BEGIN
    ALTER TABLE [dbo].[HT_CheckIns] ADD [Cin_Payment_Status] NVARCHAR(50) NULL;
END
GO

PRINT 'HotelNew database initialization complete.'
GO
