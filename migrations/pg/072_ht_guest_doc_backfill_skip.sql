-- Migration: 072_ht_guest_doc_backfill_skip
-- Version: (release-please)
-- Date: 2026-07-01
--
-- Convergence backstop for the guest-image sync-in poll
-- (bin/sync.rs::sync_guest_documents). That poll pulls each check-in's legacy
-- Tb_Save_Image card image into ht_guest_documents, processing the newest
-- check-ins still MISSING an image, 20 per tick. Without this table, a check-in
-- that has NO legacy image (e.g. a walk-in with no card scan) stays "missing"
-- forever; once ≥20 of the newest missing check-ins are imageless the batch pins
-- there and never advances to older records. This table records check-ins that
-- yielded no legacy image so the poll skips them next tick and keeps advancing.
-- (To force a re-check later — e.g. after a card was scanned in iHOTEL post
-- check-in — DELETE the row.)

-- UP MIGRATION
CREATE TABLE IF NOT EXISTS ht_guest_doc_backfill_skip (
    cin_id       INTEGER PRIMARY KEY REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    attempted_at TIMESTAMP DEFAULT NOW()
);

-- DOWN MIGRATION
-- DROP TABLE IF EXISTS ht_guest_doc_backfill_skip;
