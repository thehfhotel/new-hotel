#!/usr/bin/env bash
# =============================================================================
# Legacy SQL Server Migration Runner (Change-Tracking prerequisite DDL)
# =============================================================================
# The MSSQL sibling of scripts/migrate.sh. Applies pending migrations from
# migrations/legacy-mssql/ to a legacy SQL Server instance and records them in
# a `dbo.ht_legacy_migrations` tracking table so they are never re-applied.
#
# WHY THIS EXISTS (incident 2026-06-24): legacy-MSSQL CT-prerequisite DDL used
# to be applied by hand, out of band from the CI/CD deploy. A binary that added
# HT_Book_Pro to the CT watch list shipped BEFORE its `023_book_pro_ct.sql`
# prerequisite was applied, so the watcher spammed "Change tracking is not
# enabled" ~1/sec per site. This runner closes that gap: legacy migrations now
# ride the same automated, tracked, idempotent path PostgreSQL migrations do.
#
# SAFETY (this touches the SHARED legacy DB the live iHOTEL app uses):
#   * `SET LOCK_TIMEOUT` bounds how long any statement waits for a Sch-M lock,
#     so a migration can NEVER block a receptionist mid-shift — it fails fast
#     (error 1222) and the deploy stops instead of queueing behind the app.
#   * `SET XACT_ABORT ON` aborts the batch on the first error.
#   * `-b` makes sqlcmd exit non-zero on any SQL error (the deploy then halts).
#   * Only PENDING migrations (not already in the tracking table) are applied.
#   * The tracking row is written only AFTER the body succeeds, so a partial
#     failure is retried (NEW migrations MUST therefore be idempotent — use
#     `IF NOT EXISTS` / `IF COL_LENGTH(...) IS NULL` guards, same discipline
#     CLAUDE.md mandates for migrations/pg/).
#
# Usage:
#   ./scripts/migrate-legacy-mssql.sh                    # apply pending → HF Hotel
#   ./scripts/migrate-legacy-mssql.sh --site hfville     # apply pending → HF Ville
#   ./scripts/migrate-legacy-mssql.sh --dry-run          # list pending, apply nothing
#   ./scripts/migrate-legacy-mssql.sh --adopt-baseline   # record all present files as
#                                                        #   applied WITHOUT running them
#                                                        #   (one-time seed for a server
#                                                        #   whose migrations were already
#                                                        #   applied manually)
#
# Flags:
#   --site <hfhotel|hfville>   Pick which legacy server to target (same vocab as
#                              migrate.sh / sync-status.sh). Default hfhotel.
#   --dry-run                  Print the pending set and exit 0 without applying.
#   --adopt-baseline           Create the tracking table and seed every CURRENT
#                              migration as already-applied (no DDL executed).
#                              Use ONCE on a server that is known to already be
#                              at the current migration level.
#   --verify-ct                Read table names on stdin (one per line) and check
#                              each has Change Tracking enabled on this server.
#                              Exit 1 (naming the offenders) if any is missing CT,
#                              else 0. The deploy's pre-worker gate — pipe it
#                              `sync --print-ct-tables` so the binary's expected
#                              set is verified against the live server before the
#                              CT watcher starts. Applies/tracks nothing.
#
# Env overrides (all optional — sensible per-site defaults below):
#   LEGACY_DB_SERVER / LEGACY_DB_PORT / LEGACY_DB_NAME   connection target
#   DB_PASSWORD                                          sa password (else read
#                                                        from a secret file)
#   SECRETS_DIR             dir holding `db_password` (default /home/deploy/secrets)
#   MSSQL_IMAGE             sqlcmd-bearing image (default mssql/server:2022-latest)
#   LEGACY_LOCK_TIMEOUT_MS  per-statement lock wait ceiling (default 5000)
#   MIGRATIONS_DIR          source dir (default migrations/legacy-mssql)
#
# Exit codes: 0 = applied / nothing to do; 1 = failure; 2 = bad args.
# =============================================================================

set -euo pipefail

# ─── Args ───────────────────────────────────────────────────────────────────
SITE="hfhotel"
DRY_RUN=0
ADOPT_BASELINE=0
VERIFY_CT=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --site)
            [[ $# -lt 2 ]] && { echo "--site requires a value (hfhotel|hfville)" >&2; exit 2; }
            SITE="$2"; shift 2 ;;
        --site=*) SITE="${1#--site=}"; shift ;;
        --dry-run) DRY_RUN=1; shift ;;
        --adopt-baseline) ADOPT_BASELINE=1; shift ;;
        --verify-ct) VERIFY_CT=1; shift ;;
        -h|--help) grep '^#' "$0" | sed 's/^# \?//'; exit 0 ;;
        *) echo "unknown flag: $1" >&2; exit 2 ;;
    esac
