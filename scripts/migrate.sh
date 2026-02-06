#!/usr/bin/env bash
# =============================================================================
# PostgreSQL Migration Runner for HotelNew Database
# =============================================================================
# Runs pending migrations from migrations/pg/ against the PostgreSQL container.
# Tracks applied migrations in the schema_migrations table.
# Creates a pg_dump backup before applying any new migrations.
#
# Usage:
#   ./scripts/migrate.sh              # Run from project root
#   DEPLOY_DIR=/path ./scripts/migrate.sh  # Run from deploy directory
# =============================================================================

set -euo pipefail

# Configuration
DB_CONTAINER="${DB_CONTAINER:-new-hotel-db}"
DB_USER="${DB_USER:-postgres}"
DB_PORT="${DB_PORT:-5439}"
DB_NAME="${DB_NAME:-hotelnew}"
MIGRATIONS_DIR="${MIGRATIONS_DIR:-migrations/pg}"
BACKUP_DIR="${BACKUP_DIR:-backups}"
MAX_BACKUPS="${MAX_BACKUPS:-10}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info()  { echo -e "${GREEN}[migrate]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[migrate]${NC} $1"; }
log_error() { echo -e "${RED}[migrate]${NC} $1"; }

# Helper: run SQL in the container
run_sql() {
    docker exec -i "$DB_CONTAINER" psql -U "$DB_USER" -p "$DB_PORT" -d "$DB_NAME" -t -A "$@"
}

# Step 1: Verify container is running and healthy
log_info "Checking database container '$DB_CONTAINER'..."
HEALTH=$(docker inspect --format='{{.State.Health.Status}}' "$DB_CONTAINER" 2>/dev/null || echo "not_found")
if [ "$HEALTH" != "healthy" ]; then
    log_error "Container '$DB_CONTAINER' is not healthy (status: $HEALTH)"
    exit 1
fi
log_info "Container is healthy."

# Step 2: Ensure schema_migrations table exists (handles existing DBs without it)
log_info "Ensuring schema_migrations table exists..."
run_sql <<'SQL'
CREATE TABLE IF NOT EXISTS schema_migrations (
    id SERIAL PRIMARY KEY,
    version VARCHAR(10) NOT NULL UNIQUE,
    filename VARCHAR(255) NOT NULL,
    checksum VARCHAR(64),
    applied_at TIMESTAMP DEFAULT NOW(),
    applied_by VARCHAR(100) DEFAULT 'migrate-script'
);

INSERT INTO schema_migrations (version, filename, applied_by)
VALUES ('000', '000_baseline.sql', 'migrate-script')
ON CONFLICT (version) DO NOTHING;
SQL

# Step 3: Get list of already-applied versions
APPLIED_VERSIONS=$(run_sql -c "SELECT version FROM schema_migrations ORDER BY version;" 2>/dev/null || echo "")

# Step 4: Find pending migrations
if [ ! -d "$MIGRATIONS_DIR" ]; then
    log_warn "Migrations directory '$MIGRATIONS_DIR' not found. Nothing to do."
    exit 0
fi

PENDING=()
for migration_file in $(ls "$MIGRATIONS_DIR"/*.sql 2>/dev/null | sort); do
    filename=$(basename "$migration_file")
    # Extract version number (everything before the first underscore)
    version=$(echo "$filename" | sed 's/_.*//' | sed 's/^0*//' )
    # Keep leading zeros for comparison
    version_padded=$(echo "$filename" | sed 's/_.*//')

    if echo "$APPLIED_VERSIONS" | grep -q "^${version_padded}$"; then
        continue
    fi
    PENDING+=("$migration_file")
done

if [ ${#PENDING[@]} -eq 0 ]; then
    log_info "Database is up to date. No pending migrations."
    exit 0
fi

log_info "Found ${#PENDING[@]} pending migration(s):"
for f in "${PENDING[@]}"; do
    echo "  - $(basename "$f")"
done

# Step 5: Create backup before applying migrations
mkdir -p "$BACKUP_DIR"
BACKUP_FILE="$BACKUP_DIR/hotelnew_$(date +%Y%m%d_%H%M%S).sql"
log_info "Creating backup: $BACKUP_FILE"
docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -p "$DB_PORT" "$DB_NAME" > "$BACKUP_FILE" 2>/dev/null

if [ ! -s "$BACKUP_FILE" ]; then
    log_error "Backup file is empty. Aborting migration."
    rm -f "$BACKUP_FILE"
    exit 1
fi

BACKUP_SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
log_info "Backup complete ($BACKUP_SIZE)."

# Step 6: Apply each pending migration in a transaction
APPLIED_COUNT=0
for migration_file in "${PENDING[@]}"; do
    filename=$(basename "$migration_file")
    version=$(echo "$filename" | sed 's/_.*//')
    checksum=$(sha256sum "$migration_file" | cut -d' ' -f1)

    log_info "Applying: $filename (version $version)..."

    # Wrap migration + tracking insert in a single transaction
    {
        echo "BEGIN;"
        cat "$migration_file"
        echo ""
        echo "INSERT INTO schema_migrations (version, filename, checksum, applied_by)"
        echo "VALUES ('${version}', '${filename}', '${checksum}', 'migrate-script');"
        echo "COMMIT;"
    } | run_sql 2>&1

    if [ $? -ne 0 ]; then
        log_error "Migration $filename FAILED. Transaction rolled back."
        log_error "Backup available at: $BACKUP_FILE"
        exit 1
    fi

    log_info "Applied: $filename"
    APPLIED_COUNT=$((APPLIED_COUNT + 1))
done

# Step 7: Prune old backups (keep last N)
BACKUP_COUNT=$(ls -1 "$BACKUP_DIR"/hotelnew_*.sql 2>/dev/null | wc -l)
if [ "$BACKUP_COUNT" -gt "$MAX_BACKUPS" ]; then
    PRUNE_COUNT=$((BACKUP_COUNT - MAX_BACKUPS))
    log_info "Pruning $PRUNE_COUNT old backup(s) (keeping last $MAX_BACKUPS)..."
    ls -1t "$BACKUP_DIR"/hotelnew_*.sql | tail -n "$PRUNE_COUNT" | xargs rm -f
fi

# Step 8: Summary
log_info "Migration complete! Applied $APPLIED_COUNT migration(s)."
run_sql -c "SELECT version, filename, applied_at FROM schema_migrations ORDER BY version;"
