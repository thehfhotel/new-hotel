-- Migration: 027_create_ht_users
-- Version: 2.60.0
-- Date: 2026-05-10
-- Description: Phase 4 PR1 — backend authentication foundation.
--              Creates the `ht_users` table holding local username/password
--              credentials (Argon2id password hashes) and role assignments
--              for admin / receptionist users.
--
-- ## Why this exists
--
-- Phase 4 introduces local username + password authentication (no SSO)
-- backed by HttpOnly cookies and a server-side session table (see
-- migration 028). PR1 lays the schema + repository + service-layer
-- foundation so PR2 can wire HTTP routes + Axum middleware on top
-- without any further DDL.
--
-- ## Columns
--
--   user_id         BIGSERIAL  PK
--   username        VARCHAR(64) NOT NULL UNIQUE   -- ASCII login handle
--   password_hash   TEXT        NOT NULL           -- Argon2id PHC string
--                                                  -- (~95-110 chars; TEXT
--                                                  -- avoids brittle width)
--   role            VARCHAR(16) NOT NULL CHECK (role IN ('admin','receptionist'))
--   active          BOOLEAN     NOT NULL DEFAULT TRUE
--   created_at      TIMESTAMP   NOT NULL DEFAULT NOW()
--   last_login_at   TIMESTAMP                       -- NULL until first login
--
-- ## Notes
--
-- - `last_login_at` is updated by `AuthService::login` inside the same
--   transaction that creates the session row (migration 028).
-- - `active = FALSE` is the soft-delete pattern used by the rest of the
--   schema (`ht_customers.cust_active`, etc.). The login service checks
--   this flag and rejects deactivated users.
-- - No FK from `ht_sessions.user_id` is added here; that constraint
--   lives in migration 028 alongside the sessions table itself.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_users (
    user_id        BIGSERIAL    PRIMARY KEY,
    username       VARCHAR(64)  NOT NULL UNIQUE,
    password_hash  TEXT         NOT NULL,
    role           VARCHAR(16)  NOT NULL CHECK (role IN ('admin', 'receptionist')),
    active         BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at     TIMESTAMP    NOT NULL DEFAULT NOW(),
    last_login_at  TIMESTAMP
);

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP TABLE IF EXISTS ht_users;
-- DELETE FROM schema_migrations WHERE version = '027';
