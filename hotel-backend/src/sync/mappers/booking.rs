//! Booking aggregate Change Tracking mapper (Phase 5.3).
//!
//! ## Aggregate composition
//!
//! Three legacy tables form the booking aggregate:
//!
//! | MSSQL table       | Role             | Cardinality                   |
//! |-------------------|------------------|-------------------------------|
//! | `HT_Book_H`       | Header           | 1 per `Book_ID`               |
//! | `HT_Book_Ds`      | Line per room    | 1+ per `Book_No`              |
//! | `HT_Book_Date`    | Per-night ledger | 1+ per `(Book_no, Book_type)` |
//!
//! Per `docs/architecture.md` §3.6d, every CT row from any of the three
//! tables resolves the parent `Book_no`, re-loads the full aggregate via
//! [`crate::sync::parent_loader::load_booking_aggregate`], and runs an
//! idempotent UPSERT into `ht_bookings` + `ht_booking_rooms`. Exactly
//! one `DomainEvent` per aggregate per tick — coalesced by the watcher
//! (see [`MssqlChangeMapper::coalesce_key`]).
//!
//! ## Why three mappers, one helper
//!
//! Each table has its own CT-projection contract (column list, PK
//! shape) so the watcher needs three `MssqlChangeMapper` impls to
//! configure the SQL. But the *behaviour* is identical: figure out
//! which `Book_no` this row belongs to, then call
//! [`apply_booking_aggregate`]. The shared helper holds the single
//! source-of-truth UPSERT logic.
//!
//! ## Status mapping
//!
//! Per the user's standing constraint, legacy literals stay verbatim:
//!
//! | `HT_Book_H.Book_Status` | PG `ht_bookings.book_status` |
//! |-------------------------|-------------------------------|
//! | `'จอง'`                 | `'confirmed'`                 |
//! | `'เข้าพัก'`             | `'checked_in'`                |
//! | `'ยกเลิก'`              | `'cancelled'`                 |
//! | `'ออกแล้ว'`             | `'completed'`                 |
//! | (anything else)         | `'pending'`                   |
//!
//! NOTE on `'ออกแล้ว'` (corrected 2026-06-11): iHOTEL itself NEVER
//! writes this literal — its checkout flow does not touch `HT_Book_H`
//! at all. The value is written exclusively by OUR writeback checkout
//! recipe (`writeback/recipes/checkout.rs`), so seeing it here means
//! the CT row is an echo of our own writeback (absorbed by mapper
//! idempotency). The mapping is kept so the echo converges instead of
//! flapping.
//!
//! The PG-side string is what the existing booking routes (`routes/new_bookings`)
//! and the writeback recipes already round-trip; we don't introduce a
//! new enum.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::db::DbPool;
use crate::outbox::event::{BookingSnapshot, DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::gate_guard::{self, GateField, HashInput, HashInputContract};
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::checkin::resolve_customer_or_eager_mirror;
use crate::sync::parent_loader::BookingAggregate;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

// =============================================================================
// HT_Book_H — booking header mapper
// =============================================================================

const BOOK_H_TABLE: &str = "HT_Book_H";
const BOOK_DS_TABLE: &str = "HT_Book_Ds";
const BOOK_DATE_TABLE: &str = "HT_Book_Date";

/// CT mapper for `HT_Book_H`. Header changes are the most common driver
/// of an aggregate re-sync — every booking-create / modify / cancel
/// emits exactly one row here.
pub struct BookingHeaderMapper;

const BOOK_H_SELECT_COLS: &str = "t.Book_ID, t.Book_Cust_ID, t.Book_Status";

#[async_trait]
impl MssqlChangeMapper for BookingHeaderMapper {
    fn table(&self) -> &'static str {
        BOOK_H_TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // CT projects HT_Book_H by its varchar PK `Book_ID`. Verified
        // from migration 017 (legacy_sync_status seed) + the `Book_ID`
        // PK convention in cheatsheet §3.3.
        &["Book_ID"]
    }

    fn select_sql(&self) -> &'static str {
        BOOK_H_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        // The watcher's coalescing layer dispatches this aggregate
        // through [`apply_booking_aggregate`] using the watcher's
        // legacy-MSSQL pool — `apply` itself is a no-op that signals
        // "I belong to a coalesced aggregate" via [`coalesce_key`].
        // The only path that calls this `apply` directly is the
        // single-row dispatch (the legacy 5.2 path) — which never
        // happens for booking mappers because [`coalesce_key`] returns
        // `Some` for them, routing the watcher to the aggregate path.
        Ok(None)
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        // Header CT rows surface `Book_ID` as the PK. On a D the joined
        // row is NULL, but the watcher's `pk_<col>` aliasing keeps the
        // value addressable by the same column name (verified in
        // `bin/sync.rs::materialise_row`).
        row.try_get_str("Book_ID").ok().flatten().map(str::to_string)
    }
}

// =============================================================================
// HT_Book_Ds — line per room
// =============================================================================

/// CT mapper for `HT_Book_Ds`. Edits to the per-room line (price
/// change, room reassignment) drive a header re-load via the same
/// `apply_booking_aggregate` helper.
pub struct BookingRoomsMapper;

const BOOK_DS_SELECT_COLS: &str = "t.id, t.Book_No, t.Book_Room_Type, t.Book_status";

#[async_trait]
impl MssqlChangeMapper for BookingRoomsMapper {
    fn table(&self) -> &'static str {
        BOOK_DS_TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // SERIAL `id` is the PK CT keys on. `Book_No` is the parent FK
        // and is projected via SELECT so I/U rows carry it directly.
        // For D rows we cannot resolve `Book_No` from this row alone
        // (CT only carries the actual PK); the per-tick coalescing
        // layer picks the parent up via its own header / sibling CT
        // row almost always present in the same TX (cheatsheet §3.3
        // "Cancel cascade" + "Delete on edit").
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        BOOK_DS_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        Ok(None) // see BookingHeaderMapper::apply for rationale
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        // I/U rows: the joined `Book_No` is present.
        // D rows: `Book_No` is NULL because the parent join failed; we
        //         return None and rely on a sibling CT row (header
        //         UPDATE or another child's I/U) to pull this booking
        //         into the aggregate sweep.
        row.try_get_str("Book_No").ok().flatten().map(str::to_string)
    }
}

// =============================================================================
// HT_Book_Date — per-night ledger
// =============================================================================

/// CT mapper for `HT_Book_Date`. Adding / removing nights or marking a
/// night cancelled (`Book_ok=1`) all flow through the same aggregate
/// re-load.
pub struct BookingDatesMapper;

const BOOK_DATE_SELECT_COLS: &str = "t.id, t.Book_no, t.Book_date_ds, t.Book_USE, t.Book_ok";

#[async_trait]
impl MssqlChangeMapper for BookingDatesMapper {
    fn table(&self) -> &'static str {
        BOOK_DATE_TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // Same shape as HT_Book_Ds — CT keys on the SERIAL `id`. See
        // BookingRoomsMapper::primary_key_cols for the D-row
        // resolution caveat.
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        BOOK_DATE_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        Ok(None) // see BookingHeaderMapper::apply for rationale
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        row.try_get_str("Book_no").ok().flatten().map(str::to_string)
    }
}

// =============================================================================
// Shared aggregate-sync helper — called once per (book_id, tick) by the
// watcher's coalescing layer.
// =============================================================================

/// PG-side projection of an aggregate row that already exists. We use it
/// to detect `Created` vs `Modified` and to skip publish-on-no-change.
struct ExistingBooking {
    book_id_serial: i32,
    aggregate_id: Option<Uuid>,
    book_status: Option<String>,
    /// Read for completeness (debug logs / future drift-reconcile job)
    /// — not currently consumed beyond the SELECT projection. Kept to
    /// keep the struct shape aligned with the SELECT columns so a
    /// future test or diff routine doesn't need to widen the query.
    #[allow(dead_code)]
    book_cust_id: i32,
    book_total_amount: Option<f64>,
    book_deposit_amount: Option<f64>,
    book_checkin: NaiveDate,
    book_checkout: NaiveDate,
    /// Current `ht_booking_rooms` CONTENT for this booking — the
    /// `(br_room_id, br_price_per_night)` pair per junction row. Needed
    /// by `existing_matches` so a header-unchanged + rooms-changed
    /// transition (notably N→0 from iHOTEL's §3.7 delete-then-reinsert
    /// or §3.6 cancel-on-room flows) is NOT treated as idempotent.
    /// Without this the early-return skips `replace_rooms` and leaves
    /// stale junction rows behind — regression caught by
    /// `re_apply_with_zero_rooms_clears_stale_booking_rooms` in CI on
    /// 2026-05-18.
    ///
    /// Was a bare `count(*)` until 2026-07-28. A count is blind to the
    /// single most common iHOTEL room edit: `FrmAddBook2.SAVE_EDIT` is
    /// field-agnostic (DELETE + re-INSERT of all four booking tables on
    /// ANY edit), so a receptionist swapping room 402→403 re-writes an
    /// otherwise byte-identical header at a CONSTANT room count. Every
    /// gate term held, `apply_booking_aggregate` returned `Ok(None)`
    /// before `replace_rooms` ever ran, and canonical kept the old room
    /// permanently. The reconcile hash can't catch it either
    /// (`booking_canonical_hash` hashes `book_id|checkin|checkout|
    /// cust_no` only — no room data), so the class was silent in BOTH
    /// detection paths. 1176 of 1178 live bookings carry room numbers.
    ///
    /// Compared against the RESOLVED projection lines (not raw
    /// `projection.rooms`) since 2026-06-11 — see
    /// [`booking_rooms_match`] for why that stays true of the set
    /// comparison.
    rooms: Vec<ExistingBookingRoom>,
    /// Denormalised customer pointer — compared by `existing_matches`
    /// since 2026-06-11 (audit P1 #6): iHOTEL's customer-delete cascade
    /// (`UPDATE HT_Book_H SET Book_Cust_ID='C0000'`, cheatsheet §3.24)
    /// changes ONLY this column, so a status/amount/date comparison
    /// silently skipped the re-point.
    legacy_cust_no: Option<String>,
    /// Booking notes — included in the idempotency comparison (guarded
    /// on the projection carrying a value, mirroring the
    /// `COALESCE($7, book_notes)` write semantics) so a notes-only
    /// iHOTEL edit re-applies instead of silently skipping.
    book_notes: Option<String>,
}

/// One `ht_booking_rooms` row as it currently lives in PG. Deliberately
/// the same shape as [`ResolvedRoomLine`] (the write side): the gate's
/// job is to answer "would `replace_rooms` be a no-op?", so it compares
/// the room identity + price EXACTLY as `replace_rooms` writes them
/// (`br_room_id`, `br_price_per_night`). No `ht_rooms_new` join is
/// needed — `room_no` is `NOT NULL UNIQUE` there, so `room_id` is a
/// faithful stand-in for the legacy room identity, and keying on the FK
/// removes any chance of the gate and the mutation disagreeing.
#[derive(Debug, Clone, PartialEq)]
struct ExistingBookingRoom {
    room_id: i32,
    price_per_night: Option<f64>,
}

