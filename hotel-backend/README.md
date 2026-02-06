# Hotel Backend (Rust)

High-performance Rust backend for the Hotel Management System, replacing the Next.js API routes.

## Tech Stack

- **Framework**: Axum 0.7
- **Runtime**: Tokio
- **Legacy Database**: SQL Server via tiberius (read-only, 192.168.100.222)
- **HotelNew Database**: PostgreSQL via sqlx (full CRUD)
- **Scheduling**: tokio-cron-scheduler
- **HTTP Client**: reqwest (for Slack webhooks)

## API Endpoints

### Legacy Endpoints (read-only from SQL Server)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/rooms` | List all rooms |
| GET | `/api/rooms/:id` | Room details with current guest |
| GET | `/api/rooms/status` | Room status history |
| GET | `/api/rooms/checkouts-today` | Today's checkouts |
| GET | `/api/bookings` | Bookings (paginated) |
| GET | `/api/bookings/:id` | Booking details |
| GET | `/api/bookings/:id/notes` | Get booking notes |
| POST | `/api/bookings/:id/notes` | Add booking note |
| DELETE | `/api/bookings/:id/notes` | Delete booking note |
| GET | `/api/checkins` | Check-ins (paginated) |
| GET | `/api/customers` | Customers (search/sort) |
| GET | `/api/customers/:id/bookings` | Customer booking history |
| GET | `/api/customers/:id/stats` | Customer statistics |
| GET | `/api/stats` | Dashboard statistics |
| GET | `/api/occupancy` | Occupancy trends |
| GET | `/api/calendar` | Calendar data (hybrid) |

### New System Endpoints (PostgreSQL - HotelNew)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET/POST | `/api/new/customers` | Customer CRUD |
| GET/PUT/DELETE | `/api/new/customers/:id` | Customer management |
| GET/POST | `/api/new/rooms` | Room CRUD |
| GET/PUT/DELETE | `/api/new/rooms/:id` | Room management |
| GET/POST | `/api/new/bookings` | Booking CRUD |
| GET/PUT/DELETE | `/api/new/bookings/:id` | Booking management |
| GET/POST | `/api/new/checkins` | Check-in management |
| GET/PUT | `/api/new/checkins/:id` | Check-in details/checkout |
| GET/POST/DELETE | `/api/new/checkins/:id/guests` | Guest registry |
| GET/POST | `/api/new/checkins/:id/payments` | Payment tracking |
| DELETE | `/api/new/payments/:id` | Void payment |
| GET | `/api/new/checkins/:id/invoice` | Invoice data |
| GET/POST/PUT/DELETE | `/api/new/rates` | Rate management |
| GET/POST/PUT/DELETE | `/api/new/room-types` | Room type management |
| GET/POST | `/api/new/inventory/*` | Inventory management |
| GET/POST/PUT | `/api/new/maintenance/*` | Maintenance requests |
| GET | `/api/new/reports/*` | Revenue/occupancy reports |
| GET | `/api/new/stats` | New system dashboard stats |
| GET | `/api/mode` | System mode |

## Development

```bash
# Copy environment variables
cp .env.example .env

# Edit .env with your database credentials

# Run in development mode
cargo run

# Run with logging
RUST_LOG=hotel_backend=debug cargo run

# Build for production
cargo build --release
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DB_SERVER` | `192.168.100.222` | Legacy SQL Server host |
| `DB_NAME` | `db` | Legacy database name |
| `DB_USER` | `sa` | Legacy database user |
| `DB_PASSWORD` | `12345678` | Legacy database password |
| `DB_POOL_MAX` | `10` | Legacy max pool connections |
| `NEW_DB_SERVER` | `newdb` | PostgreSQL host (Docker service name) |
| `NEW_DB_PORT` | `5439` | PostgreSQL port |
| `NEW_DB_NAME` | `hotelnew` | PostgreSQL database name |
| `NEW_DB_USER` | `postgres` | PostgreSQL user |
| `NEW_DB_PASSWORD` | `NewHotel@2026!` | PostgreSQL password |
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `3003` | Server port |
| `SLACK_WEBHOOK_URL` | - | Slack webhook URL |
| `SLACK_NOTIFICATIONS_ENABLED` | `true` | Enable Slack notifications |

