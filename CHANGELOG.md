# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.11.0] - 2026-01-29

### Changed
- Thai ID Middleware distribution changed from source code (zip) to pre-built executables
- Card reader download page now offers platform-specific downloads (Windows .exe, macOS .dmg)
- Simplified installation: download and run, no npm required
- Added macOS Gatekeeper bypass instructions for unsigned app (right-click → Open or System Settings → Privacy & Security)

### Added
- GitHub Actions workflow (`middleware-build.yml`) for automated cross-platform builds
  - Builds Windows portable executable on `windows-latest`
  - Builds macOS disk image on `macos-latest`
  - Creates GitHub Release when manually triggered with version

### Removed
- `public/downloads/thai-id-middleware.zip` - replaced by GitHub Releases

## [1.10.0] - 2026-01-29

### Security
- Fixed npm vulnerabilities in thai-id-middleware: updated electron (^40.0.0) and electron-builder (^26.6.0)

### Added
- Middleware download available from card reader page (`/card-reader`) - users can download the zip file directly from the web app
- Thai ID Middleware Electron desktop app (`thai-id-middleware/`) for cross-platform Thai ID card reading
  - GUI status display: HTTP server, reader connection, card insertion status
  - HTTP server on localhost:9898 with `/health` and `/read` endpoints
  - System tray support for background operation
  - Cross-platform builds: Windows portable .exe, macOS .dmg, Linux .AppImage
  - PC/SC smart card communication using @pcsclite/client
  - Full Thai National ID card data reading: CID, names (Thai/English), DOB, gender, address, issue/expiry dates

## [1.9.0] - 2026-01-29

### Changed
- Reverted card reader from WebUSB to middleware approach (WebUSB blocked for CCID devices)

### Removed
- WebUSB card reader module (browser security prevents access to smart card readers)

## [1.8.0] - 2026-01-29

### Changed
- Card reader middleware URL is now configurable via `NEXT_PUBLIC_CARD_READER_URL` environment variable (build-time)

### Added
- Thai ID Card Reader POC page (`/card-reader`) for reading guest information from Thai national ID cards
- Connects to local middleware service on `localhost:9898` for PC/SC card reader communication
- Displays all card data: citizen ID, Thai/English names, birth date, gender, address, issue/expiry dates, and photo
- Connection status indicator with automatic health checks
- Setup instructions displayed when middleware is not running
- "ใช้ข้อมูลนี้" button for future check-in integration
- New "อ่านบัตร" navigation link in navbar

## [1.7.3] - 2026-01-29

### Changed
- Rooms page detail panel now displays Room_Group, Room_Book_Name (ผู้จอง), and all price tiers (A, B, C)
- Updated GuestInfo interface to match API response (`checkIn`/`checkOut` instead of `checkInDate`/`checkOutDate`)
- fetchRoomDetail now correctly handles new `/api/rooms/[id]` response structure

## [1.7.2] - 2026-01-29

### Added
- Room detail endpoint `/api/rooms/[id]` returning room details with current guest information from check-in records
- Additional room fields in `/api/rooms` API: `Room_PriceA`, `Room_PriceB`, `Room_PriceC`, `Room_Group`, `Room_Book_Name`

### Changed
- Room interface updated to use actual database column names (`Room_PriceA` instead of `Room_Price`, `Room_Group` instead of `Room_Floor`)

## [1.7.1] - 2026-01-28

### Fixed
- Customer API failing with "Invalid column name 'Book_Cust_No'" when `includeLastVisit=true` - changed to correct column name `Book_Cust_ID`
- Customer bookings API (`/api/customers/[id]/bookings`) using wrong column `Book_Cust_No` - changed to `Book_Cust_ID`
- Customer stats API (`/api/customers/[id]/stats`) using wrong columns - changed `Book_Cust_No` to `Book_Cust_ID` and `Cin_Cust_No` to `Cin_cust_no` (case-sensitive)
- Parameter types changed from `sql.Int` to `sql.NVarChar` since customer IDs are strings like "C0001"