/// In-memory projection of the legacy aggregate, in canonical PG shape.
/// This is what we UPSERT.
#[derive(Debug, Clone, PartialEq)]
struct CanonicalProjection {
    legacy_book_id: String,
    legacy_cust_no: Option<String>,
    book_status: String,
    book_checkin: NaiveDate,
    book_checkout: NaiveDate,
    total_amount: Option<f64>,
    deposit_amount: Option<f64>,
    notes: Option<String>,
    /// One per `HT_Book_Ds` row — the room number (legacy stores it in
    /// the misleading `Book_Room_Type` column per cheatsheet §3.4) +
    /// optional per-room price.
    rooms: Vec<RoomLine>,
}

#[derive(Debug, Clone, PartialEq)]
struct RoomLine {
    room_no: String,
    price_per_night: Option<f64>,
}

/// Re-sync one booking aggregate. Idempotent — safe to call any number
/// of times for the same `book_id` per tick.
///
/// `mssql` is borrowed for the customer eager-mirror fallback: when the
/// booking references a `Cust_no` not yet in PG, the matching
/// `HT_Customers` row is fetched from MSSQL and mirrored in the same TX
/// (the June-3 2026 fix — see below). Pass `Some(&pool)` from the
/// watcher; `None` only from contexts without legacy access (tests),
/// where an unresolvable customer becomes an error instead.
///
/// Returns:
/// * `Ok(Some(DomainEvent))` when the canonical row genuinely changed
///   (or a new one was inserted, or the booking was cancelled).
/// * `Ok(None)` ONLY when the canonical row already mirrors the legacy
///   aggregate (idempotent skip).
/// * `Err(SyncError::Mapper)` when the customer FK cannot be resolved
///   even after the eager-mirror attempt. The watcher records the error
///   and HOLDS the watermark (loud retry).
///
/// ## The 2026-06-03 silent drop (C22209 / R015290)
///
/// Pre-fix, a customer-FK miss here returned `Ok(None)`. The watcher
/// counted that as `skipped` — not `errored` — so the watermark
/// advanced past the booking's CT version. Nothing ever re-fired the
/// aggregate (the `resolve.rs` "next CT tick re-surfaces it" contract
/// was false), and after the 2-day CT retention both the customer and
/// the booking were unrecoverable. The fix is the same eager-mirror
/// pattern the check-in mapper already had
/// (`checkin::resolve_customer_or_eager_mirror`): pull the customer
/// from MSSQL synchronously inside this TX, and error (never skip) if
/// that is impossible.
///
/// ## Header-only bookings (iHOTEL coexistence)
///
/// `aggregate.rooms.is_empty()` is a **legitimate, persistable state** —
/// it mirrors the legacy `HT_Book_H` row existing with zero matching
/// `HT_Book_Ds` lines. Several iHOTEL flows leave the aggregate in this
/// shape:
///
/// 1. **ClickBook cancel-on-room** (cheatsheet §3.6) deletes every
///    `HT_Book_Ds` for the booking but does NOT delete the `HT_Book_H`
///    header — the header lingers (often with `Book_Status='ยกเลิก'` or
///    `'จอง'`) while the per-room lines are gone.
/// 2. **FrmAddBook2.SAVE_EDIT delete-then-reinsert** (cheatsheet §3.7)
///    transiently deletes every `HT_Book_Ds` before re-inserting the
///    edited set. CT can surface a snapshot mid-edit.
/// 3. **Pre-bootstrap / pre-CT data**: bookings that completed before
///    Phase 5.x CT bootstrap may have lost their Ds rows to the
///    `frmMain1` 60-day startup prune (cheatsheet §3.7 "Startup prune")
///    while the legacy app retains the header for receipt / audit
///    lookups via `HT_CheckIn_H.Cin_Book_no`.
///
/// We MUST mirror these faithfully so downstream FKs that point at the
/// header (most importantly `HT_CheckIn_H.Cin_Book_no` →
/// `ht_checkins.cin_book_id`) can resolve. Otherwise check-ins
/// referencing such a booking stay deferred forever (the 2026-05-18
/// "18 stuck check-ins" PROD-CRIT incident — see
/// `docs/coexistence/`).
///
/// Header-only bookings produce a canonical `ht_bookings` row with ZERO
/// matching `ht_booking_rooms` rows. The "is this a header-only / walk-in
/// shaped booking?" predicate is therefore the existence query
/// `SELECT count(*) = 0 FROM ht_booking_rooms WHERE br_book_id = ?` —
/// no schema column required.
pub async fn apply_booking_aggregate(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    aggregate: &BookingAggregate,
    book_id: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    if !aggregate.is_present() {
        return apply_cancelled(tx, book_id).await;
    }

    let projection = project_aggregate(aggregate, book_id)?;
    let existing = fetch_existing(tx, book_id).await?;

    // Resolve the per-line room FKs BEFORE the idempotency check so the
    // junction comparison sees only RESOLVABLE lines (2026-06-11 fix —
    // see `ExistingBooking::rooms`). Unresolvable lines are skipped,
    // matching `replace_rooms`'s historical behaviour: the gate compares
    // what will be WRITTEN against what is STORED, never the raw legacy
    // line list.
    let resolved_rooms = resolve_room_lines(tx, book_id, &projection.rooms).await?;

    // Idempotent skip — every projected field matches the canonical row,
    // INCLUDING the per-room (room_id, price) set (2026-07-28: a bare
    // room count let iHOTEL's constant-cardinality room swap through).
    if let Some(ex) = existing.as_ref() {
        if existing_matches(ex, &projection, &resolved_rooms) && ex.aggregate_id.is_some() {
            return Ok(None);
        }
    }

    // Resolve the customer FK before the UPSERT — `ht_bookings.book_cust_id`
    // is `NOT NULL` and references `ht_customers(cust_id)`. On miss,
    // eager-mirror the customer from MSSQL inside this TX (June-3 2026
    // fix); if even that fails, ERROR so the watermark holds — never
    // return Ok(None) for an FK miss (the watcher would count it
    // `skipped` and advance the watermark: permanent silent drop).
    let cust_id =
        match resolve_customer_or_eager_mirror(tx, mssql, projection.legacy_cust_no.as_deref())
            .await?
        {
            Some(id) => id,
            None => {
                return Err(SyncError::Mapper {
                    table: BOOK_H_TABLE,
                    message: format!(
                        "customer FK unresolvable for book_id={book_id} \
                         legacy_cust_no={:?} — eager-mirror failed or no MSSQL \
                         pool; holding watermark for loud retry",
                        projection.legacy_cust_no
                    ),
                });
            }
        };

    // Trace the header-only / walk-in-shaped applies so operators can
    // see them in production logs without grepping for the absence of
    // a `ht_booking_rooms` count. See the doc comment above for the
    // legitimate iHOTEL flows that produce this shape.
    if projection.rooms.is_empty() {
        tracing::debug!(
            target: "sync::booking",
            book_id,
            legacy_cust_no = ?projection.legacy_cust_no,
            book_status = %projection.book_status,
            "applying header-only booking (no HT_Book_Ds lines) — \
             unblocks downstream HT_CheckIn_H.Cin_Book_no FK resolution"
        );
    }

    let (book_id_serial, agg_id, was_insert) = match existing {
        Some(ex) => {
            let agg_id = ex
                .aggregate_id
                .unwrap_or_else(|| aggregate_uuid(AggregateKind::Booking, ex.book_id_serial));
            update_existing(tx, ex.book_id_serial, cust_id, &projection, agg_id).await?;
            (ex.book_id_serial, agg_id, false)
        }
        None => {
            let new_id = insert_new(tx, cust_id, &projection).await?;
            let agg_id = aggregate_uuid(AggregateKind::Booking, new_id);
            sqlx::query("UPDATE ht_bookings SET aggregate_id = $1 WHERE book_id = $2")
                .bind(agg_id)
                .bind(new_id)
                .execute(&mut **tx)
                .await?;
            (new_id, agg_id, true)
        }
    };

    // `replace_rooms` is the single mutation point for `ht_booking_rooms`.
    // It must be called even when the resolved set is empty so that an
    // edit transitioning a booking from N-rooms → 0-rooms (a legitimate
    // mid-edit state per the §3.7 delete-then-reinsert pattern) drops
    // the stale junction rows.
    replace_rooms(tx, book_id_serial, &resolved_rooms).await?;

    let event = build_event(was_insert, agg_id, cust_id, &projection);
    Ok(Some(event))
}

/// Mark the canonical row cancelled when the legacy header has gone
/// (delete or null-header re-load). Emits `BookingCancelled` if a row
/// existed; `Ok(None)` otherwise.
async fn apply_cancelled(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    book_id: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    let existing = fetch_existing(tx, book_id).await?;
    let Some(ex) = existing else {
        // Nothing to cancel.
        return Ok(None);
    };
    if ex.book_status.as_deref() == Some("cancelled") {
        // Already cancelled — idempotent no-op.
        return Ok(None);
    }
    sqlx::query(
        "UPDATE ht_bookings \
            SET book_status = 'cancelled', \
                updated_at  = NOW() \
          WHERE book_id = $1",
    )
    .bind(ex.book_id_serial)
    .execute(&mut **tx)
    .await?;
    let agg_id = ex
        .aggregate_id
        .unwrap_or_else(|| aggregate_uuid(AggregateKind::Booking, ex.book_id_serial));
    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };
    Ok(Some(DomainEvent::BookingCancelled {
        id: agg_id,
        source,
        reason: Some("legacy app cancelled or deleted booking".into()),
    }))
}

// -----------------------------------------------------------------------------
// Projection helpers
// -----------------------------------------------------------------------------

