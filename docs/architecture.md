# Architecture

**Stack: Rust+Axum backend, Next.js 16+React 19 frontend, PostgreSQL, Legacy MSSQL via Change Tracking.**

> **Stack decision (2026-04-25):** Keep current stack. Rejected paths:
> - Full Elysia/Bun rewrite — would throw away the proven `tiberius`+`TABLOCKX` writeback infrastructure that the spike validated. ~2-3 month rewrite for marginal velocity gains.
> - Hybrid (Elysia API + Rust workers) — adds two-language complexity for one engineer.
> - Leptos / HTMX — a hotel-management UI is exactly what React's component ecosystem (calendars, datepickers, datatables, charts) was built for.
>
> The chosen path: layered architecture inside the existing Rust backend + event-driven sync. See §1 for the layout, §3.6 for the event bus, §8 for the migration roadmap.

**Core principle:** PostgreSQL is the **single source of truth** from day one.
The legacy MSSQL is treated as an **external system we currently mirror to/from** for backward-compatibility with the 3rd-party Windows app. When the 3rd-party app is decommissioned, we turn off the sync + writeback workers; everything else keeps working unchanged.

The architecture must support **three operational states** without code changes — only `.env` toggles:

```
State A (today):     Legacy app is primary UI       — sync + writeback both ON
State B (transition): Both apps coexist               — sync + writeback both ON
State C (decommissioned): Only our app                — sync + writeback both OFF
```

---

## 1. Layered architecture (event-driven)

```
                    BROWSER  ◀── SSE /api/events ──┐
                       │                            │
                       │ HTTPS                      │
                       ▼                            │ real-time
        ┌──────────────────────────────┐            │ event push
        │  Frontend: Next.js 16        │            │ (sub-100ms)
        │  – single /app/* tree        │            │
        │  – fetch /api/*              │            │
        │  – useRealtimeEvents() hook  │ ───────────┘
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
        │  Postgres (newdb) — SOURCE OF TRUTH + EVENT BUS │
        │                                                 │
        │  ┌─────────────────────────────┐                │
        │  │ canonical tables (ht_*)     │                │
        │  │ – customers, bookings, etc. │                │
        │  │ – PG owns UUID PKs internal │                │
        │  │ – stores legacy_id refs     │                │
        │  │ – writeback emits legacy-   │                │
        │  │   shape string IDs (C0001,  │                │
        │  │   R000001, CH26-000001)     │                │
        │  └─────────────────────────────┘                │
        │                                                 │
        │  ┌─────────────────────────────┐                │
        │  │ writeback_jobs (outbox)     │  ◀─┐           │
        │  │ – queue for legacy MSSQL    │    │ enqueued  │
        │  └─────────────────────────────┘    │ in same   │
        │                                     │ TX as     │
        │  ┌─────────────────────────────┐    │ canonical │
        │  │ event_log (durable bus)     │  ◀─┤ write     │
        │  │ – every domain event        │    │           │
        │  │ – pg_notify('domain_events')│    │           │
        │  │ – replay-safe               │    │           │
        │  └────────────┬────────────────┘    │           │
        │               │                     │           │
        │  ┌────────────│────────────────┐    │           │
        │  │ legacy_mir │or schema       │    │           │
        │  │ – read-onl │ snapshot of MSS│    │           │
        │  │ – CT water │ark stored too  │    │           │
        │  └────────────│────────────────┘    │           │
        └───────┬───────┴──────┬──────────────┴───────────┘
                │              │ NOTIFY
                │              │ 'domain_events'
                │ SELECT       │
                │              ├────────────────────┐
                ▼              ▼                    ▼
        ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐
        │ writeback    │  │ SSE          │  │ audit logger,    │
        │ worker       │  │ broadcaster  │  │ notifications,   │
        │ (Rust bin)   │  │ (in api proc)│  │ etc.             │
        │              │  │              │  │ (subscribers)    │
        │ – LISTEN'er  │  │ – LISTEN'er  │  └──────────────────┘
        │ – pops outbox│  │ – pushes via │
        │ – tiberius   │  │   SSE to all │
        │ – TABLOCKX   │  │   browsers   │
        └──────┬───────┘  └──────────────┘
               │ tiberius (TDS)
               ▼
        ┌─────────────────────┐
        │ Legacy MSSQL        │ ◀──── direct writes from .NET app
        │ (external system)   │
        │ + Change Tracking   │
        │   ENABLED           │
        └────────┬────────────┘
                 │ SYS_CHANGE_VERSION poll (every 1s)
                 ▼
        ┌──────────────────────────────────────┐
        │  bin/sync.rs  CT watcher             │
        │  – pulls changed rows                │
        │  – translates → DomainEvent          │
        │  – UPSERTs into public.ht_*          │
        │  – publishes to event bus            │
        │  – TOGGLE: LEGACY_SYNC_ENABLED       │
        └──────────────────────────────────────┘
        
        Note: bin/ville_sync.rs retired (task #77, 2026-04-30) post Ville cutover.
        HF Ville now uses the same bin/sync.rs CT watcher via per-site env.
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

## 3.5. Bidirectional data flow (the daily reality)

The receptionist will use BOTH apps during transition. Data must flow in both directions seamlessly. Our app must show:
- Bookings/check-ins/customers we created (obviously)
- **Bookings/check-ins/customers receptionist created in the .NET app today** (the use case you raised)
- Updates the .NET app made to records we created earlier

### 3.5a. Read path (always against canonical PG)

The repository ALWAYS reads from `public.ht_*`. Routes never know about MSSQL. But `public.ht_*` contains:
- ✅ Rows our app created (immediately visible)
- ✅ Rows the .NET app created (visible after next reconcile cycle, ~1-5 min)
- ✅ Rows that exist in both (de-duplicated by `legacy_book_id`)

```
        Browser
          │ GET /api/bookings?date=2026-04-25
          ▼
        Axum route
          │
          ▼
        BookingService::list_by_date()
          │
          ▼
        BookingRepository::list_by_date()  ──────┐
                                                  │
                       ┌─────────────────────────┐│
                       │ SELECT * FROM           ││
                       │   public.ht_bookings    │◀┘
                       │  WHERE stay_overlaps... │
                       └─────────────────────────┘
                       Returns:
                       - rows with legacy_book_id set (synced from .NET)
                       - rows with legacy_book_id NULL (we created, writeback pending)
                       - rows with both (we created, writeback completed)
                       
                       ALL appear in the result set indistinguishably.
