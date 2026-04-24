-- 06-snapshot-rowcounts.sql
-- Same as 02-snapshot-rowcounts.sql, run AFTER the receptionist is done.
-- Diff this output against 02-rowcounts-before.txt to see which tables
-- changed during the session (any row_count delta or checksum change).

SET NOCOUNT ON;

DECLARE @sql NVARCHAR(MAX) = N'';
SELECT @sql = STRING_AGG(
  'SELECT ''' + s.name + '.' + t.name + ''' AS table_name, '
  + 'COUNT_BIG(*) AS row_count, '
  + 'CHECKSUM_AGG(BINARY_CHECKSUM(*)) AS data_checksum '
  + 'FROM ' + QUOTENAME(s.name) + '.' + QUOTENAME(t.name) + ' WITH (NOLOCK)',
  ' UNION ALL '
)
FROM sys.tables t
JOIN sys.schemas s ON s.schema_id = t.schema_id
WHERE OBJECT_SCHEMA_NAME(t.object_id) NOT IN ('sys');

SET @sql = @sql + ' ORDER BY table_name';
EXEC sp_executesql @sql;
