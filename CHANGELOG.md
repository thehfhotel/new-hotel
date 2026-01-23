# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