done

# ─── Per-site connection defaults ─────────────────────────────────────────────
# Mirrors the DB_SERVER/MSSQL_PORT/DB_NAME the sync+writeback containers use
# (docker inspect, 2026-06-24). Both sites share the same `sa` password secret
# (`db_password`) until ADR 0001 Phase 8. Env vars override for one-off sessions.
case "$SITE" in
    hfhotel)
        DEF_SERVER="192.168.100.222"; DEF_PORT="1433"; DEF_DB="db" ;;
    hfville)
        DEF_SERVER="192.168.11.51";   DEF_PORT="1436"; DEF_DB="HOTEL" ;;
    *)
        echo "unknown site: $SITE (expected hfhotel or hfville)" >&2; exit 2 ;;
esac
SERVER="${LEGACY_DB_SERVER:-$DEF_SERVER}"
PORT="${LEGACY_DB_PORT:-$DEF_PORT}"
DB="${LEGACY_DB_NAME:-$DEF_DB}"
DB_USER="${DB_USER:-sa}"

MSSQL_IMAGE="${MSSQL_IMAGE:-mcr.microsoft.com/mssql/server:2022-latest}"
LOCK_TIMEOUT_MS="${LEGACY_LOCK_TIMEOUT_MS:-5000}"
MIGRATIONS_DIR="${MIGRATIONS_DIR:-migrations/legacy-mssql}"
SECRETS_DIR="${SECRETS_DIR:-/home/deploy/secrets}"

# Colors (suppressed when not a TTY)
if [[ -t 1 ]]; then
    RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
else
    RED=''; GREEN=''; YELLOW=''; NC=''
fi
log_info()  { echo -e "${GREEN}[mssql-migrate]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[mssql-migrate]${NC} $1"; }
log_error() { echo -e "${RED}[mssql-migrate]${NC} $1" >&2; }

# ─── Password resolution (never echoed) ───────────────────────────────────────
PW="${DB_PASSWORD:-}"
if [[ -z "$PW" ]]; then
    if [[ -r "$SECRETS_DIR/db_password" ]]; then
        PW="$(cat "$SECRETS_DIR/db_password")"
    else
        log_error "No DB_PASSWORD and no readable $SECRETS_DIR/db_password"
        exit 1
    fi
fi

# ─── sqlcmd helper ────────────────────────────────────────────────────────────
# Runs sqlcmd inside the MSSQL image over the host network so it can reach the
# legacy server (HF Ville via the WireGuard interface on evergreen). SQL is fed
# on stdin. `-b` → non-zero exit on SQL error. Password passed via env into the
# container so it never appears in `ps`/argv.
sqlcmd_in() {
    docker run --rm -i --network host \
        -e SQLCMDPASSWORD="$PW" \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        "$MSSQL_IMAGE" \
        -C -S "$SERVER,$PORT" -U "$DB_USER" -d "$DB" -b -W -l "${SQLCMD_LOGIN_TIMEOUT:-8}" "$@"
}
# One-shot query returning bare rows (no headers/footers).
sqlcmd_query() {  # $1 = SQL
    printf '%s\n' "SET NOCOUNT ON;" "$1" | sqlcmd_in -h -1
}

log_info "Target: site=$SITE server=$SERVER,$PORT db=$DB (lock_timeout=${LOCK_TIMEOUT_MS}ms)"

# ─── Connectivity (retry — tolerate transient WireGuard/site blips) ───────────
# HF Ville's MSSQL rides a WireGuard tunnel to a site PC that can briefly drop
# (asleep, NAT keepalive gap, a slow handshake). A SINGLE failed probe used to
# abort the WHOLE deploy — including HF-Hotel-only changes — whenever Ville was
# momentarily unreachable (observed 2026-06-28: an 8s blip failed the deploy;
# the port was open again seconds later). Retry with linear backoff so a
# transient drop self-heals instead of halting the pipeline. Tunable via
# LEGACY_MSSQL_CONNECT_ATTEMPTS (default 5 → ~6+12+18+24s ≈ 60s of patience)
# and SQLCMD_LOGIN_TIMEOUT (per-attempt login bound, default 8s).
conn_attempts="${LEGACY_MSSQL_CONNECT_ATTEMPTS:-5}"
conn_ok=false
for attempt in $(seq 1 "$conn_attempts"); do
    if sqlcmd_query "SELECT 1;" >/dev/null 2>&1; then
        conn_ok=true
        [[ "$attempt" -gt 1 ]] && \
            log_info "Connected to $SERVER,$PORT on attempt ${attempt}/${conn_attempts}."
        break
    fi
    if [[ "$attempt" -lt "$conn_attempts" ]]; then
        backoff=$(( attempt * 6 ))
        log_info "Cannot reach $SERVER,$PORT yet (attempt ${attempt}/${conn_attempts}) — retrying in ${backoff}s..."
        sleep "$backoff"
    fi
