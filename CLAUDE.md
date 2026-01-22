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

## Project Structure

- `/app` - Next.js App Router pages and API routes
- `/components` - React components
- `/lib` - Database and utility functions
- `/__tests__` - Jest test files

## Database

- SQL Server at 192.168.100.222
- Tables: HT_Rooms, View_Booking_Ds, View_CheckIn_Ds, View_Customers

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

## Development

- Dev server: `npm run dev` (runs on port 3003)
- Build: `npm run build`
