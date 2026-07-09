-- Migration: 078_loyalty_channel
-- Version: vNext
-- Date: 2026-07-10
-- Description: Loyalty-app booking channel + membership link. Adds
--              `ht_customers.cust_membership_id` (the guest's loyalty
--              membership id, typed/scanned at the desk or attached by the
--              channel API) and `ht_bookings.book_hold_expires_at` (the
--              payment-hold deadline for channel-created TENTATIVE bookings).
--
-- ## Why this exists
--
-- The loyalty app (separate deployment) becomes (a) a first-party booking
-- channel into this PMS (`/api/channel/*`, bearer-token gated, ships DARK
-- behind LOYALTY_CHANNEL_ENABLED) and (b) a loyalty program fed by PMS
-- checkouts (the checkout hook posts stays for guests with a linked
-- membership).
--
-- * `cust_membership_id` — nullable, PG-CANONICAL ONLY. Legacy `HT_Customers`
--   has no membership column, so this is NEVER mirrored to MSSQL (same policy
--   as `cust_dob`, migration 069). Set/cleared via
--   `PUT /api/customers/{id}/membership` (desk flow) or attached by
--   `POST /api/channel/bookings` when the loyalty app supplies one.
-- * `book_hold_expires_at` — nullable TIMESTAMPTZ, PG-CANONICAL ONLY. Only
--   channel-created holds (`book_channel='loyalty'`, `book_status='pending'`)
--   carry a value; the scheduler's expiry sweep auto-cancels holds past this
--   deadline (belt-and-braces behind the loyalty app's own release call).
--   The cancel rides the EXISTING booking-cancel writeback path, so no new
--   legacy write shape is introduced.
--
-- ALTER-only (no new table → no CARDINALITY_MAP.md row; annotates the existing
-- `ht_customers` / `ht_bookings` rows' notes instead).

-- =============================================================================
-- UP MIGRATION
-- =============================================================================

ALTER TABLE ht_customers
    ADD COLUMN IF NOT EXISTS cust_membership_id VARCHAR(64);

COMMENT ON COLUMN ht_customers.cust_membership_id IS
    'Loyalty membership id (loyalty-app). PG-canonical only — legacy HT_Customers has no counterpart; never written back to MSSQL.';

-- Membership → guest lookup (checkout hook joins on it; desk lookups by
-- member QR). Partial: only linked guests are indexed.
CREATE INDEX IF NOT EXISTS ix_ht_customers_membership_id
    ON ht_customers (cust_membership_id) WHERE cust_membership_id IS NOT NULL;

ALTER TABLE ht_bookings
    ADD COLUMN IF NOT EXISTS book_hold_expires_at TIMESTAMPTZ;

COMMENT ON COLUMN ht_bookings.book_hold_expires_at IS
    'Payment-hold deadline for loyalty-channel TENTATIVE bookings (book_channel=''loyalty'', book_status=''pending''). PG-canonical only. The scheduler sweep cancels holds past this instant.';

-- Sweep lookup: pending holds with a deadline. Partial predicate keeps the
-- index tiny (every non-channel booking has NULL here).
CREATE INDEX IF NOT EXISTS ix_ht_bookings_hold_expiry
    ON ht_bookings (book_hold_expires_at)
    WHERE book_hold_expires_at IS NOT NULL AND book_status = 'pending';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented for reference)
-- =============================================================================
-- DROP INDEX IF EXISTS ix_ht_bookings_hold_expiry;
-- ALTER TABLE ht_bookings DROP COLUMN IF EXISTS book_hold_expires_at;
-- DROP INDEX IF EXISTS ix_ht_customers_membership_id;
-- ALTER TABLE ht_customers DROP COLUMN IF EXISTS cust_membership_id;
-- DELETE FROM schema_migrations WHERE version = '078';
