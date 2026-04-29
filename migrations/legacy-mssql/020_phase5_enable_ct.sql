-- Phase 5 — enable Change Tracking on the 11 canonical-sync transactional
-- tables that drive `bin/sync.rs`'s real-time CT watcher mappers.
--
-- This file BACKFILLS the original Phase 5 cutover DDL — at HF Hotel the
-- equivalent statements were applied manually on 2026-04-25 before the
-- migration system existed. Captured here so HF Ville (and any future
-- restore / new site) can reach the same pre-Phase-5.5 baseline from a
-- single auditable file.
--
-- Tables enabled (11):
--   HT_Customers, HT_Rooms,
--   HT_Book_H, HT_Book_Ds, HT_Book_Date,
--   HT_CheckIn_H, HT_CheckIn_Ds, HT_CheckIn_Pay,
--   HT_Room_Status, HT_Rooms_Cancel, HT_Receipt_H
--
-- Pre-flight (verified 2026-04-29 against Ville HOTEL via WG path
-- 192.168.11.51:1436):
--   * Zero NULLs and zero duplicates on every PK candidate column
--   * Five tables had their PK column still NULLABLE (HT_Rooms,
--     HT_Book_H, HT_Room_Status, HT_Rooms_Cancel, HT_Receipt_H) —
--     these get an explicit ALTER COLUMN NOT NULL first.
--   * The other six tables already had the column NOT NULL (identity
--     columns or natural keys) — no ALTER needed before PK.
--   * No existing PRIMARY KEY on any of the 11 tables.
--   * No existing per-table CT on any of the 11 tables.
--   * No existing DB-level CT (the `ALTER DATABASE ... SET CHANGE_TRACKING`
--     at top is required at sites that haven't been Phase-5'd yet; at
--     HF Hotel re-running it would error so coordinate accordingly).
--
-- Rollback: see 020_phase5_enable_ct.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/020_phase5_enable_ct.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools/bin/sqlcmd \
--       mcr.microsoft.com/mssql-tools \
--       -S <ip,port> -U sa -P "$DB_PASSWORD" -d <db> -W'
--
-- For HF Ville: -S 192.168.11.51,1436 -d HOTEL.
-- For HF Hotel: ALREADY APPLIED (2026-04-25, manually). Do not re-run —
--   ALTER DATABASE SET CHANGE_TRACKING ON would error if already on.

SET NOCOUNT ON;
GO

-- 0. Database-level CT enable (NEW SITES ONLY — error if already enabled).
--    Retention=2 DAYS matches HF Hotel's setting; auto-cleanup ON.
ALTER DATABASE CURRENT SET CHANGE_TRACKING = ON (CHANGE_RETENTION = 2 DAYS, AUTO_CLEANUP = ON);
GO

-- IMPORTANT: GO separators between each statement so SQL Server commits
-- each metadata change before parsing the next. Without GO,
-- ALTER COLUMN ... NOT NULL and the subsequent ADD CONSTRAINT PK run
-- in the same batch and the PK validation reads the OLD column
-- nullability — Msg 8111 "Cannot define PRIMARY KEY constraint on
-- nullable column" even though the ALTER COLUMN succeeded. Learned the
-- hard way 2026-04-28 (see 021_phase55b_enable_ct.sql for the
-- post-mortem on this trap).

-- 1. HT_Customers — id is already NOT NULL.
ALTER TABLE HT_Customers ADD CONSTRAINT PK_HT_Customers PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Customers ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 2. HT_Rooms — id is nullable; tighten then PK + CT.
ALTER TABLE HT_Rooms ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_Rooms ADD CONSTRAINT PK_HT_Rooms PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Rooms ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 3. HT_Book_H — natural key Book_ID is nullable VARCHAR(50); tighten then PK + CT.
ALTER TABLE HT_Book_H ALTER COLUMN Book_ID VARCHAR(50) NOT NULL;
GO
ALTER TABLE HT_Book_H ADD CONSTRAINT PK_HT_Book_H PRIMARY KEY CLUSTERED (Book_ID);
GO
ALTER TABLE HT_Book_H ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 4. HT_Book_Ds — id is identity; already NOT NULL.
ALTER TABLE HT_Book_Ds ADD CONSTRAINT PK_HT_Book_Ds PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Book_Ds ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 5. HT_Book_Date — id is already NOT NULL.
ALTER TABLE HT_Book_Date ADD CONSTRAINT PK_HT_Book_Date PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Book_Date ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 6. HT_CheckIn_H — natural key Cin_no is already NOT NULL.
ALTER TABLE HT_CheckIn_H ADD CONSTRAINT PK_HT_CheckIn_H PRIMARY KEY CLUSTERED (Cin_no);
GO
ALTER TABLE HT_CheckIn_H ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 7. HT_CheckIn_Ds — id is already NOT NULL.
ALTER TABLE HT_CheckIn_Ds ADD CONSTRAINT PK_HT_CheckIn_Ds PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_CheckIn_Ds ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 8. HT_CheckIn_Pay — id is identity; already NOT NULL.
ALTER TABLE HT_CheckIn_Pay ADD CONSTRAINT PK_HT_CheckIn_Pay PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_CheckIn_Pay ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 9. HT_Room_Status — id is nullable; tighten then PK + CT.
ALTER TABLE HT_Room_Status ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_Room_Status ADD CONSTRAINT PK_HT_Room_Status PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Room_Status ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 10. HT_Rooms_Cancel — id is nullable; tighten then PK + CT.
ALTER TABLE HT_Rooms_Cancel ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_Rooms_Cancel ADD CONSTRAINT PK_HT_Rooms_Cancel PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Rooms_Cancel ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

-- 11. HT_Receipt_H — id is nullable; tighten then PK + CT.
ALTER TABLE HT_Receipt_H ALTER COLUMN id INT NOT NULL;
GO
ALTER TABLE HT_Receipt_H ADD CONSTRAINT PK_HT_Receipt_H PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Receipt_H ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO
