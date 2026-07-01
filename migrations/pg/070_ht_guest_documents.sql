-- Migration: 070_ht_guest_documents
-- Version: vNext
-- Date: 2026-07-01
-- Check-in registration — guest identity document / photo storage.
--
-- ## Background
--
-- The check-in registration flow captures identity artifacts for a guest:
-- a Thai national ID card image (chip photo), a passport page image, and/or a
-- webcam face photo. iHOTEL stores these in the legacy `Tb_Save_Image` table,
-- linked to a check-in by a provisional `tmp_no` string that the walk-in /
-- check-in recipe later resolves (`UPDATE Tb_Save_Image ... WHERE tmp_no=`).
--
-- This table is the canonical home for those artifacts. Rows carry the image
-- bytes plus the linkage columns needed to mirror into `Tb_Save_Image`:
--   - `doc_legacy_tmp_no` — the provisional `Tb_Save_Image.tmp_no` we minted,
--     handed back to the client and threaded onto the check-in POST as
--     `photoTmpNo` so the existing check-in writeback links the legacy image row.
--   - `doc_legacy_id`     — the allocated `Tb_Save_Image.id`, back-populated once
--     the (flag-gated) `MirrorGuestImage` writeback has run.
--
-- ## Cardinality / provenance
--
-- `1:N` canonical (ours) → `Tb_Save_Image`. A folio / customer may have several
-- document rows (id card + passport + face photo). doc_type drives the legacy
-- `Tb_Save_Image.ttype` mapping:
--   'thai_id_card' → บัตรประชาชน   |  'face_photo' → รูปลูกค้า   |  'passport' → หนังสือเดินทาง
--
-- Read path: `routes/guest_documents.rs` (list metadata / stream bytes).
-- Write path (legacy mirror): `writeback/recipes/save_image.rs` — SHIPPED DARK
-- behind `GUEST_DOCUMENT_STORAGE_ENABLED` (default off). Until the flag is on,
-- documents persist canonical-only.
--
-- Site scoping is connection-level (this table exists in both the `hotelnew`
-- and `hotelville` logical DBs) — same model as `ht_shifts` / `ht_notes`.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_guest_documents (
    doc_id            SERIAL PRIMARY KEY,
    doc_cust_id       INTEGER REFERENCES ht_customers(cust_id),
    doc_cin_id        INTEGER REFERENCES ht_checkins(cin_id),
    doc_type          VARCHAR(30) NOT NULL,   -- 'thai_id_card' | 'passport' | 'face_photo'
    doc_mime          VARCHAR(50) NOT NULL DEFAULT 'image/jpeg',
    doc_image         BYTEA NOT NULL,
    doc_source        VARCHAR(20),            -- 'chip' | 'scanner' | 'webcam'
    doc_legacy_tmp_no VARCHAR(50),            -- Tb_Save_Image.tmp_no linkage
    doc_legacy_id     INTEGER,                -- Tb_Save_Image.id once mirrored
    doc_created_at    TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ht_guest_documents_cin ON ht_guest_documents(doc_cin_id);
CREATE INDEX IF NOT EXISTS idx_ht_guest_documents_cust ON ht_guest_documents(doc_cust_id);

COMMENT ON TABLE ht_guest_documents IS
    'Guest identity documents / photos captured at check-in registration '
    '(Thai ID card chip image, passport page, webcam face photo). Canonical '
    'home for legacy Tb_Save_Image (1:N). doc_legacy_tmp_no is the provisional '
    'Tb_Save_Image.tmp_no minted here and echoed to the check-in POST as '
    'photoTmpNo; doc_legacy_id is back-populated after the legacy mirror runs. '
    'Legacy mirror (writeback/recipes/save_image.rs) is SHIPPED DARK behind '
    'GUEST_DOCUMENT_STORAGE_ENABLED (default off). Per-site (connection-level '
    'scoping). Migration 070.';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS idx_ht_guest_documents_cust;
-- DROP INDEX IF EXISTS idx_ht_guest_documents_cin;
-- DROP TABLE IF EXISTS ht_guest_documents;
-- DELETE FROM schema_migrations WHERE version = '070';
