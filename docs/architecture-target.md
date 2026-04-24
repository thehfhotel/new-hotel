# Target Architecture — Decommission-Ready (Stay-Current Stack)

**Stack: unchanged.** Next.js 16 frontend, Rust + Axum backend, Postgres + Legacy MSSQL.

**Core principle:** PostgreSQL is the **single source of truth** from day one.
The legacy MSSQL is treated as an **external system we currently mirror to/from** for backward-compatibility with the 3rd-party Windows app. When the 3rd-party app is decommissioned, we turn off the sync + writeback workers; everything else keeps working unchanged.

The architecture must support **three operational states** without code changes — only `.env` toggles:

```
State A (today):     Legacy app is primary UI       — sync + writeback both ON
State B (transition): Both apps coexist               — sync + writeback both ON
State C (decommissioned): Only our app                — sync + writeback both OFF
```

---

## 1. Layered architecture

```
                    BROWSER
                       │
                       │ HTTPS
                       ▼
        ┌──────────────────────────────┐
        │  Frontend: Next.js 16        │
        │  – single /app/* tree        │
        │  – fetch /api/*              │
        └──────────────┬───────────────┘
                       │
                       ▼
        ╔══════════════════════════════════════╗
        ║                                      ║
        ║   ┌──────────────────────────────┐   ║
        ║   │  HTTP layer (Axum routes)    │   ║   thin
        ║   │  – validation, auth, shape   │   ║   ~50 LOC/file
        ║   └──────────────┬───────────────┘   ║
        ║                  │                   ║
        ║                  ▼                   ║
        ║   ┌──────────────────────────────┐   ║
        ║   │  SERVICE layer  ★NEW★        │   ║   business
        ║   │  – domain operations         │   ║   logic
        ║   │  – state machines            │   ║   lives here
        ║   │  – cross-aggregate workflows │   ║
        ║   │  – emits outbox jobs         │   ║
        ║   └──────────────┬───────────────┘   ║
        ║                  │                   ║
        ║                  ▼                   ║
        ║   ┌──────────────────────────────┐   ║
        ║   │  REPOSITORY layer  ★NEW★     │   ║   data access
        ║   │  – trait per aggregate       │   ║   ALWAYS PG
        ║   │  – CustomerRepo, BookingRepo │   ║   never MSSQL
        ║   │  – pure SQL against ht_*     │   ║
        ║   └──────────────┬───────────────┘   ║
        ║                  │                   ║
        ║   ┌──────────────────────────────┐   ║
        ║   │  DOMAIN layer  ★NEW★         │   ║   pure types,
        ║   │  – Customer, Booking, etc.   │   ║   no I/O,
        ║   │  – business invariants       │   ║   testable
        ║   │  – state enums               │   ║   in isolation
        ║   └──────────────────────────────┘   ║
        ║                                      ║
        ║   hotel-backend/src/                 ║
        ╚════════════════╤═════════════════════╝
                         │
                         ▼
        ┌──────────────────────────────────────┐
        │  Postgres (newdb) — SOURCE OF TRUTH  │
        │                                      │
        │  ┌─────────────────────────────┐     │
        │  │ canonical tables (ht_*)     │     │
        │  │ – customers, bookings, etc. │     │
        │  │ – owns its own IDs (UUIDs)  │     │
        │  │ – stores legacy_id refs     │     │
        │  └─────────────────────────────┘     │
        │                                      │
        │  ┌─────────────────────────────┐     │
        │  │ writeback_jobs (outbox)     │     │
        │  │ – id, intent, payload       │     │
        │  │ – idempotency_key, status   │     │
        │  │ – LISTEN/NOTIFY channel     │     │
        │  └─────────────────────────────┘     │
        │                                      │
        │  ┌─────────────────────────────┐     │
        │  │ legacy_mirror schema        │     │
        │  │ – read-only snapshot of MSS │     │
        │  │ – populated by sync worker  │     │
        │  │ – used to detect drift /    │     │
        │  │   backfill new legacy data  │     │
        │  └─────────────────────────────┘     │
        └──────────┬──────────┬────────────────┘
                   │          │
                   │ NOTIFY   │ SELECT (drift detection)
                   ▼          ▼
        ┌──────────────────────────────────────┐
        │  ★Adapter workers★ (3 Rust binaries) │
        │                                      │
        │  bin/writeback.rs                    │
        │  – LISTENs writeback_channel         │
        │  – pops jobs, replays to MSSQL       │
        │  – TABLOCKX, HOLDLOCK pattern        │
        │  – TOGGLE: WRITEBACK_ENABLED         │
        │                                      │
        │  bin/sync.rs (was scheduler/sync.rs) │
        │  – pulls MSSQL → legacy_mirror       │
        │  – detects new rows the .NET app     │
        │    created (other clerks)            │
        │  – upserts into canonical ht_* if    │
        │    no matching row exists            │
        │  – TOGGLE: LEGACY_SYNC_ENABLED       │
        │                                      │
        │  bin/ville_sync.rs (existing)        │
        │  – cross-site to HF Ville            │
        │  – TOGGLE: VILLE_SYNC_ENABLED        │
        └──────────────┬───────────────────────┘
                       │ tiberius (TDS)
                       ▼
              ┌─────────────────────┐
              │ Legacy MSSQL        │ ◀──▶ 3rd-party .NET app
              │ (external system)   │      (will be retired)
              └─────────────────────┘
```

