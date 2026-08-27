# Claude Code Instructions

## Agent skills

### Issue tracker

Issues live in GitHub Issues (thehfhotel/new-hotel) via the `gh` CLI; external PRs are NOT a triage/request surface. See `docs/agents/issue-tracker.md`.

### Triage labels

Canonical defaults (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`); only `wontfix` pre-exists, others created on first use. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one root `CONTEXT.md` (lazy-created) + `docs/adr/`. See `docs/agents/domain.md`.

## Target Architecture (READ FIRST)

**Source of truth:** [`docs/architecture.md`](docs/architecture.md). Read it before designing or implementing.

Summary:
- Stay-current stack: Rust+Axum backend, Next.js 16 frontend, PostgreSQL, legacy MSSQL.
- PostgreSQL is the source of truth from day one. iHOTEL/legacy MSSQL **coexists indefinitely — no planned decommission (ADR 0002)**; we mirror to/from it both ways. Both apps write both sites, kept consistent by the sync.
- Layered architecture inside the decommission boundary (the same clean seam that makes durable coexistence safe): `domain/` → `repository/` (PG-only) → `service/` (business logic + outbox emission) → thin `routes/`.
- Adapter workers OUTSIDE the boundary: `bin/writeback.rs` (LISTEN'er → MSSQL via tiberius), `bin/sync.rs` (Change Tracking watcher → publishes events; serves both HF Hotel and HF Ville via per-site env).
- Event-driven sync via PG `LISTEN/NOTIFY` + SQL Server Change Tracking. Sub-second latency target. Runs **permanently** (coexistence), not "until decommission".
- Operational states are `.env` toggles: **State B (both apps coexist) is the permanent target**; State A (legacy-primary) is historical; **State C (our app only) is a dormant, unplanned capability** — iHOTEL is not being decommissioned (ADR 0002). No code changes between states.

**Companion docs:**
- `docs/legacy-spike/findings.md` — validated SQL recipes for every writeback flow. Don't re-derive.
- `docs/legacy-app/` — iHOTEL coexistence reference (`COMPAT_CHEATSHEET.md`, `FEATURE_MAP.md`, `REPORTS_INVENTORY.md`, `SCHEMA.sql`) derived from the legal de4dot+ilspycmd decompile. Authoritative when canonical PG state and iHOTEL displays disagree. The off-repo vendor binaries live on evergreen — see `docs/legacy-app/EVERGREEN_ARTIFACTS.md`.

## Versioning & Changelog Policy

**As of v2.67+, versioning is automated via release-please.**

1. **DO NOT edit CHANGELOG.md or package.json in feature PRs.** release-please reads conventional commits and auto-aggregates into a Release PR.
2. **Conventional commit message** is now the load-bearing artifact. Format: `<type>(<scope>): <description>`. Types that surface in changelog: `feat:` → Added, `fix:` → Fixed, `perf:`/`refactor:` → Changed. Hidden types: `chore:`, `docs:`, `test:`, `style:`, `build:`, `ci:`.
3. **Breaking changes**: add `!` (e.g., `feat!:`) or include `BREAKING CHANGE:` in body. Triggers MAJOR bump.
4. **Release flow**: release-please opens a "chore(main): release X.Y.Z" PR on every push to master. Merging that PR bumps the version + writes CHANGELOG + creates a git tag.
5. **First-time bootstrap**: the existing manually-edited CHANGELOG entries through v2.66.4 remain. release-please picks up from there.

## Deployment Policy

**MANDATORY**: All deployments to production MUST go through the CI/CD pipeline.

1. **Never deploy manually** - Do not run `docker compose up` or `docker pull` manually on production
2. **Pipeline is the only way** - All changes must be committed, pushed, and deployed via GitHub Actions
3. **Workflow**: `.github/workflows/docker-build.yml` handles:
   - Running tests
   - Building Docker images (frontend + backend)
   - Pushing to GitHub Container Registry (ghcr.io/thehfhotel/*)
   - Deploying to production server via cloudflared SSH from a GH-hosted runner
     (Phase 1 modernization, v2.57.x — self-hosted runner has been retired)

4. **To deploy**: Simply `git push` to master - the pipeline handles everything automatically

5. **Deploy script (`run-deploy.sh`) is version-controlled** (Track J5, 2026-05-14):
   - The repo copy at `scripts/deploy/run-deploy.sh` is the source of truth.
   - Edit it IN THE REPO — never on the host directly.
   - The workflow bundles it in the deploy tarball; the live `/srv/run-deploy.sh`
     on evergreen self-updates from the repo on every deploy (effective next deploy).
   - Forced-command SSH config on evergreen blocks scp/rsync; the self-update
     tail block in the script itself is the sync mechanism.

## Project Structure

- `/app` - Next.js App Router pages (frontend only, no API routes except /api/changelog)
- `/components` - React components
- `/hotel-backend` - Rust/Axum backend API server
- `/__tests__` - Jest test files (component tests only)

## Database Architecture

This application uses a **dual-database architecture**:

| Database | Location | Purpose |
|----------|----------|---------|
| Legacy DB | <legacy-mssql-host> | Shared with legacy app, READ-ONLY |
| HotelNew DB | Docker container (`newdb`) | Self-hosted, full CRUD |

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    DOCKER COMPOSE                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Frontend   │  │   Backend   │  │   PostgreSQL        │  │
│  │  (web)      │  │  (backend)  │  │   (newdb)           │  │
│  │  Port 3003  │  │  Port 3003  │  │   Port 5439         │  │
│  │             │  │             │  │                     │  │
│  │             │──│─────────────│──│──▶ HotelNew DB      │  │
│  └─────────────┘  │             │  │                     │  │
│                   │─────────────│──│──▶ (internal only)  │  │
│                   └─────────────┘  └─────────────────────┘  │
│                          │                                   │
└──────────────────────────│───────────────────────────────────┘
                           │
                           ▼
              ┌─────────────────────────┐
              │  Legacy DB              │
              │  <legacy-mssql-host>:1433   │
              │  (external, read-only)  │
              └─────────────────────────┘
```

### Self-Hosted Database (HotelNew)

The HotelNew database runs in a Docker container (`newdb` service):
- **Image**: `postgres:17-alpine` (~100MB image, ~50-100MB RAM idle)
- **Rust driver**: `sqlx` crate with `query!()` compile-time macros (offline mode via `.sqlx/` cache)
- **Data persistence**: Docker volume `newdb_data`
- **Access**: Internal only (not exposed to host network)
- **Initialization**: Automatic on first startup (PostgreSQL runs `.sql` files from `/docker-entrypoint-initdb.d/`)

**First-time setup**: Fully automatic. Just run `docker compose up` - PostgreSQL auto-creates the database and runs `init-db/init-hotelnew.sql`.

### Credentials & Docker secrets (security audit 2026-05-14)

Sensitive credentials (`DB_PASSWORD`, `POSTGRES_PASSWORD`, `VILLE_DB_PASSWORD`, `SLACK_WEBHOOK_URL`) are read from files under `/run/secrets/` inside the container, NOT from environment variables. This prevents `docker exec <container> printenv` from leaking them — the original audit finding.

- **Compose**: top-level `secrets:` block declares each file under `${SECRETS_DIR:-/home/deploy/secrets}/<name>` and each service mounts the subset it needs.
- **Rust hydration**: `hotel-backend/src/secrets.rs::hydrate_env_from_secret_files` is the FIRST call in every binary's `main()`. It reads the file (stripping a trailing newline) and populates the matching env var IN-PROCESS only. Existing `env::var(...)` call sites (`config.rs`, `bin/sync.rs`, `bin/writeback.rs`) keep working unchanged.
- **`DATABASE_URL` reconstruction**: `sync` / `writeback` / `backfill_*` bins still read `DATABASE_URL`, but it's no longer pre-baked in the compose file. The hydrator builds it from `POSTGRES_USER` + `POSTGRES_PASSWORD` (secret) + `NEW_DB_SERVER` + `NEW_DB_PORT` + `NEW_DB_NAME` (or `POSTGRES_DB`). HF Ville workers override `NEW_DB_NAME=hotelville` to target the sibling logical DB.
- **PostgreSQL**: `newdb` service uses the stock `POSTGRES_PASSWORD_FILE` env var (a feature of the official image's entrypoint) pointing at `/run/secrets/postgres_password`.
- **Local dev**: set `SECRETS_DIR=$PWD/.secrets` (or any directory of your choice) and populate that directory with files matching the names in the top-level `secrets:` block. Alternatively, set `DB_PASSWORD=...` in `.env` — `dotenvy::dotenv()` runs before the hydrator, so the env var wins.
- **Rollback**: a single `git revert` of the secret-files-migration commit restores the pre-2026-05-14 `environment:` blocks. No state to clean up.

**Required secret files on evergreen** (must exist BEFORE the compose stack starts):
- `/home/deploy/secrets/db_password` — legacy MSSQL `sa`
- `/home/deploy/secrets/postgres_password` — newdb superuser
- `/home/deploy/secrets/ville_db_password` — alias of `postgres_password` until HF Ville gets a distinct DB credential (coexistence hardening — ADR 0002; formerly tied to the now-superseded "ADR 0001 Phase 8")
- `/home/deploy/secrets/slack_webhook_url`
- `/home/deploy/secrets/ota_bridge_token` — OTA booking-bridge shared bearer (`docs/ota-bridge.md`). **INVARIANT: must hold the IDENTICAL string as ota-desk's `PMS_BRIDGE_TOKEN`** — two names for one value, same idiom as `PORTAL_NOTIFY_TOKEN` ≡ portal `NOTIFY_INGRESS_TOKEN`. Verify without exposing it: both repos print `sha256(token)[0..6]` (here, in the `OTA bridge: …` startup line). An unset GH secret yields an EMPTY file, which the hydrator treats as absent — the gate then has nothing to accept and `/api/ota/*` fails closed.
- `/home/deploy/secrets/ota_bridge_token_previous` — rotation slot for the above; accepted with a "finish the rotation" WARN. Normally empty.
- `/home/deploy/secrets/hfid_resolve_secret` → `HFID_RESOLVE_SECRET` — shared secret for the `/hk` employee-location lookup (`hotel-backend/src/hfid_location.rs`), sent as the `X-Reader-Secret` header on `POST {HFID_LOCATION_URL}`.

  **`HFID_RESOLVE_SECRET` carries the same value as fingerprint-time-logger's `READER_RESOLVE_SECRET`.** HF ID guards its ENTIRE app↔central surface — `/resolve`, `/resolve-badge`, `/claim`, `/wait` — with that one secret, so there is no separate upstream credential to find; do not go hunting for one. It is therefore also the same value as this repo's own `READER_RESOLVE_SECRET` (card-login pairing, `service::reader`). Three names, one secret:

  | name | where | consumer |
  |---|---|---|
  | `READER_RESOLVE_SECRET` | fingerprint-time-logger (HF ID) | the authority — guards its whole app↔central surface |
  | `READER_RESOLVE_SECRET` | new-hotel | card-login pairing (`/claim`, `/wait`) |
  | `HFID_RESOLVE_SECRET` | new-hotel | `/hk` badge→location lookup (`/resolve-badge`) |

  The distinct new-hotel name is deliberate: it lets the two consumers be rotated independently later without a code change, and it keeps a `/hk` outage from being indistinguishable from a card-login outage. An unset GH secret yields an EMPTY file, which the hydrator treats as absent — no lookup client is built, and with `HK_LOCATION_ENFORCEMENT_ENABLED` on every `/hk` request fails closed with `503` (never a fallback to the `HK_BRANCHES` allowlist).

The deploy script (`/srv/run-deploy.sh` on evergreen, NOT in this repo) writes these from the JSON payload's `.secrets` block on every deploy, with mode `0400` and owner `deploy:docker`.

### SQLx Offline Mode (`.sqlx/` Cache)

The backend uses `sqlx::query!()` compile-time macros for ~76 static SQL queries. These macros verify queries against the PostgreSQL schema at compile time, catching typos and type mismatches before runtime.

**How it works**:
- `hotel-backend/.sqlx/` contains JSON cache files for each `query!()` call
- When `SQLX_OFFLINE=true` is set (Docker builds, CI), sqlx reads from this cache instead of connecting to a live database
- Dynamic queries using `sqlx::query()` (string concatenation) don't need the cache

**MANDATORY**: When modifying any `sqlx::query!()` SQL in the backend:
1. Ensure a PostgreSQL database is running with the current schema
2. Set `DATABASE_URL` environment variable (e.g., `postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew`)
3. Run `scripts/sqlx-prepare.sh` (or manually: `cd hotel-backend && cargo sqlx prepare`)
4. Commit the updated `.sqlx/` directory

**When to regenerate**:
- After changing any SQL string inside `query!()`, `query_scalar!()`, or `query_as!()`
- After schema changes (new columns, renamed tables, type changes)
- After adding new `query!()` calls

**DECIMAL column handling**:
- SELECT: Cast to `::float8` for `f64` return (e.g., `r.room_price_weekday::float8`)
- INSERT/UPDATE parameters: Cast with `$N::float8` so sqlx accepts `f64` and PostgreSQL converts to NUMERIC

### Legacy Database (<legacy-mssql-host>)

- **SHARED DATABASE**: Used by another legacy application. Exercise caution.

**Legacy tables** (READ-ONLY - do not modify schema):
- `HT_Rooms` - Room information
- `View_Booking_Ds` - Booking records
- `View_CheckIn_Ds` - Check-in records
- `View_Customers` - Customer information

**PROHIBITED on legacy tables/views:**
- `ALTER TABLE` / `ALTER VIEW` - Do not modify columns or definitions
- `DROP TABLE` / `DROP VIEW` - Do not delete
- `CREATE INDEX` on legacy tables - May affect legacy app

**Sole standing exception (2026-06-11 audit carve-out):** the Change Tracking
prerequisite DDL in `migrations/legacy-mssql/` (020/021/022) — `ALTER COLUMN … NOT
NULL` + `ADD CONSTRAINT … PRIMARY KEY CLUSTERED` on the CT-enabled tables, applied
2026-04..05 in Sch-M maintenance windows with pre-flight NULL/duplicate checks. CT
requires a PK; no other legacy DDL is permitted. Residual hazard to remember: iHOTEL
allocates ids app-side (MAX+1, race-prone) — a duplicate-id race that used to succeed
silently now hard-fails iHOTEL's INSERT on the PK. If a receptionist reports a save
error in iHOTEL, check for a concurrent same-table save first.

That hazard is **per-table, and only where the id is not an IDENTITY column** — check
`sys.columns.is_identity` before assuming it applies. On the two busiest folio tables
they differ: `HT_CheckIn_Ds.id` is `is_identity = 0`, so iHOTEL supplies the value and
the race is real; `HT_CheckIn_Pay.id` is `is_identity = 1`, so SQL Server allocates it,
iHOTEL's INSERT omits the column, and no duplicate-id race is possible there at all
(verified against HF Ville, 2026-08-19).

Relatedly, **"has this intent ever run?" is answered from canonical `writeback_jobs`,
never from `dbo.ht_writeback_ledger`** — the ledger only records create-writebacks, so
non-ledgered intents (ExtendStay, MarkRoomClean/Dirty) never appear there and an empty
ledger lookup proves nothing.

**Before hand-editing ANY legacy value (not just schema) — read this.** Two guards
exist, one accepted gap sits between them, and one column is a live lock:

1. **Schema is guarded automatically.** `hotel-backend/src/writeback/fingerprint.rs`
   hashes the column shapes of every legacy table we touch on startup and **refuses to
   boot** on a mismatch. A hand-applied `ALTER` will stop the workers, not corrupt data.
   Re-capture via `scripts/writeback-fingerprint.sh` if a change is ever legitimate.
2. **NEVER hand-write SQL `NULL` into `HT_Book_H.Book_Cust_ID` or
   `HT_CheckIn_H.Cin_cust_no`.** Our sync cannot represent a legacy value being
   *cleared* (it is indistinguishable from "not observed"), so canonical would keep the
   stale customer id **forever** and the resulting reconcile row **can never
   auto-close** — a permanent, unfixable page. iHOTEL itself never does this: its
   customer-delete cascade writes the reserved sentinel `'C0000'` instead, which is
   Some→Some and handled correctly. Audited 2026-07-31: zero NULLs on both columns at
   both sites. A per-tick tripwire alerts if that ever changes. If you genuinely must
   clear a customer link, **write `'C0000'`, never NULL.**
   Full analysis and the designed (deliberately unbuilt) fix:
   `docs/adr/0005-null-clear-sentinel-semantics.md`.
3. **NEVER write `HT_CheckIn_H.Cin_Work_number` casually** — it is not vestigial and
   not a TM.30 batch number. It is iHOTEL's per-folio **optimistic-lock token**,
   taken on folio LOAD by five reception forms and re-checked on save; changing it
   makes the next save show `มีการแก้ไข … จากเครื่องอื่น`, close the form, and
   **discard the receptionist's in-progress edit**. Exactly one recipe writes it on
   purpose (`extend_stay`); no other may without its own decision record. Detail,
   call sites and caveats: `docs/legacy-app/COMPAT_CHEATSHEET.md` §7.4. That decision
   record — why the one write stays, why the other six recipes deliberately do not bump
   the token, and why the lock must never be treated as a mutex — is
   `docs/adr/0007-folio-lock-participation.md` §"Decision". Read it before widening the
   write; do not infer permission from this bullet.

### HotelNew Tables (owned by this app, PostgreSQL - all lowercase)

All tables in the HotelNew database are owned by this application:
- `ht_customers` - Customer master data
- `ht_room_types` - Room type definitions
- `ht_rooms_new` - Room inventory
- `ht_bookings` - Booking records
- `ht_booking_rooms` - Booking-room assignments
- `ht_checkins` - Check-in records
- `ht_guest_registry` - Guest registration (TM.30)
- `ht_rates` - Room rate configurations
- `ht_settings` - System settings
- `ht_booking_notes` - Booking annotations
- `ht_inventory_categories` - Inventory categories
- `ht_inventory_items` - Inventory items
- `ht_inventory_transactions` - Inventory transactions
- `ht_room_inventory` - Room inventory assignments
- `ht_payments` - Payment records
- `ht_maintenance_categories` - Maintenance categories
- `ht_maintenance_requests` - Maintenance requests
- `schema_migrations` - Migration version tracking

### Database Migrations

Migrations are **automated** via `scripts/migrate.sh`, which runs during CI/CD deployment.

**MANDATORY**: When making ANY HotelNew schema changes:

1. **Create a migration file** in `migrations/pg/`:
   - Name format: `NNN_description.sql` (e.g., `001_add_customer_notes.sql`)
   - Include both UP and DOWN (rollback) migrations
   - Use `IF NOT EXISTS` to make migrations idempotent

2. **Update `init-db/init-hotelnew.sql`** with the same changes (for fresh deployments)

3. **Update the migrations README** (`/migrations/README.md`):
   - Add entry to the PostgreSQL migrations table
   - Document any new tables in "Tables Owned by This Application"

4. **Never auto-create tables in code** without a corresponding migration file

5. **Example migration structure** (PostgreSQL):
   ```sql
   -- Migration: 001_description
   -- Version: x.x.x
   -- Date: YYYY-MM-DD

   -- UP MIGRATION
   CREATE TABLE IF NOT EXISTS ...

   -- DOWN MIGRATION (commented)
   -- DROP TABLE IF EXISTS ...
   ```

The pipeline will automatically: create a backup, apply pending migrations in transactions, and track them in the `schema_migrations` table. See `migrations/README.md` for details.

### Legacy-MSSQL migrations are also automated (since 2026-06-24)

`migrations/legacy-mssql/` (Change-Tracking prerequisite DDL on the shared
legacy SQL Server) is **no longer applied by hand**. The deploy
(`scripts/deploy/run-deploy.sh`) runs `scripts/migrate-legacy-mssql.sh` for both
sites, then a **pre-worker CT gate** verifies every table the binary expects CT
on (from `sync --print-ct-tables` — single source of truth) is actually
CT-enabled on both servers, failing the deploy otherwise. Tracking is
`dbo.ht_legacy_migrations` on each server (analog of `schema_migrations`).

When adding a CT-enabled table:
1. Add the `NNN_*.sql` to `migrations/legacy-mssql/` — **idempotent** (`IF NOT
   EXISTS` guards); `GO` between `ALTER COLUMN NOT NULL` and `ADD CONSTRAINT PK`.
   The runner sets a bounded `LOCK_TIMEOUT` so a busy table fails fast instead of
   blocking the live iHOTEL app.
2. Add the table to `CT_ENABLED_TABLES` in `hotel-backend/src/bin/sync.rs` (the
   `--print-ct-tables` / CT-gate contract) **and** wire its mapper.
3. The deploy applies the migration before starting workers; the binary also
   self-guards at startup (refuses to start with one alert, not 1/sec spam, if a
   table lacks CT — override `LEGACY_SYNC_ALLOW_CT_GAP=true`).

This closed the 2026-06-24 incident where a binary shipped ahead of its
`023_book_pro_ct.sql` prerequisite. See `migrations/legacy-mssql/README.md`.

**Not all legacy sync rides Change Tracking.** Low-volume ledgers can instead be
mirrored by a plain per-tick read-only poll. `HT_Round_Bill` (iHOTEL cashier
rounds / รอบบิล → canonical `public.ht_shifts`) is the first such case:
`sync.rs::sync_round_bills` SELECTs it each tick and runs outside the CT-mapper
loop — it is **deliberately NOT in `CT_ENABLED_TABLES`** (no PK/CT prerequisite
DDL on the legacy table, so nothing in `migrations/legacy-mssql/`). Co-equal
open/close write-back to `HT_Round_Bill` is **shipped dark** behind
`ROUND_WRITEBACK_ENABLED` (default off — zero legacy writes pending a
reception-coordinated live test); recipe in `writeback/recipes/round_bill.rs`,
design in `docs/coexistence/ville-coequal-writes-plan.md`.

### Vocabulary note: "drift" vs. "sync lag"

The table `ht_reconcile_log` reads as a divergence ledger, but the actual semantic is a **sync-lag observation queue**. Rows are snapshots of moments when the diff-only sweep noticed two hashes had not yet converged; the auto-resolve sweep at every reconcile tick re-hashes both sides and closes converged rows. The CT watcher and writeback worker do the real reconciliation; the sweep is just an auditor that notices when those engines are temporarily behind.

When discussing this table in code comments, runbooks, or alert templates, prefer **"sync lag" / "unconverged"** over "drift" / "divergence" — the latter implies a durable state that needs operator action, when in practice most rows clear on their own within one tick. The Slack templates in `scheduler::sync` and the `COMMENT ON TABLE` in migration 054 capture this framing. The table itself is not renamed because the blast radius (queries, alerts, dashboards, runbooks, incident links) was disproportionate to the one-time conceptual-clarity benefit.

The only rows that represent actual durable divergence are those that resist multiple sweep cycles. The 4h-unconverged Slack alert (`level_drift_alert`) is calibrated for exactly that case.

### Timezone Handling
- SQL Server stores datetime values in **local Thai time (GMT+7)** without timezone information
- The `mssql` library returns datetime fields as ISO strings with `Z` suffix (e.g., `2026-01-22T11:59:00.000Z`)
- JavaScript interprets `Z` as UTC, but the actual values are already in Thai time
- **When formatting dates for display**: Use `timeZone: 'UTC'` to show the stored value as-is
  ```typescript
  new Date(dateValue).toLocaleTimeString('th-TH', {
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC',  // Shows stored value without conversion
  })
  ```
- **Do NOT use** `timeZone: 'Asia/Bangkok'` - this would add 7 hours to the already-local time

## Testing

Run tests before committing: `npm test`

## Architecture

This application uses a **split architecture**:

| Component | Technology | Description |
|-----------|------------|-------------|
| Frontend | Next.js 16 + React 19 | UI layer, proxies API requests to backend |
| Backend | Rust/Axum | API server, database queries, background jobs |
| Legacy DB | SQL Server (tiberius) | Shared with legacy application, read-only |
| HotelNew DB | PostgreSQL (sqlx) | Self-hosted, full CRUD |

**API Routing**:
- Frontend runs on port 3003 (exposed)
- Backend runs on port 3003 (internal, container network)
- Next.js rewrites `/api/*` requests to `http://backend:3003/api/*`
- Exception: `/api/changelog` is handled by Next.js (reads local CHANGELOG.md)

**Docker Services** (defined in `docker-compose.yml`):
- `web` - Next.js frontend (ghcr.io/thehfhotel/new-hotel)
- `backend` - Rust backend (ghcr.io/thehfhotel/new-hotel-backend)
- `newdb` - PostgreSQL 17 (postgres:17-alpine) for HotelNew database

## Development

- Frontend dev server: `pnpm dev` (runs on port 3003)
- Frontend build: `pnpm build`
- Backend: See `/hotel-backend/README.md` for Rust development

## Estate task board

Cross-repo tasks live in ~/HF/hf-tasks (thehfhotel/hf-tasks). Read `tasks/INDEX.md` before cross-repo work; update task status as you work.
