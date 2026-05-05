# Legacy DB Reverse-Engineering Spike

Goal: capture every SQL statement the 3rd-party hotel app sends to its MSSQL DB
during a controlled set of receptionist actions, so we can later mirror those
writes from our app.

## Prerequisites

- Network access to the legacy MSSQL server (HF Hotel: `<legacy-mssql-host>:1433`)
- `sa` credentials (or any login with `ALTER ANY EVENT SESSION` server-level perm)
- `mssql-cli` or `sqlcmd` installed locally; `bash` for the runner script
- ~30 minutes of dedicated receptionist time (no other clerks using the app)
- A spare room or two to use as test inventory

## Files

| File | Purpose |
|---|---|
| `00-prereqs.sql`               | Sanity checks: server version, perms, DB name, identify 3rd-party app login |
| `01-baseline-schema.sql`       | Dump tables, columns, FKs, indexes, views, triggers, procedures, functions |
| `02-snapshot-rowcounts.sql`    | Row count + checksum per table (BEFORE) |
| `03-xe-create-session.sql`     | Create Extended Events session targeting the 3rd-party app's queries |
| `04-xe-start.sql`              | Start the XE session |
| `05-xe-stop.sql`               | Stop the XE session |
| `06-snapshot-rowcounts.sql`    | Row count + checksum per table (AFTER — diff vs 02 to see what changed) |
| `07-xe-read.sql`               | Pull the captured events out of the .xel file as a result set |
| `08-xe-drop-session.sql`       | Cleanup |
| `run.sh`                       | Wrapper: runs 00 + 01 + 02 + 03 + 04 (gets you ready for receptionist) |
| `finalize.sh`                  | Wrapper: runs 05 + 06 + 07 (after receptionist is done) |
| `ACTION-PLAYBOOK.md`           | Step-by-step receptionist actions with sentinel values |

## Workflow

```
1. SSH / RDP to a host with line of sight to <legacy-mssql-host>
2. Edit run.sh + finalize.sh to set DB_HOST / DB_USER / DB_PASS
3. ./run.sh                          # captures baseline + starts XE session
4. Hand ACTION-PLAYBOOK.md to receptionist; they perform actions
5. ./finalize.sh                     # stops XE, takes after-snapshot, exports events
6. Commit captured-{date}/ folder to a private repo for analysis
7. Diff before/after row counts → identify which tables each action touched
8. Cross-reference with XE event log → exact INSERT/UPDATE statements
```

## Output

After `finalize.sh` you'll have a `captured-YYYY-MM-DD-HHMM/` folder with:
- `00-prereqs.txt`        — server info + login identification
- `01-schema.txt`         — full schema dump
- `02-rowcounts-before.txt`
- `06-rowcounts-after.txt`
- `07-events.txt`         — every captured SQL statement with timestamp, login, duration
- `events.xel`            — raw Extended Events file (binary, openable in SSMS)

## Safety

- **All scripts are READ-ONLY against the legacy schema** except for `03-xe-create-session.sql`
  which creates a server-level Extended Events session. XE is observation-only and does not
  alter user data; it can be dropped cleanly with `08-xe-drop-session.sql`.
- The XE session writes to `xe_capture.xel` in the SQL Server's default log directory.
  At default 100MB max ring + 1 rollover, the worst-case disk impact is 200MB on the
  legacy server — drop the session if disk is tight.
- No `ALTER TABLE`, `DROP`, `CREATE INDEX` against existing legacy tables. Confirmed.
