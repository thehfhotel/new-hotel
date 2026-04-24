-- 04-xe-start.sql
SET NOCOUNT ON;
ALTER EVENT SESSION [hotel_legacy_capture] ON SERVER STATE = START;
PRINT 'Session started at ' + CONVERT(VARCHAR(30), SYSUTCDATETIME(), 126) + ' UTC';
PRINT 'Hand the action playbook to the receptionist now.';
