-- Migration: 007_maintenance_system
-- Version: 0.8.0
-- Date: 2026-02-06

-- Categories table
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'HT_Maintenance_Categories') AND type = N'U')
BEGIN
    CREATE TABLE HT_Maintenance_Categories (
        MCat_ID INT IDENTITY(1,1) PRIMARY KEY,
        MCat_Name NVARCHAR(100) NOT NULL,
        MCat_Name_En NVARCHAR(100) NULL,
        MCat_Priority INT DEFAULT 2,
        MCat_Active BIT DEFAULT 1
    );

    INSERT INTO HT_Maintenance_Categories (MCat_Name, MCat_Name_En, MCat_Priority) VALUES
    (N'ไฟฟ้า', 'Electrical', 3),
    (N'ประปา', 'Plumbing', 3),
    (N'เครื่องปรับอากาศ', 'Air Conditioning', 3),
    (N'เฟอร์นิเจอร์', 'Furniture', 2),
    (N'ทั่วไป', 'General', 2);
END
GO

-- Requests table
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'HT_Maintenance_Requests') AND type = N'U')
BEGIN
    CREATE TABLE HT_Maintenance_Requests (
        MReq_ID INT IDENTITY(1,1) PRIMARY KEY,
        MReq_No NVARCHAR(20) NOT NULL UNIQUE,
        MReq_Room_ID INT NOT NULL,
        MReq_Category_ID INT NOT NULL,
        MReq_Title NVARCHAR(200) NOT NULL,
        MReq_Description NVARCHAR(MAX) NULL,
        MReq_Priority INT DEFAULT 2,
        MReq_Status NVARCHAR(20) DEFAULT 'open',
        MReq_Assigned_To NVARCHAR(100) NULL,
        MReq_Started_At DATETIME NULL,
        MReq_Completed_At DATETIME NULL,
        MReq_Resolution NVARCHAR(MAX) NULL,
        MReq_Cost DECIMAL(10,2) NULL,
        MReq_Created_At DATETIME DEFAULT GETDATE(),
        MReq_Updated_At DATETIME DEFAULT GETDATE(),
        CONSTRAINT FK_MReq_Room FOREIGN KEY (MReq_Room_ID) REFERENCES HT_Rooms_New(Room_ID),
        CONSTRAINT FK_MReq_Category FOREIGN KEY (MReq_Category_ID) REFERENCES HT_Maintenance_Categories(MCat_ID)
    );

    CREATE INDEX IX_MReq_Room ON HT_Maintenance_Requests(MReq_Room_ID);
    CREATE INDEX IX_MReq_Status ON HT_Maintenance_Requests(MReq_Status);
END
GO

-- Sequence for request numbers
IF NOT EXISTS (SELECT * FROM sys.sequences WHERE name = 'SQ_Maintenance_No')
BEGIN
    CREATE SEQUENCE SQ_Maintenance_No AS INT START WITH 1 INCREMENT BY 1;
END
GO

-- DOWN MIGRATION (rollback)
-- DROP INDEX IF EXISTS IX_MReq_Status ON HT_Maintenance_Requests;
-- DROP INDEX IF EXISTS IX_MReq_Room ON HT_Maintenance_Requests;
-- DROP TABLE IF EXISTS HT_Maintenance_Requests;
-- DROP TABLE IF EXISTS HT_Maintenance_Categories;
-- DROP SEQUENCE IF EXISTS SQ_Maintenance_No;
