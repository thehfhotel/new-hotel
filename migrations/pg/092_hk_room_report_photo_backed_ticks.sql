-- Migration: 092_hk_room_report_photo_backed_ticks
-- Version: vNext
-- Date: 2026-09-02
-- Report HK v2 — the equipment checklist becomes PHOTO-BACKED TICKS (owner
-- directives 2026-09-02: "1 picture for each tick", "fast and easy for a maid
-- working against the clock"). Vocabulary `app/hk/report-vocab.ts`
-- (REPORT_ZONES / TICK_STATES / REPORT_MAX_PHOTOS_TOTAL), domain language
-- `CONTEXT.md` §Housekeeping "Photo-backed tick" / "Capture zone", ADR 0008.
--
-- ## What changes, and what it corrects
--
-- Migration 091 shipped the checklist EXCEPTION-BASED, hours ago: a report
-- carried `rr_all_items_ok` (ครบทุกรายการ) and, when that was false, one
-- `ht_hk_room_report_items` row per excepted item. The owner read it and
-- rejected the model the same day — a tick nobody photographed is an
-- attestation reception cannot check, and "the room is fine" as ONE checkbox is
-- exactly the tap a maid working against the clock makes without looking.
--
-- So `ht_hk_room_report_items` stops being the EXCEPTIONS table and becomes the
-- TICK table: **one row per checklist item per report — all 22, every time** —
-- each carrying the state the maid recorded (`ok` | `missing` | `damaged`) and
-- **the id of the photo that backs it**. A perfect room is now four camera taps
-- (one per capture ZONE), 22 rows all `ok`, and no further interaction; a หาย /
-- ชำรุด item takes its own close-up and a quantity.
--
-- One photo may back SEVERAL ticks (the เตียง shot vouches for all five bed
-- items) — that is the whole speed argument, and it is why the backing photo is
-- a plain FK from the tick rather than a photo→item join table. The server does
-- NOT enforce that a problem tick has a close-up of its own: the UI drives that
-- (it is a capture flow, not an invariant of the record), and a server rule
-- there would refuse a maid whose close-up genuinely doubles as the zone shot.
--
-- ## Capture zones are NOT a data-model entity
--
-- `rrp_zone` records which of the four shooting zones a picture was taken in,
-- and it is INFORMATIONAL only: nothing joins on it, nothing is refused because
-- of it, and the "at least one photo per zone" rule is the CLIENT's (the server
-- enforces only the 4..=24 total). CONTEXT.md says so in as many words — zones
-- are a capture ORDER; the record is the ticks and their photos. A NOT NULL
-- zone column would have made a re-shot picture unfileable.
--
-- ## Photos are kept FOREVER
--
-- OWNER DECISION, 2026-09-02, recorded here because it settles migration 091's
-- open question rather than deferring it again: there is **no purge job, no
-- TTL, no retention window and no sweeper** for `ht_hk_room_report_photos`, and
-- none may be added without a new owner decision. 091's "accepted debt"
-- (unattached rows linger) is therefore no longer debt at all — it is the
-- policy, now stated for ATTACHED rows too. The only bounded thing is a maid's
-- own pre-submit housekeeping: `DELETE /api/hk/report-photos/{id}` removes an
-- UNATTACHED photo she just took, and cannot touch one a report names.
--
-- ## `rr_all_items_ok` is now DERIVED
--
-- It is true iff every tick is `ok`, which is the same thing as "this report
-- has no problem rows". The COLUMN stays — v1 readers (the shipped `/hk`
-- bundle, the day-overview summary) still read it and the submit still writes
-- it — but it is no longer the ATTESTATION, it is a cache of a fact that now
-- lives in the tick rows. Nothing may write it independently of them.
--
-- ## Reading v1 rows (both directions of tolerance)
--
-- Rows filed by v1 are EXCEPTIONS: `rri_photo_id IS NULL`, `rri_problem` set,
-- `rri_qty` set. `rri_photo_id IS NOT NULL` is therefore the discriminator —
-- **a v2 tick is exactly a photo-backed row** — and a v1 report reads as
-- `ticks: []` with its `items` array unchanged. The backfill below sets
-- `rri_state` from `rri_problem` on every existing row so the state column is
-- TRUTHFUL for history too (a v1 exception must never read as an `ok` tick,
-- which is what the column DEFAULT alone would have made it).
--
-- New writes set BOTH `rri_state` and `rri_problem` for a problem tick, and
-- leave `rri_problem` NULL for an `ok` one. Writing both is deliberate: it
-- keeps the v1 `items` projection a pure `WHERE rri_problem IS NOT NULL` read
-- for old bundles, keeps the DB CHECK on `rri_problem` doing real work, and
-- means the two columns can never disagree because one statement writes them
-- from one value (`NULLIF(state, 'ok')`).
--
-- ## Still PG-CANONICAL ONLY
--
-- Unchanged from 091 and not a fill-in-the-blank: iHOTEL has no Report HK
-- counterpart at all, so there is NO sync mapper, NO writeback recipe, NO
-- `WritebackIntent` and no dark flag waiting to enable one — coexistence
-- invariant #6 is upheld by there being nothing legacy-coupled to gate. The
-- submit still publishes only the EXISTING `item_missing` / `item_damaged` room
-- signals (089) it raises in its own transaction, one per problem KIND.
--
-- Site scoping is connection-level — `scripts/migrate.sh --site hfville` runs
-- this same file against `hotelville`, and `init-db/01-create-hotelville-database.sh`
-- replays `init-hotelnew.sql` there on a fresh cluster, so no per-site file is
-- needed.
--
-- ⚠️ NOT INERT ON ITS OWN, unlike 090: the UNIQUE (report, item) index below is
-- a real constraint change, and v1 allowed the SAME item twice in one report
-- (the v1 duplicate key was the (item, PROBLEM) pair — "two glasses missing AND
-- one glass damaged"). The preflight below refuses with an actionable message
-- rather than letting the index build fail on internals. Both sites hold zero
-- such rows (Report HK shipped hours before this migration and is
-- verification-only), so the preflight is a guard, not a task.

-- UP MIGRATION

-- ============================================================================
-- ht_hk_room_report_items — from EXCEPTIONS to TICKS
-- ============================================================================

-- Preflight: the UNIQUE index below cannot be built over a v1 report that
-- carried one item twice (missing AND damaged). Fail with the remedy rather
-- than with "could not create unique index".
DO $$
DECLARE
    dupes bigint;
BEGIN
    SELECT COUNT(*) INTO dupes
      FROM (
        SELECT rri_report_id, rri_item
          FROM ht_hk_room_report_items
         GROUP BY rri_report_id, rri_item
        HAVING COUNT(*) > 1
      ) d;
    IF dupes > 0 THEN
        RAISE EXCEPTION
            'migration 092: % (report, item) pair(s) appear twice in ht_hk_room_report_items; '
            'v1 allowed one item to carry BOTH a missing and a damaged exception, and the tick '
            'model allows one row per item. Reconcile those reports by hand (keep the row whose '
            'problem the desk acted on, or re-file the report) before re-running this migration.',
            dupes;
    END IF;
END
$$;

-- What the maid recorded for this item — the TICK. `ok` is the pre-ticked
-- default she never has to touch; the other two are problems and carry a
-- quantity.
--
-- CHECKED, like `rri_problem` and for the same reason: the triple is
-- STRUCTURAL, not product vocabulary. `ok` is the absence of a problem and the
-- other two are exactly the pair the `item_missing` / `item_damaged` room
-- signals (089, ADR 0008) are built on — the submit transaction maps this
-- column onto them. A fourth state would mean the guest-accountability signal
-- vocabulary no longer covers the checklist, i.e. a redesign, not a list edit.
-- (Contrast `rri_item`, which stays unCHECKed: the 22-row checklist IS product
-- vocabulary and lives in `domain::hk_report::REPORT_ITEMS`.)
--
-- DEFAULT 'ok' exists to make the column NOT NULL over the rows already
-- present; the backfill immediately below then tells the truth about them. New
-- writes always send the state explicitly.
ALTER TABLE ht_hk_room_report_items
    ADD COLUMN IF NOT EXISTS rri_state TEXT NOT NULL DEFAULT 'ok';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'ht_hk_room_report_items_rri_state_check'
    ) THEN
        ALTER TABLE ht_hk_room_report_items
            ADD CONSTRAINT ht_hk_room_report_items_rri_state_check
            CHECK (rri_state IN ('ok', 'missing', 'damaged'));
    END IF;
