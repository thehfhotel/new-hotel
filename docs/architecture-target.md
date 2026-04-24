# Target Architecture (Hybrid: Rust Workers + Elysia API)

Visualization of the recommended end-state from `architecture-review.md`.
Reads top-to-bottom: browser → API → data layer → external systems.

---

## 1. Single-site topology (HF Hotel)

```
                                    BROWSER
                                       │
                                       │ HTTPS
                                       ▼
        ┌────────────────────────────────────────────────────────────┐
        │  evergreen.thehfhotel.org  (Linux, docker compose)         │
        │                                                            │
        │  ┌──────────────────┐                                      │
        │  │  web             │  Next.js 16 + React 19               │
        │  │  port 3003       │  – single /app/ tree (no legacy)     │
        │  │  (Node)          │  – Eden Treaty client → Elysia       │
        │  └────────┬─────────┘                                      │
        │           │                                                │
        │           │ /api/*  (Eden Treaty, type-shared)             │
        │           ▼                                                │
        │  ┌──────────────────┐                                      │
        │  │  api             │  Elysia + Bun + Drizzle              │
        │  │  port 3004       │  – read-heavy: SELECT from PG        │
        │  │  (Bun)           │  – write-heavy: INSERT into PG       │
        │  │                  │    + enqueue writeback_jobs row      │
        │  └────────┬─────────┘                                      │
        │           │                                                │
        │           │ SQL (sqlx in Rust workers, postgres.js in TS)  │
        │           ▼                                                │
        │  ┌─────────────────────────────────────────────────────┐   │
        │  │  newdb       PostgreSQL 17                          │   │
        │  │  port 5439   – source of truth (ht_*)               │   │
        │  │              – mirror tables (ht_*_legacy)          │   │
        │  │              – writeback_jobs queue + LISTEN/NOTIFY │   │
        │  └────┬─────────────────────────────────┬──────────────┘   │
        │       │ NOTIFY                          │                  │
        │       │                                 │                  │
        │       │                                 │ SELECT (mirror)  │
        │       ▼                                 │ INSERT (sync)    │
        │  ┌─────────────────────┐                │                  │
        │  │  workers            │  Rust binary   │                  │
        │  │  (no port)          │  – sync.rs    ◀┘                  │
        │  │                     │    (every 5min: MSSQL→PG mirror)  │
        │  │                     │                                   │
        │  │                     │  – writeback.rs                   │
        │  │                     │    (LISTEN/NOTIFY → tiberius)     │
        │  └────────┬────────────┘                                   │
        │           │                                                │
        └───────────┼────────────────────────────────────────────────┘
                    │ tiberius (TDS 7.4)
                    │ Race-safe via TABLOCKX, HOLDLOCK
                    ▼
                ┌──────────────────────┐
                │  Legacy MSSQL        │       ┌──────────────────┐
                │  192.168.100.222     │ ◀───▶│  3rd-party .NET  │
                │  SQL Server 2022 Exp │       │  Windows app     │
                │  (db database)       │       │  (FRONT2 box)    │
                └──────────────────────┘       └──────────────────┘
                          ▲ writeback (TABLOCKX-serialized)
                          │ reads (mirror sync)
```

**3 containers, 4 processes total.** `web` and `api` are stateless and
restart-safe. `workers` holds the in-process sync scheduler + the writeback
LISTEN loop in one Rust binary (or split into two binaries if desired).

---

## 2. Data flow paths

### 2a. Read path (the common case)

```
   Browser
     │ GET /api/bookings?date=2026-04-25
     ▼
   web (Next.js)  ──proxies──▶  api (Elysia/Bun)
                                  │
                                  │ Drizzle: SELECT * FROM ht_bookings WHERE ...
                                  ▼
                                Postgres (newdb)
                                  │
                                  │ rows
                                  ▼
                                api  ──Eden response (typed)──▶  web  ──▶  Browser
```

**~5-15ms total.** Never touches MSSQL on the read path. The legacy
DB's data lives in `ht_*_legacy` mirror tables (refreshed every 5 min
by the sync worker).

### 2b. Write path (with writeback)

