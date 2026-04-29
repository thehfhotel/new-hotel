#!/usr/bin/env bash
# =============================================================================
# Manual Backup Utility for new-hotel PostgreSQL Databases
# =============================================================================
# Creates a pg_dump backup of the canonical PG database for either site
# (HF Hotel or HF Ville). Both databases live in the same `new-hotel-db`
# container; only the database name (and the output filename discriminator)
# changes between sites.
#
# Usage:
#   ./scripts/backup-db.sh                 # default: --site hfhotel
#   ./scripts/backup-db.sh --site hfhotel  # → hotelnew DB
#   ./scripts/backup-db.sh --site hfville  # → hotelville DB
#   ./scripts/backup-db.sh --help
#
# Flags:
#   --site <hfhotel|hfville>   Pick which site's PG database to back up.
#                              hfhotel (default) → database `hotelnew`.
#                              hfville           → database `hotelville`.
#                              Any other value exits non-zero with an error
#                              on stderr.
#
# Output files:
#   --site hfhotel  →  $BACKUP_DIR/hotelnew-hfhotel-YYYYMMDD_HHMMSS.sql
#   --site hfville  →  $BACKUP_DIR/hotelville-hfville-YYYYMMDD_HHMMSS.sql
#
# Env overrides (apply to both sites):
#   DB_CONTAINER (default: new-hotel-db)
#   DB_USER      (default: postgres)
#   DB_PORT      (default: 5439)
#   BACKUP_DIR   (default: backups)
#   DB_NAME      Explicit override — when set, wins over --site derivation.
#                Useful for one-off operator sessions targeting a renamed DB.
#
# Recommended cron entries (DO NOT auto-install — operator action):
#
#   # /etc/cron.d/new-hotel-backups (or your operator's equivalent)
#   0 2 * * * nut /home/nut/new-hotel-production/scripts/backup-db.sh --site hfhotel
#   5 2 * * * nut /home/nut/new-hotel-production/scripts/backup-db.sh --site hfville
#
# The 5-minute stagger keeps the two pg_dump runs from competing for I/O on
# the same disk (both DBs live in the same Postgres container on evergreen).
# =============================================================================

set -euo pipefail

# ─── Config (env overrides preserved) ─────────────────────────────────────────
DB_CONTAINER="${DB_CONTAINER:-new-hotel-db}"
DB_USER="${DB_USER:-postgres}"
DB_PORT="${DB_PORT:-5439}"
BACKUP_DIR="${BACKUP_DIR:-backups}"

# ─── Args ─────────────────────────────────────────────────────────────────────
# `--site` follows the same parsing pattern as scripts/sync-status.sh (#78):
# value-bearing flag, default `hfhotel` for back-compat, reject unknowns with
# stderr message + exit 2 (matches sync-status.sh's chosen failure code).
SITE="hfhotel"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --site)
            [[ $# -lt 2 ]] && { echo "--site requires a value (hfhotel|hfville)" >&2; exit 2; }
            SITE="$2"; shift 2 ;;
        --site=*)
            SITE="${1#--site=}"; shift ;;
        -h|--help)
            grep '^#' "$0" | sed 's/^# \?//'; exit 0 ;;
        *)
            echo "unknown flag: $1" >&2; exit 2 ;;
    esac
done

# ─── Derive DB_NAME + filename discriminator from --site ──────────────────────
# Explicit DB_NAME env wins (operator one-off use); otherwise derive from site.
case "$SITE" in
    hfhotel)
        SITE_DB_DEFAULT="hotelnew"
        FILENAME_PREFIX="hotelnew-hfhotel"
        ;;
    hfville)
        SITE_DB_DEFAULT="hotelville"
        FILENAME_PREFIX="hotelville-hfville"
        ;;
    *)
        echo "unknown site: $SITE (expected hfhotel or hfville)" >&2
        exit 2
        ;;
esac
DB_NAME="${DB_NAME:-$SITE_DB_DEFAULT}"

# ─── Run pg_dump ──────────────────────────────────────────────────────────────
mkdir -p "$BACKUP_DIR"
BACKUP_FILE="$BACKUP_DIR/${FILENAME_PREFIX}-$(date +%Y%m%d_%H%M%S).sql"

echo "Backing up $DB_NAME (site=$SITE) from container $DB_CONTAINER..."
docker exec "$DB_CONTAINER" pg_dump -U "$DB_USER" -p "$DB_PORT" "$DB_NAME" > "$BACKUP_FILE"

if [ -s "$BACKUP_FILE" ]; then
    SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
    echo "Backup saved: $BACKUP_FILE ($SIZE)"
else
    echo "ERROR: Backup file is empty."
    rm -f "$BACKUP_FILE"
    exit 1
fi
