-- Migration: 004_create_inventory_tables
-- Version: 2.1.0
-- Date: 2026-02-05
-- Description: Creates inventory management tables for hotel inventory tracking

-- =====================================================
-- UP MIGRATION
-- =====================================================

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

-- Room Inventory (what items assigned to each room)
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
        Trans_Type VARCHAR(20) NOT NULL, -- IN, OUT, ADJUST, MOVE
        Trans_Quantity INT NOT NULL,
        Trans_Room_ID INT,
        Trans_Notes NVARCHAR(500),
        Trans_Date DATETIME DEFAULT GETDATE(),
        Trans_By NVARCHAR(100)
    );
END
GO

-- Create indexes for performance
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

-- =====================================================
-- DOWN MIGRATION (Rollback - Use with caution!)
-- =====================================================
/*
DROP INDEX IF EXISTS IX_HT_Inventory_Transactions_Date ON HT_Inventory_Transactions;
DROP INDEX IF EXISTS IX_HT_Inventory_Transactions_Item ON HT_Inventory_Transactions;
DROP INDEX IF EXISTS IX_HT_Room_Inventory_Item ON HT_Room_Inventory;
DROP INDEX IF EXISTS IX_HT_Room_Inventory_Room ON HT_Room_Inventory;
DROP INDEX IF EXISTS IX_HT_Inventory_Items_Code ON HT_Inventory_Items;
DROP INDEX IF EXISTS IX_HT_Inventory_Items_Category ON HT_Inventory_Items;

DROP TABLE IF EXISTS HT_Inventory_Transactions;
DROP TABLE IF EXISTS HT_Room_Inventory;
DROP TABLE IF EXISTS HT_Inventory_Items;
DROP TABLE IF EXISTS HT_Inventory_Categories;
*/
