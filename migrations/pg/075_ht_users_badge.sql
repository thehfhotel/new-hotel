-- Migration: 075_ht_users_badge
-- Version: vNext
-- Date: 2026-07-06
-- Description: NFC staff-card login — adds a nullable `badge` column to
--              `ht_users` so a tap of a staff NFC card (resolved against
--              the central HF-ID registry) can be mapped to a local user
--              row and mint an app session without a password.
--
-- ## Why this exists
--
-- The reader device POSTs a card UID to `POST /api/reader/scan`. The PMS
-- resolves it against the central HF-ID service and, when authorized,
-- maps the returned `badge` to a `ht_users` row via
-- `service::reader::find_or_provision_user_by_badge` (auto-provisioning a
-- `role='receptionist'` card-only account on first tap). `badge` is the
-- join key. Rows with NULL badge simply have no card and keep using the
-- password / CF-Access paths — this is an OPTIONAL alternate lookup key,
-- never a credential (the card→person authority lives centrally in HF-ID;
-- the PMS only trusts the resolve response).
--
-- ## Uniqueness
--
-- One badge maps to at most one local user, enforced by a partial UNIQUE
-- index (NULLs exempt — most rows keep NULL badge). Mirrors the pattern
-- established by migration 074's `ux_ht_users_email_lower`.
--
-- ## Auto-provisioned accounts
--
-- A first tap of an unknown-but-authorized badge inserts a card-only
-- account: `username='card-<badge>'`, `role='receptionist'`,
-- `active=TRUE`, and a password_hash sentinel (`!card-only-no-password`)
-- that is NOT a valid Argon2 PHC string, so `verify_password` always
-- returns false and the account can never be logged into with a password.
-- The receptionist junction row (migration 046 `ht_user_roles`) is written
-- in the same transaction so permission resolution works on day one.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row). PG-CANONICAL
-- ONLY — legacy iHOTEL has no badge column; no sync, no writeback.

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_users
    ADD COLUMN IF NOT EXISTS badge VARCHAR(50);

-- One badge → one user; NULL-tolerant via the partial predicate so the
-- vast majority of rows (no card) stay exempt from the uniqueness check.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_users_badge
    ON ht_users (badge)
    WHERE badge IS NOT NULL;

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX,
-- includes the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ux_ht_users_badge;
-- ALTER TABLE ht_users DROP COLUMN IF EXISTS badge;
-- DELETE FROM schema_migrations WHERE version = '075';
