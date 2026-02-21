-- Migration: 010_ville_cache_schema
-- Version: 2.22.0
-- Date: 2026-02-21
-- Description: Create ville schema in production hotelnew database to cache
--              HF Ville data locally. ville_sync pushes data here so the backend
--              reads locally instead of crossing the WireGuard tunnel.

-- UP MIGRATION

-- Create the ville schema
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

-- Record migration
INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('010', '010_ville_cache_schema.sql', 'migrate-script')
ON CONFLICT (version) DO NOTHING;

-- DOWN MIGRATION (rollback)
-- DROP SCHEMA IF EXISTS ville CASCADE;
-- DELETE FROM schema_migrations WHERE version = '010';
