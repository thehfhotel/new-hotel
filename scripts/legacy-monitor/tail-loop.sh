#!/usr/bin/env bash
# tail-loop.sh — runs forever (or until killed), polls the XE session every N seconds
# and writes new events to log files. Designed to run on evergreen as a long-lived
# background process.
#
# Three log files in ~/legacy-monitor/:
#   events.log    — all events (tab-separated, append-only)
#   errors.log    — only error_reported events. EMPTY = healthy. Any line = ALERT.
#   activity.log  — summary every minute: write-event count by table
#
# Usage:
#   ./tail-loop.sh &
#   tail -f ~/legacy-monitor/errors.log     # in another shell — alerts here
#   tail -f ~/legacy-monitor/activity.log   # write activity over time
#   ./tail-loop.sh --stop                   # kill the running loop

set -uo pipefail
# NB: no -e — the loop body intentionally tolerates per-command failures
# (e.g. grep returns 1 when no matches; that's normal, not fatal).

INTERVAL="${INTERVAL:-60}"   # poll every 60 seconds by default
LOG_DIR="$HOME/legacy-monitor"
mkdir -p "$LOG_DIR"

PIDFILE="$LOG_DIR/tail-loop.pid"
EVENTS_LOG="$LOG_DIR/events.log"
ERRORS_LOG="$LOG_DIR/errors.log"
ACTIVITY_LOG="$LOG_DIR/activity.log"
CURSOR_FILE="$LOG_DIR/.cursor"

# --- stop sub-command -------------------------------------------------------
if [[ "${1:-}" == "--stop" ]]; then
    if [[ -f "$PIDFILE" ]]; then
        pid=$(cat "$PIDFILE")
        kill "$pid" 2>/dev/null && echo "stopped pid $pid" || echo "no running process"
        rm -f "$PIDFILE"
    else
        echo "no PID file"
    fi
    exit 0
fi

# --- guard: only one instance ------------------------------------------------
if [[ -f "$PIDFILE" ]]; then
    if kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
        echo "already running (pid $(cat "$PIDFILE"))" >&2
        exit 1
    fi
fi
echo $$ > "$PIDFILE"
trap 'rm -f "$PIDFILE"' EXIT

# --- credentials -------------------------------------------------------------
DB_PASSWORD=$(grep "^DB_PASSWORD=" "$HOME/new-hotel-production/.env" | cut -d= -f2- | tr -d "\"'")

# --- runner ------------------------------------------------------------------
runsql() {
    docker run --rm -i --network host \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        mcr.microsoft.com/mssql/server:2022-latest \
        -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d master -y 8000 -Y 8000 \
        -s $'\t' -h -1 \
        -v since="$1"
}

# --- cursor: pick up where we left off ---------------------------------------
if [[ -f "$CURSOR_FILE" ]]; then
    cursor=$(cat "$CURSOR_FILE")
else
    cursor=$(date -u +%Y-%m-%dT%H:%M:%S)
fi
echo "$(date -u +%H:%M:%S) starting from cursor=$cursor (poll every ${INTERVAL}s)" | tee -a "$EVENTS_LOG"

# --- main loop ---------------------------------------------------------------
# Every command inside the loop tolerates failure — we'd rather log "no events"
# than die. The trap deliberately doesn't fire on per-iteration failures.
while true; do
    {
        raw=$(runsql "$cursor" < "$(dirname "$0")/02-tail-events.sql" 2>&1 \
              | grep -v 'container is running' \
              | grep -v 'non-root' \
              | grep -v 'linkid' \
              | grep -v 'rows affected' \
              | grep -v '^$' \
              || true)

        if [[ -n "${raw:-}" ]]; then
            # 1. Append everything to events.log
            printf '%s\n' "$raw" >> "$EVENTS_LOG"

            # 2. Errors → errors.log (events with non-zero severity)
            printf '%s\n' "$raw" \
                | awk -F'\t' '$3 != "0" && $3 != "" {print}' \
                >> "$ERRORS_LOG" 2>/dev/null || true

            # 3. Activity summary: count writes per table (best-effort)
            writes_summary=$(
                printf '%s\n' "$raw" \
                    | awk -F'\t' '$2 == "sql_batch_completed" || $2 == "rpc_completed" {print $9}' \
                    | grep -ioE '(INSERT INTO|UPDATE|DELETE FROM) *\[?[A-Za-z_]+' \
                    | awk '{op=$1; tbl=$NF; gsub(/[\[\]]/,"",tbl); print op" "tbl}' \
                    | sort | uniq -c | sort -rn | head -10 \
                    || true
            )
            if [[ -n "${writes_summary:-}" ]]; then
                echo "[$(date -u +%H:%M:%S)] activity in last cycle:" >> "$ACTIVITY_LOG"
                printf '%s\n' "$writes_summary" | sed 's/^/  /' >> "$ACTIVITY_LOG"
            fi

            # 4. Update cursor
            latest=$(printf '%s\n' "$raw" | awk -F'\t' '$1 ~ /^20/ {print $1}' | sort | tail -1 || true)
            if [[ -n "${latest:-}" ]]; then
                echo "$latest" > "$CURSOR_FILE"
                cursor="$latest"
            fi
        fi
    } || {
        # iteration failed — log and continue rather than die
        echo "[$(date -u +%H:%M:%S)] iteration failed (continuing)" >> "$EVENTS_LOG"
    }

    sleep "$INTERVAL"
done
