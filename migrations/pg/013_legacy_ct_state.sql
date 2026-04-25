-- Migration: 013_legacy_ct_state
-- Version: 2.8.2
-- Date: 2026-04-25
-- Description: Single-row Change Tracking watermark. Tracks the highest
--              SYS_CHANGE_VERSION the CT watcher (bin/sync.rs, Phase 5) has
--              successfully imported from MSSQL. On worker restart we resume
--              from this point. Per docs/architecture.md §4d-tris.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS legacy_ct_state (
    id                  BIGINT       PRIMARY KEY DEFAULT 1 CHECK (id = 1),  -- single row
    last_seen_version   BIGINT       NOT NULL DEFAULT 0,
    last_polled_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

INSERT INTO legacy_ct_state (id, last_seen_version)
VALUES (1, 0)
ON CONFLICT DO NOTHING;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here — duplicates in the same TX violate
-- schema_migrations_version_key.

-- =============================================================================
-- DOWN MIGRATION (commented — apply manually for rollback)
-- =============================================================================
-- DROP TABLE IF EXISTS legacy_ct_state;
-- DELETE FROM schema_migrations WHERE version = '013';
