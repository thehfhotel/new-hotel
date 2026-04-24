-- 07-xe-read.sql
-- Read every event captured by the XE session out of the .xel file as a
-- structured result set: timestamp, event_name, app_name, host, login,
-- session_id, sql_text, duration_ms.
--
-- The file path resolution: sys.fn_xe_file_target_read_file accepts a
-- pattern with a wildcard, so we use 'xe_hotel_legacy_capture*.xel' to
-- catch the rolled-over files too.

SET QUOTED_IDENTIFIER ON;
SET NOCOUNT ON;

WITH events_xml AS (
  SELECT CAST(event_data AS XML) AS event_xml
  FROM sys.fn_xe_file_target_read_file(
    'xe_hotel_legacy_capture*.xel',  -- relative to SQL Server log dir
    NULL, NULL, NULL
  )
)
SELECT
  event_xml.value('(event/@timestamp)[1]',                         'DATETIME2')   AS event_time_utc,
  event_xml.value('(event/@name)[1]',                              'NVARCHAR(64)')  AS event_name,
  event_xml.value('(event/action[@name="client_app_name"]/value)[1]','NVARCHAR(128)') AS client_app_name,
  event_xml.value('(event/action[@name="client_hostname"]/value)[1]','NVARCHAR(128)') AS client_host,
  event_xml.value('(event/action[@name="username"]/value)[1]',     'NVARCHAR(128)')  AS username,
  event_xml.value('(event/action[@name="session_id"]/value)[1]',   'INT')          AS session_id,
  event_xml.value('(event/action[@name="transaction_id"]/value)[1]','BIGINT')      AS xact_id,
  event_xml.value('(event/data[@name="duration"]/value)[1]',       'BIGINT') / 1000 AS duration_ms,
  event_xml.value('(event/action[@name="sql_text"]/value)[1]',     'NVARCHAR(MAX)') AS sql_text_action,
  event_xml.value('(event/data[@name="statement"]/value)[1]',      'NVARCHAR(MAX)') AS statement_data,
  event_xml.value('(event/data[@name="batch_text"]/value)[1]',     'NVARCHAR(MAX)') AS batch_text
FROM events_xml
ORDER BY event_time_utc;
