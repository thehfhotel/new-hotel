-- Long-running XE session for legacy DB monitoring.
--
-- Captures TWO things simultaneously:
--   1. ANY error from the .NET app (severity >= 11)        → for CT/PK rollback alerting
--   2. EVERY write the .NET app does                        → for morning-routine analysis
--
-- Excludes our own backend (tiberius) so the noise stays low.
-- Output: rolling .xel file in SQL Server LOG dir, max 4 × 100MB = 400MB cap.
--
-- Run: cat 01-setup-session.sql | sqlcmd -S <legacy-mssql-host> -U sa -P ... -d master

SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;
GO

IF EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = 'hotel_monitor')
BEGIN
    ALTER EVENT SESSION [hotel_monitor] ON SERVER STATE = STOP;
    DROP EVENT SESSION [hotel_monitor] ON SERVER;
    PRINT 'Dropped pre-existing hotel_monitor session';
END
GO

CREATE EVENT SESSION [hotel_monitor] ON SERVER

  -- ERRORS: anything user-visible from the .NET app
  ADD EVENT sqlserver.error_reported (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text,
      sqlserver.tsql_stack
    )
    WHERE sqlserver.database_name = N'db'
      AND severity >= 11
      AND sqlserver.client_app_name NOT LIKE N'tiberius%'
      AND sqlserver.client_app_name NOT LIKE N'SQLCMD%'
  ),

  -- WRITES: every batch the .NET app sends (filter to writes only at read time)
  ADD EVENT sqlserver.sql_batch_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text,
      sqlserver.transaction_id
    )
    WHERE sqlserver.database_name = N'db'
      AND sqlserver.client_app_name = N'.Net SqlClient Data Provider'
  ),

  -- RPCs: parameterized stored proc calls (no procs in this DB but capture anyway)
  ADD EVENT sqlserver.rpc_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text
    )
    WHERE sqlserver.database_name = N'db'
      AND sqlserver.client_app_name = N'.Net SqlClient Data Provider'
  )

  ADD TARGET package0.event_file (
    SET filename = N'xe_hotel_monitor.xel',
        max_file_size = (100),     -- MB per file
        max_rollover_files = (4)   -- 400MB total cap
  )

  WITH (
    MAX_MEMORY = 16 MB,
    EVENT_RETENTION_MODE = ALLOW_SINGLE_EVENT_LOSS,  -- never block the legacy app
    MAX_DISPATCH_LATENCY = 5 SECONDS,
    TRACK_CAUSALITY = ON,
    STARTUP_STATE = ON      -- auto-start on SQL Server restart
  );
GO

ALTER EVENT SESSION [hotel_monitor] ON SERVER STATE = START;
GO

PRINT 'hotel_monitor session created and started';
PRINT 'Output: xe_hotel_monitor*.xel in SQL Server LOG dir';
PRINT 'STARTUP_STATE = ON — survives SQL Server restart';