END
$$;

-- Tell the truth about the rows v1 filed: every one of them is an EXCEPTION, so
-- its state is its problem. Without this a v1 "TV remote missing" row would read
-- as an `ok` tick — the DEFAULT above is a NOT NULL enabler, not a claim.
-- Idempotent: after it runs, no row has a problem that disagrees with its state.
UPDATE ht_hk_room_report_items
   SET rri_state = rri_problem
 WHERE rri_problem IS NOT NULL
   AND rri_state IS DISTINCT FROM rri_problem;

-- THE PHOTO THAT BACKS THIS TICK. **NULL only on v1 (exception) rows** — which
-- makes `rri_photo_id IS NOT NULL` the discriminator every read uses to tell a
-- v2 tick from a v1 exception, and is why the column is nullable at all. Every
-- new tick names a photo; the app enforces it (the DB cannot, because it must
-- keep accepting the history v1 wrote).
--
-- ON DELETE CASCADE is a CASCADE-ORDERING BACKSTOP, not a feature. The app
-- never deletes a photo a tick names — `DELETE /api/hk/report-photos/{id}`
-- refuses anything already attached — so the only way this fires is the
-- pre-existing room→report cascade (`rr_room_id … ON DELETE CASCADE`), which
-- would otherwise delete this table's rows and the photos table's rows in an
-- unspecified order and could hit this FK from the wrong side. With CASCADE
-- every order works, and the ticks it removes were being removed anyway.
--
-- Plain FK rather than a photo↔tick join table on purpose: one photo backs
-- SEVERAL ticks (the เตียง shot covers all five bed items) but a tick has
-- exactly ONE backing photo, so this is a many-ticks-to-one-photo edge and a
-- join table would only add a way to record two.
ALTER TABLE ht_hk_room_report_items
    ADD COLUMN IF NOT EXISTS rri_photo_id BIGINT
        REFERENCES ht_hk_room_report_photos(rrp_id) ON DELETE CASCADE;

