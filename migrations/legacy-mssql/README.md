# Legacy MSSQL migrations

Schema changes that we apply to the **legacy MSSQL database**
(`192.168.100.222 / db` for HF Hotel; HF Ville's MSSQL behind the
WireGuard mesh) to support the event-driven sync architecture.

The legacy DB is **shared with the legacy .NET app** (per `CLAUDE.md`
and `docs/architecture.md` §11) — every change here is reviewed for
.NET-app compatibility before applying.

## Convention

```
NNN_phaseX_short-description.sql            -- apply
NNN_phaseX_short-description.rollback.sql   -- revert
```

* `NNN` continues the numbering of `migrations/pg/` so cross-phase
  references stay sequential. Phase 5 used numbers up to 019 in PG;
  Phase 5.5a added 020 in PG; Phase 5.5b is 021 here.
* Rollback file is order-sensitive (DISABLE CT → DROP PK → ALTER
  COLUMN nullable). Every statement is `IF EXISTS`-guarded so re-runs
  no-op cleanly.
* `GO` separators are mandatory between `ALTER COLUMN ... NOT NULL`
  and `ADD CONSTRAINT PRIMARY KEY` — without them SQL Server validates
  the PK against pre-ALTER metadata in the same batch and fails with
  Msg 8111. Learned 2026-04-28; applied throughout.

## Application

Both apply and rollback run via `sqlcmd` in a one-shot Docker
container on the deploy host (no need to install MSSQL tools locally):

```bash
ssh evergreen "cat <path-to-sql> | docker run --rm -i --network host \
    --entrypoint /opt/mssql-tools18/bin/sqlcmd \
    mcr.microsoft.com/mssql/server:2022-latest \
    -C -S 192.168.100.222 -U sa -P \"\$DB_PASSWORD\" -d db -W"
```

For HF Ville: same command, swap the `-S` server address for Ville's
MSSQL — `192.168.11.51,1436` over the WireGuard `hfville` interface
(after the 2026-04-29 cutover; see `ville_constraint.md` for the
network path). Database is `HOTEL`, not `db`.

## Coordination

These changes take Sch-M locks on the target tables. While the lock
is held, the legacy .NET app blocks on every transaction touching
those tables. Coordinate a maintenance window with receptionists at
**both sites** before applying. Memory: `legacy_db_state.md` in the
auto-memory tracks current state across runs.

## Files

| # | File | Date | What |
|---|------|------|------|
| 020 | `020_phase5_enable_ct.sql` / `.rollback.sql` | written 2026-04-29 (backfill); applied to HF Hotel manually 2026-04-25 | Phase 5 — DB-level CT + PKs + CT on the 11 canonical-sync tables (HT_Customers, HT_Rooms, HT_Book_H, HT_Book_Ds, HT_Book_Date, HT_CheckIn_H, HT_CheckIn_Ds, HT_CheckIn_Pay, HT_Room_Status, HT_Rooms_Cancel, HT_Receipt_H). At HF Hotel the equivalent statements were applied manually before this file existed; this captures them for new sites (HF Ville, future restores). DO NOT re-apply at HF Hotel — `ALTER DATABASE SET CHANGE_TRACKING ON` would error if already enabled. |
| 021 | `021_phase55b_enable_ct.sql` / `.rollback.sql` | 2026-04-28 | Phase 5.5b — PKs + CT on 6 legacy-only tables (HT_Cupon, HT_CheckIn_Product, HT_Deposit, HT_Changed_Room, HT_Bill_Debt_H, HT_Bill_Debt_Ds) for the legacy_mirror.\* schema (CT mappers in Phase 5.5c will populate) |
