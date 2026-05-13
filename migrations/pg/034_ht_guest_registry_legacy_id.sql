-- Migration: 034_ht_guest_registry_legacy_id
-- Version: 2.63.12
-- Date: 2026-05-13
-- Track E1 (audit 2026-05-13) — companion-guest sync prerequisite.
--
-- The new `GuestRegistryMapper` for `HT_CheckIn_Other_People`
-- (T2 HIGH-3) keys on the legacy IDENTITY column so the iHOTEL
-- DELETE-then-REINSERT edit pattern (FrmCheckIn.cs:9975) produces a
-- clean UPDATE on the canonical side instead of accumulating
-- duplicate companion rows.
--
-- This migration adds the minimum bookkeeping column to
-- `ht_guest_registry`:
--
-- * `guest_legacy_id INTEGER UNIQUE` — the legacy
--   `HT_CheckIn_Other_People.id`. NULL for rows authored by our own
--   app (no legacy IDENTITY equivalent yet); NOT NULL for sync-driven
--   rows. The UNIQUE constraint enables `ON CONFLICT (guest_legacy_id)`
--   in the mapper.
--
-- E2 (Track E phase 2) will tackle the wider column expansion for
-- `ht_customers` + `ht_rooms_new`. This migration is intentionally
-- scoped to the single column needed to unblock the mapper.

-- UP MIGRATION

ALTER TABLE ht_guest_registry
    ADD COLUMN IF NOT EXISTS guest_legacy_id INTEGER;

-- Unique constraint as a separate statement so re-application is
-- idempotent (ADD CONSTRAINT IF NOT EXISTS is not supported pre-PG 17
-- for unique constraints; do the existence check explicitly).
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
          FROM pg_constraint
         WHERE conname = 'uq_ht_guest_registry_legacy_id'
           AND conrelid = 'ht_guest_registry'::regclass
    ) THEN
        ALTER TABLE ht_guest_registry
            ADD CONSTRAINT uq_ht_guest_registry_legacy_id
            UNIQUE (guest_legacy_id);
    END IF;
END$$;

-- DOWN MIGRATION (commented — destructive)
-- ALTER TABLE ht_guest_registry
--     DROP CONSTRAINT IF EXISTS uq_ht_guest_registry_legacy_id;
-- ALTER TABLE ht_guest_registry DROP COLUMN IF EXISTS guest_legacy_id;
