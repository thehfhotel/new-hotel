-- 00-prereqs.sql
-- Sanity checks before the spike: server version, perms, current DB,
-- and identify which login/host the 3rd-party app uses (so we can filter
-- the Extended Events session to its sessions only).

SET NOCOUNT ON;
PRINT '==== Server identity ====';
SELECT
  @@SERVERNAME           AS server_name,
  @@VERSION              AS sql_version,
  DB_NAME()              AS current_db,
  SUSER_SNAME()          AS current_login,
  HOST_NAME()            AS client_host,
  GETDATE()              AS server_now;

PRINT '';
PRINT '==== Required permissions check ====';
-- ALTER ANY EVENT SESSION is server-level; needed to create XE session.
SELECT
  HAS_PERMS_BY_NAME(NULL, NULL, 'ALTER ANY EVENT SESSION') AS has_alter_xe,
  HAS_PERMS_BY_NAME(NULL, NULL, 'VIEW SERVER STATE')        AS has_view_server_state,
  IS_SRVROLEMEMBER('sysadmin')                              AS is_sysadmin;
-- All three should be 1 if you logged in as sa.

PRINT '';
PRINT '==== Currently connected sessions (find the 3rd-party app) ====';
-- Look for sessions whose program_name matches the .NET app, e.g.
-- ".Net SqlClient Data Provider" or a custom name. Note the login_name
-- and host_name — we'll filter the XE session to these.
SELECT
  s.session_id,
  s.login_name,
  s.host_name,
  s.program_name,
  s.client_interface_name,
  s.login_time,
  s.last_request_start_time,
  s.status
FROM sys.dm_exec_sessions s
WHERE s.is_user_process = 1
ORDER BY s.last_request_start_time DESC;

PRINT '';
PRINT '==== Available databases ====';
SELECT name, database_id, create_date, state_desc, recovery_model_desc
FROM sys.databases
WHERE database_id > 4  -- skip system DBs
ORDER BY name;

PRINT '';
PRINT '==== SQL Server log path (where xe file will land) ====';
-- The XE session writes the .xel file to this directory by default.
SELECT
  SERVERPROPERTY('ErrorLogFileName') AS errorlog_path,
  SERVERPROPERTY('InstanceDefaultDataPath') AS data_path,
  SERVERPROPERTY('InstanceDefaultLogPath')  AS log_path;
