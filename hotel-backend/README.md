# Hotel Backend (Rust)

High-performance Rust backend for the Hotel Management System, replacing the Next.js API routes.

## Tech Stack

- **Framework**: Axum 0.7
- **Runtime**: Tokio
- **Database**: SQL Server via tiberius
- **Scheduling**: tokio-cron-scheduler
- **HTTP Client**: reqwest (for Slack webhooks)

## API Endpoints

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
| `DB_SERVER` | `192.168.100.222` | SQL Server host |
| `DB_NAME` | `db` | Database name |
| `DB_USER` | `sa` | Database user |
| `DB_PASSWORD` | `12345678` | Database password |
| `DB_POOL_MAX` | `10` | Max pool connections |
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
  hotel-backend
```

## Background Jobs

When Slack notifications are enabled, the following jobs run:

- **Hourly Report**: Minute 0 of every hour
- **Check-in Polling**: Every 2 minutes
- **Checkout Polling**: Every 2 minutes
- **Booking Polling**: Every 2 minutes

## Database

This backend connects to the same SQL Server database as the legacy application.

**Read-only tables** (do not modify):
- `HT_Rooms`
- `View_Booking_Ds`
- `View_CheckIn_Ds`
- `View_Customers`

**App-owned tables** (safe to modify):
- `HT_Booking_Notes`

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
│   │   └── pool.rs          # tiberius connection pool
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── rooms.rs
│   │   ├── bookings.rs
│   │   ├── checkins.rs
│   │   ├── customers.rs
│   │   ├── stats.rs
│   │   └── occupancy.rs
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
