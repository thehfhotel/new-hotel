# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.15.5] - 2026-01-29

### Fixed
- **Fixed /api/changelog endpoint returning 500 error** - Changed from fetching GitHub releases API (which returned 404 for private/non-existent repo) to parsing local CHANGELOG.md file directly. This is more reliable and doesn't require external API calls or authentication.

## [1.15.4] - 2026-01-29

### Changed
- **Refactored CustomTooltip in Charts.tsx** - Moved outside OccupancyChart component to prevent unnecessary recreation on each render
- **Added ESLint 9 flat config** - Configured with Next.js and core-web-vitals rules

## [1.15.3] - 2026-01-29

### Changed
- **Added caching to middleware CI workflow** - Significantly speeds up Windows and macOS builds
  - Added Rust dependency caching using `Swatinem/rust-cache@v2` (~55% faster builds after initial run)
  - Added npm caching via `actions/setup-node@v4` cache option
  - Expected improvement: ~13 min → ~5-6 min (Windows), ~11 min → ~4-5 min (macOS)

## [1.15.2] - 2026-01-29

### Fixed
- **Fixed middleware CI workflow** - Corrected Rust toolchain action from non-existent `dtolnay/rust-action` to `dtolnay/rust-toolchain`, and fixed invalid `universal-apple-darwin` target by installing the correct `aarch64-apple-darwin` and `x86_64-apple-darwin` targets separately

## [1.15.1] - 2026-01-29

### Changed
- **Migrated package manager to pnpm** - Full migration from npm to pnpm for consistent tooling
  - Dockerfile now uses `corepack enable && corepack prepare pnpm@9` with `pnpm install --frozen-lockfile`
  - CI/CD workflow updated to use pnpm version 9 (was 8)
  - Removed `package-lock.json`, now using `pnpm-lock.yaml`
  - Benefits: faster installs, better disk efficiency, consistent with CI environment

## [1.15.0] - 2026-01-29

### Security
- **Upgraded Next.js from 15.5.11 to 16.1.6** - Resolves 1 Dependabot security alert:
  - MODERATE: Unbounded Memory Consumption via PPR Resume Endpoint (GHSA-5f7q-jpqc-wp7h) - fixed in >= 15.6.0
- **Upgraded ESLint from 8.57.1 to 9.39.2** - Required by eslint-config-next 16.x
- Upgraded eslint-config-next from 15.5.11 to 16.1.6

### Changed
- **ESLint configuration migrated to flat config format** (ESLint 9 requirement)
  - Added `eslint.config.mjs` with Next.js flat config
  - Added `@eslint/eslintrc` dependency for flat config support
  - Updated lint script to use `eslint .` (Next.js 16 removed `next lint` command)

### Fixed
- Moved `CustomTooltip` component outside of render function in `Charts.tsx` to fix React Hooks lint error

## [1.14.1] - 2026-01-29

### Changed
- **Middleware build pipeline migrated from Electron to Tauri** - `middleware-build.yml` now builds the Tauri-based Thai ID Middleware instead of the Electron version
  - Triggers on changes to `thai-id-middleware-tauri/` instead of `thai-id-middleware/`
  - Uses Rust toolchain with `dtolnay/rust-action` for cross-platform builds
  - Builds macOS Universal binary (Apple Silicon + Intel) and Windows x64
  - Produces smaller artifacts (~10MB vs ~150MB Electron)

### Removed
- `tauri-build.yml` workflow - consolidated into `middleware-build.yml`

## [1.14.0] - 2026-01-29

### Security
- **Upgraded Next.js from 14.2.35 to 15.5.11** - Resolves 4 Dependabot security alerts:
  - HIGH: HTTP request deserialization DoS via React Server Components (GHSA-qpjv-v59x-3qc4) - fixed in >= 15.0.8
  - HIGH: HTTP request deserialization DoS via React Server Components (duplicate alert) - fixed in >= 15.0.8
  - MEDIUM: Image Optimizer remotePatterns DoS (GHSA-qfcj-68r8-w26x) - fixed in >= 15.5.10
  - MEDIUM: Image Optimizer remotePatterns DoS (duplicate alert) - fixed in >= 15.5.10
- **Upgraded React from 18.3.1 to 19.1.0** - Required by Next.js 15
- Upgraded eslint-config-next from 14.2.35 to 15.5.11

### Changed
- **Breaking Change Migration (Next.js 15)**:
  - API route params are now async (Promise-based) - updated all dynamic routes
  - JSX namespace changed from `JSX.Element` to `React.JSX.Element`
  - Removed deprecated `experimental.instrumentationHook` from next.config.js (now enabled by default)

