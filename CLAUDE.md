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

## Database

- SQL Server at 192.168.100.222
- **SHARED DATABASE**: This database is used by another legacy application. Exercise caution with schema changes.

### Tables

**Owned by this app** (safe to modify):
- `HT_Booking_Notes` - Booking annotations

**Legacy tables** (READ-ONLY - do not modify schema):
- `HT_Rooms` - Room information
- `View_Booking_Ds` - Booking records
- `View_CheckIn_Ds` - Check-in records
- `View_Customers` - Customer information

**PROHIBITED on legacy tables/views:**
- `ALTER TABLE` / `ALTER VIEW` - Do not modify columns or definitions
- `DROP TABLE` / `DROP VIEW` - Do not delete
- `CREATE INDEX` on legacy tables - May affect legacy app

**Instead:** Create new tables/views owned by this app (prefix with `HT_`)

### Database Migrations

**MANDATORY**: When making ANY database schema changes:

1. **Create a migration file** in `/migrations/` folder:
   - Name format: `NNN_description.sql` (e.g., `002_add_customer_notes.sql`)
   - Include both UP and DOWN (rollback) migrations
   - Use `IF NOT EXISTS` to make migrations idempotent

2. **Update the migrations README** (`/migrations/README.md`):
   - Add entry to the migrations table
   - Document any new tables in "Tables Owned by This Application"

3. **Never auto-create tables in code** without a corresponding migration file

4. **Example migration structure**:
   ```sql
   -- Migration: 002_description
   -- Version: x.x.x
   -- Date: YYYY-MM-DD

   -- UP MIGRATION
   IF NOT EXISTS (...)
   CREATE TABLE ...

   -- DOWN MIGRATION (commented)
   -- DROP TABLE IF EXISTS ...
   ```

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
| Database | SQL Server | Shared with legacy application |

**API Routing**:
- Frontend runs on port 3003 (exposed)
- Backend runs on port 3003 (internal, container network)
- Next.js rewrites `/api/*` requests to `http://backend:3003/api/*`
- Exception: `/api/changelog` is handled by Next.js (reads local CHANGELOG.md)

**Docker Services** (defined in `docker-compose.yml`):
- `web` - Next.js frontend (ghcr.io/thehfhotel/new-hotel)
- `backend` - Rust backend (ghcr.io/thehfhotel/new-hotel-backend)

## Development

- Frontend dev server: `pnpm dev` (runs on port 3003)
- Frontend build: `pnpm build`
- Backend: See `/hotel-backend/README.md` for Rust development
