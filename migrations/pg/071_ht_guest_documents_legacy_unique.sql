-- Migration: 071_ht_guest_documents_legacy_unique
-- Version: vNext
-- Date: 2026-07-01
-- Guest-image sync-in dedup hardening (legacy Tb_Save_Image read-in poll).
--
-- ## Background
--
-- The sync worker (`bin/sync.rs::sync_guest_documents`) pulls the reconstructed
-- Thai-ID card / face-photo images iHOTEL stores in legacy `Tb_Save_Image` into
-- the canonical `ht_guest_documents` table (doc_source='legacy'), so the
-- registration form serves the SAME image iHOTEL prints without the API ever
-- touching legacy (architecture rule). The poll is idempotent via an UPSERT on
-- the legacy row id (`Tb_Save_Image.id` → `doc_legacy_id`); this partial UNIQUE
-- index is the conflict target so a re-poll (or crash-after-commit replay) never
-- lands the same legacy image twice.
--
-- App-native documents (`doc_source` in 'chip'/'scanner'/'webcam') carry no
-- legacy id — the partial WHERE exempts NULL `doc_legacy_id` rows so multiple
-- app-captured documents per stay stay allowed.
--
-- ## Pre-flight (MANDATORY before deploy)
--
-- Creation FAILS if existing non-NULL `doc_legacy_id` duplicates already exist.
-- Pre-flight BOTH hotelnew + hotelville:
--   SELECT doc_legacy_id, COUNT(*)
--     FROM ht_guest_documents
--    WHERE doc_legacy_id IS NOT NULL
--    GROUP BY doc_legacy_id
--   HAVING COUNT(*) > 1;
-- Resolve any duplicates before applying. (Fresh feature — expected empty.)

-- UP MIGRATION

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_guest_documents_legacy_id
    ON ht_guest_documents (doc_legacy_id) WHERE doc_legacy_id IS NOT NULL;

-- DOWN MIGRATION (commented)
-- DROP INDEX IF EXISTS ux_ht_guest_documents_legacy_id;
-- DELETE FROM schema_migrations WHERE version = '071';
