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
    cust_active BOOLEAN DEFAULT true
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
    room_price_weekday DECIMAL(10,2),
    room_price_weekend DECIMAL(10,2),
    room_price_special DECIMAL(10,2),
    -- Writeback resolver back-populates these (migration 014).
    legacy_room_no VARCHAR(10),
    legacy_room_id_int INTEGER,
    aggregate_id UUID,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

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

    CONSTRAINT fk_ht_guestreg_checkin FOREIGN KEY (guest_cin_id)
        REFERENCES ht_checkins(cin_id) ON DELETE CASCADE,
    CONSTRAINT fk_ht_guestreg_customer FOREIGN KEY (guest_cust_id)
        REFERENCES ht_customers(cust_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_guestreg_checkin ON ht_guest_registry(guest_cin_id);

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
    ('customer.prefix', 'CU', 'string', 'Customer code prefix')
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

-- ht_inventory_items
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
    item_updated TIMESTAMP
);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_category ON ht_inventory_items(item_category_id);
CREATE INDEX IF NOT EXISTS ix_ht_inventory_items_code ON ht_inventory_items(item_code);

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
    CONSTRAINT fk_ht_payments_checkin FOREIGN KEY (pay_cin_id) REFERENCES ht_checkins(cin_id)
);
CREATE INDEX IF NOT EXISTS ix_ht_payments_checkin ON ht_payments(pay_cin_id);
CREATE INDEX IF NOT EXISTS ix_ht_payments_date ON ht_payments(pay_date);

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
-- HF Ville Local Cache Schema (v2.22.0)
-- =============================================================================
-- ville_sync on the jump box pushes HF Ville data here so the backend reads
-- locally instead of crossing the WireGuard tunnel.

CREATE SCHEMA IF NOT EXISTS ville;

-- ville.ht_rooms_legacy - mirrors HT_Rooms from HF Ville SQL Server
CREATE TABLE IF NOT EXISTS ville.ht_rooms_legacy (
    id SERIAL PRIMARY KEY,
    room_no VARCHAR(100) NOT NULL UNIQUE,
    room_type VARCHAR(200),
    room_details TEXT,
    room_clean VARCHAR(100),
    room_use VARCHAR(100),
    room_book VARCHAR(100),
    room_manternace VARCHAR(100),
    room_price_a DECIMAL(10,2),
    room_price_b DECIMAL(10,2),
    room_price_c DECIMAL(10,2),
    room_group VARCHAR(200),
    room_book_name TEXT,
    room_book_time TIMESTAMP,
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ville_rooms_legacy_roomno ON ville.ht_rooms_legacy(room_no);
CREATE INDEX IF NOT EXISTS ix_ville_rooms_legacy_synced ON ville.ht_rooms_legacy(synced_at);

-- ville.ht_bookings_legacy - mirrors View_Booking_Ds from HF Ville SQL Server
CREATE TABLE IF NOT EXISTS ville.ht_bookings_legacy (
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
    CONSTRAINT uq_ville_bookings_legacy_key UNIQUE (book_no, book_room_type)
);
CREATE INDEX IF NOT EXISTS ix_ville_bookings_legacy_bookno ON ville.ht_bookings_legacy(book_no);
CREATE INDEX IF NOT EXISTS ix_ville_bookings_legacy_synced ON ville.ht_bookings_legacy(synced_at);
CREATE INDEX IF NOT EXISTS ix_ville_bookings_legacy_datein ON ville.ht_bookings_legacy(book_date_in);
CREATE INDEX IF NOT EXISTS ix_ville_bookings_legacy_custid ON ville.ht_bookings_legacy(book_cust_id);

-- ville.ht_checkins_legacy - mirrors View_CheckIn_Ds from HF Ville SQL Server
CREATE TABLE IF NOT EXISTS ville.ht_checkins_legacy (
    id SERIAL PRIMARY KEY,
    cin_no VARCHAR(50) NOT NULL UNIQUE,
    cin_room_no VARCHAR(50),
    cin_room_in TIMESTAMP,
    cin_room_out TIMESTAMP,
    cin_cust_name VARCHAR(200),
    cin_cust_no VARCHAR(50),
    cin_status VARCHAR(50),
    cin_checkin_no VARCHAR(50),
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ville_checkins_legacy_cinno ON ville.ht_checkins_legacy(cin_no);
CREATE INDEX IF NOT EXISTS ix_ville_checkins_legacy_synced ON ville.ht_checkins_legacy(synced_at);
CREATE INDEX IF NOT EXISTS ix_ville_checkins_legacy_roomin ON ville.ht_checkins_legacy(cin_room_in);
CREATE INDEX IF NOT EXISTS ix_ville_checkins_legacy_roomno ON ville.ht_checkins_legacy(cin_room_no);

-- ville.ht_customers_legacy - mirrors View_Customers from HF Ville SQL Server
CREATE TABLE IF NOT EXISTS ville.ht_customers_legacy (
    id SERIAL PRIMARY KEY,
    cust_no VARCHAR(100) NOT NULL UNIQUE,
    cust_name VARCHAR(200),
    cust_type VARCHAR(100),
    cust_phone VARCHAR(200),
    cust_idcard VARCHAR(100),
    cust_address VARCHAR(500),
    sync_hash VARCHAR(64),
    synced_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS ix_ville_customers_legacy_custno ON ville.ht_customers_legacy(cust_no);
CREATE INDEX IF NOT EXISTS ix_ville_customers_legacy_synced ON ville.ht_customers_legacy(synced_at);

-- ville.sync_status - tracks push sync health per entity type
CREATE TABLE IF NOT EXISTS ville.sync_status (
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

INSERT INTO ville.sync_status (entity_type) VALUES
    ('customers'),
    ('rooms'),
    ('bookings'),
    ('checkins')
ON CONFLICT (entity_type) DO NOTHING;

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
    id              BIGSERIAL    PRIMARY KEY,
    detected_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
    table_name      TEXT         NOT NULL,
    legacy_pk       TEXT         NOT NULL,
    pg_hash         TEXT,
    mssql_hash      TEXT,
    mssql_row_json  JSONB,
    pg_row_json     JSONB,
    resolved_at     TIMESTAMPTZ
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

-- =============================================================================
-- Initialization complete
-- =============================================================================