/// Project the legacy booking aggregate (`HT_Book_H` + `HT_Book_Ds` +
/// `HT_Book_Date`) onto our canonical PG row shape.
///
/// **Header-only is legal:** when `agg.rooms` is empty the returned
/// `CanonicalProjection.rooms` is also empty. The caller
/// (`apply_booking_aggregate`) creates the `ht_bookings` row with zero
/// matching `ht_booking_rooms` rows — see that function's doc comment
/// for the iHOTEL flows that legitimately produce this shape.
///
/// **Intentional drop (Track E1 / T2 MED-1, audit 2026-05-13):** the
/// per-night `agg.nights` collection (loaded from `HT_Book_Date`) is
/// NOT read here. Each `Book_Date` row carries `Book_ok` which the
/// legacy app flips to `1` when that specific night is cancelled
/// mid-stay (e.g. guest checks out early on a 5-night booking → 2 of
/// the 5 `Book_Date` rows flip). Surfacing a "nights cancelled" count
/// on `ht_bookings` waits on a Track E2 / Track G schema column —
/// until then the aggregate parent-loader keeps the rows present (so
/// the bus shape mirrors the legacy schema) and this projection
/// drops them. See `parent_loader::load_booking_aggregate` for the
/// matching note on the load side.
fn project_aggregate(
    agg: &BookingAggregate,
    book_id: &str,
) -> Result<CanonicalProjection, SyncError> {
    let header = agg.header.as_ref().ok_or_else(|| SyncError::Mapper {
        table: BOOK_H_TABLE,
        message: "project_aggregate called with header=None".into(),
    })?;

    let legacy_cust_no = header.try_get_str("Book_Cust_ID")?.map(str::to_string);
    let legacy_status = header
        .try_get_str("Book_Status")?
        .unwrap_or_default()
        .to_string();
    let book_status = legacy_status_to_pg(&legacy_status).to_string();

    let (book_checkin, book_checkout) = derive_stay_range(header)?;

    let total_amount = header.try_get_decimal("Book_Price_Total")?;
    let deposit_amount = header.try_get_decimal("Book_Price_Pay")?;
    let notes = header
        .try_get_str("Book_room_note")?
        .map(str::to_string)
        .filter(|s| !s.is_empty());

    // `HT_Book_H.Book_room_type` disambiguates what `HT_Book_Ds.
    // Book_Room_Type` holds (cheatsheet §1.5 / §3.3 / §3.4):
    //   * 1 — booking "ระบุประเภทห้อง" (FrmAddBook, no specific rooms):
    //         Ds lines carry a room-TYPE code, NOT a room number.
    //   * 2 — booking with specific rooms (FrmAddBook2): Ds lines carry
    //         the room NUMBER.
    // Pre-2026-06-11 the mapper treated every Ds line as a room number;
    // for type-1 bookings each line failed the `ht_rooms_new.room_no`
    // lookup, warn-skipped, and the `rooms_count` mismatch made every CT
    // touch re-emit `BookingModified` forever. Type-1 bookings now
    // project as header-only (zero room assignments — there ARE no
    // specific rooms to assign). `.ok().flatten()` tolerates fixtures /
    // pre-widening loads that don't carry the column; absent defaults to
    // the room-number interpretation (type-2 behaviour, the historical
    // path).
    let book_room_type = header.try_get_i32("Book_room_type").ok().flatten();
    let mut rooms = Vec::with_capacity(agg.rooms.len());
    if book_room_type != Some(1) {
        for r in &agg.rooms {
            // Skip cancelled lines (Book_status=3 per cheatsheet §3.4).
            let line_status = r.try_get_i32("Book_status").ok().flatten();
            if line_status == Some(3) {
                continue;
            }
            // Misleading column name — stores room NUMBER per spike §3b /
            // cheatsheet §3.4 (when Book_room_type=2; see above).
            let Some(room_no) = r.try_get_str("Book_Room_Type")?.map(str::to_string) else {
                continue;
            };
            let price_per_night = r.try_get_decimal("Book_Room_Price")?;
            rooms.push(RoomLine {
                room_no,
                price_per_night,
            });
        }
    } else if !agg.rooms.is_empty() {
        tracing::debug!(
            target: "sync::booking",
            book_id,
            ds_lines = agg.rooms.len(),
            "Book_room_type=1 (no specific rooms): Ds lines carry room-TYPE \
             codes — projecting header-only, no ht_booking_rooms assignments"
        );
    }

    Ok(CanonicalProjection {
        legacy_book_id: book_id.to_string(),
        legacy_cust_no,
        book_status,
        book_checkin,
        book_checkout,
        total_amount,
        deposit_amount,
        notes,
        rooms,
    })
}

/// Translate the legacy `Book_Status` string to our PG canonical literal.
/// Unknown / empty values → `'pending'`.
fn legacy_status_to_pg(legacy: &str) -> &'static str {
    match legacy {
        "จอง" => "confirmed",
        "เข้าพัก" => "checked_in",
        "ยกเลิก" => "cancelled",
        "ออกแล้ว" => "completed",
        _ => "pending",
    }
}

/// `HT_Book_H.Book_Date_in/out` carry the stay start/end as datetimes
/// (legacy stores them at midnight per the booking-create recipe). PG's
/// `book_checkin/checkout` are `DATE` columns — drop the time component.
fn derive_stay_range(header: &dyn MappableRow) -> Result<(NaiveDate, NaiveDate), SyncError> {
    let date_in: NaiveDateTime = header
        .try_get_datetime("Book_Date_in")?
        .ok_or_else(|| SyncError::Mapper {
            table: BOOK_H_TABLE,
            message: "Book_Date_in is NULL on header".into(),
        })?;
    let date_out: NaiveDateTime = header
        .try_get_datetime("Book_Date_out")?
        .ok_or_else(|| SyncError::Mapper {
            table: BOOK_H_TABLE,
            message: "Book_Date_out is NULL on header".into(),
        })?;
    Ok((date_in.date(), date_out.date()))
}

// -----------------------------------------------------------------------------
// PG access — runtime queries (NOT `query!` macros, to keep this file
// out of the .sqlx offline cache during 5.x churn).
// -----------------------------------------------------------------------------

async fn fetch_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    legacy_book_id: &str,
) -> Result<Option<ExistingBooking>, SyncError> {
    let row = sqlx::query_as::<_, (
        i32,
        Option<Uuid>,
        Option<String>,
        i32,
        Option<f64>,
        Option<f64>,
        NaiveDate,
        NaiveDate,
        Option<String>,
        Option<String>,
    )>(
        "SELECT book_id, aggregate_id, book_status, book_cust_id, \
                book_total_amount::float8, book_deposit_amount::float8, \
                book_checkin, book_checkout, legacy_cust_no, book_notes \
           FROM ht_bookings \
          WHERE legacy_book_id = $1 \
          LIMIT 1",
    )
    .bind(legacy_book_id)
    .fetch_optional(&mut **tx)
    .await?;

    let Some((
        book_id_serial,
        aggregate_id,
        book_status,
        book_cust_id,
        book_total_amount,
        book_deposit_amount,
        book_checkin,
        book_checkout,
        legacy_cust_no,
        book_notes,
    )) = row
    else {
        return Ok(None);
    };

    // Room CONTENT, not a count (2026-07-28) — a count is blind to the
    // constant-cardinality room swap that iHOTEL's field-agnostic
    // `SAVE_EDIT` produces. `::float8` mirrors the header amounts'
    // read-back cast; the column is `DECIMAL(10,2)`.
    let rooms: Vec<ExistingBookingRoom> = sqlx::query_as::<_, (i32, Option<f64>)>(
        "SELECT br_room_id, br_price_per_night::float8 \
           FROM ht_booking_rooms \
          WHERE br_book_id = $1",
    )
    .bind(book_id_serial)
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(|(room_id, price_per_night)| ExistingBookingRoom {
        room_id,
        price_per_night,
    })
    .collect();

    Ok(Some(ExistingBooking {
        book_id_serial,
        aggregate_id,
        book_status,
        book_cust_id,
        book_total_amount,
        book_deposit_amount,
        book_checkin,
        book_checkout,
        rooms,
        legacy_cust_no,
        book_notes,
    }))
}

/// Compare the existing canonical row to the freshly projected legacy
/// one. Skip publication when every mirrored field matches.
///
/// `resolved` is the output of [`resolve_room_lines`] — the projection
/// lines whose room actually resolves in `ht_rooms_new`, i.e. EXACTLY
/// what `replace_rooms` is about to write. It is NOT `p.rooms`: lines
/// that don't resolve never reach the junction, so comparing against the
/// raw projection could never converge (every CT touch would re-emit
/// `BookingModified`). See [`booking_rooms_match`].
///
/// `legacy_cust_no` and `notes` comparisons are guarded on the
/// projection carrying a value, mirroring their `COALESCE($n, existing)`
/// write-semantics: a transient NULL on the legacy side never overwrites
/// the canonical value, so treating it as a mismatch would also force a
/// non-converging re-apply every tick. A Some-valued change — including
/// the `'C0000'` customer-delete-cascade re-point (cheatsheet §3.24) and
/// a notes-only edit — MUST mismatch so the apply re-runs.
fn existing_matches(
    ex: &ExistingBooking,
    p: &CanonicalProjection,
    resolved: &[ResolvedRoomLine],
) -> bool {
    // Two stages, deliberately NOT flattened into one table: the header
    // terms compare `ExistingBooking` against the projection, while the
    // room stage compares two SLICES. Both are pure, so evaluating the
    // header block first is behaviour-identical to the previous
    // hand-written `&&` chain.
    HEADER_GATE_FIELDS.iter().all(|f| (f.matches)(ex, p))
        && booking_rooms_match(&ex.rooms, resolved)
}

/// The header half of the idempotency gate, as NAMED comparators.
///
/// [`existing_matches`] is `.all()` over this table, so removing a name
/// removes the comparison. Names are the canonical (PG) column, which is
/// also what `scheduler::sync::booking_canonical_hash` reads, so
/// [`HASH_INPUTS`] cites them directly. See [`crate::sync::gate_guard`].
const HEADER_GATE_FIELDS: [GateField<ExistingBooking, CanonicalProjection>; 7] = [
    GateField {
        name: "book_status",
        guarded: false,
        matches: |ex, p| ex.book_status.as_deref() == Some(p.book_status.as_str()),
    },
    GateField {
        name: "book_total_amount",
        guarded: false,
        matches: |ex, p| ex.book_total_amount == p.total_amount,
    },
    GateField {
        name: "book_deposit_amount",
        guarded: false,
        matches: |ex, p| ex.book_deposit_amount == p.deposit_amount,
    },
    GateField {
        name: "book_checkin",
        guarded: false,
        matches: |ex, p| ex.book_checkin == p.book_checkin,
    },
    GateField {
        name: "book_checkout",
        guarded: false,
        matches: |ex, p| ex.book_checkout == p.book_checkout,
    },
    // Guarded — `COALESCE($9, legacy_cust_no)` write semantics: a
    // transient NULL never overwrites, so treating it as a mismatch
    // would re-emit `BookingModified` every tick without converging. A
    // Some→Some move (including the `'C0000'` delete cascade) still
    // mismatches, which is what the reconcile hash needs.
    GateField {
        name: "legacy_cust_no",
        guarded: true,
        matches: |ex, p| p.legacy_cust_no.is_none() || ex.legacy_cust_no == p.legacy_cust_no,
    },
    // Guarded — `COALESCE($7, book_notes)`, same rationale.
    GateField {
        name: "book_notes",
        guarded: true,
        matches: |ex, p| p.notes.is_none() || ex.book_notes == p.notes,
    },
];