### Note
- **glib vulnerability (RUSTSEC-2024-0429)** - Unsoundness in `VariantStrIter` iterator
  - Status: **Cannot be fixed** - glib 0.18.5 is constrained by Tauri's GTK3 stack (gtk 0.18.x)
  - Impact: **Low** - Linux builds only, vulnerable API not used by application
  - The gtk-rs GTK3 bindings are unmaintained and pinned to glib 0.18.x
  - Fix will come when Tauri migrates to GTK4 or updates dependencies
  - Tracked upstream: waiting for Tauri ecosystem update

## [1.13.3] - 2026-01-29

### Added
- **Photo display in Tauri GUI** - Cardholder's photo now displays in the Tauri frontend when reading cards
  - Photo appears at the top of the debug output when clicking "Test Read"
  - Styled with rounded corners and blue border to match the app theme
- **Debug mode toggle** (🔧 button) in Tauri GUI header
  - When debug=off: Only shows status indicators (HTTP Server, Card Reader, Card) - 400×340px
  - When debug=on: Shows full UI with endpoints, debug tools, and footer - 400×760px
  - Fixed window sizes (non-resizable) that adjust when toggling debug mode
  - Starts in compact mode (debug=off) by default
- **System tray icon** restored - click to show/focus the main window
- **Photo reading support** in Thai ID Middleware Tauri - Read cardholder's photo from Thai ID card
  - New `?photo=true` query parameter for `GET /read` endpoint
  - Photo returned as base64-encoded JPEG in `data.photo` field
  - Photo reading adds ~2 seconds (20 APDU commands for 5KB JPEG data)
  - Example: `curl "http://localhost:9898/read?photo=true" | jq '.data.photo'`
- **Enhanced debug endpoint** (`GET /debug`) now returns comprehensive card information:
  - ATR (Answer To Reset) - card identification bytes
  - Protocol (T=0 or T=1) - smart card communication protocol
  - Reader name
  - AID test results - tests 4 known Thai ID card application IDs with status words
  - Raw read result - shows actual APDU response for CID read command
  - Human-readable status word descriptions (6A82 = File not found, etc.)

### Changed
- Thai ID Middleware Tauri version bumped to 1.1.0
- `read_card` Tauri command now accepts optional `include_photo` parameter
- CardData struct now includes optional `photo` field (base64 string)

## [1.13.2] - 2026-01-29

### Fixed
- **Tauri app crash on macOS** - Fixed SIGABRT crash during app launch
  - Root cause: PNG icon files had 16-bit color depth instead of 8-bit RGBA
  - Converted all icons (32x32.png, 128x128.png, 128x128@2x.png, icon.png) to 8-bit RGBA format
  - Simplified HTTP server lifecycle management to prevent premature shutdown
- **Card reader connection issues** - Fixed "smart card not responding to reset" error
  - Changed from `Protocols::ANY` to explicit `Protocols::T0` for Thai ID cards
  - Added fallback to T1 and ANY protocols if T0 fails
  - Thai ID cards use T=0 protocol which is now tried first for better compatibility

### Added
- **Debug mode for card reader** - Verbose logging can be enabled for troubleshooting
  - HTTP endpoints: `GET /debug`, `GET /debug/enable`, `GET /debug/disable`
  - Tauri commands: `set_debug(enabled)`, `get_debug()`
  - When enabled, logs APDU commands, responses, and connection details to stderr

### Removed
- System tray functionality (temporarily) - Removed to simplify debugging; will be re-added in future version

## [1.13.1] - 2026-01-29

### Fixed
- Memory leak in card reader page causing PC freezing - health check useEffect was using `[checkHealth]` dependency which could cause interval accumulation during re-renders or React Strict Mode double-mounting; changed to `[]` to ensure interval is created exactly once on mount

## [1.13.0] - 2026-01-29

### Added
- **Thai ID Middleware Tauri application** - Complete migration from Electron to Tauri for better macOS Gatekeeper support
  - New `thai-id-middleware-tauri/` directory with full Tauri 2.0 implementation
  - **Rust PC/SC card reader** (`card_reader.rs`) - Native implementation using `pcsc` crate
    - All APDU commands for Thai National ID cards (CID, names, DOB, gender, address, dates)
    - TIS-620 to UTF-8 Thai text encoding conversion
    - Retry logic for cold-inserted cards (5 retries, 1000ms delay)
    - Proper SW1=0x61 response handling with GET RESPONSE
  - **Axum HTTP server** (`server.rs`) - Rust HTTP API on port 9898
    - `GET /health` - Server and reader status
    - `GET /status` - Alias for /health
    - `GET /read` - Read Thai ID card data
    - CORS enabled for localhost web apps
  - **Tauri IPC commands** (`commands.rs`) - Frontend integration
    - `get_status`, `get_version`, `read_card`, `debug_card` commands
  - Frontend ported from Electron with Tauri API integration
