-- Migration: 006_payment_tracking
-- Version: 0.8.0
-- Date: 2026-02-06
-- Description: Add payment tracking system for multiple payments per check-in

-- UP MIGRATION

-- Payments table
IF NOT EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'HT_Payments') AND type = N'U')
BEGIN
    CREATE TABLE HT_Payments (
        Pay_ID INT IDENTITY(1,1) PRIMARY KEY,
        Pay_Cin_ID INT NOT NULL,
        Pay_Amount DECIMAL(12,2) NOT NULL,
        Pay_Method NVARCHAR(50) NOT NULL,
        Pay_Reference NVARCHAR(100) NULL,
        Pay_Notes NVARCHAR(500) NULL,
        Pay_Date DATETIME DEFAULT GETDATE(),
        Pay_Created_By NVARCHAR(50) NULL,
        Pay_Voided BIT DEFAULT 0,
        Pay_Voided_At DATETIME NULL,
        Pay_Voided_By NVARCHAR(50) NULL,
        Created_At DATETIME DEFAULT GETDATE(),
        CONSTRAINT FK_HT_Payments_CheckIn FOREIGN KEY (Pay_Cin_ID) REFERENCES HT_CheckIns(Cin_ID)
    );

    CREATE INDEX IX_HT_Payments_CheckIn ON HT_Payments(Pay_Cin_ID);
    CREATE INDEX IX_HT_Payments_Date ON HT_Payments(Pay_Date);
END
GO

-- DOWN MIGRATION (commented out)
-- DROP TABLE IF EXISTS HT_Payments;
