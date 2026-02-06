#!/usr/bin/env bash
# =============================================================================
# Manual Backup Utility for HotelNew PostgreSQL Database
# =============================================================================
# Creates a pg_dump backup of the HotelNew database.
#
# Usage: ./scripts/backup-db.sh
# =============================================================================

set -euo pipefail

DB_CONTAINER="${DB_CONTAINER:-new-hotel-db}"
DB_USER="${DB_USER:-postgres}"
DB_PORT="${DB_PORT:-5439}"
DB_NAME="${DB_NAME:-hotelnew}"
BACKUP_DIR="${BACKUP_DIR:-backups}"

mkdir -p "$BACKUP_DIR"
BACKUP_FILE="$BACKUP_DIR/hotelnew_$(date +%Y%m%d_%H%M%S).sql"

echo "Backing up $DB_NAME from container $DB_CONTAINER..."
docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -p "$DB_PORT" "$DB_NAME" > "$BACKUP_FILE"

if [ -s "$BACKUP_FILE" ]; then
    SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
    echo "Backup saved: $BACKUP_FILE ($SIZE)"
else
    echo "ERROR: Backup file is empty."
    rm -f "$BACKUP_FILE"
    exit 1
fi
