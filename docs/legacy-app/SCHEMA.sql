-- Legacy iHOTEL database schema (HF Hotel production)
-- Regenerated 2026-06-11 from live INFORMATION_SCHEMA/sys.columns via read-only
-- sqlcmd dump (2026-06-11 coexistence audit) - supersedes the truncated
-- decompile-derived dump (every table was cut at ~256 bytes; 29/61 incomplete).
-- NOTE: PRIMARY KEY constraints listed here were added by OUR migrations
-- (migrations/legacy-mssql/020-022) as Change Tracking prerequisites; the
-- vendor schema had no PKs. Cross-check column authority:
-- hotel-backend/schema-baseline.txt (enforced by projection_guard tests).

-- Table: dbo.HT_Bank_Accounts
CREATE TABLE [dbo].[HT_Bank_Accounts] (
  [id] int,
  [AC_NAME] varchar(100),
  [AC_BANK_NAME] varchar(100),
  [AC_BANK_NO] varchar(100),
  [AC_BANK_TYPE] varchar(100),
  [AC_BANK_BRANCH] varchar(100)
);

-- Table: dbo.HT_Bank_Transfer
CREATE TABLE [dbo].[HT_Bank_Transfer] (
  [id] int,
  [Bank_Name] varchar(100),
  [Bank_Account_Name] varchar(100),
  [Bank_Account_No] varchar(100),
  [Bank_Account_Type] varchar(100),
  [Bank_Account_BRANCH] varchar(100),
  [Cust_Name] varchar(100),
  [Cust_Bank_Name] varchar(100),
  [Transfer_Date] datetime,
  [Transfer_Time] varchar(100),
  [Transfer_no] varchar(100),
  [Transfer_Price] float,
  [Transfer_Note] float,
  [Transfer_Slip] varbinary(max),
  [All_Price] float,
  [emp_name] varchar(100)
);

