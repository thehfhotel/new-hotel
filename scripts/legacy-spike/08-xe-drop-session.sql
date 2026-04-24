-- 08-xe-drop-session.sql
-- Cleanup. Removes the session definition from the server. The .xel file
-- on disk is NOT deleted — copy it off the server first if you want it.

SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

IF EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = 'hotel_legacy_capture')
BEGIN
  -- Stop first if still running
  IF EXISTS (SELECT 1 FROM sys.dm_xe_sessions WHERE name = 'hotel_legacy_capture')
    ALTER EVENT SESSION [hotel_legacy_capture] ON SERVER STATE = STOP;

  DROP EVENT SESSION [hotel_legacy_capture] ON SERVER;
  PRINT 'Dropped session hotel_legacy_capture';
END
ELSE
BEGIN
  PRINT 'Session hotel_legacy_capture not present; nothing to drop';
END;