```

### 3.5b. Write paths (both directions converge to canonical)

Two write origins, both end up in `public.ht_*`:

```
   ╭─────────────────────────────────────────────────────────────────────╮
   │                                                                     │
   │  Origin A: Our app                                                  │
   │  ════════════════════                                               │
   │   user clicks "create booking"                                      │
   │     │                                                               │
   │     ▼                                                               │
   │   Service: BEGIN TRAN                                               │
   │     INSERT public.ht_bookings (id=UUID-A, legacy_book_id=NULL)      │
   │     INSERT writeback_jobs (intent='create_booking', ...)            │
   │     NOTIFY                                                          │
   │   COMMIT                                                            │
   │                                                                     │
   │   ── ~5-50ms async later ──                                         │
   │                                                                     │
   │   Writeback worker:                                                 │
   │     allocate Book_ID via TABLOCKX → 'R014820'                       │
   │     INSERT into MSSQL (Book_ID='R014820', ...)                      │
   │     UPDATE public.ht_bookings SET legacy_book_id='R014820'          │
   │            WHERE id=UUID-A                                          │
   │                                                                     │
   │   Now visible in:                                                   │
   │   - Our app (was visible immediately at INSERT)                     │
   │   - .NET app (visible after writeback)                              │
   │                                                                     │
   ╰─────────────────────────────────────────────────────────────────────╯

   ╭─────────────────────────────────────────────────────────────────────╮
   │                                                                     │
   │  Origin B: .NET app                                                 │
   │  ════════════════════                                               │
   │   receptionist clicks "save" in 3rd-party Windows app               │
   │     │                                                               │
   │     ▼                                                               │
   │   .NET app: INSERT INTO HT_Book_H (Book_ID='R014821', ...)          │
   │             [direct T-SQL, no involvement from our system]          │
   │                                                                     │
   │   ── up to ~5 min later (sync interval) ──                          │
   │                                                                     │
   │   Sync worker Phase 1:                                              │
   │     Pull MSSQL changes → INSERT INTO legacy_mirror.ht_book_h        │
   │            (book_id='R014821', mirror_synced_at=NOW)                │
   │                                                                     │
   │   Sync worker Phase 2 (reconcile):                                  │
   │     SELECT mirror.* FROM legacy_mirror.ht_book_h mirror             │
   │      LEFT JOIN public.ht_bookings public ON public.legacy_book_id   │
   │                                          = mirror.book_id           │
   │      WHERE public.id IS NULL  -- new from .NET app                  │
   │                                                                     │
   │     For each new row:                                               │
   │       INSERT INTO public.ht_bookings                                │
   │              (id=NEW UUID, legacy_book_id='R014821', ...)           │
   │                                                                     │
   │   Now visible in:                                                   │
   │   - .NET app (was visible immediately at INSERT)                    │
   │   - Our app (visible after reconcile completes)                     │
   │                                                                     │
   ╰─────────────────────────────────────────────────────────────────────╯
```

**Net result: `public.ht_*` is the unified view of all bookings/customers/check-ins from both apps, with sub-5-minute eventual consistency.**

### 3.5c. Conflict resolution (when both apps modify the same row)

Rare in practice (one receptionist works on one booking at a time), but possible. Example:

```
T+0:00  .NET app reads R014820 (sees phone='0900000076')
T+0:30  Our app reads R014820  (sees phone='0900000076')
T+0:45  Our app saves phone='0900000099'
        → public.ht_bookings.updated_at = T+0:45
        → outbox enqueued (status=pending)
T+1:00  .NET app saves phone='0900000088'
        → MSSQL HT_Book_H.Book_Cust_Tel = '0900000088'
        → MSSQL has no concept of last-modified timestamp from us
T+1:15  Writeback worker fires our pending job
        → MSSQL HT_Book_H.Book_Cust_Tel = '0900000099' (overwrites theirs)
T+5:00  Sync runs
        → mirror.book_cust_tel = '0900000099' (matches us — no conflict)
```

**Order matters:** the writeback fires the moment our user clicks save. The .NET app saves to MSSQL directly. **The later writer wins at MSSQL.** If they collide perfectly within ~50ms (unlikely), TABLOCKX serializes them — one waits for the other, then proceeds with the latest data.

For the inverse (their write happens AFTER our user starts typing but BEFORE we save):
```
T+0:00  Our app reads R014820 (sees phone='0900000076')
T+0:15  .NET app saves phone='0900000088'
T+0:45  Our app saves phone='0900000099'
        → public.updated_at = T+0:45
        → outbox enqueued
T+1:00  Writeback fires → MSSQL gets '0900000099'
T+5:00  Sync runs
        → mirror.book_cust_tel = '0900000099' (because we overwrote at T+1:00)
