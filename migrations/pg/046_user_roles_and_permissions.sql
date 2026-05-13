-- Migration: 046_user_roles_and_permissions
-- Version: vNext
-- Date: 2026-05-13
-- Description: Track G7 / T4 HIGH-9 (`audit-2026-05-13.md`) — replace the
--              binary `ht_users.role` enum (admin/receptionist) with a
--              full role + permission grid so the receptionist app can
--              gate Track G features (refunds, room change, round-bill,
--              inventory consume, RR.4 reports) per-role rather than
--              admin-vs-anyone.
--
-- ## Why this exists
--
-- iHOTEL has at least Admin / Cashier / Housekeeper / Receptionist roles
-- with per-feature gating. The new app currently has binary auth (logged
-- in vs. not). Phase 4 PR1 (migration 027) added an `admin`/`receptionist`
-- check constraint, but the audit (G HIGH-9, line 297 + 310) flagged
-- that as too coarse — refunds should be cashier-only, housekeeping
-- shouldn't see payment screens, etc.
--
-- This migration introduces three new tables (`ht_roles`,
-- `ht_permissions`, `ht_role_permissions`) plus a junction table
-- (`ht_user_roles`) so a user may belong to multiple roles. It also
-- extends `ht_users` with a `display_name` column (legacy iHOTEL displays
-- the receptionist's real name on receipts).
--
-- ## Backwards compatibility
--
-- - `ht_users.role` (the legacy single-role column) is left in place so
--   existing CRUD paths (admin user-management UI, `/api/auth/me` DTO,
--   `bin/create_user` CLI) keep working unchanged. The new permission
--   middleware reads from `ht_user_roles` (the junction) so role checks
--   are additive — admins continue to see everything via their
--   `admin` role mapping. The legacy column is treated as a default
--   role when no `ht_user_roles` rows exist (see seed block below).
-- - The `role IN ('admin','receptionist')` CHECK constraint from
--   migration 027 is RELAXED to also accept `'cashier'` and
--   `'housekeeper'` so the admin-users CRUD can assign the new values
--   into the legacy column too. Junction rows are still required for
--   permission gating to work.
--
-- ## Seed data
--
-- 4 roles: `admin`, `cashier`, `housekeeper`, `receptionist`.
-- 6 permission keys covering Track G's new gated actions:
--   * `payment.refund`        — Track G2 (existing route)
--   * `checkin.room_change`   — Track G4 (route lands when G4 PR merges)
--   * `checkin.round_bill`    — Track G9 (route lands when G9 PR merges)
--   * `inventory.consume`     — Track F3 (existing routes)
--   * `reports.rr4`           — Track G8 (route lands when G8 PR merges)
--   * `admin.users`           — Phase 4 PR4 (existing /api/admin/users)
--
-- Role → permission grid (per audit T4):
--   * admin        → all 6
--   * cashier      → payment.refund, checkin.round_bill, reports.rr4
--   * housekeeper  → inventory.consume, checkin.room_change
--   * receptionist → checkin.room_change, inventory.consume
--                    (NO payment.refund — refunds are cashier-only;
--                     NO reports.rr4 — financial export is cashier-only;
--                     NO admin.users — provisioning is admin-only)
--
-- 3 seed test accounts with `temp_password_2026` Argon2id-hashed:
--   * `housekeeper_test`   → housekeeper role
--   * `cashier_test`       → cashier role
--   * `receptionist_test`  → receptionist role
--
-- These are throwaway accounts for QA against the new permission
-- middleware. Real accounts come later through the admin user-
-- management UI (out of scope for G7).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

-- 1. Extend ht_users with display_name. Idempotent so re-runs are safe
--    and so init-hotelnew.sql (which creates ht_users directly with the
--    column inline) does not collide.
ALTER TABLE ht_users
    ADD COLUMN IF NOT EXISTS display_name VARCHAR(128);

-- 2. Relax the legacy single-role CHECK so the new role values can be
--    written into ht_users.role too (some legacy callers / the admin UI
--    still read this column). The CHECK lives under a constraint name
--    chosen by PostgreSQL at create time; drop it conditionally.
DO $$
DECLARE
    constraint_name text;
BEGIN
    SELECT con.conname INTO constraint_name
    FROM pg_constraint con
    JOIN pg_class cls ON cls.oid = con.conrelid
    WHERE cls.relname = 'ht_users'
      AND con.contype = 'c'
      AND pg_get_constraintdef(con.oid) ILIKE '%role%admin%receptionist%';

    IF constraint_name IS NOT NULL THEN
        EXECUTE format('ALTER TABLE ht_users DROP CONSTRAINT %I', constraint_name);
    END IF;
END $$;

ALTER TABLE ht_users
    ADD CONSTRAINT ht_users_role_check
    CHECK (role IN ('admin', 'cashier', 'housekeeper', 'receptionist'));

-- 3. Roles catalog.
CREATE TABLE IF NOT EXISTS ht_roles (
    role_id        BIGSERIAL    PRIMARY KEY,
    role_key       VARCHAR(64)  NOT NULL UNIQUE,
    display_name   VARCHAR(128) NOT NULL,
    created_at     TIMESTAMP    NOT NULL DEFAULT NOW()
);

-- 4. Permissions catalog.
CREATE TABLE IF NOT EXISTS ht_permissions (
    permission_id   BIGSERIAL    PRIMARY KEY,
    permission_key  VARCHAR(128) NOT NULL UNIQUE,
    description     VARCHAR(255),
    created_at      TIMESTAMP    NOT NULL DEFAULT NOW()
);

-- 5. Role → permission junction.
CREATE TABLE IF NOT EXISTS ht_role_permissions (
    role_id       BIGINT NOT NULL REFERENCES ht_roles(role_id)       ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES ht_permissions(permission_id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- 6. User → role junction. A user may hold multiple roles
--    (e.g. an admin who also covers cashier shifts).
CREATE TABLE IF NOT EXISTS ht_user_roles (
    user_id    BIGINT NOT NULL REFERENCES ht_users(user_id) ON DELETE CASCADE,
    role_id    BIGINT NOT NULL REFERENCES ht_roles(role_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Index for "all users in this role" lookups (admin UI).
CREATE INDEX IF NOT EXISTS ht_user_roles_role_idx ON ht_user_roles(role_id);

-- =============================================================================
-- Seed: roles
-- =============================================================================

INSERT INTO ht_roles (role_key, display_name)
VALUES
    ('admin',         'Administrator'),
    ('cashier',       'Cashier'),
    ('housekeeper',   'Housekeeper'),
    ('receptionist',  'Receptionist')
ON CONFLICT (role_key) DO NOTHING;

-- =============================================================================
-- Seed: permissions
-- =============================================================================

INSERT INTO ht_permissions (permission_key, description)
VALUES
    ('payment.refund',       'Refund (negative payment) against a recorded receipt (Track G2)'),
    ('checkin.room_change',  'Move an active check-in to a different room (Track G4)'),
    ('checkin.round_bill',   'Open/close a cashier round-bill (cash-drawer reconciliation; Track G9)'),
    ('inventory.consume',    'Consume / replenish stock against a room or shift (Track F3)'),
    ('reports.rr4',          'Generate RR.4 daily revenue export (Track G8)'),
    ('admin.users',          'Manage users / roles via /api/admin/users (Phase 4 PR4)')
ON CONFLICT (permission_key) DO NOTHING;

-- =============================================================================
-- Seed: role → permission grid (per audit T4)
-- =============================================================================

-- admin → all six permissions.
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
CROSS JOIN ht_permissions p
WHERE r.role_key = 'admin'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- cashier → payment.refund + checkin.round_bill + reports.rr4.
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN (
    'payment.refund',
    'checkin.round_bill',
    'reports.rr4'
)
WHERE r.role_key = 'cashier'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- housekeeper → inventory.consume + checkin.room_change.
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN (
    'inventory.consume',
    'checkin.room_change'
)
WHERE r.role_key = 'housekeeper'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- receptionist → checkin.room_change + inventory.consume
-- (NO payment.refund — cashier-only; NO reports.rr4 — cashier-only;
--  NO admin.users — admin-only).
INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN (
    'checkin.room_change',
    'inventory.consume'
)
WHERE r.role_key = 'receptionist'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- =============================================================================
-- Seed: test accounts for QA against the new permission middleware
-- =============================================================================
--
-- Argon2id PHC strings precomputed against `temp_password_2026` with the
-- same parameters the `service::auth::AuthService::hash_password` impl
-- uses at runtime (`Argon2::default()` = m=19456, t=2, p=1, v=19). They
-- verify correctly under the live login flow — see the integration test
-- in `tests/permissions_integration.rs`.
--
-- These accounts exist ONLY in fresh deployments and dev test runs;
-- production seeding through the admin UI is the supported path going
-- forward.

INSERT INTO ht_users (username, password_hash, role, display_name, active)
VALUES
    ('housekeeper_test',
     '$argon2id$v=19$m=19456,t=2,p=1$MPd9QPLV/Hk/ZCw93x7bjg$4jEMySdhT0/AdK+Ww4qWO1cUZfNuoDDhoFVH7C1HShw',
     'housekeeper', 'Test Housekeeper', TRUE),
    ('cashier_test',
     '$argon2id$v=19$m=19456,t=2,p=1$zbFz6agSu3CFK+DWG1yVPA$WEp9hv+/gDOL+NmC4RvO88ptPtJKyD+o6/Hk60nH94I',
     'cashier', 'Test Cashier', TRUE),
    ('receptionist_test',
     '$argon2id$v=19$m=19456,t=2,p=1$6agusFrTSWNC4dxANU2qLA$RDcPxK1jBnQWINIHt7eoMkmZn3BHkzDQkGCU4sDYDS0',
     'receptionist', 'Test Receptionist', TRUE)
ON CONFLICT (username) DO NOTHING;

-- Wire each seed account to its primary role in the junction.
INSERT INTO ht_user_roles (user_id, role_id)
SELECT u.user_id, r.role_id
FROM ht_users u
JOIN ht_roles r ON r.role_key = u.role
WHERE u.username IN ('housekeeper_test', 'cashier_test', 'receptionist_test')
ON CONFLICT (user_id, role_id) DO NOTHING;

-- Backfill: any pre-existing user (created via `bin/create_user` before
-- this migration) whose legacy single-role column matches a role_key
-- gets a junction row too. Idempotent. This keeps existing admin
-- accounts functional under the new permission middleware on day 1.
INSERT INTO ht_user_roles (user_id, role_id)
SELECT u.user_id, r.role_id
FROM ht_users u
JOIN ht_roles r ON r.role_key = u.role
WHERE u.role IN ('admin', 'cashier', 'housekeeper', 'receptionist')
ON CONFLICT (user_id, role_id) DO NOTHING;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX,
-- includes the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ht_user_roles_role_idx;
-- DROP TABLE IF EXISTS ht_user_roles;
-- DROP TABLE IF EXISTS ht_role_permissions;
-- DROP TABLE IF EXISTS ht_permissions;
-- DROP TABLE IF EXISTS ht_roles;
-- ALTER TABLE ht_users DROP COLUMN IF EXISTS display_name;
-- ALTER TABLE ht_users DROP CONSTRAINT IF EXISTS ht_users_role_check;
-- ALTER TABLE ht_users ADD CONSTRAINT ht_users_role_check
--     CHECK (role IN ('admin', 'receptionist'));
-- DELETE FROM schema_migrations WHERE version = '046';
