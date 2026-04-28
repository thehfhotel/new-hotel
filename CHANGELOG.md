# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.50.0] - 2026-04-28

### Added

- **Phase 5.5a — `legacy_mirror.*` schema for opaque pass-through of
  legacy-only features.** New PG schema with 11 mirror tables for the
  legacy-only features per `docs/architecture.md` §11: `ht_cupon`
  (food/breakfast vouchers), `ht_checkin_product` (in-stay POS /
  minibar), `ht_deposit` (standalone deposit ledger), `ht_continuetime`
  (hourly extension price master), `ht_changed_room` (mid-stay room-
  move audit), `ht_rooms_cancel` (per-room cancel audit),
  `ht_rooms_price` (per-customer-type room price overrides),
  `ht_bill_debt_h` + `ht_bill_debt_ds` (credit-sales ledger),
  `ht_order_up` + `ht_order_down` (per-customer-type pricing tiers).
  Each mirror table uses the legacy natural key as PK plus two
  bookkeeping columns: `mirrored_at TIMESTAMPTZ` (last write) and
  `mirror_source TEXT` (`'reconcile'` or `'ct'`). On decommission the
  schema is dropped wholesale.
- **Phase 5.5a — full-table reload of 4 dimension mirror tables in
  `scheduler::sync`.** `HT_ContinueTime`, `HT_Rooms_Price`,
  `HT_Order_Up`, `HT_Order_Down` are slow-changing reference data
  (max ~32 rows per table at HF Hotel) — reloaded via DELETE+INSERT
  in one PG TX per table on the existing reconcile cadence. No CT
  enablement needed on the legacy DB. Lays the foundation for the
  CT-mapper work in Phase 5.5c which will populate the 6 transactional
  mirror tables incrementally.

## [2.49.4] - 2026-04-28

### Added

- **Phase 5 sync — startup-time CT-retention overflow guardrail.**
  After the 2-day shadow-mode soak we discovered a foreseeable trap:
  shadow mode rolls back the PG transaction every tick, which freezes
  `legacy_ct_state.last_seen_version`. Meanwhile SQL Server's CT
  garbage collector keeps running, and once `MIN_VALID_VERSION`
  marches past our frozen watermark on any tracked table the row
  history we'd need to catch up incrementally is gone. Per-table
  observability counters (`bump_skipped` writes outside the TX) hide
  this — the dashboard stays green right up until everything goes red
  at once. The watcher now runs `check_retention()` against every
  CT-tracked table at startup and refuses to start (parallel to the
  existing cold-replay refusal) if any table has overflowed. Operator
  must run `bin/sync --bootstrap` to recover. Override available via
  `LEGACY_SYNC_ALLOW_OVERFLOW=true` (data-loss escape hatch — never
  in production). Documented in `docs/runbook-sync.md` §4b with the
  full recovery sequence and prevention guidance.

## [2.49.3] - 2026-04-27

### Fixed

- **Phase 5 sync — drop dead `Cin_Pay_Status` column from check-in
  aggregate loader.** `parent_loader::load_checkin_aggregate` projected
  `Cin_Pay_Status` from `HT_CheckIn_Pay`, but that column does not
  exist in the legacy schema (verified against the live INSERT captured
  in `docs/legacy-spike/raw/checkout2-20260424-101023/07-events.txt`).
  Result: every CT-driven check-in aggregate load failed with
  `Invalid column name 'Cin_Pay_Status'` — ~470 errors/min, all
  silently swallowed by the skip-and-continue path. Caught during the
  shadow-mode soak (TX rolled back so no PG damage); would have meant
  zero check-in / payment mirroring once flipped live. The mapper
  itself only consumes `id, Cin_No, Cin_Pay_Cash, Cin_Pay_Credit,
  Cin_Pay_Tran, Pay_No`, so dropping the projection is non-functional.
- **Phase 5 sync — close alerting gap that hid the above for ~16h.**
  `bump_counters` and `bump_skipped` unconditionally cleared
  `last_error`, `last_error_at`, and reset `consecutive_failures = 0`
  at the end of every tick. Per-row error increments from
  `record_table_error` were therefore wiped before the dashboard could
  read them, so `legacy_sync_status.consecutive_failures` stayed at 0
  through a 100%-failing 16h window. Threaded an `errored` flag
  through `poll_table` → `bump_counters` / `bump_skipped`; when set,
  the SQL no longer touches the error fields, so per-error increments
  survive into the next tick and the dashboard's
  `consecutive_failures >= 5` alert criteria actually fires.

## [2.49.2] - 2026-04-26

### Changed

- **Phase 5 sync — bump MSSQL pool to 20 + throttle CT retention check
  30s → 5min.** Operational tuning to silence "Timed out in bb8"
  warnings observed on the hotter mappers (HT_CheckIn_H, HT_CheckIn_Ds,
  HT_CheckIn_Pay, HT_Receipt_H — the ones whose mappers also do
  parent-aggregate re-loads). Two complementary changes:
  - **`MSSQL_POOL_MAX_SIZE` env var (default 20).** Was previously
    `DB_POOL_MAX` (default 10). The bb8-tiberius pool is shared by
    writeback + sync + ville-sync, so doubling the cap gives all three
    workers headroom. Backward compatible: `DB_POOL_MAX` still honored
    if set, but the new name is preferred for clarity.
  - **`LEGACY_SYNC_RETENTION_CHECK_INTERVAL_SECS` env var (default
    300).** Was effectively firing every poll-tick (1s) per table — 10
    tables × 1Hz × 1 MSSQL connection each = 10 conn/s of pure
    safety-net overhead. Throttled to once per 5 min per table inside
    the watcher's main loop via a per-table `Instant` map.
    Trade-off: retention overflow is a >48h outage scenario; recovery
    is operator-driven via Slack alert + manual `--bootstrap`
    reconcile, so 5-min detection vs 30s detection is operationally
    equivalent. The 10× reduction in pool pressure removes the bb8
    timeout noise.
  - Data-path CT poll cadence (`CT_POLL_INTERVAL_MS=1000`) untouched —
    latency budget preserved.

## [2.49.1] - 2026-04-26

### Fixed

- **`scripts/sync-status.sh` — mode-aware readiness checks.** The
  cutover-readiness section now detects whether the sync worker is in
  `shadow` or `live` mode (via `docker inspect` of the container's
  `LEGACY_SYNC_SHADOW_MODE` env) and runs different checks accordingly.
  - **Shadow mode** (the cutover-soak state): only checks container
    alive, bootstrap done, no per-table consecutive failures. Soak
    duration is informational (recommend 24h+, not blocking). Doesn't
    check watermark freshness or rows_skipped because those counters
    update inside the polling TX which rolls back in shadow mode —
    stale `legacy_sync_status` is expected design, not a problem.
    Doesn't check reconcile drift growth either, because the watcher
    rolls back its writes and the 15-min reconcile keeps detecting the
    same divergence — that's expected, not a regression.
  - **Live mode**: keeps the strict watermark-fresh + drift-stable
    checks since these signals are real once the worker commits.
  - JSON output adds `mode` field; `ready_to_flip` is mode-aware.

## [2.49.0] - 2026-04-26

### Changed

- **Phase 5 cutover step 1 — wire `LEGACY_SYNC_*` flags into deploy `.env`.**
  The `deploy` job now reads `LEGACY_SYNC_ENABLED` and `LEGACY_SYNC_SHADOW_MODE`
  from GH secrets and writes them into `~/new-hotel-production/.env` on
  evergreen, so the operator's flip survives subsequent deploys (the known
  pitfall documented in `docs/runbook-sync.md` §4a). Both secrets are
  added to the non-empty validation gate. Initial values: both `true` —
  worker enabled, shadow-mode rolling back every TX (observation only).
  Cutover sequence next: `--bootstrap` once, then `./scripts/sync-status.sh
  --watch` to soak 24–48h before flipping `LEGACY_SYNC_SHADOW_MODE=false`
  for live writes.

## [2.48.1] - 2026-04-26

### Reverted

- **Reverted 07b1dcb (`perf(ci): move backend tests into Docker test
  stage`).** The Docker `test` stage RUN step requires reaching the
  runner's PG service container at `localhost:5439`. Buildkit's
  docker-container driver runs the buildkit daemon in its OWN
  container; even with `network: host` + `--allow-insecure-entitlement
  network.host`, the host-network namespace exposed to the test RUN is
  the BUILDKIT container's host, not the runner's host. Result:
  `Failed to connect to test database: PoolTimedOut` on every test
  that calls `common::create_test_pool()`.
  - 3 failures in `test_bookings.rs::booking_*`, then aborted (sequential
    --test-threads=1 means the suite stops at the first PG-touching
    test). CI run 24958364935 — failure log retained for audit.
  - To re-attempt this optimisation we'd need either:
    `driver: docker` on `setup-buildx-action` (uses runner's docker
    daemon, so RUN host-network = runner host-network), or a sidecar
    PG container inside the buildkit network. Both are non-trivial
    and out of scope for this batch.
  - Test-backend reverts to runner-mode cargo with sccache + Swatinem
    rust-cache. Net effect on per-push CI time: zero (we're back to
    the post-cargo-chef baseline).
  - cargo-chef itself (commit e51eea8) remains in place — the
    revert is scoped to the Docker `test` stage + workflow changes.

## [2.48.0] - 2026-04-26

### Changed