```

**Last writer wins**, with no per-field merge. This is acceptable for hotel domain (rare same-record concurrent edits). To be more careful, we could:
- Show an "updated by another clerk" warning in our app's edit form (compare `updated_at` from initial load vs current state on save)
- Add an "audit" view that lists rows where mirror and canonical disagreed during reconcile (engineer reviews weekly)

Both are nice-to-have, not blockers.

### 3.5d. Sync via event bus (not polling)

**Replaced full-table polling with event-driven sync.** Latency target: sub-second in both directions.

See §3.6 below for the full event-driven design. Summary here:

| Direction | Mechanism | Latency |
|---|---|---|
| Our app → PG → MSSQL | PG `LISTEN/NOTIFY` 'domain_events' channel + `writeback_jobs` outbox + worker | **~50ms** local, ~200ms to MSSQL |
| .NET app → MSSQL → PG | SQL Server **Change Tracking** + watcher polling SYS_CHANGE_VERSION every 1 sec → publish to event bus | **~1 sec** worst case |
| PG → Frontend browsers | Axum SSE endpoint subscribes to PG NOTIFY → pushes to all connected clients | **~50ms** end-to-end |

**No more 1-minute polling tables.** The full-table scan moves to a fallback "reconcile drift" job that runs every 15 min as a safety net, not as the primary sync path.

### 3.5e. Transition-period reality

During State B, the receptionist will probably:
- Use whichever app they're more comfortable with for each task
- Notice **sub-2-second lag** for the OTHER app to "see" their changes (event-driven, not polling)
- Occasionally hit a conflict (last-write-wins)

This matches what they're already used to (the .NET app is multi-client, already has clerks racing each other with no conflict resolution). Our system adds nothing worse — and adds real-time UI updates as a bonus.

---

## 3.6. Event-driven sync design

**Goal:** sub-second propagation of changes from any source (our app, .NET app, system events) to all interested parties (other connected browsers, the writeback worker, the legacy MSSQL adapter, audit logs).

### 3.6a. The event bus topology

```
                                  EVENT BUS
                          (PG LISTEN/NOTIFY 'domain_events')
                                       │
       ┌───────────────────────────────┼───────────────────────────────┐
       │                               │                               │
   PUBLISHERS                          │                          SUBSCRIBERS
       │                               │                               │
       │                               │                               │
       │                               │                  ┌────────────▼─────────┐
   ┌───┴──────────────┐                │                  │ writeback worker     │
   │ Service layer    │                │                  │ – picks intent       │
   │ – publishes on   │ ──INSERT into──┤                  │   from outbox        │
   │   every write    │  pg_notify(...)│                  │ – pushes to MSSQL    │
   └──────────────────┘                │                  └──────────────────────┘
                                       │
                                       │                  ┌──────────────────────┐
   ┌──────────────────┐                │                  │ SSE broadcaster      │
   │ MSSQL CT watcher │ ──polls SYS_───┤                  │ – holds open HTTP    │
   │ – every 1 sec    │  CHANGE_VERSION│ ────receives───▶ │   conns to browsers  │
   │ – publishes      │  & publishes   │                  │ – pushes events      │
   │   detected       │                │                  │   to UI              │
   │   changes        │                │                  └──────────────────────┘
   └──────────────────┘                │
                                       │                  ┌──────────────────────┐
   ┌──────────────────┐                │                  │ audit log writer     │
   │ Cron jobs        │ ──occasionally─┤                  │ – appends every event│
   │ (notifications,  │                │                  │   to ht_audit_log    │
   │  reports, etc.)  │                │                  └──────────────────────┘
   └──────────────────┘                │
                                       │                  ┌──────────────────────┐
                                       │                  │ notification fanout  │
                                       │                  │ – Slack / email      │
                                       └──────────────▶  │ – filters by type    │
                                                          └──────────────────────┘
```

### 3.6b. Domain events (the contract)

Every change in the system emits a typed event:

```rust
// outbox/event.rs
pub enum DomainEvent {
    BookingCreated     { id: Uuid, source: EventSource, snapshot: BookingSnapshot },
    BookingModified    { id: Uuid, source: EventSource, before: BookingSnapshot, after: BookingSnapshot },
    BookingCancelled   { id: Uuid, source: EventSource, reason: Option<String> },
    
    CheckInCreated     { id: Uuid, source: EventSource, snapshot: CheckInSnapshot },
    CheckOutCompleted  { id: Uuid, source: EventSource },
    CheckInCancelled   { id: Uuid, source: EventSource, reason: Option<String> },
    
    CustomerCreated    { id: Uuid, source: EventSource, snapshot: CustomerSnapshot },
    CustomerModified   { id: Uuid, source: EventSource, changed_fields: Vec<String> },
    
    PaymentReceived    { check_in_id: Uuid, amount: Money, method: PaymentMethod, source: EventSource },
    
    RoomMarkedClean    { room_id: Uuid, by: String, source: EventSource },
    RoomMarkedDirty    { room_id: Uuid, source: EventSource },
}

pub enum EventSource {
    OurApp { user_id: Uuid, request_id: Uuid },     // came through our routes
    LegacyApp { detected_at: DateTime<Utc> },        // detected via Change Tracking
    System { reason: String },                       // scheduled job, reconcile, etc.
}
```

Event payloads are JSON-serializable, durable (stored in `event_log` table), and back-compatible (new fields are optional).

### 3.6c. Publication path (our writes)

Our app's writes publish events in the **same transaction** as the canonical PG write:

```rust
// service/booking.rs
impl BookingService {
    pub async fn create(&self, cmd: CreateBookingCommand) -> Result<Booking, ServiceError> {
        let mut tx = self.pg.begin().await?;
        
        // 1. Write canonical
        let booking = self.repo.insert(&mut tx, ...).await?;
        
        // 2. Enqueue writeback (existing)
        self.outbox.enqueue(&mut tx, WritebackIntent::CreateBooking { ... }).await?;
        
        // 3. Publish domain event
        self.events.publish(&mut tx, DomainEvent::BookingCreated {
            id: booking.id,
            source: EventSource::OurApp { user_id: cmd.user_id, request_id: cmd.request_id },
            snapshot: BookingSnapshot::from(&booking),
        }).await?;
        
        // 4. Commit. PG NOTIFY fires AFTER commit (PG semantics).
        //    All 3 effects are atomic — either all happen or none.
        tx.commit().await?;
        
        Ok(booking)
    }
}

