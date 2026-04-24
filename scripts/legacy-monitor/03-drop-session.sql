-- Cleanup: drop the monitoring XE session entirely.
-- The .xel files on disk are NOT deleted — copy them off first if you want them.

SET QUOTED_IDENTIFIER ON;
GO

IF EXISTS (SELECT 1 FROM sys.server_event_sessions WHERE name = 'hotel_monitor')
BEGIN
    IF EXISTS (SELECT 1 FROM sys.dm_xe_sessions WHERE name = 'hotel_monitor')
        ALTER EVENT SESSION [hotel_monitor] ON SERVER STATE = STOP;
    DROP EVENT SESSION [hotel_monitor] ON SERVER;
    PRINT 'hotel_monitor session dropped';
END
ELSE
    PRINT 'hotel_monitor session not present';
GO