The double-walled box (`╔ ... ╗`) is the **decommission boundary**. Everything inside survives the legacy app's removal. Everything outside (the workers + the legacy MSSQL) gets turned off without touching application code.

---

## 2. Why the layers matter (separation of concerns)

| Layer | Owns | Knows about |
|---|---|---|
| **HTTP routes** | request/response shape, status codes, JSON serialization | service layer's API |
| **Service** | business workflows, transactions, outbox emission | domain types, repository traits |
| **Repository** | SQL against PG only | domain types, sqlx |
| **Domain** | pure types, business invariants, state machines | nothing (no imports) |
| **Outbox** | durable queue of pending writeback jobs | nothing (just data) |
| **Adapter workers** | external system communication | domain types, repository, tiberius |

**Critical rule:** Routes never call repositories directly. Routes never know about MSSQL. Repositories never call MSSQL. Only the **adapter workers** touch external systems. This is what makes decommission a single config flip.

### Example: "create a booking"

**Today's code (mixed concerns in one file):**
```rust
// hotel-backend/src/routes/new_bookings.rs (current)
async fn create_booking(State(state): ..., Json(req): ...) -> ... {
    // validation inline
    // SQL inline
    // (would also need MSSQL writeback inline if we added it)
}
```

**New layered code:**
```rust
// routes/booking.rs  (thin HTTP adapter)
async fn create(state: AppState, Json(req): Json<CreateBookingRequest>) -> impl IntoResponse {
    let cmd = req.into_command()?;            // request → domain command
    let booking = state.bookings.create(cmd).await?;  // service handles it
    Json(booking.into_response())             // domain → response DTO
}

// service/booking.rs  (business logic, transaction control)
impl BookingService {
    async fn create(&self, cmd: CreateBookingCommand) -> Result<Booking, ServiceError> {
        let mut tx = self.db.begin().await?;
        let booking = self.repo.insert(&mut tx, cmd.into_record()).await?;
        self.outbox.enqueue(&mut tx, WritebackIntent::CreateBooking {
            booking_id: booking.id,
            payload: cmd.into_writeback_payload(),
        }).await?;
        tx.commit().await?;
        // PG is now consistent. Worker will push to MSSQL async.
        Ok(booking)
    }
}

// repository/booking.rs  (PG-only)
#[async_trait]
impl BookingRepository for PgBookingRepository {
    async fn insert(&self, tx: &mut PgTx, rec: NewBookingRecord) -> Result<Booking, sqlx::Error> {
        sqlx::query_as!(Booking, "INSERT INTO ht_bookings (...) RETURNING ...", ...)
            .fetch_one(&mut **tx).await
    }
}

// domain/booking.rs  (pure types)
pub struct Booking {
    pub id: Uuid,
    pub legacy_book_id: Option<String>,    // R\d{6} — set by writeback worker
    pub customer_id: Uuid,
    pub state: BookingState,
    pub stay: DateRange,
    // ...
}

pub enum BookingState { Pending, Active, CheckedIn, Completed, Cancelled }

// outbox/writeback_intent.rs  (durable command)
pub enum WritebackIntent {
    CreateBooking { booking_id: Uuid, payload: CreateBookingPayload },
    ModifyBooking { booking_id: Uuid, changes: BookingChanges },
    CancelBooking { booking_id: Uuid },
    CreateCheckIn { check_in_id: Uuid, payload: CreateCheckInPayload },
    // ... one variant per writeback recipe
}
```