```
   Browser
     │ POST /api/bookings  { customer_id, room, dates }
     ▼
   api (Elysia)
     │
     │ ┌───────────────────────────────────────────┐
     │ │ BEGIN TRAN                                │
     │ │   INSERT INTO ht_bookings (..., id=…)     │  ← own data
     │ │   INSERT INTO writeback_jobs (            │  ← queue row
     │ │     intent='create_booking',              │
     │ │     payload=jsonb{...},                   │
     │ │     uuid=our_idempotency_key,             │
     │ │     status='pending')                     │
     │ │   NOTIFY writeback_channel, '<job_id>'    │  ← wake worker
     │ │ COMMIT                                    │
     │ └───────────────────────────────────────────┘
     │
     ▼ 200 OK (returned immediately, before legacy write completes)
   Browser

                  ┌─ asynchronous, in parallel ─────────────────┐
                  │                                             │
                  ▼                                             │
   workers (Rust, LISTEN'ing on writeback_channel)              │
     │                                                          │
     │ pop job from writeback_jobs WHERE status='pending'       │
     │ parse intent → call create_booking() in writeback.rs     │
     │                                                          │
     │ ┌───────────────────────────────────────────────────────┐│
     │ │ BEGIN TRAN  (against legacy MSSQL via tiberius)        ││
     │ │   SELECT @cid = MAX(id)+1 FROM HT_Customers            ││
     │ │     WITH (TABLOCKX, HOLDLOCK)                          ││
     │ │   INSERT INTO HT_Customers (id, Cust_no, ...)          ││
     │ │   SELECT @bid = ... FROM HT_Book_H WITH (TABLOCKX,...) ││
     │ │   INSERT INTO HT_Book_H (Book_ID, ...)                 ││
     │ │   INSERT INTO HT_Book_Ds (...)                         ││
     │ │   INSERT INTO HT_Book_Date (... × n nights)            ││
     │ │ COMMIT  (~5-10ms hold time)                            ││
     │ └───────────────────────────────────────────────────────┘│
     │                                                          │
     │ UPDATE writeback_jobs SET status='done', legacy_ids={..} │
     │   WHERE id=<job_id>                                      │
     │                                                          │
     ▼                                                          │
   Postgres (newdb)  ◀── job marked done, legacy IDs stored ────┘
```

**Key property:** the API responds to the user **immediately** after the
PG insert + queue enqueue. The user doesn't wait for the legacy MSSQL
round-trip. If the writeback fails (network hiccup, schema drift), the
job stays `pending` and retries. Our app never blocks on the legacy DB.

### 2c. Mirror sync path (read-direction, every 5 min)

```
   workers (Rust scheduler, cron-like)
     │
     │ tiberius: SELECT * FROM HT_Customers WHERE ... (incremental)
     ▼
   Legacy MSSQL  ──rows──▶  workers
                              │
                              │ sqlx: UPSERT INTO ht_customers_legacy ...
                              ▼
                            Postgres (newdb)
                              ▲
                              │ used by api's read-path queries
                              │ (joins ht_*_legacy with ht_*)
```

This is the existing `scheduler/sync.rs` flow — unchanged. Keeps reading
the legacy DB efficient because the API never has to wait for MSSQL.

---

## 3. Multi-site deployment (HF Hotel + HF Ville)

End state — Phase 6, after writeback proven on HF Hotel:

```
┌─────────────────────────────── HF HOTEL site ────────────────────────────────┐
│                                                                              │
│   ┌─────────────────┐                ┌─────────────────────────┐             │
│   │ User browser    │                │ 3rd-party .NET app      │             │
│   │ (mobile/laptop) │                │ on FRONT2 desktop PC    │             │
│   └────────┬────────┘                └──────────┬──────────────┘             │
│            │ HTTPS                              │ T-SQL                       │
│            ▼                                    ▼                             │
│   ┌────────────────────────────┐      ┌────────────────────────┐             │
│   │ evergreen.thehfhotel.org   │      │ Legacy MSSQL           │             │
│   │ ┌──────┬──────┬─────────┐  │      │ 192.168.100.222        │             │
│   │ │ web  │ api  │ workers │  │ ◀───▶│ db database            │             │
│   │ │ N16  │ Elys │ Rust    │  │      │ (writeback target)     │             │
│   │ └──┬───┴──┬───┴────┬────┘  │      └────────────────────────┘             │
│   │    │      │        │       │                                             │
│   │    └──────┴────────┴───┐   │                                             │
│   │                        ▼   │                                             │
│   │              ┌─────────────┐│                                             │
│   │              │ newdb (PG)  ││                                             │
│   │              └─────────────┘│                                             │
│   └────────────────┬────────────┘                                             │
│                    │ central PG also stores Ville's mirror data               │
└────────────────────┼──────────────────────────────────────────────────────────┘
                     │
                     │  Tailscale tailnet (100.x.x.x)
                     │  + WireGuard mesh (10.10.10.0/24)
                     │
┌────────────────────┼──────────── HF VILLE site ──────────────────────────────┐
│                    │                                                          │
│   ┌────────────────┴───────────┐      ┌────────────────────────┐             │
│   │ ville-evergreen (new box)  │      │ Legacy MSSQL Ville     │             │
│   │ ┌──────┬──────┬─────────┐  │      │ 192.168.x.x  (Ville)   │             │
│   │ │ web  │ api  │ workers │  │ ◀───▶│ ville_db database      │             │
│   │ │ N16  │ Elys │ Rust    │  │      │ (writeback target)     │             │
│   │ └──┬───┴──┬───┴────┬────┘  │      └────────────────────────┘             │
│   │    │      │        │       │                                             │
│   │    └──────┴────────┴───┐   │                                             │
│   │                        ▼   │                                             │
│   │              ┌─────────────┐│                                             │
│   │              │ newdb (PG)  ││  same image, different .env                 │
│   │              └─────────────┘│  branch_id='hfville' baked in               │
│   └────────────────┬────────────┘                                             │
│                    │                                                          │
│                    │ ville_sync push to evergreen's central PG                │
│                    └─────▶  (cross-site reporting / unified dashboards)       │
│                                                                               │
│   ┌─────────────────┐                ┌─────────────────────────┐             │
│   │ Ville staff     │                │ Ville's 3rd-party       │             │
│   │ browsers        │                │ .NET app                │             │
│   └────────┬────────┘                └──────────┬──────────────┘             │
│            │ HTTPS                              │ T-SQL                       │
│            └─────────────▶ ville-evergreen      └─▶ Legacy MSSQL Ville       │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Properties:**
- Each site is **operationally autonomous** — works even if WireGuard is down
- Same image, different `.env` (branch_id, DB hostnames, secrets)
- `ville_sync` (existing) keeps central PG informed for cross-site reporting
- Goal #2 (standalone): a third site with NO legacy app would deploy with `LEGACY_BACKEND=disabled` env — workers' MSSQL connection skipped, sync disabled, writeback disabled. PG-only mode.
- Goal #3 (multi-site): two independent stacks federated via the existing ville_sync push

---

## 4. Repository layout (final)

```
new-hotel/
├── app/                       Next.js 16 app router (single tree, no legacy)
│   ├── bookings/
│   ├── checkins/
│   ├── rooms/
│   ├── customers/
│   ├── inventory/
│   └── ...
├── components/                React components (unchanged)
├── lib/
│   └── api-client.ts          Eden Treaty client (auto-imports api types)
│
├── hotel-api/                 NEW — Elysia + Bun + Drizzle
│   ├── src/
│   │   ├── routes/
│   │   │   ├── bookings.ts
│   │   │   ├── customers.ts
│   │   │   └── ...
│   │   ├── db/
│   │   │   ├── schema.ts      Drizzle schema (mirrors PG)
│   │   │   └── client.ts
│   │   ├── writeback/
│   │   │   └── enqueue.ts     INSERT INTO writeback_jobs + NOTIFY
│   │   └── index.ts           Elysia entry, exports app for Eden
│   ├── package.json           bun runtime
│   └── Dockerfile             FROM oven/bun:1
│
├── hotel-backend/             EXISTING — Rust workers (slimmed down)
│   ├── src/
│   │   ├── bin/
│   │   │   ├── workers.rs     scheduler + writeback LISTEN loop
│   │   │   ├── ville_sync.rs  (existing, unchanged)
│   │   │   └── migrate_legacy.rs (existing)
│   │   ├── scheduler/
│   │   │   └── sync.rs        (existing, unchanged)
│   │   ├── writeback/
│   │   │   ├── mod.rs         job dispatcher
│   │   │   ├── walkin.rs      INSERT recipe from findings.md §3a
│   │   │   ├── booking.rs     §3b
│   │   │   ├── checkin.rs     §3d
│   │   │   ├── checkout.rs    §3e (without destructive Phase 1)
│   │   │   ├── extend.rs      §3f
│   │   │   ├── payment.rs     §3h
│   │   │   ├── cancel_booking.rs   §3g-bis
│   │   │   ├── cancel_checkin.rs   §3i
│   │   │   ├── housekeeping.rs     §3j
│   │   │   └── allocate.rs    TABLOCKX MAX+1 helpers
│   │   └── lib.rs             (no main.rs/Axum — API moved to Elysia)
│   └── Dockerfile             FROM rust:1.89-bookworm AS chef (existing)
│
├── newdb/init-db/             PG init scripts (existing)
├── migrations/pg/             PG migrations (existing)
│   └── NNN_writeback_jobs.sql NEW — queue table + indexes
│
├── docs/
│   ├── legacy-spike/          (existing — source of truth for writeback recipes)
│   ├── architecture-review.md (just written — decision rationale)
│   └── architecture-target.md (this file)
│
├── docker-compose.yml         3 services: web, api, workers, newdb
└── .github/workflows/
    └── docker-build.yml       Build + deploy 3 images instead of 2