## Docker

```bash
# Build the image
docker build -t hotel-backend .

# Run the container
docker run -p 3003:3003 \
  -e DB_SERVER=192.168.100.222 \
  -e DB_NAME=db \
  -e DB_USER=sa \
  -e DB_PASSWORD=12345678 \
  -e NEW_DB_SERVER=newdb \
  -e NEW_DB_PORT=5439 \
  -e NEW_DB_NAME=hotelnew \
  -e NEW_DB_USER=postgres \
  -e NEW_DB_PASSWORD=NewHotel@2026! \
  hotel-backend
```

## Background Jobs

When Slack notifications are enabled, the following jobs run:

- **Hourly Report**: Minute 0 of every hour
- **Check-in Polling**: Every 2 minutes
- **Checkout Polling**: Every 2 minutes
- **Booking Polling**: Every 2 minutes

## Database

This backend uses a **dual-database architecture**:

| Database | Driver | Access | Purpose |
|----------|--------|--------|---------|
| Legacy SQL Server (192.168.100.222) | tiberius/bb8 | Read-only | Shared with legacy application |
| HotelNew PostgreSQL (Docker `newdb`) | sqlx | Full CRUD | Application-owned tables |

**Legacy read-only tables**:
- `HT_Rooms`, `View_Booking_Ds`, `View_CheckIn_Ds`, `View_Customers`

**HotelNew tables** (PostgreSQL, all lowercase):
- `ht_customers`, `ht_room_types`, `ht_rooms_new`, `ht_bookings`, `ht_booking_rooms`
- `ht_checkins`, `ht_guest_registry`, `ht_rates`, `ht_settings`, `ht_booking_notes`
- `ht_inventory_categories`, `ht_inventory_items`, `ht_inventory_transactions`, `ht_room_inventory`
- `ht_payments`, `ht_maintenance_categories`, `ht_maintenance_requests`

## Project Structure

```
hotel-backend/
├── Cargo.toml
├── src/
│   ├── main.rs              # Server startup, router
│   ├── config.rs            # Environment config
│   ├── error.rs             # Error types (thiserror)
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs          # tiberius connection pool (legacy SQL Server)
│   │   ├── pg_pool.rs       # sqlx connection pool (PostgreSQL)
│   │   └── dual_pool.rs     # AppState with both pools
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── rooms.rs         # Legacy room routes
│   │   ├── bookings.rs      # Legacy booking routes
│   │   ├── checkins.rs      # Legacy check-in routes
│   │   ├── customers.rs     # Legacy customer routes
│   │   ├── stats.rs         # Legacy stats routes
│   │   ├── occupancy.rs     # Legacy occupancy routes
│   │   ├── calendar.rs      # Hybrid calendar route
│   │   ├── mode.rs          # System mode route
│   │   ├── new_bookings.rs  # New booking CRUD
│   │   ├── new_checkins.rs  # New check-in management
│   │   ├── new_customers.rs # New customer CRUD
│   │   ├── new_rooms.rs     # New room CRUD
│   │   ├── new_room_types.rs# Room type management
│   │   ├── new_rates.rs     # Rate management
│   │   ├── new_stats.rs     # New system stats
│   │   ├── new_inventory.rs # Inventory management
│   │   ├── new_invoice.rs   # Invoice data
│   │   ├── new_payments.rs  # Payment tracking
│   │   ├── new_reports.rs   # Revenue/occupancy reports
│   │   └── new_maintenance.rs # Maintenance requests
│   ├── models/
│   │   ├── mod.rs
│   │   ├── room.rs
│   │   ├── booking.rs
│   │   ├── checkin.rs
│   │   ├── customer.rs
│   │   └── note.rs
│   ├── scheduler/
│   │   ├── mod.rs
│   │   └── jobs.rs
│   ├── notifications/
│   │   ├── mod.rs
│   │   └── slack.rs
│   └── utils/
│       ├── mod.rs
│       └── thai_date.rs
```