/// The second gate stage: the `ht_booking_rooms` SET comparison.
///
/// Modelled as ONE named term (`rooms`) covering both the room identity
/// and its per-room price, because that is the granularity the stage
/// actually decides at. Shipped 2026-07-28 after a bare room COUNT let
/// iHOTEL's constant-cardinality 402→403 swap through the gate.
const ROOM_SET_GATE_FIELD: GateField<[ExistingBookingRoom], [ResolvedRoomLine]> = GateField {
    name: "rooms",
    guarded: false,
    matches: |existing, resolved| {
        use std::collections::HashMap;

        // Intended junction state = resolved lines deduped by room_id,
        // first-wins (mirrors ON CONFLICT DO NOTHING).
        let mut intended: HashMap<i32, Option<f64>> = HashMap::with_capacity(resolved.len());
        for r in resolved {
            intended.entry(r.room_id).or_insert(r.price_per_night);
        }

        // `uq_ht_br_bookroom UNIQUE (br_book_id, br_room_id)` guarantees
        // the stored side is already unique per room, so a length check
        // on the deduped map is exact.
        if existing.len() != intended.len() {
            return false;
        }
        existing.iter().all(|ex| match intended.get(&ex.room_id) {
            Some(price) => prices_match(ex.price_per_night, *price),
            None => false,
        })
    },
};

/// Gate term names (both stages), for
/// [`crate::sync::gate_guard::reconcile_entity_contracts`].
pub(crate) fn gate_field_names() -> Vec<&'static str> {
    let mut names = gate_guard::gate_field_names(&HEADER_GATE_FIELDS);
    names.push(ROOM_SET_GATE_FIELD.name);
    names
}

