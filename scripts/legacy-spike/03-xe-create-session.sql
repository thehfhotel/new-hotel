-- 03-xe-create-session.sql
-- Create an Extended Events session that captures every SQL statement and
-- RPC executed against the legacy DB while the session is running. Output
-- goes to a rolling .xel file in the SQL Server log directory.
--
-- ## Filtering
--
-- We filter by sqlserver.database_name to avoid catching unrelated DBs.
-- EDIT @target_db_name BELOW to match your legacy DB name (see 00-prereqs
-- output for the candidate list).
--
-- We do NOT filter by login here — easier to catch everything, then filter
-- in 07-xe-read.sql by `client_app_name` once we know what name the .NET
-- app reports.
--
-- ## Events captured
--
-- - sql_batch_completed : full T-SQL batch finished (most app queries land here)
-- - rpc_completed       : stored procedure / parameterised exec (also common)
-- - sp_statement_completed : individual statements inside a proc (verbose;
--                            uncomment if you need stmt-level detail)
-- - error_reported      : SQL errors raised during the session — useful
--                         to see when the app's own queries fail
--
-- ## Performance
--
-- Filter by database_name keeps overhead small. Capacity: 100 MB ring buffer
-- with 2 file rollover = ~200 MB peak disk. Drop the session as soon as
-- the receptionist is done (05-xe-stop.sql).

SET NOCOUNT ON;

-- ============================================================================
-- EDIT THIS to your legacy DB name (from 00-prereqs.sql output, sys.databases)
-- ============================================================================
DECLARE @target_db_name SYSNAME = N'db';   -- <-- change me if needed
DECLARE @session_name   SYSNAME = N'hotel_legacy_capture';
DECLARE @file_path      NVARCHAR(260) = N'xe_hotel_legacy_capture.xel';
-- file_path is relative to SQL Server's default LOG dir. To use absolute,
-- e.g. N'C:\temp\xe_hotel_legacy_capture.xel', uncomment and edit:
-- SET @file_path = N'C:\temp\xe_hotel_legacy_capture.xel';

-- Drop existing session of the same name if it's still there from a previous run
IF EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = @session_name)
BEGIN
  DECLARE @drop NVARCHAR(MAX) = N'DROP EVENT SESSION ' + QUOTENAME(@session_name) + N' ON SERVER';
  EXEC sp_executesql @drop;
  PRINT 'Dropped pre-existing session: ' + @session_name;
END;

DECLARE @ddl NVARCHAR(MAX) = N'
CREATE EVENT SESSION ' + QUOTENAME(@session_name) + N' ON SERVER

  ADD EVENT sqlserver.sql_batch_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.tsql_stack,
      sqlserver.sql_text, sqlserver.transaction_id
    )
    WHERE sqlserver.database_name = N''' + REPLACE(@target_db_name, '''', '''''') + N'''
          AND sqlserver.client_app_name <> N''tiberius''
          AND sqlserver.client_app_name NOT LIKE N''SQLServerCEIP%''
          AND sqlserver.client_app_name NOT LIKE N''SQLCMD%''
  ),

  ADD EVENT sqlserver.rpc_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text,
      sqlserver.transaction_id
    )
    WHERE sqlserver.database_name = N''' + REPLACE(@target_db_name, '''', '''''') + N'''
          AND sqlserver.client_app_name <> N''tiberius''
          AND sqlserver.client_app_name NOT LIKE N''SQLServerCEIP%''
          AND sqlserver.client_app_name NOT LIKE N''SQLCMD%''
  ),

  ADD EVENT sqlserver.error_reported (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text
    )
    WHERE sqlserver.database_name = N''' + REPLACE(@target_db_name, '''', '''''') + N'''
          AND sqlserver.client_app_name <> N''tiberius''
          AND sqlserver.client_app_name NOT LIKE N''SQLServerCEIP%''
          AND sqlserver.client_app_name NOT LIKE N''SQLCMD%''
      AND severity >= 11
  )

  -- Uncomment for per-statement-inside-proc detail (verbose):
  -- ,ADD EVENT sqlserver.sp_statement_completed (
  --    ACTION (sqlserver.session_id, sqlserver.client_app_name, sqlserver.sql_text)
  --    WHERE sqlserver.database_name = N''' + REPLACE(@target_db_name, '''', '''''') + N'''
          AND sqlserver.client_app_name <> N''tiberius''
          AND sqlserver.client_app_name NOT LIKE N''SQLServerCEIP%''
          AND sqlserver.client_app_name NOT LIKE N''SQLCMD%''
  --  )

  ADD TARGET package0.event_file (
    SET filename = N''' + REPLACE(@file_path, '''', '''''') + N''',
        max_file_size = (100),         -- MB per file
        max_rollover_files = (2)
  )

  WITH (
    MAX_MEMORY = 8 MB,
    EVENT_RETENTION_MODE = ALLOW_SINGLE_EVENT_LOSS,  -- never block app
    MAX_DISPATCH_LATENCY = 5 SECONDS,
    TRACK_CAUSALITY = ON,                            -- group related events
    STARTUP_STATE = OFF
  );';

EXEC sp_executesql @ddl;
PRINT 'Created XE session: ' + @session_name;
PRINT 'Filter: database_name = ' + @target_db_name;
PRINT 'Output file (relative to SQL log dir): ' + @file_path;
