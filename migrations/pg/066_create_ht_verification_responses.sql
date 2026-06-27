-- Migration: 066_create_ht_verification_responses
-- Version: vNext
-- Date: 2026-06-28
-- Task #58 (in-app reception verification form — คู่มือตรวจสอบระบบใหม่ สำหรับฝ่ายต้อนรับ).
--
-- ## Background
--
-- Before cutover, reception verifies the new app against iHOTEL using the printed
-- checklist `docs/coexistence/reception-verification-TH.html`:
--   §1 screen-compare (room status / round-bill totals / today's roster /
--      availability calendar / tax invoice),
--   §2 policy questions (late-checkout charging / deposit handling / VAT),
--   §3 last-5-bills total check (iHOTEL total vs new-app computed total),
--   §4 coordinated live tests (done later WITH IT — kept optional in the form),
--   §5 overall readiness verdict.
-- That checklist is the PRINT version; this table is the ONLINE equivalent so the
-- answers land in PostgreSQL and are QUERYABLE by IT instead of living on paper /
-- phone photos.
--
-- ## Schema notes
--
-- - PG-CANONICAL ONLY. This is app-internal operational data — iHOTEL has NO
--   counterpart (legacy = none, like `ht_users`). There is NO sync mapper and NO
--   legacy writeback (unlike `ht_notes`).
-- - `vr_answers JSONB` holds EVERY choice/text answer keyed by question id, e.g.
--   {"q1_1":"match","q1_2":"mismatch","q1_2_reasons":["cash"],"q2_1":"a",
--    "q3_bills":[{"no":"...","ihotel":"...","newsys":"...","match":"match"}],
--    "q5":"a"}. JSONB keeps the form flexible: the checklist can gain/lose
--   questions WITHOUT a migration, and IT can query a single answer with
--   `vr_answers->>'q2_1'`.
-- - `vr_site` is the branch the form was submitted for ('hfhotel'/'hfville').
-- - `vr_overall` denormalizes the §5 readiness verdict out of the JSONB so a
--   "ready vs not-ready" filter needs no JSON path lookup.
--
-- Site scoping is connection-level (this table exists in BOTH the `hotelnew` and
-- `hotelville` logical DBs; each site's pool holds its own responses) — same
-- model as `ht_shifts` / `ht_notes`. No site column drives routing; `vr_site` is
-- captured for display/audit only.

-- UP MIGRATION

CREATE TABLE IF NOT EXISTS ht_verification_responses (
    vr_id           BIGSERIAL    PRIMARY KEY,
    -- When the receptionist submitted the checklist.
    vr_submitted_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    -- Branch the form was submitted for ('hfhotel' / 'hfville' / 'all').
    vr_site         TEXT,
    -- ผู้ตรวจ — inspector name (session username when auth is on, else typed).
    vr_inspector    TEXT,
    -- All choice/text answers keyed by question id (q1_1, q1_2_reasons, …).
    vr_answers      JSONB        NOT NULL,
    -- §5 overall readiness verdict ('a' | 'b' | 'c'), denormalized from answers.
    vr_overall      TEXT,
    created_at      TIMESTAMPTZ  DEFAULT NOW()
);

-- Recent-first listing for the IT review screen (GET /api/verification).
CREATE INDEX IF NOT EXISTS ix_ht_verification_responses_submitted_at
    ON ht_verification_responses (vr_submitted_at DESC);

COMMENT ON TABLE ht_verification_responses IS
    'In-app reception verification checklist responses (task #58, migration 066). '
    'Online equivalent of docs/coexistence/reception-verification-TH.html — every '
    'answer lands in vr_answers JSONB keyed by question id. PG-CANONICAL ONLY (no '
    'legacy counterpart, no sync, no writeback). Per-site (connection-level '
    'scoping); vr_site captured for display/audit only.';

-- DOWN MIGRATION (commented — destructive)
-- DROP INDEX IF EXISTS ix_ht_verification_responses_submitted_at;
-- DROP TABLE IF EXISTS ht_verification_responses;
-- DELETE FROM schema_migrations WHERE version = '066';
