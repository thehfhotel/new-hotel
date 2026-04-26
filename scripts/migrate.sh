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
#
# ## Per-migration pragmas
#
# Migration files MAY declare a header pragma to opt out of the default
# per-migration BEGIN/COMMIT atomic wrap. The pragma is a SQL comment that
# must appear in the first 20 lines of the file:
#
#     -- @transactional false
#
# When detected, the runner streams the file body directly to psql with
# `-f` instead of wrapping it in BEGIN/COMMIT. This is required for
# statements that PostgreSQL forbids inside a transaction block, e.g.
# `CREATE INDEX CONCURRENTLY`, `VACUUM`, `REINDEX CONCURRENTLY`,
# `ALTER TYPE ... ADD VALUE` (in older versions).
#
# IMPORTANT: a non-transactional migration that fails mid-way leaves the
# DB in a partially-applied state. The schema_migrations row is recorded
# in a SEPARATE follow-up transaction only after the file completes
# successfully — so a partial failure will be re-attempted on the next
# run, not silently skipped. The migration body is responsible for being
# idempotent (e.g. `CREATE INDEX CONCURRENTLY IF NOT EXISTS`).
# =============================================================================

set -euo pipefail

# Configuration
DB_CONTAINER="${DB_CONTAINER:-new-hotel-db}"
DB_USER="${POSTGRES_USER:-postgres}"
DB_PORT="${PGPORT:-5439}"
DB_NAME="${POSTGRES_DB:-hotelnew}"
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

# Helper: run SQL in the container.
# `-v ON_ERROR_STOP=1` ensures any SQL error aborts psql with a non-zero exit
# code, which lets the migration loop detect failures (otherwise psql would
# happily keep going and the schema_migrations row would still be inserted).
run_sql() {
    docker exec -i "$DB_CONTAINER" psql -v ON_ERROR_STOP=1 -U "$DB_USER" -p "$DB_PORT" -d "$DB_NAME" -t -A "$@"
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

# Step 3: Get list of already-applied versions into an associative array.
# Set-membership lookup avoids the substring-match footgun of `grep -q`
# (e.g. version "1" would falsely match "10" with the previous code).
declare -A APPLIED_SET=()
while IFS= read -r v; do
    [ -n "$v" ] && APPLIED_SET["$v"]=1
done < <(run_sql -c "SELECT version FROM schema_migrations ORDER BY version;" 2>/dev/null || true)

# Step 4: Find pending migrations.
# Expected naming: `NNN_*.sql` where NNN is up to 3 digits, zero-padded.
# The schema_migrations.version column is VARCHAR(10) so we compare on
# the exact zero-padded prefix (e.g. "001", "017") — never the stripped
# integer form.
if [ ! -d "$MIGRATIONS_DIR" ]; then
    log_warn "Migrations directory '$MIGRATIONS_DIR' not found. Nothing to do."
    exit 0
fi

PENDING=()
for migration_file in $(ls "$MIGRATIONS_DIR"/*.sql 2>/dev/null | sort); do
    filename=$(basename "$migration_file")
    # Extract version number (everything before the first underscore).
    # Reject malformed names early so a typo can't be silently skipped.
    if [[ ! "$filename" =~ ^([0-9]{1,3})_.+\.sql$ ]]; then
        log_error "Skipping malformed migration filename: $filename (expected NNN_*.sql)"
        exit 1
    fi
    version_padded="${BASH_REMATCH[1]}"

    if [ -n "${APPLIED_SET[$version_padded]:-}" ]; then
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

# Helper: detect `-- @transactional false` pragma in the first 20 lines of a
# migration file. Case-insensitive, tolerates extra whitespace. Echoes "false"
# when the pragma opts out of the default BEGIN/COMMIT wrap, "true" otherwise.
detect_transactional_pragma() {
    local file="$1"
    # Grep -E with case-insensitive (-i) match against:
    #   ^\s*--\s*@transactional\s+false\s*$
    # Look only at the first 20 lines (head) so a stray comment lower in the
    # file can't accidentally flip the pragma.
    if head -n 20 "$file" | grep -Eiq '^[[:space:]]*--[[:space:]]*@transactional[[:space:]]+false[[:space:]]*$'; then
        echo "false"
    else
        echo "true"
    fi
}

# Step 6: Apply each pending migration (transactionally by default)
APPLIED_COUNT=0
for migration_file in "${PENDING[@]}"; do
    filename=$(basename "$migration_file")
    version=$(echo "$filename" | sed 's/_.*//')
    checksum=$(sha256sum "$migration_file" | cut -d' ' -f1)
    transactional=$(detect_transactional_pragma "$migration_file")

    if [ "$transactional" = "false" ]; then
        log_info "Applying: $filename (version $version) [non-transactional]..."

        # Stream the file body directly without BEGIN/COMMIT so statements
        # like `CREATE INDEX CONCURRENTLY` are allowed. We pipe the body
        # through psql with `\set ON_ERROR_STOP on` so the FIRST failing
        # statement aborts and exits non-zero.
        if ! {
            echo "\\set ON_ERROR_STOP on"
            cat "$migration_file"
        } | run_sql 2>&1; then
            log_error "Migration $filename FAILED (non-transactional — DB may be partially applied)."
            log_error "Re-running migrate.sh will retry this migration. Make sure the file is idempotent."
            log_error "Backup available at: $BACKUP_FILE"
            exit 1
        fi

        # Record the migration in a separate, atomic statement — only after
        # the body succeeded. If THIS step fails the row is missing and a
        # re-run will attempt the (idempotent) body again.
        if ! run_sql <<SQL 2>&1
\set ON_ERROR_STOP on
INSERT INTO schema_migrations (version, filename, checksum, applied_by)
VALUES ('${version}', '${filename}', '${checksum}', 'migrate-script');
SQL
        then
            log_error "Migration $filename body applied but schema_migrations INSERT failed."
            log_error "Manually insert: INSERT INTO schema_migrations (version, filename, checksum, applied_by) VALUES ('${version}', '${filename}', '${checksum}', 'migrate-script');"
            exit 1
        fi
    else
        log_info "Applying: $filename (version $version)..."

        # Wrap migration + tracking insert in a single transaction.
        # `\set ON_ERROR_STOP on` plus a guarded BEGIN/COMMIT means a failure inside
        # the migration aborts the transaction, psql exits non-zero, and the
        # schema_migrations INSERT never runs.
        if ! {
            echo "\\set ON_ERROR_STOP on"
            echo "BEGIN;"
            cat "$migration_file"
            echo ""
            echo "INSERT INTO schema_migrations (version, filename, checksum, applied_by)"
            echo "VALUES ('${version}', '${filename}', '${checksum}', 'migrate-script');"
            echo "COMMIT;"
        } | run_sql 2>&1; then
            log_error "Migration $filename FAILED. Transaction rolled back."
            log_error "Backup available at: $BACKUP_FILE"
            exit 1
        fi
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
