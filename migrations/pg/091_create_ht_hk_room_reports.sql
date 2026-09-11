-- Migration: 091_create_ht_hk_room_reports
-- Version: vNext
-- Date: 2026-09-02
-- Report HK — the maid's per-room daily attestation and reception's
-- countersignature (owner's `Report HK.xlsx`, digitized; vocabulary pinned in
-- `app/hk/report-vocab.ts` and `CONTEXT.md` §Housekeeping "Room report (Report
-- HK)" / "Report verification").
--
-- ## Background
--
-- The domain object is the ROOM REPORT: ONE maid's attestation about ONE room
-- on ONE day, carrying three things and nothing else —
--
--   1. the room's STATUS CODE (VC/CO/OO/SO). Prefilled client-side from known
--      room facts, but the maid may override it, and what lands here is what
--      SHE reported. The prefill is a convenience, never a claim.
--   2. the in-room equipment checklist, EXCEPTION-BASED: `rr_all_items_ok` is
--      the ครบทุกรายการ attestation, and when it is false the named exceptions
--      live in `ht_hk_room_report_items` (one row per excepted item+problem).
--      A clean room therefore stores ZERO item rows — the common case costs
--      one header row.
--   3. PHOTO EVIDENCE, 1..=4 pictures from the maid and, on verification,
--      1..=4 more from reception (`REPORT_MIN_PHOTOS` / `REPORT_MAX_PHOTOS`).
--      Two-sided evidence IS the feature: a verify is a walk-up, not a desk
--      stamp.
--
-- Lifecycle: `submitted` → `verified` | `returned`. A RETURNED report is never
-- edited — it is superseded by a NEW submission carrying `rr_parent_id` → the
-- returned one, so history is APPEND-ONLY and the whole chain stays readable.
-- The return reason is CANNED (`not_clean` | `items_mismatch` |
-- `photos_unclear`); there is deliberately NO free-text column anywhere in
-- these three tables, the same discipline ADR 0008 records for room signals.
--
-- ## What it does NOT do
--
-- PG-CANONICAL ONLY, exactly as narrowly as `ht_hk_linen_reports` (088) and
-- `ht_hk_room_signals` (089). iHOTEL has no counterpart to a Report HK sheet at
-- all, so there is NO sync mapper, NO writeback recipe, NO `WritebackIntent`
-- and no dark flag waiting to enable one — coexistence invariant #6 is upheld
-- by there being nothing legacy-coupled to gate. Unlike `ht_hk_cleaning_events`
-- (whose `done` phase grew a `MarkRoomClean` writeback on 2026-08-11) these
-- tables have no path to legacy by design.
--
-- It publishes no domain event of its own either (linen's posture, not
-- signals'): both surfaces re-read the day overview. What it DOES do inside the
-- submit transaction is raise the EXISTING `item_missing` / `item_damaged` room
-- signals (089) when the checklist carries exceptions — one signal per excepted
-- PROBLEM kind, maid→desk — so reception hears about chargeable items
-- immediately rather than when someone next opens the report. Those signals
-- publish their own `RoomSignalRaised` events through the existing fan-out;
-- that is signal plumbing, not a legacy write.
--
-- Site scoping is connection-level (both `hotelnew` and `hotelville` logical
-- DBs get these tables; each site's pool holds its own reports) — same model as
-- `ht_hk_cleaning_events` / `ht_hk_linen_reports` / `ht_hk_room_signals`.
-- `scripts/migrate.sh --site hfville` runs this same file against
-- `hotelville`, and `init-db/01-create-hotelville-database.sh` replays
-- `init-hotelnew.sql` there on a fresh cluster, so no per-site file is needed.

-- UP MIGRATION

-- ============================================================================
-- ht_hk_room_reports — the header (one maid, one room, one day)
-- ============================================================================

CREATE TABLE IF NOT EXISTS ht_hk_room_reports (
    rr_id             BIGSERIAL   PRIMARY KEY,
    -- The ONE room this report attests about (site-local canonical room). FK +
    -- CASCADE mirrors `ht_hk_room_signals.sig_room_id` — the route 404s an
    -- unknown or inactive room before the insert, so this is a backstop, not
    -- the gate.
    rr_room_id        INTEGER     NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    -- The Bangkok CIVIL DAY the report is FOR, stored as a bare DATE.
    --
    -- A stored date, not a derivation from `rr_submitted_at`: the paper sheet
    -- is filled per day, a maid finishing a floor at 00:10 is still working
    -- yesterday's sheet, and the day overview must be able to ask for a past
    -- date without re-deriving a timezone every time. The app computes it as
    -- `(NOW() AT TIME ZONE 'Asia/Bangkok')::date` when the client omits one —
    -- `CURRENT_DATE` is BANNED for `routes::hk::TODAY_BKK`'s reason (it is the
    -- SERVER's date, which names YESTERDAY in Bangkok between 17:00 and 24:00
    -- UTC).
    rr_date           DATE        NOT NULL,
    -- Lifecycle position. CHECKED, because statuses ARE structural — exactly
    -- the argument `ht_hk_room_signals.sig_status` records: every transition
    -- rule and the overview's "is this room's day settled" reading are written
    -- over this closed set, so a fourth value would be a redesign rather than a
    -- vocabulary addition.
    rr_status         TEXT        NOT NULL DEFAULT 'submitted'
                                  CHECK (rr_status IN ('submitted', 'verified', 'returned')),
    -- The room-status code the maid REPORTED (vc | co | oo | so).
    --
    -- DELIBERATELY plain TEXT with NO CHECK enumerating the codes — the same
    -- decision, for the same reason, as `ht_hk_linen_reports.hklr_kind` (088)
    -- and `ht_hk_room_signals.sig_type` (089). The allowlist lives ONLY in the
    -- app (`domain::hk_report::ROOM_STATUS_CODES`, mirroring
    -- `app/hk/report-vocab.ts`), which is what the 400 for an unknown code is
    -- served from, so adding a fifth legend row later is a one-line constant
    -- edit shipping with the frontend — no ALTER on a live table at two sites
    -- and no window where the deployed binary and the deployed CHECK disagree
    -- about the valid set (exactly the coupling migration 087 had to unpick).
    -- The cost is that the DB would accept a code the app refuses; the app is
    -- the only writer, so that cost is not paid.
    rr_room_status    TEXT        NOT NULL,
    -- The ครบทุกรายการ attestation. `true` ⇒ this report has NO item rows;
    -- `false` ⇒ it has at least one. The app enforces the biconditional
    -- ("items empty iff allItemsOk") in one place before the insert; it is not
    -- expressible as a row CHECK, because the truth of it lives in a different
    -- table.
    rr_all_items_ok   BOOLEAN     NOT NULL,
    -- The CANNED return reason (not_clean | items_mismatch | photos_unclear).
    -- NULL for every report that is not `returned`. No CHECK, for
    -- `rr_room_status`'s reason at one remove: the rejection vocabulary belongs
    -- to the product decision, which `domain::hk_report::RETURN_REASONS` owns.
    -- **There is no free-text sibling and there must never be one** — canned
    -- rejection is the decision (CONTEXT.md §Housekeeping "Report verification":
    -- _Avoid_ free-text rejection notes).
    rr_return_reason  TEXT        NULL,
    -- Set on a report that SUPERSEDES a returned one: points at the report it
    -- fixes. Self-referencing FK, NULL for a first submission. Deliberately NO
    -- cascade — a report is never deleted (the chain IS the audit record), so a
    -- cascade rule would only describe a situation that cannot arise. Same
    -- shape and same reasoning as `ht_hk_room_signals.sig_parent_id`.
    rr_parent_id      BIGINT      NULL REFERENCES ht_hk_room_reports(rr_id),
    -- HF ID badge of the MAID who filed it (verified identity, never
    -- client-typed) plus the display-name snapshot — same convention as
    -- `hkev_badge`/`hkev_name`, `hklr_badge`/`hklr_name` and
    -- `sig_created_badge`/`sig_created_name`, and deliberately NO FK to
    -- `ht_users`: maids are CF Access + HF ID identities, not PMS accounts.
    rr_submitted_badge TEXT       NOT NULL,
    rr_submitted_name  TEXT,
    rr_submitted_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Who countersigned, and when. NULL until the report leaves `submitted`.
    -- The BADGE is stamped for audit and the NAME is what the sheet shows —
    -- "stamped by name" is the owner's requirement, and reception identities
    -- do carry a display name.
    --
    -- ONE pair of columns for BOTH terminal transitions, deliberately: a
    -- verify and a return are the same act by the same person at the same
    -- moment (reception judged this report), and `rr_status` already says
    -- which way it went. A second `rr_returned_*` triple would be three more
    -- columns that can disagree with `rr_status` about who acted.
    rr_verified_badge TEXT,
    rr_verified_name  TEXT,
    rr_verified_at    TIMESTAMPTZ
);

-- THE overview query's index: "every active room of this branch with its
-- LATEST report for this date". The day overview is one statement with a
-- LATERAL per room, and this is exactly the (room, day) → newest-first lookup
-- it wants; `rr_id DESC` breaks the tie inside a day deterministically (a
-- returned report and its replacement share `rr_date`).
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_reports_room_date
    ON ht_hk_room_reports (rr_room_id, rr_date, rr_id DESC);

-- The submit guard and reception's work queue: reports still awaiting a
-- verdict. PARTIAL on exactly `rr_status = 'submitted'` — same reasoning as
-- `ix_ht_hk_room_signals_live` (089) and `ix_ht_hk_linen_reports_open` (090):
-- verified and returned rows are history that only the audit reads, and they
-- are the ones that grow without bound, so keeping them out keeps this index
-- the size of the open queue forever.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_reports_open
    ON ht_hk_room_reports (rr_room_id, rr_date)
    WHERE rr_status = 'submitted';

COMMENT ON TABLE ht_hk_room_reports IS
    'Report HK header — one maid''s per-room daily attestation, migration 091. '
    'Room status code as SHE reported it, the exception-based ครบทุกรายการ flag, and '
    '1..=4 maid photos; reception countersigns with 1..=4 of its own or returns it with '
    'a CANNED reason. Lifecycle submitted -> verified | returned; a returned report is '
    'superseded by a NEW submission carrying rr_parent_id, never edited — history is '
    'append-only. NO free-text column anywhere, by decision. rr_date is the Bangkok '
    'civil day the report is FOR (CURRENT_DATE is banned — it is the server''s date). '
    'rr_room_status and rr_return_reason are TEXT with NO CHECK on purpose (app-owned '
    'vocabulary in domain::hk_report, mirroring app/hk/report-vocab.ts — the 088 '
    'rationale); rr_status IS checked because it is structural. PG-CANONICAL ONLY: no '
    'legacy counterpart, no sync mapper, no writeback, no domain event of its own — but '
    'a submission with item exceptions raises the existing item_missing / item_damaged '
    'room signals (089) in the SAME transaction. Per-site (connection-level scoping).';

-- ============================================================================
-- ht_hk_room_report_items — the EXCEPTIONS, and only the exceptions
-- ============================================================================

CREATE TABLE IF NOT EXISTS ht_hk_room_report_items (
    rri_id        BIGSERIAL PRIMARY KEY,
    -- The report this exception belongs to. CASCADE because an item row has no
    -- meaning without its header — unlike the report itself, which is never
    -- deleted, this is a component of one aggregate.
    rri_report_id BIGINT    NOT NULL REFERENCES ht_hk_room_reports(rr_id) ON DELETE CASCADE,
    -- Which checklist item (a `REPORT_ITEMS` code — `water_glass`,
    -- `tv_remote`, `bath_towel`, …). DELIBERATELY plain TEXT with NO CHECK:
    -- the 22-item list is the owner's paper form and it WILL move, so the
    -- allowlist lives ONLY in `domain::hk_report::REPORT_ITEMS` (mirroring
    -- `app/hk/report-vocab.ts`) — the 088 rationale, and here it is the
    -- strongest case for it in the codebase, because this vocabulary is the
    -- one most likely to gain a row.
    --
    -- Items that ARE linen reuse the exact `VALID_LINEN_KINDS` codes on
    -- purpose, so an item exception and a ขาดผ้า report name the same thing.
    rri_item      TEXT      NOT NULL,
    -- What is wrong with it. CHECKED, unlike `rri_item`: the pair
    -- missing/damaged is STRUCTURAL — it is the same pair the room-signal
    -- vocabulary is built on (`item_missing` / `item_damaged`, ADR 0008), and
    -- the submit transaction maps this column ONTO those two signal types. A
    -- third problem would not be a new product option, it would mean the
    -- guest-accountability signal vocabulary no longer covers the checklist.
    rri_problem   TEXT      NOT NULL CHECK (rri_problem IN ('missing', 'damaged')),
    -- How many pieces. Bounded here as well as in the app, exactly as
    -- `hklr_qty` is: 1..=99 is a maid counting what is wrong in one room, and a
    -- fat-fingered 5000 is worth refusing at both layers. This CHECK is a real
    -- invariant of the data (unlike the item set, which is a product decision
    -- that moves). The ceiling is 99 rather than linen's 20 because a room can
    -- legitimately be short many small items (ไม้แขวนเสื้อ, แก้วน้ำ) where a
    -- linen line is per-bed.
    rri_qty       INTEGER   NOT NULL CHECK (rri_qty >= 1 AND rri_qty <= 99)
);

-- The detail read: one report's exceptions, in insert order (which is the
-- order the maid ticked them, which is `REPORT_ITEMS` order client-side).
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_items_report
    ON ht_hk_room_report_items (rri_report_id, rri_id);

