# Legacy MSSQL migrations

Schema changes that we apply to the **legacy MSSQL database**
(`<legacy-mssql-host> / db` for HF Hotel; HF Ville's MSSQL behind the
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

## Application (automated — since 2026-06-24)

These migrations are now **auto-applied by the CI/CD deploy**, the same
way `migrations/pg/` are. `scripts/deploy/run-deploy.sh` runs
`scripts/migrate-legacy-mssql.sh --site hfhotel` and `--site hfville` on
every deploy, then a **pre-worker CT gate** (`--verify-ct`, fed
`sync --print-ct-tables`) confirms every table the binary expects CT on
is actually CT-enabled on both servers before the watcher starts. A miss
fails the deploy.

* **Tracking:** `dbo.ht_legacy_migrations` (version, filename, checksum,
  applied_at, applied_by) on each legacy server — the MSSQL analog of
  PG's `schema_migrations`. Only PENDING migrations (not already in the
  table) run, so re-deploys are no-ops. Servers already at the current
  level were seeded once via `--adopt-baseline` (020–023 recorded
  `baseline-adopt`, 2026-06-24).
* **Safety:** the runner sets a bounded `SET LOCK_TIMEOUT`
  (`LEGACY_LOCK_TIMEOUT_MS`, default 5000) so a migration that hits a
  busy table fails fast (error 1222) and halts the deploy instead of
  holding a Sch-M lock that would block the live iHOTEL app.
* **NEW migrations MUST be idempotent** (`IF NOT EXISTS` / `IF COL_LENGTH(...)
  IS NULL` guards), the same discipline `CLAUDE.md` mandates for
  `migrations/pg/` — the tracking row is written only after the body
  succeeds, so a partial failure is retried on the next deploy.
* **Defense in depth:** the sync binary itself probes CT enablement at
  startup and refuses to start (one Slack alert, no 1/sec spam) if an
  expected table lacks CT — overridable with `LEGACY_SYNC_ALLOW_CT_GAP=true`.

### Manual application (fallback / one-off)

Still possible via `sqlcmd` in a one-shot Docker container on the deploy
host (no MSSQL tools needed locally):

```bash
ssh evergreen "cat <path-to-sql> | docker run --rm -i --network host \
    --entrypoint /opt/mssql-tools18/bin/sqlcmd \
    mcr.microsoft.com/mssql/server:2022-latest \
    -C -S <legacy-mssql-host> -U sa -P \"\$DB_PASSWORD\" -d db -W"
```

For HF Ville: same command, swap the `-S` server address for Ville's
MSSQL — `<ville-mssql-host>,1436` over the WireGuard `hfville` interface
(after the 2026-04-29 cutover; see `ville_constraint.md` for the
network path). Database is `HOTEL`, not `db`. A manual apply should be
followed by recording the row in `dbo.ht_legacy_migrations` (or just let
the next deploy's runner skip it — it keys on `version`).

## Coordination

These changes take Sch-M locks on the target tables. The runner's bounded
`LOCK_TIMEOUT` keeps that wait short (fails fast rather than blocking the
.NET app), but a large/long DDL still belongs in a maintenance window —
prefer landing such a migration in a deploy timed for a quiet period, and
coordinate with receptionists at **both sites**. Memory: `legacy_db_state.md`
in the auto-memory tracks current state across runs.

## Files

| # | File | Date | What |
|---|------|------|------|
| 020 | `020_phase5_enable_ct.sql` / `.rollback.sql` | written 2026-04-29 (backfill); applied to HF Hotel manually 2026-04-25 | Phase 5 — DB-level CT + PKs + CT on the 11 canonical-sync tables (HT_Customers, HT_Rooms, HT_Book_H, HT_Book_Ds, HT_Book_Date, HT_CheckIn_H, HT_CheckIn_Ds, HT_CheckIn_Pay, HT_Room_Status, HT_Rooms_Cancel, HT_Receipt_H). At HF Hotel the equivalent statements were applied manually before this file existed; this captures them for new sites (HF Ville, future restores). DO NOT re-apply at HF Hotel — `ALTER DATABASE SET CHANGE_TRACKING ON` would error if already enabled. |
| 021 | `021_phase55b_enable_ct.sql` / `.rollback.sql` | 2026-04-28 | Phase 5.5b — PKs + CT on 6 legacy-only tables (HT_Cupon, HT_CheckIn_Product, HT_Deposit, HT_Changed_Room, HT_Bill_Debt_H, HT_Bill_Debt_Ds) for the legacy_mirror.\* schema (CT mappers in Phase 5.5c will populate) |
| 022 | `022_phase5e_other_people_rooms_cancel.sql` / `.rollback.sql` | 2026-05-13 | Track E1 / T2 HIGH-3 — PK + CT on `HT_CheckIn_Other_People` so the new `GuestRegistryMapper` can sync companion-guest entries into canonical `ht_guest_registry` (TM.30 immigration compliance). HT_Rooms_Cancel was already CT-enabled in 020; no new MSSQL DDL needed for it — only the missing mapper (see `sync/mappers/mirror.rs::RoomsCancelMirrorMapper`). |
| 023 | `023_book_pro_ct.sql` / `.rollback.sql` | 2026-06-12 | Phase 5/E2 (coexistence audit 2026-06-11 P2) — PK + CT on `HT_Book_Pro` (pre-booked products attached to a booking by FrmAddBook2) so the new `BookProMirrorMapper` (`sync/mappers/mirror.rs`) can mirror rows into `legacy_mirror.ht_book_pro` (migration pg/056 creates the mirror table + seeds `legacy_sync_status` / `legacy_ct_state_per_table`). `id` is IDENTITY and already NOT NULL per the live baseline, so no ALTER COLUMN batch — same shape as 021's HT_CheckIn_Product entry. Apply BEFORE deploying the binary that adds the table to `CT_ENABLED_TABLES`. |
| 024 | `024_writeback_ledger.sql` / `.rollback.sql` | 2026-06-24 | App-owned `dbo.ht_writeback_ledger` — the crash-after-commit duplicate guard for the writeback worker's sequential-allocator create recipes (the ones minting a legacy id app-side via MAX+1). The recipe INSERTs one ledger row keyed on `writeback_jobs.idempotency_key` IN-TX with the create, so a replay after a crash between the MSSQL commit and the PG mark probes the ledger and skips the duplicate INSERT (reusing the recorded `legacy_ids`). NOT an iHOTEL `HT_*` table — no CT, no `HT_*` DDL, invisible to the schema fingerprint / CT gate; same `dbo.*` posture as `ht_legacy_migrations`. Apply BEFORE the worker is recreated. |