-- `ok` ticks carry NO quantity (there is nothing to count) and problem ticks
-- carry 1..=99. Dropping NOT NULL is what makes the 22-rows-every-time model
-- representable at all; the surviving `CHECK (rri_qty >= 1 AND rri_qty <= 99)`
-- keeps doing its job, because a CHECK is satisfied by NULL.
ALTER TABLE ht_hk_room_report_items
    ALTER COLUMN rri_qty DROP NOT NULL;

-- Same move, same reason: an `ok` tick has no problem. The 091 CHECK
-- (`rri_problem IN ('missing','damaged')`) still constrains every non-NULL
-- value, so "a problem code we do not know" stays impossible.
ALTER TABLE ht_hk_room_report_items
    ALTER COLUMN rri_problem DROP NOT NULL;

-- ONE ROW PER ITEM PER REPORT — the invariant the tick model is built on, and
-- the thing the v1 duplicate rule (unique per item+PROBLEM pair) deliberately
-- allowed. It is what lets a read trust "22 rows = the whole checklist" and
-- what stops a retried submit from doubling a report's ticks.
--
-- CREATE UNIQUE INDEX (not ADD CONSTRAINT) so `IF NOT EXISTS` makes the
-- migration re-runnable, the convention every other index here follows.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_hk_room_report_items_report_item
    ON ht_hk_room_report_items (rri_report_id, rri_item);

-- The problem-count read behind `problemCount` on the day-overview summary and
-- the derived `allItemsOk`. PARTIAL on the problem rows only — 21 of 22 ticks
-- in a healthy property are `ok`, so indexing all of them would be indexing the
-- answer "nothing is wrong" over and over.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_items_problems
    ON ht_hk_room_report_items (rri_report_id)
    WHERE rri_state <> 'ok';

COMMENT ON TABLE ht_hk_room_report_items IS
    'Report HK equipment-checklist TICKS, migration 092 (was EXCEPTIONS-only in 091). '
    'ONE ROW PER ITEM PER REPORT — all 22, every time — UNIQUE (rri_report_id, rri_item). '
    'rri_state (ok | missing | damaged) is what the maid recorded and rri_photo_id is THE '
    'PHOTO THAT BACKS IT; one photo may back several ticks (the bed shot covers the bed '
    'linen), which is the whole speed argument. rri_qty is NULL for an ok tick and 1..=99 '
    'for a problem; rri_problem is NULL for an ok tick and otherwise equals rri_state, '
    'written from the same value by one statement so the two cannot disagree — which keeps '
    'the v1 "exceptions" projection a pure WHERE rri_problem IS NOT NULL read for bundles '
    'that predate v2. V1 TOLERANCE: rows filed by migration 091 have rri_photo_id IS NULL, '
    'so rri_photo_id IS NOT NULL is the discriminator and a v1 report reads as ticks = []. '
    'rri_item is TEXT with NO CHECK on purpose (the checklist is product vocabulary living '
    'in domain::hk_report::REPORT_ITEMS, mirroring app/hk/report-vocab.ts; items that ARE '
    'linen reuse the exact VALID_LINEN_KINDS codes); rri_state and rri_problem ARE checked '
    'because ok/missing/damaged is structural — the submit maps the problems onto the '
    'item_missing / item_damaged room signals (089), one signal per problem KIND. '
    'Per-site (connection-level scoping).';

-- ============================================================================
-- ht_hk_room_report_photos — capture zone + size, and a stated retention policy
-- ============================================================================

