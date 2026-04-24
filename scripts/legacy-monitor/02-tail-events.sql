-- Pull every event captured since the given UTC timestamp.
-- The :since variable is replaced before execution by tail-loop.sh.
-- Output as TSV-friendly rows: timestamp | event_name | severity | client_app | host | session | sql_text

SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

WITH events_xml AS (
  SELECT CAST(event_data AS XML) AS x
    FROM sys.fn_xe_file_target_read_file('xe_hotel_monitor*.xel', NULL, NULL, NULL)
)
SELECT
  CONVERT(VARCHAR(30), x.value('(event/@timestamp)[1]',                            'DATETIME2'), 126) AS ts,
  x.value('(event/@name)[1]',                                                      'NVARCHAR(64)')   AS event_name,
  ISNULL(x.value('(event/data[@name="severity"]/value)[1]',                        'INT'), 0)        AS severity,
  ISNULL(x.value('(event/data[@name="error_number"]/value)[1]',                    'INT'), 0)        AS error_number,
  x.value('(event/action[@name="client_app_name"]/value)[1]',                      'NVARCHAR(128)')  AS client_app,
  x.value('(event/action[@name="client_hostname"]/value)[1]',                      'NVARCHAR(128)')  AS client_host,
  x.value('(event/action[@name="session_id"]/value)[1]',                           'INT')            AS session_id,
  x.value('(event/data[@name="duration"]/value)[1]',                               'BIGINT') / 1000  AS duration_ms,
  REPLACE(REPLACE(x.value('(event/action[@name="sql_text"]/value)[1]',             'NVARCHAR(MAX)'), CHAR(13), ' '), CHAR(10), ' ') AS sql_text,
  ISNULL(REPLACE(REPLACE(x.value('(event/data[@name="message"]/value)[1]',         'NVARCHAR(MAX)'), CHAR(13), ' '), CHAR(10), ' '), '') AS error_message
FROM events_xml
WHERE x.value('(event/@timestamp)[1]', 'DATETIME2') > '$(since)'
ORDER BY x.value('(event/@timestamp)[1]', 'DATETIME2');