- **Tightened `paths-filter` so doc-only commits skip every build job.**
  Previous filter set used a coarse `code` filter that triggered on most
  frontend-side files; doc-only commits (CHANGELOG, docs/**, README.md,
  AGENTS.md) sometimes still triggered jobs depending on which file
  in the root was touched.
  - Renamed `code` → `frontend`. New filter list explicitly enumerates
    every Next.js / tooling path: `app/**`, `components/**`, `lib/**`,
    `public/**`, `__tests__/**`, package files, `next.config.*`,
    `tsconfig.json`, `tailwind.config.*`, `postcss.config.*`,
    `Dockerfile`, `.dockerignore`.
  - Added `migrations/pg/**` and `init-db/**` to the `backend` filter.
    Schema changes affect `sqlx::query!()` compile-time validation, so
    a migration MUST retest + rebuild the backend before deploy. This
    closes a class of subtle drift bugs where the `.sqlx/` cache could
    silently go stale.
  - Added `scripts/backup-db.sh` to the `deploy` filter (it ships into
    the deploy directory; a change must trigger a re-deploy).
  - Module-level comment in the workflow documents the design
    contract: paths NOT covered by any filter intentionally cause every
    job to skip — that's the correct outcome for doc-only commits.
  - All `if:` conditions migrated from `outputs.code` to
    `outputs.frontend`. No catch-all globs introduced; the test/build
    matrix remains legible for future maintainers.

## [2.47.0] - 2026-04-26

### Changed

- **Restored cargo-chef in `hotel-backend/Dockerfile` + `Dockerfile.ville-sync`.**
  Source-only changes (the common case) now hit a cached `cargo chef cook`
  layer and skip the ~800-crate dependency recompile. Expected backend
  build time on warm cache: ~5min → ~1-2min. Cold-cache build (deps
  changed) is unchanged.
  - `chef → planner → builder → runtime` four-stage layout. The planner
    derives `recipe.json` (a hash of the dependency graph); the builder
    `cooks` deps once before COPYing the source. Docker's layer cache
    keys the cook step on `recipe.json` alone — when only source files
    change, the cook layer is a hit and never re-executes.
  - cargo-chef pinned to `0.1.71` (security + reproducibility — there is
    no Dependabot ecosystem for `cargo install` binaries; bumps are
    Dockerfile edits).
  - All Batch C / D guarantees preserved: pinned base-image digest,
    non-root runtime user, OCI labels, HEALTHCHECK, `--locked` flag,
    `inspect_*` debug bins excluded from runtime.
  - Both Dockerfiles share the `hotel-backend-target` cache id so a
    build of either image warms the other's `target/`.

## [2.46.5] - 2026-04-26

### Fixed

- **`customer_search_by_name` flake — shared-DB race, root-caused.**
  The integration suite shares a single PostgreSQL instance and the
  `ht_*` schema namespace. `customer_crud_lifecycle` called the shared
  `tests/common/mod.rs::cleanup` helper, which deleted every row matching
  `cust_notes LIKE 'TEST_%'`. That glob also matched `'TEST_search_by_name'`
  — the marker on the search test's just-inserted rows — so when the two
  tests ran concurrently (cargo's default), the lifecycle cleanup nuked
  the search test's data between its first assertion (1 row, passed)
  and its second (expected 2, got 0). Surfaced as 3 cascading CI failures
  (runs 24956846194, 24957055216, 24957295146).
  - Tightened `cleanup()` to use exact-match markers (`'TEST_customer_crud'`)
    instead of `LIKE 'TEST_%'`, so the helper only touches rows owned by
    the lifecycle test that calls it. Each query now uses parameter
    binding (`= $1`) for safety.
  - Added `--test-threads=1` to the CI `cargo test` invocation as
    defense-in-depth against shared-DB races we haven't enumerated.
    Cost: ~5-10s on a 13-test suite.
  - Documented the rule in the module-level comment of `tests/common/mod.rs`.

## [2.46.4] - 2026-04-26

### Fixed

- **BFG over-scrubbed `12345678` from non-credential contexts.** The
  history-rewrite pass replaced every occurrence of the 4 leaked
  literals with `***REMOVED***`, but `12345678` was the most generic of
  them — it appeared as fake Thai ID/phone/tax-ID test data, as fallback
  default values in dev binaries, and as the literal in legacy-reference
  decompiled C#. The replacement broke a regex literal in
  `InvoiceTemplate.test.tsx` (turning `/…12345678…/` into
  `/…***REMOVED***…/` — `*` is a regex metacharacter, "Nothing to repeat"
  parse error), which cascaded to 3 failed CI runs.
  - Restored `12345678` in test fixtures (Thai IDs/phones/tax IDs),
    .env.example fallbacks, dev-binary `unwrap_or_else` defaults,
    sqlx-prepare.sh dev DSN, and legacy-reference decompiled C#.
  - Kept the scrub in `CHANGELOG.md` and one workflow comment, but
    rewrote those entries with descriptive prose ("the SSH password",
    "<hardcoded>") so future readers understand without the literal
    being re-introduced.
  - The actual sensitive `12345678` use (HF Ville `sa` MSSQL password
    in `deploy/hfville/docker-compose.yml`) stays gone — it's now
    `${HFVILLE_MSSQL_PASSWORD:?…}` from a GH secret per Batch A.

## [2.46.3] - 2026-04-26

### Added

- **`scripts/sync-status.sh`** — Phase 5 sync worker observability dashboard.
  SSHes to evergreen and prints a sectioned, color-coded report covering:
  container status (sync/writeback/backend/newdb running + healthy?),
  CT watermark + freshness (last_seen_version vs last_polled_at),
  per-table activity (`legacy_sync_status` rows_ingested/skipped/errors),
  reconcile drift (`ht_reconcile_log` unresolved counts + sample),
  and a 6-check cutover-readiness verdict. Modes: `--watch` (refresh
  every 30s), `--json` (machine-readable), `--readiness` (focused
  exit-code-driven check; 0=green-to-flip, 1=not ready). Designed as
  the operator's primary tool for deciding when to flip
  `LEGACY_SYNC_SHADOW_MODE=false`.

### Fixed

- **Disable image-baked HEALTHCHECK on writeback + sync services.**
  The `hotel-backend` Dockerfile bakes a `HEALTHCHECK CMD curl /api/mode`
  for the backend HTTP service. Writeback and sync share that image but
  run different binaries that never bind port 3003 — Docker was reporting
  both as `unhealthy` indefinitely (surfaced by `sync-status.sh`).
  Compose-level `healthcheck: disable: true` per service.

## [2.46.2] - 2026-04-26

### Security

- **Removed unused `tiberius` features carrying vulnerable transitive deps.**
  Dropped `rustls` and `winauth` features from the legacy MSSQL driver. We
  connect to legacy SQL Server in plaintext over WireGuard (no TLS) using
  SA auth (no Windows NTLM), so neither feature was exercised. Eliminates
  4 Dependabot alerts:
  - `rustls-webpki@0.101.7` (GHSA-xgp8-3hg3-c2mh, GHSA-965h-392x-2mh5) — `< 0.103.12`
  - `rand@0.7.3` (GHSA-cq8v-f236-94qc) — pulled in by `winauth` (Windows NTLM)
  Bumped `bb8-tiberius 0.15 → 0.16` (necessitates `bb8 0.8 → 0.9`).
  HIGH alert (#133) on `rustls-webpki@0.103.13` was already patched (we have
  the patched version); will auto-close on next Dependabot scan.

## [2.46.1] - 2026-04-26

### Changed

- **CI/CD operational quality (Batch E of post-Phase-5.5 audit).**
  Final follow-up to the 5x batch audit (A–D shipped 2026-04-26).
  - **`docs/runbook-sync.md` — operator-flip-revert pitfall (doc-only).**
    New §4a "Known operator pitfall — `.env` is rewritten by every CI
    deploy" documents that every master push regenerates
    `~/new-hotel-production/.env` from GitHub Secrets, so SSH-edited
    flag flips (`LEGACY_SYNC_ENABLED=true` etc.) are silently reverted
    on the next deploy. Two remediations laid out side-by-side:
    (a) recommended GH-secrets approach (`gh secret set
    LEGACY_SYNC_ENABLED ...` + add to the deploy heredoc) and
    (b) alternative PG flag-table (`legacy_sync_control`) for mid-tick
    flips without redeploy. Cutover §4 steps 3-4 + rollback §5 now
    cross-link §4a. **Wiring is intentionally NOT included in this
    commit** — the operator decides when to flip the flag for the
    first time before exposing it via secrets.
  - **`init-db-migrations-drift-check` CI job** in
    `.github/workflows/docker-build.yml`. Spins up a throwaway
    `postgres:17-alpine` (same Batch C digest as the runtime newdb),
    applies `init-db/init-hotelnew.sql`, then runs `scripts/migrate.sh`
    and asserts "No pending migrations". Fails loudly with
    `Drift detected: init-db/init-hotelnew.sql is out of sync with
    migrations/pg/.` if the seed file is missing DDL or
    `schema_migrations` rows that any migration adds. Runs on every
    push (PRs catch drift early) and gates the `deploy` job's `if:`.
    Backfilled the four missing seed rows (009, 014, 015, 016) into
    `init-hotelnew.sql` so the check passes on master today.
  - **`-- @transactional false` pragma in `scripts/migrate.sh`.**
    Migration files can now opt out of the default per-migration
    `BEGIN`/`COMMIT` wrap by including the pragma comment in the first
    20 lines. Required for statements PostgreSQL forbids inside a
    transaction (`CREATE INDEX CONCURRENTLY`, `VACUUM`, etc.). The
    runner streams the body via `\set ON_ERROR_STOP on` and records
    the `schema_migrations` row in a separate atomic statement only
    after the body succeeds. Documented in the `migrate.sh` header
    block + `migrations/README.md`. Pragma-detection coverage added
    to `scripts/test-migrate-parse.sh` (case-insensitive,
    extra-whitespace, header-window-bounded — 9 new assertions, 25
    total).

## [2.46.0] - 2026-04-26

### Added

- **`.github/dependabot.yml`** — covers cargo (`/hotel-backend`), npm
  (`/` and `/thai-id-middleware-tauri`), github-actions, and docker
  (`/` and `/hotel-backend`). Weekly cadence. The previous file was
  the GitHub-template stub with `package-ecosystem: ""`.

### Changed

- **CI hygiene (Batch D of post-Phase-5.5 audit).**
  - **`build-frontend`, `build-backend`, `build-ville-sync` now
    `needs:` their respective test jobs** with explicit success-or-
    skipped guards in the `if:`. The `:latest` tag is no longer
    pushed to GHCR if tests fail, which the existing `deploy.if`
    treated as recoverable.
  - **Per-job `permissions:` blocks** (least-privilege GITHUB_TOKEN):
    - `changes`, `test-frontend`, `test-backend`, middleware `build`:
      `{ contents: read }` only.
    - `build-frontend`, `build-backend`, `build-ville-sync`:
      `{ contents: read, packages: write }` (push to GHCR).
    - `deploy`, `deploy-hfville`: `{ contents: read, packages: read }`
      (pull only — they don't push).
  - **GHA cache `scope:`** added to all 3 `docker/build-push-action`
    invocations: `frontend`, `backend`, `ville-sync`. Stops one image
    build's cache from evicting another's.
  - **`cargo test --locked`** in CI (matches the production Dockerfile
    build).
  - **`npm install` -> `npm ci`** in `middleware-build.yml`: refuses to
    update the lockfile, fails on any drift.

### Security

- **Pin third-party actions to commit SHA**:
    dorny/paths-filter           v3      -> d1c1ffe0…
    mozilla-actions/sccache-action v0.0.10 -> 9e7fa8a1…
    Swatinem/rust-cache          v2      -> e18b4977…
    pnpm/action-setup            v4      -> b906affc…
    webfactory/ssh-agent         v0.9.0  -> dc588b65…
    softprops/action-gh-release  v2      -> 3bb12739…
    dtolnay/rust-toolchain       1.89    -> 193d6aa1…
  Each call site has a `# vN.x.y` comment so future bumps stay obvious.
  The `actions/*` and `docker/*` calls remain on float — they're owned
  by GitHub and Docker respectively, so a SHA pin trades trust for
  Dependabot churn with no real attack-surface reduction.
- **Pin `dtolnay/rust-toolchain` to a `1.89` branch SHA** in BOTH
  `docker-build.yml` (was `@stable`) and `middleware-build.yml` (was
  `@stable`). Same Rust toolchain across both pipelines + matches the
  version that generated `.sqlx/` cache files.

## [2.45.3] - 2026-04-26

### Security

- **Image hardening (Batch C of post-Phase-5.5 audit).**
  - **Pinned every base image to digest** in all 3 Dockerfiles +
    `docker-compose.yml`. Sources (Docker Hub registry API on 2026-04-26):
    `node:20-alpine` -> `sha256:fb4cd12c…372293`,
    `rust:1.89-bookworm` -> `sha256:948f9b08…3c84ff`,
    `debian:bookworm-slim` -> `sha256:f9c6a2fd…c5c252`,
    `postgres:17-alpine` -> `sha256:c7526c0f…338609`.
    Stops a base-image hijack from injecting a malicious layer between
    pull and rebuild. Dependabot (`docker` ecosystem, added in Batch D)
    will surface new digests as PRs.
  - **Web runner stage now drops root.** Mirrors the backend pattern:
    creates `nextjs:nextjs` (UID 1001), `chown -R nextjs /app`,
    `USER nextjs`. UID 1001 picked to avoid colliding with the
    backend's UID 1000 if both ever land on the same host.
  - **Web runner stage is bare `node:20-alpine` (NOT `FROM base`).**
    No pnpm, no corepack, no build tooling in the runtime image —
    smaller surface, nothing for an attacker to leverage if they
    get RCE on the web pod.
  - **`hotel-backend/.dockerignore` widened.** Was 6 lines (`target/`,
    `.git/`, `.gitignore`, `Dockerfile`, `.dockerignore`, `*.md`,
    `.env`), now explicitly excludes `node_modules/`, `.next/`,
    `coverage/`, `.github/`, `.vscode/`, `.claude/`, `*.log`,
    `docs/`, `__tests__/`, `app/`, `components/`, `public/`, `lib/`,
    `legacy-reference/`, etc. Defense-in-depth: even if the build
    context ever changes from `./hotel-backend` to `.`, sensitive
    repo state stays out of the image build.
  - **Dropped `inspect_booking` and `inspect_schema` debug binaries
    from the production runtime image.** They remain available via
    `cargo run --bin inspect_*` locally for ops use, but never ship.
    Removes ~10 MB and a debugger-grade Postgres client from the
    image attack surface.
  - **Added `HEALTHCHECK` directives to all 3 runtime stages.** Web:
    `wget --spider http://localhost:3003`. Backend:
    `curl /api/mode`. ville-sync: declared no-op (`CMD true`) — the
    binary has no HTTP endpoint or liveness sentinel today; a deeper
    probe (touch `/tmp/sync_alive` per poll-loop iteration) is a
    follow-up. Container-level health complements the existing
    compose-level healthchecks for non-compose runners.
  - **Added OCI labels** (`org.opencontainers.image.source`,
    `org.opencontainers.image.licenses=Proprietary`) to all 3 images.
    Makes the GHCR repo back-link visible in `docker inspect` and
    `docker scout`.

### Changed

- **Resource limits added to every service in `docker-compose.yml`**
  (`deploy.resources.limits` / `reservations`). Per-service ceilings:
  newdb 2G/2 cpus, backend 1G/2 cpus, web 512M/1 cpu, writeback
  512M/1 cpu, sync 512M/1 cpu. Caps a runaway query or worker from
  starving the host.

## [2.45.2] - 2026-04-26

### Fixed

- **Deploy ordering, concurrency, healthcheck polling, restart policies
  (Batch B of post-Phase-5.5 audit).**
  - **Concurrency guards** on both deploy jobs — `deploy-prod-evergreen`
    and `deploy-hfville` groups, `cancel-in-progress: false`. Two
    back-to-back master pushes can no longer interleave their
    `docker compose up` (the cancel-mid-flight case left
    `migrate.sh` half-applied + backend on stale schema).
  - **Migrations now run BEFORE the new backend image starts**
    (`pull -> up -d newdb -> migrate -> up -d`). Previously:
    `pull -> up -d (full stack) -> migrate -> restart backend`, which
    started the new backend against the OLD schema for ~10s. Operator
    note: backwards-incompatible migrations still need expand/contract
    or a manual `scale backend=0` first — flagged in the deploy step
    comment.
  - **`wait_healthy` polling helper** replaces the fixed `sleep 3/5/5`
    waits. Polls `State.Health.Status` (or falls back to `State.Status`
    for services without a healthcheck) every 2s up to a per-call
    timeout, and dumps the last 50 log lines on timeout.
  - **Backend post-deploy probe** now uses `State.Health.Status`
    (actual healthcheck) instead of `State.Status` (which can be
    `running` mid-crash-loop). Same probe is invoked twice — once
    after `up -d`, once after the writeback/sync recreation — so a
    crash-loop introduced by the worker recreations is also caught.
  - **`deploy-hfville` now `needs: [deploy]`** with
    `needs.deploy.result == 'success'` in the `if:`. Stops a half-broken
    HF Hotel deploy from cascading into a HF Ville push that would
    write to a stale-schema target.
  - **`web` service healthcheck** added (`wget --spider http://localhost:3003`).
    The deploy job's wait_healthy on the backend container now has a
    parallel for the frontend, and `depends_on: backend service_healthy`
    in compose chains correctly.
  - **`writeback` and `sync` restart policy** changed from
    `unless-stopped` to `on-failure:5`. The clean `Ok(0)` exit when
    `WRITEBACK_ENABLED=false` / `LEGACY_SYNC_ENABLED=false` no longer
    triggers the respawn-loop (Docker treats exit 0 as expected under
    on-failure; the `:5` cap still protects against a real hot-loop).
  - **Migration version-comparison hardening** in `scripts/migrate.sh`.
    The previous `grep -q "^${version_padded}$"` would substring-match
    on a multi-line input under some `grep` versions and failed silently
    on malformed filenames. Replaced with:
    1. Strict regex on filename (`^([0-9]{1,3})_.+\.sql$` — anything
       else aborts the script with a clear error).
    2. Explicit set-membership lookup against an associative array
       (`APPLIED_SET[$version_padded]`). Documented expected naming.
  - **Tests:** `scripts/test-migrate-parse.sh` covers 16 cases against
    the parser logic without needing a live PostgreSQL — runs in <1s.

## [2.45.1] - 2026-04-26

### Security

- **CI/CD pipeline credential cleanup (Batch A of post-Phase-5.5 audit).**
  Replaced every hardcoded credential in `.github/workflows/docker-build.yml`
  and `deploy/hfville/docker-compose.yml` with GitHub Secrets references.
  - **Removed `sshpass -p <hardcoded>`** from the `deploy-hfville` job
    (previously plaintext-shipped the jump-box SSH password literal twice
    per deploy and once per `sudo -S`). The job now loads
    `HFVILLE_SSH_KEY` into ssh-agent via `webfactory/ssh-agent@v0.9.0`,
    pins the host key with a one-shot `ssh-keyscan -H` (drops
    `StrictHostKeyChecking=no`), and runs `sudo docker …` directly —
    requires a one-time NOPASSWD sudoers rule on the jump box (see
    "Operator setup" below).
  - **Replaced the production-PG password literal** in the `test-backend`
    job (CI-only Postgres) with `${{ secrets.TEST_POSTGRES_PASSWORD }}`. The
    secret value is constrained to URL-safe characters so it embeds in
    `DATABASE_URL` without percent-encoding.
  - **Replaced the HF Ville hfville-db PG, legacy-MSSQL `sa`, and
    production-PG password literals** in `deploy/hfville/docker-compose.yml` with
    `${HFVILLE_PG_PASSWORD:?…}`, `${HFVILLE_MSSQL_PASSWORD:?…}`, and
    `${POSTGRES_PASSWORD:?…}` (fail-fast if unset). The deploy-hfville
    step writes `~/hfville/.env` from GH Secrets via `umask 077`+heredoc
    so the file is born 600 (no 644-then-chmod race window).
  - **Added secret-non-empty validation** at the top of both `deploy`
    and `deploy-hfville` jobs — fails loud (`::error::`) before writing
    `.env`, so a misconfigured GH secret never silently produces an
    empty value the runtime would accept and misuse.
  - **Removed the `--- .env file check ---` debug echo** from the
    `deploy` job (info disclosure: enumerated env-var key names in the
    log even though it omitted the values).
  - **Operator setup required (one-time, separately from this commit):**
    1. Install the public half of `HFVILLE_SSH_KEY` into
       `~nut/.ssh/authorized_keys` on the HF Ville jump box (10.10.10.3).
    2. Add `nut ALL=(ALL) NOPASSWD: /usr/bin/docker, /usr/bin/docker compose`
       to `/etc/sudoers.d/nut-docker` on the jump box (replaces the
       `echo <password> | sudo -S …` pattern).
    3. After the operator confirms steps 1–2, rotate the in-place
       passwords on the jump box's PG + MSSQL + production-push target.
    4. Future password rotations: edit the corresponding GH Secret +
       re-run the deploy workflow — no code change needed.

## [2.45.0] - 2026-04-26

### Added

- **Phase 5.5 — production cutover scaffolding for the CT watcher.**
  The 10-table mapper coverage shipped in 5.4 (commit ea3dae0); 5.5
  ships the operational scaffolding so an operator can flip the
  watcher live.
  - **`sync` docker service block** (`docker-compose.yml`). Same image
    as `backend` + `writeback`, different command (`./sync`), under
    `profiles: [legacy]` so plain `docker compose up` skips it.
    Activate via `docker compose --profile legacy up -d`. Ships
    default-DISABLED — `LEGACY_SYNC_ENABLED=false` causes the binary
    to log + exit 0 cleanly. `.github/workflows/docker-build.yml`
    `deploy` step now force-recreates the `sync` container alongside
    the existing writeback recreation, with a state check that
    surfaces `running` (live) vs `exited` (default-disabled — the
    expected post-deploy state) without failing the deploy.
  - **`bin/sync --bootstrap` flag.** Cold-start path the operator
    runs ONCE before flipping `LEGACY_SYNC_ENABLED=true`. Steps:
    (1) verify schema fingerprint, (2) invoke
    `scheduler::sync::run_sync` in temporarily-overridden `upsert`
    mode to bring canonical PG state up to date with MSSQL, (3) read
    `CHANGE_TRACKING_CURRENT_VERSION()` from MSSQL, (4) UPSERT it as
    the watermark via direct UPDATE (overwrites unconditionally — does
    NOT use `watermark::advance`'s `<=` guard, so a partial prior run
    can be force-recovered). Exit 0. Bootstrap runs INDEPENDENTLY of
    `LEGACY_SYNC_ENABLED` — operator bootstraps first, then flips the
    flag (per `docs/runbook-sync.md` cutover sequence).
  - **Cold-replay refusal** (`bin/sync.rs` main loop). When
    `last_seen_version=0` (the migration's seed value) AND
    `LEGACY_SYNC_ALLOW_COLD_REPLAY != true` (default), the watcher
    refuses to start with a clear error message pointing at
    `--bootstrap`, fires a Slack alert, sleeps 60s before exit (to
    throttle Docker `restart: unless-stopped` cadence), and returns
    Err. Mitigates the "process every CT row from time-zero" footgun
    flagged in the original Phase 5 plan.
  - **Migration 019 — `ht_reconcile_log`.** Drift-detection tripwire
    table consumed by the demoted `scheduler::sync::run_sync`
    diff-only mode. Schema: `id`, `detected_at`, `table_name`,
    `legacy_pk`, `pg_hash`, `mssql_hash`, `mssql_row_json`,
    `pg_row_json`, `resolved_at`. Two partial indexes on
    `WHERE resolved_at IS NULL` for cheap dashboard / alerting reads.
    Mirrored in `init-db/init-hotelnew.sql` for fresh deployments.
  - **`docs/runbook-sync.md`** — operator runbook covering: bootstrap
    procedure, env-var matrix (every flag, default, when to flip),
    Slack alert meanings, cutover procedure (deploy → bootstrap →
    shadow soak 24h → enable → live soak 24h), rollback procedure,
    known limitations (payment-cancel cascade race window, receipt
    `pay_method='cash'` default, MSSQL probe in tests), 5-scenario
    receptionist test plan (create / cancel booking, check-in /
    check-out, add / cancel payment) for the receptionist team to
    run during live soak, and observability dashboard SQL pointers
    (`legacy_sync_status`, `legacy_ct_state`, `event_log` filter,
    `ht_reconcile_log` unresolved-set query).
  - **`SYNC_TEST_SKIP_MSSQL_PROBE` env var.** QoL escape hatch in
    `tests/test_sync_phase54_integration.rs::mssql_stub`. When set,
    skips the bb8-tiberius probe and returns `None` immediately —
    saves ~30s per test process when MSSQL is unreachable. Documented
    in the runbook env-var matrix.
  - **Tests:** `tests/test_scheduler_sync_diff_only.rs` (4 cases —
    PG-miss logged, hash divergence logged, canonical state
    untouched, partial-index unresolved-set query) +
    `tests/test_sync_phase55_bootstrap.rs` (6 cases — watermark
    overwrite semantics, cold-replay sentinel contract, post-bootstrap
    non-zero invariant, plus 3 compile-time guards on the env-var /
    flag literal names that must stay in sync with the runbook).

### Changed

- **`scheduler::sync::run_sync` demoted to diff-only safety net**
  (Phase 5.5). The CT watcher (`bin/sync`) is now authoritative for
  canonical PG state. The legacy 5-min full-UPSERT job is downgraded
  to a 15-min drift-detection tripwire that LOGS divergent rows into
  `ht_reconcile_log` instead of UPSERTing canonical state.
  - Env var `LEGACY_SYNC_RECONCILE_MODE` controls behaviour:
    `diff_only` (default) = hash-compare + log; `upsert` = original
    behaviour, kept as an escape hatch for operational rollback.
    Unknown values fall back to the safe (non-mutating) default with
    a warning log.
  - Cron schedule changed from `0 */5 * * * *` (every 5 min) to
    `0 */15 * * * *` (every 15 min — quarters of the hour for easier
    log correlation). Operators relying on the legacy 5-min cadence
    must understand canonical state is now sub-second-fresh via the
    CT watcher; the 15-min reconcile is purely a safety net.
  - Per-entity log lines now include the active mode tag (e.g.
    `[Sync] Customers (DiffOnly): … added, … updated, … unchanged …`)
    so operators can tell at a glance which path ran.

## [2.44.1] - 2026-04-26

### Added

- **Phase 5.4 — checkin + payment CT mappers (completing 10-table CT
  coverage). Every CT-enabled table now has a real mapper or an
  intentional retired stub.**
  - **`sync::parent_loader::load_checkin_aggregate(cin_no)`** — thin
    wrapper around the 5.3-factored `fetch_rows` helper. Owns
    `HT_CheckIn_H` (where_col=`Cin_no`), `HT_CheckIn_Ds` (where_col
    `Cin_No` — capital N, verbatim from legacy schema), and
    `HT_CheckIn_Pay`. Returns `CheckInAggregate { header, rooms,
    payments }`.
  - **`sync::resolve` module** — factored out of `mappers/booking.rs`
    for symmetry with the check-in mapper. Exposes
    `resolve_customer_id`, `resolve_booking_id`, `resolve_room_id`,
    `resolve_checkin_id`. Defer-on-missing semantics
    (`Ok(None)` → caller skips publish, next tick re-resolves).
  - **`sync::mappers::checkin`** ships two `MssqlChangeMapper` impls
    (`CheckInHeaderMapper`, `CheckInRoomsMapper`) that delegate to the
    shared `apply_checkin_aggregate` helper. The helper:
    - Re-loads the aggregate via `load_checkin_aggregate`.
    - Idempotently UPSERTs `ht_checkins` (skip-on-no-change).
    - Emits exactly one `CheckInCreated` / `CheckInCancelled` /
      `CheckOutCompleted` event per aggregate per tick.
    - On full check-out (every `Cin_Room_Status='Check-Out'`) AND a
      non-empty `Cin_Book_no`, also re-projects the parent booking
      aggregate inside the same TX so `Book_Status='ออกแล้ว'` →
      `book_status='completed'` propagates atomically.
  - **`sync::mappers::payment`** ships two impls — `PaymentMapper`
    (`HT_CheckIn_Pay` — coalesces by `Cin_No` so payment changes flow
    through the check-in aggregate sweep, keeping `cin_paid_amount`
    in sync with the legacy `Total_Price_Pay`) and `ReceiptMapper`
    (`HT_Receipt_H` — per-row dispatch, UPSERTs into `ht_payments`,
    emits `PaymentReceived`). Receipt cancellations
    (`status_name='ยกเลิก'`) flip `pay_voided=true`.
  - **`Cin_status` Thai literal mapping** — `'ปกติ'` (and unknown) →
    `'active'`; `'ยกเลิก'` → `'cancelled'`. Per-room
    `Cin_Room_Status='Check-Out'` (English with HYPHEN, verbatim from
    legacy) on every detail row promotes the canonical status to
    `'checked_out'`. Header cancel always wins.
  - **`RoomStatusMapper` retired**. `HT_Room_Status` CT rows are now
    documented log-only stubs; the check-in aggregate is the source
    of truth for "which room is occupied tonight". The mapper stays
    registered so the table appears in `legacy_sync_status`
    observability — operators can still see CT row counts tick.
  - **Wiring**: `bin/sync.rs::build_mappers` now wires real mappers
    for `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`, and
    `HT_Receipt_H` (was `NoopMapper` in 5.3). The coalesced dispatch
    path branches on table name to route booking / checkin / payment
    aggregates through their respective `apply_*_aggregate` helpers.
  - **`apply_checkin_aggregate` mssql parameter is `Option<&DbPool>`**
    so contexts without legacy access (unit tests, walk-in flows
    that never trigger the parent re-projection side-effect) can
    pass `None`. The watcher always passes `Some(&pool)`.
  - 30+ new unit tests covering legacy-status mapping, projection,
    coalescing dedup, idempotency, FK defer paths, checkout
    detection, and event-shape regression. 6 new integration tests
    against testcontainers PG covering walk-in upsert, idempotent
    re-apply, status flip to cancelled, header-delete cancel,
    full-checkout transition, and customer-defer behavior.
  - **Migration**: none needed — `ht_checkins.legacy_*` +
    `aggregate_id` already added in migration 014;
    `ht_payments.pay_reference` (the receipt-no key) is part of the
    baseline schema.

- **Phase 5.3 — booking aggregate CT mapper (3-table HT_Book_H +
  HT_Book_Ds + HT_Book_Date with parent re-load + per-tick coalescing
  emitting one DomainEvent per aggregate).**
  - **`sync::parent_loader`** (`hotel-backend/src/sync/parent_loader.rs`) —
    `load_booking_aggregate(book_no)` pulls header + every line + every
    calendar night for one booking from MSSQL into a `BookingAggregate`
    struct of `MappableRow` projections. Internal `fetch_rows` helper
    is generic over `(table, where_col, projection)` so 5.4's
    `load_checkin_aggregate(cin_no)` is one extra public wrapper, no
    refactor.
  - **`sync::mappers::booking`** ships three `MssqlChangeMapper` impls
    (`BookingHeaderMapper`, `BookingRoomsMapper`, `BookingDatesMapper`)
    that all delegate to a shared `apply_booking_aggregate` helper.
    The helper re-loads the aggregate, idempotently UPSERTs
    `ht_bookings` + `ht_booking_rooms`, and emits exactly one
    `BookingCreated` / `BookingModified` / `BookingCancelled` event
    per aggregate per tick.
  - **Coalescing layer in `bin/sync.rs::poll_table`.** Mappers opt in
    via the new `MssqlChangeMapper::coalesce_key(row)` trait method;
    when present, the watcher groups CT rows by aggregate root key
    (HashSet dedup) and dispatches `apply_booking_aggregate` exactly
    once per unique key per tick. Customer / room mappers return
    `None` (the trait default) and stay on the legacy 5.2 per-row
    dispatch path.
  - **`MappableRow` extended with a decimal arm** (`try_get_decimal`)
    so booking-header monetary columns (`Book_Price_Total`,
    `Book_Price_Pay`) project cleanly. Stored as `f64` to match the
    existing writeback (`writeback/format.rs`) + repository
    (`$N::float8` casts) precedent — see the rustdoc on the trait
    method for the rationale.
  - **`HashMapRow::MockValue::Decimal`** mirrors the new arm;
    `try_get_f64` and `try_get_decimal` cells are interchangeable so
    fixtures pick whichever variant reads more naturally.
  - **`Book_Status` literal mapping** (`'จอง'` → `'confirmed'`,
    `'เข้าพัก'` → `'checked_in'`, `'ยกเลิก'` → `'cancelled'`,
    `'ออกแล้ว'` → `'completed'`, anything else → `'pending'`) —
    legacy literals stay verbatim per the user's standing constraint;
    PG canonical column reuses the existing string convention from
    `routes/new_bookings`.
  - **Customer / room FK resolvers** in the booking mapper defer the
    apply when `legacy_cust_no` (resolved against `ht_customers`) or
    `room_no` (resolved against `ht_rooms_new`) hasn't been mirrored
    yet — the next tick's customer / room CT row brings the booking
    in via the same code path.
  - **Real wiring in `bin/sync.rs::build_mappers`** for `HT_Book_H`,
    `HT_Book_Ds`, `HT_Book_Date` (was `NoopMapper` in 5.2). Mapper
    count and CT-enabled-table list unchanged.
  - 28 new unit tests covering legacy-status mapping, projection,
    coalescing dedup, idempotency, and event-shape regression. 5 new
    integration tests under `tests/test_sync_phase53_integration.rs`
    covering insert / re-apply / modify / cancel / coalescing
    end-to-end against the canonical PG.
  - **No new migration** — `ht_bookings.legacy_book_id` /
    `legacy_cust_no` / `aggregate_id` already added by migration 014.
    `ht_booking_rooms` schema (already shipped) accommodates the
    projection.
  - **`LEGACY_SYNC_ENABLED` stays default-false.** Operator decision
    to flip live; the watcher remains opt-in.

## [2.43.1] - 2026-04-26

### Added

- **Phase 5.2 — customer + room CT mappers (read-only mode).**
  - **Migration 018** (`018_ht_customers_aggregate_keys.sql`) adds
    `legacy_cust_no VARCHAR(20)` and `aggregate_id UUID` to
    `ht_customers`, with partial unique indexes mirroring the
    migration-014 pattern. Required so the new CT mapper can map MSSQL
    `Cust_no` to the canonical PG row and emit `DomainEvent` payloads
    keyed on a stable aggregate UUID.
  - **`MappableRow` trait** (`hotel-backend/src/sync/row.rs`) — testable
    abstraction over a single CT-projection row. Production impl wraps
    `tiberius::Row`; the `HashMapRow` fixture (also reused by the
    watcher binary as the boundary representation) means production and
    tests both flow through the same `MappableRow` code path. Mapper
    `apply` signature changed from `Option<&tiberius::Row>` to
    `Option<&dyn MappableRow>`.
  - **Real JOIN dispatch in `bin/sync.rs`.** `poll_table` now composes
    the per-mapper `CHANGETABLE(CHANGES …) LEFT JOIN <table>` query
    using `mapper.select_sql()` + `primary_key_cols()`, parses each row
    into a `HashMapRow`, dispatches to the mapper, and persists the
    returned `DomainEvent` into `event_log` via `EventBus::publish`
    (live mode) or logs "would publish" + rolls back (shadow mode).
    Per-table TX with per-table watermark advance after commit. `0x4E48`
    `SYS_CHANGE_CONTEXT` filter applied on the SELECT.
  - **`CustomerMapper`** (`hotel-backend/src/sync/mappers/customer.rs`,
    ~370 LOC). Full I/U/D coverage for `HT_Customers`. UPSERT into
    `ht_customers` resolved by `legacy_cust_no`; `aggregate_id` derived
    via `service::ids::aggregate_uuid(AggregateKind::Customer, …)` and
    pinned on first insert. Idempotent — re-applies with identical
    content skip publication. D events soft-delete via
    `cust_deleted_at = now()` and emit no `DomainEvent` in 5.2
    (subscribers will re-add it as needed in later phases). Translates
    legacy Thai `Cust_Type_Main` literal back to the canonical
    `CustomerType` enum.
  - **`RoomMasterMapper`** (`hotel-backend/src/sync/mappers/room.rs`).
    Mirrors `HT_Rooms` (Room_Clean / Room_Use) into `ht_rooms_new` per
    user constraint "stick to current setup we have in HOTEL legacy
    app for now" — no new English status taxonomy, no metadata schema
    additions, no `RoomMasterChanged` event variant. Emits
    `RoomMarkedClean` / `RoomMarkedDirty` only when `Room_Clean`
    actually flips. Resolves rows by `legacy_room_id_int` then
    `room_no`. Refuses to auto-create unknown rooms (operator runs
    `bin/backfill_rooms` first).
  - **`RoomStatusMapper`** (same file). Phase 5.2 logging stub for
    `HT_Room_Status` — the per-night occupancy ledger is owned by the
    upcoming 5.3 / 5.4 booking + checkin mappers (rebuilt from
    `HT_Book_Date` / `HT_CheckIn_Ds`); duplicating it here would
    diverge from the canonical reconstruction. Returns `Ok(None)` for
    every row in 5.2.
  - **Tests.** 33 unit tests across `sync::mappers::customer` (10),
    `sync::mappers::room` (8), `sync::row` (5), plus 4 wiring tests in
    `bin/sync.rs` confirming each table dispatches to the right mapper
    type. Integration suite `tests/test_sync_phase52_integration.rs`
    (6 tests, gated on live PG via `DATABASE_URL`) covers the
    customer-insert / customer-idempotent-reapply / customer-update /
    customer-soft-delete + room-clean-flip / room-clean-no-op end-to-end
    against the real `event_log`.

  Phase 5.2 stays opt-in: `LEGACY_SYNC_ENABLED` defaults to `false`
  (the watcher binary exits 0 when disabled, intentionally). Flipping
  it to `true` in production is an operator decision.

## [2.43.0] - 2026-04-26

### Added

- **Phase 5.1 — Change Tracking watcher skeleton.** First sub-phase of
  the MSSQL→PG sync half of the decommission boundary
  (`docs/architecture.md` §3.6d, §3.7, §4d-tris, §10 #8). Lands the
  scaffolding the 5.2+ per-table mappers will plug into without changing
  any existing flow:
  - **New binary `bin/sync.rs`** — long-running CT watcher modeled on
    `bin/writeback.rs`. Honours `LEGACY_SYNC_ENABLED` (intentional
    disable exits 0, not 1, so Docker `restart: unless-stopped` doesn't
    loop). Parses `LEGACY_SYNC_SHADOW_MODE`, `LEGACY_SYNC_TABLE_ALLOWLIST`,
    `CT_POLL_INTERVAL_MS` (default 1000ms). Verifies the legacy schema
    fingerprint, surfaces drift to Slack, sleeps 60s before exit to
    throttle Docker restart cadence. Per-mapper panic isolation via
    `tokio::spawn`, SIGTERM drains the in-flight tick. Per-poll
    `MIN_VALID_VERSION` retention check refuses to advance past CT
    cleanup and Slack-alerts ops with the recovery path. Ships 10
    `NoopMapper`s in 5.1 — real mappers swap in one-by-one in 5.2+.
  - **New module `src/sync/`** — `MssqlChangeMapper` async-trait + the
    `NoopMapper` placeholder, `ChangeOp` typed wrapper around CT's
    `'I'`/`'U'`/`'D'` `SYS_CHANGE_OPERATION` codes,
    `watermark::{read_last_seen, advance}` against `legacy_ct_state`,
    and a `SyncError` enum covering sqlx/tiberius/bb8 + a typed
    `RetentionOverflow` variant for the CT-cleanup path.
  - **Loop-prevention chokepoint.** `db::mssql_session::set_context_info`
    issues `SET CONTEXT_INFO 0x4E48` ("NH" = New Hotel) on the writeback
    session. SQL Server CT surfaces it as `SYS_CHANGE_CONTEXT`; the
    watcher's `CHANGETABLE` SELECT filters those rows out. Wired as the
    first statement of `writeback::dispatcher::dispatch` — the single
    entry point through which every recipe runs. Belt-and-suspenders:
    5.2+ mappers will be idempotent UPSERTs so a missed tag costs at
    most one extra cycle.
  - **Per-table observability table `legacy_sync_status`** (migration
    017) — pre-seeded for the 10 CT-enabled tables (`HT_Customers`,
    `HT_Rooms`, `HT_Room_Status`, `HT_Book_H`, `HT_Book_Ds`,
    `HT_Book_Date`, `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`,
    `HT_Receipt_H`). Rows ingested / skipped / last error / consecutive
    failure count so operators can spot a wedged mapper without
    log-tailing. Same migration adds `ht_customers.cust_deleted_at` for
    the upcoming HT_Customers `D` (delete) mapper in 5.2.
  - **Tests** — exhaustive unit coverage of the new code: `ChangeOp`
    parsing (I/U/D recognised, unknown rejected, char round-trip),
    allowlist parsing (none/blank/comma/whitespace), mapper-builder
    filtering, `CT_ENABLED_TABLES` ↔ migration-017 seed parity,
    structural assertion that `dispatch()` calls `set_context_info`
    BEFORE the recipe `match`, structural assertion that
    `count_ct_rows` filters `SYS_CHANGE_CONTEXT <> 0x4E48`. Integration
    test `tests/test_sync_watermark.rs` exercises the read/advance
    round-trip against a live PG.

  Phase 5.1 is intentionally behavior-preserving: every entry in the
  watcher's mapper list is a `NoopMapper`, the watermark is left
  un-advanced (commented-out with a 5.2 TODO so we don't silently skip
  the rows the real mappers will need), and no `docker-compose.yml`
  service block is wired (5.5). The chokepoint is the only code change
  that touches an existing runtime path, and its only effect on the
  legacy DB is one extra `SET CONTEXT_INFO 0x4E48` statement per
  writeback session.

## [2.42.0] - 2026-04-26

### Changed

- **`docs/architecture.md`** — incorporated 2026-04 reverse-engineering
  findings from `legacy-reference/`. Recipe-level audits (booking, check-in,
  checkout/payment) found no new code divergences — recipes are aligned with
  decompiled C#. Doc updates only:
  - Fixed misleading "owns its own IDs (UUIDs)" bullet in §1 ASCII diagram —
    PG owns UUIDs internally; writeback emits legacy-shape string IDs
    (`C0001`, `R000001`, `CH26-000001`).
  - New §3.7 "Ground-truth principle" — codifies the 3-tier source-of-truth
    precedence (live captures > decompiled C# > inferred analysis), with the
    `HT_CheckIn_Ds.id IDENTITY` mistake as the case-in-point. Documents
    pricing source (`HT_Rooms_Price` not `HT_Rooms.Room_PriceA/B/C`), VAT 7%
    inclusive split formula, mixed Thai/English status enum landmines, and
    the `varchar Thai_CI_AS` text-encoding rule.
  - §4a — added legacy ID format table + the `MAX(id)+1 + TABLOCKX/HOLDLOCK`
    allocation pattern for the two non-IDENTITY PKs (`HT_CheckIn_Ds.id`,
    `HT_Receipt_H.id`).
  - §8 roadmap — promoted Phase 5 (CT watcher) to **TOP PRIORITY** and
    framed it as the missing half of co-existence; added Phase 5.5
    (read-only mirror tables for legacy-only entities — coupons, deposits,
    products, room moves, etc.).
  - §10 #8 — Change Tracking marked enabled & live-verified (was warning).
  - New §11 "Legacy-only features (opaque pass-through)" — explicit table
    of 10 legacy-only tables we mirror but never write to, plus 10 legacy
    behaviors we don't replicate (coupons, standalone deposits, in-stay
    POS, credit sales, hourly pricing, room-move, photos, SMS, etc.).

## [2.41.1] - 2026-04-26

### Added

- **`legacy-reference/`** — reverse-engineering artifacts derived from the
  original Windows `HOTEL.exe` binary, complementary to (not authoritative
  over) `docs/legacy-spike/`. Contains:
  - `analysis/` — `_FEATURE_MAP.md` (every form + navigation graph),
    `_COMPAT_CHEATSHEET.md` (1,901-line code-derived coexistence contract),
    `_REPORTS_INVENTORY.md` (46 Crystal Reports cataloged + QuestPDF
    replacement plan), `_SCHEMA.sql` (all 61 CREATE TABLEs).
  - `decompiled-source/` — buildable C# reference codebase (~298 .cs files)
    reconstructed from `HOTEL-cleaned.exe` via ilspycmd. Loads in
    Rider/VS/VSCode for F12 navigation; doesn't fully build (~280 known
    decompiler-artifact errors documented in its own README), but that
    doesn't matter for reference use.
  - `binaries/` — original `HOTEL.exe` (obfuscated, Dec 2024),
    `HOTEL-cleaned.exe` (de4dot output with .NET Reactor protections
    stripped and string literals decrypted), and `HOTEL.pdb` (debug
    symbols — what made the decompile so clean).
  - `vendor/` — commercial 3rd-party DLLs the .csproj references
    (DotNetBar, C1FlexGrid, BarcodeLib, etc.). Internal use only;
    do not redistribute.
  - Excluded from Docker builds via `.dockerignore`.

  Where this folder and `docs/legacy-spike/` disagree, **trust
  `docs/legacy-spike/`** — it's based on live Extended Events captures of
  the running app; this folder is inferred from decompiled source.

## [2.41.0] - 2026-04-25

### Fixed

- **Writeback recipes now emit byte-for-byte legacy SQL.** Re-derived
  every `INSERT INTO [HT_*]` statement against the live capture log
  (`/tmp/legacy-events-full.log`, 270 MB of `.Net SqlClient` events
  recorded from the production legacy app), then patched each recipe
  to match column order, casing, value forms, and Thai literals
  exactly. Receivers diffing our writeback output against legacy
  output now see no spurious deltas.

  Recipe-level changes:

  - `payment.rs` (`HT_CheckIn_Pay`):
    - Reordered to the legacy 20-column canonical layout (PriceTotal
      precedes PriceOne).
    - Added 6 missing columns: `Cin_Pay_Note`, `Pay_by`,
      `Cin_Pay_Free`, `Cin_Pay_Tran`, `Branch`, `Cin_Pay_web`.
    - `Cin_Pay_Ds` now carries the room number (was empty); unit
      column emits the literal string `'รายการ'` (was integer 1);
      product-id is `'P001'` (the payment-row code, distinct from
      the receipt-line `'SEV-001'`); branch defaults to
      `'สำนักงานใหญ่'` (Thai "head office"); per-night vs total
      price split honored.
  - `payment.rs` (`HT_Receipt_H`):
    - VAT-inclusive split (`Total / 1.07`) computed via the new
      `vat_inclusive_split` helper. `Receipt_VatIn='True'`,
      `Receipt_VatPer=7`.
    - Added missing columns: `Receipt_ref` (lowercase r),
      `Receipt_note` (Thai stay-period blurb),
      `Receipt_noteUP` (`'Booking'` when sourced from a booking),
      `Receipt_Tax` (caller-supplied tax/customer ID).
    - `status_name='ปกติ'` (was empty).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_H`):
    - Dropped 3 obsolete columns (`Total_Price_vat`, `Cin_note`,
      `Cin_Work_number`); fixed casing on `Cin_Date_Out` and
      `Cin_Type`; reordered `Cin_by` before `Cin_Date_in`.
    - `Cin_status='ปกติ'` (was `'เข้าพัก'` — that's the Ds-level
      `Cin_Room_Status` value, not the header status); added new
      `CIN_STATUS_NORMAL` constant.
    - `Cin_cust_price='ราคาปกติ'` (was empty);
      `Cin_Room_ALL='{room_no} '` with trailing space;
      `Cin_foreign='False'` (literal string); `Cin_Type=0` integer;
      `Total_Price_Pay=0.00` and `Total_Price_Balance=Net` (settle
      is now correctly pre-payment).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_Ds`):
    - Renamed `[Cin_Dep_Status]` to lowercase `[Cin_dep_status]`;
      dropped `[Dep_by]`; value is `'ไม่เก็บค่ามัดจำ'` (added
      `CIN_DEP_STATUS_NONE` constant).
  - `walkin.rs` + `checkin_to_booking.rs` (`HT_CheckIn_Other_People`):
    - Replaced hardcoded `'Mr. {name}'` with country-aware prefix
      heuristic (`'TH*'` → `'นาย'`, else `'Mr.'`) and a trailing
      space matching the legacy `name + ' ' + name2` pattern when
      `name2` is empty. Plumbing the actual `Cust_perfix` through
      the payload is a separate task.
  - `walkin.rs` + `checkin_to_booking.rs` (`Tb_Save_Image`):
    - Added optional photo-link UPDATE driven by new
      `CreateCheckInPayload.photo_tmp_no`. The .NET app fires this
      on every save (UPDATE matches 0 rows when no photo was
      uploaded). When the field is `None` we skip the UPDATE.
  - `booking_create.rs` (`HT_Customers`):
    - Dropped 4 obsolete columns (`Cust_sex`, `Cust_IDcard`,
      `Cust_Contry`, `Cust_Work_Tax`); added `Cust_Last_Change`
      with today's Bangkok date.
    - `Cust_Type_Main='ราคาปกติ'` (was `'บุคคลธรรมดา'` — that
      latter form is the UPDATE path's value, not INSERT). Added
      `CUST_TYPE_MAIN_NORMAL` constant; kept
      `CUST_TYPE_MAIN_INDIVIDUAL` for the UPDATE-path value.
  - `booking_create.rs` (`HT_Book_H`):
    - Dropped `[Book_Notify_Note]`; emits `Book_Notify_Day,Book_sale`
      WITHOUT square brackets to match the legacy column-list builder.
    - `Book_Date_in/out` are date-only forms (`'4/25/2026'`) — not
      midnight datetimes; the .NET booking-list view binds to the
      date string directly.
  - `booking_modify.rs` (`UPDATE [HT_Customers]`):
    - Extended the customer re-save from 26 to 31 SET fields,
      adding `Cust_Work_tax`, `Cust_perfix`, `Cust_sex`,
      `Cust_IDcard`, `Cust_Contry`. Renamed `[Cust_Type_Main]` to
      lowercase `[Cust_Type_main]`. `WHERE` becomes lowercase `where`.
      Extended `CustomerResave` payload with the 5 new fields.
  - `booking_cancel.rs`:
    - Documented the per-room DELETE pattern the legacy app emits;
      the bulk DELETE is functionally equivalent and remains in
      place until multi-room tracking lands.

  New helpers:

  - `format::vat_inclusive_split` — splits a VAT-inclusive total
    into `(before_vat, vat)` rounded to 2dp. Three tests assert
    parity with captured legacy receipts.
  - `format::money_2dp` — renders a finite f64 with exactly 2
    decimal places, the form the legacy app uses for VAT split
    and tender columns.
  - `constants::CIN_STATUS_NORMAL` (`'ปกติ'`),
    `CIN_DEP_STATUS_NONE` (`'ไม่เก็บค่ามัดจำ'`),
    `CUST_TYPE_MAIN_NORMAL` (`'ราคาปกติ'`),
    `BRANCH_HEAD_OFFICE` (`'สำนักงานใหญ่'`),
    `PAY_DS_NAME_ROOM` (`'ค่าห้อง'`),
    `PAY_DS_UNIT_ITEM` (`'รายการ'`),
    `PAY_DS_ID_ROOM` (`'P001'`),
    `RECEIPT_VAT_PERCENT=7`,
    `RECEIPT_VAT_INCLUSIVE` (`'True'`),
    `RECEIPT_STATUS_NORMAL` (`'ปกติ'`),
    `RECEIPT_NOTE_UP_BOOKING` (`'Booking'`),
    plus the `receipt_stay_note(check_in, check_out)` helper.

  Each touched recipe gained a `*_matches_legacy_capture_byte_for_byte`
  test that asserts column-list + value-tail substrings against the
  exact statement captured in `/tmp/legacy-events-full.log`. Total
  test count grew from 122 to 147 (25 new tests).

## [2.37.0] - 2026-04-25

### Added

- **Legacy booking number displayed in our app's UI.** `BookingListRow`
  and `BookingDetailRow` now carry `legacy_book_id` (the `R\d{6}`
  identifier the writeback worker back-populates after mirroring to the
  .NET app). The `/api/new/bookings*` JSON response surfaces it as
  `legacyBookId` (omitted when absent — booking hasn't been written
  back yet). The bookings list table shows it as a small mono-font
  reference under the booking number. Lets receptionists cross-
  reference a single booking between our app and the legacy app
  without manual lookups.

  `repository::booking::get` was converted from `sqlx::query!` macro
  to runtime `sqlx::query()` so column additions don't require
  regenerating the offline cache for every change.

## [2.36.0] - 2026-04-25

### Fixed

- **`booking_create` recipe now bumps `HT_Book_Date.Book_ok`** —
  closes the 2026-04-25 incident where bookings written via our app's
  writeback (R014831/2/3/...) appeared in the .NET app's booking-list
  view but NOT in the calendar grid view (room view).

  Per spike capture `booking-checkin/writes.txt:27`, the legacy app's
  pattern after every `HT_Book_Date` INSERT is to immediately fire
  `update HT_Book_Date set Book_ok=Book_ok+1 where id=<id>`.
  `Book_ok` defaults to 0 on INSERT; the calendar grid view filters
  for rows where `Book_ok > 0` (treating `Book_ok = 0` as draft state),
  so without the bump our bookings stayed invisible in the grid.

  Fix is a single additional `update HT_Book_Date set Book_ok=Book_ok+1
  where id={id}` after each night's INSERT. New regression test
  `book_date_inserts_each_followed_by_book_ok_increment` locks in both
  the count (one bump per night) and the ordering (bump immediately
  follows its INSERT).

  Existing bookings R014831/2/3 written before this release will need
  manual patching — either receptionist re-saves them in the .NET app
  (which fires the legacy pattern) or operator runs:
  `UPDATE HT_Book_Date SET Book_ok=Book_ok+1 WHERE Book_no IN
   ('R014831','R014832','R014833') AND Book_ok=0`.

  119 lib tests pass.

## [2.35.0] - 2026-04-25

### Fixed

- **Writeback worker — final 6 audit findings closed in parallel.**
  Three worktree-isolated agents tackled the deferred items
  concurrently; reconciled into 3 commits on top of `ac4ca67`.

  - **HIGH-1: atomic `mark_failed`** (commit `25ccb3c`). Folded the
    backoff computation into the post-claim `attempts` already on
    `ClaimedJob`, eliminating the second SELECT (`get_attempts_for_backoff`
    deleted). Removes the read-modify-write race against the janitor in
    a multi-worker deploy.
  - **MED-2: claim-gated `mark_done`/`mark_failed`** (commit `25ccb3c`).
    `ClaimedJob` carries `claimed_at: DateTime<Utc>`; the UPDATE adds
    `AND status='in_progress' AND claimed_at = $X`. On 0-row response
    the worker logs "re-claimed by another worker; discarding result"
    and skips back-population, preventing two workers from racing to
    write different `legacy_*` values into the same `ht_*` row.
  - **MED-4: throttled self-heal Slack alert** (commit `d3307cb`).
    `salvage_legacy_ids` now records each event into a process-local
    counter; after 5 events within 5 minutes a single Slack message
    fires (with the inspection SQL the operator should run), then
    resets. Pure `should_alert(state, now, threshold, window)` extracted
    for unit testing.
  - **LOW-2: resolution Slack on exhausted→done** (commit `d3307cb`).
    `mark_done`'s UPDATE rewritten as a CTE capturing the prior status
    atomically with the flip; on `prior_status='exhausted'` posts a
    `:white_check_mark:` Slack via new `send_resolved_alert`. Operator
    sees closure, not just alarms.
  - **LOW-3: listener auto-respawn** (commit `d3307cb`). `run_listener`
    wrapped in `run_listener_supervised` that loops with 5s backoff
    after every error; after 10 consecutive failures fires Slack and
    backs off to 60s. Never gives up — exiting would silently degrade
    the worker to 30s polling.
  - **MED-1: `validate_finite` in `booking_modify` + `checkin_cancel`**
    (this commit). Adds NaN/Infinity guards at the top of `execute()`
    in both recipes, matching the pattern in `payment.rs` /
    `extend_stay.rs` / `booking_create.rs`. 4 new unit tests.

  All 118 lib + 15 binary tests pass. Release build clean.

  Operational footprint of the audit work (commits `e6e5e66` →
  `25ccb3c`, ~10 commits):
  - 2 CRIT, 4 HIGH, 5 MED, 4 LOW closed.
  - 3 PG migrations: 014 (legacy_* columns), 015 (writeback_jobs retry
    state).
  - Lib tests: 78 → 118 (+40); binary tests: 0 → 15.
  - Self-heal, panic isolation, atomic claim-gating, exponential
    backoff, exhausted state, Slack alerts on exhaustion + schema drift
    + resolution + listener degradation. End-to-end error catching /
    retry / recovery / reporting now production-ready for the live
    receptionist test.

## [2.34.0] - 2026-04-25

### Fixed

- **Writeback worker — final pre-live-test hardening** (audit findings
  HIGH-2, HIGH-3, MED-3, MED-5, LOW-1).

  - **HIGH-3 — panic isolation.** Every `process_job` call now runs in
    `tokio::spawn`; a panic inside any recipe (or a tiberius driver
    bug) is caught via `JoinHandle::is_panic`, the job is force-
    exhausted with the panic message, and the main loop continues.
    Without this, a single panic would kill the worker; Docker would
    restart it but the in-flight job would sit `in_progress` for 5
    minutes before the janitor reclaimed — invisible to the operator
    until the alert fired elsewhere.
  - **HIGH-2 — non-retryable errors short-circuit to exhausted.**
    `mark_failed_with_retryable` checks `WritebackError::is_retryable()`
    and skips the retry budget for deterministic failures (intent
    mismatch, recipe business-rule errors, payload deserialize, schema
    drift). Slack alert fires immediately instead of waiting 12 minutes
    of wasted retries on the same payload.
  - **MED-5 — schema-drift Slack flood throttle.** Worker sleeps 60s
    after posting the schema-fingerprint Slack alert and before
    returning `Err`, so Docker's `restart: unless-stopped` doesn't loop
    the worker every 5s and page the operator 6×/min.
  - **MED-3 — exhausted-job Slack instruction.** The `:rotating_light:`
    Slack message now includes the full reset SQL (`status='pending',
    attempts=0, next_retry_at=NULL`) instead of just the status — the
    previous instruction would leave `attempts` at the cap and
    re-exhaust on the first retry.
  - **LOW-1 — head+tail error truncation in Slack.** `truncate_head_tail()`
    keeps the first 200 + last 300 chars (multi-byte safe for Thai),
    preserving the row context tiberius/sqlx errors put at the end.

  3 new binary unit tests for `truncate_head_tail` (short pass-through,
  long head+tail preservation, Thai multi-byte safety). 114 lib + 7
  binary tests pass.

## [2.33.0] - 2026-04-25

### Added

- **Operational hardening for the writeback worker** before first live
  test — error catching / retry / recovery / reporting paths now closed.

  **Recovery from stuck `in_progress` jobs.** Worker process crashes
  mid-recipe, or `mark_done`/`mark_failed` PG failures, previously left
  the `writeback_jobs` row in `status='in_progress'` forever and
  required manual SQL intervention. Migration 015 adds `claimed_at`;
  `claim_next_job` now also re-claims `in_progress` rows whose
  `claimed_at < NOW() - 5 min` (`STUCK_IN_PROGRESS_TIMEOUT_SECS`).
  No more stuck jobs.

  **Retry backoff.** A `failed` job was previously re-claimable on the
  next loop tick, burning through `WRITEBACK_MAX_ATTEMPTS` in seconds
  during an MSSQL outage. Migration 015 adds `next_retry_at`;
  `mark_failed` schedules retries with exponential backoff (30s, 2min,
  10min by default) and `claim_next_job` filters by it.

  **`exhausted` terminal status.** Once `attempts >= max_attempts` the
  worker now transitions the row to `status='exhausted'` instead of
  leaving it as `failed` (which was indistinguishable from "currently
  retrying"). Operator triage is one query:
  `SELECT * FROM writeback_jobs WHERE status='exhausted'`.

  **Slack alerts on terminal failures.** New
  `send_exhausted_alert()` posts a `:rotating_light:` message to
  `SLACK_WEBHOOK_URL` (when configured) the moment a job exhausts its
  retry budget, with the intent name, aggregate id, attempt count,
  truncated last error, and the recovery one-liner. Schema-fingerprint
  failures at startup also alert before the worker exits — the operator
  sees the failure even if they're not tailing logs.

  4 new unit tests for the backoff + timeout constants. 114 lib +
  4 binary tests pass.

  Operator notes:
  - Set `SLACK_WEBHOOK_URL` to enable alerts; otherwise failures are
    log-only.
  - To manually retry an `exhausted` job after fixing the underlying
    cause: `UPDATE writeback_jobs SET status='pending',
    attempts=0, next_retry_at=NULL WHERE id=...`.
  - To inspect queue depth: `SELECT status, COUNT(*) FROM
    writeback_jobs GROUP BY status`.

## [2.32.0] - 2026-04-25

### Fixed

- **Writeback resolver self-heals from `writeback_jobs.legacy_ids`** —
  closes audit LOW-1 (back-population race). The `writeback_jobs` audit
  row's `legacy_ids` JSONB is the **source of truth** for what the recipe
  allocated; the `ht_*.legacy_*` columns are a denormalized cache. If
  `mark_done` fails to back-populate the cache (PG hiccup, pool
  starvation, network glitch between MSSQL COMMIT and the PG UPDATE),
  the next intent's resolver now falls back to a `SELECT legacy_ids
  FROM writeback_jobs WHERE aggregate_id = $1 AND status = 'done' ORDER
  BY completed_at DESC LIMIT 1` lookup. Logs `warn!` so an operator can
  see when the cache is stale, but no manual intervention is needed —
  the system recovers automatically.

  Also: `mark_done` now retries back-population with bounded
  exponential backoff (3 attempts, 100/400/1600 ms) before giving up.
  Most transient failures self-recover at write time without ever
  reaching the resolver fallback.

  Net effect: the previously-documented "manually patch the row"
  recovery path is no longer required. Worst case is one extra SELECT
  per future intent on the affected aggregate.

## [2.31.0] - 2026-04-25

### Fixed

- **Writeback resolver — close CRIT-3.** The Phase 4b writeback worker's
  resolver in `bin/writeback.rs` queried `legacy_book_id` /
  `legacy_cin_no` / `legacy_room_no` columns that never existed in the PG
  schema, and joined on `WHERE id = $1` against tables whose primary keys
  are actually `book_id` / `cin_id` / `room_id` (and they're SERIAL `i32`,
  while the intent's `*_id` is a `Uuid`). Result: only `CreateBooking` and
  walk-in `CreateCheckIn` worked end-to-end; every modify / cancel /
  extend / checkout / payment / mark-clean intent silently failed at
  resolution time.

  Closes the gap with three coordinated changes (commit `<this release>`):

  - **Migration 014** adds `legacy_book_id` / `legacy_cust_no` to
    `ht_bookings`; `legacy_cin_no` / `legacy_room_no` / `legacy_cust_no`
    / `legacy_checkin_ds_id` to `ht_checkins`; `legacy_room_no` /
    `legacy_room_id_int` to `ht_rooms_new`; plus an `aggregate_id UUID`
    column on each (UNIQUE partial index, NULL allowed for pre-migration
    rows). `init-db/init-hotelnew.sql` mirrors the same columns for
    fresh deployments.
  - **Service layer** (`service/booking.rs`, `service/checkin.rs`) now
    stamps `aggregate_id = aggregate_uuid(kind, serial_id)` onto each
    freshly inserted row in the same transaction as the outbox enqueue.
    New `BookingRepository::set_aggregate_id` and
    `CheckInRepository::set_aggregate_id` methods. Without this stamp,
    the worker's resolver cannot map UUID→row.
  - **Writeback worker** now resolves via `WHERE aggregate_id = $1` (no
    more spurious `id` column reference). The renamed `mark_done`
    callback also back-populates the freshly allocated `legacy_*`
    identifiers (e.g. `R014812`, `CH26-005230`) onto the canonical PG
    row, so the next intent on the same aggregate (modify, cancel,
    extend, etc.) resolves immediately. Walk-in / check-in-to-booking
    recipes capture `SCOPE_IDENTITY()` after the `HT_CheckIn_Ds` INSERT
    via the new `recipes::execute_capturing_identity_at` helper, so
    `legacy_checkin_ds_id` is available for ExtendStay / CheckOut.
  - **`backfill_rooms`** also writes `legacy_room_no` / `legacy_room_id_int`
    / `aggregate_id` so MarkRoomClean works on the existing 58 rooms
    without waiting for the next room mutation.

  New `LegacyIds.with_room_no()` and `with_checkin_ds_id()` builders;
  walkin / checkin_to_booking populate both. JSON shape of
  `writeback_jobs.legacy_ids` extended with `room_no` and `checkin_ds_id`.

  All 114 lib tests pass — no recipe SQL changed, only orchestration
  around the existing INSERT statements.

## [2.30.0] - 2026-04-25

### Fixed

- **Writeback safety audit — CRIT/HIGH/MED gaps closed before live test.**
  Independent re-audit of the merged writeback worker (commit `d02d20d`)
  surfaced 2 CRIT bugs that would corrupt data on the first receptionist
  action, plus 3 HIGH and 2 MED issues. All addressed in this release.
  One CRIT (PG schema missing `legacy_*` resolver columns) is documented
  as task #24 — substantive design work, not a quick patch.

  **CRIT (2):**
  - **CRIT-1: Recipes now run inside an explicit MSSQL transaction.**
    Previously `bin/writeback.rs` issued statements in autocommit mode,
    which releases `TABLOCKX, HOLDLOCK` at the end of each statement —
    voiding the spike §6 race-safety guarantee. The MAX+1 SELECT and the
    INSERT it feeds were no longer in the same lock scope, so a
    concurrent worker / .NET client could read the same value. Wrapping
    `dispatch()` in `BEGIN TRAN ... COMMIT/ROLLBACK` restores the
    guarantee and gives us atomic rollback on partial-recipe failure.
  - **CRIT-2: Bangkok timezone conversion for every legacy datetime.**
    `format_legacy_datetime` previously called `dt.naive_utc()`, which
    treats a real UTC instant as Thai-local. Recipes pulling from PG
    `TIMESTAMPTZ` columns and `Utc::now()` would write times 7h behind
    the wall clock the receptionist actually entered. New `chrono-tz`
    dependency; `format_legacy_datetime`, `midnight_of`,
    `end_of_stay_at_almost_noon`, and `enumerate_calendar_nights` all
    convert to `Asia/Bangkok` first. New `format::bangkok_date` and
    `format::format_bangkok` helpers used across recipes.

  **HIGH (3):**
  - **HIGH-1: `booking_modify` falls back to existing room_no when the
    user only changes dates.** Previously every new `HT_Book_Date` row
    got `Book_type=''`, making the booking disappear from the .NET app's
    calendar grid. New `ModifyBookingInputs.existing_room_no` field;
    `execute()` queries MSSQL for the current `Book_type` when needed.
  - **HIGH-3: `payment` recipe accumulates instead of overwriting
    totals.** Previously set `Total_Price_Pay={amount}` and
    `Balance=0`, losing prior partial payments and clobbering Room/Net
    totals (which `booking_create` / `extend_stay` own). Now uses
    additive `Total_Price_Pay = ISNULL(...,0) + amount` and recomputes
    `Balance = Net - new_Pay`.
  - **HIGH-4: NaN/Infinity guard before SQL formatting.** New
    `helpers::validate_finite()` rejects non-finite f64 at recipe
    `execute()` entry. `format!("{}", f64::NAN)` would have produced
    the literal string `"NaN"` in SQL. Wired into `payment`,
    `extend_stay`, and `booking_create`.

  **MED (2):**
  - **MED-1: Empty legacy ID strings are now treated as missing.**
    `dispatcher.rs` resolution previously accepted `Some("")` as a
    valid resolved ID, producing silent no-op `WHERE Cin_no=''`
    UPDATEs. New `nonempty()` helper used at every resolution site.
  - **MED-3: TM.30 random IDs constrained to positive i32 range.**
    `rand::random::<i32>()` could produce negatives that the .NET
    WinForms grid may mishandle. New `positive_i32()` helper masks the
    sign bit.

  **Test suite:** 99 → 104 writeback unit tests (5 new regressions).
  Existing tests' wall-clock-as-UTC fixtures updated to Bangkok-local-as-UTC
  (e.g. `Utc.with_ymd_and_hms(_, _, _, 12, 0, 0)` became `(_, _, _, 5, 0, 0)`
  to mean noon Bangkok). All 114 lib tests pass.

## [2.29.0] - 2026-04-25

### Fixed

- **Writeback recipe gaps from spike-vs-recipe audit** — comprehensive
  follow-up to commit `0179f81` (which closed the room-display gap on
  booking create). The audit compared every recipe in
  `hotel-backend/src/writeback/recipes/` against the captured legacy SQL
  in `docs/legacy-spike/raw/*/writes.txt` and `docs/legacy-spike/findings.md`
  §3a–k and surfaced 17 divergences. This release fixes 16 of them in
  one branch (gap #1 was a miscount in the audit and verified
  already-correct). New shared helper `writeback/recipes/helpers.rs`
  hosts cross-recipe SQL (`mark_cupon_printed`).

  **HIGH severity (3):**
  - `booking_modify` now re-saves `HT_Customers` (spike §3c capture
    lines 5/16/28) when the new `BookingChanges.customer_resave` payload
    is set. Phone/address edits will propagate to the customer master
    once the route is enriched.
  - `payment` now updates `HT_CheckIn_Ds.Cin_Room_Pay_Total` per-room
    (spike §3h capture line 3) when `RecordPaymentCommand.checkin_ds_id`
    is `Some`. Multi-room revenue tracking restored.
  - `extend_stay` now emits two leading TM.30 touches
    (`UPDATE HT_CheckIn_H SET Cin_Work_number=<random>`) per spike §3a +
    §3f capture lines 1-2. Random `i32` per touch via the new `rand`
    dependency.

  **MEDIUM severity (6):**
  - `walkin` and `checkin_to_booking` both now emit
    `UPDATE HT_Cupon SET cupon_print=1` (spike §3a `walkin/writes.txt:9`
    / §3d `booking-checkin/writes.txt:39`) via the shared
    `mark_cupon_printed` helper.
  - `booking_modify` clears stale `HT_Rooms` display fields BEFORE the
    date diff (spike §3c capture lines 6/14/17) — fires on every modify
    to keep the calendar grid in sync.
  - `booking_modify` re-writes the `HT_Rooms` display caption with the
    new dates after a stay-range change (spike §3c capture line 26)
    when the new `BookingChanges.new_customer_name` field is supplied.
    Mirrors the `booking_create` caption format from `0179f81`.
  - `checkin_to_booking` `UPDATE HT_Customers` now spans the full ~25
    address + work field set (spike §3d capture line 28) instead of just
    the 5 fields it previously updated.
  - `payment` increments `HT_CheckIn_H.Total_Price_vat` (spike §3h
    capture line 6) — this hotel uses no VAT but the .NET app still
    writes the running total, so we mirror for parity with reports.
  - `payment` clears `HT_CheckIn_Product` defensively (spike §3h capture
    line 2) — almost always a no-op since we don't write that table, but
    emitted for byte-level parity with the legacy capture.

  **LOW severity (6):**
  - `payment` `HT_CheckIn_Pay` INSERT now includes the `[Cin_Pay_Ds]`
    column (spike §3h capture line 4 / schema column 4, varchar(500)).
    Empty-string default mirrors the .NET capture.
  - `payment` `HT_Receipt_H` INSERT — `[Receipt_c_no]` column verified
    against the schema baseline (column 4, varchar(50), nullable). The
    recipe already included it; documented the rationale (storing
    `cin_no` here lets receipts trace back to the originating check-in).
  - `extend_stay` payload extended with `stay_start`, `guest_label`, and
    the four totals (`new_room_price_total`, `new_net_total`,
    `new_pay_total`, `new_balance_total`). The recipe now enumerates
    calendar nights across the full `[stay_start, new_end)` range
    instead of `[today, new_end)`, fixing the bug where extends from a
    past start date would leave gaps in `HT_Room_Status`.
  - `checkin_cancel` payload extended with `room_price` + `pay_to_subtract`
    so the recipe can SUBTRACT the right amounts from `HT_CheckIn_H`
    totals (multi-room safe per spike §3i). Previously zeroed,
    leaving legacy totals stale.
  - `payment` recipe logs a warning when `RecordPaymentReceipt` is
    fully empty (all three of `customer_name` / `address` / `tel`
    blank) — surfaces silent route-enrichment failures in worker logs.

  **Audit gap #1 (CRIT, claimed):** verified the
  `booking_create.rs` `HT_Customers` INSERT has 33 columns and 33
  values aligned correctly (`'บุคคลธรรมดา'` lands in `[Cust_Type_Main]`
  at position 31). The audit's gap claim was a miscount; no fix needed.

  **Payload extensions:**
  - `outbox::intent::WritebackIntent::CancelCheckIn` gained `room_price`,
    `pay_to_subtract`.
  - `outbox::intent::WritebackIntent::ExtendStay` gained `stay_start`,
    `guest_label`, `new_room_price_total`, `new_net_total`,
    `new_pay_total`, `new_balance_total`.
  - `outbox::intent::WritebackIntent::RecordPayment` gained
    `checkin_ds_id: Option<i32>` for per-room apportionment.
  - `outbox::intent::BookingChanges` gained `new_customer_name` and
    `customer_resave: Option<CustomerResave>`. New `CustomerResave`
    struct mirrors the .NET app's full `HT_Customers` UPDATE field set.
  - `service::checkin::CancelCheckInCommand` and `ExtendStayCommand`
    gained matching fields. `service::payment::RecordPaymentCommand`
    gained `checkin_ds_id`.
  - All payload struct extensions use `#[serde(default)]` so jobs
    enqueued under the previous schema continue to deserialize cleanly
    (zero-downtime upgrade).

  **Tests:** all 107 lib tests pass (97 in `writeback::*` — up from 78).
  Added 11 new tests covering the cupon helper, walkin / checkin-to-booking
  cupon emission, the full HT_Customers field set, the `extend_stay`
  TM.30 touches and night-enumeration, the `payment` per-room
  apportionment / cart-clear / VAT accumulator / `Cin_Pay_Ds` column,
  the `booking_modify` room-book clear ordering, customer re-save, and
  caption rewrite, and the `checkin_cancel` price subtraction.

  **New dep:** `rand = "0.8"` (default-features-off, `std` + `std_rng`)
  for the TM.30 batch-number generator.

## [2.28.1] - 2026-04-25

### Added
- **One-shot rooms backfill binary** (`hotel-backend/src/bin/backfill_rooms.rs`).
  Mirrors the 58 rooms in legacy `HT_Rooms` and the 8 room types in
  `HT_SET_RoomType` into `ht_rooms_new` + `ht_room_types`, preserving the
  legacy integer ids as PG primary keys so the writeback worker can resolve
  `room_id → legacy room_no` and so the frontend's room picker has rows.
  Idempotent (ON CONFLICT DO UPDATE). Inverts `Room_Clean` per spike §3i.
  New `backfill-rooms` service in `docker-compose.yml` under
  `profiles: [backfill]`. Run via `docker compose --profile backfill run --rm backfill-rooms`.

### Fixed
- **Writeback payload gaps** — bookings synced from the new app to legacy MSSQL
  appeared in the .NET booking list with empty `Book_Cust_Name`,
  `Book_Cust_Tel`, `Book_Room_Type` (room number column), and
  `Book_Room_Price=0`. Same blanks on `HT_CheckIn_Ds.Cin_Room_No` /
  `Cin_Room_Type` / `Cin_Room_Price` for check-ins, and on the receipt
  header (`HT_Receipt_H.Receipt_Name` / `Address` / `Tel`) for payments. Root
  cause: route handlers (`create_booking`, `create_checkin`, `create_payment`)
  filled the writeback context with empty placeholders — a known gap flagged
  in [2.28.0]. Fix lookups + forwards customer/room metadata into the
  writeback intent payload:
  - `routes::new_bookings::create_booking` — looks up the customer
    (`ht_customers`) and the first assigned room (`ht_rooms_new`) before
    constructing `BookingWritebackContext`. Pulls `customer_name`,
    `customer_phone`, `room_no`, `room_type` (from the joined `ht_room_types`),
    and per-night price (request override → room weekday default →
    booking total).
  - `routes::new_checkins::create_checkin` — same pattern; resolves the
    customer through the booking for booking-linked check-ins, otherwise
    uses the request `customer_id`. Populates `room_no` / `room_type` /
    `price_per_night` (with weekday-default fallback) and
    `guest_name_for_registry` for the walk-in customer INSERT and the
    TM.30 primary-guest row.
  - `routes::new_payments::create_payment` — looks up the check-in's
    customer to populate the receipt header
    (`Receipt_Name` / `Address` / `Tel`).
  - **`outbox::intent::WritebackIntent::RecordPayment`** gained a `receipt:
    RecordPaymentReceipt` field carrying the receipt-header metadata. New
    `RecordPaymentReceipt` struct (customer_name + address + tel) lives in
    `outbox::intent`. Recipe signature
    `writeback::recipes::payment::execute` updated to consume it.
  - **`service::payment::RecordPaymentCommand`** gained a matching `receipt`
    field. The other two contexts (`BookingWritebackContext`,
    `CheckInWritebackContext`) already had the right shape — only the route
    helpers needed enrichment.
  - Lookup failures (deleted customer / deleted room between form submit
    and route) degrade to empty strings rather than failing the canonical
    write — preserves the legacy app's tolerance for orphaned FKs and
    matches the prior behavior for receipts.

## [2.28.0] - 2026-04-25

### Added
- **Phase 4b — writeback worker** (per `docs/architecture.md` §3.6c, §6, §8).
  New `hotel-backend/src/writeback/` module + `bin/writeback.rs` binary that
  drains the `writeback_jobs` outbox into the legacy MSSQL DB.
  - **9 recipes** in `writeback/recipes/`, each a faithful translation of
    `docs/legacy-spike/findings.md` §3a–k:
    - `walkin` (§3a) — 7 INSERTs + 3 UPDATEs across 7 tables for a walk-in
      check-in. Allocates `Cust_no`, `Cin_no`, `HT_Customers.id`, and
      `HT_Room_Status.id` under TABLOCKX.
    - `checkin_to_booking` (§3d) — like walk-in but skips the customer
      INSERT, sets `Cin_Book_no`, marks the booking `'เข้าพัก'`, and
      UPDATEs the first existing `HT_Room_Status` row instead of INSERTing
      it.
    - `booking_create` (§3b + §3k visibility) — 4 INSERTs (5 if customer
      new). Sets `book_room_type=2`, `Book_status=1`, midnight
      `Book_Date_in/out`, `Book_Notify_Day=3`, all-empty optional varchars
      so the booking is visible in the .NET app's main booking-list view.
    - `booking_modify` (§3c) — **targeted UPDATEs only**. Deliberately
      skips the legacy app's destructive DELETE-everything+REINSERT
      pattern; diffs `HT_Book_Date` rows to drop dates that no longer
      apply and INSERTs new ones with `WHERE NOT EXISTS` for idempotency.
    - `booking_cancel` (§3g-bis) — 4 UPDATEs + 1 DELETE. Soft-cancels
      `HT_Book_H` (`'ยกเลิก'`) and `HT_Book_Ds` (`status=3`); hard-deletes
      `HT_Book_Date`. Preserves the duplicate `book_status` UPDATE for
      byte-for-byte parity with the .NET capture.
    - `checkin_cancel` (§3i) — 7 statements. Allocates `HT_Rooms_Cancel.id`
      under TABLOCKX. **Subtracts** the cancelled room's price from
      `HT_CheckIn_H` totals (multi-room safe). Lights off with the
      cancel-specific note `'ปิดไฟ เนื่องจากยกเลิกห้องพัก'`.
    - `extend_stay` (§3f) — 5 fixed UPDATEs + N HT_Room_Status INSERTs.
      Skips the destructive Phase B per spike recommendation.
    - `checkout` (§3e Phase 2 ONLY) — 5 UPDATEs. **Skips Phase 1** (the
      destructive DELETE+REINSERT pattern in the legacy app). Uses
      `'Check-Out'` (hyphen) on `HT_CheckIn_Ds` and `'Check Out'` (space)
      on `HT_Room_Status` per spike §4c verbatim.
    - `payment` (§3h) — 4 statements: payment + totals refresh + receipt
      header + receipt line. Cash → `Cin_Pay_Cash`; credit/transfer →
      `Cin_Pay_Credit`. Receipt label `'ค่าห้องพัก [{room_no}]'`,
      service code `SEV-001`, unit `'คืน'`, no-VAT zeros.
    - `mark_clean` (§3j) — 2 statements + a prior-occupant lookup query.
      Filters by `HT_Rooms.id` (not `room_no`!) per spike §4e. Looks up
      the **most recent non-cancelled** occupant of the room for the
      `HT_Housewife.h_cin/h_cin_name` audit fields.
  - **Race-safe MAX+1 ID allocation** via `WITH (TABLOCKX, HOLDLOCK)`
    (verified live in spike §6 Test 2 — receptionist's UI hitched 10s
    but never erred). Helpers in `writeback/allocate.rs` cover all 10
    legacy ID counters (`Cust_no`, `Book_ID`, `Cin_no`, `Pay_No`,
    `Receipt_no`, `HT_Book_Date.id`, `HT_Room_Status.id`,
    `HT_Rooms_Cancel.id`, `HT_CheckIn_Ds.id`, `HT_Receipt_H.id`).
  - **Schema fingerprint guard** (`writeback/fingerprint.rs`): on startup
    hashes live MSSQL `(table, ord, column, type)` for the 10
    fingerprinted tables and refuses to start if it differs from the
    captured baseline. Fingerprint:
    `5f2c17bc402edfc80e04fecb9dd741e26ed4cf1036f16855626051cd276376d2`.
  - **Worker loop** (`bin/writeback.rs`): `LISTEN writeback_channel` +
    30-sec poll fallback, atomic `UPDATE … RETURNING` claim with
    `FOR UPDATE SKIP LOCKED` so multiple worker replicas can run safely.
    SIGTERM drains in-flight, then exits. Toggle via `WRITEBACK_ENABLED`
    env var (per architecture §7).
  - **Date format helpers** (`writeback/format.rs`): `M/D/YYYY h:mm:ss tt`
    matching .NET's `CultureInfo.InvariantCulture` per spike §4b.
    `format_legacy_datetime`, `format_legacy_date`, `date_to_ole_serial`
    (for `room_date_oa`), `sql_quote` (with `''`-doubling), `midnight_of`
    for §3k visibility.
  - **Constants** (`writeback/constants.rs`): mixed Thai/English literals
    copied verbatim per spike §4c — `'เข้าพัก'`, `'ยกเลิก'`, `'Check-Out'`
    (hyphen), `'Check Out'` (space), power-log note templates, etc.
  - **docker-compose**: new `writeback` service under `profiles: [legacy]`
    so State C deploys (`docker compose up`) skip it. Activate with
    `docker compose --profile legacy up -d`.
  - **90 unit tests** (`cargo test --lib writeback::`) cover SQL output,
    section-3k visibility, mixed Thai/English literal preservation, and
    skipping the destructive Phase 1/B patterns.

### Changed
- `hotel-backend/Cargo.toml` version bumped 2.8.3 → 2.9.0 (new feature).
- Added `signal` feature to tokio dependency for SIGTERM handling in the
  writeback worker.

## [2.27.3] - 2026-04-25

### Fixed
- **Migration 011/012/013 deploy failure** — each file's body contained a
  redundant `INSERT INTO schema_migrations ... ON CONFLICT DO NOTHING`,
  which collided with `scripts/migrate.sh`'s appended INSERT (without
  ON CONFLICT) in the same transaction:
  `duplicate key value violates unique constraint "schema_migrations_version_key"`.
  Removed the internal INSERTs; tracking is owned by `migrate.sh` (it also
  records the file checksum, which the internal INSERTs did not).

## [2.27.2] - 2026-04-25

### Fixed
- **Backend integration tests** (`hotel-backend/tests/test_outbox.rs`):
  - `test_enqueue_inserts_row` asserted on the wrong JSON path: with
    `serde(tag="intent", content="payload")` the variant fields are wrapped
    under `payload`, and the `CreateBooking` variant has a struct field also
    named `payload` — so the inner `CreateBookingPayload` lives at
    `payload.payload`, not `payload`. Assertion adjusted; no source change.
  - `test_publish_inserts_event_log_and_notifies` and
    `test_rollback_emits_no_event_and_no_notify` raced against each other on
    the shared `domain_events` PG channel under cargo's parallel test runner.
    Added a `recv_for_booking` helper that drains `pg_notify` messages until
    one matches the test's own `booking_id` (or times out), so cross-test
    notifications no longer cause spurious failures.

## [2.27.1] - 2026-04-25

### Security
- **Bumped Next.js 16.1.6 → 16.2.4** (and `eslint-config-next` to match) — closes
  6 advisories: DoS via Server Components, request smuggling in rewrites,
  unbounded `next/image` cache, unbounded postponed-resume buffering, null-origin
  Server Actions CSRF bypass, null-origin dev HMR websocket CSRF bypass.
- **Forced patched transitive deps via pnpm overrides**: `lodash >=4.18.0`,
  `handlebars >=4.7.9`, `postcss >=8.5.10`, `flatted >=3.4.2`, `ajv >=6.14.0`,
  scoped `brace-expansion`/`minimatch`/`picomatch` to patched versions per
  affected major. Resolves ~19 transitive advisories (lodash code-injection,
  handlebars JS-injection, postcss XSS, flatted prototype-pollution, ajv ReDoS,
  brace-expansion DoS, minimatch ReDoS, picomatch ReDoS).
- **Backend `cargo update`**: `rustls-webpki 0.103.9 → 0.103.13` (CRL-panic
  DoS + CRL-distribution-point logic), `rand 0.8.5 → 0.8.6` in both
  `hotel-backend` and `thai-id-middleware-tauri`.
- 3 low-severity Rust advisories remain transitively pinned by `tiberius@0.12.3`
  (latest), and will resolve when MSSQL is decommissioned per
  `docs/architecture.md`: `rand@0.7.3` (via `winauth`) and two
  `rustls-webpki@0.101.7` name-constraint issues (via `rustls@0.21`).

### Fixed
- **CI `test-backend` job**: install `mold` + `clang` on the Ubuntu runner so
  `hotel-backend/.cargo/config.toml`'s `-fuse-ld=mold` link flag resolves.
  Previously `cargo test` failed with `invalid linker name in argument
  '-fuse-ld=mold'` because the Dockerfile installs mold but the bare-runner
  backend test job did not.

### Removed
- **Legacy `thai-id-middleware/` (Electron) sub-project** — superseded by
  `thai-id-middleware-tauri/` (the only target of `.github/workflows/middleware-build.yml`,
  per its own release notes "~10MB Tauri vs ~150MB Electron"). Deletion drops
  ~50 Dependabot advisories tied to the bundled Electron + npm transitive tree
  (electron CVEs, xmldom XML injection, lodash, minimatch, picomatch, tar,
  path-to-regexp, brace-expansion, ajv, electron-builder).

## [2.27.0] - 2026-04-25

### Added
- **Backend SSE endpoint** `GET /api/events` (`hotel-backend/src/routes/events.rs`)
  — Phase 4a per `docs/architecture.md` §3.6e. Long-lived Server-Sent Events
  stream of every `DomainEvent` published via `EventBus::publish`. Each request
  opens a dedicated `sqlx::postgres::PgListener`, `LISTEN`s on the
  `domain_events` channel, and forwards each notification to the browser as
  `event: <DomainEvent::type_name()>` / `data: <raw JSON payload>`. 30-second
  `KeepAlive` heartbeat; client disconnect releases the PG connection.
- **Cargo deps** — `async-stream = "0.3"`, `futures-util = "0.3"`.
- **Frontend `useRealtimeEvents` hook** (`lib/use-realtime-events.ts`) — Phase 4a-frontend
  per `docs/architecture.md` §3.6e. Opens a single `EventSource('/api/events')` and
  fans 11 `DomainEvent` variants out to mapped cache buckets via a window
  `CustomEvent('realtime:invalidate')`. Companion `useRealtimeInvalidate(key, refetch)`
  lets list views subscribe in one line.
  - Mapping: `BookingCreated/Modified/Cancelled → ['bookings', 'rooms']`;
    `CheckInCreated/CheckOutCompleted/CheckInCancelled → ['checkins', 'rooms']`;
    `CustomerCreated/Modified → ['customers']`;
    `PaymentReceived → ['payments', 'checkins']`;
    `RoomMarkedClean/Dirty → ['rooms', 'housekeeping']`.
  - `EventSource` auto-reconnects per WHATWG spec; `onerror` only logs.
  - **Window `CustomEvent` fallback** because the app does not currently bundle
    React Query / SWR. `EVENT_TO_QUERY_KEYS` is the migration contract: once a
    cache lib lands, swap the dispatch for `queryClient.invalidateQueries(...)`
    without renaming any listener. TODO marked in the source.
  - Wired into `<AppShell>` — active app-wide.
- **Hook unit tests** (`__tests__/components/useRealtimeEvents.test.tsx`) — 14 tests
  covering EventSource lifecycle, per-variant fan-out, and key filtering.

### Changed
- **Routes thinned (Phase 2.5)** per `docs/architecture.md` §1, §6. Write
  handlers in `routes/new_{customers,bookings,checkins,payments}.rs` now
  delegate to the service layer instead of calling repositories directly.
  Reads (GET/list) keep calling repositories. `EventSource::our_app(Uuid::nil(),
  Uuid::new_v4())` is a temporary placeholder pending auth middleware.
- **Endpoint contracts unchanged.** Frontend `/api/new/*` calls behave
  identically; specific 4xx wording preserved via thin error mappers.

## [2.26.0] - 2026-04-25

### Added
- **Backend service layer** (`hotel-backend/src/service/`) — Phase 2 per
  `docs/architecture.md` §1, §6. One service per aggregate, each opening a
  single PG transaction, performing the canonical write through the
  repository, enqueuing the matching `WritebackIntent` via
  `OutboxRepository::enqueue`, publishing the matching `DomainEvent` via
  `EventBus::publish`, and committing — all four effects atomic.
  - `customer.rs` — `CustomerService { create, update }` + `CreateCustomerCommand` / `UpdateCustomerCommand` / `CustomerOutcome`.
  - `booking.rs` — `BookingService { create, modify, cancel }` + `CreateBookingCommand` / `ModifyBookingCommand` / `CancelBookingCommand` / `BookingOutcome` / `BookingSnapshotInputs` / `BookingWritebackContext` / `BookingRoomCommand`.
  - `checkin.rs` — `CheckInService { walk_in, check_in_to_booking, cancel, extend, check_out }` + `WalkInCommand` / `CheckInToBookingCommand` / `CancelCheckInCommand` / `ExtendStayCommand` / `CheckOutCommand` / `CheckInOutcome` / `CheckInWritebackContext`.
  - `payment.rs` — `PaymentService { record_payment, generate_receipt }` + `RecordPaymentCommand` / `GenerateReceiptCommand`.
  - `housekeeping.rs` — `HousekeepingService { mark_clean, mark_dirty }` + `MarkCleanCommand` / `MarkDirtyCommand`.
  - `error.rs` — `ServiceError` enum (`Validation` / `NotFound` / `Conflict` / `Repository(sqlx::Error)` / `Outbox` / `Internal`) with `From<sqlx::Error>` and a bridge `From<ServiceError> for ApiError`.
  - `ids.rs` — deterministic `i32` ⇄ `Uuid` aggregate-id bridge via `Uuid::new_v5(NAMESPACE_OID + "new-hotel.aggregate.<kind>")`. Lets the `WritebackIntent`/`DomainEvent` `Uuid` contracts coexist with today's SERIAL `i32` PG schema. Forward-compatible: when the schema migrates to native UUID columns the shim disappears without changing event payloads.
- **`AppState` service handles** — `customers_service`, `bookings_service`, `checkins_service`, `payments_service`, `housekeeping_service` (each `Arc<…Service>`). Constructed via `AppState::wire_services` from existing repositories + outbox + event bus + new pool. Routes are NOT yet refactored to delegate (Wave 4 Agent F).

### Changed
- **`hotel-backend/src/lib.rs`** — declares `pub mod service;` so the service layer is reachable from the binary, integration tests, and Wave 4 routes.
## [2.25.0] - 2026-04-25

### Added
- **Backend repository layer** (`hotel-backend/src/repository/`) — Phase 1b per
  `docs/architecture.md` §1, §6. Each aggregate gets a trait + PostgreSQL impl
  (`customer`, `booking`, `checkin`, `room`, `payment`, `inventory`).
- **Backend outbox + event-bus runtime** (`hotel-backend/src/outbox/`) — Phase 3b
  per `docs/architecture.md` §3.6c:
  - `queue.rs` — `OutboxRepository::enqueue()` writes a `writeback_jobs` row
    inside the caller's `&mut Transaction<Postgres>`, atomic with canonical write.
  - `bus.rs` — `EventBus::publish()` performs `INSERT event_log` + `pg_notify('domain_events', ...)` in caller's TX (NOTIFY deferred to COMMIT).
  - `idempotency.rs` — deterministic UUID v5 keys (namespace `d86fe320-5424-58cd-8c00-50ea7d998b36`).
- **`AppState.outbox: Arc<OutboxRepository>`** + **`AppState.events: Arc<EventBus>`** — wired into route state for service-layer callers.
- **`hotel-backend/src/lib.rs`** — exposes modules so integration tests can `use hotel_backend::…`.
- **Integration tests** (`hotel-backend/tests/test_outbox.rs`): 4 sqlx::test cases + 5 pure unit tests in `outbox::idempotency`.

### Changed
- **Routes thinned**: `routes/new_{customers,bookings,checkins,rooms,payments,inventory}.rs` no longer call `sqlx::query!()` directly; SQL text is byte-identical so existing `.sqlx/` cache stays valid.
- **`hotel-backend/Cargo.toml`**: declared `async-trait = "0.1"` explicitly; `uuid` feature `v5` enabled; `sqlx` feature `uuid` enabled.
- **`hotel-backend/src/main.rs`**: switched from inline `mod foo;` to `use hotel_backend::{...}` (single compilation between binary + tests).
- **Endpoint contracts unchanged.** Frontend `/api/new/*` calls behave identically.

## [2.24.0] - 2026-04-25

### Removed
- **Legacy app tree** (`app/(legacy)/*`): legacy dashboard, bookings, calendar, rooms pages and their `BlueNavbar` shell. Superseded by the modern Sidebar-based UI. Per `docs/architecture.md` §8 Phase 0.
- **`components/Navbar.tsx`** (legacy blue navbar with branch picker + Legacy/New mode toggle). Replaced by the renamed `Navbar` (formerly `NewNavbar`).
- **`__tests__/components/LegacyDashboard.test.tsx`**: tested the deleted `app/(legacy)/page.tsx` (10 tests).

### Added
- **Backend domain layer** (`hotel-backend/src/domain/`) — pure types, no I/O, no SQL.
  Per `docs/architecture.md` §1, §2 (Phase 1a). New modules:
  - `customer.rs` — `Customer` struct + `CustomerType` enum + Thai national-ID checksum validation
  - `booking.rs` — `Booking` struct + `BookingState` state machine (Pending / Active / CheckedIn / Completed / Cancelled) with legacy literal mappings
  - `checkin.rs` — `CheckIn` struct + `CheckInState` enum (Active / CheckedOut / Cancelled) with split `Cin_Room_Status` vs `room_status` legacy-literal helpers per spike findings §3e
  - `room.rs` — `Room` struct + `RoomStatus` + `CleanState` enums (with the inverted `Room_Clean='yes'` = "needs cleaning" semantic preserved per spike §3i)
  - `payment.rs` — `Payment` struct + `PaymentMethod` enum (Cash / Credit / Transfer)
  - `shared.rs` — `DateRange`, `Money` (i64 satang), `RoomNumber` primitives
- **Backend outbox enums** (`hotel-backend/src/outbox/`) — type-only contracts (Phase 3a):
  - `event.rs` — `DomainEvent` (11 variants) + `EventSource` + `BookingSnapshot` / `CheckInSnapshot` / `CustomerSnapshot` per `architecture.md` §3.6b
  - `intent.rs` — `WritebackIntent` (one variant per spike-validated recipe §3a–k) with `CreateBookingPayload` / `CreateCheckInPayload` / `BookingChanges`
- **PostgreSQL migrations** (Phase 3a tables, no consumers yet — Wave 2 fills them in):
  - `011_writeback_jobs.sql` — outbox queue (per `architecture.md` §4c)
  - `012_event_log.sql` — durable domain-event bus with 3 indexes (per `architecture.md` §4d-bis)
  - `013_legacy_ct_state.sql` — single-row Change Tracking watermark (per `architecture.md` §4d-tris)
  - Same DDL appended to `init-db/init-hotelnew.sql` for fresh deploys
- **Cargo dependency**: `uuid = "1"` declared explicitly with `["serde", "v4"]` features
  (was previously only available transitively through tiberius).

### Changed
- **App tree collapsed**: `app/new/*` promoted to `app/*` — every former `/new/...` URL is now its canonical `/...` path (e.g. `/new/bookings` → `/bookings`). Internal `<Link>` hrefs and Sidebar entries updated accordingly. **Backend `/api/new/*` routes are unaffected.**
- **`components/NewNavbar.tsx` → `components/Navbar.tsx`**: renamed (history preserved via `git mv`); breadcrumb logic updated to drop the obsolete `/new` prefix; "Legacy" escape link removed.
- **Root layout (`app/layout.tsx`)**: now wraps children in a new `AppShell` client component that renders `<Sidebar>` + `<BranchProvider>` (logic lifted from the deleted `app/new/layout.tsx`). Single root layout for the whole app.
- **Sidebar**: nav entries point at the new flat URLs; ported `card-reader`, `customers`, `changelog` added so all formerly-legacy features remain reachable. Bottom "Legacy" exit link removed (no longer a destination).
- **`hotel-backend/Cargo.toml`** version bumped 2.8.1 → 2.8.2
- **`hotel-backend/src/main.rs`** — registers new `domain` and `outbox` top-level modules

## [2.23.0] - 2026-04-24

### Added
- **Design system**: SAP Fiori Compact UI with oxidized blood-red brand palette (`brand-50` … `brand-800`)
  - 13px base font, 28px row height, dense spacing
  - Sarabun (Thai government typeface) replaces DM Sans — supports Thai + Latin
  - Tailwind tokens for `shell`, `panel`, `headerBar`, `border`, `borderStrong`, `text`, `textMuted`
  - Semantic colours: `success`, `warning`, `error`, `info`
  - All border radii squashed to 2px (rounded-full preserved for circular elements)
- **Inventory backend**: missing mutation endpoints
  - `GET /api/new/inventory/rooms` — room rollup list with inventory count + last-checked
  - `POST /api/new/inventory/rooms/:room_id/check` — record an inventory check
  - `POST /api/new/inventory/rooms/:room_id/replenish` — replenish room stock (deducts from main inventory, logs OUT transactions)
  - `POST /api/new/inventory/adjustments` — generic add/remove/set stock adjustment
- **Backend healthcheck**: `/api/mode` probe + curl in Docker image; web service now waits for backend `service_healthy` before starting

### Fixed
- **Backend NUMERIC casts**: every dynamic-SQL `SELECT` reading DECIMAL columns now `::float8` casts so `try_get::<f64, _>` succeeds instead of silently defaulting (rooms prices, rate values, room-type base price/size, booking totals, inventory cost, report revenue)
- **Backend invoice**: `LEFT JOIN` columns `book_no`/`cust_firstname`/`room_no`/`type_name` are now `COALESCE`'d so walk-in check-ins without a booking/customer no longer fail (`new_invoice.rs`)
- **Backend invoice/inventory**: `.sqlx/` cache regenerated for all modified `query!()` calls
- **Branch filter**: `GET /api/new/{rooms,bookings,customers,checkins,inventory/rooms}` now honour `?branch=hfville` by returning empty results (HotelNew DB only contains HF Hotel data)
- **Room inventory checklist**: backend response shape now matches frontend expectations (`{ success, data: { roomId, roomNumber, roomType, items: [{ assignedQuantity, ... }] } }`)
- **Migrations**: `psql -v ON_ERROR_STOP=1` + `\set ON_ERROR_STOP on` so a failed migration aborts and the `schema_migrations` row is NOT inserted (previously a SQL error was silently ignored and the migration was marked applied)
- **Docker compose**: `web` waits for `backend: service_healthy` (and backend has a healthcheck on `/api/mode`); previously web could start before backend was listening

### Changed
- **Sidebar**: redesigned per Fiori — active item is `bg-brand-50` + 3px left border + `brand-700` text; nav rows reduced to `px-3 py-1.5`; section labels at 10px uppercase; removed `Hotel` logo icon and red "NEW" pill
- **NewNavbar**: thin 40px top bar with breadcrumb + Legacy link
- **DataTable**: `bg-headerBar` 12px header, 32px (h-8) rows with `even:bg-rowAlt` zebra stripes, sort indicators in `text-brand-500`; removed `rounded-lg` wrapper
- **Button / Input / Card / Badge / StatCard**: re-skinned to brand palette and Fiori sizing
  - Button primary: `bg-brand-500` + `border-brand-700`; sizes `h-6/h-7/h-8`
  - Input: 28px tall (`h-7`), `bg-panel`, `border-borderStrong`, `focus:border-brand-500`
  - Card: flat panel, `p-3` default, optional `<CardHeader>` with header-bar styling
  - Badge: flat rectangles with semantic 1px border (no pills)
  - StatCard: 20px value text, 11px uppercase label
- **Dashboard tiles**: removed `bg-{red,yellow,orange,blue}-50` tint backgrounds and `border-b-4` colored borders; now neutral white panels with a 8px coloured status dot in the corner
- **Dashboard modal**: flat panel, no shadow, no rounded corners
- **Page headers**: top pages (`/new`, `/new/bookings`, `/new/rooms`) now use a 40px-tall flat panel header bar with `text-base font-semibold` titles instead of `text-2xl font-bold`
- **app/layout.tsx**: switched to `Sarabun` from `next/font/google`, exposed as `--font-sarabun` CSS variable
- **app/globals.css**: removed body gradient; added brand palette CSS variables; re-skinned react-datepicker to brand palette
- **Backend Dockerfile**: install `curl` for HTTP healthcheck
- **Backend Docker builds**: switched from the dummy-source dependency-cache trick to `cargo-chef` (`planner` + `builder` stages cooking `recipe.json`); fragile-when-Cargo.toml-changes pattern replaced with the standard chef recipe (applied to both `Dockerfile` and `Dockerfile.ville-sync`)
- **Backend Docker builds**: install `mold` + `clang` in the Rust builder stage and added `hotel-backend/.cargo/config.toml` with a target-scoped `linker = "clang"` + `-fuse-ld=mold` rustflag (only applies to `x86_64-unknown-linux-gnu`, so macOS/aarch64 local builds are unaffected); cuts release link time substantially
- **CI test-backend job**: added `mozilla-actions/sccache-action` + `RUSTC_WRAPPER=sccache` (with `SCCACHE_GHA_ENABLED=true`) alongside the existing `Swatinem/rust-cache` step, giving per-rustc-call cache hits on top of the whole-`target/` cache
- **Backend dep graph**: shrunk transitive crate count from 662 → 568 (-94, ~14%) for faster cold Docker builds (`hotel-backend` v2.8.0 → v2.8.1):
  - Replaced `reqwest` with `ureq 2.12` for the Slack webhook client (drops `hyper-tls`, `h2`, `hyper-rustls`, ~90 transitive crates); blocking call dispatched via `tokio::task::spawn_blocking` so the async runtime is never blocked; same 3-attempt retry semantics with exponential backoff
  - Slimmed `tokio` from `["full"]` to explicit minimal feature list `["macros", "rt-multi-thread", "net", "time", "sync"]` based on actual usage audit
  - Dropped the `bigdecimal` feature from `sqlx` — no `BigDecimal` types are read in code; all `DECIMAL`/`NUMERIC` columns are `::float8`-cast to `f64` at the SQL level (per CLAUDE.md guidance)

### Removed
- `Hotel` lucide icon import in Sidebar (now bare wordmark)

## [2.22.1] - 2026-03-04

### Fixed
- **Charts**: Fix missing `yAxisId="right"` YAxis in LineChart causing runtime error; guard empty data domain
- **BookingDetailDrawer**: Fix stale closure in useEffect, add dialog role/aria-modal/Escape handler, replace alert() with inline errors, increase delete button touch target
- **PaymentModal**: Fix useEffect silently resetting user-typed payment amount on re-render
- **XSS**: Escape all server data in inventory transactions print view (document.write)
- **Timezone**: Use UTC methods in formatDateBE, toBuddhistYear, customers page dates per CLAUDE.md convention
- **StayTimeline**: Guard dayData.reduce against empty array crash; add aria-labels to nav buttons
- **StockAdjustmentModal**: Differentiate add/remove/set button colors (green/red/blue)
- **DataTable**: Use unique data ID as React key instead of array index
- **Dashboard**: Accumulate fetch errors instead of overwriting; show error banner
- **Customers page**: Add error banner for fetch failures; fix date-fns timezone issue
- **Bookings page**: Add aria-labels to pagination buttons
- **BookingForm**: Rename shadowing BookingFormData to BookingFormState; fix calculateNights date parsing

### Security
- **CORS**: Restrict thai-id-middleware from `origin: '*'` to localhost app origins
- **Headers**: Add X-Content-Type-Options, X-Frame-Options, Referrer-Policy; disable X-Powered-By
- **Credentials**: Remove hardcoded passwords from docker-compose.yml; require .env file
- **escapeHtml**: Add single-quote escaping for defense-in-depth
- **URL encoding**: Add encodeURIComponent to branch query parameter

### Changed
- BranchContext and ModeContext now wrap children in Provider during initialization (no flash of empty)
- Exclude playwright.config.ts from TypeScript compilation
- Update .env.example with placeholder passwords and POSTGRES container vars
- GuestRegistryModal TM.30 notice uses higher-contrast text colors

## [2.22.0] - 2026-02-21

### Added
- **Push HF Ville data to local cache for faster API reads** — ville_sync now pushes data to production `ville` schema in newdb
  - `ville` schema with 4 cached tables (`ht_rooms_legacy`, `ht_bookings_legacy`, `ht_checkins_legacy`, `ht_customers_legacy`) + `sync_status`
  - ville_sync writes to two targets: local jump box PG (store-and-forward buffer) + production newdb (primary target for API reads)
  - Backend reads from local `ville` schema instead of crossing WireGuard tunnel (<1ms vs ~50ms latency)
  - Push is optional/graceful — if production unreachable, local buffer continues; next cycle reconciles via SHA256 hash comparison
  - Migration `010_ville_cache_schema.sql` creates the ville schema
  - newdb port exposed on WireGuard interface (`10.10.10.4:5439`) for ville_sync push access

### Changed
- Backend `ville_pool` now connects to local newdb with `search_path=ville` instead of remote PG via socat
- Removed VILLE_DB_SERVER/PORT/NAME/USER/PASSWORD env vars from backend (uses newdb credentials)
- `hfville-pg-forward` socat service on production can now be stopped (no longer needed)

## [2.21.0] - 2026-02-19

### Added
- **Multi-branch support: HF Ville integration** — second hotel branch (สุราษฎร์ธานี, 34 rooms) integrated into the system
  - Branch selector in Sidebar (new system) and Navbar (legacy system): HF Hotel | HF Ville | ทั้งหมด
  - `BranchContext` + `useBranchFetch` hook — all API calls automatically include `?branch=X`
  - Backend `Branch` enum (`hfhotel`, `hfville`, `all`) with `ville_pool: Option<PgPool>` in AppState
  - Branch parameter added to 7 route handlers: rooms, bookings, checkins, customers, stats, occupancy, calendar
  - HF Ville room layout (2 floors, rooms 101-218) with stacked "All" view showing both hotels
  - `VilleDbConfig` in backend config with `VILLE_DB_ENABLED` env var for graceful degradation
  - SSH tunnel (`hfville-tunnel` systemd service) for remote PG access via cloudflared
  - `ville_sync` binary: syncs HF Ville SQL Server 2005 → PostgreSQL mirror via FreeTDS (SHA256 delta sync, 90s interval)
  - Jump box deployment: `deploy/hfville/docker-compose.yml` with postgres:17-alpine + sync binary
  - HF Ville PG mirror schema: `deploy/hfville/init-db/init-hfville.sql` (rooms, bookings, checkins, customers + sync_status)
  - 8 frontend pages updated with branch-aware fetching (legacy: dashboard, calendar, bookings, rooms, customers; new: dashboard, calendar, bookings)

### Fixed
- HF Ville room queries failing with `relation "ht_rooms_new" does not exist` — added legacy-only query functions for ville pool (which only has `ht_rooms_legacy` tables, not HotelNew tables)
- Room grid not showing "ทำความสะอาด" (cleaning) status — `Room_Clean = "yes"` means room needs cleaning (not "is clean"); fixed in both legacy and new dashboards
- Garbage tsql output artifacts (locale messages, prompt markers) stored as room/customer/checkin rows in HF Ville PG mirror — fixed parser and added cleanup

## [2.20.0] - 2026-02-19

### Changed
- **Calendar revamp** — simplified `StayTimeline` from 3 confusing stacked segments to a clean 2-color model
  - Past dates: single sky-500 bar showing rooms checked-in (occupied) that day
  - Future dates: single amber-400 bar showing rooms booked for that day
  - Today: two bars side-by-side — checked-in (sky) + booked (amber) with red ring highlight
  - Simplified `DayData` interface: `checkedIn`/`booked`/`checkinStays`/`bookingStays` (was 6 fields)
  - Simplified detail panel: `'checkin' | 'booking'` segment types (was 3 types including `'continuing'`)
  - Legend reduced from 3 items to 2: เข้าพัก (sky-500) + การจอง (amber-400)
  - Tooltip now context-aware: shows only relevant data for past/today/future
  - Bars scaled independently against `maxCount` (max of all checkedIn/booked) instead of stacked totals

## [2.19.0] - 2026-02-19

### Changed
- **Light theme for new system** — converted entire `app/new/` from dark zinc theme to light gray theme
  - Foundation: `globals.css` (removed `.new-system-layout` dark overrides, light datepicker/scrollbar styles), `app/new/layout.tsx` (bg-gray-50), `Sidebar.tsx` (white bg, gray borders, red accent)
  - 13 UI primitives: Card, Modal, Drawer, Input, Select, Textarea, Button, Badge, PageHeader, StatCard, Skeleton, EmptyState, PrintButton
  - 10 pages: dashboard, bookings, calendar, room-types, housekeeping, maintenance, inventory, reports, billing, rates
  - 5 additional pages: inventory/items, inventory/rooms, inventory/transactions, billing/[id], admin/sync
  - 22 shared components: forms, modals, pickers, housekeeping, maintenance, inventory, DataTable, BookingDetailDrawer, StayTimeline, RateCalendar
  - Color mapping: zinc-950→gray-50, zinc-900→white, zinc-800→gray-100, text-zinc-100→text-gray-900, etc.
  - Preserved: red-600 accent buttons, status colors (emerald/amber/sky/orange), print templates (light)

## [2.18.0] - 2026-02-18

### Added
- **Unified architecture foundation** — new shared utilities, types, UI primitives, sidebar navigation, and unified layout for the single-system redesign
  - `lib/format.ts` — consolidated formatting utilities (`formatCurrency`, `toBuddhistYear`, `formatBuddhistDate`, `formatDateForApi`, `formatThaiDate`, `calculateNights`) from 8+ duplicate implementations
  - `lib/status.ts` — centralized status color/label maps for bookings, rooms, housekeeping, maintenance, payments with `getStatusColor()`/`getStatusLabel()` helpers
  - `types/common.ts`, `types/booking.ts`, `types/customer.ts`, `types/room.ts`, `types/checkin.ts` — shared TypeScript type definitions extracted from scattered page-level types
  - 12 UI primitives in `components/ui/`: Badge, Button, Card, Modal, Drawer, Input, Select, Textarea, PageHeader, StatCard, Skeleton, EmptyState
  - `components/Sidebar.tsx` — collapsible left sidebar navigation (240px/64px) with localStorage persistence, responsive defaults, and smooth transitions
  - `app/(unified)/layout.tsx` — unified layout using Sidebar with synchronized collapse state

- **PostgreSQL paths for remaining SQL Server endpoints** (Phase A — SQL Server independence)
  - `GET /api/rooms/status` — new `get_room_status_pg()` using `generate_series()` + joins on `ht_rooms_legacy`, `ht_checkins_legacy`, `ht_bookings_legacy` to replicate `View_Room_status` behavior
  - `GET /api/bookings/:id` — new `get_booking_pg()` querying `ht_bookings_legacy` with `book_total` support, dispatched via `use_pg_source()` feature flag
  - `GET /api/calendar` — new `fetch_legacy_calendar_data_pg()` querying PG mirror tables for bookings and check-ins, dispatched via `use_pg_source()` feature flag

## [2.17.1] - 2026-02-18

### Fixed
- **Sync: invalid column names** — removed `Book_Room_No` from booking sync and `Cin_CheckIn_No` from check-in sync (columns don't exist in SQL Server views)
- **Sync: customer truncation** — widened `cust_no`, `cust_type`, `cust_phone`, `cust_idcard` columns in `ht_customers_legacy` to prevent "value too long" errors
- **Sync: datetime panics** — use `try_get()` for all datetime fields in booking/check-in sync to handle empty/invalid values gracefully

## [2.17.0] - 2026-02-18

### Added
- **One-time legacy data migration CLI** (`cargo run --bin migrate_legacy`) — imports all historical data from SQL Server into PostgreSQL in a single transaction
  - Extracts distinct room types from legacy rooms into `ht_room_types`
  - Imports rooms (`HT_Rooms` -> `ht_rooms_new`) with floor parsing and type linking
  - Imports customers (`View_Customers` -> `ht_customers`) with name splitting (first/last)
  - Imports bookings (`View_Booking_Ds` -> `ht_bookings` + `ht_booking_rooms`) grouped by Book_No
  - Imports check-ins (`View_CheckIn_Ds` -> `ht_checkins`) with customer/room linking
  - Bumps PostgreSQL sequences past max imported IDs to avoid conflicts
  - All imported records tagged with `source = 'legacy'`
- **Status code mapping**: Legacy `Book_Status` integers mapped to string statuses (1=confirmed, 2=checkedin, 3=completed, 4=cancelled, 0/other=pending)
- **Safety features**: Full transaction rollback on error, `--dry-run` flag, idempotent (skips existing records)

## [2.16.0] - 2026-02-18

### Added
- **Legacy-to-PostgreSQL background sync** — new scheduler job replicates data from SQL Server every 5 minutes using SHA256 change detection, enabling gradual migration away from the legacy database
  - `ht_rooms_legacy` mirrors `HT_Rooms`
  - `ht_bookings_legacy` mirrors `View_Booking_Ds` (composite key: book_no + room_type)
  - `ht_checkins_legacy` mirrors `View_CheckIn_Ds` (unique key: cin_no)
  - `ht_customers_legacy` mirrors `View_Customers` (unique key: cust_no)
  - `sync_status` table tracks per-entity sync health, timing, and error counts
- **Sync status API** — `GET /api/new/sync/status` returns last sync time, record counts, and health indicator per entity type
- **Sync admin dashboard** — `app/new/admin/sync/page.tsx` displays real-time sync health with auto-refresh every 30 seconds
- **Data source tracking** — `source` column added to `ht_bookings`, `ht_checkins`, `ht_customers` to distinguish between 'new' (app-created) and 'legacy' (synced) records
- **SYNC_ENABLED environment variable** — set to `false` to disable the background sync job without code changes

- **`LEGACY_READ_SOURCE` feature flag** — all legacy read routes now default to PostgreSQL mirror tables; set `LEGACY_READ_SOURCE=sqlserver` to fall back to direct SQL Server queries
  - `GET /api/rooms` — reads from `ht_rooms_legacy` (with `ht_rooms_new` price overrides)
  - `GET /api/rooms/:id` — room detail + current guest from `ht_checkins_legacy`
  - `GET /api/rooms/checkouts-today` — checkout detection from PG
  - `GET /api/bookings` — paginated bookings from `ht_bookings_legacy`
  - `GET /api/checkins` — paginated check-ins from `ht_checkins_legacy`
  - `GET /api/customers` — search/sort/pagination from `ht_customers_legacy`
  - `GET /api/customers/:id/bookings` — booking history from `ht_bookings_legacy`
  - `GET /api/customers/:id/stats` — customer stats from PG mirror tables
  - `GET /api/stats` — dashboard statistics from PG mirror tables
  - `GET /api/occupancy` — occupancy trends from `ht_checkins_legacy`
  - Exceptions: `GET /api/rooms/status` and `GET /api/bookings/:id` still use SQL Server (no PG equivalent)

### Changed
- Scheduler `init_scheduler()` now accepts an optional `PgPool` for the sync job (backwards compatible — Slack notification jobs unchanged)
- All legacy read routes refactored with dual-source pattern: PG implementation + SQL Server fallback per endpoint

## [2.15.2] - 2026-02-07

### Fixed
- Fixed all legacy endpoint datetimes showing 7 hours behind actual time — `NaiveDateTime` from SQL Server now converted to `DateTime<Utc>` with `Z` suffix so frontend `timeZone: 'UTC'` displays stored Thai time correctly
  - Affected: checkins, bookings, rooms (detail + status), customers (list, booking history, stats), calendar (legacy + new sources)
  - 19 datetime fields across 10 structs updated

## [2.15.1] - 2026-02-07

### Changed
- Renamed CI/CD pipeline jobs for consistency: `test` → `test-frontend`, `build-and-push` → `build-frontend`
- Simplified `build-backend` condition — removed unnecessary `always()` and `skipped` check
- Simplified `deploy` dependencies — removed redundant `test-backend` from `needs`

## [2.15.0] - 2026-02-07

### Added
- **Compile-time SQL verification with `sqlx::query!()` macros** — ~76 static SQL queries now verified at compile time against the PostgreSQL schema, catching column name typos, type mismatches, and schema drift before runtime
  - Dynamic queries (~30) that build SQL with string concatenation remain as `sqlx::query()` runtime queries
  - `DECIMAL` columns use `::float8` casts for ergonomic `f64` return types
  - Added `bigdecimal` feature to sqlx for `NUMERIC` parameter binding
- **`.sqlx/` offline compilation cache** — 76 query cache files enable compilation without a live database connection
  - `SQLX_OFFLINE=true` environment variable enables offline mode in Docker builds and CI
  - `scripts/sqlx-prepare.sh` helper script to regenerate the cache after query changes
- **Backend integration tests** — New `hotel-backend/tests/` directory with database integration tests
  - `test_schema.rs` — Validates all 18 expected tables exist and `schema_migrations` has baseline row
  - `test_customers.rs` — Customer CRUD lifecycle and search tests
  - `test_rooms.rs` — Room CRUD, status updates, and room type association tests
  - `test_bookings.rs` — Booking creation with room assignments and cancellation
  - `test_payments.rs` — Payment recording and void (soft delete) tests
  - `test_stats.rs` — Dashboard statistics query validation
  - Shared test infrastructure (`tests/common/mod.rs`) with `TEST_` prefix cleanup
- **CI/CD backend test job** — `test-backend` job runs integration tests against PostgreSQL 17 service before Docker build

### Changed
- Backend sqlx features updated: added `macros` and `bigdecimal` (was runtime queries only)
- Dockerfile updated with `SQLX_OFFLINE=true` and `.sqlx/` directory copy for offline compilation
- Backend version bumped to 2.7.0

### Fixed
- **Latent SQL bugs caught by compile-time verification**:
  - `new_bookings.rs` — `br_total_price` column reference that doesn't exist in `ht_booking_rooms`
  - Various type mismatches between `Option<T>` struct fields and NOT NULL database columns

## [2.14.0] - 2026-02-07

### Added
- **Automated PostgreSQL migration system** — Schema changes are now automatically applied during CI/CD deployment
  - `scripts/migrate.sh` — Migration runner that applies pending `migrations/pg/*.sql` files
  - `scripts/backup-db.sh` — Manual database backup utility
  - `schema_migrations` table tracks applied migrations with version, filename, checksum, and timestamp
  - Pre-migration `pg_dump` backups created automatically (keeps last 10)
  - Each migration runs in a transaction — rolls back on failure
  - Backup pruning to prevent disk bloat
  - `migrations/pg/000_baseline.sql` — Baseline marker for initial schema
- **CI/CD pipeline integration** — Deploy job now copies migration files, runs `migrate.sh` after DB health check, and restarts backend
- **Path filter expansion** — Pipeline triggers on changes to `migrations/pg/**` and `scripts/migrate.sh`

## [2.13.1] - 2026-02-07

### Fixed
- **Backend crash-loop due to PostgreSQL port mismatch** - PostgreSQL container was listening on default port 5432 while backend expected port 5439 (`NEW_DB_PORT=5439`). Added `PGPORT=5439` environment variable to `newdb` service and updated healthcheck to use `-p 5439`. This caused ALL APIs (legacy + new) to fail since the backend couldn't start.
- **StatsCard dark theme on legacy page** - Reverted `StatsCard.tsx` from dark theme colors (`bg-zinc-900`, `text-zinc-100`) back to light theme (`bg-white`, `text-gray-900`). The component is only used by the legacy light-themed dashboard and was accidentally changed during v2.12.0 dark theme redesign.

### Added
- **StatsCard regression test** (`__tests__/components/StatsCard.test.tsx`) - 8 tests verifying rendering, light theme colors, and subtitle behavior
- **Legacy Dashboard regression test** (`__tests__/components/LegacyDashboard.test.tsx`) - 9 tests covering loading state, stats cards, room grid, occupancy chart, recent activity, empty states, and API error handling
- **Playwright E2E test setup** - End-to-end testing framework for the legacy dashboard
  - `playwright.config.ts` - Chromium-only config targeting localhost:3003
  - `e2e/legacy-dashboard.spec.ts` - 4 E2E tests: page load, stats cards, room grid, navigation
  - `test:e2e` script in package.json

## [2.13.0] - 2026-02-07

### Changed
- **Migrate HotelNew database from SQL Server to PostgreSQL** - Major infrastructure change
  - Replaced SQL Server 2022 container (~2GB RAM, 1.6GB image) with PostgreSQL 17 Alpine (~50-100MB RAM, ~100MB image)
  - Backend now uses `sqlx` crate for PostgreSQL queries (replacing tiberius/bb8 for HotelNew DB)
  - Legacy database (192.168.100.222) unchanged - still uses tiberius/bb8 for read-only access
  - Converted all 14 route files from T-SQL to PostgreSQL syntax
  - Converted DDL init script (`init-db/init-hotelnew.sql`) to PostgreSQL
  - Stored procedures replaced with PL/pgSQL functions
  - Updated `docker-compose.yml` for PostgreSQL service
  - PostgreSQL auto-initializes from `/docker-entrypoint-initdb.d/` (no manual init needed)
- **Updated CI/CD pipeline for PostgreSQL** - Removed `sqlcmd` database initialization step (PostgreSQL auto-initializes)
- **Updated documentation for PostgreSQL migration** - `.env.example`, `hotel-backend/README.md`, `migrations/README.md`
- **Bumped Rust Docker image from 1.83 to 1.85** - Required by `base64ct` crate needing Rust edition 2024

## [2.12.0] - 2026-02-07

### Changed
- **New system dark theme redesign** - Complete visual overhaul of the new system (`/new/*`) with a professional dark black/red color scheme
  - Dark backgrounds (zinc-950/900/800) replacing white/gray
  - Red accent color (red-600/500/400) replacing emerald/blue/purple
  - DM Sans font replacing Inter for a more professional look
  - Updated all pages: dashboard, bookings, calendar, room-types, rates, housekeeping, maintenance, reports, inventory, billing
  - Updated all shared components: DataTable, StatsCard, NewNavbar, ModeToggle
  - Updated all form components: BookingForm, RoomTypeForm, RateForm, CustomerForm, InventoryItemForm
  - Updated all modal components: QuickCheckInModal, CheckOutModal, GuestRegistryModal, StockAdjustmentModal, PaymentModal, MaintenanceRequestModal
  - Updated picker/misc components: CustomerPicker, RoomPicker, MaintenanceCard, RoomInventoryChecklist, RateCalendar, StayTimeline, BookingDetailDrawer, PrintButton
  - Dark scrollbar and date picker CSS overrides via `.new-system-layout` class
  - Legacy system appearance unchanged (only "New System" switch button color updated)

## [2.11.2] - 2026-02-06

### Fixed
- **Legacy main page** - Removed check-in/check-out functionality (legacy is read-only)

### Changed
- **CI/CD workflow** - Added cancel-previous job to handle stuck runs

## [2.11.1] - 2026-02-06

### Fixed
- **Legacy customers page not displaying customers** - Removed ModeContext dependency that caused page to use wrong API endpoint when localStorage had 'new' mode saved from previous session

### Added
- **Calendar page for new system** (`/app/new/calendar/page.tsx`) - Moved calendar functionality to new system
  - Uses hybrid calendar endpoint to show both legacy and new bookings/check-ins
  - Added calendar link to NewNavbar navigation

### Changed
- **Separated legacy and new mode dependencies** - Legacy pages now always use legacy APIs, new pages always use new APIs (no more mode context interference)

## [2.11.0] - 2026-02-06

### Added
- **Reports Dashboard (Phase 1)** - Analytics and reporting at `/new/reports`
  - **Reports Page** (`/app/new/reports/page.tsx`) - Revenue and occupancy analytics
    - Date range picker with preset options (last 7, 14, 30 days, last month)
    - Period grouping selector (Day/Week/Month)
    - Stats cards: Total Revenue, Occupancy Rate, ADR, RevPAR, Avg Stay Length
    - Revenue trend chart (bar/line toggle)
    - Room type revenue pie chart breakdown
    - Thai language labels throughout
  - **Chart Components** (`/components/Charts.tsx`)
    - `RevenueChart` - Bar/line chart for revenue trends using Recharts
    - `PieChart` - Room type revenue breakdown with color-coded segments
    - Formatted tooltips showing revenue in Thai Baht and booking counts
  - **Report Types** (`/types/reports.ts`)
    - `RevenueDataPoint`, `RevenueResponse` - Revenue report data structures
    - `OccupancyResponse` - Occupancy metrics with ADR/RevPAR
    - `RoomTypeRevenue`, `RevenueByRoomTypeResponse` - Room type breakdown
    - `MaintenanceRequest`, `MaintenanceCategory` - Maintenance types
    - `Payment`, `PaymentsResponse` - Payment tracking types
- **Maintenance Request System (Phase 3)** - Kanban-style maintenance tracking at `/new/maintenance`
  - **Maintenance Page** (`/app/new/maintenance/page.tsx`) - Main maintenance dashboard
    - Three-column Kanban board: "Open" (red), "In Progress" (yellow), "Completed" (green)
    - Request cards showing title, room, category, priority badge, and time elapsed
    - Priority indicators with color coding (High=red, Medium=yellow, Low=gray)
    - Overdue badge for requests waiting > 2 hours
    - Filters: room, category, priority
    - Add request button opens modal form
    - Auto-refresh every 30 seconds
    - Thai language labels throughout
  - **MaintenanceCard Component** (`/components/maintenance/MaintenanceCard.tsx`)
    - Displays request number, title, room, category, priority
    - Time elapsed since created or since started
    - Assigned technician display
    - Quick action buttons: "Start Repair" (open -> in_progress), "Done" (in_progress -> completed)
    - Edit button to modify request details
    - Resolution and cost display for completed requests
  - **MaintenanceRequestModal Component** (`/components/modals/MaintenanceRequestModal.tsx`)
    - Create mode: room picker, category dropdown, title, description, priority, assignedTo
    - Edit mode: adds resolution, cost fields
    - Priority selection with color-coded buttons
    - Validation for required fields
  - **Backend API** (`/api/new/maintenance/*`)
    - `GET /api/new/maintenance/categories` - List maintenance categories
    - `GET /api/new/maintenance/requests` - List requests with filters (status, room, category, priority)
    - `POST /api/new/maintenance/requests` - Create request (generates MR-YYMM-NNNN format)
    - `GET /api/new/maintenance/requests/:id` - Get single request
    - `PUT /api/new/maintenance/requests/:id` - Update request
    - `PUT /api/new/maintenance/requests/:id/status` - Quick status update
  - **Database Migration** (`migrations/007_maintenance_system.sql`)
    - `HT_Maintenance_Categories` table with default categories (Electrical, Plumbing, AC, Furniture, General)
    - `HT_Maintenance_Requests` table with status, priority, cost, resolution tracking
    - `SQ_Maintenance_No` sequence for request number generation
- **Thai Labels**:
  - "แจ้งซ่อม" (Maintenance Request)
  - "รอดำเนินการ" (Pending) / "กำลังดำเนินการ" (In Progress) / "เสร็จสิ้น" (Completed)
  - "ความเร่งด่วน" (Priority): "สูง" (High), "ปานกลาง" (Medium), "ต่ำ" (Low)
  - "เริ่มซ่อม" (Start Repair) / "ซ่อมเสร็จ" (Done)
  - "ผลการซ่อม" (Resolution) / "ค่าใช้จ่าย" (Cost)
- **Test Coverage**
  - `PaymentModal.test.tsx` - Payment modal component tests (form inputs, API submission, validation)
  - `MaintenanceRequestModal.test.tsx` - Maintenance modal component tests (create/edit modes)
  - `MaintenanceCard.test.tsx` - Maintenance card component tests (status changes, priority display)
  - `Charts.test.tsx` - Chart components tests (OccupancyChart, RevenueChart, PieChart)

## [2.10.0] - 2026-02-06

### Added
- **Billing Module** - Invoice viewing and printing functionality at `/new/billing`
  - **Billing List Page** (`/app/new/billing/page.tsx`) - Check-in list with invoice actions
    - Search by guest name or check-in number
    - Filter by status (all, active, checked out)
    - Date range filter for check-in/check-out dates
    - Table showing: Check-in number, Room, Guest Name, Check-in Date, Checkout Date, Total Amount, Status
    - "View Invoice" button linking to invoice detail page
    - Pagination with page navigation
    - Thai language labels throughout
  - **Invoice Detail Page** (`/app/new/billing/[id]/page.tsx`) - Individual invoice view with print
    - Fetches invoice data from `/api/new/checkins/:id/invoice`
    - Displays InvoiceTemplate component with hotel and guest information
    - Print button with PDF save option
    - Back button to return to billing list
    - Loading and error state handling
    - Hotel info: The HF Hotel with Thai address and tax ID
- **Payment Tracking System (Phase 2)** - Multiple payments per check-in support
  - **Database Schema** (`migrations/006_payment_tracking.sql`)
    - `HT_Payments` table for tracking multiple payments per check-in
    - Supports payment methods: cash, credit, transfer, QR code
    - Soft delete (void) capability for payment corrections
    - Reference field for card/transfer numbers
    - Automatic balance calculation (total - paid)
  - **Backend API** (`/api/new/checkins/:id/payments`)
    - `GET /api/new/checkins/:id/payments` - List payments with balance summary
    - `POST /api/new/checkins/:id/payments` - Record a new payment
    - `DELETE /api/new/payments/:id` - Void a payment (soft delete)
  - **PaymentModal Component** (`/components/modals/PaymentModal.tsx`)
    - Amount input with auto-fill remaining balance option
    - Payment method selection buttons (Cash, Credit Card, Transfer, QR)
    - Optional reference field for card/transfer numbers
    - Notes field for additional information
    - Balance summary display (total, paid, remaining)
    - Thai language labels throughout

## [2.9.0] - 2026-02-06

### Fixed
- **Legacy Database Read-Only Enforcement** - Fixed booking notes writing to legacy database
  - Moved `HT_Booking_Notes` table from legacy database to HotelNew database
  - Updated booking routes to use dual-pool architecture:
    - `GET /api/bookings/:id` - Uses legacy DB for booking data, HotelNew for notes
    - `GET /api/bookings/:id/notes` - Uses HotelNew DB (read)
    - `POST /api/bookings/:id/notes` - Uses HotelNew DB (write)
    - `DELETE /api/bookings/:id/notes` - Uses HotelNew DB (write)
  - Legacy database (192.168.100.222) is now truly read-only

### Changed
- **Backend Architecture** - `bookings.rs` now uses `AppState` instead of `DbPool` for booking detail and notes routes
- **Route Configuration** - Booking notes routes moved from `legacy_routes` to `new_routes` in main.rs
- **Optional Legacy Database** - The app can now run without a legacy database connection
  - When `SYSTEM_MODE=new`, the app starts even if legacy database is unavailable
  - Legacy routes (`/api/rooms`, `/api/bookings`, `/api/checkins`, etc.) return 404 when legacy DB is unavailable
  - HotelNew database is required in New mode
  - Scheduler (checkout reminders) only runs when legacy database is available

### Added
- Migration `005_move_booking_notes_to_hotelnew.sql` - Creates HT_Booking_Notes table in HotelNew database
- `HT_Booking_Notes` table definition added to `init-db/init-hotelnew.sql`
- `create_new_pool` function exported from db module for standalone HotelNew connections

## [2.8.0] - 2026-02-05

### Added
- **Self-Hosted Database for HotelNew** - Dedicated SQL Server Docker container for the new hotel management system
  - **Docker Infrastructure**
    - New `newdb` service in `docker-compose.yml` running `mcr.microsoft.com/mssql/server:2022-latest`
    - SQL Server Express edition (free, suitable for hotel workloads)
    - Data persistence via Docker volume `newdb_data`
    - Health check with automatic service dependency management
    - Internal Docker network (`hotel-network`) - database not exposed to host
  - **Database Initialization**
    - `init-db/init-hotelnew.sql` - Complete database bootstrap script
    - Combines all migrations (002, 003, 004) into single idempotent script
    - Creates HotelNew database with all tables, indexes, sequences, and stored procedures
  - **Environment Configuration**
    - `NEW_DB_SERVER=newdb` - Backend connects to Docker container via service name
    - `NEW_DB_PASSWORD=<see GH secret>` - Strong password for SA account
    - `SYSTEM_MODE=new` - Default to New Mode for fresh deployments
  - **Documentation**
    - Updated `CLAUDE.md` with dual-database architecture diagram
    - Updated `.env.example` with new database configuration pattern
    - First-time setup instructions for database initialization
  - **CI/CD Pipeline Updates** (`.github/workflows/docker-build.yml`)
    - Pipeline now copies `init-db/` folder to production server
    - Automatic database health check with 2-minute timeout
    - Automatic database initialization after container is healthy
    - Idempotent deployment - safe to run from scratch or on existing setup
    - Detailed logging for deployment troubleshooting

### Changed
- **Backend database connection** - `NEW_DB_SERVER` now points to Docker container (`newdb`) instead of external server (192.168.100.222)
- **System mode default** - Changed from `legacy` to `new` in docker-compose.yml for new deployments

### Security
- SQL Server container only accessible within Docker network (not exposed to host)
- Strong SA password enforced (was using a weak literal for legacy connection)

## [2.7.1] - 2026-02-05

### Added
- **Comprehensive Test Suite** - 509 new tests for New Mode components (555 total)
  - **Test Utilities** (`/__tests__/utils/`)
    - `mockFactories.ts` - Mock data factories for customers, rooms, bookings, check-ins, inventory, invoices
    - `commonMocks.ts` - Lucide icon mocks, fetch mocks, date mocks, browser mocks
    - `testUtils.tsx` - Custom render functions, Thai language assertions
    - `asyncUtils.ts` - Async testing helpers for loading, modals, debounce
  - **Tier 1 Critical Tests**
    - `QuickCheckInModal.test.tsx` - 29 tests for walk-in check-in
    - `CheckOutModal.test.tsx` - 39 tests for checkout process
    - `InvoiceTemplate.test.tsx` - 49 tests for invoice rendering
  - **Picker Component Tests**
    - `CustomerPicker.test.tsx` - 43 tests including keyboard navigation
    - `RoomPicker.test.tsx` - 43 tests for multi-select room selection
  - **Form Component Tests**
    - `CustomerForm.test.tsx` - 35 tests for customer CRUD
    - `RoomTypeForm.test.tsx` - 34 tests for room type configuration
    - `InventoryItemForm.test.tsx` - 37 tests for inventory items
    - `RateForm.test.tsx` - 46 tests for special rates
  - **Operations Component Tests**
    - `HousekeepingStats.test.tsx` - 13 tests for stats display
    - `RoomCleaningCard.test.tsx` - 33 tests for cleaning workflow
    - `StockAdjustmentModal.test.tsx` - 36 tests for stock management
    - `RoomInventoryChecklist.test.tsx` - 34 tests for room inventory

## [2.7.0] - 2026-02-05

### Added
- **Inventory Management System** - Phase 4 inventory tracking module at `/new/inventory`
  - **Inventory Dashboard** (`/app/new/inventory/page.tsx`) - Main inventory overview
    - Summary cards: Total items count, Low stock alerts count, Categories count
    - Quick action buttons: Add Item, Stock Adjustment, View Transactions
    - Low stock alerts section showing items below minimum threshold
    - Recent transactions list with type indicators (IN/OUT/ADJUST/MOVE)
    - Click-through navigation to detailed pages
  - **Item Management** (`/app/new/inventory/items/page.tsx`) - Full CRUD for inventory items
    - Table view with columns: Code, Name, Category, Unit, Min Stock, Current Stock, Status, Actions
    - Search by item name or code
    - Filter by category (Minibar, Amenities, Linens, Equipment)
    - Low stock filter toggle
    - Sortable columns (Code, Name, Stock level)
    - Stock level indicators with color coding (green=good, yellow=low, orange=critical, red=out)
    - Inline stock adjustment and edit actions
  - **Room Inventory** (`/app/new/inventory/rooms/page.tsx`) - Per-room inventory view
    - Grid of room cards showing assigned inventory items
    - Room status indicators (checked today, has missing items, not checked)
    - Click room to open checklist modal
    - Filter by status (all, missing items, checked today)
    - Search by room number
    - Legend explaining status colors
  - **Transaction History** (`/app/new/inventory/transactions/page.tsx`) - Audit trail
    - Full transaction log with Date, Type, Item, Quantity, Room, Notes, By columns
    - Filter by transaction type (IN, OUT, ADJUST, MOVE)
    - Date range filter (from/to)
    - Search by item name/code
    - Print view functionality for reports
    - Stock change display (previous -> new)

- **Inventory Components**
  - `InventoryItemForm` (`/components/forms/InventoryItemForm.tsx`) - Modal for add/edit items
    - Fields: Item Code, Name, Category, Unit, Min Stock, Current Stock, Cost per Unit
    - Category dropdown with Thai labels
    - Unit dropdown with common units (pieces, bottles, boxes, sets, etc.)
    - Validation: unique code, non-negative stock values
    - Delete functionality with confirmation
  - `StockAdjustmentModal` (`/components/modals/StockAdjustmentModal.tsx`) - Quick stock changes
    - Item search with autocomplete
    - Three adjustment types: Add stock, Remove stock, Set stock (absolute)
    - Real-time preview of new stock level
    - Notes field for audit trail
    - Color-coded adjustment type buttons
  - `RoomInventoryChecklist` (`/components/inventory/RoomInventoryChecklist.tsx`) - Room verification
    - Checklist of items assigned to room
    - Checkbox and quantity input for each item
    - Items grouped by category
    - Missing items highlighted in orange
    - "Replenish" button to auto-create transactions for missing items
    - Notes field for housekeeper comments

- **Type Definitions** (`/types/inventory.ts`)
  - `InventoryItem` - Item data structure
  - `InventoryTransaction` - Transaction record structure
  - `RoomInventory` - Room inventory assignment
  - `InventoryCategory` - Enum: Minibar, Amenities, Linens, Equipment
  - `TransactionType` - Enum: IN, OUT, ADJUST, MOVE
  - Stock status helpers: `getStockStatus()`, `getStockStatusColor()`, `getStockStatusLabel()`

- **Thai Labels**:
  - "สินค้าคงคลัง" (Inventory)
  - "หมวดหมู่" (Category)
  - "จำนวนคงเหลือ" (Current Stock)
  - "ขั้นต่ำ" (Minimum)
  - "ปรับสต็อก" (Adjust Stock)
  - "รับเข้า" (Stock In)
  - "เบิกออก" (Stock Out)
  - "โอนย้าย" (Transfer)
  - "ปกติ/ใกล้หมด/วิกฤต/หมด" (Good/Low/Critical/Out stock status)

- **Categories**:
  - "Minibar" - เครื่องดื่ม/ของว่าง
  - "Amenities" - อุปกรณ์อำนวยความสะดวก
  - "Linens" - ผ้าและเครื่องนอน
  - "Equipment" - อุปกรณ์ในห้อง

- **Inventory Backend APIs** (Rust/Axum)
  - `GET/POST /api/new/inventory/categories` - Category management
  - `GET/POST /api/new/inventory/items` - Item CRUD with filters (category, low_stock, search)
  - `GET/PUT/DELETE /api/new/inventory/items/:id` - Item management
  - `GET/PUT /api/new/inventory/rooms/:room_id` - Room inventory assignment
  - `GET/POST /api/new/inventory/transactions` - Transaction log with stock updates
  - `GET /api/new/inventory/stats` - Dashboard statistics
  - `GET /api/new/inventory/low-stock` - Low stock alert items

- **Database Migration** (`migrations/004_create_inventory_tables.sql`)
  - `HT_Inventory_Categories` - Category definitions
  - `HT_Inventory_Items` - Item master with stock tracking
  - `HT_Room_Inventory` - Room-item assignments
  - `HT_Inventory_Transactions` - Stock movement audit log

## [2.6.0] - 2026-02-05

### Added
- **Housekeeping Module** - Kanban-style housekeeping board for room cleaning management at `/new/housekeeping`
  - **Housekeeping Page** (`/app/new/housekeeping/page.tsx`) - Main housekeeping dashboard
    - Three-column Kanban board: "Dirty" (red), "Cleaning" (yellow), "Ready" (green)
    - Room cards display room number, type, floor, and time in current status
    - Priority indicator for rooms that have been dirty > 2 hours
    - Floor filter dropdown to focus on specific floors
    - Auto-refresh every 30 seconds for real-time updates
    - Thai language labels throughout
  - **HousekeepingStats Component** (`/components/housekeeping/HousekeepingStats.tsx`) - Summary statistics
    - Total rooms needing cleaning count
    - Rooms currently being cleaned count
    - Rooms cleaned today count
    - Average cleaning time display (when available)
    - Color-coded stat cards matching Kanban columns
  - **RoomCleaningCard Component** (`/components/housekeeping/RoomCleaningCard.tsx`) - Individual room cards
    - Large room number display with room type and floor
    - Priority badge for urgent rooms (> 2 hours since checkout)
    - Time tracking: checkout time, time in current status
    - Housekeeper assignment display (when available)
    - Expandable notes field for housekeeper comments
    - Action buttons: "Start Cleaning", "Done", "Mark as Dirty"
    - Visual status indicators with color coding
- **Thai Labels**:
  - "Dirty Room" - "Waiting for Cleaning"
  - "Cleaning" - "In Progress"
  - "Ready" - "Clean Room"
  - "Start Cleaning" / "Done" action buttons

## [2.5.0] - 2026-02-05

### Added
- **Phase 3 Financial Backend APIs** - Rust/Axum backend endpoints for rate management and financial reports
  - **Rate Management API** (`/api/new/rates`) - Full CRUD for room rates
    - `GET /api/new/rates` - List all rates with optional `room_type_id` and `active` filters
    - `POST /api/new/rates` - Create a new rate (multiplier or fixed type)
    - `GET /api/new/rates/:id` - Get single rate details
    - `PUT /api/new/rates/:id` - Update rate configuration
    - `DELETE /api/new/rates/:id` - Delete a rate
    - Supports rate fields: name, room type, rate type (multiplier/fixed), value, valid date range, days of week, active status
  - **Financial Reports API** (`/api/new/reports`) - Revenue and occupancy analytics
    - `GET /api/new/reports/revenue?from=&to=&group_by=day|week|month` - Revenue report with period grouping
      - Returns: `{ data: [{ period, revenue, bookings }] }`
      - Revenue calculated from completed check-ins (rate per night x nights stayed)
    - `GET /api/new/reports/occupancy?from=&to=` - Occupancy statistics
      - Returns: occupancy_rate, total_rooms, occupied_nights, available_nights, ADR, RevPAR, avg_stay_length
      - Occupancy = (Occupied room-nights / Total available room-nights) x 100
    - `GET /api/new/reports/revenue-by-room-type?from=&to=` - Revenue breakdown by room type
      - Returns: `[{ room_type, revenue, percentage }]`
  - **Invoice Data API** (`/api/new/checkins/:id/invoice`) - Complete invoice data retrieval
    - Returns guest details, room assignment, rate calculations, totals
    - Includes all data needed for invoice/receipt generation

- **Database Migration** (`migrations/003_alter_ht_rates_table.sql`)
  - Alters HT_Rates table to support multiplier/fixed rate types
  - Adds Rate_Type (varchar), Rate_Value (decimal) columns
  - Renames date columns for API consistency
  - Adds Rate_Updated timestamp column

## [2.4.0] - 2026-02-05

### Added
- **Invoice and Receipt Generation** - Phase 3 financial features for hotel billing
  - `InvoiceTemplate` component (`/components/documents/InvoiceTemplate.tsx`) - Printable invoice layout
    - Hotel information header with logo, name, address, tax ID
    - Guest details section (name, ID card, contact)
    - Room charges table (room number, type, dates, nights, rate, subtotal)
    - Summary section: subtotal, discount, VAT (optional), grand total
    - Thai Buddhist Era dates (Gregorian + 543)
    - Thai/English bilingual labels
    - Print-optimized CSS with @media print rules for A4 paper
  - `ReceiptTemplate` component (`/components/documents/ReceiptTemplate.tsx`) - Payment confirmation document
    - Similar layout to invoice with payment details
    - Payment method and amount display
    - Receipt number field
    - "Paid in Full" indicator (ชำระเงินครบถ้วน / PAID IN FULL)
    - Signature lines for cashier and guest
  - `PrintButton` component (`/components/ui/PrintButton.tsx`) - Print/PDF action button
    - Triggers window.print() for browser printing
    - Dropdown option for "Save as PDF" via browser print dialog
    - Thai labels: "พิมพ์" (Print), "บันทึก PDF" (Save PDF)
    - Size variants (sm, md, lg)
    - Loading state during print operation
  - Type definitions (`/types/invoice.ts`)
    - `InvoiceData` - Invoice data structure
    - `InvoiceRoom` - Room charge line item
    - `HotelInfo` - Hotel information
    - `ReceiptData` - Receipt data extending InvoiceData

## [2.3.0] - 2026-02-05

### Added
- **Room Type Management for New Mode** - Full CRUD room type management at `/new/room-types`
  - `RoomTypeForm` component (`/components/forms/RoomTypeForm.tsx`) - Modal form for create/edit room types
    - Fields: Type Code, Type Name, Base Price, Max Guests, Bed Type, Room Size
    - Thai language labels
    - Validation for required fields
  - New Mode Room Types Page (`/app/new/room-types/page.tsx`)
    - Grid view of room types with cards
    - Each card shows type info, price, and amenities
    - Add/Edit/Delete functionality
  - Backend API: `/api/new/room-types` - Full CRUD with validation
    - Unique type code enforcement
    - Protection against deleting types in use

- **Guest Registry for New Mode** - TM.30 compliance guest tracking
  - `GuestRegistryModal` component (`/components/modals/GuestRegistryModal.tsx`)
    - Add additional guests to a check-in record
    - Guest fields: Name, ID Number, Nationality, Contact
    - Nationality dropdown with common countries
    - View and remove registered guests
  - Backend API: `/api/new/checkins/:id/guests` - Guest registry endpoints
    - GET - List all guests for a check-in
    - POST - Add a guest to a check-in
    - DELETE - Remove a guest from a check-in
    - Validates check-in is active before allowing changes

- **Booking Management UI for New Mode** - Full CRUD booking management at `/new/bookings`
  - `BookingForm` component (`/components/forms/BookingForm.tsx`) - Modal form for create/edit bookings
    - Thai language labels throughout
    - Buddhist Era (B.E.) date display with automatic night calculation
    - Customer picker integration with "Add New Customer" option
    - Multi-room selection via RoomPicker
    - Booking source dropdown (Walk-in, Phone, Online, OTA)
    - Deposit amount field
    - Combined notes field for special requests and internal notes
    - Cancel booking functionality with confirmation
  - `RoomPicker` component (`/components/pickers/RoomPicker.tsx`) - Visual room selector
    - Card-based room display with number, type, and price
    - Multi-select capability with selected room badges
    - Filter by room type
    - Rooms grouped by floor
    - Visual status indicators (available, occupied, maintenance, cleaning)
  - New Mode Bookings Page (`/app/new/bookings/page.tsx`)
    - Table with columns: booking number, date, status, customer, check-in, check-out, rooms
    - Search by booking number or customer name
    - Filter by status and date range
    - Add booking button
    - Click row to edit
    - Pagination with page navigation
    - Status badges with color coding

## [2.2.0] - 2026-02-05

### Added
- **Quick Check-In/Check-Out Modals for New Mode** - Dashboard room cards now support quick actions
  - `QuickCheckInModal` - Walk-in guest check-in form with customer search, expected checkout date picker, rate per night input
  - `CheckOutModal` - Checkout confirmation with stay summary, total calculation (nights x rate), payment method selection
  - Thai Buddhist Era (B.E.) date display support
  - Payment methods: Cash, Credit Card, Transfer
- **RoomGrid Enhanced for New Mode** - Room cards show action buttons when in New Mode
  - Available rooms: "Quick Check-In" button
  - Occupied/Checkout rooms: "Check-Out" button
  - Visual indicator for New Mode active

### Changed
- Dashboard page now detects system mode via `useMode()` hook
- Room grid displays appropriate actions based on room status and system mode

## [2.1.0] - 2026-02-05

### Added
- **Dual-Database Architecture** - Support for both legacy and new HotelNew database
  - New database `HotelNew` with application-owned tables (HT_Customers, HT_Rooms_New, HT_Bookings, HT_CheckIns, etc.)
  - Migration file `migrations/002_create_new_hotel_database.sql` with complete schema
  - Backend supports dual connection pools (legacy + new_hotel)
  - System mode toggle: Legacy (view-only) vs New (full CRUD)
- **Mode Toggle UI** - Navbar button to switch between Legacy and New modes
  - Mode persisted in localStorage
  - Visual indicators: amber for Legacy, green for New
  - Calendar page shows data source indicator
- **Hybrid Calendar Endpoint** - `/api/calendar` fetches from both databases in New mode
  - Color-coded entries by data source (legacy vs new)
  - Combined view of bookings and check-ins from both systems
- **New Database CRUD Routes** (Rust backend)
  - `/api/new/customers` - Full CRUD for HT_Customers
  - `/api/new/rooms` - Full CRUD for HT_Rooms_New
  - `/api/new/bookings` - Full CRUD for HT_Bookings with room assignments
  - `/api/new/checkins` - Check-in/check-out management
  - `/api/mode` - Get current system mode

### Changed
- Calendar page now uses mode context to fetch from appropriate endpoint
- Frontend wrapped with ModeProvider for global mode state

## [2.0.0] - 2026-02-05

### Changed
- **BREAKING: Backend Migration Complete** - All API endpoints now served by Rust/Axum backend
  - Frontend proxies API requests via Next.js rewrites to `http://backend:3003`
  - Removed all Next.js API routes (except `/api/changelog` which reads local CHANGELOG.md)
  - Removed `lib/db.ts`, `lib/scheduler.ts`, `lib/slack.ts`, and `instrumentation.ts`
  - Removed `mssql` and `node-cron` dependencies
  - Frontend is now purely a React UI layer

### Removed
- Next.js API routes: `/api/rooms/*`, `/api/bookings/*`, `/api/checkins`, `/api/customers/*`, `/api/stats`, `/api/occupancy`
- Database-related tests (`__tests__/api/`, `__tests__/integration/`)
- Test scripts: `test:db`, `test:api`, `test:slack`

## [1.19.1] - 2026-02-05

### Fixed
- **Rust backend build issues** - Fixed CI/CD pipeline failures:
  - Updated Dockerfile to use Rust 1.83 (yoke dependency requires rustc 1.82+)
  - Fixed bb8_tiberius error type conversion in error.rs
- **API proxy configuration** - Added Next.js rewrites to forward `/api/*` requests to Rust backend
- **Docker Compose configuration** - Added `BACKEND_URL` environment variable for frontend-to-backend communication

## [1.19.0] - 2026-02-05

### Added
- **Rust Backend Implementation** - Complete Rust/Axum backend in `hotel-backend/` directory
  - All 15 API endpoints ported from Next.js API routes to Rust
  - tiberius for SQL Server connection with bb8 connection pooling
  - tokio-cron-scheduler for background jobs (hourly reports, polling)
  - Slack notification integration with retry logic
  - Thai Buddhist date formatting utilities
  - Docker support with multi-stage build
  - Full API compatibility with existing React frontend

## [1.18.0] - 2026-01-30

### Added
- **Clickable bar chart segments** - Calendar stacked bar chart segments are now clickable
  - Click on any colored segment (continuing stays, new check-ins, or bookings) to view details
  - Detail panel shows list of stays with check-in date, check-out date, and number of nights
  - Bookings also display booking date in the detail view
  - Visual hover feedback on bar segments

## [1.17.0] - 2026-01-30

### Added
- **Stay Timeline Calendar** - New Gantt-style visualization for stays at `/calendar`
  - Horizontal bars showing stay duration (check-in to check-out)
  - Shows both check-ins (actual guests) AND bookings (reservations)
  - **Aggregates stays with same dates** - Groups identical check-in/check-out patterns with count
  - Daily occupancy heat bar showing room density per day
  - Color-coded by stay length (1 night = light blue, 7+ nights = purple)
  - Stats summary: total stays, check-ins, bookings, nights, and average stay
  - Stay length distribution breakdown (1 night, 2-3 nights, 4-7 nights, 7+ nights)
  - Month navigation with Thai Buddhist Era dates

### Changed
- **Calendar page completely redesigned** - Replaced grid calendar with timeline view
- **Simplified stay display** - Focus on stay patterns, not individual customer details

## [1.16.2] - 2026-01-29

### Changed
- **Dashboard stats cards redesign** - Clean white card design with improved spacing
  - Removed colored backgrounds and icons for cleaner look
  - Consistent white color scheme matching other dashboard cards
  - Responsive grid layout (2 columns mobile, 3 tablet, 4 desktop)

## [1.16.1] - 2026-01-29

### Fixed
- **Performance: Removed sluggish animations** - Eliminated `transition-all` and unnecessary `transition-colors` classes that caused layout thrashing and janky interactions:

### Added
- **Database migrations folder** (`/migrations/`) - SQL migration files for tracking database schema changes
  - `001_create_booking_notes_table.sql` - Documents the HT_Booking_Notes table created in v1.16.0
  - `README.md` - Migration guidelines, shared database warnings, and table ownership documentation
- **CLAUDE.md database migration instructions** - Mandatory process for creating migration files when modifying database schema

### Changed
- **Upgraded pnpm to version 10 in Dockerfile** - Matches CI workflow pnpm version, eliminates version mismatch warnings
  - `/app/bookings/page.tsx`: Removed `transition-all duration-300` from main content container, removed `transition-colors` from table rows
  - `/components/RoomGrid.tsx`: Removed `transition-all duration-200` from room cards (kept `hover:shadow-md`)
  - `/app/rooms/page.tsx`: Removed `transition-all` from filter cards and `transition-colors` from list rows
  - `/app/page.tsx`: Removed `transition-colors` from activity list items

### Changed
- **Moved react-datepicker CSS to root layout** - CSS now loads once in `/app/layout.tsx` instead of per-component in `/app/bookings/page.tsx`, reducing redundant CSS parsing

## [1.16.0] - 2026-01-29

### Added
- **Bookings Admin Console Overhaul** - Complete rewrite of the bookings page with improved UX:
  - Bookings now grouped by booking number (multi-room bookings show as single row)
  - Click any booking row to open detail drawer with full info
  - New booking notes feature - add, view, and delete notes per booking
  - Shows all rooms in a booking with room types
  - Customer details section
  - Enhanced search: search by booking number OR customer name

### Changed
- **API: /api/bookings** - Now returns grouped bookings instead of individual room records
- **New API: /api/bookings/[id]** - Get single booking detail with notes
- **New API: /api/bookings/[id]/notes** - CRUD operations for booking notes
- **New Component: BookingDetailDrawer** - Side drawer for comprehensive booking view
- **Database: HT_Booking_Notes table** - Auto-created on first note addition

## [1.15.9] - 2026-01-29

### Changed
- **Fixed Navbar to exactly 2 breakpoints** - Removed container class, use single lg: breakpoint (1024px). Desktop shows full text, mobile shows icons only. All menus always visible at top.

## [1.15.8] - 2026-01-29

### Changed
- **Improved Navbar responsive scaling** - Simplified to 2 states: desktop (full text) and mobile (icons only). Title now hides on mobile, added whitespace-nowrap to prevent text wrapping

## [1.15.7] - 2026-01-29

### Fixed
- **Fixed Docker build excluding CHANGELOG.md** - Added exception in .dockerignore to include CHANGELOG.md in build context

## [1.15.6] - 2026-01-29

### Fixed
- **Fixed CHANGELOG.md missing in Docker container** - Added CHANGELOG.md to Dockerfile runner stage so the /api/changelog endpoint can read it at runtime

## [1.15.5] - 2026-01-29

### Fixed
- **Fixed /api/changelog endpoint returning 500 error** - Changed from fetching GitHub releases API (which returned 404 for private/non-existent repo) to parsing local CHANGELOG.md file directly. This is more reliable and doesn't require external API calls or authentication.

## [1.15.4] - 2026-01-29

### Changed
- **Refactored CustomTooltip in Charts.tsx** - Moved outside OccupancyChart component to prevent unnecessary recreation on each render
- **Added ESLint 9 flat config** - Configured with Next.js and core-web-vitals rules

## [1.15.3] - 2026-01-29

### Changed
- **Added caching to middleware CI workflow** - Significantly speeds up Windows and macOS builds
  - Added Rust dependency caching using `Swatinem/rust-cache@v2` (~55% faster builds after initial run)
  - Added npm caching via `actions/setup-node@v4` cache option
  - Expected improvement: ~13 min → ~5-6 min (Windows), ~11 min → ~4-5 min (macOS)

## [1.15.2] - 2026-01-29

### Fixed
- **Fixed middleware CI workflow** - Corrected Rust toolchain action from non-existent `dtolnay/rust-action` to `dtolnay/rust-toolchain`, and fixed invalid `universal-apple-darwin` target by installing the correct `aarch64-apple-darwin` and `x86_64-apple-darwin` targets separately

## [1.15.1] - 2026-01-29

### Changed
- **Migrated package manager to pnpm** - Full migration from npm to pnpm for consistent tooling
  - Dockerfile now uses `corepack enable && corepack prepare pnpm@9` with `pnpm install --frozen-lockfile`
  - CI/CD workflow updated to use pnpm version 9 (was 8)
  - Removed `package-lock.json`, now using `pnpm-lock.yaml`
  - Benefits: faster installs, better disk efficiency, consistent with CI environment

## [1.15.0] - 2026-01-29

### Security
- **Upgraded Next.js from 15.5.11 to 16.1.6** - Resolves 1 Dependabot security alert:
  - MODERATE: Unbounded Memory Consumption via PPR Resume Endpoint (GHSA-5f7q-jpqc-wp7h) - fixed in >= 15.6.0
- **Upgraded ESLint from 8.57.1 to 9.39.2** - Required by eslint-config-next 16.x
- Upgraded eslint-config-next from 15.5.11 to 16.1.6

### Changed
- **ESLint configuration migrated to flat config format** (ESLint 9 requirement)
  - Added `eslint.config.mjs` with Next.js flat config
  - Added `@eslint/eslintrc` dependency for flat config support
  - Updated lint script to use `eslint .` (Next.js 16 removed `next lint` command)

### Fixed
- Moved `CustomTooltip` component outside of render function in `Charts.tsx` to fix React Hooks lint error

## [1.14.1] - 2026-01-29

### Changed
- **Middleware build pipeline migrated from Electron to Tauri** - `middleware-build.yml` now builds the Tauri-based Thai ID Middleware instead of the Electron version
  - Triggers on changes to `thai-id-middleware-tauri/` instead of `thai-id-middleware/`
  - Uses Rust toolchain with `dtolnay/rust-action` for cross-platform builds
  - Builds macOS Universal binary (Apple Silicon + Intel) and Windows x64
  - Produces smaller artifacts (~10MB vs ~150MB Electron)

### Removed
- `tauri-build.yml` workflow - consolidated into `middleware-build.yml`

## [1.14.0] - 2026-01-29

### Security
- **Upgraded Next.js from 14.2.35 to 15.5.11** - Resolves 4 Dependabot security alerts:
  - HIGH: HTTP request deserialization DoS via React Server Components (GHSA-qpjv-v59x-3qc4) - fixed in >= 15.0.8
  - HIGH: HTTP request deserialization DoS via React Server Components (duplicate alert) - fixed in >= 15.0.8
  - MEDIUM: Image Optimizer remotePatterns DoS (GHSA-qfcj-68r8-w26x) - fixed in >= 15.5.10
  - MEDIUM: Image Optimizer remotePatterns DoS (duplicate alert) - fixed in >= 15.5.10
- **Upgraded React from 18.3.1 to 19.1.0** - Required by Next.js 15
- Upgraded eslint-config-next from 14.2.35 to 15.5.11

### Changed
- **Breaking Change Migration (Next.js 15)**:
  - API route params are now async (Promise-based) - updated all dynamic routes
  - JSX namespace changed from `JSX.Element` to `React.JSX.Element`
  - Removed deprecated `experimental.instrumentationHook` from next.config.js (now enabled by default)

### Note
- **glib vulnerability (RUSTSEC-2024-0429)** - Unsoundness in `VariantStrIter` iterator
  - Status: **Cannot be fixed** - glib 0.18.5 is constrained by Tauri's GTK3 stack (gtk 0.18.x)
  - Impact: **Low** - Linux builds only, vulnerable API not used by application
  - The gtk-rs GTK3 bindings are unmaintained and pinned to glib 0.18.x
  - Fix will come when Tauri migrates to GTK4 or updates dependencies
  - Tracked upstream: waiting for Tauri ecosystem update

## [1.13.3] - 2026-01-29

### Added
- **Photo display in Tauri GUI** - Cardholder's photo now displays in the Tauri frontend when reading cards
  - Photo appears at the top of the debug output when clicking "Test Read"
  - Styled with rounded corners and blue border to match the app theme
- **Debug mode toggle** (🔧 button) in Tauri GUI header
  - When debug=off: Only shows status indicators (HTTP Server, Card Reader, Card) - 400×340px
  - When debug=on: Shows full UI with endpoints, debug tools, and footer - 400×760px
  - Fixed window sizes (non-resizable) that adjust when toggling debug mode
  - Starts in compact mode (debug=off) by default
- **System tray icon** restored - click to show/focus the main window
- **Photo reading support** in Thai ID Middleware Tauri - Read cardholder's photo from Thai ID card
  - New `?photo=true` query parameter for `GET /read` endpoint
  - Photo returned as base64-encoded JPEG in `data.photo` field
  - Photo reading adds ~2 seconds (20 APDU commands for 5KB JPEG data)
  - Example: `curl "http://localhost:9898/read?photo=true" | jq '.data.photo'`
- **Enhanced debug endpoint** (`GET /debug`) now returns comprehensive card information:
  - ATR (Answer To Reset) - card identification bytes
  - Protocol (T=0 or T=1) - smart card communication protocol
  - Reader name
  - AID test results - tests 4 known Thai ID card application IDs with status words
  - Raw read result - shows actual APDU response for CID read command
  - Human-readable status word descriptions (6A82 = File not found, etc.)

### Changed
- Thai ID Middleware Tauri version bumped to 1.1.0
- `read_card` Tauri command now accepts optional `include_photo` parameter
- CardData struct now includes optional `photo` field (base64 string)

## [1.13.2] - 2026-01-29

### Fixed
- **Tauri app crash on macOS** - Fixed SIGABRT crash during app launch
  - Root cause: PNG icon files had 16-bit color depth instead of 8-bit RGBA
  - Converted all icons (32x32.png, 128x128.png, 128x128@2x.png, icon.png) to 8-bit RGBA format
  - Simplified HTTP server lifecycle management to prevent premature shutdown
- **Card reader connection issues** - Fixed "smart card not responding to reset" error
  - Changed from `Protocols::ANY` to explicit `Protocols::T0` for Thai ID cards
  - Added fallback to T1 and ANY protocols if T0 fails
  - Thai ID cards use T=0 protocol which is now tried first for better compatibility

### Added
- **Debug mode for card reader** - Verbose logging can be enabled for troubleshooting
  - HTTP endpoints: `GET /debug`, `GET /debug/enable`, `GET /debug/disable`
  - Tauri commands: `set_debug(enabled)`, `get_debug()`
  - When enabled, logs APDU commands, responses, and connection details to stderr

### Removed
- System tray functionality (temporarily) - Removed to simplify debugging; will be re-added in future version

## [1.13.1] - 2026-01-29

### Fixed
- Memory leak in card reader page causing PC freezing - health check useEffect was using `[checkHealth]` dependency which could cause interval accumulation during re-renders or React Strict Mode double-mounting; changed to `[]` to ensure interval is created exactly once on mount

## [1.13.0] - 2026-01-29

### Added
- **Thai ID Middleware Tauri application** - Complete migration from Electron to Tauri for better macOS Gatekeeper support
  - New `thai-id-middleware-tauri/` directory with full Tauri 2.0 implementation
  - **Rust PC/SC card reader** (`card_reader.rs`) - Native implementation using `pcsc` crate
    - All APDU commands for Thai National ID cards (CID, names, DOB, gender, address, dates)
    - TIS-620 to UTF-8 Thai text encoding conversion
    - Retry logic for cold-inserted cards (5 retries, 1000ms delay)
    - Proper SW1=0x61 response handling with GET RESPONSE
  - **Axum HTTP server** (`server.rs`) - Rust HTTP API on port 9898
    - `GET /health` - Server and reader status
    - `GET /status` - Alias for /health
    - `GET /read` - Read Thai ID card data
    - CORS enabled for localhost web apps
  - **Tauri IPC commands** (`commands.rs`) - Frontend integration
    - `get_status`, `get_version`, `read_card`, `debug_card` commands
  - Frontend ported from Electron with Tauri API integration
- Benefits over Electron:
  - Smaller binary size (~10MB vs ~150MB)
  - Better macOS code signing and Gatekeeper compatibility
  - Lower memory usage
  - Native Rust performance for card operations

## [1.12.5] - 2026-01-29

### Added
- Comprehensive diagnostic logging for Thai ID card reader to help diagnose connection issues
  - Operation counter (`[op:N]`) to correlate connect/disconnect pairs across functions
  - Detailed logging in `resetCard()` showing connect success/failure and disconnect results
  - Detailed logging in `connectWithRetry()` showing each attempt, errors, and retry decisions
  - Entry/exit logging in `readCard()` and `debugCard()` with success/failure status
  - Protocol name logging (T=0/T=1) for successful connections
  - Now logs exact PC/SC error messages to help diagnose silent failures

### Changed
- `connectWithRetry()` now retries on any error, not just "unresponsive" errors
- Middleware version bumped to 1.1.5

## [1.12.4] - 2026-01-29

### Fixed
- Thai ID card reader reliability issues: cards becoming unreadable after ~30 seconds and cold-inserted cards failing
  - Root cause 1: `SCARD_LEAVE_CARD` disconnect mode leaves card in corrupted state after repeated use
  - Root cause 2: Insufficient retry time (1.5s) for cold-inserted cards needing full power cycle
  - Changed all `SCARD_LEAVE_CARD` to `SCARD_RESET_CARD` - performs warm reset clearing card state
  - Added `resetCard()` function to reset cards in unknown state before connecting
  - Increased retry parameters from 3×500ms to 5×1000ms (5 seconds total)
  - On first connection failure, attempts card reset before retrying
- Middleware version bumped to 1.1.4

## [1.12.3] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Card is unresponsive" error when app starts with card already inserted
  - Root cause: Race condition between card detection and card readiness during power-up sequence
  - Added `connectWithRetry()` helper that retries connection up to 3 times with 500ms delay
  - Both `readCard()` and `debugCard()` now use retry logic to handle cards still initializing
- Middleware version bumped to 1.1.3

## [1.12.2] - 2026-01-29

### Fixed
- Thai ID card reader returning empty data for all fields (CID, names, dates, etc.) despite successful card communication
  - Root cause: `readCard()` used plain `transmit()` instead of `transmitWithGetResponse()` for READ commands
  - When card returns SW1=61 (more data available), `transmitWithGetResponse()` sends GET RESPONSE to retrieve data
  - Also changed from parallel `Promise.all()` to sequential reads (smart cards are sequential devices)
- Middleware version bumped to 1.1.2

## [1.12.1] - 2026-01-29

### Fixed
- Thai ID card reader failing with "Failed to select Thai ID applet" on real Thai National ID cards
  - Root cause: Cards return SW 61XX (more data available) instead of 9000 for SELECT commands
  - SW1=61 is valid ISO 7816-4 success response meaning XX bytes of data are pending
  - Added `transmitWithGetResponse()` to automatically send GET RESPONSE (00 C0 00 00 XX) when needed
  - Updated `debugCard()` to recognize SW1=61 as success indicator
- Middleware version bumped to 1.1.1

## [1.12.0] - 2026-01-29

### Added
- Thai ID Middleware debug mode for diagnosing card reading issues
  - "Test Read" button to attempt card read and display results in the app
  - "Debug Info" button to show card ATR and test multiple Application IDs (AIDs)
  - Dark-themed output panel displaying diagnostic information
  - Tests multiple known Thai ID card AIDs (Standard, Alternate, MOI, EMV)
  - Shows APDU status words with human-readable descriptions
- ATR (Answer To Reset) capture when card is inserted for identification
- Window is now resizable to accommodate debug panel

## [1.11.1] - 2026-01-29

### Fixed
- Middleware build workflow failing with "flate: corrupt input before offset 79" on both Windows and macOS
- Root cause: icon.png was only 64x64 pixels, but electron-builder requires at least 512x512 for macOS and 256x256 for Windows
- Solution: Updated icon.svg to 512x512 dimensions and added SVG-to-PNG conversion step in workflow
  - macOS: Uses `librsvg` (`rsvg-convert`)
  - Windows: Uses `Inkscape` CLI

### Changed
- Icon is now generated from SVG during CI/CD build instead of being committed as PNG
- Updated `generate-icon.js` script with new 512x512 SVG design

## [1.11.0] - 2026-01-29

### Changed
- Thai ID Middleware distribution changed from source code (zip) to pre-built executables
- Card reader download page now offers platform-specific downloads (Windows .exe, macOS .dmg)
- Simplified installation: download and run, no npm required
- Added macOS Gatekeeper bypass instructions for unsigned app (right-click → Open or System Settings → Privacy & Security)

### Added
- GitHub Actions workflow (`middleware-build.yml`) for automated cross-platform builds
  - Builds Windows portable executable on `windows-latest`
  - Builds macOS disk image on `macos-latest`
  - Creates GitHub Release when manually triggered with version

### Removed
- `public/downloads/thai-id-middleware.zip` - replaced by GitHub Releases

## [1.10.0] - 2026-01-29

### Security
- Fixed npm vulnerabilities in thai-id-middleware: updated electron (^40.0.0) and electron-builder (^26.6.0)

### Added
- Middleware download available from card reader page (`/card-reader`) - users can download the zip file directly from the web app
- Thai ID Middleware Electron desktop app (`thai-id-middleware/`) for cross-platform Thai ID card reading
  - GUI status display: HTTP server, reader connection, card insertion status
  - HTTP server on localhost:9898 with `/health` and `/read` endpoints
  - System tray support for background operation
  - Cross-platform builds: Windows portable .exe, macOS .dmg, Linux .AppImage
  - PC/SC smart card communication using @pcsclite/client
  - Full Thai National ID card data reading: CID, names (Thai/English), DOB, gender, address, issue/expiry dates

## [1.9.0] - 2026-01-29

### Changed
- Reverted card reader from WebUSB to middleware approach (WebUSB blocked for CCID devices)

### Removed
- WebUSB card reader module (browser security prevents access to smart card readers)

## [1.8.0] - 2026-01-29

### Changed
- Card reader middleware URL is now configurable via `NEXT_PUBLIC_CARD_READER_URL` environment variable (build-time)

### Added
- Thai ID Card Reader POC page (`/card-reader`) for reading guest information from Thai national ID cards
- Connects to local middleware service on `localhost:9898` for PC/SC card reader communication
- Displays all card data: citizen ID, Thai/English names, birth date, gender, address, issue/expiry dates, and photo
- Connection status indicator with automatic health checks
- Setup instructions displayed when middleware is not running
- "ใช้ข้อมูลนี้" button for future check-in integration
- New "อ่านบัตร" navigation link in navbar

## [1.7.3] - 2026-01-29

### Changed
- Rooms page detail panel now displays Room_Group, Room_Book_Name (ผู้จอง), and all price tiers (A, B, C)
- Updated GuestInfo interface to match API response (`checkIn`/`checkOut` instead of `checkInDate`/`checkOutDate`)
- fetchRoomDetail now correctly handles new `/api/rooms/[id]` response structure

## [1.7.2] - 2026-01-29

### Added
- Room detail endpoint `/api/rooms/[id]` returning room details with current guest information from check-in records
- Additional room fields in `/api/rooms` API: `Room_PriceA`, `Room_PriceB`, `Room_PriceC`, `Room_Group`, `Room_Book_Name`

### Changed
- Room interface updated to use actual database column names (`Room_PriceA` instead of `Room_Price`, `Room_Group` instead of `Room_Floor`)

## [1.7.1] - 2026-01-28

### Fixed
- Customer API failing with "Invalid column name 'Book_Cust_No'" when `includeLastVisit=true` - changed to correct column name `Book_Cust_ID`
- Customer bookings API (`/api/customers/[id]/bookings`) using wrong column `Book_Cust_No` - changed to `Book_Cust_ID`
- Customer stats API (`/api/customers/[id]/stats`) using wrong columns - changed `Book_Cust_No` to `Book_Cust_ID` and `Cin_Cust_No` to `Cin_cust_no` (case-sensitive)
- Parameter types changed from `sql.Int` to `sql.NVarChar` since customer IDs are strings like "C0001"

## [1.7.0] - 2026-01-28

### Added
- Server-side sorting for customers table - sorts the entire dataset, not just the visible page
- "Last Visit" column in customers table showing each customer's most recent checkout date
- DataTable component now supports controlled server-side sorting via `onSort`, `sortColumn`, and `sortDirection` props

### Changed
- Customers API now accepts `sortBy` and `sortOrder` query parameters for server-side sorting
- Customers API supports optional `includeLastVisit=true` parameter to include last visit dates

## [1.6.9] - 2026-01-28

### Fixed
- Slack notification times displaying 7 hours ahead - now uses UTC for database dates (which store local Thai time) and Asia/Bangkok for current timestamps

## [1.6.8] - 2026-01-28

### Added
- Customer statistics cards in detail modal showing: total bookings, total stays, first/last visit dates, favorite room type, and average stay duration
- New API endpoint `/api/customers/[id]/stats` for customer statistics
- New API endpoint `/api/customers/[id]/bookings` for customer booking history

## [1.6.7] - 2026-01-28

### Changed
- Customer search now supports multiple fields: name, phone number, ID card (13-digit), and customer ID

## [1.6.6] - 2026-01-28

### Added
- Changelog page (`/changelog`) displaying GitHub release history
- New API endpoint `/api/changelog` fetching releases from GitHub API with 5-minute caching
- "ประวัติ" navigation link in navbar

## [1.6.5] - 2026-01-28

### Fixed
- `/customers` page not working due to API response structure mismatch - transformed SQL column names (`Cust_no`, `Cust_name`, etc.) to frontend-expected format (`id`, `name`, etc.) and flattened pagination response

## [1.6.4] - 2026-01-28

### Added
- `pnpm test:slack` script to run Slack integration tests with verbose output

## [1.6.3] - 2026-01-28

### Fixed
- Slack notifications not working in production Docker container - added `SLACK_WEBHOOK_URL` and `SLACK_NOTIFICATIONS_ENABLED` environment variables to docker-compose.yml
- Updated GitHub Actions workflow to pass Slack webhook secret during deployment

## [1.6.2] - 2026-01-28

### Changed
- RoomGrid mobile view now displays rooms as a sorted list instead of floor plan grid
- Desktop view retains the original floor plan grid layout
- Mobile list shows room number, type, and details with status indicator

## [1.6.1] - 2026-01-28

### Changed
- RoomGrid now mobile responsive with horizontal scroll, preserving floor plan layout
- Responsive cell sizes (60px mobile, 70px desktop), text sizes, and legend
- Fixed React key warning in RoomGrid row fragments

## [1.6.0] - 2026-01-28

### Added
- Checkout notifications: Real-time Slack alerts when guests check out (polled every 2 minutes via `Cin_Room_Out` field)
- New booking notifications: Real-time Slack alerts when new bookings are created (polled every 2 minutes via `Book_Date` field)
- New functions in `lib/slack.ts`: `buildCheckOutAlertMessage`, `buildNewBookingAlertMessage`
- New polling functions in `lib/scheduler.ts`: `pollCheckouts`, `pollNewBookings`

## [1.5.7] - 2026-01-28

### Fixed
- Added missing rooms V.201, A2-1, A2-3 to RoomGrid display on the 4th floor row

## [1.5.6] - 2026-01-28

### Changed
- Switched from cloudflared SSH to self-hosted GitHub Actions runner for deployment - simpler and more reliable

## [1.5.5] - 2026-01-28

### Added
- Automated deployment via SSH in CI/CD pipeline - after build, automatically deploys to production server via Cloudflare tunnel

## [1.5.4] - 2026-01-28

### Fixed
- Fixed stats API counts not matching room grid display - checkout queries now filter to only the most recent check-in record per room using MAX(Cin_Room_In) subquery, preventing historical records from incorrectly counting as today's checkouts

## [1.5.3] - 2026-01-26

### Fixed
- Fixed checkout-today query returning old records from guests who already checked out - now only returns rooms where guest is still checked in (`Room_Use = 'yes'`)

## [1.5.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch between stats card and room grid - stats now excludes checkout rooms (after 6 AM) from occupied count
- Occupied rooms count now matches the number of red squares on the grid

### Added
- New "ห้องรอเช็คเอาท์" stat card (blue) showing rooms waiting for checkout today

### Changed
- API integration tests now spin up their own Next.js dev server on port 30031, making tests self-contained and independent of manually running dev server

## [1.5.1] - 2026-01-26

### Fixed
- Fixed "รอเช็คเอาท์" (waiting for checkout) rooms not showing on grid - now uses `View_CheckIn_Ds.Cin_Room_Out` date matching (same method as stats API) instead of unreliable `View_Room_status.room_status` filtering
- Added new `/api/rooms/checkouts-today` endpoint for reliable checkout room detection

## [1.5.0] - 2026-01-26

### Added
- New "booked" (จองแล้ว) room status with yellow color to distinguish rooms with reservations from rooms with checked-in guests
- New "ห้องที่จองแล้ว" stat card on dashboard showing count of booked-but-not-checked-in rooms
- Separate tracking: "ห้องที่มีผู้เข้าพัก" now only counts checked-in guests, "ห้องที่จองแล้ว" counts pending arrivals

## [1.4.2] - 2026-01-26

### Fixed
- Fixed occupied room count mismatch in stats API - now correctly counts rooms with any non-empty `Room_Book` value, matching the RoomGrid display logic (was only counting `Room_Book = 'yes'`)

## [1.4.1] - 2026-01-26

### Added
- Slack integration test (`__tests__/integration/slack.test.ts`) that sends actual test messages to verify webhook connectivity
- Added `dotenv` dev dependency for loading environment variables in tests

## [1.4.0] - 2026-01-24

### Added
- Slack notifications for hotel operations
  - Hourly report: Occupied rooms count and today's new bookings (sent every hour at minute 0)
  - Check-in alerts: Real-time notifications when guests check in (polled every 2 minutes)
- New files: `lib/slack.ts`, `lib/scheduler.ts`, `instrumentation.ts`
- Environment variables: `SLACK_WEBHOOK_URL`, `SLACK_NOTIFICATIONS_ENABLED`
- `.env.example` file with all available configuration options

### Technical
- Uses `node-cron` for scheduling background tasks
- Leverages Next.js instrumentation hook for server-side startup
- Includes retry logic (3 attempts with exponential backoff) for Slack API calls
- Thai language message formatting with Buddhist Era dates

## [1.3.4] - 2026-01-24

### Changed
- Pinned pnpm version to 9.x in Dockerfile (was `pnpm@latest`) to prevent cache invalidation on pnpm updates
- Expanded .dockerignore to exclude .husky, .swc, .claude, .github, .vscode, and *.log files from build context

## [1.3.3] - 2026-01-24

### Security
- Added pnpm override to force glob >=10.5.0 to resolve CVE-2025-64756 (command injection in CLI)
  - Vulnerability is in `@next/eslint-plugin-next` dependency chain
  - Note: Only affects CLI usage; ESLint uses glob as a library, so actual risk is minimal

## [1.3.2] - 2026-01-24

### Security
- Upgraded Next.js from 14.2.21 to 14.2.35 to resolve 17 Dependabot security alerts:
  - Authorization Bypass in Middleware (High)
  - Race Condition to Cache Poisoning (High)
  - SSRF via Improper Middleware Redirect (High)
  - DoS with Server Components (Medium)
  - Content Injection in Image Optimization (Medium)
  - Cache Key Confusion in Image Optimization (Medium)
  - Information exposure in dev server (Medium)
- Upgraded eslint-config-next from 14.2.21 to 14.2.35 (resolves transitive glob vulnerability)

## [1.3.1] - 2026-01-24

### Security
- Fixed SQL injection vulnerability in `/api/checkins/route.ts` - now uses parameterized queries for startDate and endDate filters

### Added
- Component tests for RoomGrid (rendering, status colors, modal interaction)
- Component tests for DataTable (sorting, pagination, empty/loading states)
- Component tests for Calendar (month navigation, date selection, booking/checkin indicators)
- CI/CD pipeline now runs component tests before building Docker image
- Pre-push git hook to run component tests before pushing (via husky)

### Fixed
- Removed hardcoded database values from API tests to prevent false failures when data changes

## [1.3.0] - 2026-01-24

### Changed
- Room status indicator bar changed from vertical (left side) to horizontal (bottom)
- Legend moved from top to bottom of room grid

## [1.2.0] - 2026-01-24

### Added
- Docker containerization with multi-stage Dockerfile for optimized image size
- GitHub Actions CI/CD workflow for automated builds to ghcr.io
- docker-compose.yml for easy local deployment from container registry
- Environment variable support for database configuration (DB_SERVER, DB_NAME, DB_USER, DB_PASSWORD)

### Changed
- Next.js output mode set to 'standalone' for container deployment
- Database credentials now configurable via environment variables with backward-compatible defaults

## [1.1.1] - 2026-01-24

### Changed
- Migrated package manager from npm to pnpm for faster installs and better disk efficiency

## [1.1.0] - 2026-01-23

### Added
- "Waiting for Checkout" room status with light blue color
- Rooms with checkout date = today display as "รอเช็คเอาท์" after 6 AM

## [1.0.2] - 2026-01-23

### Fixed
- Fixed timezone display in Recent Activity: database stores local Thai time but mssql marks it as UTC, so using `timeZone: 'UTC'` displays the stored values correctly without adding 7 hours

## [1.0.1] - 2026-01-23

### Fixed
- Recent Activity section now shows check-in and check-out dates with times (e.g., "23 ม.ค. 04:32 - 24 ม.ค. 12:00")
- Fixed timezone display issue where UTC dates appeared as same-day ranges after conversion to Thai timezone

## [1.0.0] - 2026-01-23

### Added
- Initial release of Hotel Management Visualization Web App
- Dashboard with room status grid and occupancy statistics
- Room grid with custom layout matching physical hotel floor plan
- Real-time room status display (available, occupied, maintenance)
- Room details from database (Room_Type, Room_Details)
- Occupancy chart showing actual daily room counts (check-ins + stay-overs)
- Calendar page for viewing bookings and check-ins by date
- Customers page with search and pagination
- Bookings page with filtering options
- Rooms page with status management

### API Endpoints
- `/api/stats` - Dashboard statistics
- `/api/rooms` - Room listing with status
- `/api/bookings` - Booking management with pagination
- `/api/customers` - Customer search and listing
- `/api/checkins` - Check-in records
- `/api/occupancy` - Daily occupancy data for charts

### Technical
- Next.js 14 with App Router
- SQL Server database connection (mssql)
- Tailwind CSS for styling
- Recharts for data visualization
- Jest for testing (22 tests passing)