**When legacy is decommissioned:** delete the worker. Set `WRITEBACK_ENABLED=false`. Service code unchanged. Outbox table can be dropped. Done.

---

## 3. The three operational states (visualized)

### State A — TODAY (legacy app is primary)

```
        Receptionist               Our future user
              │                          │
              ▼                          ▼
    ┌─────────────────┐          ┌──────────────┐
    │ 3rd-party .NET  │          │  Our app     │
    │ Windows app     │          │  (Next.js)   │
    └────────┬────────┘          └──────┬───────┘
             │                          │
             │ direct                   │ /api/*
             │ T-SQL                    ▼
             ▼                  ┌────────────────┐
    ┌────────────────┐          │  Axum + Service│
    │  Legacy MSSQL  │          │  + Repository  │
    └───┬───────▲────┘          └───────┬────────┘
        │       │                       │
   sync ▼       │ writeback             ▼
        │       │              ┌────────────────┐
        │       │              │  Postgres      │ ←── source of truth
        │       │              │  (newdb)       │     for OUR data
        │       │              └───────┬────────┘
        │       │                      │
        │       │                      │ NOTIFY
        │       └──────────────────────┤
        │                              │
        ▼                              ▼
    ┌──────────────────┐    ┌────────────────────┐
    │ legacy_mirror    │    │ writeback worker   │
    │ schema in PG     │    │ – LISTENs queue    │
    │ – sync worker    │    │ – TABLOCKX writes  │
    │   pulls every 5m │    │   to legacy MSSQL  │
    └──────────────────┘    └────────────────────┘

    Toggles:  LEGACY_SYNC_ENABLED=true, WRITEBACK_ENABLED=true
    Reads:    Our app reads from PG (canonical + mirror joined)
    Writes:   Our app writes to PG, worker async-pushes to MSSQL
```

### State B — TRANSITION (both apps in use)

```
    Same as State A. No code changes. Just usage shifts.
    Some receptionists use 3rd-party app, some use ours.
    Both apps see each other's data via the existing sync paths.

    Sync worker importance: HIGH (other clerks adding data legacy-side)
    Writeback worker importance: HIGH (.NET app must see our writes)
```

### State C — DECOMMISSIONED (only our app)

```
        All users
            │
            ▼
    ┌──────────────┐
    │  Our app     │
    │  (Next.js)   │
    └──────┬───────┘
           │
           ▼
    ┌────────────────┐
    │  Axum + Service│
    │  + Repository  │
    └───────┬────────┘
            │
            ▼
    ┌────────────────┐
    │  Postgres      │ ← only data store
    │  (newdb)       │
    └────────────────┘

    Toggles:  LEGACY_SYNC_ENABLED=false, WRITEBACK_ENABLED=false
    Workers:  not deployed (or deployed but idle)
    Application code: unchanged from State A
    Legacy MSSQL: powered off, decommissioned

    Optional cleanup later:
    - Drop legacy_mirror schema from PG
    - Drop writeback_jobs table
    - Delete writeback.rs + sync.rs binaries from build
```

---

## 4. Data layer design (PG schema for source-of-truth)

