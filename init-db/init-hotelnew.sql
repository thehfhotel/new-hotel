-- =============================================================================
-- HotelNew Database Initialization Script (PostgreSQL)
-- =============================================================================
-- This script creates all required tables for the HotelNew database.
-- It runs automatically when the PostgreSQL container starts for the first time
-- via docker-entrypoint-initdb.d volume mount.
--
-- PostgreSQL auto-runs .sql files from /docker-entrypoint-initdb.d/ on first startup.
-- The POSTGRES_DB env var creates the database; this script just creates tables.
-- =============================================================================

-- =============================================================================
-- Core Tables
-- =============================================================================

-- ht_booking_notes - Booking annotations (stored in HotelNew to keep legacy DB read-only)
-- Notes are linked to legacy booking numbers (book_no) but stored here
CREATE TABLE IF NOT EXISTS ht_booking_notes (
    note_id SERIAL PRIMARY KEY,
    book_no VARCHAR(50) NOT NULL,
    note_text TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_booking_notes_bookno ON ht_booking_notes(book_no);

-- ht_customers - Customer master data
-- VARCHAR widths follow migration 024 — Ville's legacy data has phone numbers
-- up to 21 chars (canonical Cust_Add_tel can carry comma-separated multi-phones).
-- Match the cache-table widths from migration 009 so a fresh deploy doesn't
-- need 024 to apply on top.
CREATE TABLE IF NOT EXISTS ht_customers (
    cust_id SERIAL PRIMARY KEY,
    cust_code VARCHAR(100),
    cust_title VARCHAR(100),
    cust_firstname VARCHAR(100) NOT NULL,
    cust_lastname VARCHAR(100),
    cust_nickname VARCHAR(50),
    cust_idcard VARCHAR(100),
    cust_passport VARCHAR(50),
    cust_nationality VARCHAR(50),
    cust_phone VARCHAR(200),
    cust_email VARCHAR(100),
    cust_address VARCHAR(500),
    cust_company VARCHAR(200),
    cust_taxid VARCHAR(100),
    cust_notes TEXT,
    cust_type VARCHAR(50),
    cust_vip BOOLEAN DEFAULT false,
    cust_blacklist BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    cust_created_by VARCHAR(50),
    cust_updated_by VARCHAR(50),
    cust_active BOOLEAN DEFAULT true,
    -- Track E2 (migration 035) — legacy `HT_Customers` parity.
    -- Captures the columns the CT mapper previously dropped silently.
    -- `cust_address` (above) continues to hold a copy of `cust_add_no`
    -- so single-line readers still work.
    cust_name2 VARCHAR(250),
    cust_sex VARCHAR(50),
    cust_price_tier VARCHAR(50),
    cust_add_no VARCHAR(250),
    cust_add_moo VARCHAR(50),
    cust_add_soi VARCHAR(250),
    cust_add_road VARCHAR(250),
    cust_add_tambon VARCHAR(250),
    cust_add_ampore VARCHAR(250),
    cust_add_province VARCHAR(250),
    cust_add_code VARCHAR(50),
    cust_add_fax VARCHAR(250),
    cust_work_name VARCHAR(250),
    cust_work_no VARCHAR(250),
    cust_work_moo VARCHAR(50),
    cust_work_soi VARCHAR(250),
    cust_work_road VARCHAR(250),
    cust_work_tambon VARCHAR(250),
    cust_work_ampore VARCHAR(250),
    cust_work_province VARCHAR(250),
    cust_work_code VARCHAR(50),
    cust_work_tel VARCHAR(250),
    cust_work_fax VARCHAR(250),
    cust_work_tax VARCHAR(50),
    cust_last_change TIMESTAMP,
    cust_contry VARCHAR(50),
    -- Running debt balance. Mirrored read-only from legacy
    -- `Module1.UPDATE_MONEY` writes; our own writeback path stays
    -- deferred to Track G.
    cust_price_over DOUBLE PRECISION,
    -- Migration 069 — guest date of birth captured at check-in registration
    -- (Thai ID chip / passport MRZ). PG-canonical-only: legacy HT_Customers
    -- has no DOB column, so this is never mirrored to MSSQL.
    cust_dob DATE,
    -- Migration 086 — loyalty membership id (loyalty-app link). PG-canonical
    -- only: legacy HT_Customers has no membership column, never mirrored.
    cust_membership_id VARCHAR(64)
);
CREATE INDEX IF NOT EXISTS ix_ht_customers_name ON ht_customers(cust_firstname, cust_lastname);
CREATE INDEX IF NOT EXISTS ix_ht_customers_phone ON ht_customers(cust_phone);
CREATE INDEX IF NOT EXISTS ix_ht_customers_idcard ON ht_customers(cust_idcard);
CREATE INDEX IF NOT EXISTS ix_ht_customers_passport ON ht_customers(cust_passport);
-- Migration 086 — membership → guest lookup (checkout hook + desk member-QR).
CREATE INDEX IF NOT EXISTS ix_ht_customers_membership_id
    ON ht_customers (cust_membership_id) WHERE cust_membership_id IS NOT NULL;

-- ht_room_types - Room type definitions
CREATE TABLE IF NOT EXISTS ht_room_types (
    type_id SERIAL PRIMARY KEY,
    type_code VARCHAR(20) NOT NULL UNIQUE,
    type_name VARCHAR(100) NOT NULL,
    type_name_en VARCHAR(100),
    type_description VARCHAR(500),
    type_base_price DECIMAL(10,2) DEFAULT 0,
    type_max_guests INTEGER DEFAULT 2,
    type_bed_type VARCHAR(50),
    type_size_sqm DECIMAL(6,2),
    type_amenities TEXT,
    type_sort_order INTEGER DEFAULT 0,
    type_active BOOLEAN DEFAULT true,
    type_created_at TIMESTAMP DEFAULT NOW(),
    type_updated_at TIMESTAMP DEFAULT NOW()
);

-- ht_rooms_new - Room inventory
CREATE TABLE IF NOT EXISTS ht_rooms_new (
    room_id SERIAL PRIMARY KEY,
    room_no VARCHAR(10) NOT NULL UNIQUE,
    room_type_id INTEGER,
    room_floor INTEGER,
    room_building VARCHAR(50),
    room_view VARCHAR(50),
    room_status VARCHAR(20) DEFAULT 'available',
    room_clean BOOLEAN DEFAULT true,
    room_maintenance BOOLEAN DEFAULT false,
    room_notes VARCHAR(500),
    room_features TEXT,
    room_active BOOLEAN DEFAULT true,
    -- NOTE: room_price_{weekday,weekend,special} below does NOT match
    -- the legacy `Room_PriceA/B/C` model (legacy indexes prices by
    -- customer-type, not day-of-week). Track F (canonical rate-table
    -- model) will reconcile this — Track E2 / migration 036
    -- intentionally leaves these columns untouched.
    room_price_weekday DECIMAL(10,2),
    room_price_weekend DECIMAL(10,2),
    room_price_special DECIMAL(10,2),
    -- Writeback resolver back-populates these (migration 014).
    legacy_room_no VARCHAR(10),
    legacy_room_id_int INTEGER,
    aggregate_id UUID,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    -- Track E2 (migration 036) — legacy `HT_Rooms` parity. Captures
    -- the columns the CT mapper previously dropped silently.
    -- Defaults mirror legacy NOT NULL / DEFAULT clauses so the
    -- canonical row stays valid even before the first CT tick lands.
    room_use_count INTEGER NOT NULL DEFAULT 0,
    room_x INTEGER NOT NULL DEFAULT 0,
    room_y INTEGER NOT NULL DEFAULT 0,
    room_group VARCHAR(50),
    room_power_open VARCHAR(50),
    room_power_close VARCHAR(50),
    room_power_status VARCHAR(50) NOT NULL DEFAULT 'off',
    room_polity INTEGER NOT NULL DEFAULT 1,

    CONSTRAINT fk_ht_rooms_type FOREIGN KEY (room_type_id)
        REFERENCES ht_room_types(type_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_rooms_status ON ht_rooms_new(room_status);
CREATE INDEX IF NOT EXISTS ix_ht_rooms_type ON ht_rooms_new(room_type_id);
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_rooms_new_aggregate_id
    ON ht_rooms_new (aggregate_id) WHERE aggregate_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS ix_ht_rooms_new_legacy_room_no
    ON ht_rooms_new (legacy_room_no) WHERE legacy_room_no IS NOT NULL;

-- ht_bookings - Booking records
CREATE TABLE IF NOT EXISTS ht_bookings (
    book_id SERIAL PRIMARY KEY,
    book_no VARCHAR(20) NOT NULL UNIQUE,
    book_date TIMESTAMP DEFAULT NOW(),
    book_cust_id INTEGER NOT NULL,
    book_checkin DATE NOT NULL,
    book_checkout DATE NOT NULL,
    book_adults INTEGER DEFAULT 1,
    book_children INTEGER DEFAULT 0,
    book_nights INTEGER GENERATED ALWAYS AS (book_checkout - book_checkin) STORED,
    book_status VARCHAR(20) DEFAULT 'confirmed',
    book_source VARCHAR(50),
    book_channel VARCHAR(50),
    -- Migration 076 — OTA Desk → PMS write-back Phase 0. `book_channel` (above,
    -- previously unwired) + `book_ext_ref` form the caller-idempotency natural
    -- key so a double-POST of one OTA reservation can't create two bookings.
    -- PG-canonical only (not mirrored to legacy).
    book_ext_ref TEXT,
    book_total_amount DECIMAL(12,2) DEFAULT 0,
    book_deposit_amount DECIMAL(12,2) DEFAULT 0,
    book_deposit_date TIMESTAMP,
    book_special_requests TEXT,
    book_internal_notes TEXT,
    book_notes TEXT,
    -- Migration 064 (task #53) — booking-reminder state for the in-shell
    -- notification bell. PG-CANONICAL ONLY (iHOTEL has no equivalent stored
    -- reminder/dismiss flag). See migrations/pg/064_booking_reminders.sql.
    book_notify_day INTEGER,
    book_notify_note TEXT,
    book_notify_dismissed_at TIMESTAMPTZ,
    book_cancelled_at TIMESTAMP,
    book_cancel_reason VARCHAR(500),
    -- Migration 086 — payment-hold deadline for loyalty-channel TENTATIVE
    -- bookings (book_channel='loyalty', book_status='pending'). PG-canonical
    -- only; the scheduler sweep cancels holds past this instant.
    book_hold_expires_at TIMESTAMPTZ,
    -- Writeback resolver back-populates these (migration 014).
    legacy_book_id VARCHAR(20),
    legacy_cust_no VARCHAR(20),
    aggregate_id UUID,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    book_created_by VARCHAR(50),
    book_updated_by VARCHAR(50),

    CONSTRAINT fk_ht_bookings_customer FOREIGN KEY (book_cust_id)
        REFERENCES ht_customers(cust_id),
    CONSTRAINT ck_ht_bookings_dates CHECK (book_checkout > book_checkin)
);
CREATE INDEX IF NOT EXISTS ix_ht_bookings_customer ON ht_bookings(book_cust_id);
CREATE INDEX IF NOT EXISTS ix_ht_bookings_checkin ON ht_bookings(book_checkin);
CREATE INDEX IF NOT EXISTS ix_ht_bookings_checkout ON ht_bookings(book_checkout);
CREATE INDEX IF NOT EXISTS ix_ht_bookings_status ON ht_bookings(book_status);
CREATE INDEX IF NOT EXISTS ix_ht_bookings_daterange ON ht_bookings(book_checkin, book_checkout);
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_bookings_aggregate_id
    ON ht_bookings (aggregate_id) WHERE aggregate_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS ix_ht_bookings_legacy_book_id
    ON ht_bookings (legacy_book_id) WHERE legacy_book_id IS NOT NULL;
-- Migration 076 — OTA caller-idempotency natural key: at most one booking per
-- (book_channel, book_ext_ref). Partial predicate exempts every existing
-- (NULL ext_ref) row.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_bookings_channel_ext_ref
    ON ht_bookings (book_channel, book_ext_ref) WHERE book_ext_ref IS NOT NULL;
-- Migration 064 (task #53) — active-reminder lookup for the notification bell.
CREATE INDEX IF NOT EXISTS ix_ht_bookings_active_reminders
    ON ht_bookings (book_checkin) WHERE book_notify_dismissed_at IS NULL;
-- Migration 086 — expiry-sweep lookup: pending loyalty-channel holds only.
CREATE INDEX IF NOT EXISTS ix_ht_bookings_hold_expiry
    ON ht_bookings (book_hold_expires_at)
    WHERE book_hold_expires_at IS NOT NULL AND book_status = 'pending';

-- ht_booking_rooms - Junction table for booking-room assignments
CREATE TABLE IF NOT EXISTS ht_booking_rooms (
    br_id SERIAL PRIMARY KEY,
    br_book_id INTEGER NOT NULL,
    br_room_id INTEGER NOT NULL,
    br_room_type_id INTEGER,
    br_price_per_night DECIMAL(10,2) DEFAULT 0,
    br_assigned_at TIMESTAMP,
    br_notes VARCHAR(500),

    CONSTRAINT fk_ht_br_booking FOREIGN KEY (br_book_id)
        REFERENCES ht_bookings(book_id) ON DELETE CASCADE,
    CONSTRAINT fk_ht_br_room FOREIGN KEY (br_room_id)
        REFERENCES ht_rooms_new(room_id),
    CONSTRAINT fk_ht_br_roomtype FOREIGN KEY (br_room_type_id)
        REFERENCES ht_room_types(type_id),
    CONSTRAINT uq_ht_br_bookroom UNIQUE (br_book_id, br_room_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_br_room ON ht_booking_rooms(br_room_id);

-- ht_checkins - Check-in records
CREATE TABLE IF NOT EXISTS ht_checkins (
    cin_id SERIAL PRIMARY KEY,
    cin_no VARCHAR(20) NOT NULL UNIQUE,
    cin_book_id INTEGER,
    cin_cust_id INTEGER NOT NULL,
    cin_room_id INTEGER NOT NULL,
    cin_checkin_time TIMESTAMP NOT NULL,
    cin_checkout_time TIMESTAMP,
    cin_expected_checkout DATE NOT NULL,
    cin_adults INTEGER DEFAULT 1,
    cin_children INTEGER DEFAULT 0,
    cin_status VARCHAR(20) DEFAULT 'active',
    cin_rate_per_night DECIMAL(10,2) DEFAULT 0,
    cin_total_amount DECIMAL(12,2) DEFAULT 0,
    -- Migration 079 — ROOM-ONLY folio total, mirrored from legacy
    -- `HT_CheckIn_H.Total_Price_Room`. `cin_total_amount` above mirrors
    -- `Total_Price_Net` = Room + Product, so it is NOT a usable room basis once
    -- a POS line exists. Deliberately nullable with NO DEFAULT: NULL means
    -- "never projected" and makes the folio read path fall back to
    -- `cin_total_amount`, whereas 0 is a legitimate room charge on a
    -- product-only folio. See migrations/pg/079_ht_checkins_room_amount.sql.
    cin_room_amount DECIMAL(12,2),
    cin_paid_amount DECIMAL(12,2) DEFAULT 0,
    cin_payment_method VARCHAR(50),
    cin_payment_status VARCHAR(50),
    cin_key_card_no VARCHAR(20),
    cin_vehicle_plate VARCHAR(20),
    cin_notes TEXT,
    -- Writeback resolver back-populates these (migration 014).
    legacy_cin_no VARCHAR(20),
    legacy_room_no VARCHAR(10),
    legacy_cust_no VARCHAR(20),
    legacy_checkin_ds_id INTEGER,
    aggregate_id UUID,
    -- Track G9 / T4 HIGH-8 (migration 048) — the cashier shift that
    -- folded the final round-bill (check-out). NULL until the folio is
    -- folded; FK declared after `ht_shifts` table is created below so
    -- the column-order in the table body stays decoupled from the F2
    -- canonical's later definition.
    cin_round_bill_shift_id BIGINT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    cin_created_by VARCHAR(50),
    cin_updated_by VARCHAR(50),

    CONSTRAINT fk_ht_checkins_booking FOREIGN KEY (cin_book_id)
        REFERENCES ht_bookings(book_id),
    CONSTRAINT fk_ht_checkins_customer FOREIGN KEY (cin_cust_id)
        REFERENCES ht_customers(cust_id),
    CONSTRAINT fk_ht_checkins_room FOREIGN KEY (cin_room_id)
        REFERENCES ht_rooms_new(room_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_booking ON ht_checkins(cin_book_id);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_customer ON ht_checkins(cin_cust_id);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_room ON ht_checkins(cin_room_id);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_status ON ht_checkins(cin_status);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_checkin ON ht_checkins(cin_checkin_time);
CREATE INDEX IF NOT EXISTS ix_ht_checkins_expectedout ON ht_checkins(cin_expected_checkout);
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_checkins_aggregate_id
    ON ht_checkins (aggregate_id) WHERE aggregate_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS ix_ht_checkins_legacy_cin_no
    ON ht_checkins (legacy_cin_no) WHERE legacy_cin_no IS NOT NULL;

-- ht_guest_registry - Guest registration (multiple guests per check-in)
CREATE TABLE IF NOT EXISTS ht_guest_registry (
    guest_id SERIAL PRIMARY KEY,
    guest_cin_id INTEGER NOT NULL,
    guest_cust_id INTEGER,
    guest_firstname VARCHAR(100) NOT NULL,
    guest_lastname VARCHAR(100),
    guest_idcard VARCHAR(20),
    guest_passport VARCHAR(50),
    guest_nationality VARCHAR(50),
    guest_is_primary BOOLEAN DEFAULT false,
    guest_created_at TIMESTAMP DEFAULT NOW(),
    -- Track E1 / migration 034 — legacy IDENTITY from
    -- HT_CheckIn_Other_People.id. NULL for rows authored locally
    -- without a legacy counterpart; UNIQUE so the sync mapper can
    -- UPSERT cleanly through iHOTEL's DELETE+REINSERT edit pattern.
    guest_legacy_id INTEGER,

    CONSTRAINT fk_ht_guestreg_checkin FOREIGN KEY (guest_cin_id)
        REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    CONSTRAINT fk_ht_guestreg_customer FOREIGN KEY (guest_cust_id)
        REFERENCES ht_customers(cust_id),
    CONSTRAINT uq_ht_guest_registry_legacy_id UNIQUE (guest_legacy_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_guestreg_checkin ON ht_guest_registry(guest_cin_id);

-- ht_guest_documents - Guest identity documents / photos captured at check-in
-- registration (migration 070). Thai ID card chip image, passport page, webcam
-- face photo. Canonical home for legacy Tb_Save_Image (1:N). doc_legacy_tmp_no is
-- the provisional Tb_Save_Image.tmp_no minted here and echoed to the check-in
-- POST as photoTmpNo; doc_legacy_id is back-populated after the legacy mirror
-- runs. Legacy mirror (writeback/recipes/save_image.rs) is SHIPPED DARK behind
-- GUEST_DOCUMENT_STORAGE_ENABLED (default off). Per-site (connection-level scoping).
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
-- Migration 071 — partial UNIQUE on the legacy id, the UPSERT conflict target for
-- the sync-worker read-in poll (bin/sync.rs::sync_guest_documents) that mirrors
-- legacy Tb_Save_Image → ht_guest_documents (doc_source='legacy'). App-native docs
-- (NULL doc_legacy_id) are exempt via the partial WHERE.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_guest_documents_legacy_id
    ON ht_guest_documents (doc_legacy_id) WHERE doc_legacy_id IS NOT NULL;

-- ht_guest_doc_backfill_skip - convergence backstop for the guest-image sync-in
-- poll (bin/sync.rs::sync_guest_documents): records check-ins that have NO legacy
-- image so the newest-first poll advances past them instead of re-polling the
-- same imageless check-ins forever. DELETE a row to force a re-check.
CREATE TABLE IF NOT EXISTS ht_guest_doc_backfill_skip (
    cin_id       INTEGER PRIMARY KEY REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    attempted_at TIMESTAMP DEFAULT NOW()
);

-- ht_hk_cleaning_events - Maid-reported room-cleaning progress (employee-login
-- plan Phase 4, migration 077). Append-only event log; latest event per room per
-- Thai day = current progress on the /hk maid surface. The TABLE is
-- PG-canonical only (no legacy counterpart, no sync mapper).
-- CHANGED 2026-08-11 (housekeeping-ops): the `done` phase is no longer
-- legacy-inert — routes/hk.rs delegates it to
-- service::housekeeping::mark_clean_if_dirty, which flips
-- ht_rooms_new.room_clean and enqueues the MarkRoomClean writeback in one
-- transaction so reception sees the finished room in iHOTEL. `started` stays
-- PG-only (iHOTEL's Room_Clean_Time drives its room-power countdown).
-- Identity = verified HF ID badge (Cloudflare Access claims), no FK to
-- ht_users. Per-site (connection-level scoping).
-- Migration 087 widened hkev_status to also accept 'dirty' (maid-reported
-- "ห้องยังไม่สะอาด", gated by HK_MARK_DIRTY_ENABLED) — inlined into the CHECK
-- below; the schema_migrations seed row for 087 is near the end of this file.
CREATE TABLE IF NOT EXISTS ht_hk_cleaning_events (
    hkev_id         BIGSERIAL    PRIMARY KEY,
    hkev_room_id    INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    hkev_status     TEXT         NOT NULL CHECK (hkev_status IN ('started', 'done', 'dirty')),
    hkev_badge      TEXT         NOT NULL,
    hkev_name       TEXT,
    hkev_created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_cleaning_events_room_created
    ON ht_hk_cleaning_events (hkev_room_id, hkev_created_at DESC);

-- ht_hk_broken_reports - Maid-submitted broken-item reports (employee-login plan
-- Phase 4, migration 077). A report, not a maintenance ticket — may be promoted
-- to ht_maintenance_requests by staff later. Optional photo stored as BYTEA
-- (same pattern as ht_guest_documents.doc_image). PG-CANONICAL ONLY (no legacy
-- counterpart, no sync, no writeback). Per-site (connection-level scoping).
CREATE TABLE IF NOT EXISTS ht_hk_broken_reports (
    hkbr_id          BIGSERIAL    PRIMARY KEY,
    hkbr_room_id     INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    hkbr_description TEXT         NOT NULL,
    hkbr_badge       TEXT         NOT NULL,
    hkbr_name        TEXT,
    hkbr_photo       BYTEA,
    hkbr_photo_mime  TEXT,
    hkbr_status      TEXT         NOT NULL DEFAULT 'open'
                     CHECK (hkbr_status IN ('open', 'acknowledged', 'resolved')),
    hkbr_created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    hkbr_updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_broken_reports_room_created
    ON ht_hk_broken_reports (hkbr_room_id, hkbr_created_at DESC);
CREATE INDEX IF NOT EXISTS ix_ht_hk_broken_reports_status
    ON ht_hk_broken_reports (hkbr_status, hkbr_created_at DESC);

-- ht_hk_linen_reports - Maid-reported linen shortages (ขาดผ้า) from the /hk
-- surface (migration 088), COMPLETABLE since migration 090.
-- Append-only, one row per (submission, kind); hklr_report_uuid groups the rows
-- of ONE submission and is generated server-side in
-- service::housekeeping::report_linen_shortage.
-- A report is OPEN until a maid marks the room restocked (เติมผ้าแล้ว, migration
-- 090). Completion is ROOM-LEVEL — one tap resolves every open row for that
-- room — and hklr_resolved_at IS NULL is the status, so there is no separate
-- status column to disagree with it. The room's ขาดผ้า indication means "has
-- OPEN reports" of ANY age: completion supersedes day-rollover, the same
-- visible-until-done convention as ht_hk_room_signals. Still append-only — a
-- resolved row keeps everything it was filed with and only gains who/when.
-- PG-CANONICAL ONLY: iHOTEL has no linen counterpart at all, so no sync mapper,
-- no writeback, no domain event, no notification (the boards re-poll).
-- hklr_kind is TEXT with NO CHECK on purpose — the kind allowlist lives in
-- routes::hk::VALID_LINEN_KINDS (bed_sheet | pillowcase | duvet_cover |
-- bath_towel | face_towel | foot_towel), so adding a kind later needs no
-- migration and no window where the deployed binary and the deployed CHECK
-- disagree. The qty bound IS a data invariant and is enforced here as well as
-- in the app.
-- Identity = verified HF ID badge (Cloudflare Access claims) on BOTH sides —
-- who reported and who restocked — with no FK to ht_users.
-- Per-site (connection-level scoping).
CREATE TABLE IF NOT EXISTS ht_hk_linen_reports (
    hklr_id             BIGSERIAL    PRIMARY KEY,
    hklr_report_uuid    UUID         NOT NULL,
    hklr_room_id        INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    hklr_kind           TEXT         NOT NULL,
    hklr_qty            INTEGER      NOT NULL CHECK (hklr_qty >= 1 AND hklr_qty <= 20),
    hklr_badge          TEXT         NOT NULL,
    hklr_name           TEXT,
    hklr_created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    hklr_resolved_at    TIMESTAMPTZ,
    hklr_resolved_badge TEXT,
    hklr_resolved_name  TEXT
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_linen_reports_room_created
    ON ht_hk_linen_reports (hklr_room_id, hklr_created_at DESC);
-- The open-backlog hot path (list EXISTS, detail totals, the resolve UPDATE).
-- PARTIAL, for ix_ht_hk_room_signals_live's reason: resolved rows are unbounded
-- history that only the audit reads.
CREATE INDEX IF NOT EXISTS ix_ht_hk_linen_reports_open
    ON ht_hk_linen_reports (hklr_room_id)
    WHERE hklr_resolved_at IS NULL;

-- ht_hk_room_signals - Canned room signals between reception and maids
-- (ADR 0008, migration 089). One room per signal, broadcast to the other role
-- at that room's branch; NO free-text column anywhere, by decision.
-- Lifecycle open -> acked -> done; the creator's SIDE may cancel while open.
-- A maid's เสร็จแล้ว cleaning report auto-completes that room's open/acked
-- priority_clean + checked_out signals in the SAME transaction
-- (sig_done_source='clean_report'); room_check completes ONLY via its answer
-- endpoint, which also spawns one child signal per problem (sig_parent_id).
-- PG-CANONICAL ONLY: iHOTEL has no counterpart, so no sync mapper, no
-- writeback, no WritebackIntent. The domain events it publishes
-- (RoomSignalRaised/Acked/Completed/Cancelled) are UI plumbing over the
-- existing event_log + pg_notify('domain_events') fan-out.
-- sig_type / sig_outcome / sig_done_source are TEXT with NO CHECK on purpose —
-- the vocabulary lives in domain::hk_signal (mirroring app/hk/signal-vocab.ts)
-- so extending it needs no migration (the 088 rationale). sig_direction and
-- sig_status ARE checked because they are structural, not product vocabulary.
-- sig_escalated_at is both the once-only escalation stamp and the monthly
-- LINE-push quota ledger (HK_ESCALATION_MONTHLY_CAP).
-- Identity = verified HF ID badge (Cloudflare Access claims), no FK to
-- ht_users. Per-site (connection-level scoping).
CREATE TABLE IF NOT EXISTS ht_hk_room_signals (
    sig_id            BIGSERIAL   PRIMARY KEY,
    sig_room_id       INTEGER     NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    sig_direction     TEXT        NOT NULL CHECK (sig_direction IN ('desk_to_maid', 'maid_to_desk')),
    sig_type          TEXT        NOT NULL,
    sig_status        TEXT        NOT NULL DEFAULT 'open'
                                  CHECK (sig_status IN ('open', 'acked', 'done', 'cancelled')),
    sig_outcome       TEXT        NULL,
    sig_parent_id     BIGINT      NULL REFERENCES ht_hk_room_signals(sig_id),
    sig_created_badge TEXT        NOT NULL,
    sig_created_name  TEXT,
    sig_created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sig_acked_badge   TEXT,
    sig_acked_name    TEXT,
    sig_acked_at      TIMESTAMPTZ,
    sig_done_badge    TEXT,
    sig_done_name     TEXT,
    sig_done_at       TIMESTAMPTZ,
    sig_done_source   TEXT        NULL,
    sig_escalated_at  TIMESTAMPTZ NULL
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_signals_live
    ON ht_hk_room_signals (sig_status, sig_room_id)
    WHERE sig_status IN ('open', 'acked');
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_signals_room_created
    ON ht_hk_room_signals (sig_room_id, sig_created_at DESC);

-- ht_hk_room_reports / ht_hk_room_report_items / ht_hk_room_report_photos -
-- Report HK, the maid's per-room daily attestation and reception's
-- countersignature (migration 091; owner's Report HK.xlsx digitized, vocabulary
-- in app/hk/report-vocab.ts and CONTEXT.md Housekeeping).
-- The header carries the room-status code as SHE reported it (vc|co|oo|so,
-- prefilled client-side from known room facts but overridable), the
-- exception-based checklist flag, and the identities of both transitions.
-- Lifecycle submitted -> verified | returned; a RETURNED report is never edited
-- but superseded by a NEW submission carrying rr_parent_id, so history is
-- append-only. There is NO free-text column anywhere in these three tables --
-- the return reason is canned, the same discipline ADR 0008 records for signals.
-- rr_date is the Bangkok civil day the report is FOR, stored (not derived):
-- a maid finishing a floor at 00:10 is still on yesterday's sheet, and
-- CURRENT_DATE is banned because it is the SERVER's date.
-- rr_room_status, rr_return_reason and rri_item are TEXT with NO CHECK on
-- purpose -- the vocabulary lives in domain::hk_report (mirroring
-- app/hk/report-vocab.ts) so extending it needs no migration (the 088
-- rationale). rr_status and rri_problem ARE checked because they are
-- structural: every transition is written over the first, and the second is the
-- pair the item_missing / item_damaged room signals are built on.
-- Items are EXCEPTIONS ONLY: a ครบทุกรายการ report has zero item rows.
-- Photos are BYTEA + mime (the 077 ht_hk_broken_reports pattern), 1..=4 per
-- side; rrp_side is DERIVED from the uploader's role, never client-sent, and
-- rrp_report_id is NULLABLE because a phone uploads BEFORE the form is
-- submitted -- the submit/verify transaction binds them (WHERE rrp_report_id IS
-- NULL AND rrp_badge = the caller), which makes "your own, not already
-- attached" one atomic check. ACCEPTED DEBT: unattached rows linger forever;
-- there is no GC in v1, deliberately.
-- PG-CANONICAL ONLY: iHOTEL has no Report HK counterpart at all, so no sync
-- mapper, no writeback, no WritebackIntent, and no domain event of its own --
-- but a submission with item exceptions raises the EXISTING item_missing /
-- item_damaged room signals (089) in the SAME transaction, one per problem
-- kind, so reception hears about chargeable items immediately.
-- Identity = verified HF ID badge (Cloudflare Access claims) on every side, no
-- FK to ht_users. Per-site (connection-level scoping).
CREATE TABLE IF NOT EXISTS ht_hk_room_reports (
    rr_id              BIGSERIAL   PRIMARY KEY,
    rr_room_id         INTEGER     NOT NULL REFERENCES ht_rooms_new(room_id) ON DELETE CASCADE,
    rr_date            DATE        NOT NULL,
    rr_status          TEXT        NOT NULL DEFAULT 'submitted'
                                   CHECK (rr_status IN ('submitted', 'verified', 'returned')),
    rr_room_status     TEXT        NOT NULL,
    rr_all_items_ok    BOOLEAN     NOT NULL,
    rr_return_reason   TEXT        NULL,
    rr_parent_id       BIGINT      NULL REFERENCES ht_hk_room_reports(rr_id),
    rr_submitted_badge TEXT        NOT NULL,
    rr_submitted_name  TEXT,
    rr_submitted_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    rr_verified_badge  TEXT,
    rr_verified_name   TEXT,
    rr_verified_at     TIMESTAMPTZ
);
-- The day overview's hot path: the LATEST report per room for one date.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_reports_room_date
    ON ht_hk_room_reports (rr_room_id, rr_date, rr_id DESC);
-- The submit guard and reception's queue. PARTIAL, for
-- ix_ht_hk_room_signals_live's reason: judged rows are unbounded history.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_reports_open
    ON ht_hk_room_reports (rr_room_id, rr_date)
    WHERE rr_status = 'submitted';

CREATE TABLE IF NOT EXISTS ht_hk_room_report_items (
    rri_id        BIGSERIAL PRIMARY KEY,
    rri_report_id BIGINT    NOT NULL REFERENCES ht_hk_room_reports(rr_id) ON DELETE CASCADE,
    rri_item      TEXT      NOT NULL,
    rri_problem   TEXT      NOT NULL CHECK (rri_problem IN ('missing', 'damaged')),
    rri_qty       INTEGER   NOT NULL CHECK (rri_qty >= 1 AND rri_qty <= 99)
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_items_report
    ON ht_hk_room_report_items (rri_report_id, rri_id);

CREATE TABLE IF NOT EXISTS ht_hk_room_report_photos (
    rrp_id         BIGSERIAL   PRIMARY KEY,
    rrp_report_id  BIGINT      NULL REFERENCES ht_hk_room_reports(rr_id) ON DELETE CASCADE,
    rrp_side       TEXT        NOT NULL CHECK (rrp_side IN ('maid', 'reception')),
    rrp_photo      BYTEA       NOT NULL,
    rrp_photo_mime TEXT        NOT NULL,
    rrp_badge      TEXT        NOT NULL,
    rrp_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_photos_report
    ON ht_hk_room_report_photos (rrp_report_id, rrp_side, rrp_id)
    WHERE rrp_report_id IS NOT NULL;
-- "My unattached photos" -- the attach predicate's own index.
CREATE INDEX IF NOT EXISTS ix_ht_hk_room_report_photos_open
    ON ht_hk_room_report_photos (rrp_badge, rrp_id)
    WHERE rrp_report_id IS NULL;

-- ht_checkin_rooms - Junction table (Track B1 / migration 043).
-- Mirrors legacy HT_CheckIn_Ds cardinality: one row per room per check-in
-- folio. The existing ht_checkins.cin_room_id stays in place until the
-- B5 backfill completes — readers and the writeback recipe still touch
-- it during the B1-B4 sub-waves.
CREATE TABLE IF NOT EXISTS ht_checkin_rooms (
    cr_id              BIGSERIAL    PRIMARY KEY,
    cr_cin_id          INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    cr_room_id         INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    cr_room_in         TIMESTAMPTZ,
    cr_room_out        TIMESTAMPTZ,
    cr_room_status     VARCHAR(50)  NOT NULL,
    cr_rate_per_night  NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_nights          INTEGER      NOT NULL DEFAULT 1,
    cr_room_total      NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_dep_amount      NUMERIC(12, 2) NOT NULL DEFAULT 0,
    cr_dep_status      VARCHAR(50),
    cr_dep_returned_at TIMESTAMPTZ,
    cr_dep_returned_by VARCHAR(50),
    cr_cupon_count     INTEGER      NOT NULL DEFAULT 0,
    cr_note            VARCHAR(500),
    cr_legacy_ds_id    INTEGER,
    cr_created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    cr_updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_ht_checkin_rooms_folio_room UNIQUE (cr_cin_id, cr_room_id)
);
CREATE INDEX IF NOT EXISTS ht_checkin_rooms_room_status
    ON ht_checkin_rooms (cr_room_id, cr_room_status);
CREATE INDEX IF NOT EXISTS ht_checkin_rooms_legacy_ds_id
    ON ht_checkin_rooms (cr_legacy_ds_id) WHERE cr_legacy_ds_id IS NOT NULL;

-- ht_room_changes - Mid-stay room-move audit (Track G4 / migration 045).
-- Mirrors legacy HT_Changed_Room cardinality (1:1 — one row per move).
-- Receptionists call POST /api/new/checkins/:id/change-room which
-- delegates to service::checkin::change_room, inserts here under one
-- PG transaction (with the junction update), and emits
-- WritebackIntent::RoomChange so writeback/recipes/room_change.rs
-- inserts the corresponding HT_Changed_Room row in legacy MSSQL.
-- Reverse-sync via sync/mappers/mirror.rs::RoomChangeCanonicalMirrorMapper
-- catches moves done from iHOTEL too.
CREATE TABLE IF NOT EXISTS ht_room_changes (
    rc_id                BIGSERIAL    PRIMARY KEY,
    rc_cin_id            INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    rc_from_room_id      INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    rc_to_room_id        INTEGER      NOT NULL REFERENCES ht_rooms_new(room_id),
    rc_reason            VARCHAR(500),
    rc_changed_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rc_changed_by        VARCHAR(64),
    rc_room_before_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    rc_to_price          VARCHAR(20),
    rc_legacy_id         INTEGER,
    rc_created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rc_updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ht_room_changes_cin_id_idx
    ON ht_room_changes (rc_cin_id);
CREATE UNIQUE INDEX IF NOT EXISTS ht_room_changes_legacy_id_uq
    ON ht_room_changes (rc_legacy_id) WHERE rc_legacy_id IS NOT NULL;

-- ht_rates - Room rate configurations
CREATE TABLE IF NOT EXISTS ht_rates (
    rate_id SERIAL PRIMARY KEY,
    rate_room_type_id INTEGER,
    rate_name VARCHAR(100) NOT NULL,
    rate_code VARCHAR(20),
    rate_price DECIMAL(10,2) NOT NULL DEFAULT 0,
    rate_type VARCHAR(20) NOT NULL DEFAULT 'fixed',
    rate_value DECIMAL(10,2),
    rate_valid_from DATE,
    rate_valid_to DATE,
    rate_days_of_week VARCHAR(50),
    rate_min_nights INTEGER DEFAULT 1,
    rate_active BOOLEAN DEFAULT true,
    rate_created TIMESTAMP DEFAULT NOW(),
    rate_updated TIMESTAMP DEFAULT NOW(),

    CONSTRAINT fk_ht_rates_roomtype FOREIGN KEY (rate_room_type_id)
        REFERENCES ht_room_types(type_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_rates_roomtype ON ht_rates(rate_room_type_id);
CREATE INDEX IF NOT EXISTS ix_ht_rates_validdates ON ht_rates(rate_valid_from, rate_valid_to);

-- ht_rate_tiers - Canonical (Room_Type, Cust_Type) pricing matrix
-- (Track F4 / migration 042 — T1 CRIT-4 in audit-2026-05-13.md).
-- Mirrors legacy `HT_Rooms_Price`. `ht_rates` (above) carries the wrong
-- axis (weekday/weekend/special) and is being phased out — see the
-- migration header for the deprecation plan.
CREATE TABLE IF NOT EXISTS ht_rate_tiers (
    rate_tier_id            BIGSERIAL       PRIMARY KEY,
    rate_tier_room_type     VARCHAR(100)    NOT NULL,
    rate_tier_cust_type     VARCHAR(100)    NOT NULL,
    rate_tier_price         NUMERIC(12, 2)  NOT NULL,
    rate_tier_price_hourly  NUMERIC(12, 2),
    rate_tier_price_monthly NUMERIC(12, 2),
    rate_tier_legacy_id     INTEGER,
    rate_tier_active        BOOLEAN         NOT NULL DEFAULT TRUE,
    rate_tier_created_at    TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    rate_tier_updated_at    TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_ht_rate_tiers_room_cust UNIQUE (rate_tier_room_type, rate_tier_cust_type)
);
CREATE INDEX IF NOT EXISTS ix_ht_rate_tiers_room_type
    ON ht_rate_tiers (rate_tier_room_type);

-- ht_settings - System settings
CREATE TABLE IF NOT EXISTS ht_settings (
    setting_id SERIAL PRIMARY KEY,
    setting_key VARCHAR(100) NOT NULL UNIQUE,
    setting_value TEXT,
    setting_type VARCHAR(20) DEFAULT 'string',
    setting_description VARCHAR(500),
    setting_updated_at TIMESTAMP DEFAULT NOW(),
    setting_updated_by VARCHAR(50)
);

-- Insert default settings
INSERT INTO ht_settings (setting_key, setting_value, setting_type, setting_description)
VALUES
    ('hotel.name', 'The HF Hotel', 'string', 'Hotel name'),
    ('hotel.check_in_time', '14:00', 'string', 'Default check-in time'),
    ('hotel.check_out_time', '12:00', 'string', 'Default check-out time'),
    ('booking.prefix', 'BK', 'string', 'Booking number prefix'),
    ('checkin.prefix', 'CI', 'string', 'Check-in number prefix'),
    ('customer.prefix', 'CU', 'string', 'Customer code prefix'),
    -- Wave 5c (migration 038): receipt VAT percent for the legacy
    -- HT_Receipt_H.Receipt_VatPer column. Read by
    -- `repository::settings::get_vat_percent` at payment time.
    ('vat_percent', '7.0', 'string', 'Receipt VAT percent (Wave 5c)')
ON CONFLICT (setting_key) DO NOTHING;

-- =============================================================================
-- Sequences for generating reference numbers
-- =============================================================================

CREATE SEQUENCE IF NOT EXISTS sq_booking_no
    AS INTEGER
    START WITH 1
    INCREMENT BY 1
    MINVALUE 1
    NO MAXVALUE
    NO CYCLE
    CACHE 10;

CREATE SEQUENCE IF NOT EXISTS sq_checkin_no
    AS INTEGER
    START WITH 1
    INCREMENT BY 1
    MINVALUE 1
    NO MAXVALUE
    NO CYCLE
    CACHE 10;

CREATE SEQUENCE IF NOT EXISTS sq_customer_code
    AS INTEGER
    START WITH 1
    INCREMENT BY 1
    MINVALUE 1
    NO MAXVALUE
    NO CYCLE
    CACHE 10;

-- =============================================================================
-- Functions (replacing SQL Server stored procedures)
-- =============================================================================

-- Generate next booking number: prefix + YYMM + 4-digit sequence
CREATE OR REPLACE FUNCTION generate_booking_no()
RETURNS VARCHAR(20) AS $$
DECLARE
    v_next_val INTEGER;
    v_prefix VARCHAR(10);
    v_year_month VARCHAR(6);
BEGIN
    SELECT setting_value INTO v_prefix
    FROM ht_settings
    WHERE setting_key = 'booking.prefix';

    v_prefix := COALESCE(v_prefix, 'BK');
    v_year_month := TO_CHAR(NOW(), 'YYMM');
    v_next_val := nextval('sq_booking_no');

    RETURN v_prefix || v_year_month || LPAD(v_next_val::TEXT, 4, '0');
END;
$$ LANGUAGE plpgsql;

-- Generate next check-in number: prefix + YYMM + 4-digit sequence
CREATE OR REPLACE FUNCTION generate_checkin_no()
RETURNS VARCHAR(20) AS $$
DECLARE
    v_next_val INTEGER;
    v_prefix VARCHAR(10);
    v_year_month VARCHAR(6);
BEGIN
    SELECT setting_value INTO v_prefix
    FROM ht_settings
    WHERE setting_key = 'checkin.prefix';

    v_prefix := COALESCE(v_prefix, 'CI');
    v_year_month := TO_CHAR(NOW(), 'YYMM');
    v_next_val := nextval('sq_checkin_no');

    RETURN v_prefix || v_year_month || LPAD(v_next_val::TEXT, 4, '0');
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- Inventory Tables
-- =============================================================================

-- ht_inventory_categories
CREATE TABLE IF NOT EXISTS ht_inventory_categories (
    cat_id SERIAL PRIMARY KEY,
    cat_name VARCHAR(100) NOT NULL,
    cat_description VARCHAR(255),
    cat_active BOOLEAN DEFAULT true,
    cat_created TIMESTAMP DEFAULT NOW()
);

-- Insert default categories
INSERT INTO ht_inventory_categories (cat_name, cat_description) VALUES
    ('Minibar', 'เครื่องดื่มและของว่างในมินิบาร์'),
    ('Amenities', 'อุปกรณ์อำนวยความสะดวก'),
    ('Linens', 'ผ้าและเครื่องนอน'),
    ('Equipment', 'อุปกรณ์ในห้องพัก')
ON CONFLICT DO NOTHING;

-- ht_products (Track F3 — migration 041)
-- Canonical mirror of legacy `HT_Products`. 1:1 on prod_legacy_no = Pro_no.
-- The sync mapper (`hotel-backend/src/sync/mappers/products.rs`) populates
-- this via periodic poll. The writeback recipe `adjust_product_stock`
-- updates `HT_Products.Pro_Amt` so the stock invariant closes from our
-- app's writes (legacy continues to maintain Pro_Amt for its own sales).
CREATE TABLE IF NOT EXISTS ht_products (
    prod_id           BIGSERIAL PRIMARY KEY,
    prod_legacy_no    VARCHAR(50)  NOT NULL UNIQUE,
    prod_name         VARCHAR(250) NOT NULL,
    prod_unit         VARCHAR(50),
    prod_price        NUMERIC(12,2) NOT NULL DEFAULT 0,
    prod_current_stock NUMERIC(12,3) NOT NULL DEFAULT 0,
    prod_category     VARCHAR(100),
    prod_active       BOOLEAN NOT NULL DEFAULT TRUE,
    aggregate_id      UUID,
    prod_created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    prod_updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ht_products_legacy_no ON ht_products(prod_legacy_no);
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_products_aggregate_id
    ON ht_products (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- ht_inventory_items
-- `inv_product_id` is the Track F3 FK (migration 041) — housekeeping/POS
-- items optionally link to the canonical `ht_products` row. Nullable so
-- the legacy Minibar/Amenities/Linens/Equipment seeds stay un-linked.
CREATE TABLE IF NOT EXISTS ht_inventory_items (
    item_id SERIAL PRIMARY KEY,
    item_code VARCHAR(50) NOT NULL UNIQUE,
    item_name VARCHAR(200) NOT NULL,
    item_category_id INTEGER REFERENCES ht_inventory_categories(cat_id),
    item_unit VARCHAR(50) NOT NULL,
    item_min_stock INTEGER DEFAULT 0,
    item_current_stock INTEGER DEFAULT 0,
    item_cost DECIMAL(10,2),
    item_active BOOLEAN DEFAULT true,
    item_created TIMESTAMP DEFAULT NOW(),
    item_updated TIMESTAMP,
    inv_product_id INTEGER REFERENCES ht_products(prod_id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_category ON ht_inventory_items(item_category_id);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_code ON ht_inventory_items(item_code);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_product
    ON ht_inventory_items(inv_product_id)
    WHERE inv_product_id IS NOT NULL;

-- ht_room_inventory
CREATE TABLE IF NOT EXISTS ht_room_inventory (
    ri_id SERIAL PRIMARY KEY,
    ri_room_id INTEGER NOT NULL,
    ri_item_id INTEGER REFERENCES ht_inventory_items(item_id),
    ri_quantity INTEGER DEFAULT 1,
    ri_last_checked TIMESTAMP
);
CREATE INDEX IF NOT EXISTS ix_ht_room_inventory_room ON ht_room_inventory(ri_room_id);
CREATE INDEX IF NOT EXISTS ix_ht_room_inventory_item ON ht_room_inventory(ri_item_id);

-- ht_inventory_transactions
CREATE TABLE IF NOT EXISTS ht_inventory_transactions (
    trans_id SERIAL PRIMARY KEY,
    trans_item_id INTEGER REFERENCES ht_inventory_items(item_id),
    trans_type VARCHAR(20) NOT NULL,
    trans_quantity INTEGER NOT NULL,
    trans_room_id INTEGER,
    trans_notes VARCHAR(500),
    trans_date TIMESTAMP DEFAULT NOW(),
    trans_by VARCHAR(100)
);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_transactions_item ON ht_inventory_transactions(trans_item_id);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_transactions_date ON ht_inventory_transactions(trans_date);

-- =============================================================================
-- Payment Tracking
-- =============================================================================

-- ht_payments - Payment records (multiple payments per check-in)
CREATE TABLE IF NOT EXISTS ht_payments (
    pay_id SERIAL PRIMARY KEY,
    pay_cin_id INTEGER NOT NULL,
    pay_amount DECIMAL(12,2) NOT NULL,
    pay_method VARCHAR(50) NOT NULL,
    pay_reference VARCHAR(100),
    pay_notes VARCHAR(500),
    pay_date TIMESTAMP DEFAULT NOW(),
    pay_created_by VARCHAR(50),
    pay_voided BOOLEAN DEFAULT false,
    pay_voided_at TIMESTAMP,
    pay_voided_by VARCHAR(50),
    created_at TIMESTAMP DEFAULT NOW(),
    -- Migration 030 — populated by the writeback worker's back-population
    -- step after the legacy `record_payment` recipe allocates them.
    legacy_pay_no VARCHAR(20),
    legacy_receipt_no VARCHAR(20),
    aggregate_id UUID,
    -- Migration 044 — Track G2 / T4 CRIT-1. `refund_of_payment_id` is the
    -- self-referential FK back to the original payment row a refund
    -- offsets. `refund_reason` carries free-text operator context (never
    -- written to legacy MSSQL — no audit column on `HT_CheckIn_Pay`).
    refund_of_payment_id INTEGER,
    refund_reason VARCHAR(500),
    CONSTRAINT fk_ht_payments_checkin FOREIGN KEY (pay_cin_id) REFERENCES ht_checkins(cin_id),
    CONSTRAINT fk_ht_payments_refund_of FOREIGN KEY (refund_of_payment_id)
        REFERENCES ht_payments(pay_id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS ix_ht_payments_checkin ON ht_payments(pay_cin_id);
CREATE INDEX IF NOT EXISTS ix_ht_payments_date ON ht_payments(pay_date);
CREATE INDEX IF NOT EXISTS ix_ht_payments_legacy_pay_no
    ON ht_payments (legacy_pay_no) WHERE legacy_pay_no IS NOT NULL;
CREATE INDEX IF NOT EXISTS ix_ht_payments_legacy_receipt_no
    ON ht_payments (legacy_receipt_no) WHERE legacy_receipt_no IS NOT NULL;
CREATE INDEX IF NOT EXISTS ix_ht_payments_refund_of_payment_id
    ON ht_payments (refund_of_payment_id) WHERE refund_of_payment_id IS NOT NULL;

-- =============================================================================
-- Room Calendar (canonical for HT_Room_Status — Track F1 / migration 039)
-- =============================================================================
--
-- Per-(room, date) booking-calendar ledger. Mirrors legacy
-- `HT_Room_Status`; the canonical sync mapper
-- `sync/mappers/room_calendar.rs` keeps it in lock-step. See
-- migration 039 for the column-by-column rationale.

CREATE TABLE IF NOT EXISTS ht_room_calendar (
    rcal_id              BIGSERIAL    PRIMARY KEY,
    rcal_room_id         INTEGER      NOT NULL,
    rcal_date            DATE         NOT NULL,
    rcal_status          VARCHAR(50)  NOT NULL,
    rcal_book_id         INTEGER,
    rcal_cin_id          INTEGER,
    rcal_customer_label  VARCHAR(500),
    rcal_legacy_id       INTEGER,
    rcal_created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    rcal_updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_ht_room_calendar_room    FOREIGN KEY (rcal_room_id)
        REFERENCES ht_rooms_new(room_id),
    CONSTRAINT fk_ht_room_calendar_booking FOREIGN KEY (rcal_book_id)
        REFERENCES ht_bookings(book_id),
    CONSTRAINT fk_ht_room_calendar_checkin FOREIGN KEY (rcal_cin_id)
        REFERENCES ht_checkins(cin_id),
    CONSTRAINT uq_ht_room_calendar_room_date UNIQUE (rcal_room_id, rcal_date)
);

CREATE INDEX IF NOT EXISTS ix_ht_room_calendar_date_status
    ON ht_room_calendar (rcal_date, rcal_status);

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_room_calendar_legacy_id
    ON ht_room_calendar (rcal_legacy_id) WHERE rcal_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_payments_aggregate_id
    ON ht_payments (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- Payment-dedup hardening (migration 068 / issue #203): one legacy receipt maps
-- to one canonical payment per check-in. Partial UNIQUE blocks retried-sync /
-- replay double-counts. See migrations/pg/068_ht_payments_legacy_receipt_unique.sql.
CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_payments_cin_legacy_receipt_no
    ON ht_payments (pay_cin_id, legacy_receipt_no) WHERE legacy_receipt_no IS NOT NULL;

-- =============================================================================
-- Maintenance System
-- =============================================================================

-- ht_maintenance_categories
CREATE TABLE IF NOT EXISTS ht_maintenance_categories (
    mcat_id SERIAL PRIMARY KEY,
    mcat_name VARCHAR(100) NOT NULL,
    mcat_name_en VARCHAR(100),
    mcat_priority INTEGER DEFAULT 2,
    mcat_active BOOLEAN DEFAULT true
);

-- Insert default maintenance categories
INSERT INTO ht_maintenance_categories (mcat_name, mcat_name_en, mcat_priority) VALUES
    ('ไฟฟ้า', 'Electrical', 3),
    ('ประปา', 'Plumbing', 3),
    ('เครื่องปรับอากาศ', 'Air Conditioning', 3),
    ('เฟอร์นิเจอร์', 'Furniture', 2),
    ('ทั่วไป', 'General', 2)
ON CONFLICT DO NOTHING;

-- ht_maintenance_requests
CREATE TABLE IF NOT EXISTS ht_maintenance_requests (
    mreq_id SERIAL PRIMARY KEY,
    mreq_no VARCHAR(20) NOT NULL UNIQUE,
    mreq_room_id INTEGER NOT NULL,
    mreq_category_id INTEGER NOT NULL,
    mreq_title VARCHAR(200) NOT NULL,
    mreq_description TEXT,
    mreq_priority INTEGER DEFAULT 2,
    mreq_status VARCHAR(20) DEFAULT 'open',
    mreq_assigned_to VARCHAR(100),
    mreq_started_at TIMESTAMP,
    mreq_completed_at TIMESTAMP,
    mreq_resolution TEXT,
    mreq_cost DECIMAL(10,2),
    mreq_created_at TIMESTAMP DEFAULT NOW(),
    mreq_updated_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_mreq_room FOREIGN KEY (mreq_room_id) REFERENCES ht_rooms_new(room_id),
    CONSTRAINT fk_mreq_category FOREIGN KEY (mreq_category_id) REFERENCES ht_maintenance_categories(mcat_id)
);
CREATE INDEX IF NOT EXISTS ix_mreq_room ON ht_maintenance_requests(mreq_room_id);
CREATE INDEX IF NOT EXISTS ix_mreq_status ON ht_maintenance_requests(mreq_status);

-- Sequence for maintenance request numbers
CREATE SEQUENCE IF NOT EXISTS sq_maintenance_no
    AS INTEGER
    START WITH 1
    INCREMENT BY 1;

-- =============================================================================
-- Schema Migration Tracking
-- =============================================================================

CREATE TABLE IF NOT EXISTS schema_migrations (
    id SERIAL PRIMARY KEY,
    version VARCHAR(10) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    checksum VARCHAR(64),
    applied_at TIMESTAMP DEFAULT NOW(),
    applied_by VARCHAR(100) DEFAULT 'init-script'
);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('000', '000_baseline.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Legacy Sync Tables (v2.16.0)
-- =============================================================================

-- ht_rooms_legacy - mirrors HT_Rooms from SQL Server
CREATE TABLE IF NOT EXISTS ht_rooms_legacy (
    id SERIAL PRIMARY KEY,
    room_no VARCHAR(10) NOT NULL UNIQUE,
    room_type VARCHAR(50),
    room_details VARCHAR(200),
    room_clean VARCHAR(10),
    room_use VARCHAR(10),
    room_book VARCHAR(10),
    room_manternace VARCHAR(10),
    room_price_a DECIMAL(10,2),
    room_price_b DECIMAL(10,2),
    room_price_c DECIMAL(10,2),
    room_group VARCHAR(50),
    room_book_name VARCHAR(200),
    room_book_time TIMESTAMP,
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW(),
    new_room_id INTEGER REFERENCES ht_rooms_new(room_id)
);
CREATE INDEX IF NOT EXISTS ix_rooms_legacy_roomno ON ht_rooms_legacy(room_no);
CREATE INDEX IF NOT EXISTS ix_rooms_legacy_synced ON ht_rooms_legacy(synced_at);

-- ht_bookings_legacy - mirrors View_Booking_Ds from SQL Server
CREATE TABLE IF NOT EXISTS ht_bookings_legacy (
    id SERIAL PRIMARY KEY,
    book_no VARCHAR(50) NOT NULL,
    book_date TIMESTAMP,
    book_date_in TIMESTAMP,
    book_date_out TIMESTAMP,
    book_cust_name VARCHAR(200),
    book_cust_id VARCHAR(50),
    book_status INTEGER,
    book_room_type VARCHAR(50),
    book_room_no VARCHAR(50),
    book_total DECIMAL(12,2),
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW(),
    new_booking_id INTEGER REFERENCES ht_bookings(book_id),
    CONSTRAINT uq_bookings_legacy_key UNIQUE (book_no, book_room_type)
);
CREATE INDEX IF NOT EXISTS ix_bookings_legacy_bookno ON ht_bookings_legacy(book_no);
CREATE INDEX IF NOT EXISTS ix_bookings_legacy_synced ON ht_bookings_legacy(synced_at);
CREATE INDEX IF NOT EXISTS ix_bookings_legacy_datein ON ht_bookings_legacy(book_date_in);

-- ht_checkins_legacy - mirrors View_CheckIn_Ds from SQL Server
CREATE TABLE IF NOT EXISTS ht_checkins_legacy (
    id SERIAL PRIMARY KEY,
    cin_no VARCHAR(50) NOT NULL UNIQUE,
    cin_room_no VARCHAR(10),
    cin_room_in TIMESTAMP,
    cin_room_out TIMESTAMP,
    cin_cust_name VARCHAR(200),
    cin_cust_no VARCHAR(50),
    cin_status VARCHAR(50),
    cin_checkin_no VARCHAR(50),
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW(),
    new_checkin_id INTEGER REFERENCES ht_checkins(cin_id)
);
CREATE INDEX IF NOT EXISTS ix_checkins_legacy_cinno ON ht_checkins_legacy(cin_no);
CREATE INDEX IF NOT EXISTS ix_checkins_legacy_synced ON ht_checkins_legacy(synced_at);
CREATE INDEX IF NOT EXISTS ix_checkins_legacy_roomin ON ht_checkins_legacy(cin_room_in);

-- ht_customers_legacy - mirrors View_Customers from SQL Server
CREATE TABLE IF NOT EXISTS ht_customers_legacy (
    id SERIAL PRIMARY KEY,
    cust_no VARCHAR(100) NOT NULL UNIQUE,
    cust_name VARCHAR(200),
    cust_type VARCHAR(100),
    cust_phone VARCHAR(200),
    cust_idcard VARCHAR(100),
    cust_address VARCHAR(500),
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW(),
    new_cust_id INTEGER REFERENCES ht_customers(cust_id)
);
CREATE INDEX IF NOT EXISTS ix_customers_legacy_custno ON ht_customers_legacy(cust_no);
CREATE INDEX IF NOT EXISTS ix_customers_legacy_synced ON ht_customers_legacy(synced_at);

-- sync_status - tracks sync health per entity type
CREATE TABLE IF NOT EXISTS sync_status (
    id SERIAL PRIMARY KEY,
    entity_type VARCHAR(50) NOT NULL UNIQUE,
    last_sync_at TIMESTAMP,
    records_synced INTEGER DEFAULT 0,
    records_added INTEGER DEFAULT 0,
    records_updated INTEGER DEFAULT 0,
    records_unchanged INTEGER DEFAULT 0,
    sync_duration_ms INTEGER DEFAULT 0,
    last_error TEXT,
    last_error_at TIMESTAMP,
    consecutive_failures INTEGER DEFAULT 0
);

INSERT INTO sync_status (entity_type) VALUES
    ('customers'),
    ('rooms'),
    ('bookings'),
    ('checkins'),
    -- Migration 080 — Phase 6-A payments reconcile arm (ships DARK behind
    -- RECONCILE_PAYMENTS_ARM_ENABLED). `record_success` UPDATEs by
    -- entity_type, so the row must exist or the counters silently no-op.
    ('payments'),
    -- Migration 081 — Phase 6-B guest-registry (companion folio) reconcile
    -- arm (ships DARK behind RECONCILE_GUEST_REGISTRY_ARM_ENABLED). Same
    -- reason: `record_success` UPDATEs by entity_type.
    ('guest_registry'),
    -- Migration 082 — Phase 6-C generic mirror probe (ships DARK behind
    -- RECONCILE_MIRROR_PROBE_ENABLED). Same reason again: `record_error`
    -- UPDATEs by entity_type, so without this row a probe failure updates
    -- zero rows and leaves only a log line.
    ('mirror_probe'),
    -- Migration 083 — Phase 6-D payment-ledger per-folio probe (ships DARK
    -- behind RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED). Same reason again:
    -- without this row a probe failure updates zero rows and leaves only a
    -- log line, and the success path has no consecutive_failures to reset.
    ('payment_ledger_probe')
ON CONFLICT (entity_type) DO NOTHING;

-- Add source column to existing tables
ALTER TABLE ht_bookings ADD COLUMN IF NOT EXISTS source VARCHAR(20) DEFAULT 'new';
ALTER TABLE ht_checkins ADD COLUMN IF NOT EXISTS source VARCHAR(20) DEFAULT 'new';
ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS source VARCHAR(20) DEFAULT 'new';

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('008', '008_legacy_sync_tables.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 009: widen legacy varchar columns
-- The widened types (cust_no/cust_type/cust_phone/cust_idcard) are already
-- baked into the ht_customers_legacy CREATE TABLE above. Seed the row so a
-- fresh init does not have migrate.sh re-apply 009 on top.
-- =============================================================================

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('009', '009_widen_legacy_varchar_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- HF Ville Local Cache Schema (v2.22.0) — RETIRED in v2.55.0 (task #77)
-- =============================================================================
-- Migration 010 created the `ville` schema in `hotelnew` as a local cache
-- written by the FreeTDS-based `ville_sync` worker on the HF Ville jumpbox.
-- Phase 5 Ville cutover (#76, 2026-04-30) repointed reads at the new
-- `hotelville` PG database (fed by the central `sync-hfville` CT watcher),
-- and migration 025 drops the orphaned `ville` schema entirely.
--
-- For fresh deployments we skip the migration-010 DDL outright: there is
-- no value in creating tables we are about to drop. The seed rows below
-- mark BOTH versions as applied so `migrate.sh` reports zero pending
-- migrations on top of this seed (per the drift-check contract).

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('010', '010_ville_cache_schema.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 011: writeback_jobs (outbox queue for legacy MSSQL writeback)
-- Per docs/architecture.md §4c.
-- =============================================================================

CREATE TABLE IF NOT EXISTS writeback_jobs (
    id                BIGSERIAL    PRIMARY KEY,
    intent            TEXT         NOT NULL,
    payload           JSONB        NOT NULL,
    aggregate_id      UUID         NOT NULL,
    idempotency_key   UUID         NOT NULL UNIQUE,
    -- Status lifecycle: pending → in_progress → done (success)
    --                                        → failed (transient, will retry per next_retry_at)
    --                                        → exhausted (attempts >= max, requires operator)
    status            TEXT         NOT NULL DEFAULT 'pending',
    attempts          INT          NOT NULL DEFAULT 0,
    last_error        TEXT,
    legacy_ids        JSONB,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    -- Set by claim_next_job; lets the janitor reset stuck `in_progress` rows.
    claimed_at        TIMESTAMPTZ,
    -- Set by mark_failed; gates retry until backoff window elapses.
    next_retry_at     TIMESTAMPTZ,
    completed_at      TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_claim
    ON writeback_jobs (status, next_retry_at, created_at)
    WHERE status IN ('pending', 'failed', 'in_progress');

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_aggregate
    ON writeback_jobs (aggregate_id);

CREATE INDEX IF NOT EXISTS ix_writeback_jobs_exhausted
    ON writeback_jobs (created_at DESC)
    WHERE status = 'exhausted';

-- Trigger: NOTIFY writeback_channel on every INSERT so the worker wakes
-- immediately instead of waiting on the 30-second poll fallback. Migration 016.
CREATE OR REPLACE FUNCTION writeback_jobs_notify() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('writeback_channel', NEW.id::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS writeback_jobs_notify_trigger ON writeback_jobs;
CREATE TRIGGER writeback_jobs_notify_trigger
    AFTER INSERT ON writeback_jobs
    FOR EACH ROW
    EXECUTE FUNCTION writeback_jobs_notify();

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('011', '011_writeback_jobs.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 012: event_log (durable domain-event bus)
-- Per docs/architecture.md §3.6, §4d-bis.
-- =============================================================================

CREATE TABLE IF NOT EXISTS event_log (
    id                  UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type          TEXT         NOT NULL,
    aggregate_id        UUID,
    payload             JSONB        NOT NULL,
    source_kind         TEXT         NOT NULL,
    source_user_id      UUID,
    source_request_id   UUID,
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS ix_event_log_created_at
    ON event_log (created_at DESC);

CREATE INDEX IF NOT EXISTS ix_event_log_aggregate_created
    ON event_log (aggregate_id, created_at DESC);

CREATE INDEX IF NOT EXISTS ix_event_log_type_created
    ON event_log (event_type, created_at DESC);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('012', '012_event_log.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 013: legacy_ct_state (Change Tracking watermark)
-- Per docs/architecture.md §4d-tris.
-- =============================================================================

CREATE TABLE IF NOT EXISTS legacy_ct_state (
    id                  BIGINT       PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    last_seen_version   BIGINT       NOT NULL DEFAULT 0,
    last_polled_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

INSERT INTO legacy_ct_state (id, last_seen_version)
VALUES (1, 0)
ON CONFLICT DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('013', '013_legacy_ct_state.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 014: legacy_* + aggregate_id columns on canonical tables
-- The columns + partial indexes are already baked into ht_bookings/ht_checkins/
-- ht_rooms_new above. Seed the row so a fresh init does not re-apply 014.
-- =============================================================================

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('014', '014_legacy_id_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 015: writeback_jobs retry state (claimed_at, next_retry_at)
-- Columns + ix_writeback_jobs_claim + ix_writeback_jobs_exhausted are already
-- baked into the writeback_jobs CREATE TABLE above. Seed the row so a fresh
-- init does not re-apply 015.
-- =============================================================================

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('015', '015_writeback_retry_state.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 016: writeback_jobs NOTIFY trigger
-- The function + trigger are already created above. Seed the row so a fresh
-- init does not re-apply 016.
-- =============================================================================

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('016', '016_writeback_notify_trigger.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 017: legacy_sync_status (per-table CT watcher observability)
-- Per docs/architecture.md §3.6d, §3.7. Adds the soft-delete column on
-- ht_customers (Phase 5.2 HT_Customers `D` events will populate it) and
-- the per-table progress / health table consumed by `bin/sync.rs`.
-- =============================================================================

ALTER TABLE ht_customers
    ADD COLUMN IF NOT EXISTS cust_deleted_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS legacy_sync_status (
    table_name           TEXT        PRIMARY KEY,
    last_processed_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    rows_ingested        BIGINT      NOT NULL DEFAULT 0,
    rows_skipped         BIGINT      NOT NULL DEFAULT 0,
    last_error           TEXT,
    last_error_at        TIMESTAMPTZ,
    consecutive_failures INT         NOT NULL DEFAULT 0
);

INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H')
ON CONFLICT (table_name) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('017', '017_legacy_sync_status.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 018: ht_customers aggregate keys (Phase 5.2 — customer CT mapper)
-- Per docs/architecture.md §3.6d. Adds legacy_cust_no + aggregate_id so the
-- HT_Customers CT mapper can map MSSQL Cust_no → canonical row and emit
-- DomainEvent::Customer{Created,Modified} with stable aggregate UUIDs.
-- =============================================================================

ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS legacy_cust_no VARCHAR(20);
ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS aggregate_id   UUID;

CREATE UNIQUE INDEX IF NOT EXISTS ht_customers_legacy_cust_no_uniq
    ON ht_customers (legacy_cust_no) WHERE legacy_cust_no IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_customers_aggregate_id_uniq
    ON ht_customers (aggregate_id) WHERE aggregate_id IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('018', '018_ht_customers_aggregate_keys.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 055: ht_customers.legacy_id (customer hard-delete handling)
-- Mirrors migrations/pg/055_add_legacy_id_to_ht_customers.sql. CT D-rows
-- for HT_Customers carry only the legacy SERIAL `id`; persisting it on the
-- canonical row lets apply_soft_delete resolve iHOTEL hard-deletes (audit
-- 2026-06-11 P1 #6 — every FrmManageCustomersNew delete was a silent no-op).
-- =============================================================================

ALTER TABLE ht_customers ADD COLUMN IF NOT EXISTS legacy_id INTEGER;

CREATE INDEX IF NOT EXISTS idx_ht_customers_legacy_id
    ON ht_customers (legacy_id) WHERE legacy_id IS NOT NULL;

COMMENT ON COLUMN ht_customers.legacy_id IS
  'Legacy HT_Customers.id (MSSQL SERIAL PK). CT D-rows carry only this '
  'key, so customer hard-deletes resolve by it. NULL for rows mirrored '
  'before migration 055 that have not been re-touched by CT since.';

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('055', '055_add_legacy_id_to_ht_customers.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 019: ht_reconcile_log (Phase 5.5 — drift-detection tripwire)
-- Per docs/architecture.md §3.6d. CT watcher is now authoritative for
-- canonical PG state; the legacy 5-min reconcile job is demoted to a
-- 15-min diff-only safety net that LOGS divergent rows here instead of
-- UPSERTing them. See migrations/pg/019_ht_reconcile_log.sql for full
-- rationale.
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_reconcile_log (
    id                BIGSERIAL    PRIMARY KEY,
    detected_at       TIMESTAMPTZ  NOT NULL DEFAULT now(),
    table_name        TEXT         NOT NULL,
    legacy_pk         TEXT         NOT NULL,
    pg_hash           TEXT,
    mssql_hash        TEXT,
    mssql_row_json    JSONB,
    pg_row_json       JSONB,
    resolved_at       TIMESTAMPTZ,
    -- Migration 032 — Track D / T7 CRIT-1. Discriminator + row counts so
    -- cardinality drift (multi-room folio collapse) is never ack-silenced.
    divergence_kind   TEXT,
    legacy_row_count  INT,
    pg_row_count      INT
);

CREATE INDEX IF NOT EXISTS idx_ht_reconcile_log_unresolved
    ON ht_reconcile_log (detected_at)
    WHERE resolved_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_ht_reconcile_log_table_unresolved
    ON ht_reconcile_log (table_name, detected_at)
    WHERE resolved_at IS NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('019', '019_ht_reconcile_log.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 054: clarify the table semantic for future operators.
-- Mirrors migrations/pg/054_comment_ht_reconcile_log_semantic.sql.
COMMENT ON TABLE ht_reconcile_log IS
  'Sync-lag observations. Not durable divergence. Rows are logged when '
  'the diff-only reconcile sweep notices the legacy and canonical hashes '
  'do not yet match; the auto-resolve sweep re-hashes both sides every '
  'tick and closes rows where they have converged. Only rows that resist '
  'multiple sweep cycles (>4h unresolved) represent actual durable '
  'divergence requiring operator action. See docs/architecture.md §3.6d.';

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('054', '054_comment_ht_reconcile_log_semantic.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 020: legacy_mirror schema (Phase 5.5a)
--
-- Opaque pass-through mirror of 11 legacy-only tables. See
-- migrations/pg/020_legacy_mirror_schema.sql for the full rationale.
-- =============================================================================

CREATE SCHEMA IF NOT EXISTS legacy_mirror;

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_cupon (
    cupon_no       INTEGER          PRIMARY KEY,
    cupon_cin_no   TEXT,
    cupon_cin_room TEXT,
    cupon_date     TIMESTAMP,
    cupon_gen_date TIMESTAMP,
    cupon_by       TEXT,
    cupon_print    INTEGER          NOT NULL DEFAULT 0,
    mirrored_at    TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source  TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_cupon_cin_no_idx
    ON legacy_mirror.ht_cupon (cupon_cin_no);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_checkin_product (
    id                  INTEGER          PRIMARY KEY,
    cin_no              TEXT,
    cin_room_no         TEXT,
    cin_ds_date         TIMESTAMP,
    cin_pro_id          TEXT,
    cin_pro_name        TEXT,
    cin_pro_unit        TEXT,
    cin_pro_num         DOUBLE PRECISION,
    cin_pro_price       DOUBLE PRECISION,
    cin_pro_pricetotal  DOUBLE PRECISION,
    cin_pro_pay         DOUBLE PRECISION,
    cin_pro_note        TEXT,
    mirrored_at         TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source       TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_checkin_product_cin_no_idx
    ON legacy_mirror.ht_checkin_product (cin_no);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_deposit (
    id            INTEGER          PRIMARY KEY,
    dep_no        TEXT,
    dep_date      TIMESTAMP,
    dep_room      TEXT,
    dep_name      TEXT,
    dep_price     DOUBLE PRECISION,
    dep_status    TEXT,
    dep_ref       TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_continuetime (
    id            INTEGER          PRIMARY KEY,
    con_name      TEXT,
    con_minute    INTEGER,
    con_price     DOUBLE PRECISION,
    con_type      TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_changed_room (
    id                INTEGER          PRIMARY KEY,
    cin_no            TEXT             NOT NULL,
    room_before       TEXT,
    room_after        TEXT,
    change_date       TIMESTAMP,
    room_before_price DOUBLE PRECISION NOT NULL DEFAULT 0,
    note              TEXT,
    toprice           TEXT,
    mirrored_at       TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source     TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_changed_room_cin_no_idx
    ON legacy_mirror.ht_changed_room (cin_no);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_rooms_cancel (
    id            INTEGER          PRIMARY KEY,
    room_no       TEXT,
    cin_no        TEXT,
    cancel_date   TIMESTAMP,
    cancel_by     TEXT,
    cancel_note   TEXT,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_rooms_cancel_cin_no_idx
    ON legacy_mirror.ht_rooms_cancel (cin_no);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_rooms_price (
    id              INTEGER          PRIMARY KEY,
    room_type       TEXT,
    room_custtype   TEXT,
    room_price      DOUBLE PRECISION,
    room_price_h    DOUBLE PRECISION,
    room_price_m    DOUBLE PRECISION,
    mirrored_at     TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source   TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_bill_debt_h (
    bill_no            TEXT             PRIMARY KEY,
    bill_cust_id       TEXT,
    bill_cust_name     TEXT,
    bill_cust_address  TEXT,
    bill_cust_tel      TEXT,
    bill_cust_fax      TEXT,
    bill_date          TIMESTAMP,
    bill_ref           TEXT,
    bill_price_type    TEXT,
    bill_type          TEXT,
    bill_total         DOUBLE PRECISION,
    bill_pay           DOUBLE PRECISION,
    bill_debt          DOUBLE PRECISION,
    bill_pay_cash      DOUBLE PRECISION,
    bill_pay_credit    DOUBLE PRECISION,
    bill_status        TEXT,
    bill_by            TEXT,
    bill_note          TEXT,
    mirrored_at        TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source      TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_bill_debt_ds (
    id              INTEGER          PRIMARY KEY,
    bill_no         TEXT,
    ds_id           INTEGER,
    ds_no           TEXT,
    ds_name         TEXT,
    ds_unit         TEXT,
    ds_num          DOUBLE PRECISION,
    ds_price        DOUBLE PRECISION,
    ds_price_total  DOUBLE PRECISION,
    mirrored_at     TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source   TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_bill_debt_ds_bill_no_idx
    ON legacy_mirror.ht_bill_debt_ds (bill_no);

-- ht_order_up / ht_order_down — pricing-tier matrices. PK is composite
-- (id, cust_type, cast_type) per migration 023 — `id` alone is a tier
-- number with duplicates, NOT a unique key. See migration 023 header
-- comment for the diagnosis (Phase 5 Ville bootstrap, 2026-04-29).
CREATE TABLE IF NOT EXISTS legacy_mirror.ht_order_up (
    id            INTEGER          NOT NULL,
    cust_type     TEXT             NOT NULL,
    cust_month    INTEGER,
    cast_type     TEXT             NOT NULL,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL,
    PRIMARY KEY (id, cust_type, cast_type)
);

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_order_down (
    id            INTEGER          NOT NULL,
    cust_type     TEXT             NOT NULL,
    cust_month    INTEGER,
    cast_type     TEXT             NOT NULL,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL,
    PRIMARY KEY (id, cust_type, cast_type)
);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('020', '020_legacy_mirror_schema.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 022: legacy_mirror sync_status seed (Phase 5.5c)
--
-- Add 6 rows to legacy_sync_status so the CT watcher tracks per-table
-- observability for the legacy_mirror.* tables. See migrations/pg/022
-- and bin/sync.rs::CT_ENABLED_TABLES for the corresponding mapper
-- wiring.
-- =============================================================================

INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds')
ON CONFLICT (table_name) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('022', '022_legacy_mirror_sync_status_seed.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 023 — `legacy_mirror.ht_order_up` / `ht_order_down` PK is composite
-- (id, cust_type, cast_type). The schema above already reflects this; this
-- INSERT marks 023 as applied for fresh deploys so migrate.sh doesn't try to
-- re-apply it on top of the already-correct baseline (drift-check requirement).
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('023', '023_legacy_mirror_order_composite_pk.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 024 — canonical `ht_customers` VARCHAR widening. The schema above
-- already reflects the wider widths (cust_phone 200, cust_idcard 100, etc.).
-- Same drift-check rationale as 023.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('024', '024_widen_canonical_customer_varchars.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 025 — drop the obsolete `ville` schema that migration 010
-- introduced for the now-retired ville_sync worker (task #77, v2.55.0).
-- Fresh deploys skip the migration-010 DDL altogether (see the section
-- above), so there is nothing to drop here. Seeded for drift-check parity.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('025', '025_drop_ville_schema.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 026 — Phase 1 deploy-pipeline soak test. Pure no-op
-- (`SELECT 1 WHERE FALSE`); seeded here for drift-check parity.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('026', '026_phase1_soak_no_op.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 027: ht_users (Phase 4 PR1 — auth foundation)
-- Local username + Argon2id password hash + role. PR1 lays the schema +
-- service layer; PR2 wires HTTP routes + Axum middleware on top.
-- =============================================================================

-- Migration 046 (Track G7) widens the legacy single-role CHECK to also
-- accept 'cashier' and 'housekeeper', and adds the display_name column.
-- Both are baked inline here for fresh deployments.
CREATE TABLE IF NOT EXISTS ht_users (
    user_id        BIGSERIAL    PRIMARY KEY,
    username       VARCHAR(64)  NOT NULL UNIQUE,
    password_hash  TEXT         NOT NULL,
    role           VARCHAR(16)  NOT NULL
                   CONSTRAINT ht_users_role_check
                   CHECK (role IN ('admin', 'cashier', 'housekeeper', 'receptionist')),
    active         BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at     TIMESTAMP    NOT NULL DEFAULT NOW(),
    last_login_at  TIMESTAMP,
    display_name   VARCHAR(128),
    -- Migration 074 (Cloudflare Access auto-login) — optional alternate
    -- lookup key mapping a verified CF Access `email` claim to this row.
    -- NULL = password-only user. Case-insensitive uniqueness enforced by
    -- ux_ht_users_email_lower below.
    email          VARCHAR(255),
    -- Migration 075 (NFC staff-card login) — optional alternate lookup key
    -- mapping a resolved HF-ID card `badge` to this row. NULL = no card
    -- (password / CF-Access only). One badge → one user via
    -- ux_ht_users_badge below. Never a credential — the card→person
    -- authority lives centrally in HF-ID; the PMS only trusts the resolve.
    badge          VARCHAR(50)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_users_email_lower
    ON ht_users (LOWER(email))
    WHERE email IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_users_badge
    ON ht_users (badge)
    WHERE badge IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('027', '027_create_ht_users.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 028: ht_sessions (Phase 4 PR1 — auth foundation)
-- Server-side session table; the HttpOnly cookie value IS the PK.
-- ON DELETE CASCADE wipes a user's sessions when the user row is removed.
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_sessions (
    session_id  VARCHAR(64) PRIMARY KEY,
    user_id     BIGINT      NOT NULL REFERENCES ht_users(user_id) ON DELETE CASCADE,
    created_at  TIMESTAMP   NOT NULL DEFAULT NOW(),
    expires_at  TIMESTAMP   NOT NULL,
    ip          INET,
    user_agent  TEXT
);

CREATE INDEX IF NOT EXISTS ix_ht_sessions_expires_at
    ON ht_sessions (expires_at);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('028', '028_create_ht_sessions.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- 029_normalize_cin_status_terminal.sql
-- Canonicalize ht_checkins.cin_status post-checkout terminal value on
-- 'checkedout'. No-op on a fresh database (no rows exist) but kept for
-- drift-check parity with migrations/pg/029_normalize_cin_status_terminal.sql.
-- =============================================================================

UPDATE ht_checkins
   SET cin_status = 'checkedout',
       updated_at = NOW()
 WHERE cin_status IN ('checked_out', 'completed');

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('029', '029_normalize_cin_status_terminal.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 030 — ht_payments.legacy_pay_no + legacy_receipt_no + aggregate_id
-- (Wave 5a writeback-audit item 3). The DDL is inlined into the `ht_payments`
-- CREATE TABLE block above; this seed row records the migration as already
-- applied so the drift check's `migrate.sh` pass against a fresh seed sees
-- zero pending migrations.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('030', '030_add_ht_payments_legacy_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 032 — ht_reconcile_log cardinality-aware columns (Track D /
-- T7 CRIT-1). `divergence_kind` + `legacy_row_count` + `pg_row_count`
-- are inlined into the `ht_reconcile_log` CREATE TABLE block above;
-- this seed row marks the migration as applied for fresh deploys.
-- (031 is intentionally absent — it was never landed.)
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('032', '032_ht_reconcile_log_cardinality.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 033: Track E1 sync_status seed (HT_CheckIn_Other_People +
-- HT_Rooms_Cancel). Two CT-watched tables get mappers in 2.63.12; the
-- watcher requires a matching legacy_sync_status row to track per-tick
-- progress. Seed for drift-check parity.
-- =============================================================================

INSERT INTO legacy_sync_status (table_name)
VALUES
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel')
ON CONFLICT (table_name) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('033', '033_sync_status_seed_track_e1.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 034 — `ht_guest_registry.guest_legacy_id` column + UNIQUE
-- constraint to support the Track E1 companion-guest sync mapper. The
-- column is inlined into the CREATE TABLE block above; this seed row
-- marks the migration as applied for fresh deploys.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('034', '034_ht_guest_registry_legacy_id.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 035 — Track E2 / T1 HIGH-2. Widen `ht_customers` to mirror
-- the full legacy `HT_Customers` surface (27 new columns including
-- `cust_price_over`, address tuple, work-address tuple, `cust_name2`,
-- `cust_sex`, `cust_contry`, etc.). The columns are inlined into the
-- `ht_customers` CREATE TABLE block above; this seed marks the
-- migration as applied for fresh deploys.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('035', '035_track_e2_customer_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 036 — Track E2 / T1 HIGH-3. Widen `ht_rooms_new` with 8
-- columns from legacy `HT_Rooms` (`room_use_count`, `room_x/y`,
-- `room_group`, `room_power_*`, `room_polity`). The columns are
-- inlined into the `ht_rooms_new` CREATE TABLE block above; this seed
-- marks the migration as applied for fresh deploys.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('036', '036_track_e2_room_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 037: scheduler_notification_state. Persisted Slack watermark
-- per (site, notification type) — fixes the post-redeploy replay storm
-- where ~45 historical checkouts were re-paged because the in-memory
-- watermark reset to UTC-now on container restart (and MSSQL stores
-- Thai local time, so UTC-now ≈ 7h behind any "real" event time).
-- =============================================================================

CREATE TABLE IF NOT EXISTS scheduler_notification_state (
    site_id           TEXT        NOT NULL,
    notification_type TEXT        NOT NULL,
    last_event_at     TIMESTAMP   NOT NULL,
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (site_id, notification_type)
);

CREATE OR REPLACE FUNCTION scheduler_notification_state_touch_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_scheduler_notification_state_touch_updated_at
    ON scheduler_notification_state;

CREATE TRIGGER trg_scheduler_notification_state_touch_updated_at
    BEFORE UPDATE ON scheduler_notification_state
    FOR EACH ROW
    EXECUTE FUNCTION scheduler_notification_state_touch_updated_at();

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('037', '037_scheduler_notification_state.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 038 — Wave 5c seed already applied above (the `vat_percent`
-- row in the ht_settings defaults block). Record the version so the
-- drift checker doesn't reapply on a fresh init.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('038', '038_seed_vat_percent.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 039 — Track F1 / T1 HIGH-4. New canonical `ht_room_calendar`
-- table mirroring legacy `HT_Room_Status`. Table DDL is inlined above;
-- this seed marks the migration as applied for fresh deploys.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('039', '039_create_ht_room_calendar.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 041 — Track F3 / T1 CRIT-3. `ht_products` canonical mirror of
-- `HT_Products` + `ht_inventory_items.inv_product_id` FK linkage already
-- applied inline above (see the inventory section). Record the version so
-- the drift checker doesn't reapply on a fresh init.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('041', '041_create_ht_products.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 042 — Track F4 / T1 CRIT-4. Canonical `ht_rate_tiers`
-- (composite key on Room_Type × Cust_Type) mirrored to legacy
-- `HT_Rooms_Price`. The DDL is inlined into the `ht_rate_tiers` CREATE
-- TABLE block above; this seed marks the migration as applied for
-- fresh deploys. (040 intentionally absent — never landed.)
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('042', '042_create_ht_rate_tiers.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 040: ht_shifts canonical (Track F2 / T1 HIGH-5).
-- iHOTEL gates payments behind an open `HT_Round_Bill` shift. Our app
-- accepted payments anytime, blocking cash-drawer reconciliation. F2
-- lands the PG canonical + a service-layer gate; the legacy mirror
-- writeback + CT sync are deferred to Track G.
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_shifts (
    shift_id              BIGSERIAL PRIMARY KEY,
    shift_site_id         VARCHAR(50) NOT NULL,
    shift_no              INTEGER     NOT NULL,
    shift_opening_float   NUMERIC(12,2) NOT NULL DEFAULT 0,
    shift_opened_by       VARCHAR(50) NOT NULL,
    shift_opened_at       TIMESTAMPTZ NOT NULL,
    shift_closed_at       TIMESTAMPTZ,
    shift_closed_by       VARCHAR(50),
    shift_legacy_round_id INTEGER,
    shift_notes           TEXT,
    shift_created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Migration 058 (Track J7c) — cash-drawer reconciliation at close.
    shift_counted_cash    NUMERIC(14,2),
    shift_cash_count      JSONB,
    UNIQUE (shift_site_id, shift_no)
);

-- Partial unique index: at most one open shift per site.
CREATE UNIQUE INDEX IF NOT EXISTS ht_shifts_one_open_per_site
    ON ht_shifts (shift_site_id)
    WHERE shift_closed_at IS NULL;

-- Listing recent shifts (cash-drawer review) reads
-- "site filter + opened_at DESC" — this index covers it.
CREATE INDEX IF NOT EXISTS ix_ht_shifts_site_opened_at
    ON ht_shifts (shift_site_id, shift_opened_at DESC);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('040', '040_create_ht_shifts.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 043 — Track B1 / T1 CRIT-1 + T2 CRIT-1. Canonical
-- `ht_checkin_rooms` junction table mirroring legacy `HT_CheckIn_Ds`
-- (one row per room per check-in folio). DDL inlined above next to
-- `ht_guest_registry`; this seed marks the migration as applied for
-- fresh deploys. Follow-on sub-waves B2 (mapper) / B3 (dashboard) /
-- B4 (writeback) / B5 (backfill) layer behavior changes on top.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('043', '043_create_ht_checkin_rooms.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 044 — Track G2 / T4 CRIT-1. `refund_of_payment_id` +
-- `refund_reason` columns on `ht_payments` so the new refund flow
-- (service::payment::refund_payment + WritebackIntent::RefundPayment)
-- can link a negative-amount refund row back to the original payment
-- it offsets. DDL inlined into the `ht_payments` CREATE TABLE block
-- above; this seed marks the migration as applied for fresh deploys.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('044', '044_ht_payments_refund_columns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 045 — Track G4 / T4 HIGH-3. Canonical `ht_room_changes`
-- audit table mirroring legacy `HT_Changed_Room` (mid-stay room-move
-- audit, 1:1 cardinality). Service path: POST
-- /api/new/checkins/:id/change-room → service::checkin::change_room
-- (insert + junction UPDATE inside one PG tx) → outbox emits
-- WritebackIntent::RoomChange → writeback/recipes/room_change.rs
-- INSERTs HT_Changed_Room in legacy MSSQL. Reverse-sync handled by
-- sync/mappers/mirror.rs::RoomChangeCanonicalMirrorMapper so moves
-- done from iHOTEL also land canonically.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('045', '045_create_ht_room_changes.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 048 — Track G9 / T4 HIGH-8. Add the FK on
-- `ht_checkins.cin_round_bill_shift_id` -> `ht_shifts.shift_id` now
-- that `ht_shifts` exists (declared above by migration 040's inlined
-- DDL). The column itself is declared in the `ht_checkins` CREATE
-- TABLE block earlier in this file. Guarded by `pg_constraint`
-- existence so re-running the init script is a no-op.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
         WHERE conname = 'fk_ht_checkins_round_bill_shift'
    ) THEN
        ALTER TABLE ht_checkins
            ADD CONSTRAINT fk_ht_checkins_round_bill_shift
            FOREIGN KEY (cin_round_bill_shift_id)
            REFERENCES ht_shifts(shift_id)
            ON DELETE SET NULL;
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS ix_ht_checkins_round_bill_shift_id
    ON ht_checkins (cin_round_bill_shift_id)
    WHERE cin_round_bill_shift_id IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('048', '048_round_bill_shift_id.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 046: ht_roles + ht_permissions + junctions (Track G7 / T4 HIGH-9)
-- iHOTEL has Admin / Cashier / Housekeeper / Receptionist roles with
-- per-feature gating. Migration 046 introduces canonical roles +
-- permission grid so Track G features (refunds, room change, round-bill,
-- inventory consume, RR.4 reports) can be gated per-role. Mirrors
-- migrations/pg/046_user_roles_and_permissions.sql one-for-one.
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_roles (
    role_id        BIGSERIAL    PRIMARY KEY,
    role_key       VARCHAR(64)  NOT NULL UNIQUE,
    display_name   VARCHAR(128) NOT NULL,
    created_at     TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ht_permissions (
    permission_id   BIGSERIAL    PRIMARY KEY,
    permission_key  VARCHAR(128) NOT NULL UNIQUE,
    description     VARCHAR(255),
    created_at      TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ht_role_permissions (
    role_id       BIGINT NOT NULL REFERENCES ht_roles(role_id)       ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES ht_permissions(permission_id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS ht_user_roles (
    user_id    BIGINT NOT NULL REFERENCES ht_users(user_id) ON DELETE CASCADE,
    role_id    BIGINT NOT NULL REFERENCES ht_roles(role_id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX IF NOT EXISTS ht_user_roles_role_idx ON ht_user_roles(role_id);

-- Seed: roles, permissions, role→permission grid.
INSERT INTO ht_roles (role_key, display_name) VALUES
    ('admin',         'Administrator'),
    ('cashier',       'Cashier'),
    ('housekeeper',   'Housekeeper'),
    ('receptionist',  'Receptionist')
ON CONFLICT (role_key) DO NOTHING;

INSERT INTO ht_permissions (permission_key, description) VALUES
    ('payment.refund',       'Refund (negative payment) against a recorded receipt (Track G2)'),
    ('checkin.room_change',  'Move an active check-in to a different room (Track G4)'),
    ('checkin.round_bill',   'Open/close a cashier round-bill (cash-drawer reconciliation; Track G9)'),
    ('inventory.consume',    'Consume / replenish stock against a room or shift (Track F3)'),
    ('reports.rr4',          'Generate RR.4 daily revenue export (Track G8)'),
    ('admin.users',          'Manage users / roles via /api/admin/users (Phase 4 PR4)')
ON CONFLICT (permission_key) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id FROM ht_roles r CROSS JOIN ht_permissions p
WHERE r.role_key = 'admin'
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN ('payment.refund','checkin.round_bill','reports.rr4')
WHERE r.role_key = 'cashier'
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN ('inventory.consume','checkin.room_change')
WHERE r.role_key = 'housekeeper'
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN ('checkin.room_change','inventory.consume')
WHERE r.role_key = 'receptionist'
ON CONFLICT (role_id, permission_id) DO NOTHING;

-- Seed test accounts (temp_password_2026, Argon2id PHC strings matching
-- the runtime hash params — see migrations/pg/046_*.sql for rationale).
INSERT INTO ht_users (username, password_hash, role, display_name, active) VALUES
    ('housekeeper_test',
     '$argon2id$v=19$m=19456,t=2,p=1$MPd9QPLV/Hk/ZCw93x7bjg$4jEMySdhT0/AdK+Ww4qWO1cUZfNuoDDhoFVH7C1HShw',
     'housekeeper', 'Test Housekeeper', TRUE),
    ('cashier_test',
     '$argon2id$v=19$m=19456,t=2,p=1$zbFz6agSu3CFK+DWG1yVPA$WEp9hv+/gDOL+NmC4RvO88ptPtJKyD+o6/Hk60nH94I',
     'cashier', 'Test Cashier', TRUE),
    ('receptionist_test',
     '$argon2id$v=19$m=19456,t=2,p=1$6agusFrTSWNC4dxANU2qLA$RDcPxK1jBnQWINIHt7eoMkmZn3BHkzDQkGCU4sDYDS0',
     'receptionist', 'Test Receptionist', TRUE)
ON CONFLICT (username) DO NOTHING;

-- Wire seed accounts to their primary roles + backfill any pre-existing
-- users into the junction so legacy admin accounts stay functional.
INSERT INTO ht_user_roles (user_id, role_id)
SELECT u.user_id, r.role_id FROM ht_users u
JOIN ht_roles r ON r.role_key = u.role
WHERE u.role IN ('admin', 'cashier', 'housekeeper', 'receptionist')
ON CONFLICT (user_id, role_id) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('046', '046_user_roles_and_permissions.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 047 — Track G8 (RR.4 Thai immigration export) audit trail.
-- One row per export attempt. `file_hash` is hex SHA-256 of the emitted
-- bytes so the regulator's re-requests can be answered with byte-exact
-- fidelity. See `migrations/pg/047_rr4_export_log.sql` for the full
-- rationale.
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_rr4_exports (
    id           SERIAL PRIMARY KEY,
    site         VARCHAR(50)  NOT NULL,
    range_from   DATE         NOT NULL,
    range_to     DATE         NOT NULL,
    format       VARCHAR(10)  NOT NULL,
    row_count    INTEGER      NOT NULL DEFAULT 0,
    exported_by  VARCHAR(100) NOT NULL DEFAULT 'system',
    exported_at  TIMESTAMP    NOT NULL DEFAULT NOW(),
    file_hash    VARCHAR(64)  NOT NULL,

    CONSTRAINT ck_ht_rr4_exports_format
        CHECK (format IN ('csv', 'xlsx')),
    CONSTRAINT ck_ht_rr4_exports_range
        CHECK (range_to >= range_from)
);

CREATE INDEX IF NOT EXISTS ix_ht_rr4_exports_site_range
    ON ht_rr4_exports (site, range_from, range_to);
CREATE INDEX IF NOT EXISTS ix_ht_rr4_exports_exported_at
    ON ht_rr4_exports (exported_at DESC);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('047', '047_rr4_export_log.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 050 — Resilience PR R3. Per-table Change Tracking watermark
-- (`legacy_ct_state_per_table`) so a wedge on one hot legacy MSSQL table
-- (canonical: HT_Book_H row lock, observed 2026-05-14) freezes only that
-- row instead of gating every CT-enabled table. Sibling to the existing
-- single-row `legacy_ct_state`; only consulted when the watcher runs
-- with `SYNC_PER_TABLE_WATERMARK=true`. Full rationale in
-- `migrations/pg/050_legacy_ct_state_per_table.sql`.
-- =============================================================================

CREATE TABLE IF NOT EXISTS legacy_ct_state_per_table (
    table_name        TEXT         PRIMARY KEY,
    last_seen_version BIGINT       NOT NULL DEFAULT 0,
    last_polled_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    last_error        TEXT,
    last_error_at     TIMESTAMPTZ
);

INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version)
SELECT t.name, COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0)
FROM (VALUES
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H'),
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds'),
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel')
) AS t(name)
ON CONFLICT (table_name) DO NOTHING;

CREATE INDEX IF NOT EXISTS ix_legacy_ct_state_per_table_polled_at
    ON legacy_ct_state_per_table (last_polled_at);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('050', '050_legacy_ct_state_per_table.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 051: Track G5 — ht_coupons canonical (mirror of HT_Cupon)
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_coupons (
    coupon_id              BIGSERIAL    PRIMARY KEY,
    legacy_cupon_no        INTEGER,
    coupon_code            VARCHAR(20)  NOT NULL UNIQUE,
    coupon_value           NUMERIC(10,2) NOT NULL DEFAULT 0,
    coupon_status          VARCHAR(20)  NOT NULL DEFAULT 'issued',
    coupon_issued_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    coupon_expires_at      DATE,
    coupon_issued_by       TEXT,
    coupon_for_cin_no      VARCHAR(50),
    coupon_for_cust_id     BIGINT       REFERENCES ht_customers(cust_id) ON DELETE SET NULL,
    coupon_redeemed_at     TIMESTAMPTZ,
    coupon_redeemed_cin_id BIGINT       REFERENCES ht_checkins(cin_id)   ON DELETE SET NULL,
    aggregate_id           UUID         NOT NULL UNIQUE,
    source                 VARCHAR(20)  NOT NULL DEFAULT 'canonical',
    created_at             TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at             TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ht_coupons_status_check
        CHECK (coupon_status IN ('issued', 'redeemed', 'expired', 'cancelled')),
    CONSTRAINT ht_coupons_source_check
        CHECK (source IN ('canonical', 'legacy')),
    CONSTRAINT ht_coupons_value_nonneg
        CHECK (coupon_value >= 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_ht_coupons_legacy_cupon_no
    ON ht_coupons (legacy_cupon_no)
    WHERE legacy_cupon_no IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_ht_coupons_status
    ON ht_coupons (coupon_status);

CREATE INDEX IF NOT EXISTS ix_ht_coupons_for_cust_id
    ON ht_coupons (coupon_for_cust_id)
    WHERE coupon_for_cust_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS ix_ht_coupons_for_cin_no
    ON ht_coupons (coupon_for_cin_no)
    WHERE coupon_for_cin_no IS NOT NULL;

-- Track G5 permission seeds (mirror migration 051).
INSERT INTO ht_permissions (permission_key, description)
VALUES
    ('coupon.issue',  'Issue a food/promo coupon to a guest (Track G5)'),
    ('coupon.redeem', 'Mark a coupon as redeemed / printed (Track G5)')
ON CONFLICT (permission_key) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
CROSS JOIN ht_permissions p
WHERE r.role_key = 'admin'
  AND p.permission_key IN ('coupon.issue', 'coupon.redeem')
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key IN ('coupon.issue', 'coupon.redeem')
WHERE r.role_key = 'receptionist'
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key = 'coupon.redeem'
WHERE r.role_key = 'cashier'
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('051', '051_create_ht_coupons.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 052: Track G6 — ht_pos_sales canonical (mirror of HT_CheckIn_Product)
-- =============================================================================

CREATE TABLE IF NOT EXISTS ht_pos_sales (
    sale_id          BIGSERIAL    PRIMARY KEY,
    sale_cin_id      INTEGER      NOT NULL REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    sale_product_id  BIGINT       NOT NULL REFERENCES ht_products(prod_id),
    sale_qty         NUMERIC(10, 3) NOT NULL CHECK (sale_qty > 0),
    sale_unit_price  NUMERIC(12, 2) NOT NULL CHECK (sale_unit_price >= 0),
    -- Total is materialized inside the row: row is durable evidence of
    -- the sale at the moment it was posted (qty × unit_price), even if
    -- the catalog price drifts later.
    sale_total       NUMERIC(14, 2) GENERATED ALWAYS AS (sale_qty * sale_unit_price) STORED,
    sale_sold_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    sale_sold_by     VARCHAR(64),
    sale_note        VARCHAR(500),
    sale_status      VARCHAR(20)  NOT NULL DEFAULT 'posted',
    sale_legacy_id   INTEGER,
    source           VARCHAR(20)  NOT NULL DEFAULT 'canonical',
    aggregate_id     UUID         NOT NULL,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_pos_sales_status_check
        CHECK (sale_status IN ('posted', 'voided')),
    CONSTRAINT ht_pos_sales_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_pos_sales_cin_id_idx
    ON ht_pos_sales (sale_cin_id);

CREATE INDEX IF NOT EXISTS ht_pos_sales_product_id_idx
    ON ht_pos_sales (sale_product_id);

CREATE INDEX IF NOT EXISTS ht_pos_sales_sold_at_idx
    ON ht_pos_sales (sale_sold_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_sales_legacy_id_uq
    ON ht_pos_sales (sale_legacy_id) WHERE sale_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_sales_aggregate_id_uq
    ON ht_pos_sales (aggregate_id);

-- ht_booking_products — pre-ordered products attached to a booking (task #52 /
-- migration 061). Canonical analog of legacy HT_Book_Pro (mirrored read-only
-- in migration 056). Keyed on the booking instead of the check-in folio
-- because a pre-order is taken before check-in. Legacy write-back deferred —
-- HT_Book_Pro INSERT shape is unverified, so this is canonical-only for now.
CREATE TABLE IF NOT EXISTS ht_booking_products (
    bp_id           BIGSERIAL    PRIMARY KEY,
    bp_book_id      INTEGER      NOT NULL REFERENCES ht_bookings(book_id) ON DELETE CASCADE,
    bp_product_id   BIGINT       NOT NULL REFERENCES ht_products(prod_id),
    bp_qty          NUMERIC(10, 3) NOT NULL CHECK (bp_qty > 0),
    bp_unit_price   NUMERIC(12, 2) NOT NULL CHECK (bp_unit_price >= 0),
    bp_total        NUMERIC(14, 2) GENERATED ALWAYS AS (bp_qty * bp_unit_price) STORED,
    bp_note         VARCHAR(500),
    bp_legacy_id    INTEGER,
    source          VARCHAR(20)  NOT NULL DEFAULT 'canonical',
    aggregate_id    UUID,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_booking_products_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_booking_products_book_id_idx
    ON ht_booking_products (bp_book_id);

CREATE INDEX IF NOT EXISTS ht_booking_products_product_id_idx
    ON ht_booking_products (bp_product_id);

CREATE UNIQUE INDEX IF NOT EXISTS ht_booking_products_legacy_id_uq
    ON ht_booking_products (bp_legacy_id) WHERE bp_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_booking_products_aggregate_id_uq
    ON ht_booking_products (aggregate_id) WHERE aggregate_id IS NOT NULL;

-- Track G6 permission seed (mirror migration 052).
INSERT INTO ht_permissions (permission_key, description)
VALUES (
    'pos.sell',
    'Ring up a POS sale and charge it to a check-in folio (Track G6)'
)
ON CONFLICT (permission_key) DO NOTHING;

INSERT INTO ht_role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM ht_roles r
JOIN ht_permissions p ON p.permission_key = 'pos.sell'
WHERE r.role_key IN ('admin', 'cashier')
ON CONFLICT (role_id, permission_id) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('052', '052_create_ht_pos_sales.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 053: persistent cooldown for the level-triggered reconcile-drift
-- digest. Replaces the process-local Mutex<HashMap> in scheduler::sync that
-- was zeroed on every backend restart (refire incident 2026-05-16).
CREATE TABLE IF NOT EXISTS ht_level_drift_alert_cooldowns (
    site_id          TEXT        NOT NULL,
    table_name       TEXT        NOT NULL,
    last_alerted_at  TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (site_id, table_name)
);

CREATE OR REPLACE FUNCTION ht_level_drift_alert_cooldowns_touch_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_ht_level_drift_alert_cooldowns_touch_updated_at
    ON ht_level_drift_alert_cooldowns;

CREATE TRIGGER trg_ht_level_drift_alert_cooldowns_touch_updated_at
    BEFORE UPDATE ON ht_level_drift_alert_cooldowns
    FOR EACH ROW
    EXECUTE FUNCTION ht_level_drift_alert_cooldowns_touch_updated_at();

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('053', '053_level_drift_alert_cooldowns.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 056: legacy_mirror.ht_book_pro (Phase 5/E2)
-- Mirrors migrations/pg/056_legacy_mirror_ht_book_pro.sql. Opaque
-- pass-through mirror of `HT_Book_Pro` (pre-booked products attached to
-- a booking by FrmAddBook2) — coexistence audit 2026-06-11 P2 gap; the
-- new `BookProMirrorMapper` (sync/mappers/mirror.rs) populates it from
-- CT once migrations/legacy-mssql/023 enables tracking. Also seeds the
-- watcher's `legacy_sync_status` row and the per-table watermark from
-- the current global watermark (0 on a fresh install).
-- =============================================================================

CREATE TABLE IF NOT EXISTS legacy_mirror.ht_book_pro (
    id            INTEGER          PRIMARY KEY,
    b_no          TEXT,
    b_room        TEXT,
    b_name        TEXT,
    b_unit        TEXT,
    b_num         DOUBLE PRECISION,
    b_price       DOUBLE PRECISION,
    b_price_total DOUBLE PRECISION,
    b_pro_id      INTEGER,
    mirrored_at   TIMESTAMPTZ      NOT NULL DEFAULT now(),
    mirror_source TEXT             NOT NULL
);
CREATE INDEX IF NOT EXISTS ht_book_pro_b_no_idx
    ON legacy_mirror.ht_book_pro (b_no);

INSERT INTO legacy_sync_status (table_name)
VALUES ('HT_Book_Pro')
ON CONFLICT (table_name) DO NOTHING;

INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version)
SELECT 'HT_Book_Pro',
       COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0)
ON CONFLICT (table_name) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('056', '056_legacy_mirror_ht_book_pro.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 057 — Track J7a. Canonical per-line mirror of legacy
-- `HT_CheckIn_Pay` (all tender splits + category + status) so the
-- round-close income-by-tender reconciliation + the iHOTEL-equivalent
-- shift report compute from canonical PG. Populated by the sync worker's
-- PaymentMapper coalesced path. Per-site (connection-level scoping, like
-- ht_shifts). See migrations/pg/057_create_ht_payment_ledger.sql.
CREATE TABLE IF NOT EXISTS ht_payment_ledger (
    ledger_id        BIGSERIAL PRIMARY KEY,
    ledger_legacy_id INTEGER     NOT NULL,
    ledger_pay_no    VARCHAR(50),
    ledger_cin_no    VARCHAR(50),
    ledger_cust_no   VARCHAR(50),
    ledger_ds_label  VARCHAR(500),
    ledger_ds_name   VARCHAR(500),
    ledger_ds_id     VARCHAR(50),
    ledger_ds_num    NUMERIC(12,2),
    ledger_cash      NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_credit    NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_free      NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_tran      NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_web       NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_amount    NUMERIC(14,2) NOT NULL DEFAULT 0,
    ledger_status    VARCHAR(50)   NOT NULL DEFAULT '1',
    ledger_branch    VARCHAR(50),
    ledger_pay_by    VARCHAR(100),
    ledger_note      VARCHAR(500),
    ledger_pay_date  TIMESTAMPTZ,
    ledger_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_payment_ledger_legacy_id_key UNIQUE (ledger_legacy_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_payment_ledger_pay_date
    ON ht_payment_ledger (ledger_pay_date);
CREATE INDEX IF NOT EXISTS ix_ht_payment_ledger_cin_no
    ON ht_payment_ledger (ledger_cin_no);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('057', '057_create_ht_payment_ledger.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 058 — Track J7c. Cash-drawer reconciliation columns on ht_shifts
-- (counted cash + raw denomination map), inlined into the ht_shifts CREATE
-- above for fresh DBs. See migrations/pg/058_add_shift_cash_count.sql.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('058', '058_add_shift_cash_count.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 059 — Cash in/out petty-cash ledger (รายรับ-รายจ่าย). Canonical
-- mirror of legacy TB_Pay_History + its 3-level account taxonomy
-- (TB_SET_MyType2 / _2_2 / 3). Populated by the sync worker's sync_cash_history
-- + sync_cash_categories per-tick polls and by POST /api/cash/{income,expense}.
-- Per-site (connection-level scoping). See migrations/pg/059_create_ht_cash_ledger.sql.
CREATE TABLE IF NOT EXISTS ht_cash_categories (
    cash_cat_id   BIGSERIAL PRIMARY KEY,
    cat_level     VARCHAR(8)   NOT NULL,
    cat_legacy_id INTEGER      NOT NULL,
    cat_id_full   VARCHAR(50),
    cat_name      VARCHAR(100),
    cat_synced_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_cash_categories_level_legacy_id_key UNIQUE (cat_level, cat_legacy_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_cash_categories_level
    ON ht_cash_categories (cat_level);

CREATE TABLE IF NOT EXISTS ht_cash_ledger (
    cash_id          BIGSERIAL PRIMARY KEY,
    cash_legacy_id   INTEGER,
    cash_kind        VARCHAR(20)  NOT NULL DEFAULT 'unknown'
                     CHECK (cash_kind IN ('income', 'expense', 'unknown')),
    cash_legacy_type VARCHAR(50),
    cash_entry_date  TIMESTAMPTZ,
    cash_bill_no     VARCHAR(255),
    cash_payee       VARCHAR(500),
    cash_amount      NUMERIC(14,2) NOT NULL DEFAULT 0,
    cash_note        VARCHAR(500),
    cash_program_date TIMESTAMPTZ,
    cash_group       VARCHAR(50),
    cash_account     VARCHAR(50),
    cash_source      VARCHAR(20)  NOT NULL DEFAULT 'legacy'
                     CHECK (cash_source IN ('legacy', 'app')),
    cash_created_by  VARCHAR(100),
    cash_synced_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    -- Writeback correlation id (v4, minted at INSERT for app rows). Added by
    -- migration 085 (issue #202) — inlined here for fresh installs.
    aggregate_id     UUID,
    CONSTRAINT ht_cash_ledger_legacy_id_key UNIQUE (cash_legacy_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_cash_ledger_entry_date
    ON ht_cash_ledger (cash_entry_date);
CREATE UNIQUE INDEX IF NOT EXISTS ix_ht_cash_ledger_aggregate_id
    ON ht_cash_ledger (aggregate_id) WHERE aggregate_id IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('059', '059_create_ht_cash_ledger.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('085', '085_ht_cash_ledger_aggregate_id.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('061', '061_create_ht_booking_products.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- ht_notes — room & staff sticky notes (task #47 / migration 062). Canonical
-- mirror of legacy HT_Room_SMS + HT_EMP_SMS collapsed via note_target_kind.
-- Per-site (connection-level scoping). See migrations/pg/062_create_ht_notes.sql.
-- -----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ht_notes (
    note_id          BIGSERIAL    PRIMARY KEY,
    note_target_kind VARCHAR(10)  NOT NULL
                     CHECK (note_target_kind IN ('room', 'staff')),
    note_target_key  VARCHAR(50)  NOT NULL,
    note_body        TEXT         NOT NULL,
    note_created_by  VARCHAR(250),
    note_is_read     BOOLEAN      NOT NULL DEFAULT FALSE,
    note_legacy_id   INTEGER,
    note_source      VARCHAR(20)  NOT NULL DEFAULT 'legacy'
                     CHECK (note_source IN ('legacy', 'app')),
    aggregate_id     UUID,
    note_created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    note_updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    note_synced_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_notes_kind_legacy_id_key UNIQUE (note_target_kind, note_legacy_id)
);

CREATE INDEX IF NOT EXISTS ix_ht_notes_target
    ON ht_notes (note_target_kind, note_target_key);

CREATE INDEX IF NOT EXISTS ix_ht_notes_unread
    ON ht_notes (note_target_kind, note_target_key)
    WHERE note_is_read = FALSE;

CREATE UNIQUE INDEX IF NOT EXISTS ix_ht_notes_aggregate_id
    ON ht_notes (aggregate_id) WHERE aggregate_id IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('062', '062_create_ht_notes.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 064 — task #53. Booking-reminder state columns on ht_bookings
-- (book_notify_day / book_notify_note / book_notify_dismissed_at) created inline
-- in the ht_bookings CREATE TABLE above for fresh DBs, plus the
-- ix_ht_bookings_active_reminders partial index. PG-canonical only (no legacy
-- writeback). See migrations/pg/064_booking_reminders.sql.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('064', '064_booking_reminders.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 065: Task #45 — POS walk-up (roomless) sale + standalone receipt.
-- Canonical mirror of legacy HT_Receipt_H (header) + HT_Receipt_Ds (lines).
CREATE TABLE IF NOT EXISTS ht_pos_receipts (
    receipt_id            BIGSERIAL    PRIMARY KEY,
    receipt_customer_no   VARCHAR(50)  NOT NULL DEFAULT 'C0000',
    receipt_customer_name VARCHAR(500) NOT NULL DEFAULT '',
    receipt_customer_addr VARCHAR(500) NOT NULL DEFAULT '',
    receipt_customer_tel  VARCHAR(50)  NOT NULL DEFAULT '',
    receipt_tax_id        VARCHAR(50)  NOT NULL DEFAULT '',
    receipt_subtotal      NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_discount      NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_total         NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_before_vat    NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_vat           NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_vat_percent   INTEGER        NOT NULL DEFAULT 0,
    receipt_paid          NUMERIC(14, 2) NOT NULL DEFAULT 0,
    receipt_payment_method VARCHAR(20)   NOT NULL DEFAULT 'cash',
    receipt_note          VARCHAR(500)   NOT NULL DEFAULT '',
    receipt_status        VARCHAR(20)    NOT NULL DEFAULT 'posted',
    receipt_sold_by       VARCHAR(64),
    receipt_sold_at       TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    receipt_legacy_id     INTEGER,
    receipt_legacy_no     VARCHAR(50),
    source                VARCHAR(20)    NOT NULL DEFAULT 'canonical',
    aggregate_id          UUID           NOT NULL,
    created_at            TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    CONSTRAINT ht_pos_receipts_status_check
        CHECK (receipt_status IN ('posted', 'voided')),
    CONSTRAINT ht_pos_receipts_source_check
        CHECK (source IN ('canonical', 'legacy'))
);

CREATE INDEX IF NOT EXISTS ht_pos_receipts_sold_at_idx
    ON ht_pos_receipts (receipt_sold_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_receipts_legacy_id_uq
    ON ht_pos_receipts (receipt_legacy_id) WHERE receipt_legacy_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS ht_pos_receipts_aggregate_id_uq
    ON ht_pos_receipts (aggregate_id);

CREATE TABLE IF NOT EXISTS ht_pos_receipt_lines (
    line_id          BIGSERIAL    PRIMARY KEY,
    line_receipt_id  BIGINT       NOT NULL REFERENCES ht_pos_receipts(receipt_id) ON DELETE CASCADE,
    line_product_id  BIGINT       REFERENCES ht_products(prod_id),
    line_product_no  VARCHAR(50)  NOT NULL DEFAULT '',
    line_product_name VARCHAR(255) NOT NULL DEFAULT '',
    line_unit_name   VARCHAR(50)  NOT NULL DEFAULT '',
    line_qty         NUMERIC(10, 3) NOT NULL CHECK (line_qty > 0),
    line_unit_price  NUMERIC(12, 2) NOT NULL CHECK (line_unit_price >= 0),
    line_discount    NUMERIC(12, 2) NOT NULL DEFAULT 0,
    line_total       NUMERIC(14, 2) NOT NULL DEFAULT 0,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ht_pos_receipt_lines_receipt_id_idx
    ON ht_pos_receipt_lines (line_receipt_id);

CREATE INDEX IF NOT EXISTS ht_pos_receipt_lines_product_id_idx
    ON ht_pos_receipt_lines (line_product_id);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('065', '065_create_ht_pos_receipts.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 066: Task #58 — in-app reception verification form. Online
-- equivalent of docs/coexistence/reception-verification-TH.html; answers land in
-- the vr_answers JSONB column keyed by question id. PG-CANONICAL ONLY (no legacy
-- counterpart, no sync, no writeback). Per-site (connection-level scoping).
-- See migrations/pg/066_create_ht_verification_responses.sql.
CREATE TABLE IF NOT EXISTS ht_verification_responses (
    vr_id           BIGSERIAL    PRIMARY KEY,
    vr_submitted_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    vr_site         TEXT,
    vr_inspector    TEXT,
    vr_answers      JSONB        NOT NULL,
    vr_overall      TEXT,
    created_at      TIMESTAMPTZ  DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ix_ht_verification_responses_submitted_at
    ON ht_verification_responses (vr_submitted_at DESC);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('066', '066_create_ht_verification_responses.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- ht_feedback_forms — data-driven reception feedback / re-verification form
-- definitions (Tier 1). form_schema JSONB drives a generic renderer so question
-- edits are a DB write, not a frontend rebuild. PG-CANONICAL ONLY (no legacy, no
-- sync, no writeback). Answers land in ht_verification_responses.vr_answers.
-- See migrations/pg/067_create_ht_feedback_forms.sql.
CREATE TABLE IF NOT EXISTS ht_feedback_forms (
    form_id      BIGSERIAL   PRIMARY KEY,
    form_key     TEXT        NOT NULL UNIQUE,
    form_site    TEXT,
    form_kind    TEXT        NOT NULL DEFAULT 'reverify',
    form_title   TEXT        NOT NULL,
    form_intro   TEXT,
    form_schema  JSONB       NOT NULL,
    form_active  BOOLEAN     NOT NULL DEFAULT TRUE,
    form_sort    INTEGER     NOT NULL DEFAULT 0,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ix_ht_feedback_forms_active
    ON ht_feedback_forms (form_active, form_sort);

-- Form CONTENT below is kept in sync with the canonical source-of-truth,
-- scripts/seed-feedback-forms.sql. Fresh deploys seed from here (init runs once);
-- existing DBs get content edits by re-running that seed script (a DB write, no
-- app rebuild). If you edit a question, update BOTH this block and the seed file.
-- These statements are byte-identical to the seed script (ON CONFLICT DO UPDATE).
INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_active, form_sort)
VALUES ('reverify_hfhotel', 'hfhotel', 'reverify', 'ตรวจสอบซ้ำ — HF Hotel', 'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)', '{"questions": [{"id": "rv_invoice", "type": "radio", "label": "บิลห้องพักหลายห้องในใบเดียว — เปิดบิล INV2606-019832 ในระบบใหม่ แล้วดูว่า: แสดงแยกบรรทัดต่อห้อง (2 ห้อง ห้องละ 1,780) และยอดรวมทั้งบิล = 3,560 ซึ่งต้องเท่ากับยอดรวมบิล iHOTEL ของลูกค้ารายนี้ทั้ง 2 ใบ ➜ ตรงกันหรือไม่?", "options": [{"label": "ตรง", "value": "match"}, {"label": "ไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_invoice_note", "type": "text", "label": "ถ้าไม่ตรง ระบุยอดที่เห็นในระบบใหม่", "showIf": {"field": "rv_invoice", "equals": "mismatch"}, "placeholder": "เช่น เห็นเป็น …"}, {"id": "rv_round_summary", "type": "radio", "label": "เปิดเมนู รายงาน ➜ หน้า สรุปรอบบิล ในระบบใหม่ → ดูช่อง รวมเงินรับ (สีแดง ไม่รวมเงินทอนตั้งต้น) ของรอบล่าสุด แล้วเทียบกับยอด รวมเงินรับ ในรายงานรายรับของ iHOTEL รอบเดียวกัน ➜ ตรงกันหรือไม่? (อย่าดูช่อง รวมทั้งหมด เพราะรวมเงินทอนตั้งต้นไว้ด้วย)", "options": [{"label": "ตรงแล้ว", "value": "match"}, {"label": "ยังไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_round_summary_note", "type": "text", "label": "ถ้ายังไม่ตรง ระบุรอบ + ยอดที่เห็น", "showIf": {"field": "rv_round_summary", "equals": "mismatch"}, "placeholder": "เช่น รอบ … ยอดในระบบใหม่ … / iHOTEL …"}]}'::jsonb, FALSE, 10)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_active = EXCLUDED.form_active,
  form_sort = EXCLUDED.form_sort, updated_at = now();

INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_active, form_sort)
VALUES ('reverify_hfville', 'hfville', 'reverify', 'ตรวจสอบซ้ำ — HF Ville', 'ทีมไอทีแก้จุดที่แจ้งมาแล้ว รบกวนช่วยเปิดดูซ้ำแล้วเลือกคำตอบครับ/ค่ะ (แค่เปิดดู ไม่กระทบข้อมูลจริง)', '{"questions": [{"id": "rv_round816", "type": "radio", "label": "รายงานรอบบิล รอบ 816 (กะบ่าย 27/06) ยอดรวม = 14,280 หรือไม่?", "options": [{"label": "ตรง", "value": "match"}, {"label": "ไม่ตรง", "value": "mismatch"}], "required": true}, {"id": "rv_round816_note", "type": "text", "label": "ระบุยอดที่เห็น", "showIf": {"field": "rv_round816", "equals": "mismatch"}, "placeholder": "เช่น เห็นเป็น …"}, {"id": "rv_room114", "type": "radio", "label": "สถานะห้อง 114 ขึ้นว่า ''ว่าง'' แล้วหรือไม่?", "options": [{"label": "ว่างแล้ว", "value": "vacant"}, {"label": "ยังมีคนพัก", "value": "occupied"}], "required": true}, {"id": "rv_arrivals", "type": "radio", "label": "คำว่า ''เข้า'' ในรายชื่อผู้เข้าพัก (ที่เคยแจ้งว่าไม่ตรง) หมายถึงข้อใด?", "options": [{"label": "ลูกค้าที่เช็คอินเข้าจริงวันนี้", "value": "a"}, {"label": "ลูกค้าที่จองไว้และยังรอเช็คอินวันนี้ (ยังไม่เข้า)", "value": "b"}], "required": true}, {"id": "rv_arrivals_screen", "type": "text", "label": "ดูจากหน้าจอไหนของ iHOTEL", "placeholder": "ชื่อหน้าจอ / เมนู"}]}'::jsonb, FALSE, 20)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_active = EXCLUDED.form_active,
  form_sort = EXCLUDED.form_sort, updated_at = now();

-- New: legacy write-back test form (writeback_test). Archives above; this one is
-- active. Reuses form_kind='reverify' so submissions ride the existing results
-- storage/hub. Applies to BOTH sites (form_site='all'). See migration 073.
INSERT INTO ht_feedback_forms (form_key, form_site, form_kind, form_title, form_intro, form_schema, form_active, form_sort)
VALUES ('writeback_test', 'all', 'reverify', 'ทดสอบเขียนข้อมูลกลับ iHOTEL · Legacy write-back test', 'การทดสอบนี้เปิดใช้งาน "การเขียนข้อมูลจากแอปใหม่กลับเข้า iHOTEL" ทีละสวิตช์ โดยประสานกับทีมไอที ทีละขั้น — ไอทีเป็นผู้เปิดสวิตช์ให้บนเซิร์ฟเวอร์ แล้วรบกวนพนักงานต้อนรับทำรายการทดสอบ 1 รายการ และตรวจใน iHOTEL ว่าข้อมูลไปถึงถูกต้อง และ iHOTEL ยังทำงานปกติ (ไม่ค้าง/ไม่ error) ⚠️ ทำเฉพาะช่วงที่ไม่มีลูกค้าหน้าเคาน์เตอร์ และหยุดทันทีถ้า iHOTEL ผิดปกติ (แจ้งไอทีให้ปิดสวิตช์กลับ)', '{"questions": [{"id": "wb_site", "type": "radio", "label": "1) กำลังทดสอบสาขาใด?", "required": true, "options": [{"label": "HF Hotel", "value": "hfhotel"}, {"label": "HF Ville", "value": "hfville"}]}, {"id": "wb_img_result", "type": "radio", "label": "2) รูปบัตร — หลังไอทีแจ้งว่าเปิดสวิตช์ ''บันทึกรูปบัตรกลับ iHOTEL'' แล้ว: ทำเช็คอินทดสอบในแอปใหม่ 1 ราย แล้วสแกนบัตรประชาชน/พาสปอร์ต → เปิด iHOTEL ดูใบลงทะเบียน/รูปบัตรของรายนั้น → รูปบัตรปรากฏใน iHOTEL หรือไม่?", "required": true, "options": [{"label": "ปรากฏถูกต้อง", "value": "match"}, {"label": "ไม่ปรากฏ", "value": "missing"}, {"label": "iHOTEL แจ้ง error / บันทึกไม่ได้", "value": "error"}]}, {"id": "wb_img_note", "type": "text", "label": "→ ระบุสิ่งที่เห็น / ข้อความ error (ถ้าไม่ปรากฏหรือ error)", "placeholder": "เช่น เห็นเป็น… / ข้อความ…", "showIf": {"field": "wb_img_result", "equals": "missing"}}, {"id": "wb_img_error_note", "type": "text", "label": "→ ระบุข้อความ error", "placeholder": "ข้อความที่ iHOTEL แจ้ง…", "showIf": {"field": "wb_img_result", "equals": "error"}}, {"id": "wb_img_ihotel_ok", "type": "radio", "label": "3) หลังทดสอบข้อ 2 — iHOTEL ยังทำงานปกติหรือไม่ (บันทึก/เปิดบิลได้ ไม่ค้าง ไม่มี error)?", "required": true, "options": [{"label": "ปกติ", "value": "ok"}, {"label": "มีปัญหา (แจ้งไอทีปิดสวิตช์)", "value": "problem"}]}, {"id": "wb_comp_result", "type": "radio", "label": "4) ผู้ติดตาม — หลังไอทีแจ้งว่าเปิดสวิตช์ ''บันทึกผู้ติดตามกลับ iHOTEL'' แล้ว: เพิ่มผู้ติดตาม 1 คนในแอปใหม่ให้ผู้เข้าพักที่มีในระบบ → เปิด iHOTEL ดูรายชื่อผู้เข้าพักร่วมของรายนั้น → ปรากฏหรือไม่?", "required": true, "options": [{"label": "ปรากฏถูกต้อง", "value": "match"}, {"label": "ไม่ปรากฏ", "value": "missing"}, {"label": "iHOTEL แจ้ง error", "value": "error"}]}, {"id": "wb_comp_note", "type": "text", "label": "→ ระบุสิ่งที่เห็น / ข้อความ (ถ้าไม่ปรากฏหรือ error)", "placeholder": "เช่น เห็นเป็น… / ข้อความ…", "showIf": {"field": "wb_comp_result", "equals": "missing"}}, {"id": "wb_comp_error_note", "type": "text", "label": "→ ระบุข้อความ error", "placeholder": "ข้อความที่ iHOTEL แจ้ง…", "showIf": {"field": "wb_comp_result", "equals": "error"}}, {"id": "wb_comp_ihotel_ok", "type": "radio", "label": "5) หลังทดสอบข้อ 4 — iHOTEL ยังทำงานปกติหรือไม่?", "required": true, "options": [{"label": "ปกติ", "value": "ok"}, {"label": "มีปัญหา (แจ้งไอทีปิดสวิตช์)", "value": "problem"}]}, {"id": "wb_overall", "type": "radio", "label": "6) สรุปผลการทดสอบ", "required": true, "options": [{"label": "ผ่านทั้งสองส่วน — พร้อมเปิดใช้ถาวร", "value": "pass"}, {"label": "มีปัญหา — ให้ไอทีปิดสวิตช์กลับก่อน", "value": "fail"}]}, {"id": "wb_notes", "type": "text", "label": "7) หมายเหตุเพิ่มเติม (ถ้ามี)", "placeholder": "รายละเอียดอื่น ๆ / เวลาที่ทดสอบ / ชื่อผู้ประสานไอที"}]}'::jsonb, TRUE, 5)
ON CONFLICT (form_key) DO UPDATE SET
  form_site = EXCLUDED.form_site, form_kind = EXCLUDED.form_kind,
  form_title = EXCLUDED.form_title, form_intro = EXCLUDED.form_intro,
  form_schema = EXCLUDED.form_schema, form_active = EXCLUDED.form_active,
  form_sort = EXCLUDED.form_sort, updated_at = now();

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('067', '067_create_ht_feedback_forms.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 068 — ht_payments per-check-in legacy-receipt dedup hardening
-- (issue #203). The partial UNIQUE index ux_ht_payments_cin_legacy_receipt_no
-- is inlined into the ht_payments index block above; this seed row records the
-- migration as already applied so the drift check sees zero pending migrations.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('068', '068_ht_payments_legacy_receipt_unique.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 069 — guest date of birth (cust_dob DATE) captured at check-in
-- registration. Inlined into the ht_customers CREATE TABLE above; this seed row
-- records the migration as applied so the drift check sees zero pending. PG-
-- canonical-only (legacy HT_Customers has no DOB column).
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('069', '069_ht_customers_dob.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 070 — ht_guest_documents (guest identity documents / photos captured
-- at check-in registration). Table + indexes inlined above; this seed row records
-- the migration as applied so the drift check sees zero pending. Legacy mirror to
-- Tb_Save_Image is SHIPPED DARK behind GUEST_DOCUMENT_STORAGE_ENABLED (default off).
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('070', '070_ht_guest_documents.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 071 — partial UNIQUE index ux_ht_guest_documents_legacy_id (UPSERT
-- conflict target for the legacy Tb_Save_Image sync-in poll). Index inlined above;
-- this seed row records the migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('071', '071_ht_guest_documents_legacy_unique.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 072 — ht_guest_doc_backfill_skip (convergence backstop for the
-- guest-image sync-in poll). Table inlined above; this seed row records the
-- migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('072', '072_ht_guest_doc_backfill_skip.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 073 — DATA migration on ht_feedback_forms: archive the completed
-- reverify_hfhotel / reverify_hfville forms (seeded above with form_active=FALSE)
-- and add the writeback_test form (seeded above, active). No schema change; this
-- seed row records the migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('073', '073_writeback_test_form.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 074 — ht_users.email (Cloudflare Access auto-login lookup key).
-- Column + partial functional UNIQUE index inlined into the ht_users block
-- above; the backfill UPDATEs are production-data-only (the usernames don't
-- exist in fresh seeds). This seed row records the migration as applied so
-- the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('074', '074_ht_users_email.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 075 — ht_users.badge (NFC staff-card login lookup key).
-- Column + partial UNIQUE index ux_ht_users_badge inlined into the ht_users
-- block above; there is no backfill (card accounts are auto-provisioned on
-- first tap, or linked via the set_user_card bin). This seed row records the
-- migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('075', '075_ht_users_badge.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 076 — ht_bookings OTA provenance / caller-idempotency natural key.
-- `book_ext_ref TEXT` column + partial UNIQUE index ux_ht_bookings_channel_ext_ref
-- inlined into the ht_bookings block above (`book_channel` already existed).
-- This seed row records the migration as applied so the drift check sees zero
-- pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('076', '076_ht_bookings_ota_provenance.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 077 — maid-facing housekeeping surface (/hk): ht_hk_cleaning_events
-- + ht_hk_broken_reports (tables + indexes inlined above, after the guest-doc
-- block). This seed row records the migration as applied so the drift check
-- sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('077', '077_create_ht_housekeeping_reports.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 087 — widen ht_hk_cleaning_events.hkev_status to accept 'dirty'
-- (maid-reported "ห้องยังไม่สะอาด" on /hk, gated by HK_MARK_DIRTY_ENABLED,
-- default off). The CHECK is inlined into the CREATE TABLE above, so a fresh
-- seed already has the widened constraint; this row records the migration as
-- applied so the drift check sees zero pending. The table stays PG-canonical
-- only — only the ht_rooms_new.room_clean FLAG crosses to legacy, via the
-- existing byte-pinned MarkRoomClean / MarkRoomDirty recipes.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('087', '087_hk_cleaning_events_dirty_status.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 088 — ht_hk_linen_reports, the maid's linen-shortage (ขาดผ้า)
-- report on /hk (table + index inlined above, after the ht_hk_broken_reports
-- block). RECORD-ONLY and PG-canonical only: no legacy counterpart, no sync, no
-- writeback, no domain event, no notification. This seed row records the
-- migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('088', '088_create_ht_hk_linen_reports.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 089 — ht_hk_room_signals, the canned reception<->maid room signals
-- of ADR 0008 (table + both indexes inlined above, after the
-- ht_hk_linen_reports block). PG-canonical only: no legacy counterpart, no sync
-- mapper, no writeback; the domain events it publishes are UI plumbing over the
-- existing SSE fan-out. This seed row records the migration as applied so the
-- drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('089', '089_create_ht_hk_room_signals.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 090 — make the linen-shortage report COMPLETABLE: the three
-- hklr_resolved_* columns and the partial open-backlog index are inlined into
-- the ht_hk_linen_reports block above. A report is OPEN until a maid marks the
-- room restocked (เติมผ้าแล้ว, POST /api/hk/rooms/{id}/linen-shortage/resolve),
-- completion is room-level, and the ขาดผ้า indication becomes "has OPEN
-- reports" of any age rather than a day-scoped flag. Still PG-canonical only:
-- no legacy counterpart, no sync, no writeback, no domain event. This seed row
-- records the migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('090', '090_hk_linen_reports_resolution.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 091 - Report HK: ht_hk_room_reports + ht_hk_room_report_items +
-- ht_hk_room_report_photos (tables + all five indexes inlined above, after the
-- ht_hk_room_signals block). One maid's per-room daily attestation, verified or
-- returned by reception with a canned reason; exception-based checklist,
-- two-sided photo evidence, append-only history via rr_parent_id.
-- PG-canonical only: no legacy counterpart, no sync mapper, no writeback, no
-- domain event of its own -- item exceptions raise the existing item_missing /
-- item_damaged room signals (089) in the submit's own transaction. This seed
-- row records the migration as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('091', '091_create_ht_hk_room_reports.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- Migration 086 — loyalty-app channel + membership link:
-- `ht_customers.cust_membership_id` + `ht_bookings.book_hold_expires_at`
-- (columns + partial indexes inlined above). Both PG-canonical only — no
-- legacy counterpart, never written back. This seed row records the migration
-- as applied so the drift check sees zero pending.
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('086', '086_loyalty_channel.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Migration 078: re-seed legacy_ct_state_per_table from the global watermark
-- Mirrors migrations/pg/078_reseed_ct_state_per_table.sql. No schema change —
-- a DATA migration that force-sets every per-table CT watermark to the current
-- global `legacy_ct_state.last_seen_version` so `SYNC_PER_TABLE_WATERMARK=true`
-- can be flipped without a retention-overflow page storm + unbounded CT replay.
--
-- Unlike the 050 backfill and the 056 HT_Book_Pro seed above (both
-- `ON CONFLICT DO NOTHING`, which is why the rows froze at their apply date),
-- this one uses an explicit `DO UPDATE` with
-- `GREATEST(existing, EXCLUDED)` — forward-only, so a table that has already
-- advanced past the global row is never rolled backward. Full rationale in the
-- migration file.
--
-- Placed LAST in this script on purpose: it must run after BOTH the 050
-- backfill and the 056 HT_Book_Pro seed. On a fresh install the global
-- watermark is 0, so this is a semantic no-op here — it exists for parity so
-- a fresh DB and a migrated DB converge on the same statement history.
--
-- Table list MUST stay in lock-step with `CT_ENABLED_TABLES` in
-- `hotel-backend/src/bin/sync.rs` (19 entries as of 2026-07-27).
-- =============================================================================

INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version, last_polled_at)
SELECT t.name,
       COALESCE((SELECT last_seen_version FROM legacy_ct_state WHERE id = 1), 0),
       now()
FROM (VALUES
    ('HT_Customers'),
    ('HT_Rooms'),
    ('HT_Room_Status'),
    ('HT_Book_H'),
    ('HT_Book_Ds'),
    ('HT_Book_Date'),
    ('HT_CheckIn_H'),
    ('HT_CheckIn_Ds'),
    ('HT_CheckIn_Pay'),
    ('HT_Receipt_H'),
    ('HT_Cupon'),
    ('HT_CheckIn_Product'),
    ('HT_Deposit'),
    ('HT_Changed_Room'),
    ('HT_Bill_Debt_H'),
    ('HT_Bill_Debt_Ds'),
    ('HT_CheckIn_Other_People'),
    ('HT_Rooms_Cancel'),
    ('HT_Book_Pro')
) AS t(name)
ON CONFLICT (table_name) DO UPDATE
    SET last_seen_version = GREATEST(
            legacy_ct_state_per_table.last_seen_version,
            EXCLUDED.last_seen_version
        ),
        last_polled_at = now();

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('078', '078_reseed_ct_state_per_table.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 079 — `ht_checkins.cin_room_amount` (room-only folio total mirrored
-- from legacy `HT_CheckIn_H.Total_Price_Room`). The column is declared inline in
-- the `ht_checkins` CREATE TABLE above, so a fresh seed already has it; this row
-- just tells scripts/migrate.sh not to re-apply the ALTER.
-- -----------------------------------------------------------------------------
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('079', '079_ht_checkins_room_amount.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 080 — `ht_receipts_legacy`, the per-PK ack cache for the Phase 6-A
-- `payments` reconcile arm (legacy `HT_Receipt_H` ↔ canonical `ht_payments`),
-- plus the `ht_payments.pay_reference` index its canonical probe needs. The
-- `sync_status` seed row rides the `sync_status` INSERT above.
--
-- The arm SHIPS DARK (`RECONCILE_PAYMENTS_ARM_ENABLED`, compose default false);
-- with the flag off this table simply stays empty. Cache ONLY — never canonical
-- state. See migrations/pg/080_ht_receipts_legacy.sql for the full rationale.
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS ht_receipts_legacy (
    id          SERIAL PRIMARY KEY,
    receipt_no  VARCHAR(50) NOT NULL UNIQUE,
    sync_hash   VARCHAR(64),
    synced_at   TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_receipts_legacy_synced ON ht_receipts_legacy(synced_at);

CREATE INDEX IF NOT EXISTS ix_ht_payments_pay_reference
    ON ht_payments (pay_reference) WHERE pay_reference IS NOT NULL;

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('080', '080_ht_receipts_legacy.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 081 — `ht_guest_registry_legacy`, the per-FOLIO ack cache for the
-- Phase 6-B `guest_registry` reconcile arm (legacy `HT_CheckIn_Other_People` ↔
-- canonical `ht_guest_registry`). The `sync_status` seed row rides the
-- `sync_status` INSERT above.
--
-- The unit of reconciliation is the FOLIO (every companion sharing one
-- `Cin_no`), not the row: iHOTEL edits companions by DELETE+REINSERT, so a
-- per-row arm would false-positive on every edit. The arm SHIPS DARK
-- (`RECONCILE_GUEST_REGISTRY_ARM_ENABLED`, compose default false); with the
-- flag off this table simply stays empty. Cache ONLY — never canonical state.
-- No new index (every lookup is covered by ix_ht_guestreg_checkin /
-- ix_ht_checkins_legacy_cin_no / ix_ht_checkins_checkin). See
-- migrations/pg/081_ht_guest_registry_legacy.sql for the full rationale.
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS ht_guest_registry_legacy (
    id         SERIAL PRIMARY KEY,
    cin_no     VARCHAR(50) NOT NULL UNIQUE,
    sync_hash  VARCHAR(64),
    synced_at  TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_guest_registry_legacy_synced
    ON ht_guest_registry_legacy(synced_at);

-- ht_reconcile_era_floor — durable, non-decreasing scope watermark per
-- reconcile arm. Seeded lazily by the arm on its first tick and thereafter
-- moved only FORWARD (GREATEST clamp in the upsert), so one historical row
-- gaining a mirrored counterpart cannot drag a derived MIN() floor backwards
-- and expand the scan by years (~19.6k permanently-open rows at HF Hotel).
-- Operators may move a floor forward by hand; the clamp makes that stick. The
-- ONLY way to lower one is to DELETE the row and let the next tick re-derive —
-- which is the documented remedy when a watermark was seeded before a
-- coverage-widening backfill (do not enable an arm at a site until that site's
-- `--all` backfill has completed; a sustained hold raises the
-- `era_floor_held:` Slack alert).
--
-- TWO bases, exactly one per arm (migration 084): `era_floor` (TIMESTAMP —
-- oldest parent time; arm `guest_registry`) and `era_floor_id` (BIGINT —
-- lowest legacy IDENTITY; arm `payment_ledger_probe`, whose mirror is keyed on
-- an integer id and whose date column would have to cross the naive-Thai /
-- TIMESTAMPTZ boundary). The row key is the arm's own
-- ht_reconcile_log.table_name / sync_status.entity_type literal, so the two
-- arms cannot collide on the PK. `era_floor` is NULLABLE because an ID-basis
-- arm has no honest timestamp to write; the CHECK forbids a row with neither.
CREATE TABLE IF NOT EXISTS ht_reconcile_era_floor (
    table_name   VARCHAR(50)  PRIMARY KEY,
    era_floor    TIMESTAMP,
    era_floor_id BIGINT,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ck_ht_reconcile_era_floor_basis
        CHECK (era_floor IS NOT NULL OR era_floor_id IS NOT NULL)
);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('081', '081_ht_guest_registry_legacy.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 082 — `sync_status` row for the Phase 6-C mirror probe
-- (`entity_type = 'mirror_probe'`). The row itself rides the `sync_status`
-- INSERT above; this block only records the migration as applied.
--
-- No table, no index: the probe is detection-only and keeps no ack cache — it
-- compares live aggregates on both sides every tick. The row exists purely so
-- `record_error` / `record_success`'s `UPDATE … WHERE entity_type =
-- 'mirror_probe'` matches something; without it a probe failure updates zero
-- rows and leaves only a log line, and the success path has no
-- `consecutive_failures` to reset. See
-- migrations/pg/082_sync_status_mirror_probe.sql.
-- -----------------------------------------------------------------------------

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('082', '082_sync_status_mirror_probe.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 083 — `sync_status` row for the Phase 6-D payment-ledger probe
-- (`entity_type = 'payment_ledger_probe'`). The row itself rides the
-- `sync_status` INSERT above; this block only records the migration as applied.
--
-- No table, no index: like 082 the probe is detection-only and keeps no ack
-- cache — it re-derives the MIN(ledger_legacy_id) coverage floor and both
-- sides' per-folio aggregates live every tick. The row exists purely so
-- `record_error` / `record_success`'s `UPDATE … WHERE entity_type =
-- 'payment_ledger_probe'` matches something. See
-- migrations/pg/083_sync_status_payment_ledger_probe.sql.
-- -----------------------------------------------------------------------------

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('083', '083_sync_status_payment_ledger_probe.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- -----------------------------------------------------------------------------
-- Migration 084 — `ht_reconcile_era_floor.era_floor_id` (BIGINT), the ID basis
-- the Phase 6-D payment-ledger probe clamps its coverage floor to, plus the
-- `era_floor` NOT NULL drop and the one-basis-minimum CHECK. All three are
-- declared inline in the `ht_reconcile_era_floor` CREATE TABLE above, so a
-- fresh seed already has them; this row just tells scripts/migrate.sh not to
-- re-apply the ALTERs.
--
-- Why: the probe's floor was a raw `MIN(ledger_legacy_id)`, which is only a
-- valid coverage boundary while coverage is an id-contiguous SUFFIX. A
-- date-windowed `backfill_payment_ledger --days=212` mirrors folios WHOLE, and
-- on 2026-07-30 one 2025-08 line on a monthly-billed long-stay dragged the
-- floor back ~7 months, sweeping 404 never-mirrored folios into the scan as
-- `missing_pg`. A persisted floor that only ratchets FORWARD removes the class.
-- See migrations/pg/084_reconcile_era_floor_id.sql.
-- -----------------------------------------------------------------------------

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('084', '084_reconcile_era_floor_id.sql', 'init-script')
ON CONFLICT (version) DO NOTHING;

-- =============================================================================
-- Initialization complete
-- =============================================================================
