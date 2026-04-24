#!/usr/bin/env bash
# check-errors.sh — quick "is everything OK?" check.
# Returns:
#   exit 0 if no errors since monitor start
#   exit 1 + prints last N errors if any errors detected

set -euo pipefail
LOG_DIR="$HOME/legacy-monitor"
ERRORS_LOG="$LOG_DIR/errors.log"

if [[ ! -f "$ERRORS_LOG" ]]; then
    echo "no monitor log yet — has tail-loop.sh been started?"
    exit 2
fi

if [[ ! -s "$ERRORS_LOG" ]]; then
    echo "OK: no errors detected since monitor started ($(stat -c %y "$LOG_DIR/.cursor" 2>/dev/null || echo unknown))"
    exit 0
fi

count=$(wc -l < "$ERRORS_LOG")
echo "ALERT: $count error events detected"
echo "--- last 20 errors ---"
tail -20 "$ERRORS_LOG"
exit 1
