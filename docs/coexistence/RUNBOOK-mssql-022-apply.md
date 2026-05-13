# Runbook — Apply legacy-mssql migration 022 (HT_CheckIn_Other_People CT)

> **Status:** UNAPPLIED on both sites as of 2026-05-13.
> **Owner of execution:** the human operator (receptionist coordination required).
> **Estimated downtime:** sub-second per site (Sch-M lock during PK creation on a small table).

## Why this is needed

Track E1 (the audit-2026-05-13 sync-gap closure) shipped a Rust mapper
`GuestRegistryMapper` that mirrors legacy `HT_CheckIn_Other_People` rows
into canonical `ht_guest_registry`. The mapper depends on per-table
Change Tracking being enabled on the legacy side. Migration
`migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql` is
the SQL that enables CT (and adds the required PK).

The Rust mapper has been live since v2.63.12 but the MSSQL migration
was never applied, so the `bin/sync.rs` watcher logs
`Change tracking is not enabled for table HT_CheckIn_Other_People`
roughly once per second on both sites, and `ht_guest_registry` does not
accrue companion-guest rows. TM.30 immigration reporting under-counts
foreign guests until this is applied.

This is the **17th** Phase-5-style PK+CT migration. The previous 16
were applied in two batches on 2026-04-25 and 2026-04-29 with the same
pattern (`legacy_db_state` memory). The receptionist tolerated those
without complaint.

## Pre-flight checks

1. **Receptionist coordination.** The `ALTER TABLE ... ADD CONSTRAINT
   ... PRIMARY KEY CLUSTERED` operation takes a Sch-M lock on
   `HT_CheckIn_Other_People` for the duration of the statement.
   Empirically this is sub-second on a small table (the entire table is
   one short companion-guest row per check-in). Pick a quiet window
   (no in-flight check-in or edit on the iHOTEL UI) and confirm with
   the receptionist via the usual channel before proceeding.

2. **Backup verification.** The MSSQL nightly backup is the rollback of
   last resort. Confirm the most recent successful backup before
   touching the schema — but the migration's own `.rollback.sql`
   (re-runnable) should be the first line of defense.

3. **Set the password.** Export `DB_PASSWORD` in the shell you're
   running from (the value is in 1Password under the legacy-MSSQL
   `sa` entry):

       export DB_PASSWORD='<value>'

4. **Tunnel for HF-Ville only.** HF-Ville's MSSQL is reachable via the
   `hfville` WireGuard interface. Bring it up first:

       sudo wg-quick up hfville
       ping -c1 192.168.11.51   # must succeed before continuing

   HF-Hotel's MSSQL is on the LAN — no tunnel needed.

## Apply commands

The apply pattern is the same as Phase 5 migrations (memory
`legacy_db_state`). Run from the project root.

### Site 1 — HF Hotel

Connect string: `FRONT2\SQLEXPRESS` (the value of `DB_HOST` in
`.env.example`). Database name: `db` (the value of `DB_NAME` in
`.env.example`).

    cat migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql | \
      docker run --rm -i --network host \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        mcr.microsoft.com/mssql/server:2022-latest \
        -C -S 'FRONT2\SQLEXPRESS' -U sa -P "$DB_PASSWORD" -d db -W

Expected output: three statements complete, then a one-row select
showing `HT_CheckIn_Other_People | YES | PK_HT_CheckIn_Other_People`.

### Site 2 — HF Ville

Connect string: `192.168.11.51,1436`. Database name: `HOTEL`.

    cat migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql | \
      docker run --rm -i --network host \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        mcr.microsoft.com/mssql/server:2022-latest \
        -C -S '192.168.11.51,1436' -U sa -P "$DB_PASSWORD" -d HOTEL -W

## Verification queries

### Pre-apply (BEFORE running the migration above)

The CT-enabled-tables list must NOT include `HT_CheckIn_Other_People`:

    SELECT t.name
      FROM sys.change_tracking_tables ct
      JOIN sys.tables t ON ct.object_id = t.object_id
     ORDER BY t.name;

If `HT_CheckIn_Other_People` already appears here, **STOP** — the
migration was applied through some other path and we'd be re-enabling
CT on a table that already has it (the script is guarded but you should
know why before proceeding).

### Post-apply

Same query as above. `HT_CheckIn_Other_People` MUST now appear.

Then confirm the PK exists:

    SELECT name
      FROM sys.indexes
     WHERE object_id = OBJECT_ID('HT_CheckIn_Other_People')
       AND is_primary_key = 1;

Expected: one row, `PK_HT_CheckIn_Other_People`.

### Post-apply — PG side

The CT watcher (`bin/sync.rs`) will start ingesting `HT_CheckIn_Other_People`
on its next tick (sub-second). Watch `legacy_sync_status`:

    SELECT entity_type, last_sync_at, consecutive_failures, last_error
      FROM legacy_sync_status
     WHERE entity_type = 'HT_CheckIn_Other_People';

Expected over the next 5 minutes:

* `consecutive_failures` drops from ~2230 toward 0
* `last_sync_at` advances on each tick
* `last_error` becomes NULL (or stale text from the pre-apply window)

And the canonical mirror starts accruing:

    SELECT count(*), max(created_at)
      FROM ht_guest_registry
     WHERE guest_legacy_id IS NOT NULL;

`count(*)` should rise as new companion-guest rows are inserted on the
legacy side; `max(created_at)` advances.

## Rollback

The companion rollback file is `migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.rollback.sql`.
It is idempotent (every statement is guarded), so it is safe to run
multiple times.

Use it if:

* The .NET iHOTEL app misbehaves immediately after apply (e.g. the new
  PK conflicts with existing duplicate `id` values — `migration 022`
  did NOT add a `WHERE EXISTS dup` pre-check; if duplicates exist the
  ADD CONSTRAINT statement will fail loudly and the rollback isn't
  strictly necessary, but run it anyway to leave the schema clean).
* The watcher's first ingestion tick consumes excessive resources on
  the legacy MSSQL host (very unlikely — `HT_CheckIn_Other_People` is
  tiny and CT scanning is cheap).

Apply commands mirror the forward pattern:

    # HF Hotel
    cat migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.rollback.sql | \
      docker run --rm -i --network host \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        mcr.microsoft.com/mssql/server:2022-latest \
        -C -S 'FRONT2\SQLEXPRESS' -U sa -P "$DB_PASSWORD" -d db -W

    # HF Ville
    cat migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.rollback.sql | \
      docker run --rm -i --network host \
        --entrypoint /opt/mssql-tools18/bin/sqlcmd \
        mcr.microsoft.com/mssql/server:2022-latest \
        -C -S '192.168.11.51,1436' -U sa -P "$DB_PASSWORD" -d HOTEL -W

## What success looks like

Within 5 minutes of apply on both sites:

* `sync` container logs (`docker logs sync-hfhotel` and `sync-hfville`)
  stop emitting `'Change tracking is not enabled'` lines for
  `HT_CheckIn_Other_People`.
* `legacy_sync_status` row for `HT_CheckIn_Other_People` shows
  `consecutive_failures = 0` and `last_sync_at` advancing.
* `ht_guest_registry` starts accruing rows for companion guests
  registered on the legacy side post-apply.
* No new Slack alerts from the watcher about this table.

## After-apply housekeeping

Update memory `legacy_db_state` to reflect 17 tables CT-enabled (was
16). Add a sentence: "2026-05-13: HT_CheckIn_Other_People PK+CT applied
both sites — Track E1 mapper now active." No code changes needed —
the mapper has been waiting for this since 2.63.12.
