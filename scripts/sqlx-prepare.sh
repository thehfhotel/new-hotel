#!/bin/bash
# Generate sqlx offline query cache (.sqlx/ directory)
#
# This script connects to a running PostgreSQL instance to validate
# all sqlx::query!() macros and generate the offline cache.
#
# Usage:
#   ./scripts/sqlx-prepare.sh              # Use default local DB
#   ./scripts/sqlx-prepare.sh <db_url>     # Use custom DATABASE_URL
#
# Prerequisites:
#   - cargo-sqlx CLI: cargo install sqlx-cli --no-default-features --features postgres
#   - Running PostgreSQL with the HotelNew schema

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BACKEND_DIR="$PROJECT_DIR/hotel-backend"

# Default DATABASE_URL (matches docker-compose.yml newdb service)
DEFAULT_DB_URL="postgres://postgres:***REMOVED***@localhost:5439/hotelnew"

DATABASE_URL="${1:-$DEFAULT_DB_URL}"
export DATABASE_URL

echo "=== sqlx prepare ==="
echo "Backend dir: $BACKEND_DIR"
echo "Database URL: ${DATABASE_URL%%@*}@***"

# Check if sqlx-cli is installed
if ! command -v cargo-sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Run cargo sqlx prepare
cd "$BACKEND_DIR"
echo "Running cargo sqlx prepare..."
cargo sqlx prepare

echo ""
echo "=== Done ==="
echo "Generated .sqlx/ cache in $BACKEND_DIR/.sqlx/"
echo ""
echo "Next steps:"
echo "  1. git add hotel-backend/.sqlx/"
echo "  2. git commit -m 'chore: update sqlx offline cache'"