done
if [[ "$conn_ok" != true ]]; then
    log_error "Cannot connect to $SERVER,$PORT (db=$DB) after ${conn_attempts} attempts."
    log_error "Check WireGuard, credentials, and that the ${SITE} site server is powered on."
    exit 1
fi

# ─── --verify-ct: pre-worker CT gate ──────────────────────────────────────────
# Read expected table names on stdin and confirm each has Change Tracking
# enabled on THIS server. CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(name)) is
# NULL exactly when CT is off (or the object is absent). One set-based query —
# returns a row per MISSING table. Exits 1 (naming them) so the deploy halts
# BEFORE the CT watcher starts and begins erroring ~1/sec. Touches nothing.
if [[ "$VERIFY_CT" -eq 1 ]]; then
    TABLES=()
    while IFS= read -r line; do
        line="$(echo "$line" | tr -d '[:space:]')"
        [[ -n "$line" ]] && TABLES+=("$line")
    done
    if [[ ${#TABLES[@]} -eq 0 ]]; then
        log_error "--verify-ct: no table names on stdin (pipe in 'sync --print-ct-tables')."
        exit 2
    fi
    # Build a VALUES list: ('T1'),('T2'),...
    values=""
    for t in "${TABLES[@]}"; do
        # Single-quote-escape defensively (table names are ASCII identifiers,
        # but never trust an interpolated value in SQL).
        esc="${t//\'/\'\'}"
        values+="('${esc}'),"
    done
    values="${values%,}"
    log_info "Verifying CT on ${#TABLES[@]} expected table(s) at site=$SITE..."
    missing="$(sqlcmd_query "SELECT v.tn FROM (VALUES ${values}) AS v(tn) \
        WHERE CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(v.tn)) IS NULL \
        ORDER BY v.tn;" | sed '/^$/d')"
    if [[ -n "$missing" ]]; then
        log_error "Change Tracking NOT enabled on the following expected table(s) at site=$SITE:"
        while IFS= read -r m; do echo "    - $m" >&2; done <<< "$missing"
        log_error "Apply the matching migrations/legacy-mssql/ DDL (re-run this script without"
        log_error "--verify-ct to auto-apply pending ones), then retry. Refusing to start workers."
        exit 1
    fi
    log_info "All ${#TABLES[@]} expected table(s) have CT enabled at site=$SITE."
    exit 0
fi

log_info "Ensuring dbo.ht_legacy_migrations tracking table exists..."
sqlcmd_in <<'SQL' >/dev/null
SET XACT_ABORT ON;
IF OBJECT_ID('dbo.ht_legacy_migrations', 'U') IS NULL
BEGIN
    CREATE TABLE dbo.ht_legacy_migrations (
        version    VARCHAR(10)  NOT NULL PRIMARY KEY,
        filename   VARCHAR(255) NOT NULL,
        checksum   VARCHAR(64)  NULL,
        applied_at DATETIME     NOT NULL DEFAULT GETDATE(),
        applied_by VARCHAR(100) NOT NULL DEFAULT 'migrate-legacy-mssql'
    );
END
GO
SQL

# ─── Load applied versions ────────────────────────────────────────────────────
declare -A APPLIED_SET=()
while IFS= read -r v; do
    v="$(echo "$v" | tr -d '[:space:]')"
    [[ -n "$v" ]] && APPLIED_SET["$v"]=1
done < <(sqlcmd_query "SELECT version FROM dbo.ht_legacy_migrations ORDER BY version;" 2>/dev/null || true)

# ─── Discover migrations (exclude *.rollback.sql) ─────────────────────────────
if [[ ! -d "$MIGRATIONS_DIR" ]]; then
    log_warn "Migrations directory '$MIGRATIONS_DIR' not found. Nothing to do."
    exit 0
fi

ALL_FILES=()
for f in $(ls "$MIGRATIONS_DIR"/*.sql 2>/dev/null | sort); do
    base="$(basename "$f")"
    [[ "$base" == *.rollback.sql ]] && continue          # rollbacks are not migrations
    if [[ ! "$base" =~ ^([0-9]{1,3})_.+\.sql$ ]]; then
        log_error "Malformed migration filename: $base (expected NNN_*.sql)"
        exit 1
    fi
    ALL_FILES+=("$f")
done

# ─── --adopt-baseline: record current files as applied, run nothing ───────────
if [[ "$ADOPT_BASELINE" -eq 1 ]]; then
    log_warn "ADOPT-BASELINE: recording ${#ALL_FILES[@]} present migration(s) as applied WITHOUT executing them."
    for f in "${ALL_FILES[@]}"; do
        base="$(basename "$f")"
        [[ "$base" =~ ^([0-9]{1,3})_ ]]; version="${BASH_REMATCH[1]}"
        checksum="$(sha256sum "$f" | cut -d' ' -f1)"
        if [[ -n "${APPLIED_SET[$version]:-}" ]]; then
            log_info "  already tracked: $base"
            continue
        fi
        sqlcmd_in <<SQL >/dev/null
SET XACT_ABORT ON;
INSERT INTO dbo.ht_legacy_migrations (version, filename, checksum, applied_by)
VALUES ('${version}', '${base}', '${checksum}', 'baseline-adopt');
GO
SQL
        log_info "  seeded baseline: $base"
    done
    log_info "Baseline adoption complete for site=$SITE."
    exit 0
fi

# ─── Compute pending ──────────────────────────────────────────────────────────
PENDING=()
for f in "${ALL_FILES[@]}"; do
    base="$(basename "$f")"
    [[ "$base" =~ ^([0-9]{1,3})_ ]]; version="${BASH_REMATCH[1]}"
    [[ -n "${APPLIED_SET[$version]:-}" ]] && continue
    PENDING+=("$f")
done

if [[ ${#PENDING[@]} -eq 0 ]]; then
    log_info "Up to date — no pending legacy-MSSQL migrations for site=$SITE."
    exit 0
fi

log_info "Found ${#PENDING[@]} pending migration(s) for site=$SITE:"
for f in "${PENDING[@]}"; do echo "  - $(basename "$f")"; done

if [[ "$DRY_RUN" -eq 1 ]]; then
    log_warn "--dry-run: not applying. Exiting."
    exit 0
fi

# ─── Apply each pending migration ─────────────────────────────────────────────
# Each file's own GO batches run as-is, preceded by lock/abort guards. The
# tracking row is inserted in a SEPARATE batch only after the body succeeds.
APPLIED=0
for f in "${PENDING[@]}"; do
    base="$(basename "$f")"
    [[ "$base" =~ ^([0-9]{1,3})_ ]]; version="${BASH_REMATCH[1]}"
    checksum="$(sha256sum "$f" | cut -d' ' -f1)"
    log_info "Applying: $base (version $version)..."

    if ! {
        printf '%s\n' "SET LOCK_TIMEOUT ${LOCK_TIMEOUT_MS};" "SET XACT_ABORT ON;" "GO"
        cat "$f"
    } | sqlcmd_in; then
        log_error "Migration $base FAILED (lock timeout or SQL error). Deploy halts."
        log_error "Legacy DB is shared with iHOTEL — a lock-timeout (error 1222) means the"
        log_error "table was busy; re-run during a quieter window. NEW migrations must be idempotent."
        exit 1
    fi

    if ! sqlcmd_in <<SQL; then
SET XACT_ABORT ON;
INSERT INTO dbo.ht_legacy_migrations (version, filename, checksum, applied_by)
VALUES ('${version}', '${base}', '${checksum}', 'migrate-legacy-mssql');
GO
SQL
        log_error "Body of $base applied but tracking INSERT failed."
        log_error "Manually record: INSERT INTO dbo.ht_legacy_migrations (version, filename, checksum, applied_by) VALUES ('${version}','${base}','${checksum}','migrate-legacy-mssql');"
        exit 1
    fi

    log_info "Applied: $base"
    APPLIED=$((APPLIED + 1))
done

log_info "Done — applied $APPLIED legacy-MSSQL migration(s) to site=$SITE."
sqlcmd_query "SELECT version + '  ' + filename + '  ' + CONVERT(VARCHAR(19), applied_at, 120) FROM dbo.ht_legacy_migrations ORDER BY version;"