// outbox/event.rs
impl EventBus {
    async fn publish(&self, tx: &mut PgTx, event: DomainEvent) -> Result<(), Error> {
        // Persist for audit + replay capability
        sqlx::query!(
            "INSERT INTO event_log (id, event_type, payload, source, created_at)
             VALUES (gen_random_uuid(), $1, $2, $3, NOW())",
            event.type_name(), serde_json::to_value(&event)?, event.source_json()
        ).execute(&mut **tx).await?;
        
        // Notify subscribers (fires on commit)
        sqlx::query!("SELECT pg_notify('domain_events', $1)",
                     serde_json::to_string(&event)?)
            .execute(&mut **tx).await?;
        
        Ok(())
    }
}
```

### 3.6d. Detection path (.NET app's writes via Change Tracking)

SQL Server Change Tracking is enabled per-database + per-table. **It doesn't modify table structure** (only adds metadata). The vendor's app is unaffected.

**Setup (one-time, requires sysadmin):**
```sql
ALTER DATABASE db SET CHANGE_TRACKING = ON
    (CHANGE_RETENTION = 2 DAYS, AUTO_CLEANUP = ON);

-- Enable per table we care about
ALTER TABLE HT_Customers     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Book_H        ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Book_Ds       ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Book_Date     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_CheckIn_H     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_CheckIn_Ds    ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_CheckIn_Pay   ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Receipt_H     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Receipt_Ds    ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Rooms         ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Room_Status   ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Rooms_Cancel  ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_Housewife     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
ALTER TABLE HT_POWER_LOG     ENABLE CHANGE_TRACKING WITH (TRACK_COLUMNS_UPDATED = ON);
```

**⚠️ Decision needed:** enabling Change Tracking technically counts as a database-level alteration. CLAUDE.md says no `ALTER TABLE` on legacy tables. CT enablement adds metadata storage but doesn't modify columns or change app behavior. **Need confirmation that this is acceptable** — if not, we fall back to high-frequency timestamp polling (more load on legacy DB, harder to detect deletes).

**Watcher loop (Rust binary, in `bin/sync.rs`):**
```rust
loop {
    let last_version = pg.get_last_seen_version().await?;
    
    let changes = mssql.query(format!(r#"
        DECLARE @current BIGINT = CHANGE_TRACKING_CURRENT_VERSION();
        
        -- Pull all changes since last_version, joined with current row data
        SELECT 
            ct.SYS_CHANGE_VERSION,
            ct.SYS_CHANGE_OPERATION,    -- 'I' / 'U' / 'D'
            ct.Book_ID,                  -- the PK
            h.*                          -- current row state (NULL if deleted)
        FROM CHANGETABLE(CHANGES HT_Book_H, {last_version}) ct
        LEFT JOIN HT_Book_H h ON h.Book_ID = ct.Book_ID
        ORDER BY ct.SYS_CHANGE_VERSION;
    "#)).await?;
    
    for change in changes {
        // Map MSSQL row → DomainEvent
        let event = translate_mssql_change_to_event(change);
        
        // Apply to canonical (UPSERT into public.ht_*)
        let mut tx = pg.begin().await?;
        apply_event_to_canonical(&mut tx, &event).await?;
        
        // Publish to event bus (other clients see it via SSE)
        events.publish(&mut tx, event).await?;
        
        tx.commit().await?;
    }
    
    pg.update_last_seen_version(@current).await?;
    
    sleep(Duration::from_secs(1)).await;  // 1-sec poll = ~1-sec worst-case latency
}
```

CT polling is **incremental** — we only pull rows changed since `@last_version`. Even on a busy day (~100 receptionist actions/hour), this is a few hundred rows/day total, queried once per second. Legacy DB load is negligible.

### 3.6e. Subscription path (real-time UI)

Browsers subscribe via Server-Sent Events (SSE):

```
   Browser                                    Axum               PG
     │                                          │                 │
     │  GET /api/events  (SSE, kept open)       │                 │
     ├─────────────────────────────────────────▶│                 │
     │                                          │                 │
     │                                          │  LISTEN         │
     │                                          │  domain_events  │
     │                                          ├────────────────▶│
     │                                          │                 │
     │                                          │                 │
     │  ── (long-lived connection) ──           │                 │
     │                                          │                 │
     │                                          │      NOTIFY     │
     │                                          │  ◀──────────────┤
     │                                          │  (event arrives)│
     │  data: {"type":"BookingCreated", ...}    │                 │
     │  ◀───────────────────────────────────────┤                 │
     │                                          │                 │
     │  React Query: invalidate('bookings')    │                 │
     │  → re-fetch /api/bookings                │                 │
     │                                          │                 │
```

**Frontend reaction pattern (recommended):**
```typescript
// lib/use-realtime-events.ts
export function useRealtimeEvents() {
  const queryClient = useQueryClient();
  
  useEffect(() => {
    const sse = new EventSource('/api/events');
    
    sse.addEventListener('BookingCreated', () => {
      queryClient.invalidateQueries({ queryKey: ['bookings'] });
    });
    sse.addEventListener('BookingModified', () => {
      queryClient.invalidateQueries({ queryKey: ['bookings'] });
    });
    sse.addEventListener('CheckInCreated', () => {
      queryClient.invalidateQueries({ queryKey: ['checkins'] });
      queryClient.invalidateQueries({ queryKey: ['rooms'] });  // room availability
    });
    // ... one mapping per event type
    
    return () => sse.close();
  }, [queryClient]);
}
```

**Why invalidate-and-refetch (not patch-from-event):** simpler, more correct, handles permission filtering centrally. The cost of an extra fetch is tiny (~5-15ms PG query).

### 3.6f. Latency budget end-to-end

Worst case (most pessimistic) for "user A in our app modifies a booking → user B in our app sees it":

```
T+0ms     User A clicks save
T+5ms     Browser POST /api/bookings/X
T+10ms    Axum starts handling
T+15ms    Service BEGIN TRAN
T+20ms    INSERT public.ht_bookings; INSERT outbox; INSERT event_log; pg_notify
T+25ms    COMMIT (NOTIFY fires)
T+30ms    SSE broadcaster wakes, sends to all connected browsers
T+35ms    User B's browser receives event
T+40ms    React Query invalidates
T+50ms    Re-fetch issued
T+65ms    User B's UI shows the new booking

Total: ~65ms — sub-100ms for in-app changes
```

For ".NET app modifies → user in our app sees it":
```
T+0ms        Receptionist clicks save in .NET app
T+5ms        MSSQL INSERT completes; CHANGE_TRACKING records version+1
T+0..1000ms  CT watcher's next poll cycle
T+1010ms     Watcher pulls the change, builds DomainEvent
T+1020ms     INSERT canonical + event_log + pg_notify; COMMIT
T+1025ms     SSE broadcaster forwards to browsers
T+1080ms     Our app shows the change

Total: ~1 sec worst case (avg ~500ms, since events are uniformly distributed)
```

### 3.6g. What still needs polling (the safety net)

A 15-min reconcile job runs as a backstop, doing a full incremental scan:
- Catches any change CT missed (bug, watcher downtime, etc.)
- Verifies our `legacy_book_id` mappings are still correct
- Logs drift to `ht_reconcile_log` for engineer review

This is the existing `scheduler/sync.rs` logic, just downgraded from "primary sync" to "safety net."

### 3.6h. Why this isn't over-engineered

We were going to need:
- ✅ `pg_notify` channel for the writeback worker anyway (outbox queue)
- ✅ Some way to detect .NET app changes (existing 5-min polling we wanted to improve)
- ✅ Change Tracking is a free SQL Server feature, no infra
- ✅ SSE in Axum is ~50 LOC — `axum::response::sse`

The only NEW concept is "publish a domain event at every write." That's already what the outbox table is doing — we're just generalizing the channel.

### 3.6i. Trade-offs and limitations

| Concern | Mitigation |
|---|---|
| Change Tracking requires DB-level enablement | One-time setup, no schema modification, vendor unaffected. ⚠️ Needs user confirmation it's acceptable. |
| SSE doesn't auto-reconnect on network blips | Browser EventSource auto-reconnects (built into the spec). On reconnect, fire a "stale check" → invalidate all queries once. |
| Many subscribers on PG NOTIFY | PG handles thousands of LISTEN'ers fine. Real concern only at very high scale (not us). |
| Event ordering across publishers | We use Postgres `event_log.created_at` as the canonical order. Subscribers can query for "events since X" if they reconnect. |
| Event payload size | Snapshots can be large for booking-with-many-rooms. Mitigation: include only the aggregate's primary fields; subscribers re-fetch full state if needed. |
| What if event_log fills up? | Time-based retention (drop events older than 30 days). Audit log fans out to a separate cold-storage table. |
| What if PG NOTIFY drops a message? | Treat NOTIFY as **best-effort**. Subscribers rely on event_log + a "since" query for guaranteed delivery. Reconnection always replays missed events from event_log. |
| What if the CT watcher is down? | When it restarts, queries with the saved `last_version` and replays everything missed. CT retains 2 days. |
| Polling frequency (1 sec) | Tunable via env. `CT_POLL_INTERVAL_MS=500` for snappier UI; `=5000` if legacy DB load matters. |

---

## 3.7. Ground-truth principle (lessons from 2026-04 reverse-engineering)

The legacy MSSQL schema and the `.NET` app's behavior are not documented anywhere we control. We have three independent sources of truth, and after the 2026-04 reverse-engineering pass we now treat them in a strict precedence order. **When sources disagree, trust the higher tier.**

| Tier | Source | Location | Confidence |
|---|---|---|---|
| 1 | Live SQL Server captures (Extended Events, sniffed `RPC:Completed`) | `docs/legacy-spike/` (11 sessions + `findings.md`, ~700 LOC) | Highest — observed reality |
| 2 | Decompiled C# from the legacy `.exe` | `legacy-reference/` | High — but reflects intent, not always behavior |
| 3 | Inferred / cheatsheet analysis | scattered notes, early write-ups | Lowest — may be stale or wrong |

**Case in point — `HT_CheckIn_Ds.id`.** The original spike cheatsheet recorded this column as `IDENTITY` (autoincrement). The decompiled C# in `legacy-reference/` clearly shows the legacy app allocating the next id manually with `MAX(id)+1`, and a re-capture against the live DB confirmed the column is a plain `int`. We were one bug away from writebacks blowing up under concurrency. The current writeback (`hotel-backend/src/writeback/allocate.rs`) treats both `HT_CheckIn_Ds.id` and `HT_Receipt_H.id` as manual-allocation columns under `TABLOCKX, HOLDLOCK`. See §4a for the SQL recipe.

Other ground-truth facts that fall out of this precedence rule:

- **Pricing lives in the per-tier override table, not the room row.** Real per-room nightly prices come from `HT_Rooms_Price` keyed by `(Room_Type, Room_CustType='ราคาปกติ')`, columns `Room_Price` (nightly), `Room_Price_H` (hourly), `Room_Price_M` (monthly). The `HT_Rooms.Room_PriceA / Room_PriceB / Room_PriceC` columns are all zero in production — they look like the obvious source but they aren't. `bin/backfill_rooms.rs` was fixed in commit `8d8864e` to read from `HT_Rooms_Price`; any new pricing reads must do the same.

- **VAT is 7% inclusive, with a specific rounding rule.** Receipts and bookings store VAT-inclusive totals; the split is computed as:

  ```
  Total      = BeforeVat + Vat
  BeforeVat  = round(Total × 100 / 107, 2)   -- banker's rounding to 2dp
  Vat        = Total − BeforeVat
  ```

  Implemented in `hotel-backend/src/writeback/format.rs::vat_inclusive_split`, with regression tests against captured legacy values (801, 1390, 3560 baht). Receipts that compute VAT any other way will fail to match the printed totals from the legacy app, which is what the receptionists reconcile against.

- **Status enums are mixed Thai/English and contain bug-for-bug surprises** — every writeback recipe respects them exactly:
  - `HT_Rooms.room_status` — Thai: `'ว่าง'` (vacant), `'เข้าพัก'` (occupied), `'จอง'` (reserved).
  - `HT_CheckIn_Ds.Cin_Room_Status` — English **with a hyphen**: `'Check-In'`, `'Check-Out'`. The legacy `.NET` app has a known bug at `FrmCheckOut.cs:6246` that writes `'Check Out'` (space, no hyphen) on one path; we **write the hyphenated form** and **tolerate both forms on read**.
  - `HT_CheckIn_H.Cin_status` — Thai: `'ปกติ'` (normal), `'ยกเลิก'` (cancelled).
  - `HT_Customers.Cust_Type_Main` — value is `'ราคาปกติ'`. **The column-name casing differs by code path**: INSERTs use `Cust_Type_Main` (capital M), UPDATEs use `Cust_Type_main` (lowercase m). SQL Server is case-insensitive on identifiers so the legacy app got away with it; we preserve both spellings bug-for-bug so the writeback diffs cleanly against legacy-app traffic captures.

- **Text encoding is `varchar Thai_CI_AS` (Windows-874 / TIS-620), not Unicode.** Sending an `N'…'` Unicode literal corrupts every Thai character into `?` because `nvarchar → varchar` conversion strips anything outside the codepage. **Always pass plain `varchar` parameters** — the `tiberius` driver handles the codepage transcoding when the column type is `varchar`. The same rule applies to the `WHERE` clause: looking up a Thai name with `N'…'` will silently miss because the search text round-trips through the same lossy conversion.

When in doubt, do a fresh live capture against `<legacy-mssql-host>` rather than trusting a years-old note. The Extended Events session in `scripts/legacy-monitor/` is the canonical way to do this.

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

**Legacy ID formats** (writeback emits these into MSSQL — never inferred from PG UUIDs):

| Counter | Format | Regex | Example |
|---|---|---|---|
| `Cust_no` | `C` + 4-digit sequence | `^C\d{4,}$` | `C21607` |
| `Book_ID` | `R` + 6-digit zero-padded | `^R\d{6}$` | `R014810` |
| `Cin_no` | `CH` + 2-digit year + `-` + 6-digit zero-padded | `^CH\d{2}-\d{6}$` | `CH26-005228` |

The format helpers and per-counter `MAX(...)+1` allocators live in `hotel-backend/src/writeback/allocate.rs` (`allocate_cust_no`, `allocate_book_id`, `allocate_cin_no`). All allocators run inside `TABLOCKX, HOLDLOCK` to prevent races against the legacy app.

**Legacy PK landmine — `HT_CheckIn_Ds.id` and `HT_Receipt_H.id` are NOT IDENTITY columns.** The original spike notes incorrectly assumed they were `IDENTITY`; live SQL Server captures and the decompiled C# both confirm they are plain `int` PKs that the legacy app allocates manually. Writeback must therefore allocate them the same way as the prefixed string IDs above:

```sql
SELECT @next = ISNULL(MAX(id), 0) + 1
  FROM HT_CheckIn_Ds WITH (TABLOCKX, HOLDLOCK);
INSERT INTO HT_CheckIn_Ds (id, ...) VALUES (@next, ...);
-- COMMIT releases the lock
```

If you ever see the writeback skip the lock or rely on `SCOPE_IDENTITY()` here, that is a bug — see §3.7 for the source-of-truth precedence that caught this.

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

The sync worker maintains `legacy_mirror.*` AND **upserts new/changed rows into canonical `public.ht_*`** via the reconcile step (§4e below). The repository layer only ever reads `public.ht_*` — bidirectional flow is hidden from it.

When legacy is decommissioned, drop the whole `legacy_mirror` schema.

### 4e. Reconcile = how legacy data becomes visible to our app

**This is the missing piece for bidirectional flow.** The sync worker runs in two phases:

```
Phase 1: Pull MSSQL → legacy_mirror      (fast, ~5s, every 1-5 min)
Phase 2: Reconcile legacy_mirror → public.ht_*  (the bidirectional bridge)
```

Reconcile logic for each table:

```
For every row in legacy_mirror.ht_book_h:
  Find canonical row WHERE legacy_book_id = mirror.book_id
  
  CASE 1 — no canonical row exists yet:
    → INSERT into public.ht_bookings with new UUID,
      legacy_book_id = mirror.book_id, all other fields from mirror
    → Row is now visible to our app's reads
  
  CASE 2 — canonical exists, mirror is newer:
    (mirror.book_date > public.updated_at AND no pending writeback for this row)
    → UPDATE public.ht_bookings with mirror's values
    → Row is now refreshed from legacy
  
  CASE 3 — canonical exists, public is newer (we have a pending writeback):
    → SKIP. Our writeback will push to MSSQL shortly.
    → Don't pull stale legacy data over our newer changes.
  
  CASE 4 — canonical exists and we just wrote to MSSQL successfully:
    → Match found by legacy_book_id (set by writeback worker on success)
    → No-op, both sides consistent
```

**Key invariant:** reconcile must NEVER overwrite a canonical row that has a `pending` outbox job for it. Otherwise we'd lose our user's just-typed changes when the next sync cycle pulls the legacy app's stale data.

To enforce this, the reconcile step does:
```sql
UPDATE public.ht_bookings
   SET (...) = (...)
 WHERE id = $canonical_id
   AND NOT EXISTS (
     SELECT 1 FROM writeback_jobs
      WHERE aggregate_id = public.ht_bookings.id
        AND status IN ('pending', 'in_progress')
   )
   AND $mirror_updated_at > public.ht_bookings.updated_at;
```

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

### 4d-bis. Event log (durable bus)

```sql
CREATE TABLE event_log (
    id              UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type      TEXT         NOT NULL,         -- 'BookingCreated', 'CheckInCancelled', etc.
    aggregate_id    UUID,                          -- the entity this event is about (nullable for system events)
    payload         JSONB        NOT NULL,
    source_kind     TEXT         NOT NULL,         -- 'our_app' | 'legacy_app' | 'system'
    source_user_id  UUID,
    source_request_id UUID,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX ON event_log (created_at DESC);
CREATE INDEX ON event_log (aggregate_id, created_at DESC);
CREATE INDEX ON event_log (event_type, created_at DESC);

-- Retention: drop events older than 30 days (run nightly)
-- Audit log preservation: separate cold-storage table mirrors important events
```

Each subscriber (SSE broadcaster, writeback worker, audit logger) maintains its own `last_processed_event_id` cursor. On reconnect they replay anything missed:
```sql
SELECT * FROM event_log
 WHERE id > $cursor
 ORDER BY created_at, id;
```

### 4d-tris. Change Tracking watermark

```sql
CREATE TABLE legacy_ct_state (
    id                BIGINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),  -- single row
    last_seen_version BIGINT NOT NULL DEFAULT 0,
    last_polled_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
INSERT INTO legacy_ct_state (id, last_seen_version) VALUES (1, 0)
    ON CONFLICT DO NOTHING;
```

Single-row table tracks the highest `SYS_CHANGE_VERSION` we've successfully imported from MSSQL. On worker restart, we resume from this point.

### 4e. Schema fingerprint guard

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
│   └── (ville_sync.rs removed task #77 — HF Ville now uses bin/sync.rs)
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
LEGACY_DB_URL=mssql://sa:...@<legacy-mssql-host>/db

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

## 8. Migration roadmap (stay-current stack, decommission-ready, event-driven)

| Phase | Time | What | Independently shippable? |
|---|---|---|---|
| **0** | 3 days | Frontend collapse — delete `app/(legacy)`, single tree | ✅ |
| **1** | 1 week | Domain + Repository layer scaffolding. Move ALL route SQL into `PgRepository` implementations. No behavior change. | ✅ — refactor only |
| **2** | 1 week | Service layer scaffolding. Move business logic out of routes into `BookingService` etc. Routes become thin. | ✅ — still no behavior change |
| **3** | 3 days | Outbox table + `event_log` table + `WritebackIntent` enum + `DomainEvent` enum + service emission helpers. **No subscribers yet — events accumulate harmlessly.** | ✅ |
| **4a** | 3 days | SSE endpoint `/api/events` + browser `useRealtimeEvents` hook. **Now: our writes propagate to other browsers in <100ms.** | ✅ |
| **4b** | 2 weeks | Writeback worker binary. Implement 11 flow recipes from spike. Schema fingerprint guard. Idempotency. **Goal #1 ✓** | ✅ |
| **5 — TOP PRIORITY** | 1 week | **Missing half of co-existence.** Split scheduler into `bin/sync.rs` (own binary). Implement CT watcher loop on the 10 CT-enabled legacy tables → publishes detected changes to event bus. **Without this, State B is aspirational: the .NET app's writes are invisible to ours until the next 5-min reconcile, so receptionists working in two apps see stale data in ours.** With it: .NET-app writes propagate to our app in ~1 sec. | ✅ — CT already enabled & live-verified 2026-04-25 |
| **5.5** | 1 week | Read-only mirror tables for legacy-only features. CT watcher imports `HT_Cupon`, `HT_Deposit`, `HT_ContinueTime`, `HT_Changed_Room`, `HT_Bill_Debt_*`, `HT_CheckIn_Product`, `HT_Rooms_Cancel` into PG read-only schema. Our UI can SHOW these entities (coupons attached, deposits taken, products charged, room changes) even though our app can't EDIT them. Dramatically improves UX during co-existence — receptionists see the full picture in our app instead of switching to the .NET app for legacy-only features. | ✅ |
| **6** | 3 days | Drift-reconcile job (15-min cron) as safety net for missed CT events. Drop polling-sync (replaced by event-driven). | ✅ |
| **7** | 1 week | Multi-site full deploy at HF Ville (after Phase 4 proven 1 month). Same image, different `.env`. **Goals #2 + #3 ✓** | ✅ |
| **∞** | 1 day | Decommission. Set `WRITEBACK_ENABLED=false`, `LEGACY_SYNC_ENABLED=false`. Stop sync + writeback workers. SSE broadcaster keeps running for in-app real-time. Drop `legacy_mirror` schema. | ✅ |

**Total: ~6-7 weeks to production-ready writeback + event-driven sync** + **1 day to decommission**.

Phases 0-3 are pure refactoring + scaffolding. Phase 4a ships real-time UI updates for our own writes (immediately useful even without writeback). Phase 4b unlocks Goal #1. Phase 5 closes the loop with .NET-app-side detection.

### Why split Phase 4 into a/b

Phase 4a (SSE + DomainEvent emission) is **immediately user-visible**: if two staff members are looking at our app, their screens stay in sync without F5. Tiny, high-impact, gives you something to show stakeholders within a sprint of starting Phase 1.

Phase 4b (writeback worker) is the bigger lift but doesn't affect the UI behavior — it's the legacy-DB-facing adapter.

### Event-driven gives you a fourth state for free

| State | Workers active | Event bus active | Behavior |
|---|---|---|---|
| A — today | sync + writeback ON | YES | bidirectional event propagation in/out of MSSQL |
| B — transition | sync + writeback ON | YES | same as A; users gradually shift to our app |
| **A.5 — read-only sneak peek** | sync ON, writeback OFF | YES | our app shows .NET app data in real-time, doesn't write back. Useful for early UAT before writeback is trusted in production. |
| C — decommissioned | sync + writeback OFF | YES | our app continues with real-time UI sync between browsers; legacy MSSQL gone |

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
| `bin/ville_sync.rs` | **Retired (task #77, 2026-04-30)** — HF Ville now uses the same `bin/sync.rs` CT watcher with per-site env. |
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
4. **Outbox + event log via PG `LISTEN/NOTIFY`** vs alternatives (Redis Streams, NATS, RabbitMQ) — recommended PG because it requires no new infra and is durable. Tens of thousands of events/day is well within PG's comfort zone.
5. **Split into 3 binaries (api, sync, writeback)** vs keep monolith with feature flags — recommended split for blast-radius isolation
6. **Frontend collapse to single `/app/*` tree** — confirmed
7. **HF Ville deployment shape** — full stack at Ville (Phase 7) vs central-only with Tailscale tunnels — recommended full stack
8. ✅ **SQL Server Change Tracking enabled 2026-04-25** — CT (and primary keys, where missing) is live on the 10 tables that drive sync: `HT_Customers`, `HT_Rooms`, `HT_Room_Status`, `HT_Book_H`, `HT_Book_Ds`, `HT_Book_Date`, `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`, `HT_Receipt_H`. Vendor app unaffected. Rollback script lives in `scripts/legacy-monitor/` and the long-running XE session there records activity + alerts on errors. No further DBA approval needed for Phase 5; the watcher can be implemented immediately.
9. **SSE vs WebSockets for real-time UI** — recommended SSE (simpler, one-way is sufficient for our needs, auto-reconnects, works through proxies). Pushback OK if WebSockets are needed later for chat / collaborative editing.
10. **Event payload size budget** — recommend ≤8KB per event (full snapshots for small aggregates, just IDs for large ones — subscribers re-fetch).

---

## 11. Legacy-only features (opaque pass-through)

The legacy .NET app implements a number of features our app does not — and likely never will, until decommission. During co-existence (State A / B), receptionists may still create coupons, take standalone deposits, ring up minibar charges, etc. in the .NET app. Our app must not corrupt those rows on writeback, and ideally should *display* them so receptionists see a complete picture in our UI even when the underlying data was authored elsewhere.

The strategy is **opaque pass-through**: Phase 5.5 mirrors the relevant tables into a PG `legacy_mirror` schema as read-only copies. Our UI renders them as "informational" panels. We never write to these tables (with one documented exception, `HT_Rooms_Cancel`, already handled by the writeback path). On decommission, the mirror schema is dropped.

### 11a. Tables we do NOT replicate but MUST preserve

| Table | Legacy purpose | Our policy |
|---|---|---|
| `HT_Cupon` | Food/breakfast vouchers generated per check-in | Read-only mirror in Phase 5.5; never write |
| `HT_CheckIn_Product` | In-stay POS / minibar charges per room | Read-only mirror; show on folio |
| `HT_Deposit` | Standalone deposit ledger + refunds (FormShowDEPBack) | Read-only mirror; show "deposit on file" |
| `HT_ContinueTime` | Hourly extension price master | Read-only mirror; informational |
| `HT_Changed_Room` | Mid-stay room-move audit | Read-only mirror; show in stay history |
| `HT_Rooms_Cancel` | Per-room cancel audit (multi-room cancellation) | Already used by `checkin_cancel.rs` writeback (we DO write here — this exists as a known exception) |
| `HT_Rooms_Price` | Per-customer-type room price overrides | Read in `bin/backfill_rooms.rs`; never write |
| `HT_Bill_Debt_H` / `HT_Bill_Debt_Ds` | Credit-sales ledger | Read-only mirror; informational only |
| `HT_Order_Up` / `HT_Order_Down` | Per-customer-type pricing tiers | Read-only mirror; informational |
| `Tb_Save_Image` | Guest/ID photo varbinary blobs | Skip in Phase 5.5; photos stay legacy-app-only until our app implements its own photo capture |

### 11b. Behaviors we do NOT replicate (will trigger receptionist to switch to .NET app)

- Coupon generation/printing (`FrmCuponMain`)
- Standalone deposit & refund (`FrmAddDep`, `FormShowDEPBack`)
- In-stay POS / minibar (`FrmAddSale`, `FrmPayAddPro`)
- Tax invoice with VAT customer info (`FrmReceiptInvoice`, `FrmAddInvoiceSale`)
- Credit sales (`FrmPayDebt`, `FrmPayDebt2`)
- Hourly / time-extension pricing
- Room-move mid-stay
- Crystal Reports (replaced by future `bin/reports.rs` + QuestPDF — see `legacy-reference/analysis/_REPORTS_INVENTORY.md`)
- Photo capture (TWAIN / webcam) — out of scope until web-camera UX designed
- SMS sending (`FormSMSSendManual`, `FormSMS_DEBT`)

Reverse-sync (Phase 5) treats these tables as opaque — Change Tracking surfaces row-change notifications, our subscriber persists them into the read-only mirror schema, no semantic interpretation.
