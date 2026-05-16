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
    cust_price_over DOUBLE PRECISION
);
CREATE INDEX IF NOT EXISTS ix_ht_customers_name ON ht_customers(cust_firstname, cust_lastname);
CREATE INDEX IF NOT EXISTS ix_ht_customers_phone ON ht_customers(cust_phone);
CREATE INDEX IF NOT EXISTS ix_ht_customers_idcard ON ht_customers(cust_idcard);
CREATE INDEX IF NOT EXISTS ix_ht_customers_passport ON ht_customers(cust_passport);

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
    book_total_amount DECIMAL(12,2) DEFAULT 0,
    book_deposit_amount DECIMAL(12,2) DEFAULT 0,
    book_deposit_date TIMESTAMP,
    book_special_requests TEXT,
    book_internal_notes TEXT,
    book_notes TEXT,
    book_cancelled_at TIMESTAMP,
    book_cancel_reason VARCHAR(500),
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
    ('checkins')
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
    display_name   VARCHAR(128)
);

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
-- Initialization complete
-- =============================================================================
