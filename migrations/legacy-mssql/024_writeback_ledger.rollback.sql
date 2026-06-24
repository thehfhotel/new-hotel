-- Writeback ledger rollback — drop the app-owned `dbo.ht_writeback_ledger`
-- table. Reverts to the pre-2026-06-24 state.
--
-- Use only if the writeback ledger needs to be backed out (e.g. the
-- dispatcher revision that depends on it is reverted). Dropping the
-- table also drops its PK + ix_ht_writeback_ledger_applied_at index.
-- No iHOTEL HT_* table is touched — this is our app-owned dbo.* table,
-- so there is no CT subscription or HT_* constraint to revert.
--
-- Safe to run multiple times — guarded by OBJECT_ID(...) IS NOT NULL,
-- so re-running just no-ops. Pattern mirrors
-- 023_book_pro_ct.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/024_writeback_ledger.rollback.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'
-- For HF Ville: swap -S for the Ville WireGuard endpoint, -d HOTEL.

SET NOCOUNT ON;
GO

-- dbo.ht_writeback_ledger
IF OBJECT_ID('dbo.ht_writeback_ledger','U') IS NOT NULL
    DROP TABLE dbo.ht_writeback_ledger;
GO

PRINT '== Writeback ledger rollback complete — verifying ==';
SELECT
    CASE WHEN OBJECT_ID('dbo.ht_writeback_ledger','U') IS NOT NULL
         THEN 'present' ELSE '(dropped)' END AS ht_writeback_ledger;
