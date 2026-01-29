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
| 001 | `001_create_booking_notes_table.sql` | Creates HT_Booking_Notes table for booking annotations | v1.16.0 |

## Tables Owned by This Application

| Table | Description | Since |
|-------|-------------|-------|
| `HT_Booking_Notes` | Stores notes/annotations for bookings | v1.16.0 |

## Tables Used (Read-Only or Shared)

These tables are owned by the legacy application. **DO NOT modify their schema.**

| Table | Usage |
|-------|-------|
| `HT_Rooms` | Room information |
| `View_Booking_Ds` | Booking records |
| `View_CheckIn_Ds` | Check-in records |
| `View_Customers` | Customer information |
