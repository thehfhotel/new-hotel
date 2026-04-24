-- 03-xe-create-session.sql
-- Create the Extended Events session that captures every SQL the 3rd-party
-- app sends. Filter: database = 'db', exclude our own backend's tiberius
-- traffic, exclude Microsoft telemetry, exclude this sqlcmd session itself.
--
-- File output goes to SQL Server's default LOG dir as xe_hotel_legacy_capture.xel
-- with one rollover file (~200 MB peak).

SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

IF EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = 'hotel_legacy_capture')
BEGIN
  DROP EVENT SESSION [hotel_legacy_capture] ON SERVER;
  PRINT 'Dropped pre-existing session: hotel_legacy_capture';
END;

CREATE EVENT SESSION [hotel_legacy_capture] ON SERVER

  ADD EVENT sqlserver.sql_batch_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.tsql_stack,
      sqlserver.sql_text, sqlserver.transaction_id
    )
    WHERE sqlserver.database_name = N'db'
      AND sqlserver.client_app_name <> N'tiberius'
      AND sqlserver.client_app_name NOT LIKE N'SQLServerCEIP%'
      AND sqlserver.client_app_name NOT LIKE N'SQLCMD%'
  ),

  ADD EVENT sqlserver.rpc_completed (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text,
      sqlserver.transaction_id
    )
    WHERE sqlserver.database_name = N'db'
      AND sqlserver.client_app_name <> N'tiberius'
      AND sqlserver.client_app_name NOT LIKE N'SQLServerCEIP%'
      AND sqlserver.client_app_name NOT LIKE N'SQLCMD%'
  ),

  ADD EVENT sqlserver.error_reported (
    ACTION (
      sqlserver.session_id, sqlserver.client_app_name, sqlserver.client_hostname,
      sqlserver.username, sqlserver.database_name, sqlserver.sql_text
    )
    WHERE sqlserver.database_name = N'db'
      AND severity >= 11
      AND sqlserver.client_app_name <> N'tiberius'
      AND sqlserver.client_app_name NOT LIKE N'SQLServerCEIP%'
      AND sqlserver.client_app_name NOT LIKE N'SQLCMD%'
  )

  ADD TARGET package0.event_file (
    SET filename = N'xe_hotel_legacy_capture.xel',
        max_file_size = (100),       -- MB per file
        max_rollover_files = (2)
  )

  WITH (
    MAX_MEMORY = 8 MB,
    EVENT_RETENTION_MODE = ALLOW_SINGLE_EVENT_LOSS,
    MAX_DISPATCH_LATENCY = 5 SECONDS,
    TRACK_CAUSALITY = ON,
    STARTUP_STATE = OFF
  );

PRINT 'Created XE session: hotel_legacy_capture';
PRINT 'Filter: database_name = db, excluding tiberius/CEIP/SQLCMD';
PRINT 'Output: xe_hotel_legacy_capture.xel (in SQL Server default LOG dir)';