- Benefits over Electron:
  - Smaller binary size (~10MB vs ~150MB)
  - Better macOS code signing and Gatekeeper compatibility
  - Lower memory usage
  - Native Rust performance for card operations

## [1.12.5] - 2026-01-29

### Added
- Comprehensive diagnostic logging for Thai ID card reader to help diagnose connection issues
  - Operation counter (`[op:N]`) to correlate connect/disconnect pairs across functions
  - Detailed logging in `resetCard()` showing connect success/failure and disconnect results
  - Detailed logging in `connectWithRetry()` showing each attempt, errors, and retry decisions
  - Entry/exit logging in `readCard()` and `debugCard()` with success/failure status
  - Protocol name logging (T=0/T=1) for successful connections
  - Now logs exact PC/SC error messages to help diagnose silent failures

### Changed
- `connectWithRetry()` now retries on any error, not just "unresponsive" errors
- Middleware version bumped to 1.1.5

## [1.12.4] - 2026-01-29

### Fixed
- Thai ID card reader reliability issues: cards becoming unreadable after ~30 seconds and cold-inserted cards failing
  - Root cause 1: `SCARD_LEAVE_CARD` disconnect mode leaves card in corrupted state after repeated use
  - Root cause 2: Insufficient retry time (1.5s) for cold-inserted cards needing full power cycle
  - Changed all `SCARD_LEAVE_CARD` to `SCARD_RESET_CARD` - performs warm reset clearing card state
  - Added `resetCard()` function to reset cards in unknown state before connecting
  - Increased retry parameters from 3×500ms to 5×1000ms (5 seconds total)
  - On first connection failure, attempts card reset before retrying
- Middleware version bumped to 1.1.4

## [1.12.3] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Card is unresponsive" error when app starts with card already inserted
  - Root cause: Race condition between card detection and card readiness during power-up sequence
  - Added `connectWithRetry()` helper that retries connection up to 3 times with 500ms delay
  - Both `readCard()` and `debugCard()` now use retry logic to handle cards still initializing
- Middleware version bumped to 1.1.3

## [1.12.2] - 2026-01-29

### Fixed
- Thai ID card reader returning empty data for all fields (CID, names, dates, etc.) despite successful card communication
  - Root cause: `readCard()` used plain `transmit()` instead of `transmitWithGetResponse()` for READ commands
  - When card returns SW1=61 (more data available), `transmitWithGetResponse()` sends GET RESPONSE to retrieve data
  - Also changed from parallel `Promise.all()` to sequential reads (smart cards are sequential devices)
- Middleware version bumped to 1.1.2

## [1.12.1] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Failed to select Thai ID applet" on real Thai National ID cards
  - Root cause: Cards return SW 61XX (more data available) instead of 9000 for SELECT commands
  - SW1=61 is valid ISO 7816-4 success response meaning XX bytes of data are pending
  - Added `transmitWithGetResponse()` to automatically send GET RESPONSE (00 C0 00 00 XX) when needed
  - Updated `debugCard()` to recognize SW1=61 as success indicator
- Middleware version bumped to 1.1.1

## [1.12.0] - 2026-01-29

### Added
- Thai ID Middleware debug mode for diagnosing card reading issues
  - "Test Read" button to attempt card read and display results in the app
  - "Debug Info" button to show card ATR and test multiple Application IDs (AIDs)
  - Dark-themed output panel displaying diagnostic information
  - Tests multiple known Thai ID card AIDs (Standard, Alternate, MOI, EMV)
  - Shows APDU status words with human-readable descriptions
- ATR (Answer To Reset) capture when card is inserted for identification
- Window is now resizable to accommodate debug panel

## [1.11.1] - 2026-01-29

### Fixed
- Middleware build workflow failing with "flate: corrupt input before offset 79" on both Windows and macOS
- Root cause: icon.png was only 64x64 pixels, but electron-builder requires at least 512x512 for macOS and 256x256 for Windows
- Solution: Updated icon.svg to 512x512 dimensions and added SVG-to-PNG conversion step in workflow
  - macOS: Uses `librsvg` (`rsvg-convert`)
  - Windows: Uses `Inkscape` CLI

### Changed
- Icon is now generated from SVG during CI/CD build instead of being committed as PNG
- Updated `generate-icon.js` script with new 512x512 SVG design

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
