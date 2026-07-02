-- Migration: 074_ht_users_email
-- Version: vNext
-- Date: 2026-07-02
-- Description: Cloudflare Access auto-login — adds a nullable `email`
--              column to `ht_users` so a CF-Access-authenticated staff
--              member (identified by the verified `email` claim in the
--              `Cf-Access-Jwt-Assertion` JWT) can be mapped to a local
--              user row and receive an app session without a second
--              password login. Password login is untouched — email is
--              an OPTIONAL alternate lookup key, never a credential.
--
-- ## Why this exists
--
-- The app sits behind Cloudflare Access; staff already authenticate at
-- the edge. `POST /api/auth/cf-login` (routes::auth::cf_login) verifies
-- the CF Access JWT (RS256 against the team JWKS) and resolves the
-- verified email to a `ht_users` row via
-- `UserRepository::get_by_email` (case-insensitive). Rows with NULL
-- email simply cannot auto-login and keep using the password form.
--
-- ## Uniqueness
--
-- Lookup is `lower(email)`, so uniqueness is enforced case-insensitively
-- via a partial functional UNIQUE index (NULLs exempt — most rows keep
-- NULL email).
--
-- ## Backfill
--
-- Only usernames with a confidently known mailbox are backfilled:
--   * winut → winut.hf@gmail.com
--   * winai → winai.sdy@gmail.com
-- Everyone else (test seed accounts, any other operator-created user)
-- stays NULL and continues password-only. The UPDATEs are no-ops on
-- databases where those usernames don't exist (e.g. fresh CI seeds).
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row). PG-CANONICAL
-- ONLY — legacy `ht_login_name` knows nothing about email; no sync, no
-- writeback.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_users
    ADD COLUMN IF NOT EXISTS email VARCHAR(255);

-- Case-insensitive uniqueness; NULL-tolerant via the partial predicate.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_users_email_lower
    ON ht_users (LOWER(email))
    WHERE email IS NOT NULL;

-- Backfill the two confidently-mapped operators. Idempotent (`email IS
-- NULL` guard) and 0-row-safe when the usernames don't exist.
UPDATE ht_users SET email = 'winut.hf@gmail.com'
WHERE username = 'winut' AND email IS NULL;

UPDATE ht_users SET email = 'winai.sdy@gmail.com'
WHERE username = 'winai' AND email IS NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX,
-- includes the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ux_ht_users_email_lower;
-- ALTER TABLE ht_users DROP COLUMN IF EXISTS email;
-- DELETE FROM schema_migrations WHERE version = '074';
