#!/usr/bin/env bash
# stop.sh — stop the tail loop and the XE session.
# Keeps the .xel file on disk for later analysis.

set -euo pipefail

cd "$(dirname "$0")"

echo "[$(date -u +%H:%M:%S)] stopping tail loop systemd unit..."
systemctl --user stop legacy-monitor.service 2>/dev/null || true
./tail-loop.sh --stop || true

echo "[$(date -u +%H:%M:%S)] stopping XE session..."
DB_PASSWORD=$(grep "^DB_PASSWORD=" "$HOME/new-hotel-production/.env" | cut -d= -f2- | tr -d "\"'")
echo "
SET QUOTED_IDENTIFIER ON;
IF EXISTS (SELECT 1 FROM sys.dm_xe_sessions WHERE name = 'hotel_monitor')
    ALTER EVENT SESSION [hotel_monitor] ON SERVER STATE = STOP;
PRINT 'hotel_monitor stopped';
" | docker run --rm -i --network host --entrypoint /opt/mssql-tools18/bin/sqlcmd mcr.microsoft.com/mssql/server:2022-latest -C -S <legacy-mssql-host> -U sa -P "$DB_PASSWORD" -d master -W 2>&1 | grep -v "container is" | grep -v "non-root" | grep -v "linkid"

echo "Done. Logs preserved in ~/legacy-monitor/"
echo "To DROP the session entirely (cleanup): cat 03-drop-session.sql | sqlcmd ..."