The PG schema must hold **everything** the legacy MSSQL holds, plus our extensions. Today many `ht_*` tables already exist — they need an audit to ensure full coverage. Key principles:

### 4a. Own IDs, reference legacy IDs

Every row gets a UUID primary key. The legacy ID (if it has one) is stored alongside in a nullable `legacy_*_id` column.

```sql
CREATE TABLE ht_bookings (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    legacy_book_id  VARCHAR(50) UNIQUE,         -- R\d{6}, NULL until writeback succeeds
    customer_id     UUID        NOT NULL REFERENCES ht_customers(id),
    state           TEXT        NOT NULL,       -- domain enum
    stay_start      TIMESTAMPTZ NOT NULL,
    stay_end        TIMESTAMPTZ NOT NULL,
    -- ... all the fields the legacy app cares about
    -- ... plus our own extensions (e.g. created_by_user_id)
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON ht_bookings (legacy_book_id) WHERE legacy_book_id IS NOT NULL;
```

**Why UUID first, legacy ID second:**
- We can create a booking instantly. Writeback later fills in `legacy_book_id` from the MAX+1 allocation.
- If MSSQL is down, our app keeps working — writeback queues up.
- When legacy is decommissioned, the `legacy_book_id` column stays as historical reference (set on rows created during States A/B).

### 4b. Legacy mirror is a separate schema, not entangled

```sql
CREATE SCHEMA legacy_mirror;

CREATE TABLE legacy_mirror.ht_book_h (
    -- exact mirror of MSSQL columns, MSSQL IDs as PK
    book_id     VARCHAR(50) PRIMARY KEY,
    book_date   TIMESTAMPTZ,
    -- ... etc
    mirror_synced_at TIMESTAMPTZ NOT NULL
);
```

The sync worker maintains `legacy_mirror.*`. The repository layer only reads canonical `public.ht_*`. The **service layer** sometimes consults `legacy_mirror.*` to detect drift or backfill — but that's a service-layer concern, not a repository one.

When legacy is decommissioned, drop the whole `legacy_mirror` schema.

### 4c. Outbox table

```sql
CREATE TABLE writeback_jobs (
    id                BIGSERIAL    PRIMARY KEY,
    intent            TEXT         NOT NULL,           -- 'create_booking', 'cancel_checkin', etc.
    payload           JSONB        NOT NULL,           -- domain data needed to replay
    aggregate_id      UUID         NOT NULL,           -- the PG row this is about
    idempotency_key   UUID         NOT NULL UNIQUE,
    status            TEXT         NOT NULL DEFAULT 'pending',  -- pending|in_progress|done|failed
    attempts          INT          NOT NULL DEFAULT 0,
    last_error        TEXT,
    legacy_ids        JSONB,                           -- e.g. {"book_id": "R014812", "cust_no": "C21613"}
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    completed_at      TIMESTAMPTZ
);

CREATE INDEX ON writeback_jobs (status, created_at) WHERE status IN ('pending', 'failed');
```

The service layer INSERTs into this in the same transaction as the canonical write. The worker LISTENs on `writeback_channel` and dequeues.

### 4d. Schema fingerprint guard

```sql
CREATE TABLE legacy_schema_fingerprint (
    captured_at         TIMESTAMPTZ PRIMARY KEY,
    column_signature    TEXT NOT NULL  -- hash of (table, column, type) for HT_* tables
);
```

Worker checks this on every startup; if MSSQL's actual schema differs from our captured fingerprint, refuse to write and alert. Protects against vendor schema changes silently corrupting writebacks.

---

## 5. Folder layout (target)

