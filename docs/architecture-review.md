# Architecture Review

**Date:** 2026-04-25
**Audience:** stakeholders + the engineer-of-one writing the writeback worker
**Companion doc:** `architecture-target.md` (visualization of the recommended end-state)

---

## A. Current state assessment

### A1. Surface area

| Area | Files | LOC | Where |
|---|---|---|---|
| Rust backend routes | 22 | **9,780** | `/Users/nut/new-hotel/hotel-backend/src/routes/*.rs` |
| Background workers | 2 | 1,068 | `/Users/nut/new-hotel/hotel-backend/src/scheduler/{sync.rs,jobs.rs}` |
| Standalone binaries | 2 | **2,047** | `/Users/nut/new-hotel/hotel-backend/src/bin/` |
| Next.js — legacy tree | 8 pages | — | `/Users/nut/new-hotel/app/(legacy)/` |
| Next.js — new tree | 17 pages | — | `/Users/nut/new-hotel/app/new/` |
| React components | ~30+ | — | `/Users/nut/new-hotel/components/` |
| Shared types | 7 | — | `/Users/nut/new-hotel/types/` |
| Docker / CI | 4 | — | `Dockerfile{,.ville-sync}`, `.github/workflows/docker-build.yml` |

**Total Rust footprint: ~12.9k LOC**, dominated by route handlers (`new_inventory.rs` = 1,340 lines, `new_checkins.rs` = 845, `rooms.rs` = 878). About 60% of these are simple PostgreSQL CRUD against `ht_*` tables — not Rust-specific business logic.

### A2. What works well

1. **`sqlx::query!()` compile-time SQL checking** — the `.sqlx/` cache catches column typos and type mismatches at build time
2. **`tiberius` MSSQL pool** — stable for the 5-min legacy sync. Handles SQL Server 2022 TDS 7.4
3. **Race-safe writeback now proven** — spike (`docs/legacy-spike/findings.md` §6) showed `TABLOCKX, HOLDLOCK` works against the live .NET app with zero collisions
4. **Mode-aware bootstrap** — `main.rs:64-187` already gracefully handles legacy-DB-down ("New mode") and new-DB-down ("Legacy mode") branches
5. **Single binary deploy** — `cargo build --release --bin hotel-backend` → one ~30MB executable on `debian:bookworm-slim`

### A3. What's painful

1. **CI build time** — even with `cargo-chef`, `mold`, `sccache`, and `Swatinem/rust-cache`, a clean backend build is multi-minute. Bun build is sub-second. Next.js build is ~30s.
2. **Dual frontend tree** (`app/(legacy)/*` + `app/new/*`) — paying double for every UI change
3. **`main.rs` mode logic is convoluted** — lines 126-214 are a comment thread of "let me restructure this." Will get worse when adding the writeback pool.
4. **`ville_sync.rs` shells out to `tsql`** (lines 48-66) — brittle string parsing. Reason: HF Ville's MSSQL is too old for `tiberius`. Any pure-Rust solution loses if a third site shows up.
5. **`HF Ville` is operationally second-class** — `deploy/hfville/` only ships the sync binary. Web/backend don't run there. Goal #3 is structurally not done.
6. **In-process scheduler shares process with web server** (`main.rs:177-186`). One panic in sync logic crashes the API too. Risky for the writeback worker.

### A4. Does the spike change the calculus?

