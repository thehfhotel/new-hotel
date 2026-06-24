-- Writeback idempotency ledger 2026-06-24 — app-owned `dbo.ht_writeback_ledger`.
--
-- Why: the writeback worker's sequential-allocator create recipes (the
-- ones that mint a legacy id app-side via MAX+1 and INSERT a new
-- HT_* row) have a crash window: once the recipe COMMITs on MSSQL but
-- BEFORE the worker marks the PG outbox/writeback_jobs row processed,
-- a crash leaves the job un-marked. On restart the worker replays it
-- and creates a DUPLICATE legacy row. This ledger closes that window:
-- the recipe INSERTs one ledger row keyed on the canonical writeback
-- idempotency_key (writeback_jobs.idempotency_key) IN THE SAME
-- TRANSACTION as the create. On replay the worker first probes the
-- ledger by idempotency_key — a hit means "already applied", so it
-- skips the INSERT and just re-marks the PG side (and reuses the
-- recorded legacy_ids). The PK on idempotency_key also makes a
-- concurrent double-apply hard-fail the second INSERT rather than
-- silently duplicating.
--
-- App-owned table: this is `dbo.ht_writeback_ledger`, OUR table — NOT
-- an iHOTEL HT_* table. Per CLAUDE.md the prohibition is on legacy
-- HT_*/View_* DDL; creating an app-owned `dbo.*` table is permitted
-- (same posture as `dbo.ht_legacy_migrations`). We deliberately do NOT
-- enable Change Tracking on it and add NO PK/DDL to any iHOTEL HT_*
-- table — iHOTEL never reads or writes it, and it is invisible to the
-- schema fingerprint / CT gate (which enumerate HT_*/View_* only).
--
-- Column names are LOAD-BEARING: the Rust writeback dispatcher INSERTs
-- and SELECTs these exact names (idempotency_key, intent_name,
-- aggregate_id, legacy_ids, applied_at). Do not rename.
--   * legacy_ids is NVARCHAR(MAX) holding the JSON map of the legacy
--     ids the recipe minted (e.g. {"booking_id":12345}) so a replay
--     can return them without re-querying.
--   * ix_ht_writeback_ledger_applied_at is reserved for a FUTURE retention
--     sweep (not yet implemented — see the ledger-retention-prune follow-up).
--     Until then the ledger grows ~one row per successful create-writeback;
--     the index is cheap to keep and the row size is tiny.
--
-- Idempotent: the whole body is guarded by OBJECT_ID(...) IS NULL so
-- re-runs no-op (same discipline as the rest of this directory).
--
-- Rollback: see 024_writeback_ledger.rollback.sql.
--
-- Application: applied per-site by `scripts/migrate-legacy-mssql.sh`
-- (HF Hotel `--site hfhotel`, HF Ville `--site hfville`) during the
-- CI/CD deploy, BEFORE the writeback worker is recreated — so the
-- worker's first create recipe finds the ledger already present.
-- Manual fallback (one-shot Docker sqlcmd):
--   ssh evergreen 'cat <path>/024_writeback_ledger.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'
-- For HF Ville: swap -S for the Ville WireGuard endpoint, -d HOTEL.

SET NOCOUNT ON;
GO

-- App-owned dbo.* table — no HT_* DDL, no Change Tracking. Guarded so
-- re-runs no-op. GO separators are kept per the directory's batch
-- discipline (the CREATE INDEX must see committed table metadata).
IF OBJECT_ID('dbo.ht_writeback_ledger','U') IS NULL
BEGIN
    CREATE TABLE dbo.ht_writeback_ledger (
        idempotency_key UNIQUEIDENTIFIER NOT NULL
            CONSTRAINT PK_ht_writeback_ledger PRIMARY KEY CLUSTERED,
        intent_name     VARCHAR(40)      NOT NULL,
        aggregate_id    UNIQUEIDENTIFIER NOT NULL,
        legacy_ids      NVARCHAR(MAX)    NULL,
        applied_at      DATETIME         NOT NULL DEFAULT GETDATE()
    );
    CREATE INDEX ix_ht_writeback_ledger_applied_at ON dbo.ht_writeback_ledger(applied_at);
END
GO

PRINT '== Writeback ledger apply complete — verifying ==';
SELECT
    t.name AS table_name,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index,
    CASE WHEN EXISTS (SELECT 1 FROM sys.indexes
                       WHERE object_id=t.object_id
                         AND name='ix_ht_writeback_ledger_applied_at')
         THEN 'YES' ELSE 'no' END AS applied_at_index
  FROM sys.tables t
 WHERE t.name = 'ht_writeback_ledger'
 ORDER BY t.name;