```
hotel-backend/src/
├── domain/                    ★ NEW — pure types, no I/O
│   ├── customer.rs            – Customer struct, validation
│   ├── booking.rs             – Booking + BookingState enum
│   ├── checkin.rs             – CheckIn + state machine
│   ├── room.rs
│   ├── payment.rs
│   ├── shared.rs              – DateRange, Money, RoomNumber, etc.
│   └── mod.rs
│
├── repository/                ★ NEW — PG-only data access
│   ├── customer.rs            – CustomerRepository trait + Pg impl
│   ├── booking.rs
│   ├── checkin.rs
│   ├── room.rs
│   ├── payment.rs
│   ├── outbox.rs              – writeback_jobs CRUD
│   └── mod.rs
│
├── service/                   ★ NEW — business logic + outbox emission
│   ├── customer.rs            – CustomerService
│   ├── booking.rs             – create/modify/cancel + outbox enqueue
│   ├── checkin.rs             – walk-in / book-in / cancel-checkin / extend
│   ├── checkout.rs            – check-out + housekeeping handoff
│   ├── payment.rs             – pay + receipt generation
│   ├── housekeeping.rs        – mark clean
│   └── mod.rs
│
├── routes/                    ◀── EXISTING, GETS THINNED OUT
│   ├── new_bookings.rs        – becomes ~50 LOC: deserialize → service → serialize
│   ├── new_checkins.rs        – likewise
│   └── ... (all 22 files thinned)
│
├── outbox/                    ★ NEW — outbox machinery
│   ├── intent.rs              – WritebackIntent enum (one variant per recipe)
│   ├── enqueue.rs             – emit a job + NOTIFY
│   └── mod.rs
│
├── writeback/                 ★ NEW — adapter to legacy MSSQL
│   ├── allocate.rs            – TABLOCKX MAX+1 helpers per counter
│   ├── walkin.rs              – §3a recipe from spike findings
│   ├── booking.rs             – §3b
│   ├── modify_booking.rs      – §3c
│   ├── checkin_to_booking.rs  – §3d
│   ├── extend.rs              – §3f
│   ├── checkout.rs            – §3e (without destructive Phase 1)
│   ├── payment.rs             – §3h
│   ├── cancel_booking.rs      – §3g-bis
│   ├── cancel_checkin.rs      – §3i
│   ├── housekeeping.rs        – §3j
│   ├── dispatcher.rs          – matches intent → recipe
│   └── mod.rs
│
├── sync/                      ◀── EXISTING scheduler/sync.rs moves here
│   ├── pull_legacy.rs         – MSSQL → legacy_mirror (existing logic)
│   ├── reconcile.rs           – ★ NEW — detect rows in legacy_mirror not in canonical → upsert
│   └── mod.rs
│
├── bin/
│   ├── api.rs                 ◀── EXISTING main.rs renamed (HTTP server only)
│   ├── writeback.rs           ★ NEW — LISTEN loop, dispatcher
│   ├── sync.rs                ★ NEW — separate scheduler binary (pulled out of API process)
│   └── ville_sync.rs          ◀── EXISTING (unchanged)
│
├── db/                        ◀── EXISTING (PG + MSSQL connection helpers)
├── error.rs                   – unified error types
└── lib.rs                     – module declarations
```