/// The inputs `scheduler::sync::booking_canonical_hash` consumes, as a
/// descriptor table over the SAME projection the gate compares.
///
/// Order IS the hash-body order; byte parity is pinned by
/// `bookings_hash_bytes_unchanged_for_golden_inputs`.
///
/// `book_status` is deliberately absent — legacy `View_Booking_Ds.
/// Book_Status` is an integer ledger code while canonical
/// `ht_bookings.book_status` is a translated literal, so it is not a
/// hash input (the gate compares it anyway; the invariant only requires
/// gate ⊇ hash).
const HASH_INPUTS: [HashInput<CanonicalProjection>; 4] = [
    HashInput {
        name: "legacy_book_id",
        // Row identity: `fetch_existing` SELECTs `WHERE legacy_book_id =
        // $1`.
        gated_by: &[],
        segmented: true,
        lookup_key: true,
        segment: |p| p.legacy_book_id.clone(),
        mutate: |p| p.legacy_book_id = "R999999".into(),
    },
    HashInput {
        name: "book_checkin",
        gated_by: &["book_checkin"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.book_checkin.to_string(),
        mutate: |p| {
            p.book_checkin = p
                .book_checkin
                .succ_opt()
                .expect("fixture date has a successor")
        },
    },
    HashInput {
        name: "book_checkout",
        gated_by: &["book_checkout"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.book_checkout.to_string(),
        mutate: |p| {
            p.book_checkout = p
                .book_checkout
                .succ_opt()
                .expect("fixture date has a successor")
        },
    },
    HashInput {
        name: "legacy_cust_no",
        gated_by: &["legacy_cust_no"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.legacy_cust_no.clone().unwrap_or_default(),
        // Some→Some, mirroring iHOTEL's customer-delete cascade
        // (cheatsheet §3.24). The gate term is guarded, so a Some→None
        // mutation would NOT defeat it — and must not, since the
        // COALESCE write could never converge that transition.
        mutate: |p| p.legacy_cust_no = Some("C0000".into()),
    },
];

/// Name-level hash contract, for
/// [`crate::sync::gate_guard::reconcile_entity_contracts`].
pub(crate) fn hash_input_contract() -> Vec<HashInputContract> {
    gate_guard::hash_input_contracts(&HASH_INPUTS)
}

/// Render the `bookings` reconcile-hash body from [`HASH_INPUTS`].
/// Test-only — see the customer mapper's equivalent for why.
#[cfg(test)]
fn hash_body(p: &CanonicalProjection) -> String {
    gate_guard::hash_body(&HASH_INPUTS, p)
}

/// True when `ht_booking_rooms` already holds exactly what
/// [`replace_rooms`] would write. Ports the check-in mapper's proven
/// `rooms_match` pattern (`sync::mappers::checkin::rooms_match`, Track
/// B2 / T2 HIGH-2) from `(room, status)` pairs to `(room, price)` pairs:
/// compares the SET, not the sequence, and not a count.
///
/// ## Why the SET (2026-07-28)
///
/// iHOTEL's `FrmAddBook2.SAVE_EDIT` is field-agnostic — it DELETEs and
/// re-INSERTs all four booking tables on ANY edit. A receptionist
/// swapping room 402→403 therefore re-writes a byte-identical header at
/// an unchanged room count, and the old `rooms_count == count` term
/// held, short-circuiting `apply_booking_aggregate` to `Ok(None)` before
/// `replace_rooms` ever ran. Canonical kept the stale room forever, and
/// the reconcile sweep was blind to it too (`booking_canonical_hash`
/// carries no room data). Order-insensitivity matters because neither
/// SELECT is ordered and iHOTEL re-inserts in edit-dialog order.
///
/// ## Unresolvable lines (intent preserved from the 2026-06-11 fix)
///
/// Both sides of this comparison are post-resolution, so an unresolvable
/// line is invisible to BOTH: `resolve_room_lines` drops blank-`room_no`
/// lines (observed on cancelled iHOTEL lines, e.g. R014826) before we
/// ever get here, and `project_aggregate` projects `Book_room_type=1`
/// (room-TYPE-code) bookings as header-only. A non-blank room that
/// misses `ht_rooms_new` never reaches this function at all —
/// `resolve_room_lines` errors and the watcher holds the watermark. So
/// the gate compares "what will be written" against "what is stored",
/// never "what legacy sent" — which is precisely what stopped the
/// forever-re-emitting `BookingModified` loop, and it stays true term
/// for term now that the comparison is content-aware.
///
/// ## Duplicate room lines
///
/// `replace_rooms` INSERTs with `ON CONFLICT (br_book_id, br_room_id) DO
/// NOTHING`, so two resolved lines for the same room collapse to ONE
/// junction row carrying the FIRST line's price. The fold below keeps
/// the first occurrence per `room_id` for the same reason: a comparison
/// that counted the duplicate would never converge (2 intended vs 1
/// stored), reintroducing the exact loop this design avoids.
///
/// The comparison itself lives in [`ROOM_SET_GATE_FIELD`] so the gate
/// stage carries a name the contract registry can see; this stays as the
/// call site + documentation anchor.
fn booking_rooms_match(existing: &[ExistingBookingRoom], resolved: &[ResolvedRoomLine]) -> bool {
    (ROOM_SET_GATE_FIELD.matches)(existing, resolved)
}

/// Compare two per-room prices at the resolution
/// `ht_booking_rooms.br_price_per_night` can actually store
/// (`DECIMAL(10,2)`).
///
/// Exact `f64` equality is wrong here: legacy `HT_Book_Ds.
/// Book_Room_Price` is a SQL Server `float`, so a value with sub-satang
/// precision is ROUNDED on the way into the column and can never read
/// back equal to the projection. That would leave the gate permanently
/// false and re-emit `BookingModified` on every CT touch — the same
/// non-convergence failure mode the 2026-06-11 resolvable-count fix
/// removed. Two prices are therefore "the same" iff they land on the
/// same stored value: less than half a satang apart, with a hair of
/// slack for the float noise in that boundary.
///
/// NULL is NOT a value: `replace_rooms` binds `None` as SQL NULL, so
/// `NULL` vs `0.00` is a real difference and must re-apply.
fn prices_match(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => (a - b).abs() < PRICE_MATCH_EPSILON,
        _ => false,
    }
}

/// Half of the `DECIMAL(10,2)` storage resolution (0.005) plus float
/// slack. Any genuine 1-satang change (0.01) is still a mismatch.
const PRICE_MATCH_EPSILON: f64 = 0.005_000_1;

async fn update_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    book_id_serial: i32,
    cust_id: i32,
    p: &CanonicalProjection,
    agg_id: Uuid,
) -> Result<(), SyncError> {
    // COALESCE argument order mirrors `sync::mappers::checkin::update_existing`:
    //   * `legacy_book_id` (PK) and `aggregate_id` (write-once UUID) keep
    //     `COALESCE(existing, new)` — set once, never overwritten.
    //   * `legacy_cust_no` is a denormalised pointer that MUST track the
    //     current MSSQL state. Pre-fix it used `COALESCE(existing, new)`,
    //     freezing on first apply: if a receptionist changed the customer
    //     associated with a booking (legacy `HT_Book_H.Book_Cust_ID`
    //     rewrites in place), the canonical `book_cust_id` FK updated via
    //     `$1` but the denormalised legacy_cust_no stayed pinned to the
    //     original customer — same shape as the CH26-005540 room-change
    //     incident on the checkin mapper (Bug A). `COALESCE($9, legacy_cust_no)`
    //     restores write-through: a NULL `$9` (transient mid-edit) keeps
    //     the existing value; a non-NULL `$9` overwrites.
    sqlx::query(
        "UPDATE ht_bookings \
            SET book_cust_id        = $1, \
                book_checkin        = $2, \
                book_checkout       = $3, \
                book_status         = $4, \
                book_total_amount   = $5::float8, \
                book_deposit_amount = $6::float8, \
                book_notes          = COALESCE($7, book_notes), \
                legacy_book_id      = COALESCE(legacy_book_id, $8), \
                legacy_cust_no      = COALESCE($9, legacy_cust_no), \
                aggregate_id        = COALESCE(aggregate_id, $10), \
                updated_at          = NOW() \
          WHERE book_id = $11",
    )
    .bind(cust_id)
    .bind(p.book_checkin)
    .bind(p.book_checkout)
    .bind(&p.book_status)
    .bind(p.total_amount)
    .bind(p.deposit_amount)
    .bind(&p.notes)
    .bind(&p.legacy_book_id)
    .bind(&p.legacy_cust_no)
    .bind(agg_id)
    .bind(book_id_serial)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// Insert a new `ht_bookings` row mirroring the legacy aggregate.
/// `book_no` is NOT NULL UNIQUE in our schema; we reuse the legacy
/// `Book_ID` (`R\d{6}`) as the canonical `book_no` to keep the human
/// reference stable across both apps.
async fn insert_new(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cust_id: i32,
    p: &CanonicalProjection,
) -> Result<i32, SyncError> {
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO ht_bookings \
             (book_no, book_cust_id, book_checkin, book_checkout, \
              book_status, book_total_amount, book_deposit_amount, book_notes, \
              legacy_book_id, legacy_cust_no, book_source) \
         VALUES \
             ($1, $2, $3, $4, $5, $6::float8, $7::float8, $8, $9, $10, 'legacy_app') \
         RETURNING book_id",
    )
    .bind(&p.legacy_book_id)
    .bind(cust_id)
    .bind(p.book_checkin)
    .bind(p.book_checkout)
    .bind(&p.book_status)
    .bind(p.total_amount)
    .bind(p.deposit_amount)
    .bind(&p.notes)
    .bind(&p.legacy_book_id)
    .bind(&p.legacy_cust_no)
    .fetch_one(&mut **tx)
    .await?;
    Ok(row.0)
}

/// A projection room line whose `room_no` resolved to a canonical
/// `ht_rooms_new.room_id`. Produced by [`resolve_room_lines`], consumed
/// by [`replace_rooms`].
#[derive(Debug, Clone)]
struct ResolvedRoomLine {
    room_id: i32,
    price_per_night: Option<f64>,
}

/// Resolve every projection room line against `ht_rooms_new`. The
/// RESOLVED set is what both `existing_matches` (via
/// [`booking_rooms_match`]) and `replace_rooms` operate on, so the
/// idempotency comparison and the junction mutation can never disagree
/// (2026-06-11 fix for the forever-re-emitting `BookingModified` loop on
/// unresolvable lines; still the invariant now that the comparison is
/// content-aware rather than a count — 2026-07-28).
///
/// Miss handling (2026-06-12, audit follow-up — matches the checkin
/// mapper's posture):
/// - BLANK `room_no` → debug-skip. Observed in production on cancelled
///   lines (e.g. R014826 carries `Book_Room_Type=''`); a retry can never
///   learn more, and erroring would be a data-quality poison pill.
/// - NON-BLANK miss → error (watermark hold + retry). The room master
///   mapper auto-creates unknown rooms from their own `HT_Rooms` CT row
///   (polled earlier in the same tick), so a persistent miss means the
///   room genuinely doesn't exist in `HT_Rooms` — operator-page
///   territory, not silent-drop territory. Both sites verified clean of
///   orphan room references (2026-06-12).
async fn resolve_room_lines(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    book_id: &str,
    rooms: &[RoomLine],
) -> Result<Vec<ResolvedRoomLine>, SyncError> {
    let mut out = Vec::with_capacity(rooms.len());
    for r in rooms {
        if r.room_no.trim().is_empty() {
            tracing::debug!(
                book_id,
                "ht_booking_rooms line skipped: blank room_no (cancelled-line \
                 data shape); excluded from junction + idempotency count"
            );
            continue;
        }
        let room_id_row: Option<(i32,)> = sqlx::query_as(
            "SELECT room_id FROM ht_rooms_new WHERE room_no = $1 LIMIT 1",
        )
        .bind(&r.room_no)
        .fetch_optional(&mut **tx)
        .await?;
        match room_id_row {
            Some((room_id,)) => out.push(ResolvedRoomLine {
                room_id,
                price_per_night: r.price_per_night,
            }),
            None => {
                return Err(SyncError::Mapper {
                    table: BOOK_DS_TABLE,
                    message: format!(
                        "booking {book_id}: room '{}' not in ht_rooms_new — \
                         holding watermark for retry (room mapper auto-creates \
                         on the room's own CT row; persistent recurrence means \
                         the room is missing from HT_Rooms itself)",
                        r.room_no
                    ),
                });
            }
        }
    }
    Ok(out)
}

/// Replace `ht_booking_rooms` for this booking. Conservative: drop and
/// re-insert. The set is small (typically 1 row, rarely >5) so the
/// extra delete is cheaper than diffing.
async fn replace_rooms(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    book_id_serial: i32,
    rooms: &[ResolvedRoomLine],
) -> Result<(), SyncError> {
    sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id = $1")
        .bind(book_id_serial)
        .execute(&mut **tx)
        .await?;

    for r in rooms {
        // ON CONFLICT keeps us idempotent if a duplicate (book_id,
        // room_id) somehow slipped through (shouldn't, the DELETE
        // above empties the set first — but the unique index on
        // (br_book_id, br_room_id) would reject it otherwise).
        sqlx::query(
            "INSERT INTO ht_booking_rooms \
                 (br_book_id, br_room_id, br_price_per_night) \
             VALUES ($1, $2, $3::float8) \
             ON CONFLICT (br_book_id, br_room_id) DO NOTHING",
        )
        .bind(book_id_serial)
        .bind(r.room_id)
        .bind(r.price_per_night)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

/// Build the appropriate `BookingCreated` / `BookingModified` event.
fn build_event(
    was_insert: bool,
    agg_id: Uuid,
    cust_id: i32,
    p: &CanonicalProjection,
) -> DomainEvent {
    use crate::domain::booking::BookingState;
    use crate::domain::shared::Money;

    // Map our PG canonical literal back to the BookingState enum for
    // the event snapshot. Unknown → Pending.
    let state = match p.book_status.as_str() {
        "confirmed" => BookingState::Active,
        "checked_in" => BookingState::CheckedIn,
        "completed" => BookingState::Completed,
        "cancelled" => BookingState::Cancelled,
        _ => BookingState::Pending,
    };

    let snapshot = BookingSnapshot {
        id: agg_id,
        legacy_book_id: Some(p.legacy_book_id.clone()),
        customer_id: aggregate_uuid(AggregateKind::Customer, cust_id),
        state,
        stay_start: naive_date_to_utc(p.book_checkin),
        stay_end: naive_date_to_utc(p.book_checkout),
        room_no: p.rooms.first().map(|r| r.room_no.clone()),
        price: p
            .total_amount
            .map(|f| Money::from_satang((f * 100.0).round() as i64))
            .unwrap_or(Money::ZERO),
    };

    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };

    if was_insert {
        DomainEvent::BookingCreated {
            id: agg_id,
            source,
            snapshot,
        }
    } else {
        DomainEvent::BookingModified {
            id: agg_id,
            source,
            // Legacy CT doesn't expose a clean before-image; the after
            // snapshot carries the new state and subscribers re-fetch
            // for diffs (same pattern as the service-layer modify
            // path when the caller omits `before_snapshot`).
            before: snapshot.clone(),
            after: snapshot,
        }
    }
}

/// Convert a legacy stay-boundary date — Bangkok wall-clock midnight
/// stored without timezone info — to a real UTC instant for the
/// `DomainEvent` snapshot.
///
/// Fixed 2026-06-11 (audit P2 "timezone mislabels"): the previous
/// implementation stamped Bangkok midnight as UTC midnight, putting
/// every snapshot instant 7 hours in the future. Midnight Bangkok is
/// 17:00 UTC the previous day. `+07:00` is a fixed offset (no DST), so
/// `single()` always yields exactly one instant.
fn naive_date_to_utc(date: NaiveDate) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    let midnight = chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight");
    let bangkok = chrono::FixedOffset::east_opt(7 * 3600).expect("+07:00 is a valid offset");
    bangkok
        .from_local_datetime(&date.and_time(midnight))
        .single()
        .expect("fixed offsets have no DST gaps/folds")
        .with_timezone(&chrono::Utc)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    fn header_row(book_id: &str, cust_no: &str, status: &str) -> HashMapRow {
        HashMapRow::new(BOOK_H_TABLE)
            .with("Book_ID", MockValue::Str(book_id.into()))
            .with("Book_Cust_ID", MockValue::Str(cust_no.into()))
            .with("Book_Status", MockValue::Str(status.into()))
            .with(
                "Book_Date_in",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 25)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            )
            .with(
                "Book_Date_out",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            )
            .with("Book_Price_Total", MockValue::Decimal(890.0))
            .with("Book_Price_Pay", MockValue::Decimal(0.0))
            .with("Book_room_note", MockValue::Null)
    }

    fn ds_row(book_id: &str, room_no: &str) -> HashMapRow {
        HashMapRow::new(BOOK_DS_TABLE)
            .with("id", MockValue::I32(7000))
            .with("Book_No", MockValue::Str(book_id.into()))
            .with("Book_Room_Type", MockValue::Str(room_no.into()))
            .with("Book_Room_Price", MockValue::Decimal(890.0))
            .with("Book_status", MockValue::I32(1))
    }

    fn date_row(book_id: &str, room_no: &str) -> HashMapRow {
        HashMapRow::new(BOOK_DATE_TABLE)
            .with("id", MockValue::I32(47200))
            .with("Book_no", MockValue::Str(book_id.into()))
            .with("Book_type", MockValue::Str(room_no.into()))
            .with(
                "Book_date_ds",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 25)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            )
            .with("Book_USE", MockValue::I32(0))
            .with("Book_ok", MockValue::I32(0))
    }

    // ----- legacy_status_to_pg -------------------------------------------

    #[test]
    fn legacy_status_jong_maps_to_confirmed() {
        assert_eq!(legacy_status_to_pg("จอง"), "confirmed");
    }

    #[test]
    fn legacy_status_yokleek_maps_to_cancelled() {
        // 'ยกเลิก' is the user's standing-constraint cancelled literal.
        assert_eq!(legacy_status_to_pg("ยกเลิก"), "cancelled");
    }

    #[test]
    fn legacy_status_khaopak_maps_to_checked_in() {
        assert_eq!(legacy_status_to_pg("เข้าพัก"), "checked_in");
    }

    #[test]
    fn legacy_status_okleow_maps_to_completed() {
        assert_eq!(legacy_status_to_pg("ออกแล้ว"), "completed");
    }

    #[test]
    fn legacy_status_unknown_falls_back_to_pending() {
        assert_eq!(legacy_status_to_pg(""), "pending");
        assert_eq!(legacy_status_to_pg("???"), "pending");
    }

    // ----- project_aggregate ---------------------------------------------

    #[test]
    fn project_aggregate_extracts_dates_and_amounts() {
        let agg = BookingAggregate {
            header: Some(header_row("R014810", "C21610", "จอง")),
            rooms: vec![ds_row("R014810", "402")],
            nights: vec![date_row("R014810", "402")],
        };
        let p = project_aggregate(&agg, "R014810").expect("must project");
        assert_eq!(p.legacy_book_id, "R014810");
        assert_eq!(p.legacy_cust_no.as_deref(), Some("C21610"));
        assert_eq!(p.book_status, "confirmed");
        assert_eq!(
            p.book_checkin,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()
        );
        assert_eq!(
            p.book_checkout,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap()
        );
        assert_eq!(p.total_amount, Some(890.0));
        assert_eq!(p.deposit_amount, Some(0.0));
        assert_eq!(p.rooms.len(), 1);
        assert_eq!(p.rooms[0].room_no, "402");
        assert_eq!(p.rooms[0].price_per_night, Some(890.0));
    }

    #[test]
    fn project_aggregate_skips_cancelled_ds_rows() {
        let mut cancelled = ds_row("R014810", "402");
        cancelled.cells.insert("Book_status".into(), MockValue::I32(3));
        let agg = BookingAggregate {
            header: Some(header_row("R014810", "C21610", "จอง")),
            rooms: vec![cancelled, ds_row("R014810", "414")],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R014810").unwrap();
        assert_eq!(p.rooms.len(), 1);
        assert_eq!(p.rooms[0].room_no, "414");
    }

    #[test]
    fn project_aggregate_errors_when_dates_missing() {
        let mut header = header_row("R014810", "C21610", "จอง");
        header.cells.insert("Book_Date_in".into(), MockValue::Null);
        let agg = BookingAggregate {
            header: Some(header),
            rooms: vec![],
            nights: vec![],
        };
        let err = project_aggregate(&agg, "R014810").expect_err("missing date must error");
        assert!(err.to_string().contains("Book_Date_in"));
    }

    #[test]
    fn project_aggregate_drops_empty_notes() {
        let mut header = header_row("R014810", "C21610", "จอง");
        header
            .cells
            .insert("Book_room_note".into(), MockValue::Str(String::new()));
        let agg = BookingAggregate {
            header: Some(header),
            rooms: vec![],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R014810").unwrap();
        assert!(p.notes.is_none());
    }

    // ----- Book_room_type=1 (room-TYPE-code Ds lines, cheatsheet §3.3) ----

    /// `Book_room_type=1` means the Ds lines carry a room-TYPE code in
    /// `Book_Room_Type` (e.g. a rate-category id), NOT a room number.
    /// Pre-2026-06-11 those lines were projected as room assignments,
    /// failed the `ht_rooms_new.room_no` lookup, warn-skipped, and the
    /// `rooms_count` mismatch re-emitted `BookingModified` on every CT
    /// touch forever. Type-1 must project header-only.
    #[test]
    fn project_aggregate_type1_booking_projects_no_room_assignments() {
        let header = header_row("R015301", "C21610", "จอง")
            .with("Book_room_type", MockValue::I32(1));
        let agg = BookingAggregate {
            header: Some(header),
            // Ds line carries a TYPE code ("4" = some room category),
            // not a room number.
            rooms: vec![ds_row("R015301", "4")],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R015301").unwrap();
        assert!(
            p.rooms.is_empty(),
            "type-1 Ds lines are room-TYPE codes and must not become \
             room assignments"
        );
        // Header fields still mirror normally.
        assert_eq!(p.book_status, "confirmed");
        assert_eq!(p.total_amount, Some(890.0));
    }

    /// `Book_room_type=2` (specific rooms) keeps the historical
    /// room-number interpretation.
    #[test]
    fn project_aggregate_type2_booking_projects_room_assignments() {
        let header = header_row("R015302", "C21610", "จอง")
            .with("Book_room_type", MockValue::I32(2));
        let agg = BookingAggregate {
            header: Some(header),
            rooms: vec![ds_row("R015302", "402")],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R015302").unwrap();
        assert_eq!(p.rooms.len(), 1);
        assert_eq!(p.rooms[0].room_no, "402");
    }

    /// Missing `Book_room_type` (sparse fixture / pre-widening load)
    /// defaults to the historical room-number path so existing
    /// aggregates keep projecting identically.
    #[test]
    fn project_aggregate_missing_book_room_type_defaults_to_room_numbers() {
        let agg = BookingAggregate {
            header: Some(header_row("R015303", "C21610", "จอง")),
            rooms: vec![ds_row("R015303", "402")],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R015303").unwrap();
        assert_eq!(p.rooms.len(), 1);
    }

    // ----- header-only / walk-in-shaped projection ------------------------
    //
    // Regression guard for the 2026-05-18 "18 stuck check-ins" PROD-CRIT:
    // iHOTEL leaves several legitimate bookings as a header-only aggregate
    // (HT_Book_H present, zero matching HT_Book_Ds) — most notably the
    // ClickBook cancel-on-room path (cheatsheet §3.6) and the
    // FrmAddBook2.SAVE_EDIT delete-then-reinsert pattern (cheatsheet §3.7).
    // `project_aggregate` must produce a `CanonicalProjection` with
    // `rooms: vec![]` (not an error, not a drop) so the caller can persist
    // the canonical booking row and unblock downstream `Cin_Book_no` FK
    // resolution. See `apply_booking_aggregate` doc comment for the full
    // context.

    #[test]
    fn project_aggregate_header_only_yields_empty_rooms_not_error() {
        let agg = BookingAggregate {
            header: Some(header_row("R001329", "C21610", "จอง")),
            rooms: vec![],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R001329").expect("header-only must project");
        assert!(p.rooms.is_empty(), "header-only aggregate yields zero room lines");
        assert_eq!(p.legacy_book_id, "R001329");
        assert_eq!(p.book_status, "confirmed");
    }

    #[test]
    fn project_aggregate_header_only_with_all_ds_cancelled_yields_empty_rooms() {
        // Mirrors §3.5 FrmShowBookNotify cancel: Book_status=3 on every
        // line. Projection drops them — same canonical shape as a true
        // header-only aggregate.
        let mut cancelled1 = ds_row("R001388", "402");
        cancelled1.cells.insert("Book_status".into(), MockValue::I32(3));
        let mut cancelled2 = ds_row("R001388", "414");
        cancelled2.cells.insert("Book_status".into(), MockValue::I32(3));
        let agg = BookingAggregate {
            header: Some(header_row("R001388", "C21611", "ยกเลิก")),
            rooms: vec![cancelled1, cancelled2],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R001388").unwrap();
        assert!(p.rooms.is_empty());
        assert_eq!(p.book_status, "cancelled");
    }

    #[test]
    fn project_aggregate_header_only_preserves_dates_and_amounts() {
        // The booking is still a real booking — every header field
        // (dates, amounts, customer pointer) must survive even when no
        // Ds line exists. This guards against a "treat header-only as
        // partial" regression.
        let agg = BookingAggregate {
            header: Some(header_row("R001633", "C99001", "จอง")),
            rooms: vec![],
            nights: vec![],
        };
        let p = project_aggregate(&agg, "R001633").unwrap();
        assert_eq!(
            p.book_checkin,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()
        );
        assert_eq!(
            p.book_checkout,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap()
        );
        assert_eq!(p.total_amount, Some(890.0));
        assert_eq!(p.legacy_cust_no.as_deref(), Some("C99001"));
    }

    #[test]
    fn build_event_emits_booking_created_when_no_rooms() {
        // Header-only canonical projection → BookingCreated with no
        // `room_no` in the snapshot. The event still carries the legacy
        // book_id so downstream subscribers can correlate.
        let mut p = sample_projection();
        p.rooms = vec![];
        let agg = aggregate_uuid(AggregateKind::Booking, 42);
        let ev = build_event(true, agg, 100, &p);
        assert_eq!(ev.type_name(), "BookingCreated");
        let json = serde_json::to_value(&ev).unwrap();
        assert!(json["data"]["snapshot"]["room_no"].is_null());
        assert_eq!(
            json["data"]["snapshot"]["legacy_book_id"],
            serde_json::Value::String("R014810".into())
        );
    }

    // ----- existing_matches ---------------------------------------------

    fn sample_projection() -> CanonicalProjection {
        CanonicalProjection {
            legacy_book_id: "R014810".into(),
            legacy_cust_no: Some("C21610".into()),
            book_status: "confirmed".into(),
            book_checkin: chrono::NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            book_checkout: chrono::NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            total_amount: Some(890.0),
            deposit_amount: Some(0.0),
            notes: None,
            rooms: vec![],
        }
    }

    // Canonical `ht_rooms_new.room_id`s for the fixture rooms. Room
    // numbers are the human handle in iHOTEL; the junction (and so the
    // gate) keys on the resolved FK — see `ExistingBookingRoom`.
    const ROOM_402: i32 = 4002;
    const ROOM_403: i32 = 4003;
    const ROOM_414: i32 = 4014;

    fn resolved(room_id: i32, price: f64) -> ResolvedRoomLine {
        ResolvedRoomLine {
            room_id,
            price_per_night: Some(price),
        }
    }

    /// `ht_booking_rooms` content mirroring the given resolved lines —
    /// i.e. what `replace_rooms` would have left behind.
    fn stored(resolved: &[ResolvedRoomLine]) -> Vec<ExistingBookingRoom> {
        resolved
            .iter()
            .map(|r| ExistingBookingRoom {
                room_id: r.room_id,
                price_per_night: r.price_per_night,
            })
            .collect()
    }

    /// Existing canonical row that exactly mirrors `p` — tests mutate
    /// one field at a time. `p.rooms` is the pre-resolution projection,
    /// so the junction starts empty; room-aware tests set `ex.rooms`
    /// explicitly via [`stored`].
    fn make_existing(p: &CanonicalProjection) -> ExistingBooking {
        ExistingBooking {
            book_id_serial: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            book_status: Some(p.book_status.clone()),
            book_cust_id: 100,
            book_total_amount: p.total_amount,
            book_deposit_amount: p.deposit_amount,
            book_checkin: p.book_checkin,
            book_checkout: p.book_checkout,
            rooms: Vec::new(),
            legacy_cust_no: p.legacy_cust_no.clone(),
            book_notes: p.notes.clone(),
        }
    }

    #[test]
    fn existing_matches_returns_true_for_unchanged_row() {
        let p = sample_projection();
        let ex = make_existing(&p);
        assert!(existing_matches(&ex, &p, &[]));
    }

    #[test]
    fn existing_matches_returns_false_when_status_differs() {
        let p = sample_projection();
        let mut ex = make_existing(&p);
        ex.book_status = Some("cancelled".into());
        assert!(!existing_matches(&ex, &p, &[]));
    }

    #[test]
    fn existing_matches_returns_false_when_total_differs() {
        let p = sample_projection();
        let mut ex = make_existing(&p);
        ex.book_total_amount = Some(900.0);
        assert!(!existing_matches(&ex, &p, &[]));
    }

    #[test]
    fn existing_matches_returns_false_when_rooms_count_differs() {
        let p = sample_projection();
        let ex = make_existing(&p); // zero junction rows
        assert!(!existing_matches(&ex, &p, &[resolved(ROOM_402, 890.0)]));
    }

    /// Audit 2026-06-11 P1 #6 — iHOTEL's customer-delete cascade
    /// (`UPDATE HT_Book_H SET Book_Cust_ID='C0000'`, cheatsheet §3.24)
    /// changes ONLY the customer pointer. Pre-fix, `existing_matches`
    /// compared status/amounts/dates/rooms only, so the cascade was
    /// silently idempotency-skipped and the canonical booking kept
    /// referencing the deleted customer forever.
    #[test]
    fn existing_matches_returns_false_when_only_cust_no_changed() {
        let p = sample_projection(); // legacy_cust_no = Some("C21610")
        let mut ex = make_existing(&p);
        ex.legacy_cust_no = Some("C0000".into()); // canonical lags the cascade
        assert!(
            !existing_matches(&ex, &p, &[]),
            "a cust_no-only change MUST force a re-apply (C0000 cascade)"
        );
    }

    /// Notes-only edits must also force a re-apply…
    #[test]
    fn existing_matches_returns_false_when_only_notes_changed() {
        let mut p = sample_projection();
        p.notes = Some("late arrival".into());
        let mut ex = make_existing(&p);
        ex.book_notes = None;
        assert!(!existing_matches(&ex, &p, &[]));
    }

    /// …but a None-notes projection against a populated canonical value
    /// must NOT mismatch: `update_existing` writes notes through
    /// `COALESCE($7, book_notes)`, so the canonical value would never
    /// converge to NULL and the mismatch would re-emit BookingModified
    /// every tick forever. Same guard pattern for legacy_cust_no.
    #[test]
    fn existing_matches_guards_none_projection_against_populated_canonical() {
        let mut p = sample_projection();
        p.notes = None;
        p.legacy_cust_no = None;
        let mut ex = make_existing(&p);
        ex.book_notes = Some("kept".into());
        ex.legacy_cust_no = Some("C21610".into());
        assert!(
            existing_matches(&ex, &p, &[]),
            "None projection vs Some canonical must stay idempotent \
             (COALESCE write semantics can never converge it)"
        );
    }

    // ----- booking_rooms_match (2026-07-28 constant-count room swap) ------
    //
    // iHOTEL's FrmAddBook2.SAVE_EDIT is field-agnostic: it DELETEs and
    // re-INSERTs all four booking tables on ANY edit. Swapping a room
    // therefore re-writes a byte-identical header, so the ONLY signal
    // that anything changed lives in the per-room set. A count-based
    // gate saw none of it and canonical kept the old room permanently —
    // silent in the reconcile sweep too (`booking_canonical_hash`
    // carries no room data).

    /// THE BUG: room swapped 402→403 at a constant room count. Must NOT
    /// match, or `apply_booking_aggregate` short-circuits before
    /// `replace_rooms` and canonical keeps room 402 forever.
    #[test]
    fn booking_rooms_match_detects_pure_room_swap() {
        let before = [resolved(ROOM_402, 890.0)];
        let after = [resolved(ROOM_403, 890.0)];
        assert!(
            !booking_rooms_match(&stored(&before), &after),
            "402→403 at constant count MUST re-apply"
        );
    }

    /// Same rooms, different order — iHOTEL re-inserts in edit-dialog
    /// order and neither SELECT is ordered, so a sequence comparison
    /// would re-apply (and re-emit `BookingModified`) on every CT touch.
    #[test]
    fn booking_rooms_match_is_order_independent() {
        let stored_rows = stored(&[resolved(ROOM_402, 890.0), resolved(ROOM_414, 1200.0)]);
        let reordered = [resolved(ROOM_414, 1200.0), resolved(ROOM_402, 890.0)];
        assert!(
            booking_rooms_match(&stored_rows, &reordered),
            "the SET is what matters, not the sequence"
        );
    }

    /// A per-room price edit at an unchanged room set must re-apply —
    /// `br_price_per_night` is mirrored state, and the header total can
    /// stay put when one room's rate is corrected against another's.
    #[test]
    fn booking_rooms_match_detects_price_change_at_constant_rooms() {
        let before = [resolved(ROOM_402, 890.0)];
        let after = [resolved(ROOM_402, 950.0)];
        assert!(!booking_rooms_match(&stored(&before), &after));
    }

    /// A 1-satang change is still a change (guards the epsilon from
    /// being widened into a real-difference mask).
    #[test]
    fn booking_rooms_match_detects_one_satang_price_change() {
        let before = [resolved(ROOM_402, 890.00)];
        let after = [resolved(ROOM_402, 890.01)];
        assert!(!booking_rooms_match(&stored(&before), &after));
    }

    /// …but sub-satang float noise must NOT: legacy `Book_Room_Price` is
    /// a SQL Server `float` and our column is `DECIMAL(10,2)`, so the
    /// stored value is rounded. Exact `f64` equality would leave the
    /// gate permanently false and re-emit `BookingModified` on every CT
    /// touch — the non-convergence failure mode this design avoids.
    #[test]
    fn booking_rooms_match_ignores_sub_satang_rounding() {
        let stored_rows = vec![ExistingBookingRoom {
            room_id: ROOM_402,
            price_per_night: Some(890.33), // as DECIMAL(10,2) stored it
        }];
        let projected = [resolved(ROOM_402, 890.333_333_3)];
        assert!(
            booking_rooms_match(&stored_rows, &projected),
            "a difference the column cannot store is not a difference"
        );
    }

    /// NULL is not 0.00 — `replace_rooms` binds `None` as SQL NULL, so
    /// the transition is real and must re-apply.
    #[test]
    fn booking_rooms_match_treats_null_price_as_distinct_from_zero() {
        let stored_rows = vec![ExistingBookingRoom {
            room_id: ROOM_402,
            price_per_night: None,
        }];
        let projected = [resolved(ROOM_402, 0.0)];
        assert!(!booking_rooms_match(&stored_rows, &projected));
    }

    /// The N→0 case the count-based gate was originally added for
    /// (iHOTEL §3.7 delete-then-reinsert / §3.6 cancel-on-room) must
    /// keep re-applying so `replace_rooms` drops the stale junction rows
    /// — regression pinned by
    /// `re_apply_with_zero_rooms_clears_stale_booking_rooms`.
    #[test]
    fn booking_rooms_match_detects_all_rooms_dropped() {
        let stored_rows = stored(&[resolved(ROOM_402, 890.0)]);
        assert!(!booking_rooms_match(&stored_rows, &[]));
    }

    /// Header-only booking on both sides — the steady state for
    /// `Book_room_type=1` bookings and post-cancel headers. Must match,
    /// or every CT touch on them re-emits forever.
    #[test]
    fn booking_rooms_match_true_when_both_sides_empty() {
        assert!(booking_rooms_match(&[], &[]));
    }

    /// UNRESOLVABLE LINES — intent preserved from the 2026-06-11 fix.
    ///
    /// `resolve_room_lines` drops blank-`room_no` lines (observed on
    /// cancelled iHOTEL lines, e.g. R014826) and `project_aggregate`
    /// drops `Book_room_type=1` room-TYPE codes, so they are absent from
    /// `resolved` — the ONLY room input the gate sees. They must
    /// therefore be invisible on both sides: the projection carries two
    /// lines, only one resolves, the junction holds that one, and the
    /// gate MUST match. Comparing against `p.rooms` instead would make
    /// the gate permanently false and re-apply on every tick — the exact
    /// failure mode the count-based design was avoiding.
    ///
    /// (A non-blank room missing from `ht_rooms_new` never reaches here:
    /// `resolve_room_lines` errors and the watcher holds the watermark.)
    #[test]
    fn existing_matches_ignores_unresolvable_projection_lines() {
        let mut p = sample_projection();
        p.rooms = vec![
            RoomLine {
                room_no: "402".into(),
                price_per_night: Some(890.0),
            },
            RoomLine {
                // Blank room_no — dropped by `resolve_room_lines`, so it
                // reaches neither the junction nor this comparison.
                room_no: String::new(),
                price_per_night: Some(890.0),
            },
        ];
        let resolved_rooms = [resolved(ROOM_402, 890.0)];
        let mut ex = make_existing(&p);
        ex.rooms = stored(&resolved_rooms);
        assert!(
            existing_matches(&ex, &p, &resolved_rooms),
            "unresolvable lines must be invisible to BOTH sides of the \
             gate — comparing against p.rooms could never converge"
        );
    }

    /// Duplicate resolved lines for one room collapse to a single
    /// junction row via `ON CONFLICT (br_book_id, br_room_id) DO
    /// NOTHING`, so the gate must dedupe the same way — otherwise
    /// 2-intended vs 1-stored never converges.
    #[test]
    fn booking_rooms_match_dedupes_duplicate_room_lines_like_replace_rooms() {
        let stored_rows = stored(&[resolved(ROOM_402, 890.0)]);
        let duplicated = [resolved(ROOM_402, 890.0), resolved(ROOM_402, 890.0)];
        assert!(booking_rooms_match(&stored_rows, &duplicated));
    }

    /// End-to-end at the gate level: header byte-identical, room swapped
    /// — `existing_matches` must be false so the caller falls through to
    /// `replace_rooms`.
    #[test]
    fn existing_matches_returns_false_when_only_room_swapped() {
        let p = sample_projection();
        let mut ex = make_existing(&p);
        ex.rooms = stored(&[resolved(ROOM_402, 890.0)]);
        assert!(
            !existing_matches(&ex, &p, &[resolved(ROOM_403, 890.0)]),
            "a room-only SAVE_EDIT MUST force a re-apply"
        );
    }

    /// …and the same gate with a per-room price edit only.
    #[test]
    fn existing_matches_returns_false_when_only_room_price_changed() {
        let p = sample_projection();
        let mut ex = make_existing(&p);
        ex.rooms = stored(&[resolved(ROOM_402, 890.0)]);
        assert!(!existing_matches(&ex, &p, &[resolved(ROOM_402, 950.0)]));
    }

    // ----- gate ⊇ reconcile-hash (see `crate::sync::gate_guard`) ---------

    /// Behavioural half of the gate/hash invariant, for bookings.
    ///
    /// Executes the PRODUCTION gate against a genuinely mutated
    /// projection, so it cannot be satisfied by editing a list. Each
    /// mutator must also move its own hashed segment, which is what
    /// stops a no-op mutator from faking a pass.
    #[test]
    fn bookings_hash_mutations_all_defeat_the_idempotency_gate() {
        let base = sample_projection();
        let ex = make_existing(&base);
        assert!(
            existing_matches(&ex, &base, &[]),
            "fixture must start converged, else the test proves nothing"
        );

        for input in HASH_INPUTS.iter() {
            if input.lookup_key {
                // Identity — `fetch_existing` resolves BY it.
                continue;
            }
            let before = (input.segment)(&base);
            let mut mutated = base.clone();
            (input.mutate)(&mut mutated);
            let after = (input.segment)(&mutated);
            assert_ne!(
                before, after,
                "hash input `{}`: mutator did not move the hashed segment",
                input.name,
            );
            assert!(
                !existing_matches(&ex, &mutated, &[]),
                "GATE/HASH INVARIANT VIOLATED — bookings: a legacy edit that \
                 moves reconcile-hash input `{}` is idempotency-SKIPPED. The CT \
                 delta ages out inside the 2-day retention window and the \
                 reconcile sweep flags a row it can never close \
                 (force_converge re-drives this same gate). Widen \
                 HEADER_GATE_FIELDS. Mechanism: d09e756.",
                input.name,
            );
        }
    }

    /// Byte-parity pin — see the customer mapper's equivalent for why a
    /// single byte of drift invalidates every stored hash.
    #[test]
    fn bookings_hash_bytes_unchanged_for_golden_inputs() {
        use crate::scheduler::sync::{booking_canonical_hash, sha256};

        let p = sample_projection();
        // Literal body under the format string this table replaced:
        //   format!("{}|{}|{}|{}", book_id, checkin, checkout, cust_no)
        //   with `.unwrap_or("")` per Option.
        let expected = sha256("R014810|2026-04-25|2026-04-26|C21610");

        assert_eq!(
            booking_canonical_hash(
                &p.legacy_book_id,
                Some(p.book_checkin.to_string()).as_deref(),
                Some(p.book_checkout.to_string()).as_deref(),
                p.legacy_cust_no.as_deref(),
            ),
            expected,
            "production booking hash changed bytes"
        );
        assert_eq!(
            sha256(&hash_body(&p)),
            expected,
            "HASH_INPUTS join no longer reproduces the production hash body"
        );
    }

    /// The gate table must expose exactly the terms the contract
    /// registry advertises — including the `rooms` set-comparison stage,
    /// which is a separate function and would otherwise be invisible to
    /// the name-level check.
    #[test]
    fn gate_field_names_include_the_room_set_stage() {
        let names = gate_field_names();
        assert!(names.contains(&"rooms"), "room stage missing: {names:?}");
        assert!(names.contains(&"book_checkin"));
        assert!(names.contains(&"book_checkout"));
        assert!(names.contains(&"legacy_cust_no"));
    }

    /// Documents the ONE booking hash input that rests on a guarded
    /// (Some-only) gate term, and its residual weakness.
    ///
    /// `legacy_cust_no` is written `COALESCE($9, legacy_cust_no)`, so a
    /// legacy `Book_Cust_ID` going NULL cannot converge and is
    /// deliberately not treated as a mismatch — while the reconcile hash
    /// WOULD move (`Some("C21610")` → `""`). Every observed iHOTEL edit
    /// is Some→Some (the `'C0000'` cascade included), so this is an
    /// accepted, recorded gap rather than an unnoticed one. Removing the
    /// guard would trade it for a permanent re-emit loop.
    #[test]
    fn guarded_gate_terms_are_recorded_with_their_residual_weakness() {
        use std::collections::HashSet;

        let guarded: HashSet<&str> = gate_guard::guarded_gate_field_names(&HEADER_GATE_FIELDS)
            .into_iter()
            .collect();
        assert_eq!(
            guarded,
            ["legacy_cust_no", "book_notes"]
                .into_iter()
                .collect::<HashSet<&str>>(),
        );

        let hash_inputs_on_guarded_terms: Vec<&str> = HASH_INPUTS
            .iter()
            .filter(|i| i.gated_by.iter().any(|n| guarded.contains(n)))
            .map(|i| i.name)
            .collect();
        assert_eq!(
            hash_inputs_on_guarded_terms,
            vec!["legacy_cust_no"],
            "a new hash input landed on a guarded gate term — decide \
             explicitly whether Some→None invisibility is acceptable for it"
        );
    }

    // ----- coalesce_key --------------------------------------------------

    #[test]
    fn header_mapper_coalesces_on_book_id() {
        let m = BookingHeaderMapper;
        let row = header_row("R014810", "C21610", "จอง");
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("R014810"));
    }

    #[test]
    fn ds_mapper_coalesces_on_book_no_when_present() {
        let m = BookingRoomsMapper;
        let row = ds_row("R014811", "402");
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("R014811"));
    }

    /// On a D row the joined `Book_No` is NULL — the mapper returns
    /// `None` and the watcher falls back to per-row dispatch (which
    /// itself returns Ok(None) for child mappers — a sibling CT row
    /// drives the actual aggregate sweep).
    #[test]
    fn ds_mapper_coalesce_returns_none_when_book_no_null() {
        let m = BookingRoomsMapper;
        let row = HashMapRow::new(BOOK_DS_TABLE)
            .with("id", MockValue::I32(7000))
            .with("Book_No", MockValue::Null);
        assert!(m.coalesce_key(&row).is_none());
    }

    #[test]
    fn date_mapper_coalesces_on_book_no_when_present() {
        let m = BookingDatesMapper;
        let row = date_row("R014812", "414");
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("R014812"));
    }

    /// The mapper trait surfaces 5.3-prep #4: customer/room mappers
    /// don't override `coalesce_key`, so they get the trait default
    /// `None` — the watcher routes them through the legacy 5.2 single-
    /// row dispatch.
    #[test]
    fn customer_mapper_coalesce_default_is_none() {
        use crate::sync::mappers::customer::CustomerMapper;
        let m = CustomerMapper;
        let row = HashMapRow::new("HT_Customers").with("Cust_no", MockValue::Str("C1".into()));
        assert!(m.coalesce_key(&row).is_none());
    }

    // ----- mapper metadata ----------------------------------------------

    #[test]
    fn header_mapper_metadata_is_correct() {
        let m = BookingHeaderMapper;
        assert_eq!(m.table(), "HT_Book_H");
        assert_eq!(m.primary_key_cols(), &["Book_ID"]);
        assert!(m.select_sql().contains("Book_Status"));
        assert!(m.select_sql().contains("Book_Cust_ID"));
    }

    #[test]
    fn rooms_mapper_metadata_is_correct() {
        let m = BookingRoomsMapper;
        assert_eq!(m.table(), "HT_Book_Ds");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("Book_No"));
        assert!(m.select_sql().contains("Book_Room_Type"));
    }

    #[test]
    fn dates_mapper_metadata_is_correct() {
        let m = BookingDatesMapper;
        assert_eq!(m.table(), "HT_Book_Date");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("Book_no"));
        assert!(m.select_sql().contains("Book_ok"));
    }

    // ----- build_event ---------------------------------------------------

    #[test]
    fn build_event_for_new_booking_emits_booking_created() {
        let p = sample_projection();
        let agg = aggregate_uuid(AggregateKind::Booking, 1);
        let ev = build_event(true, agg, 100, &p);
        assert_eq!(ev.type_name(), "BookingCreated");
        assert_eq!(ev.aggregate_id(), agg);
    }

    #[test]
    fn build_event_for_existing_booking_emits_booking_modified() {
        let p = sample_projection();
        let agg = aggregate_uuid(AggregateKind::Booking, 1);
        let ev = build_event(false, agg, 100, &p);
        assert_eq!(ev.type_name(), "BookingModified");
        assert_eq!(ev.aggregate_id(), agg);
    }

    #[test]
    fn build_event_carries_legacy_book_id_in_snapshot() {
        let p = sample_projection();
        let ev = build_event(true, uuid::Uuid::nil(), 100, &p);
        let json = serde_json::to_value(&ev).unwrap();
        assert_eq!(
            json["data"]["snapshot"]["legacy_book_id"],
            serde_json::Value::String("R014810".into())
        );
    }

    /// User constraint regression: legacy `Book_Status='ยกเลิก'` MUST
    /// translate to canonical `'cancelled'`, and the round-trip event
    /// must carry that state. Locks the verbatim mapping.
    #[test]
    fn cancelled_legacy_status_round_trips_to_cancelled_state() {
        let mut p = sample_projection();
        p.book_status = legacy_status_to_pg("ยกเลิก").into();
        let ev = build_event(false, uuid::Uuid::nil(), 100, &p);
        let json = serde_json::to_value(&ev).unwrap();
        assert_eq!(
            json["data"]["after"]["state"],
            serde_json::Value::String("cancelled".into())
        );
    }

    // ----- coalescing semantics ------------------------------------------

    /// Mirrors the watcher's coalescing pre-pass. Given a tick that
    /// surfaces 1 H + 2 Ds + 5 Date CT rows for the same booking,
    /// `coalesce_key` collected into a HashSet must yield exactly ONE
    /// unique key — guaranteeing a single `apply_booking_aggregate`
    /// call per tick per aggregate.
    #[test]
    fn coalescing_dedups_one_h_plus_two_ds_plus_five_date_to_single_apply() {
        let header = BookingHeaderMapper;
        let rooms = BookingRoomsMapper;
        let dates = BookingDatesMapper;

        let h_row = header_row("R014810", "C21610", "จอง");
        let ds_row1 = ds_row("R014810", "402");
        let ds_row2 = ds_row("R014810", "414");
        let date_rows: Vec<HashMapRow> =
            (0..5).map(|_| date_row("R014810", "402")).collect();

        let mut keys = std::collections::HashSet::new();
        if let Some(k) = header.coalesce_key(&h_row) {
            keys.insert(k);
        }
        for r in &[&ds_row1, &ds_row2] {
            if let Some(k) = rooms.coalesce_key(*r) {
                keys.insert(k);
            }
        }
        for r in &date_rows {
            if let Some(k) = dates.coalesce_key(r) {
                keys.insert(k);
            }
        }

        assert_eq!(keys.len(), 1, "all 8 rows belong to one aggregate");
        assert!(keys.contains("R014810"));
    }

    /// Two distinct bookings in the same tick must produce two unique
    /// keys — guaranteeing one apply per booking.
    #[test]
    fn coalescing_keeps_distinct_keys_for_two_bookings() {
        let header = BookingHeaderMapper;
        let rooms = BookingRoomsMapper;

        let h1 = header_row("R014810", "C1", "จอง");
        let h2 = header_row("R014811", "C2", "จอง");
        let ds1 = ds_row("R014810", "402");
        let ds2 = ds_row("R014811", "414");

        let mut keys = std::collections::HashSet::new();
        for r in &[&h1, &h2] {
            if let Some(k) = header.coalesce_key(*r) {
                keys.insert(k);
            }
        }
        for r in &[&ds1, &ds2] {
            if let Some(k) = rooms.coalesce_key(*r) {
                keys.insert(k);
            }
        }

        assert_eq!(keys.len(), 2);
        assert!(keys.contains("R014810"));
        assert!(keys.contains("R014811"));
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards.
    //
    // tiberius does not validate column names at compile time. A typo'd
    // `SELECT t.<col>` aborts the CT JOIN with `Invalid column name`,
    // and the watcher tolerates the error + advances watermark, silently
    // dropping every booking CT row until someone notices in prod (see
    // PR #90 H1 hotfix for the symmetric `HT_CheckIn_Ds` incident on
    // 2026-05-14). These three lock tests pin each booking projection
    // const against the authoritative HF Hotel schema dump.
    // -------------------------------------------------------------------

    #[test]
    fn book_h_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BOOK_H_SELECT_COLS, "HT_Book_H");
    }

    #[test]
    fn book_ds_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BOOK_DS_SELECT_COLS, "HT_Book_Ds");
    }

    #[test]
    fn book_date_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BOOK_DATE_SELECT_COLS, "HT_Book_Date");
    }
}
