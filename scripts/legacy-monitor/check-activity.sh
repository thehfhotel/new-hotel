#!/usr/bin/env bash
# check-activity.sh — recent .NET app activity summary.
# Useful to answer: "did the morning checkout-flag routine actually write to the DB?"

set -euo pipefail
LOG_DIR="$HOME/legacy-monitor"
EVENTS_LOG="$LOG_DIR/events.log"

if [[ ! -s "$EVENTS_LOG" ]]; then
    echo "no events captured yet"; exit 0
fi

# Optional time-window filter: ./check-activity.sh "06:00" "08:00"
if [[ $# -ge 2 ]]; then
    from_t="$1"; to_t="$2"
    echo "=== activity in window $from_t — $to_t (UTC) ==="
    awk -F'\t' -v from="$from_t" -v to="$to_t" '
        $1 ~ /^20/ {
            tm = substr($1, 12, 5)
            if (tm >= from && tm <= to) print
        }' "$EVENTS_LOG"
else
    echo "=== last 1 hour of activity ==="
    one_hour_ago=$(date -u -d "1 hour ago" +%H:%M 2>/dev/null || date -u -v-1H +%H:%M)
    awk -F'\t' -v from="$one_hour_ago" '
        $1 ~ /^20/ {
            tm = substr($1, 12, 5)
            if (tm >= from) print
        }' "$EVENTS_LOG"
fi | (
    echo
    echo "=== writes by table ==="
    awk -F'\t' '$2 == "sql_batch_completed" {print $9}' \
      | grep -ioE '(INSERT INTO|UPDATE|DELETE FROM) *\[?[A-Za-z_]+' \
      | awk '{op=$1; tbl=$NF; gsub(/[\[\]]/,"",tbl); print op, tbl}' \
      | sort | uniq -c | sort -rn

    echo
    echo "=== sample SQL by op ==="
    awk -F'\t' '$2 == "sql_batch_completed" {print $9}' \
      | grep -iE 'INSERT INTO|UPDATE|DELETE FROM' \
      | head -10
)
