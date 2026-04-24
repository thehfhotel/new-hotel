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

set -euo pipefail

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
        -C -S 192.168.100.222 -U sa -P "$DB_PASSWORD" -d master \
        -W -s $'\t' -h -1 \
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
while true; do
    started=$(date -u +%H:%M:%S)
    raw=$(runsql "$cursor" < "$(dirname "$0")/02-tail-events.sql" 2>&1 \
        | grep -v 'container is running' | grep -v 'non-root' | grep -v 'linkid' \
        | grep -v 'rows affected' | grep -v '^$')

    if [[ -n "$raw" ]]; then
        # 1. Append everything to events.log
        echo "$raw" >> "$EVENTS_LOG"

        # 2. Errors → errors.log (events with severity > 0)
        echo "$raw" | awk -F'\t' '$3 != "0" && $3 != "" {print}' >> "$ERRORS_LOG" || true

        # 3. Activity summary: count writes per table
        writes_summary=$(echo "$raw" \
            | awk -F'\t' '$2 == "sql_batch_completed" || $2 == "rpc_completed" {print $9}' \
            | grep -ioE '(INSERT INTO|UPDATE|DELETE FROM) *\[?[A-Za-z_]+' \
            | awk '{op=$1; tbl=$NF; gsub(/[\[\]]/,"",tbl); print op" "tbl}' \
            | sort | uniq -c | sort -rn | head -10)

        if [[ -n "$writes_summary" ]]; then
            echo "[$(date -u +%H:%M:%S)] activity in last cycle:" >> "$ACTIVITY_LOG"
            echo "$writes_summary" | sed 's/^/  /' >> "$ACTIVITY_LOG"
        fi

        # 4. Update cursor to the latest event timestamp seen
        latest=$(echo "$raw" | awk -F'\t' '$1 ~ /^20/ {print $1}' | sort | tail -1)
        if [[ -n "$latest" ]]; then
            echo "$latest" > "$CURSOR_FILE"
            cursor="$latest"
        fi
    fi

    sleep "$INTERVAL"
done