-- Table: dbo.HT_Bill_Debt_Ds
CREATE TABLE [dbo].[HT_Bill_Debt_Ds] (
  [id] int IDENTITY NOT NULL,
  [Bill_No] varchar(20),
  [DS_ID] int,
  [DS_NO] varchar(50),
  [DS_NAME] varchar(250),
  [DS_UNIT] varchar(50),
  [DS_NUM] float,
  [DS_PRICE] float,
  [DS_PRICE_TOTAL] float,
  CONSTRAINT [PK_HT_Bill_Debt_Ds] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Bill_Debt_H
CREATE TABLE [dbo].[HT_Bill_Debt_H] (
  [Bill_No] varchar(20) NOT NULL,
  [Bill_Cust_ID] varchar(50),
  [Bill_Cust_Name] varchar(250),
  [Bill_Cust_Address] varchar(500),
  [Bill_Cust_Tel] varchar(250),
  [Bill_Cust_Fax] varchar(250),
  [Bill_Date] datetime,
  [Bill_Ref] varchar(250),
  [Bill_Price_Type] varchar(50),
  [Bill_Type] varchar(50),
  [Bill_Total] float,
  [Bill_Pay] float,
  [Bill_Debt] float,
  [Bill_Pay_CASH] float,
  [Bill_Pay_CREDIT] float,
  [Bill_Status] varchar(50),
  [Bill_by] varchar(50),
  [Bill_Note] varchar(50),
  CONSTRAINT [PK_HT_Bill_Debt_H] PRIMARY KEY CLUSTERED ([Bill_No])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Book_Date
CREATE TABLE [dbo].[HT_Book_Date] (
  [id] int NOT NULL,
  [Book_no] varchar(50),
  [Book_type] varchar(50),
  [Book_date_ds] datetime,
  [Book_Num] int,
  [Book_USE] int NOT NULL DEFAULT ((0)),
  [Book_ok] int NOT NULL DEFAULT ((0)),
  [Cin_no] varchar(50),
  CONSTRAINT [PK_HT_Book_Date] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Book_Ds
CREATE TABLE [dbo].[HT_Book_Ds] (
  [id] int IDENTITY NOT NULL,
  [Book_No] varchar(50),
  [Book_Room_Type] varchar(50),
  [Book_Room_Start] datetime,
  [Book_Room_End] datetime,
  [Book_Room_Price] float,
  [Book_Room_Night] float,
  [Book_Room_Num] float,
  [Book_Room_PriceToTal] float,
  [Book_Room_Note] varchar(500),
  [Book_status] int NOT NULL DEFAULT ((1)),
  CONSTRAINT [PK_HT_Book_Ds] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Book_Ds2
CREATE TABLE [dbo].[HT_Book_Ds2] (
  [id] int,
  [Book_No] varchar(50),
  [Book_Room_Type] varchar(50),
  [Book_Room_Ds] varchar(550),
  [Book_Room_Num] float,
  [Book_Room_Days] float,
  [Book_Room_Price] float,
  [Book_Room_PriceToTal] float
);

-- Table: dbo.HT_Book_H
CREATE TABLE [dbo].[HT_Book_H] (
  [Book_ID] varchar(50) NOT NULL,
  [Book_Date] datetime,
  [Book_Date_in] datetime,
  [Book_Date_out] datetime,
  [Book_Cust_ID] varchar(50),
  [Book_Cust_Name] varchar(200),
  [Book_Cust_Name2] varchar(200),
  [Book_Cust_Tel] varchar(80),
  [Book_Price_Total] float,
  [Book_Price_Pay] float,
  [Book_Status] varchar(50),
  [Book_by] varchar(100),
  [Book_room_all] text,
  [Book_room_note] text,
  [Book_room_type] int NOT NULL DEFAULT ((1)),
  [Book_Notify_Day] int NOT NULL DEFAULT ((0)),
  [Book_Notify_Note] varchar(50),
  [Book_Sale] varchar(150),
  CONSTRAINT [PK_HT_Book_H] PRIMARY KEY CLUSTERED ([Book_ID])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Book_H2
CREATE TABLE [dbo].[HT_Book_H2] (
  [id] int,
  [Book_ID] varchar(50),
  [Book_Date] datetime,
  [Book_Date_in] datetime,
  [Book_Date_out] datetime,
  [Book_days] float,
  [Book_Cust_ID] varchar(50),
  [Book_Cust_Name] varchar(500),
  [Book_Cust_Tel] varchar(80),
  [Book_Cust_Fax] varchar(80),
  [Book_Cust_Address] varchar(500),
  [Book_Cust_Price] varchar(50),
  [Book_Cust_Note] varchar(500),
  [Book_Price_Total] float,
  [Book_Status] varchar(50)
);

-- Table: dbo.HT_Book_Pro
CREATE TABLE [dbo].[HT_Book_Pro] (
  [id] int IDENTITY NOT NULL,
  [B_NO] varchar(50),
  [B_ROOM] varchar(50),
  [B_NAME] varchar(250),
  [B_UNIT] varchar(50),
  [B_NUM] float,
  [B_PRICE] float,
  [B_PRICE_TOTAL] float,
  [B_PRO_ID] int
);

-- Table: dbo.HT_Book_Status
CREATE TABLE [dbo].[HT_Book_Status] (
  [id] int,
  [room_no] varchar(50),
  [room_date] datetime,
  [room_status] varchar(50),
  [room_Details] varchar(500),
  [room_Book_No] varchar(50),
  [room_CheckIn_No] varchar(50)
);

-- Table: dbo.HT_Booking_Notes
CREATE TABLE [dbo].[HT_Booking_Notes] (
  [Note_ID] int IDENTITY NOT NULL,
  [Book_No] nvarchar(50) NOT NULL,
  [Note_Text] nvarchar(max) NOT NULL,
  [Created_At] datetime DEFAULT (getdate()),
  [Updated_At] datetime DEFAULT (getdate()),
  CONSTRAINT [PK__HT_Booki__F94B51471D6AB78C] PRIMARY KEY CLUSTERED ([Note_ID])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Changed_Room
CREATE TABLE [dbo].[HT_Changed_Room] (
  [id] int IDENTITY NOT NULL,
  [cin_no] varchar(50) NOT NULL,
  [room_before] varchar(50),
  [room_after] varchar(50),
  [change_date] datetime,
  [room_before_price] float NOT NULL DEFAULT ((0)),
  [Note] varchar(255),
  [ToPrice] varchar(20),
  CONSTRAINT [PK_HT_Changed_Room] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_CheckIn_Ds
CREATE TABLE [dbo].[HT_CheckIn_Ds] (
  [id] int NOT NULL,
  [Cin_No] varchar(50),
  [Cin_Room_No] varchar(50),
  [Cin_Room_Type] varchar(50),
  [Cin_Room_In] datetime,
  [Cin_Room_Out] datetime,
  [Cin_Room_Status] varchar(500),
  [Cin_Room_Dep] float,
  [Cin_Room_Price] float,
  [Cin_Room_Night] float,
  [Cin_Room_PriceToTal] float,
  [Cin_Room_Pay_Before] float,
  [Cin_Room_Pay_Total] float,
  [Cin_note] varchar(500),
  [Cin_Dep_Status] varchar(50),
  [Dep_by] varchar(100),
  [Cin_cupon] int NOT NULL DEFAULT ((0)),
  [Cin_Dep_return_date] datetime,
  [Cin_Dep_return_by] varchar(50),
  CONSTRAINT [PK_HT_CheckIn_Ds] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_CheckIn_H
CREATE TABLE [dbo].[HT_CheckIn_H] (
  [Cin_no] varchar(50) NOT NULL,
  [Cin_Date] datetime,
  [Cin_Book_no] varchar(50),
  [Cin_cust_no] varchar(50),
  [Cin_cust_price] varchar(50),
  [Cin_Car_type] varchar(50),
  [Cin_Car_id] varchar(50),
  [Cin_status] varchar(50),
  [Total_Price_Room] float NOT NULL DEFAULT ((0)),
  [Total_Price_Product] float NOT NULL DEFAULT ((0)),
  [Total_Price_Net] float NOT NULL DEFAULT ((0)),
  [Total_Price_Pay] float NOT NULL DEFAULT ((0)),
  [Total_Price_Balance] float NOT NULL DEFAULT ((0)),
  [Cin_Room_ALL] varchar(500),
  [Total_Price_vat] float NOT NULL DEFAULT ((0)),
  [Cin_by] varchar(100),
  [Cin_Date_in] datetime,
  [Cin_Date_out] datetime,
  [Cin_type] int NOT NULL DEFAULT ((0)),
  [Cin_note] varchar(250),
  [Cin_foreign] varchar(50) NOT NULL DEFAULT ('False'),
  [Cin_Work_number] int NOT NULL DEFAULT ((0)),
  CONSTRAINT [PK_HT_CheckIn_H] PRIMARY KEY CLUSTERED ([Cin_no])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_CheckIn_Other_People
CREATE TABLE [dbo].[HT_CheckIn_Other_People] (
  [id] int IDENTITY NOT NULL,
  [Cin_no] varchar(50),
  [Cin_name] varchar(250),
  [Cin_contry] varchar(50),
  CONSTRAINT [PK_HT_CheckIn_Other_People] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_CheckIn_Pay
CREATE TABLE [dbo].[HT_CheckIn_Pay] (
  [id] int IDENTITY NOT NULL,
  [Pay_no] varchar(50),
  [Cin_No] varchar(50),
  [Cin_Pay_Ds] varchar(500),
  [Cin_Pay_Cash] float,
  [Cin_Pay_Credit] float,
  [Cin_Pay_Date] datetime,
  [Cin_Pay_Ds_Name] varchar(500),
  [Cin_Pay_Ds_ID] varchar(50),
  [Cin_Pay_Ds_Price] float,
  [Cin_Pay_Ds_unit] varchar(100),
  [Cin_Pay_Ds_Num] float,
  [Cin_Pay_Ds_PriceOne] float,
  [Cin_Pay_Ds_PriceTotal] float,
  [Cin_Cust_no] varchar(50),
  [Cin_Status] varchar(50) NOT NULL DEFAULT ((1)),
  [Cin_Pay_Note] varchar(500),
  [Pay_by] varchar(100),
  [Cin_Pay_Free] float NOT NULL DEFAULT ((0)),
  [Cin_Pay_Tran] float NOT NULL DEFAULT ((0)),
  [Branch] varchar(50),
  [Cin_Pay_web] float NOT NULL DEFAULT ((0)),
  CONSTRAINT [PK_HT_CheckIn_Pay] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_CheckIn_Product
CREATE TABLE [dbo].[HT_CheckIn_Product] (
  [id] int IDENTITY NOT NULL,
  [Cin_No] varchar(50),
  [Cin_Room_no] varchar(50),
  [Cin_Ds_date] datetime,
  [Cin_Pro_id] varchar(250),
  [Cin_Pro_name] varchar(500),
  [Cin_Pro_Unit] varchar(250),
  [Cin_Pro_num] float,
  [Cin_Pro_price] float,
  [Cin_Pro_priceTotal] float,
  [Cin_Pro_pay] float,
  [Cin_Pro_note] varchar(500),
  CONSTRAINT [PK_HT_CheckIn_Product] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_ContinueTime
CREATE TABLE [dbo].[HT_ContinueTime] (
  [id] int IDENTITY NOT NULL,
  [Con_Name] varchar(250),
  [Con_Minute] int,
  [Con_Price] float,
  [Con_Type] varchar(50)
);

-- Table: dbo.HT_Cupon
CREATE TABLE [dbo].[HT_Cupon] (
  [cupon_no] int NOT NULL,
  [cupon_cin_no] varchar(50),
  [cupon_cin_room] varchar(50),
  [cupon_date] datetime,
  [cupon_gen_date] datetime,
  [cupon_by] varchar(50),
  [cupon_print] int NOT NULL DEFAULT ((0)),
  CONSTRAINT [PK_HT_Cupon] PRIMARY KEY CLUSTERED ([cupon_no])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Customers
CREATE TABLE [dbo].[HT_Customers] (
  [id] int NOT NULL,
  [Cust_no] varchar(50),
  [Cust_perfix] varchar(50),
  [Cust_name] varchar(250),
  [Cust_name2] varchar(250),
  [Cust_sex] varchar(50),
  [Cust_IDcard] varchar(50),
  [Cust_Type] varchar(50),
  [Cust_Email] varchar(250),
  [Cust_Add_no] varchar(250),
  [Cust_Add_moo] varchar(50),
  [Cust_Add_soi] varchar(250),
  [Cust_Add_road] varchar(250),
  [Cust_Add_tambon] varchar(250),
  [Cust_Add_ampore] varchar(250),
  [Cust_Add_province] varchar(250),
  [Cust_Add_code] varchar(50),
  [Cust_Add_tel] varchar(250),
  [Cust_Add_fax] varchar(250),
  [Cust_Work_Name] varchar(250),
  [Cust_Work_no] varchar(250),
  [Cust_Work_moo] varchar(50),
  [Cust_Work_soi] varchar(250),
  [Cust_Work_road] varchar(250),
  [Cust_Work_tambon] varchar(250),
  [Cust_Work_ampore] varchar(250),
  [Cust_Work_province] varchar(250),
  [Cust_Work_code] varchar(50),
  [Cust_Work_tel] varchar(250),
  [Cust_Work_fax] varchar(250),
  [Cust_Last_Change] datetime,
  [Cust_Type_Main] varchar(250),
  [Cust_Price_Over] float NOT NULL DEFAULT ((0)),
  [Cust_Contry] varchar(50),
  [Cust_Work_Tax] varchar(50),
  CONSTRAINT [PK_HT_Customers] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Deposit
CREATE TABLE [dbo].[HT_Deposit] (
  [id] int NOT NULL,
  [Dep_no] varchar(250),
  [Dep_Date] datetime,
  [Dep_Room] varchar(250),
  [Dep_Name] varchar(250),
  [Dep_Price] float,
  [Dep_Status] varchar(50),
  [Dep_ref] varchar(50),
  CONSTRAINT [PK_HT_Deposit] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_EMP_SMS
CREATE TABLE [dbo].[HT_EMP_SMS] (
  [SMS_ID] int IDENTITY NOT NULL,
  [SMS_TO] varchar(50),
  [SMS_Details] text,
  [SMS_By] varchar(250),
  [SMS_Readed] varchar(50)
);

-- Table: dbo.HT_Housewife
CREATE TABLE [dbo].[HT_Housewife] (
  [id] int IDENTITY NOT NULL,
  [h_name] varchar(150),
  [h_room] varchar(50),
  [h_date] datetime,
  [h_note] text,
  [h_cin] varchar(50),
  [h_cin_name] varchar(250)
);

-- Table: dbo.HT_INVOICE
CREATE TABLE [dbo].[HT_INVOICE] (
  [INV_NO] int NOT NULL,
  [INV_booking_no] varchar(50),
  [INV_STAY] varchar(100),
  [INV_DATE] datetime,
  [INV_BY] varchar(50),
  [INV_TITLE] varchar(400),
  [INV_NAME] varchar(300),
  [INV_COMPANY] varchar(300),
  [INV_ADDRESS] varchar(500),
  [INV_TEL] varchar(50),
  [INV_NIGHT] varchar(20),
  [INV_PAX] varchar(20),
  [INV_PAX_CHILD] varchar(20),
  [INV_PAYMENT] varchar(50),
  [INV_DUEDATE] datetime,
  [INV_NOTE] text
);

-- Table: dbo.HT_Invoice_Ds
CREATE TABLE [dbo].[HT_Invoice_Ds] (
  [id] int IDENTITY NOT NULL,
  [S_Sale_id] int,
  [S_Product_no] varchar(50),
  [S_Product_name] varchar(50),
  [S_Unit] float,
  [S_UnitName] varchar(50),
  [S_Price] float,
  [S_Total] float,
  [S_PriceDiscount] float,
  [S_PriceDiscount_per] varchar(50)
);

-- Table: dbo.HT_Invoice_H
CREATE TABLE [dbo].[HT_Invoice_H] (
  [id] int,
  [Receipt_no] varchar(50),
  [Receipt_Date] datetime,
  [Receipt_c_no] varchar(50),
  [Receipt_Name] varchar(500),
  [Receipt_Address] varchar(500),
  [Receipt_Tel] varchar(50),
  [Receipt_Fax] varchar(50),
  [Receipt_Discount] float,
  [Receipt_Total] float,
  [Receipt_Vat] float,
  [Receipt_BeforeVat] float,
  [Receipt_VatIn] varchar(50),
  [Receipt_VatPer] float,
  [status_name] varchar(50),
  [Receipt_Ref] varchar(50),
  [Receipt_cin_vat_before] float,
  [Receipt_note] text,
  [Receipt_Tax] varchar(50),
  [Receipt_noteUP] text
);

-- Table: dbo.HT_Invoice_Note
CREATE TABLE [dbo].[HT_Invoice_Note] (
  [Cin_no] varchar(50),
  [note] text,
  [NOTE_STATUS] varchar(20)
);

-- Table: dbo.HT_Log
CREATE TABLE [dbo].[HT_Log] (
  [id] int IDENTITY NOT NULL,
  [details] varchar(250),
  [Emp_name] varchar(150),
  [DODATE] varchar(50)
);

-- Table: dbo.HT_Log_Debt
CREATE TABLE [dbo].[HT_Log_Debt] (
  [id] int IDENTITY NOT NULL,
  [log_cus] varchar(50),
  [log_ds] varchar(250),
  [log_date] datetime,
  [log_price_From] float,
  [log_price] float,
  [log_price_To] float
);

-- Table: dbo.HT_Order_Down
CREATE TABLE [dbo].[HT_Order_Down] (
  [id] int,
  [Cust_Type] varchar(50),
  [Cust_Month] int,
  [Cast_Type] varchar(250)
);

-- Table: dbo.HT_Order_Up
CREATE TABLE [dbo].[HT_Order_Up] (
  [id] int,
  [Cust_Type] varchar(50),
  [Cust_Month] int,
  [Cast_Type] varchar(250)
);

-- Table: dbo.HT_POWER_LOG
CREATE TABLE [dbo].[HT_POWER_LOG] (
  [id] int IDENTITY NOT NULL,
  [ROOM_NO] varchar(50),
  [ROOM_POWER_START] datetime,
  [ROOM_POWER_END] datetime,
  [ROOM_POWER_START_BY] varchar(50),
  [ROOM_POWER_END_BY] varchar(50),
  [ROOM_POWER_NOTE] varchar(250),
  [ROOM_POWER_NOTE2] varchar(250)
);

-- Table: dbo.HT_Products
CREATE TABLE [dbo].[HT_Products] (
  [id] int,
  [Pro_no] varchar(50),
  [Pro_Type] varchar(50),
  [Pro_Name] varchar(500),
  [Pro_PriceA] float,
  [Pro_PriceB] float,
  [Pro_PriceC] float,
  [Pro_Amt] float,
  [Pro_Unit] varchar(50),
  [Pro_cap] float DEFAULT ((0)),
  [Pro_Barcode] varchar(50)
);

-- Table: dbo.HT_Products_Price
CREATE TABLE [dbo].[HT_Products_Price] (
  [id] int IDENTITY NOT NULL,
  [P_ID] varchar(50),
  [P_CustType] varchar(50),
  [P_Price] float
);

-- Table: dbo.HT_Receipt_Ds
CREATE TABLE [dbo].[HT_Receipt_Ds] (
  [id] int IDENTITY NOT NULL,
  [S_Sale_id] int,
  [S_Product_no] varchar(50),
  [S_Product_name] varchar(50),
  [S_Unit] float,
  [S_UnitName] varchar(50),
  [S_Price] float,
  [S_Total] float,
  [S_PriceDiscount] float,
  [S_PriceDiscount_per] varchar(50)
);

-- Table: dbo.HT_Receipt_H
CREATE TABLE [dbo].[HT_Receipt_H] (
  [id] int NOT NULL,
  [Receipt_no] varchar(50),
  [Receipt_Date] datetime,
  [Receipt_c_no] varchar(50),
  [Receipt_Name] varchar(500),
  [Receipt_Address] varchar(500),
  [Receipt_Tel] varchar(50),
  [Receipt_Fax] varchar(50),
  [Receipt_Discount] float,
  [Receipt_Total] float,
  [Receipt_Vat] float,
  [Receipt_BeforeVat] float,
  [Receipt_VatIn] varchar(50),
  [Receipt_VatPer] float,
  [status_name] varchar(50),
  [Receipt_Ref] varchar(50),
  [Receipt_cin_vat_before] float,
  [Receipt_note] text,
  [Receipt_Tax] varchar(50),
  [Receipt_noteUP] text,
  CONSTRAINT [PK_HT_Receipt_H] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Register
CREATE TABLE [dbo].[HT_Register] (
  [id] int,
  [Reg_no] varchar(250),
  [Reg_Date] datetime,
  [Reg_Room] varchar(250),
  [Reg_Name] varchar(250),
  [Reg_Address] varchar(500),
  [Reg_Doing] varchar(250),
  [Reg_Nation] varchar(250),
  [Reg_From] varchar(250),
  [Reg_To] varchar(250),
  [Reg_Passport] varchar(250),
  [Reg_Status] varchar(250),
  [Reg_ref] varchar(50)
);

-- Table: dbo.HT_Room_SMS
CREATE TABLE [dbo].[HT_Room_SMS] (
  [SMS_ID] int IDENTITY NOT NULL,
  [SMS_Room] varchar(50),
  [SMS_Details] text,
  [SMS_By] varchar(250),
  [SMS_Readed] varchar(50)
);

-- Table: dbo.HT_Room_Status
CREATE TABLE [dbo].[HT_Room_Status] (
  [id] int NOT NULL,
  [room_no] varchar(50),
  [room_date] datetime,
  [room_status] varchar(50),
  [room_Details] varchar(500),
  [room_Book_No] varchar(50),
  [room_CheckIn_No] varchar(50),
  [room_date_oa] float NOT NULL DEFAULT ((0)),
  CONSTRAINT [PK_HT_Room_Status] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Rooms
CREATE TABLE [dbo].[HT_Rooms] (
  [id] int NOT NULL,
  [Room_no] varchar(50),
  [Room_Type] varchar(50),
  [Room_Details] varchar(500),
  [Room_PriceA] float,
  [Room_PriceB] float,
  [Room_PriceC] float,
  [Room_Clean] varchar(50) NOT NULL DEFAULT ('no'),
  [Room_Use] varchar(50) NOT NULL DEFAULT ('no'),
  [Room_Manternace] varchar(50) NOT NULL DEFAULT ('no'),
  [Room_X] int NOT NULL DEFAULT ((0)),
  [Room_Y] int NOT NULL DEFAULT ((0)),
  [Room_Use_Count] int NOT NULL DEFAULT ((0)),
  [Room_Polity] int NOT NULL DEFAULT ((1)),
  [Room_Book] varchar(50),
  [Room_Book_Name] varchar(250),
  [Room_Book_Time] varchar(50),
  [Room_Group] varchar(50),
  [Room_Book_ds] text,
  [Room_Power_OPEN] varchar(50),
  [Room_Power_CLOSE] varchar(50),
  [Room_Power_STATUS] varchar(50) NOT NULL DEFAULT ('off'),
  [Room_Clean_Time] varchar(30),
  CONSTRAINT [PK_HT_Rooms] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Rooms_Cancel
CREATE TABLE [dbo].[HT_Rooms_Cancel] (
  [id] int NOT NULL,
  [room_no] varchar(50),
  [cin_no] varchar(50),
  [cancel_date] datetime,
  [cancel_by] varchar(50),
  [cancel_note] text,
  CONSTRAINT [PK_HT_Rooms_Cancel] PRIMARY KEY CLUSTERED ([id])  -- added by migrations/legacy-mssql (CT prerequisite)
);

-- Table: dbo.HT_Rooms_Price
CREATE TABLE [dbo].[HT_Rooms_Price] (
  [id] int IDENTITY NOT NULL,
  [Room_Type] varchar(50),
  [Room_CustType] varchar(50),
  [Room_Price] float,
  [Room_Price_H] float,
  [Room_Price_M] float
);

-- Table: dbo.HT_Rooms_Repair
CREATE TABLE [dbo].[HT_Rooms_Repair] (
  [id] int IDENTITY NOT NULL,
  [room_no] varchar(50),
  [Repair_date] datetime,
  [Repair_by] varchar(250),
  [Repair_note] text
);

-- Table: dbo.HT_Round_Bill
CREATE TABLE [dbo].[HT_Round_Bill] (
  [id] int NOT NULL,
  [round_no] int,
  [round_price] float,
  [round_by] varchar(150),
  [round_start] datetime,
  [round_end] datetime
);

-- Table: dbo.HT_SET_CusType
CREATE TABLE [dbo].[HT_SET_CusType] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(50),
  [deposit] float NOT NULL DEFAULT ((0))
);

-- Table: dbo.HT_SET_CusType_Main
CREATE TABLE [dbo].[HT_SET_CusType_Main] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(50)
);

-- Table: dbo.HT_SET_ProductType
CREATE TABLE [dbo].[HT_SET_ProductType] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(50)
);

-- Table: dbo.HT_SET_RoomType
CREATE TABLE [dbo].[HT_SET_RoomType] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(50),
  [Room_PriceA] float,
  [Room_PriceB] float,
  [Room_PriceC] float
);

-- Table: dbo.HT_SET_Sale
CREATE TABLE [dbo].[HT_SET_Sale] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(150),
  [tel] varchar(50),
  [address] varchar(400),
  [other] varchar(500)
);

-- Table: dbo.MSchange_tracking_history
CREATE TABLE [dbo].[MSchange_tracking_history] (
  [internal_table_name] sysname NOT NULL,
  [table_name] sysname NOT NULL,
  [start_time] datetime NOT NULL,
  [end_time] datetime NOT NULL,
  [rows_cleaned_up] bigint NOT NULL,
  [cleanup_version] bigint NOT NULL,
  [comments] nvarchar(max) NOT NULL
);

-- Table: dbo.TB_FOLIO
CREATE TABLE [dbo].[TB_FOLIO] (
  [id] int,
  [NO] varchar(20),
  [CIN_NAME1] varchar(255),
  [CIN_NAME2] varchar(255),
  [CIN_NAME3] varchar(255),
  [F_ROOM] varchar(50),
  [F_NAME] varchar(250),
  [F_IN] varchar(50),
  [F_OUT] varchar(50),
  [F_NIGHT] varchar(50),
  [F_PRICE] varchar(50),
  [F_PRICE_TOTAL] varchar(50),
  [F_STATUS] varchar(20)
);

-- Table: dbo.TB_MRP_EMPLOYEE
CREATE TABLE [dbo].[TB_MRP_EMPLOYEE] (
  [ID] int IDENTITY NOT NULL,
  [Emp_Username] varchar(50),
  [Emp_Password] varchar(50),
  [Emp_Name] varchar(50),
  [Emp_Level] varchar(50),
  [Emp_Vat] varchar(50)
);

-- Table: dbo.TB_MRP_Permission
CREATE TABLE [dbo].[TB_MRP_Permission] (
  [Level_Name] varchar(50),
  [Level_Command] varchar(50)
);

-- Table: dbo.TB_MRP_Permission_name
CREATE TABLE [dbo].[TB_MRP_Permission_name] (
  [Permission_name] varchar(50)
);

-- Table: dbo.TB_Pay_History
CREATE TABLE [dbo].[TB_Pay_History] (
  [id] int,
  [Pay_Date] float,
  [Pay_Bill] varchar(255),
  [Pay_Cust] varchar(500),
  [Pay_Type] varchar(50),
  [Pay_Total] float,
  [Pay_Note] varchar(500),
  [Pay_Program] float,
  [Pay_Group] varchar(50),
  [Pay_Account] varchar(50)
);

-- Table: dbo.Tb_Save_Image
CREATE TABLE [dbo].[Tb_Save_Image] (
  [id] int IDENTITY NOT NULL,
  [cin_no] varchar(50),
  [ttype] varchar(250),
  [pic] varbinary(max),
  [cust_no] varchar(50),
  [tmp_no] varchar(50),
  [pic_date] datetime
);

-- Table: dbo.TB_SET_Branch
CREATE TABLE [dbo].[TB_SET_Branch] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(100)
);

-- Table: dbo.TB_SET_MyType2
CREATE TABLE [dbo].[TB_SET_MyType2] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(100)
);

-- Table: dbo.TB_SET_MyType2_2
CREATE TABLE [dbo].[TB_SET_MyType2_2] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(100)
);

-- Table: dbo.TB_SET_MyType3
CREATE TABLE [dbo].[TB_SET_MyType3] (
  [id] int IDENTITY NOT NULL,
  [id_full] varchar(50),
  [name] varchar(100)
);

-- Table: dbo.TB_SETTINGS
CREATE TABLE [dbo].[TB_SETTINGS] (
  [Company_Name] varchar(500),
  [Company_Address] varchar(500),
  [Company_Tel] varchar(100),
  [Company_Fax] varchar(100),
  [Company_Image] varbinary(max),
  [Company_Tax] varchar(50),
  [CHK_IN_Before] float,
  [CHK_Out] float,
  [CHK_Out_Alert] float,
  [CHK_Out_Before] float,
  [CHK_Out_H_price] float,
  [Maximum_Book] float,
  [Pority] float,
  [Vat_Over] float,
  [Cal_Pority_Cust] varchar(50),
  [receipt_type] varchar(50),
  [receipt_print] varchar(50),
  [receipt_preview] varchar(50),
  [Vat_per] float,
  [Vat_Head] varchar(50),
  [reg_type] varchar(10),
  [login_url] varchar(250),
  [Min_HOURS] float,
  [AUTO_CUT_POWER] varchar(10),
  [MANUAL_POWER] varchar(10),
  [POWER_Delay] varchar(10),
  [VAT_OUT] varchar(10),
  [Vat_Head2] varchar(50),
  [Vat_Rows] int,
  [Room_Clean_Time] varchar(10),
  [SHOW_ICON] varchar(10),
  [Time_Logout] varchar(10),
  [CompanyName] varchar(500)
);

-- Table: dbo.Tb_Version
CREATE TABLE [dbo].[Tb_Version] (
  [V_NO] varchar(50)
);