```

---

## 5. Per-process responsibilities (one-line each)

| Process | Owns | Talks to | Talks back via |
|---|---|---|---|
| **web** (Next.js) | UI rendering, routing, SSR | api | Eden HTTP client |
| **api** (Elysia/Bun) | request validation, auth, business logic, PG reads, mutations + writeback enqueue | newdb (PG) | Drizzle / postgres.js |
| **workers** (Rust binary) | scheduled sync, writeback dispatcher, LISTEN loop | newdb (PG) for jobs/sync, legacy MSSQL for writeback | sqlx (PG), tiberius (MSSQL) |
| **newdb** (Postgres 17) | source of truth, mirror tables, queue | itself | n/a |
| **legacy MSSQL** | external system we read+write to | n/a | shared with .NET app |

---

## 6. Lifecycle of one operation (worked example: cancel a booking)

```
[T+0ms] User clicks "Cancel" on a booking in Next.js UI
[T+5ms] web → api: DELETE /api/bookings/R014812
[T+10ms] api validates user has permission, opens PG transaction:
           UPDATE ht_bookings SET status='cancelled' WHERE id='R014812'
           INSERT INTO writeback_jobs (intent='cancel_booking',
             payload='{"book_id":"R014812"}',
             uuid=<idempotency_key>, status='pending')
           NOTIFY writeback_channel, '<job_id>'
         COMMIT
[T+15ms] api responds 200 OK to web
[T+20ms] web shows "cancelled ✓" toast to user

         ─── meanwhile, async ───

[T+15ms] workers' LISTEN handler fires
[T+16ms] worker SELECTs the job, dispatches to cancel_booking.rs
[T+17ms] worker BEGIN TRAN against legacy MSSQL via tiberius:
           UPDATE HT_Rooms SET room_book_ds='', ... WHERE room_book IN (...)
           UPDATE HT_Book_H SET Book_Status='ยกเลิก' WHERE Book_ID='R014812'
           UPDATE HT_Book_Ds SET Book_status=3 WHERE Book_No='R014812'
           DELETE FROM HT_Book_Date WHERE Book_no='R014812'
         COMMIT
[T+45ms] worker UPDATEs writeback_jobs SET status='done',
           completed_at=NOW() WHERE id=<job_id>
[T+46ms] receptionist's .NET app sees the cancellation on next refresh
```

**User-visible latency: ~20ms.** Legacy MSSQL latency: hidden behind the queue.

---

## 7. Failure modes (what happens when something breaks)

| Failure | Effect on user | Recovery |
|---|---|---|
| Legacy MSSQL down | None on UI; writeback jobs queue up as `pending` | Worker retries on each NOTIFY (debounced) until MSSQL responds |
| Network to HF Ville flaps | Ville site keeps working locally; central PG sync delayed | `ville_sync` retries; cross-site reports show stale data temporarily |
| api crashes | UI shows errors on new requests; PG state preserved | Container restart, no data loss |
| workers crashes | API and UI keep working; pending jobs accumulate | Container restart picks up the queue |
| newdb (Postgres) down | Whole system down (this is the SPOF) | Restore from `backups/` (existing automated backup) |
| .NET app race-collides with our writeback | Theirs OR ours wins; loser blocks ~10ms then proceeds with new MAX | Built into TABLOCKX pattern; verified in spike |
| Schema drift in legacy DB | Worker fails-fast on startup fingerprint check | Engineer reads new schema, updates writeback recipe, redeploys |

---

## 8. Migration timeline visualized

```
Today                                                            ~7 weeks
  │                                                                  │
  ▼                                                                  ▼
  ├── Phase 0 ──┤
  │  3 days
  │  Frontend
  │  collapse
  │
  └─────┬───────┤
        │
        ├── Phase 1 ───────────────────┤
        │  2 weeks
        │  Writeback worker (Rust)
        │  Goal #1 ✓
        │
        └─────┬─────────────────────────┤
              │
              ├── Phase 2 ──┤
              │  1 week
              │  Elysia API skeleton
              │  on port 3004
              │
              └─────┬───────┤
                    │
                    ├── Phase 3 ──────────────┤
                    │  2 weeks
                    │  Migrate read routes
                    │  page-by-page
                    │
                    └─────┬───────────────────┤
                          │
                          ├── Phase 4 ──┤
                          │  1 week
                          │  Migrate
                          │  write routes
                          │
                          └─────┬───────┤
                                │
                                ├── Phase 5 ──┤
                                │  3 days
                                │  Decommiss.
                                │  Rust API
                                │
                                └────────┬─────┤
                                         │
                                         ├── Phase 6 ──┤
                                         │  1 week
                                         │  HF Ville
                                         │  full deploy
                                         │  Goals #2 + #3 ✓
                                         │
                                         └─────────────┘

  ◀───── reversible at any phase boundary ─────▶
```

Each phase is **independently deployable** and **reversible** until the
next phase starts. No big-bang switch-over.
