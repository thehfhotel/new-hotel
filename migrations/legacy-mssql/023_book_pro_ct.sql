-- Phase 5/E2 — iHOTEL coexistence audit 2026-06-11 (P2 gap closure):
-- enable Change Tracking on `HT_Book_Pro` (pre-booked products).
--
-- Why: FrmAddBook2 attaches product lines (food/drinks pre-ordered) to
-- a booking by INSERTing `HT_Book_Pro` rows (COMPAT_CHEATSHEET line
-- 711-716, §3.4 step 3.5). The table had no PK and no CT subscription,
-- so iHOTEL-entered booking products were invisible to the new app —
-- a booking with products checked in via the new app silently dropped
-- those charges. The new `BookProMirrorMapper`
-- (sync/mappers/mirror.rs) mirrors rows into
-- `legacy_mirror.ht_book_pro` (migrations/pg/056).
--
-- Pre-flight (verified against `hotel-backend/schema-baseline.txt`
-- lines 179-187 and the regenerated `docs/legacy-app/SCHEMA.sql`,
-- both from live prod 2026-06-11; re-verify at apply time):
--   * No existing PRIMARY KEY on `HT_Book_Pro` (only HT_Booking_Notes
--     has a PK in the baseline's "==== Primary keys ====" section)
--   * No existing per-table CT
--   * `id` is IDENTITY (`is_identity=1`) and already NOT NULL
--     (`is_nullable=0`) per INFORMATION_SCHEMA.COLUMNS — so no
--     ALTER COLUMN is needed (same as HT_CheckIn_Product /
--     HT_Changed_Room / HT_Bill_Debt_Ds in migration 021)
--   * approx_row_count = 0 at HF Hotel (schema-baseline.txt line 13) —
--     adding a PK + enabling CT is microseconds; per migration 021's
--     rationale, empty tables get the same treatment so future data
--     never appears without observability
--
-- Rollback: see 023_book_pro_ct.rollback.sql.
--
-- Application:
--   ssh evergreen 'cat <path>/023_book_pro_ct.sql | \
--     docker run --rm -i --network host \
--       --entrypoint /opt/mssql-tools18/bin/sqlcmd \
--       mcr.microsoft.com/mssql/server:2022-latest \
--       -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d db -W'
--
-- For HF Ville: swap -S for the Ville WireGuard endpoint, -d HOTEL.
-- Apply BEFORE deploying the binary that adds HT_Book_Pro to
-- CT_ENABLED_TABLES, so the watcher's first poll of the table finds
-- CT already enabled (matches the 022 → migration-033-seed rollout
-- order; migration pg/056 seeds the watermark rows on deploy).

SET NOCOUNT ON;
GO

-- HT_Book_Pro — IDENTITY id, already NOT NULL per the live baseline,
-- so no ALTER COLUMN batch is needed (migration 021 pattern for
-- identity-PK tables). GO separators between DDL statements are kept
-- per the Msg 8111 discipline documented in migration 020/021.
ALTER TABLE HT_Book_Pro ADD CONSTRAINT PK_HT_Book_Pro PRIMARY KEY CLUSTERED (id);
GO
ALTER TABLE HT_Book_Pro ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
GO

PRINT '== Phase 5/E2 apply complete — verifying ==';
SELECT
    t.name AS table_name,
    CASE WHEN ct.object_id IS NOT NULL THEN 'YES' ELSE 'no' END AS ct_enabled,
    ISNULL((SELECT name FROM sys.indexes WHERE object_id=t.object_id AND is_primary_key=1), '(none)') AS pk_index
  FROM sys.tables t
  LEFT JOIN sys.change_tracking_tables ct ON ct.object_id=t.object_id
 WHERE t.name IN ('HT_Book_Pro')
 ORDER BY t.name;
