-- 04-xe-start.sql
SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

IF NOT EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = 'hotel_legacy_capture')
BEGIN
  RAISERROR('Session hotel_legacy_capture does not exist. Run 03-xe-create-session.sql first.', 16, 1);
  RETURN;
END;

ALTER EVENT SESSION [hotel_legacy_capture] ON SERVER STATE = START;
PRINT 'Session started at ' + CONVERT(VARCHAR(30), SYSUTCDATETIME(), 126) + ' UTC';
PRINT 'Hand the action playbook to the receptionist now.';