## [1.7.0] - 2026-01-28

### Added
- Server-side sorting for customers table - sorts the entire dataset, not just the visible page
- "Last Visit" column in customers table showing each customer's most recent checkout date
- DataTable component now supports controlled server-side sorting via `onSort`, `sortColumn`, and `sortDirection` props

### Changed
- Customers API now accepts `sortBy` and `sortOrder` query parameters for server-side sorting
- Customers API supports optional `includeLastVisit=true` parameter to include last visit dates

## [1.6.9] - 2026-01-28

### Fixed
- Slack notification times displaying 7 hours ahead - now uses UTC for database dates (which store local Thai time) and Asia/Bangkok for current timestamps

## [1.6.8] - 2026-01-28

### Added
- Customer statistics cards in detail modal showing: total bookings, total stays, first/last visit dates, favorite room type, and average stay duration
- New API endpoint `/api/customers/[id]/stats` for customer statistics
- New API endpoint `/api/customers/[id]/bookings` for customer booking history

## [1.6.7] - 2026-01-28

### Changed
- Customer search now supports multiple fields: name, phone number, ID card (13-digit), and customer ID

## [1.6.6] - 2026-01-28

### Added
- Changelog page (`/changelog`) displaying GitHub release history
- New API endpoint `/api/changelog` fetching releases from GitHub API with 5-minute caching
- "ประวัติ" navigation link in navbar

## [1.6.5] - 2026-01-28

### Fixed
- `/customers` page not working due to API response structure mismatch - transformed SQL column names (`Cust_no`, `Cust_name`, etc.) to frontend-expected format (`id`, `name`, etc.) and flattened pagination response

## [1.6.4] - 2026-01-28

### Added
- `pnpm test:slack` script to run Slack integration tests with verbose output

## [1.6.3] - 2026-01-28

### Fixed
- Slack notifications not working in production Docker container - added `SLACK_WEBHOOK_URL` and `SLACK_NOTIFICATIONS_ENABLED` environment variables to docker-compose.yml
- Updated GitHub Actions workflow to pass Slack webhook secret during deployment

## [1.6.2] - 2026-01-28

### Changed
- RoomGrid mobile view now displays rooms as a sorted list instead of floor plan grid
- Desktop view retains the original floor plan grid layout
- Mobile list shows room number, type, and details with status indicator

## [1.6.1] - 2026-01-28

### Changed
- RoomGrid now mobile responsive with horizontal scroll, preserving floor plan layout
- Responsive cell sizes (60px mobile, 70px desktop), text sizes, and legend
- Fixed React key warning in RoomGrid row fragments

## [1.6.0] - 2026-01-28

### Added
- Checkout notifications: Real-time Slack alerts when guests check out (polled every 2 minutes via `Cin_Room_Out` field)
- New booking notifications: Real-time Slack alerts when new bookings are created (polled every 2 minutes via `Book_Date` field)
- New functions in `lib/slack.ts`: `buildCheckOutAlertMessage`, `buildNewBookingAlertMessage`
- New polling functions in `lib/scheduler.ts`: `pollCheckouts`, `pollNewBookings`

## [1.5.7] - 2026-01-28

### Fixed
- Added missing rooms V.201, A2-1, A2-3 to RoomGrid display on the 4th floor row

## [1.5.6] - 2026-01-28

### Changed
- Switched from cloudflared SSH to self-hosted GitHub Actions runner for deployment - simpler and more reliable

## [1.5.5] - 2026-01-28

### Added
- Automated deployment via SSH in CI/CD pipeline - after build, automatically deploys to production server via Cloudflare tunnel

## [1.5.4] - 2026-01-28

### Fixed
- Fixed stats API counts not matching room grid display - checkout queries now filter to only the most recent check-in record per room using MAX(Cin_Room_In) subquery, preventing historical records from incorrectly counting as today's checkouts

## [1.5.3] - 2026-01-26

### Fixed
- Fixed checkout-today query returning old records from guests who already checked out - now only returns rooms where guest is still checked in (`Room_Use = 'yes'`)

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
