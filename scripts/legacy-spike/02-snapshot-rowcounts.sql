-- 02-snapshot-rowcounts.sql
-- Row count + lightweight checksum per table. Run BEFORE the receptionist
-- starts. Re-run as 06-snapshot-rowcounts.sql AFTER. Diff the two files
-- column by column to identify which tables changed during the session.
--
-- CHECKSUM_AGG over all rows is fast and detects ANY data change in the
-- selected columns. We use * (all columns) which gives the strongest
-- signal at the cost of more CPU on wide tables.

SET NOCOUNT ON;

-- Loop over all base tables and emit one row per table with count + checksum.
-- Build the union dynamically. STRING_AGG truncates at 8000 bytes when its
-- inputs aren't already MAX-typed, so we pre-CAST each fragment to NVARCHAR(MAX).
DECLARE @sql NVARCHAR(MAX) = N'';
SELECT @sql = STRING_AGG(CAST(
  'SELECT ''' + s.name + '.' + t.name + ''' AS table_name, '
  + 'COUNT_BIG(*) AS row_count, '
  + 'CHECKSUM_AGG(BINARY_CHECKSUM(*)) AS data_checksum '
  + 'FROM ' + QUOTENAME(s.name) + '.' + QUOTENAME(t.name) + ' WITH (NOLOCK)'
  AS NVARCHAR(MAX)),
  N' UNION ALL '
)
FROM sys.tables t
JOIN sys.schemas s ON s.schema_id = t.schema_id
WHERE OBJECT_SCHEMA_NAME(t.object_id) NOT IN ('sys');

-- Wrap in a derived table so ORDER BY applies to the union as a whole.
SET @sql = N'SELECT * FROM (' + @sql + N') AS t ORDER BY table_name';
EXEC sp_executesql @sql;
