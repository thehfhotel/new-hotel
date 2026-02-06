# Database Migrations

This directory contains SQL migration scripts for the hotel management database.

## IMPORTANT: Shared Database

This database is shared with another application. **All schema changes must be documented here** to prevent breaking changes.

## Migration Naming Convention

```
NNN_description.sql
```

- `NNN`: Sequential number (001, 002, etc.)
- `description`: Brief description using underscores (e.g., `create_booking_notes_table`)

## How to Apply Migrations

1. **Review the migration** - Check if the changes are compatible with other applications
2. **Backup the database** - Always backup before applying migrations
3. **Run manually** - Execute the SQL file against the database using SSMS or sqlcmd:

```bash
# Using sqlcmd
sqlcmd -S 192.168.100.222 -d HotelDB -U username -P password -i migrations/001_create_booking_notes_table.sql
```

## Migration Guidelines

1. **Always use IF NOT EXISTS** - Migrations should be idempotent (safe to run multiple times)
2. **Include rollback scripts** - Add commented rollback SQL at the bottom of each file
3. **Document dependencies** - Note which tables/columns are required
4. **Test on staging first** - Never apply untested migrations to production

## Current Migrations

| # | File | Description | Applied |
|---|------|-------------|---------|
| 001 | `001_create_booking_notes_table.sql` | Creates HT_Booking_Notes table for booking annotations | v1.16.0 (deprecated) |
| 002 | `002_create_new_hotel_database.sql` | Creates new HotelNew database with all application-owned tables | Pending |
| 003 | `003_alter_ht_rates_table.sql` | Alters HT_Rates table to support multiplier/fixed rate types for Phase 3 Financial features | Pending |
| 004 | `004_create_inventory_tables.sql` | Creates inventory management tables (categories, items, room inventory, transactions) | Pending |
| 005 | `005_move_booking_notes_to_hotelnew.sql` | Moves HT_Booking_Notes to HotelNew database (enforces legacy DB read-only) | v2.2.0 |

## Tables Owned by This Application

### Legacy Database (db)

**IMPORTANT**: The legacy database should be treated as READ-ONLY. Do not create or modify any tables in the legacy database.

Previously, `HT_Booking_Notes` was created in the legacy database. As of v2.2.0, it has been moved to HotelNew to enforce the read-only principle.

### New Database (HotelNew)

These tables are created by migration 002 in the new HotelNew database:

| Table | Description | Since |
|-------|-------------|-------|
| `HT_Customers` | Customer information (replaces View_Customers) | v2.0.0 |
| `HT_Room_Types` | Room type definitions (Standard, Deluxe, Suite, etc.) | v2.0.0 |
| `HT_Rooms_New` | Room information (replaces HT_Rooms) | v2.0.0 |
| `HT_Bookings` | Booking records (replaces View_Booking_Ds) | v2.0.0 |
| `HT_Booking_Rooms` | Junction table linking bookings to rooms | v2.0.0 |
| `HT_CheckIns` | Check-in records (replaces View_CheckIn_Ds) | v2.0.0 |
| `HT_Guest_Registry` | Guest registry for TM30 compliance | v2.0.0 |
| `HT_Rates` | Room rates by type and date range | v2.0.0 |
| `HT_Settings` | Application settings key-value store | v2.0.0 |
| `HT_Inventory_Categories` | Inventory categories (Minibar, Amenities, Linens, Equipment) | v2.1.0 |
| `HT_Inventory_Items` | Inventory items with stock tracking | v2.1.0 |
| `HT_Room_Inventory` | Items assigned to each room | v2.1.0 |
| `HT_Inventory_Transactions` | Stock movement transactions (IN, OUT, ADJUST, MOVE) | v2.1.0 |
| `HT_Booking_Notes` | Booking notes (moved from legacy DB in v2.2.0) | v2.2.0 |

## Tables Used (Read-Only or Shared)

These tables and views are owned by the legacy application.

| Table/View | Usage |
|------------|-------|
| `HT_Rooms` | Room information |
| `View_Booking_Ds` | Booking records |
| `View_CheckIn_Ds` | Check-in records |
| `View_Customers` | Customer information |

### PROHIBITED Actions on Legacy Tables/Views

**DO NOT** perform any of the following on tables/views not owned by this app:

- `ALTER TABLE` - Do not add, modify, or drop columns
- `ALTER VIEW` - Do not modify view definitions
- `DROP TABLE` / `DROP VIEW` - Do not delete
- `CREATE INDEX` on legacy tables - May affect legacy app performance
- `ADD CONSTRAINT` - Do not add foreign keys or other constraints

### What To Do Instead

If you need additional data or relationships:

1. **Create a new table** - Store additional data in a new table owned by this app
   ```sql
   -- Example: Need to add notes to bookings
   -- DON'T: ALTER TABLE View_Booking_Ds ADD Note_Text NVARCHAR(MAX)
   -- DO: CREATE TABLE HT_Booking_Notes (Book_No, Note_Text, ...)
   ```

2. **Create a new view** - If you need a different data shape, create your own view
   ```sql
   -- Example: Need a combined view
   -- DON'T: ALTER VIEW View_Booking_Ds AS ...
   -- DO: CREATE VIEW HT_Booking_Summary AS SELECT ... FROM View_Booking_Ds JOIN ...
   ```

3. **Use application-level joins** - Join data in your API code instead of modifying the database