-- WHICH capture zone this picture was taken in (`bed` | `desk` | `bathroom` |
-- `general`, mirroring REPORT_ZONES). **Informational only**, and NULLABLE
-- because of it: nothing joins on it, no read is filtered by it, and no
-- submission is refused for a zone that is absent or unrepresented. The "one
-- photo per zone" rule is the client's shooting discipline (CONTEXT.md: zones
-- are a capture ORDER, not a data-model entity); the server enforces only the
-- 4..=24 total. NULL is what a v1 photo and a free-hand re-shot close-up both
-- look like, and both are legitimate.
--
-- TEXT with NO CHECK, the 088 rationale: the zone list is product vocabulary
-- (`domain::hk_report::REPORT_ZONES`, mirroring app/hk/report-vocab.ts), so
-- re-cutting the shooting order later ships with the frontend rather than as an
-- ALTER on a live table at two sites.
ALTER TABLE ht_hk_room_report_photos
    ADD COLUMN IF NOT EXISTS rrp_zone TEXT;

-- Stored size in BYTES. A denormalisation of `octet_length(rrp_photo)`, on
-- purpose: the upload response and the report's photo summary both want it, and
-- reading it off the column costs nothing while `octet_length` on a BYTEA has
-- to detoast megabytes of image per row to answer. INTEGER is ample — the
-- handler refuses anything over 5 MB.
ALTER TABLE ht_hk_room_report_photos
    ADD COLUMN IF NOT EXISTS rrp_bytes INTEGER;

-- Backfill the photos v1 stored, so the summary is honest about them too.
-- One-shot and idempotent (the predicate empties itself).
UPDATE ht_hk_room_report_photos
   SET rrp_bytes = octet_length(rrp_photo)
 WHERE rrp_bytes IS NULL;

COMMENT ON TABLE ht_hk_room_report_photos IS
    'Report HK photo evidence — BYTEA + mime, the 077 ht_hk_broken_reports.hkbr_photo '
    'storage pattern (migration 091; rrp_zone + rrp_bytes added by 092). Every checklist '
    'tick names one of these rows (ht_hk_room_report_items.rri_photo_id) and one photo may '
    'back several ticks. rrp_side (maid | reception) is CHECKED and DERIVED from the '
    'uploader''s role, never client-sent. rrp_report_id is NULLABLE by design — a phone '
    'uploads before the form is submitted, so a photo is minted unattached and bound by the '
    'submit/verify transaction (WHERE rrp_report_id IS NULL AND rrp_badge = the caller AND '
    'rrp_side = …), which makes "your own, not already attached, not the other side''s" one '
    'atomic verdict. rrp_zone is the capture zone and is INFORMATIONAL ONLY (nothing joins '
    'or filters on it; the one-per-zone rule is the client''s), rrp_bytes is the stored size '
    'for the upload response and the report summary. RETENTION: photos are KEPT FOREVER — '
    'owner decision 2026-09-02. There is no purge job, no TTL and no sweeper, for attached '
    'or unattached rows, and none may be added without a new owner decision; the only '
    'deletion path is DELETE /api/hk/report-photos/{id}, which is uploader-only and refuses '
    'anything already attached to a report. PG-CANONICAL ONLY. Per-site (connection-level '
    'scoping).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented — destructive)
-- =============================================================================
-- -- Dropping these columns DISCARDS which photo backed which tick, i.e. the
-- -- entire evidence link the v2 report is built on, and every `ok` tick with
-- -- it (the rows become unreadable as exceptions). It also DISCARDS the stored
-- -- capture zone and size of every photo. Do not run this to "undo a deploy" —
-- -- a v2 report cannot be re-derived from what would be left.
-- DROP INDEX IF EXISTS ix_ht_hk_room_report_items_problems;
-- DROP INDEX IF EXISTS ux_ht_hk_room_report_items_report_item;
-- ALTER TABLE ht_hk_room_report_photos DROP COLUMN IF EXISTS rrp_bytes;
-- ALTER TABLE ht_hk_room_report_photos DROP COLUMN IF EXISTS rrp_zone;
-- ALTER TABLE ht_hk_room_report_items DROP COLUMN IF EXISTS rri_photo_id;
-- ALTER TABLE ht_hk_room_report_items DROP CONSTRAINT IF EXISTS ht_hk_room_report_items_rri_state_check;
-- ALTER TABLE ht_hk_room_report_items DROP COLUMN IF EXISTS rri_state;
-- -- NOTE: the two DROP NOT NULLs are NOT reversible while ok-ticks exist —
-- -- restoring them requires deleting every ok tick first, which is the
-- -- evidence loss described above.
-- DELETE FROM schema_migrations WHERE version = '092';
