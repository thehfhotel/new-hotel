# Legacy DB Monitor

Long-running passive monitoring of the legacy MSSQL DB (`192.168.100.222`).

## Two purposes

1. **Error watcher** — alerts immediately if our recent CT/PK schema changes
   break anything in the .NET app. Empty `errors.log` = healthy.
2. **Activity recorder** — captures EVERY write the .NET app sends so we can
   later analyze specific time windows (e.g. "what does the app do during
   the morning checkout-flag routine?").

Both purposes share one Extended Events session (`hotel_monitor`), polled
every 60 seconds by a background loop on `evergreen`. No code on the legacy
SQL Server box itself.

## Files

| File | Role |
|---|---|
| `01-setup-session.sql` | Creates the XE session (errors + writes from .NET app, filtered) |
| `02-tail-events.sql`   | Pulls events newer than `:since` cursor — used by the loop |
| `03-drop-session.sql`  | Cleanup: removes the session entirely |
| `tail-loop.sh`         | Runs forever on evergreen, polling every 60s, writing logs |
| `check-errors.sh`      | Quick health check — exit 0 = OK, exit 1 = errors detected |
| `check-activity.sh`    | Recent write summary; supports time-window filter |
| `start.sh`             | One-command bootstrap from dev machine |
| `stop.sh`              | Stop the loop + XE session, keep logs |

## Quick start

From the new-hotel repo on your dev machine:

```bash
./scripts/legacy-monitor/start.sh
# → copies scripts to evergreen, enables XE session, spawns background loop
```

The loop runs as a `nohup` background process on evergreen. It survives SSH
disconnects.

## Daily use

```bash
# Health check (fast, exit code-driven)
ssh evergreen ~/legacy-monitor/scripts/check-errors.sh

# Live error tail (alerts as soon as one appears)
ssh evergreen tail -f ~/legacy-monitor/errors.log

# Activity in a specific window — e.g. morning checkout routine
ssh evergreen ~/legacy-monitor/scripts/check-activity.sh 06:00 08:00

# Last hour of activity (no args)
ssh evergreen ~/legacy-monitor/scripts/check-activity.sh

# Stop everything
ssh evergreen ~/legacy-monitor/scripts/stop.sh
```

## Output files (on evergreen, in `~/legacy-monitor/`)

| File | Format | Use |
|---|---|---|
| `events.log`   | TSV: `timestamp \t event_name \t severity \t error_no \t app \t host \t session \t duration_ms \t sql_text \t error_msg` | full firehose, append-only |
| `errors.log`   | TSV (subset) | **EMPTY = HEALTHY**. Any line = .NET app threw an error |
| `activity.log` | text summary every minute | per-table write counts over time |
| `tail-loop.out`| stdout of the loop process | troubleshooting |
| `.cursor`      | last-seen UTC timestamp | persists across restarts |
| `tail-loop.pid`| PID of background loop | guards against double-start |

## Time-zone note

Logs are in **UTC**. The .NET app's `GETDATE()` is in **Bangkok local time
(UTC+7)**. So a 06:00–08:00 UTC window = 13:00–15:00 Bangkok local. For the
morning routine in Bangkok (06:00–08:00 local), filter by **23:00–01:00 UTC**.

## Answering the morning-room-status question

Question: does the .NET app write to the DB during the morning checkout flag
routine, or is it a UI-only state?

After the monitor has been running 24+ hours through one morning cycle:

```bash
# Find the morning Bangkok window in UTC (23:00-01:00 UTC = 06:00-08:00 BKK)
ssh evergreen ~/legacy-monitor/scripts/check-activity.sh 23:00 23:59
ssh evergreen ~/legacy-monitor/scripts/check-activity.sh 00:00 01:00
```

If you see writes (specifically UPDATEs to `HT_CheckIn_*`, `HT_Rooms`, or
`HT_Room_Status`) in that window → the app DOES write to the DB.
If the window is empty → the "check out pending" highlighting is UI-only.

## Disk impact

XE file: 4 × 100MB rolling = 400MB max on the legacy SQL Server box.
Logs on evergreen: ~1MB/hour at typical write rates, kept indefinitely until
manually rotated.

## Performance impact on legacy DB

- XE session: ~1-3% CPU overhead at typical write volume
- Polling: one query per minute against `sys.fn_xe_file_target_read_file` (incremental)
- The `EVENT_RETENTION_MODE = ALLOW_SINGLE_EVENT_LOSS` guarantees the session
  never blocks the .NET app even under load.

## Rollback if monitor causes problems

```bash
ssh evergreen ~/legacy-monitor/scripts/stop.sh
ssh evergreen 'cat ~/legacy-monitor/scripts/03-drop-session.sql | docker run --rm -i --network host --entrypoint /opt/mssql-tools18/bin/sqlcmd mcr.microsoft.com/mssql/server:2022-latest -C -S 192.168.100.222 -U sa -P "$DB_PASSWORD" -d master'
```