**Yes, decisively, but in a specific way.** The spike proved writeback is *mechanically possible*. It did NOT prove writeback is operationally safe. The next 2 weeks of writeback work isn't about Rust vs TypeScript — it's about getting the operational discipline right (transactions, idempotency, schema-fingerprint guards, monitoring of the .NET app's reaction to our writes). That work is roughly the same in either language.

**What the spike *does* change**: writeback is now confidently estimated, so the rest of the rewrite question can be evaluated on its own merits.

---

## B. Stack choice — three options

### Option 1 — Stay with Rust + Axum (current)

**Keep:** all 9,780 LOC of routes, both workers, the `sqlx`/`tiberius` setup, the deploy pipeline.
**Add:** writeback worker as a third Rust binary (or in-process scheduler job).

| Axis | Verdict |
|---|---|
| Dev velocity | **Bad and getting worse** — CI is the felt pain |
| Performance | **Excellent** |
| Deploy simplicity | **Excellent** — one static binary |
| MSSQL tooling | **Good** for HF Hotel; **bad** for HF Ville (tsql shell-out) |
| Postgres tooling | **Excellent** (sqlx) |
| Type sharing | **None** — manually maintained `/types/*.ts`, drifts |
| AI codegen | **Mediocre** — Rust + sqlx macros + axum extractors are a thin slice of training data |
| Migration cost | **~2 weeks** for writeback + adapter refactor |

### Option 2 — Full rewrite to Elysia + Bun

**Keep:** Postgres schema, React components, Next.js pages, Docker compose topology.
**Replace:** all 22 route files (Axum → Elysia), `scheduler/sync.rs` (tokio scheduler → Bun cron), `bin/ville_sync.rs` (tsql → `tedious`/`mssql` npm package), the writeback worker.
**Wrap:** nothing — Rust binary goes away entirely.

| Axis | Verdict |
|---|---|
| Dev velocity | **Excellent** — Bun's `--hot` is sub-second; Eden gives end-to-end type inference |
| Performance | **Good enough** — SQL bound is the actual bottleneck regardless |
| Deploy simplicity | **Good** — `bun build --compile` produces a single binary |
| MSSQL tooling | **Mature** — `mssql`/`tedious` packages support TDS 7.0–7.4, work for HF Hotel AND HF Ville (kills the tsql hack) |
| Postgres tooling | **Worse than sqlx** — Drizzle/Kysely give type inference but no compile-time SQL verification |
| Type sharing | **Excellent** — Eden Treaty pulls types from backend into client |
| AI codegen | **Excellent** — TS + Elysia is well-represented in training data |
| Migration cost | **2-3 months calendar time** for one engineer |

**Lose:** sqlx compile-time SQL checking (single best feature of current stack).
**Gain:** type-safe end-to-end API contracts, fast CI, single language, much better AI ergonomics, kills tsql subprocess.

### Option 3 — Hybrid (Rust workers + Elysia API) ⭐

**Keep:** `scheduler/sync.rs`, `bin/ville_sync.rs`, the Postgres schema, the Next.js frontend, all CRUD against PG.
**Add:** new Rust binary for the writeback worker (high-stakes write-to-MSSQL path). Reuse `tiberius` and the `TABLOCKX, HOLDLOCK` pattern.
**Replace:** the 22 route files with Elysia + Drizzle/Kysely against the same PG.
**Wrap:** nothing — API and worker surfaces are independent.

| Axis | Verdict |
|---|---|
| Dev velocity | **Good for routes, unchanged for workers** — workers are stable, don't change weekly |
| Performance | **Excellent on both sides** |
| Deploy simplicity | **Slightly worse** — three images instead of two |
| MSSQL tooling | **Best of both** — Rust workers keep tiberius for HF Hotel writeback; Elysia API never touches MSSQL directly |
| Postgres tooling | **Mostly TypeScript** for API; Rust uses sqlx for sync only |
| Type sharing | **Excellent for API** |
| AI codegen | **Good** for the bulk of code; workers small enough to maintain by hand |
| Migration cost | **3-4 weeks** — most is mechanical route porting |

---

## C. Frontend collapse

Independent of backend choice — collapse the dual `app/(legacy)/*` + `app/new/*` tree NOW. **Recommendation: kill `(legacy)` entirely** and consolidate. The `(legacy)` pages exist because the original architecture treated MSSQL as authoritative; that's no longer true after writeback ships.

**Sequence:**
1. Audit `app/(legacy)/*` for features missing from `app/new/*` (e.g. `card-reader/page.tsx`)
2. Port the gaps
3. Delete `app/(legacy)/`. Move `app/new/*` → `app/*`. Drop `Navbar.tsx`, keep `NewNavbar.tsx` (rename).

**Should we switch frontend frameworks if going Elysia?** **No.** Next.js 16 + React 19 is fine; Eden Treaty consumes Elysia routes from any TS client. Don't introduce a third churn (TanStack Start, Bun's serve) on top of a stack rewrite.

---

## D. Multi-site strategy (HF Ville)

Three deployment shapes:

