-- 01-baseline-schema.sql
-- Full schema dump: tables, columns, FKs, indexes, view definitions,
-- trigger definitions, stored procs, functions. Read-only.

SET NOCOUNT ON;

PRINT '==== Tables (filter to HT_* and View_* — adjust if your prefix differs) ====';
SELECT
  s.name + '.' + t.name        AS table_name,
  t.create_date,
  t.modify_date,
  p.rows                       AS approx_row_count
FROM sys.tables t
JOIN sys.schemas s ON s.schema_id = t.schema_id
JOIN sys.partitions p ON p.object_id = t.object_id AND p.index_id IN (0,1)
ORDER BY t.name;

PRINT '';
PRINT '==== Columns ====';
SELECT
  s.name + '.' + t.name        AS table_name,
  c.column_id                  AS ord,
  c.name                       AS column_name,
  ty.name                      AS data_type,
  c.max_length, c.precision, c.scale,
  c.is_nullable, c.is_identity,
  OBJECT_DEFINITION(c.default_object_id) AS default_value
FROM sys.tables t
JOIN sys.schemas s ON s.schema_id = t.schema_id
JOIN sys.columns c ON c.object_id = t.object_id
JOIN sys.types   ty ON ty.user_type_id = c.user_type_id
ORDER BY t.name, c.column_id;

PRINT '';
PRINT '==== Primary keys ====';
SELECT
  OBJECT_NAME(i.object_id)     AS table_name,
  i.name                       AS index_name,
  STRING_AGG(c.name, ', ')     AS pk_columns
FROM sys.indexes i
JOIN sys.index_columns ic ON ic.object_id = i.object_id AND ic.index_id = i.index_id
JOIN sys.columns c ON c.object_id = ic.object_id AND c.column_id = ic.column_id
WHERE i.is_primary_key = 1
GROUP BY i.object_id, i.name
ORDER BY OBJECT_NAME(i.object_id);

PRINT '';
PRINT '==== Foreign keys ====';
SELECT
  fk.name                                              AS fk_name,
  OBJECT_NAME(fk.parent_object_id)                     AS parent_table,
  cp.name                                              AS parent_column,
  OBJECT_NAME(fk.referenced_object_id)                 AS ref_table,
  cr.name                                              AS ref_column,
  fk.delete_referential_action_desc                    AS on_delete,
  fk.update_referential_action_desc                    AS on_update
FROM sys.foreign_keys fk
JOIN sys.foreign_key_columns fkc ON fkc.constraint_object_id = fk.object_id
JOIN sys.columns cp ON cp.object_id = fkc.parent_object_id    AND cp.column_id = fkc.parent_column_id
JOIN sys.columns cr ON cr.object_id = fkc.referenced_object_id AND cr.column_id = fkc.referenced_column_id
ORDER BY parent_table, fk.name;

PRINT '';
PRINT '==== Indexes (excluding PK / unique constraints) ====';
SELECT
  OBJECT_NAME(i.object_id)     AS table_name,
  i.name                       AS index_name,
  i.type_desc,
  i.is_unique,
  STRING_AGG(c.name, ', ') WITHIN GROUP (ORDER BY ic.key_ordinal) AS columns
FROM sys.indexes i
JOIN sys.index_columns ic ON ic.object_id = i.object_id AND ic.index_id = i.index_id
JOIN sys.columns c       ON c.object_id  = ic.object_id AND c.column_id  = ic.column_id
WHERE i.is_primary_key = 0
  AND i.type > 0
  AND OBJECT_SCHEMA_NAME(i.object_id) NOT IN ('sys','INFORMATION_SCHEMA')
GROUP BY i.object_id, i.name, i.type_desc, i.is_unique
ORDER BY table_name, index_name;

PRINT '';
PRINT '==== Views (definitions — critical: shows whether updatable) ====';
SELECT
  s.name + '.' + v.name        AS view_name,
  m.definition                 AS definition_sql
FROM sys.views v
JOIN sys.schemas s ON s.schema_id = v.schema_id
JOIN sys.sql_modules m ON m.object_id = v.object_id
ORDER BY v.name;

PRINT '';
PRINT '==== Triggers (definitions — what fires on INSERT/UPDATE/DELETE) ====';
SELECT
  OBJECT_NAME(tr.parent_id)    AS table_name,
  tr.name                      AS trigger_name,
  tr.type_desc,
  tr.is_disabled,
  m.definition                 AS definition_sql
FROM sys.triggers tr
JOIN sys.sql_modules m ON m.object_id = tr.object_id
WHERE tr.parent_class = 1  -- table-level
ORDER BY table_name, trigger_name;

PRINT '';
PRINT '==== Stored procedures (definitions) ====';
SELECT
  s.name + '.' + p.name        AS proc_name,
  p.create_date,
  p.modify_date,
  m.definition                 AS definition_sql
FROM sys.procedures p
JOIN sys.schemas s ON s.schema_id = p.schema_id
JOIN sys.sql_modules m ON m.object_id = p.object_id
WHERE OBJECT_SCHEMA_NAME(p.object_id) NOT IN ('sys')
ORDER BY p.name;

PRINT '';
PRINT '==== Scalar / table-valued functions ====';
SELECT
  s.name + '.' + o.name        AS func_name,
  o.type_desc,
  m.definition                 AS definition_sql
FROM sys.objects o
JOIN sys.schemas s ON s.schema_id = o.schema_id
JOIN sys.sql_modules m ON m.object_id = o.object_id
WHERE o.type IN ('FN','IF','TF','FS','FT')
ORDER BY o.name;

PRINT '';
PRINT '==== Sequences (if any — alternative to identity) ====';
SELECT name, current_value, increment, start_value, minimum_value, maximum_value
FROM sys.sequences
ORDER BY name;
