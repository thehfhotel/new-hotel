# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch between stats card and room grid - stats now excludes checkout rooms (after 6 AM) from occupied count
- Occupied rooms count now matches the number of red squares on the grid

### Added
- New "ห้องรอเช็คเอาท์" stat card (blue) showing rooms waiting for checkout today

### Changed
- API integration tests now spin up their own Next.js dev server on port 30031, making tests self-contained and independent of manually running dev server

## [1.5.1] - 2026-01-26

### Fixed
- Fixed "รอเช็คเอาท์" (waiting for checkout) rooms not showing on grid - now uses `View_CheckIn_Ds.Cin_Room_Out` date matching (same method as stats API) instead of unreliable `View_Room_status.room_status` filtering
- Added new `/api/rooms/checkouts-today` endpoint for reliable checkout room detection

## [1.5.0] - 2026-01-26

### Added
- New "booked" (จองแล้ว) room status with yellow color to distinguish rooms with reservations from rooms with checked-in guests
- New "ห้องที่จองแล้ว" stat card on dashboard showing count of booked-but-not-checked-in rooms
- Separate tracking: "ห้องที่มีผู้เข้าพัก" now only counts checked-in guests, "ห้องที่จองแล้ว" counts pending arrivals

## [1.4.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch in stats API - now correctly counts rooms with any non-empty `Room_Book` value, matching the RoomGrid display logic (was only counting `Room_Book = 'yes'`)

## [1.4.1] - 2026-01-26

### Added
- Slack integration test (`__tests__/integration/slack.test.ts`) that sends actual test messages to verify webhook connectivity
- Added `dotenv` dev dependency for loading environment variables in tests

## [1.4.0] - 2026-01-24

### Added
- Slack notifications for hotel operations
  - Hourly report: Occupied rooms count and today's new bookings (sent every hour at minute 0)
  - Check-in alerts: Real-time notifications when guests check in (polled every 2 minutes)
- New files: `lib/slack.ts`, `lib/scheduler.ts`, `instrumentation.ts`
- Environment variables: `SLACK_WEBHOOK_URL`, `SLACK_NOTIFICATIONS_ENABLED`
- `.env.example` file with all available configuration options

### Technical
- Uses `node-cron` for scheduling background tasks
- Leverages Next.js instrumentation hook for server-side startup
- Includes retry logic (3 attempts with exponential backoff) for Slack API calls
- Thai language message formatting with Buddhist Era dates

## [1.3.4] - 2026-01-24

### Changed
- Pinned pnpm version to 9.x in Dockerfile (was `pnpm@latest`) to prevent cache invalidation on pnpm updates
- Expanded .dockerignore to exclude .husky, .swc, .claude, .github, .vscode, and *.log files from build context

## [1.3.3] - 2026-01-24

### Security
- Added pnpm override to force glob >=10.5.0 to resolve CVE-2025-64756 (command injection in CLI)
  - Vulnerability is in `@next/eslint-plugin-next` dependency chain
  - Note: Only affects CLI usage; ESLint uses glob as a library, so actual risk is minimal

## [1.3.2] - 2026-01-24

### Security
- Upgraded Next.js from 14.2.21 to 14.2.35 to resolve 17 Dependabot security alerts:
  - Authorization Bypass in Middleware (High)
  - Race Condition to Cache Poisoning (High)
  - SSRF via Improper Middleware Redirect (High)
  - DoS with Server Components (Medium)
  - Content Injection in Image Optimization (Medium)
  - Cache Key Confusion in Image Optimization (Medium)
  - Information exposure in dev server (Medium)
- Upgraded eslint-config-next from 14.2.21 to 14.2.35 (resolves transitive glob vulnerability)

## [1.3.1] - 2026-01-24

### Security
- Fixed SQL injection vulnerability in `/api/checkins/route.ts` - now uses parameterized queries for startDate and endDate filters

### Added
- Component tests for RoomGrid (rendering, status colors, modal interaction)
- Component tests for DataTable (sorting, pagination, empty/loading states)
- Component tests for Calendar (month navigation, date selection, booking/checkin indicators)
- CI/CD pipeline now runs component tests before building Docker image
- Pre-push git hook to run component tests before pushing (via husky)

### Fixed
- Removed hardcoded database values from API tests to prevent false failures when data changes

## [1.3.0] - 2026-01-24

### Changed
- Room status indicator bar changed from vertical (left side) to horizontal (bottom)
- Legend moved from top to bottom of room grid

## [1.2.0] - 2026-01-24

### Added
- Docker containerization with multi-stage Dockerfile for optimized image size
- GitHub Actions CI/CD workflow for automated builds to ghcr.io
- docker-compose.yml for easy local deployment from container registry
- Environment variable support for database configuration (DB_SERVER, DB_NAME, DB_USER, DB_PASSWORD)

### Changed
- Next.js output mode set to 'standalone' for container deployment
- Database credentials now configurable via environment variables with backward-compatible defaults

## [1.1.1] - 2026-01-24

### Changed
- Migrated package manager from npm to pnpm for faster installs and better disk efficiency

## [1.1.0] - 2026-01-23

### Added
- "Waiting for Checkout" room status with light blue color
- Rooms with checkout date = today display as "รอเช็คเอาท์" after 6 AM

## [1.0.2] - 2026-01-23

### Fixed
- Fixed timezone display in Recent Activity: database stores local Thai time but mssql marks it as UTC, so using `timeZone: 'UTC'` displays the stored values correctly without adding 7 hours

## [1.0.1] - 2026-01-23

### Fixed
- Recent Activity section now shows check-in and check-out dates with times (e.g., "23 ม.ค. 04:32 - 24 ม.ค. 12:00")
- Fixed timezone display issue where UTC dates appeared as same-day ranges after conversion to Thai timezone

## [1.0.0] - 2026-01-23

### Added
- Initial release of Hotel Management Visualization Web App
- Dashboard with room status grid and occupancy statistics
- Room grid with custom layout matching physical hotel floor plan
- Real-time room status display (available, occupied, maintenance)
- Room details from database (Room_Type, Room_Details)
- Occupancy chart showing actual daily room counts (check-ins + stay-overs)
- Calendar page for viewing bookings and check-ins by date
- Customers page with search and pagination
- Bookings page with filtering options
- Rooms page with status management

### API Endpoints
- `/api/stats` - Dashboard statistics
- `/api/rooms` - Room listing with status
- `/api/bookings` - Booking management with pagination
- `/api/customers` - Customer search and listing
- `/api/checkins` - Check-in records
- `/api/occupancy` - Daily occupancy data for charts

### Technical
- Next.js 14 with App Router
- SQL Server database connection (mssql)
- Tailwind CSS for styling
- Recharts for data visualization
- Jest for testing (22 tests passing)