COMMENT ON TABLE ht_hk_room_report_items IS
    'Report HK equipment-checklist EXCEPTIONS, migration 091. EXCEPTIONS ONLY: a report '
    'with rr_all_items_ok = true has ZERO rows here, so the common case (the room is '
    'fine) costs one header row and nothing else. One row per (report, item, problem) '
    'with a 1..=99 quantity. rri_item is TEXT with NO CHECK on purpose — the checklist '
    'lives in domain::hk_report::REPORT_ITEMS (mirroring app/hk/report-vocab.ts), and '
    'items that are linen reuse the exact VALID_LINEN_KINDS codes. rri_problem IS '
    'checked: missing/damaged is the structural pair the item_missing / item_damaged '
    'room signals are built on, and the submit transaction maps this column onto them. '
    'Per-site (connection-level scoping).';

-- ============================================================================
-- ht_hk_room_report_photos — the two-sided evidence
-- ============================================================================

CREATE TABLE IF NOT EXISTS ht_hk_room_report_photos (
    rrp_id         BIGSERIAL   PRIMARY KEY,
    -- The report these bytes belong to — **NULLABLE, and that is the design**.
    --
    -- A phone uploads pictures ONE AT A TIME while the maid is still filling
    -- the form, so a photo exists BEFORE the report it will belong to. The
    -- upload endpoint mints a row with `rrp_report_id IS NULL` and returns its
    -- id; the submit/verify call names those ids and the SAME transaction
    -- attaches them (`UPDATE … SET rrp_report_id = … WHERE rrp_id = ANY(…) AND
    -- rrp_report_id IS NULL AND rrp_badge = <the caller>`), which is also what
    -- makes "not already attached elsewhere" and "your own photo" one atomic
    -- check rather than a read-then-write race.
    --
    -- ⚠️ ACCEPTED DEBT (v1): an unattached row LINGERS FOREVER — a maid who
    -- uploads three photos and never submits leaves three orphans, and there is
    -- deliberately NO garbage collector, no TTL and no sweep job. The rows are
    -- small in count (one maid, one room, one abandoned form), they are
    -- harmless (nothing reads an unattached photo but its own uploader), and a
    -- deleter is exactly the thing that could destroy evidence if it ever got
    -- its predicate wrong. If the orphan count ever becomes a real problem the
    -- fix is a REVIEWED sweep of `rrp_report_id IS NULL AND rrp_created_at <
    -- NOW() - INTERVAL '…'`, written deliberately — not bolted on here.
    --
    -- CASCADE for the same reason as `rri_report_id`: attached bytes are a
    -- component of the report aggregate.
    rrp_report_id  BIGINT      NULL REFERENCES ht_hk_room_reports(rr_id) ON DELETE CASCADE,
    -- WHICH SIDE took the picture: 'maid' (the submission's evidence) or
    -- 'reception' (the verification's). CHECKED, like `sig_direction` and for
    -- the same reason: the two-sided-evidence rule is written over exactly
    -- these two values — a submit may attach only `maid` photos and a verify
    -- only `reception` ones — so a third value would mean the role model no
    -- longer holds.
    --
    -- DERIVED FROM THE UPLOADER'S ROLE at upload time (`HkIdentity::can_report`
    -- ⇒ maid), never sent by the client: a receptionist cannot post a maid
    -- photo, and a maid cannot manufacture reception evidence.
    rrp_side       TEXT        NOT NULL CHECK (rrp_side IN ('maid', 'reception')),
    -- The image bytes and their content type — the SAME storage pattern as
    -- `ht_hk_broken_reports.hkbr_photo` / `hkbr_photo_mime` (077) and
    -- `ht_guest_documents.doc_image`. NOT NULL on both: a photo row with no
    -- bytes is not a photo, and this table has no other purpose (unlike 077's
    -- report, where the picture was optional).
    --
    -- The 5 MB per-file cap and the jpeg/png/webp allowlist are enforced in the
    -- handler, not here: they are transport policy that moves with phone
    -- cameras, and a CHECK on a BYTEA length would silently 500 a maid mid-shift
    -- instead of answering her a 400 she can act on.
    rrp_photo      BYTEA       NOT NULL,
    rrp_photo_mime TEXT        NOT NULL,
    -- HF ID badge of whoever uploaded it (verified identity, never
    -- client-typed). It is not decoration: it is half the attachment
    -- predicate — you may only attach your OWN unattached photos — which is
    -- what stops one identity from binding another's evidence to a report.
    -- No display name: the report header already carries the actor's name for
    -- both sides, and a second snapshot could only disagree with it.
    rrp_badge      TEXT        NOT NULL,
    rrp_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- The detail read: one report's photos per side, in upload order.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_photos_report
    ON ht_hk_room_report_photos (rrp_report_id, rrp_side, rrp_id)
    WHERE rrp_report_id IS NOT NULL;

-- The attachment predicate's own index: "my unattached photos". PARTIAL on
-- `rrp_report_id IS NULL` for `ix_ht_hk_linen_reports_open`'s reason — attached
-- rows are the unbounded majority, and every attach-time lookup is scoped to
-- the handful that are still open.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_photos_open
    ON ht_hk_room_report_photos (rrp_badge, rrp_id)
    WHERE rrp_report_id IS NULL;

COMMENT ON TABLE ht_hk_room_report_photos IS
    'Report HK photo evidence, migration 091 — BYTEA + mime, the 077 '
    'ht_hk_broken_reports.hkbr_photo storage pattern. rrp_side (maid | reception) is '
    'CHECKED and is DERIVED from the uploader''s role, never sent by the client: a '
    'submit attaches only maid photos, a verify only reception ones, 1..=4 each. '
    'rrp_report_id is NULLABLE by design — a phone uploads before the form is '
    'submitted, so a photo is minted unattached and bound by the submit/verify '
    'transaction (WHERE rrp_report_id IS NULL AND rrp_badge = the caller), which makes '
    '"your own, not already attached" one atomic check. ACCEPTED DEBT: unattached rows '
    'linger forever — there is no GC in v1, deliberately. PG-CANONICAL ONLY. Per-site '
    '(connection-level scoping).';

-- Schema-migrations row is inserted by scripts/migrate.sh (same TX, includes
-- the file checksum). Do NOT INSERT here.

-- =============================================================================
-- DOWN MIGRATION (commented — destructive)
-- =============================================================================
-- -- Dropping these tables DISCARDS every attestation a maid filed and every
-- -- countersignature reception gave, INCLUDING the photo evidence behind them.
-- -- Do not run this to "undo a deploy".
-- DROP INDEX IF EXISTS ix_ht_hk_room_report_photos_open;
-- DROP INDEX IF EXISTS ix_ht_hk_room_report_photos_report;
-- DROP TABLE IF EXISTS ht_hk_room_report_photos;
-- DROP INDEX IF EXISTS ix_ht_hk_room_report_items_report;
-- DROP TABLE IF EXISTS ht_hk_room_report_items;
-- DROP INDEX IF EXISTS ix_ht_hk_room_reports_open;
-- DROP INDEX IF EXISTS ix_ht_hk_room_reports_room_date;
-- DROP TABLE IF EXISTS ht_hk_room_reports;
-- DELETE FROM schema_migrations WHERE version = '091';