1. **Full stack at HF Ville** (own evergreen-equivalent) — Local PG, local backend, local frontend. Resilient to WireGuard outages. Cost: a second machine to babysit.
2. **Central deployment, two MSSQL connections** — One evergreen reads/writes both legacy DBs via Tailscale. Cheaper; brittle if WireGuard flaps.
3. **Status quo (sync-only)** — HF Ville has only `ville_sync` pushing data to evergreen. Reads work centrally; writes don't reach Ville's .NET app.

**Recommendation:** **shape 1 (full stack at HF Ville)** as long-term answer, **shape 3 (sync-only) as interim** until writeback is mature on HF Hotel. Once writeback is proven on HF Hotel for ~1 month, deploy same image to a Ville-local machine pointing at Ville's MSSQL.

The architecture must produce a **deployment artifact that is location-agnostic**: same image, different `.env`. Goal #2 (standalone): a third site with NO legacy app would deploy with `LEGACY_BACKEND=disabled`.

---

## E. Recommendation

**Pick Option 3 (Hybrid).**

**Reasoning:**

1. **Pain points are 80% in the API/route layer** (slow CI, hard to type-share, AI struggles with Rust). That layer is mostly mechanical PG CRUD — exactly TypeScript/Elysia's sweet spot.
2. **Strengths of current stack are concentrated in workers** — tiberius for MSSQL, sqlx for PG mirror, the proven `TABLOCKX` writeback pattern. Throwing those away to rewrite in TS adds risk against a database we don't own without solving any voiced complaint.
3. **Migration is incremental and reversible** — ship Elysia as a *new* API on a different port, point selected frontend pages at it, keep the Rust API live until everything migrates. If Elysia surprises us, revert by changing API base URLs.
4. The writeback worker — highest-stakes new code — gets to use the same tiberius infrastructure the spike already proved. **Don't change the tooling for the most dangerous part of the project.**
5. Frontend collapse and multi-site deployment are orthogonal to stack choice; can proceed in parallel.

**Prerequisites / decisions still needed:**

- **Confirm "API + worker split is acceptable."** Two languages, two binaries, two mental models. The user is one engineer — they will hit both daily.
- **Pick a Postgres query layer for Elysia.** Drizzle (best DX, schema-first) vs Kysely (closer-to-SQL, type-safe builder) vs raw `postgres`. Recommend **Drizzle**.
- **Pick an Elysia ↔ Worker RPC mechanism.** HTTP, Postgres `LISTEN/NOTIFY` queue, or SQS-style. **Recommend Postgres queue** — no new infra, durable, retries trivial.
- **Confirm "delete `app/(legacy)` is a hard requirement."** It is, but should be explicit.
- **Confirm HF Ville deployment shape.** Recommendation is "shape 3 now → shape 1 later."

---

## F. Migration roadmap (Hybrid path)

| Phase | Time | Goal | Independently shippable? |
|---|---|---|---|
| 0 | ~3 days | Frontend collapse — delete `(legacy)`, single tree | ✅ |
| 1 | ~2 weeks | Writeback worker in Rust binary; Postgres queue | ✅ — Goal #1 done |
| 2 | ~1 week | Elysia API skeleton on port 3004; 5 most-used GETs | ✅ |
| 3 | ~2 weeks | Migrate read routes page-by-page (high-traffic first) | ✅ each page |
| 4 | ~1 week | Migrate write routes; mutations enqueue via PG queue | ✅ |
| 5 | ~3 days | Decommission Rust API; 3 containers (web/api/workers) | ✅ |
| 6 | ~1 week | Multi-site full deploy at HF Ville (after Phase 1 proven 1 month) | ✅ — Goals #2 + #3 done |

**Total: ~6-7 weeks calendar time** for one engineer, all reversible, all incrementally shippable. No big-bang rewrite.

---

## Critical files referenced

- `/Users/nut/new-hotel/hotel-backend/src/main.rs` — current bootstrap; split into worker-only after Phase 5
- `/Users/nut/new-hotel/hotel-backend/src/scheduler/sync.rs` — pattern the writeback worker mirrors
- `/Users/nut/new-hotel/docs/legacy-spike/findings.md` — writeback recipe source of truth
- `/Users/nut/new-hotel/docker-compose.yml` — gains a third service in Phase 5
- `/Users/nut/new-hotel/.github/workflows/docker-build.yml` — gains an `api` build step in Phase 2
