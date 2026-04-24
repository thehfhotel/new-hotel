-- 05-xe-stop.sql
SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

IF EXISTS (SELECT 1 FROM sys.dm_xe_sessions WHERE name = 'hotel_legacy_capture')
  ALTER EVENT SESSION [hotel_legacy_capture] ON SERVER STATE = STOP;
PRINT 'Session stopped at ' + CONVERT(VARCHAR(30), SYSUTCDATETIME(), 126) + ' UTC';

-- Show where the .xel files landed (so finalize.sh can pull them).
SELECT
  CAST(target_data AS XML).value(
    '(EventFileTarget/File/@name)[1]', 'NVARCHAR(260)'
  ) AS xel_file
FROM sys.dm_xe_session_targets st
JOIN sys.dm_xe_sessions s ON s.address = st.event_session_address
WHERE s.name = 'hotel_legacy_capture' AND st.target_name = 'event_file';
