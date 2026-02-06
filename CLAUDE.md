# Claude Code Instructions

## Versioning & Changelog Policy

**MANDATORY**: When making changes to this project, Claude MUST:

1. **Update CHANGELOG.md** for every significant change:
   - New features go under `### Added`
   - Bug fixes go under `### Fixed`
   - Changes to existing features go under `### Changed`
   - Removed features go under `### Removed`
   - Security fixes go under `### Security`
   - Deprecations go under `### Deprecated`

2. **Version Bumping** (in package.json):
   - MAJOR version (x.0.0): Breaking changes or major new features
   - MINOR version (0.x.0): New features, backward compatible
   - PATCH version (0.0.x): Bug fixes, minor improvements

3. **Commit Messages**: Use conventional commits format:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `style:` for formatting changes
   - `refactor:` for code refactoring
   - `test:` for adding tests
   - `chore:` for maintenance tasks

## Deployment Policy

**MANDATORY**: All deployments to production MUST go through the CI/CD pipeline.

1. **Never deploy manually** - Do not run `docker compose up` or `docker pull` manually on production
2. **Pipeline is the only way** - All changes must be committed, pushed, and deployed via GitHub Actions
3. **Workflow**: `.github/workflows/docker-build.yml` handles:
   - Running tests
   - Building Docker images (frontend + backend)
   - Pushing to GitHub Container Registry (ghcr.io/thehfhotel/*)
   - Deploying to production server via self-hosted runner

4. **To deploy**: Simply `git push` to master - the pipeline handles everything automatically

## Project Structure

- `/app` - Next.js App Router pages (frontend only, no API routes except /api/changelog)
- `/components` - React components
- `/hotel-backend` - Rust/Axum backend API server
- `/__tests__` - Jest test files (component tests only)

## Database Architecture

This application uses a **dual-database architecture**:

| Database | Location | Purpose |
|----------|----------|---------|
| Legacy DB | 192.168.100.222 | Shared with legacy app, READ-ONLY |
| HotelNew DB | Docker container (`newdb`) | Self-hosted, full CRUD |

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    DOCKER COMPOSE                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Frontend   │  │   Backend   │  │   PostgreSQL        │  │
│  │  (web)      │  │  (backend)  │  │   (newdb)           │  │
│  │  Port 3003  │  │  Port 3003  │  │   Port 5439         │  │
│  │             │  │             │  │                     │  │
│  │             │──│─────────────│──│──▶ HotelNew DB      │  │
│  └─────────────┘  │             │  │                     │  │
│                   │─────────────│──│──▶ (internal only)  │  │
│                   └─────────────┘  └─────────────────────┘  │
│                          │                                   │
└──────────────────────────│───────────────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │  Legacy DB              │
              │  192.168.100.222:1433   │
              │  (external, read-only)  │
              └─────────────────────────┘
```

### Self-Hosted Database (HotelNew)

The HotelNew database runs in a Docker container (`newdb` service):
- **Image**: `postgres:17-alpine` (~100MB image, ~50-100MB RAM idle)
- **Rust driver**: `sqlx` crate with `query!()` compile-time macros (offline mode via `.sqlx/` cache)
- **Data persistence**: Docker volume `newdb_data`
- **Access**: Internal only (not exposed to host network)
- **Initialization**: Automatic on first startup (PostgreSQL runs `.sql` files from `/docker-entrypoint-initdb.d/`)

**First-time setup**: Fully automatic. Just run `docker compose up` - PostgreSQL auto-creates the database and runs `init-db/init-hotelnew.sql`.

### SQLx Offline Mode (`.sqlx/` Cache)

The backend uses `sqlx::query!()` compile-time macros for ~76 static SQL queries. These macros verify queries against the PostgreSQL schema at compile time, catching typos and type mismatches before runtime.

**How it works**:
- `hotel-backend/.sqlx/` contains JSON cache files for each `query!()` call
- When `SQLX_OFFLINE=true` is set (Docker builds, CI), sqlx reads from this cache instead of connecting to a live database
- Dynamic queries using `sqlx::query()` (string concatenation) don't need the cache

**MANDATORY**: When modifying any `sqlx::query!()` SQL in the backend:
1. Ensure a PostgreSQL database is running with the current schema
2. Set `DATABASE_URL` environment variable (e.g., `postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew`)
3. Run `scripts/sqlx-prepare.sh` (or manually: `cd hotel-backend && cargo sqlx prepare`)
4. Commit the updated `.sqlx/` directory

**When to regenerate**:
- After changing any SQL string inside `query!()`, `query_scalar!()`, or `query_as!()`
- After schema changes (new columns, renamed tables, type changes)
- After adding new `query!()` calls

**DECIMAL column handling**:
- SELECT: Cast to `::float8` for `f64` return (e.g., `r.room_price_weekday::float8`)
- INSERT/UPDATE parameters: Cast with `$N::float8` so sqlx accepts `f64` and PostgreSQL converts to NUMERIC

### Legacy Database (192.168.100.222)

- **SHARED DATABASE**: Used by another legacy application. Exercise caution.

**Legacy tables** (READ-ONLY - do not modify schema):
- `HT_Rooms` - Room information
- `View_Booking_Ds` - Booking records
- `View_CheckIn_Ds` - Check-in records
- `View_Customers` - Customer information

**PROHIBITED on legacy tables/views:**
- `ALTER TABLE` / `ALTER VIEW` - Do not modify columns or definitions
- `DROP TABLE` / `DROP VIEW` - Do not delete
- `CREATE INDEX` on legacy tables - May affect legacy app

### HotelNew Tables (owned by this app, PostgreSQL - all lowercase)

All tables in the HotelNew database are owned by this application:
- `ht_customers` - Customer master data
- `ht_room_types` - Room type definitions
- `ht_rooms_new` - Room inventory
- `ht_bookings` - Booking records
- `ht_booking_rooms` - Booking-room assignments
- `ht_checkins` - Check-in records
- `ht_guest_registry` - Guest registration (TM.30)
- `ht_rates` - Room rate configurations
- `ht_settings` - System settings
- `ht_booking_notes` - Booking annotations
- `ht_inventory_categories` - Inventory categories
- `ht_inventory_items` - Inventory items
- `ht_inventory_transactions` - Inventory transactions
- `ht_room_inventory` - Room inventory assignments
- `ht_payments` - Payment records
- `ht_maintenance_categories` - Maintenance categories
- `ht_maintenance_requests` - Maintenance requests
- `schema_migrations` - Migration version tracking

### Database Migrations

Migrations are **automated** via `scripts/migrate.sh`, which runs during CI/CD deployment.

**MANDATORY**: When making ANY HotelNew schema changes:

1. **Create a migration file** in `migrations/pg/`:
   - Name format: `NNN_description.sql` (e.g., `001_add_customer_notes.sql`)
   - Include both UP and DOWN (rollback) migrations
   - Use `IF NOT EXISTS` to make migrations idempotent

2. **Update `init-db/init-hotelnew.sql`** with the same changes (for fresh deployments)

3. **Update the migrations README** (`/migrations/README.md`):
   - Add entry to the PostgreSQL migrations table
   - Document any new tables in "Tables Owned by This Application"

4. **Never auto-create tables in code** without a corresponding migration file

5. **Example migration structure** (PostgreSQL):
   ```sql
   -- Migration: 001_description
   -- Version: x.x.x
   -- Date: YYYY-MM-DD

   -- UP MIGRATION
   CREATE TABLE IF NOT EXISTS ...

   -- DOWN MIGRATION (commented)
   -- DROP TABLE IF EXISTS ...
   ```

The pipeline will automatically: create a backup, apply pending migrations in transactions, and track them in the `schema_migrations` table. See `migrations/README.md` for details.

### Timezone Handling
- SQL Server stores datetime values in **local Thai time (GMT+7)** without timezone information
- The `mssql` library returns datetime fields as ISO strings with `Z` suffix (e.g., `2026-01-22T11:59:00.000Z`)
- JavaScript interprets `Z` as UTC, but the actual values are already in Thai time
- **When formatting dates for display**: Use `timeZone: 'UTC'` to show the stored value as-is
  ```typescript
  new Date(dateValue).toLocaleTimeString('th-TH', {
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC',  // Shows stored value without conversion
  })
  ```
- **Do NOT use** `timeZone: 'Asia/Bangkok'` - this would add 7 hours to the already-local time

## Testing

Run tests before committing: `npm test`

## Architecture

This application uses a **split architecture**:

| Component | Technology | Description |
|-----------|------------|-------------|
| Frontend | Next.js 16 + React 19 | UI layer, proxies API requests to backend |
| Backend | Rust/Axum | API server, database queries, background jobs |
| Legacy DB | SQL Server (tiberius) | Shared with legacy application, read-only |
| HotelNew DB | PostgreSQL (sqlx) | Self-hosted, full CRUD |

**API Routing**:
- Frontend runs on port 3003 (exposed)
- Backend runs on port 3003 (internal, container network)
- Next.js rewrites `/api/*` requests to `http://backend:3003/api/*`
- Exception: `/api/changelog` is handled by Next.js (reads local CHANGELOG.md)

**Docker Services** (defined in `docker-compose.yml`):
- `web` - Next.js frontend (ghcr.io/thehfhotel/new-hotel)
- `backend` - Rust backend (ghcr.io/thehfhotel/new-hotel-backend)
- `newdb` - PostgreSQL 17 (postgres:17-alpine) for HotelNew database

## Development

- Frontend dev server: `pnpm dev` (runs on port 3003)
- Frontend build: `pnpm build`
- Backend: See `/hotel-backend/README.md` for Rust development
