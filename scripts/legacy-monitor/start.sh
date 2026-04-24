#!/usr/bin/env bash
# start.sh — one-command start: setup XE session + spawn background poll loop on evergreen.
# Run from the new-hotel repo root on the dev machine.

set -euo pipefail

cd "$(dirname "$0")"
echo "[$(date -u +%H:%M:%S)] copying scripts to evergreen..."
ssh evergreen 'mkdir -p ~/legacy-monitor/scripts'
scp -q 01-setup-session.sql 02-tail-events.sql tail-loop.sh check-errors.sh check-activity.sh stop.sh evergreen:~/legacy-monitor/scripts/

echo "[$(date -u +%H:%M:%S)] enabling XE session..."
ssh evergreen 'pw=$(grep "^DB_PASSWORD=" ~/new-hotel-production/.env | cut -d= -f2- | tr -d "\"'\''"); cat ~/legacy-monitor/scripts/01-setup-session.sql | docker run --rm -i --network host --entrypoint /opt/mssql-tools18/bin/sqlcmd mcr.microsoft.com/mssql/server:2022-latest -C -S 192.168.100.222 -U sa -P "$pw" -d master -W 2>&1 | grep -v "container is" | grep -v "non-root" | grep -v "linkid"'

echo "[$(date -u +%H:%M:%S)] starting tail loop in background on evergreen..."
ssh evergreen 'chmod +x ~/legacy-monitor/scripts/*.sh && cd ~/legacy-monitor/scripts && nohup ./tail-loop.sh > ~/legacy-monitor/tail-loop.out 2>&1 & disown'
sleep 2
ssh evergreen 'cat ~/legacy-monitor/tail-loop.pid 2>/dev/null && echo "OK — running"'

echo
echo "Monitor active. Files on evergreen:"
echo "  ~/legacy-monitor/events.log    — all .NET app activity"
echo "  ~/legacy-monitor/errors.log    — alerts (empty = healthy)"
echo "  ~/legacy-monitor/activity.log  — periodic summary"
echo
echo "To check status: ssh evergreen ~/legacy-monitor/scripts/check-errors.sh"
echo "To watch live:   ssh evergreen tail -f ~/legacy-monitor/errors.log"
echo "To analyze 6-8am: ssh evergreen ~/legacy-monitor/scripts/check-activity.sh 06:00 08:00"
echo "To stop:         ssh evergreen ~/legacy-monitor/scripts/stop.sh"