**Why split bin/api, bin/writeback, bin/sync into 3 binaries:**
- API can crash / restart without losing pending writebacks (currently a panic in sync crashes the API too — `main.rs:177-186`)
- Writeback can be turned off via deployment (don't run the container) without affecting API
- Each binary has its own resource limits and can scale independently
- Same Cargo workspace, shared `lib.rs` modules — code is reused

---

## 6. Per-route refactor pattern (worked example)

**Before** — `routes/new_bookings.rs` (current, ~600 LOC mixing concerns):
```rust
pub async fn create_booking(
    State(state): State<AppState>,
    Json(req): Json<CreateBookingReq>,
) -> Result<Json<BookingResp>, AppError> {
    // inline validation
    if req.stay_start >= req.stay_end { return Err(...); }
    
    // inline SQL
    let booking = sqlx::query_as!(Booking, r#"
        INSERT INTO ht_bookings (id, customer_id, state, stay_start, stay_end, ...)
        VALUES ($1, $2, 'pending', $3, $4, ...)
        RETURNING *
    "#, ...).fetch_one(&state.pg).await?;
    
    // (no writeback today)
    
    Ok(Json(booking.into()))
}
```

**After** — same file, ~30 LOC:
```rust
pub async fn create_booking(
    State(state): State<AppState>,
    Json(req): Json<CreateBookingReq>,
) -> Result<Json<BookingResp>, AppError> {
    let cmd = req.try_into()?;                          // → domain command
    let booking = state.bookings.create(cmd).await?;    // service does everything
    Ok(Json(booking.into()))
}
```

**Service** — `service/booking.rs` (new, ~80 LOC for create+modify+cancel):
```rust
impl BookingService {
    pub async fn create(&self, cmd: CreateBookingCommand) -> Result<Booking, ServiceError> {
        let mut tx = self.pg.begin().await?;
        
        // 1. Write to canonical PG (source of truth)
        let booking = self.repo.insert(&mut tx, NewBookingRecord {
            customer_id: cmd.customer_id,
            state: BookingState::Pending,
            stay_start: cmd.stay_start,
            stay_end: cmd.stay_end,
            // ...
        }).await?;
        
        // 2. Enqueue writeback (same transaction → atomic)
        self.outbox.enqueue(&mut tx, WritebackIntent::CreateBooking {
            booking_id: booking.id,
            payload: CreateBookingPayload::from(&cmd, &booking),
        }).await?;
        
        // 3. Commit. PG is consistent. Worker will async-push to MSSQL.
        tx.commit().await?;
        
        Ok(booking)
    }
}
```

**Worker** — `bin/writeback.rs` + `writeback/booking.rs`:
```rust
// writeback/booking.rs
pub async fn create_booking(
    pg: &mut PgTx, mssql: &mut MssqlTx, payload: CreateBookingPayload
) -> Result<LegacyIds, WritebackError> {
    // Allocate IDs with TABLOCKX
    let cust_no = allocate_cust_no(mssql).await?;
    let book_id = allocate_book_id(mssql).await?;
    
    // Apply the recipe from spike findings §3b
    insert_ht_customers(mssql, &payload, &cust_no).await?;
    insert_ht_book_h(mssql, &payload, &book_id, &cust_no).await?;
    insert_ht_book_ds(mssql, &payload, &book_id).await?;
    insert_ht_book_date(mssql, &payload, &book_id).await?;
    
    // Update PG row with the legacy IDs (so future writebacks can reference)
    sqlx::query!("UPDATE ht_bookings SET legacy_book_id = $1 WHERE id = $2",
                 &book_id, payload.booking_id)
        .execute(pg).await?;
    
    Ok(LegacyIds { book_id: Some(book_id), cust_no: Some(cust_no) })
}
```

**To decommission later:** delete `bin/writeback.rs`. Set `WRITEBACK_ENABLED=false`. Service code unchanged.

---

## 7. The 3-state toggle implementation

Single `.env` file controls all three states:

```bash
# State A or B (today/transition)
LEGACY_SYNC_ENABLED=true
WRITEBACK_ENABLED=true
LEGACY_DB_URL=mssql://sa:...@192.168.100.222/db

# State C (decommissioned)
LEGACY_SYNC_ENABLED=false
WRITEBACK_ENABLED=false
# LEGACY_DB_URL not set — workers refuse to start
```

In `docker-compose.yml`:
```yaml
services:
  api:
    image: ghcr.io/...
    # always runs
  
  workers:
    image: ghcr.io/...
    profiles: [legacy]   # only run when 'legacy' profile is active
    
# State A/B:  docker compose --profile legacy up -d
# State C:    docker compose up -d   (workers omitted)
```

When the writeback worker is disabled:
- Service layer still INSERTs into `writeback_jobs` (no harm — the table just accumulates jobs that nobody dequeues)
- Or: service layer checks a `WRITEBACK_ENABLED` flag and skips the outbox enqueue
- Decision: **always enqueue** so we have an audit trail of what would have been sent. Cheaper than refactoring service code later.

---

## 8. Migration roadmap (stay-current stack, decommission-ready)

| Phase | Time | What | Independently shippable? |
|---|---|---|---|
| **0** | 3 days | Frontend collapse — delete `app/(legacy)`, single tree | ✅ |
| **1** | 1 week | Domain + Repository layer scaffolding. Move ALL existing route SQL into PgRepository implementations. No behavior change. | ✅ — refactor only |
| **2** | 1 week | Service layer scaffolding. Move business logic out of routes into BookingService, CheckInService, etc. Routes become thin. | ✅ — still no behavior change |
| **3** | 3 days | Outbox table + WritebackIntent enum + outbox enqueue helpers. Wired into service layer. **Worker not yet built — jobs accumulate harmlessly.** | ✅ |
| **4** | 2 weeks | Writeback worker binary. Implement 11 flow recipes from spike. Schema fingerprint guard. Idempotency. **Goal #1 ✓** | ✅ |
| **5** | 3 days | Split scheduler into `bin/sync.rs` (own binary). Add reconcile logic — when sync detects new rows in legacy_mirror not in canonical, upsert into canonical. | ✅ |
| **6** | 1 week | Multi-site full deploy at HF Ville (after Phase 4 proven 1 month). Same image, different `.env`. **Goals #2 + #3 ✓** | ✅ |
| **∞** | 1 day (someday) | Decommission. Set `WRITEBACK_ENABLED=false`. Stop sync workers. Drop legacy_mirror schema. | ✅ |

**Total: ~5-6 weeks to production-ready writeback** + **1 day to decommission whenever you're ready**.

Phases 0-3 are pure refactoring — they ship the same behavior with cleaner code. Phase 4 is where the new capability lands. After Phase 4 the architecture is decommission-ready forever.

---

## 9. What changes vs. what stays (summary)

| Component | Status |
|---|---|
| Frontend (Next.js) | Stays. Single tree after Phase 0. |
| Backend language (Rust) | Stays. |
| Backend framework (Axum) | Stays. |
| Database 1 (Postgres) | Stays. **Becomes source of truth.** Schema audit + outbox added. |
| Database 2 (Legacy MSSQL) | External system. Eventually retired. |
| `routes/*.rs` | Stays but **thinned** — calls service, returns DTOs. |
| `scheduler/sync.rs` | Stays but **moves** to `bin/sync.rs`. Reconcile logic added. |
| `bin/ville_sync.rs` | Stays unchanged. |
| `main.rs` mode logic | Goes away — replaced by clean layer boundaries. |
| **NEW: `domain/`** | Pure types, no I/O |
| **NEW: `repository/`** | PG-only SQL behind traits |
| **NEW: `service/`** | Business logic + outbox emission |
| **NEW: `outbox/`** | Durable queue machinery |
| **NEW: `writeback/`** | MSSQL adapter (one file per recipe) |
| **NEW: `bin/writeback.rs`** | LISTEN loop + dispatcher |
| **NEW: `bin/sync.rs`** | Standalone sync scheduler |
| Frontend stack churn | Zero |
| New languages | Zero |
| New frameworks | Zero |

**Total new code: ~3-4k LOC** of well-scoped Rust modules. Mostly mechanical from the spike findings.

---

## 10. Confirming decisions still needed

1. **PG schema as source of truth** — confirmed by your "our own data layer" requirement
2. **Layered architecture (domain/repo/service)** — recommended approach. Pushback OK.
3. **UUID primary keys with `legacy_*_id` reference columns** — best practice for decommission readiness
4. **Outbox pattern with PG `LISTEN/NOTIFY`** vs alternatives (HTTP RPC, dedicated queue like Redis) — recommended outbox/PG because it requires no new infra and is durable
5. **Split into 3 binaries (api, sync, writeback)** vs keep monolith with feature flags — recommended split for blast-radius isolation
6. **Frontend collapse to single `/app/*` tree** — confirmed
7. **HF Ville deployment shape** — full stack at Ville (Phase 6) vs central-only with Tailscale tunnels — recommended full stack
