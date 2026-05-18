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
//! The PG-side string is what the existing booking routes (`routes/new_bookings`)
//! and the writeback recipes already round-trip; we don't introduce a
//! new enum.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::outbox::event::{BookingSnapshot, DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::parent_loader::BookingAggregate;
use crate::sync::resolve;
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
    /// Current `ht_booking_rooms` row count for this booking. Needed by
    /// `existing_matches` so a header-unchanged + rooms-changed transition
    /// (notably N→0 from iHOTEL's §3.7 delete-then-reinsert or §3.6
    /// cancel-on-room flows) is NOT treated as idempotent. Without this
    /// the early-return skips `replace_rooms` and leaves stale junction
    /// rows behind — regression caught by
    /// `re_apply_with_zero_rooms_clears_stale_booking_rooms` in CI on
    /// 2026-05-18.
    rooms_count: i64,
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
/// Returns:
/// * `Ok(Some(DomainEvent))` when the canonical row genuinely changed
///   (or a new one was inserted, or the booking was cancelled).
/// * `Ok(None)` when the canonical row already mirrors the legacy
///   aggregate (idempotent skip).
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
    aggregate: &BookingAggregate,
    book_id: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    if !aggregate.is_present() {
        return apply_cancelled(tx, book_id).await;
    }

    let projection = project_aggregate(aggregate, book_id)?;
    let existing = fetch_existing(tx, book_id).await?;

    // Idempotent skip — every projected field matches the canonical row.
    if let Some(ex) = existing.as_ref() {
        if existing_matches(ex, &projection) && ex.aggregate_id.is_some() {
            return Ok(None);
        }
    }

    // Resolve the customer FK before the UPSERT — `ht_bookings.book_cust_id`
    // is `NOT NULL` and references `ht_customers(cust_id)`. Defer-on-
    // missing per `sync::resolve` contract.
    let cust_id = match resolve::resolve_customer_id(tx, projection.legacy_cust_no.as_deref())
        .await?
    {
        Some(id) => id,
        None => {
            // Customer hasn't been mirrored yet (the customer mapper
            // hasn't seen its own CT row, OR the booking references a
            // legacy customer never added to PG). Defer — the next
            // tick that brings the customer in will surface the
            // booking again via this same code path.
            return Ok(None);
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
    // It must be called even when `projection.rooms` is empty so that an
    // edit transitioning a booking from N-rooms → 0-rooms (a legitimate
    // mid-edit state per the §3.7 delete-then-reinsert pattern) drops
    // the stale junction rows.
    replace_rooms(tx, book_id_serial, &projection.rooms).await?;

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

    let mut rooms = Vec::with_capacity(agg.rooms.len());
    for r in &agg.rooms {
        // Skip cancelled lines (Book_status=3 per cheatsheet §3.4).
        let line_status = r.try_get_i32("Book_status").ok().flatten();
        if line_status == Some(3) {
            continue;
        }
        // Misleading column name — stores room NUMBER per spike §3b /
        // cheatsheet §3.4.
        let Some(room_no) = r.try_get_str("Book_Room_Type")?.map(str::to_string) else {
            continue;
        };
        let price_per_night = r.try_get_decimal("Book_Room_Price")?;
        rooms.push(RoomLine {
            room_no,
            price_per_night,
        });
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
    )>(
        "SELECT book_id, aggregate_id, book_status, book_cust_id, \
                book_total_amount::float8, book_deposit_amount::float8, \
                book_checkin, book_checkout \
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
    )) = row
    else {
        return Ok(None);
    };

    let rooms_count: i64 = sqlx::query_scalar(
        "SELECT count(*)::bigint FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_id_serial)
    .fetch_one(&mut **tx)
    .await?;

    Ok(Some(ExistingBooking {
        book_id_serial,
        aggregate_id,
        book_status,
        book_cust_id,
        book_total_amount,
        book_deposit_amount,
        book_checkin,
        book_checkout,
        rooms_count,
    }))
}

/// Compare the existing canonical row to the freshly projected legacy
/// one. Skip publication when every mirrored field matches.
fn existing_matches(ex: &ExistingBooking, p: &CanonicalProjection) -> bool {
    ex.book_status.as_deref() == Some(p.book_status.as_str())
        && ex.book_total_amount == p.total_amount
        && ex.book_deposit_amount == p.deposit_amount
        && ex.book_checkin == p.book_checkin
        && ex.book_checkout == p.book_checkout
        && ex.rooms_count == p.rooms.len() as i64
}

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

/// Replace `ht_booking_rooms` for this booking. Conservative: drop and
/// re-insert. The set is small (typically 1 row, rarely >5) so the
/// extra delete is cheaper than diffing.
async fn replace_rooms(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    book_id_serial: i32,
    rooms: &[RoomLine],
) -> Result<(), SyncError> {
    sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id = $1")
        .bind(book_id_serial)
        .execute(&mut **tx)
        .await?;

    for r in rooms {
        // Resolve room_id by room_no. Skip rows where the room hasn't
        // been backfilled — same defer pattern as the customer
        // resolver.
        let room_id_row: Option<(i32,)> = sqlx::query_as(
            "SELECT room_id FROM ht_rooms_new WHERE room_no = $1 LIMIT 1",
        )
        .bind(&r.room_no)
        .fetch_optional(&mut **tx)
        .await?;
        let Some((room_id,)) = room_id_row else {
            tracing::warn!(
                book_id = book_id_serial,
                room_no = %r.room_no,
                "ht_booking_rooms skipped: room_no not found in ht_rooms_new"
            );
            continue;
        };
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
        .bind(room_id)
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

fn naive_date_to_utc(date: NaiveDate) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    let midnight = chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight");
    chrono::Utc.from_utc_datetime(&date.and_time(midnight))
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

    #[test]
    fn existing_matches_returns_true_for_unchanged_row() {
        let p = sample_projection();
        let ex = ExistingBooking {
            book_id_serial: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            book_status: Some("confirmed".into()),
            book_cust_id: 100,
            book_total_amount: Some(890.0),
            book_deposit_amount: Some(0.0),
            book_checkin: p.book_checkin,
            book_checkout: p.book_checkout,
            rooms_count: p.rooms.len() as i64,
        };
        assert!(existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_returns_false_when_status_differs() {
        let p = sample_projection();
        let ex = ExistingBooking {
            book_id_serial: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            book_status: Some("cancelled".into()),
            book_cust_id: 100,
            book_total_amount: Some(890.0),
            book_deposit_amount: Some(0.0),
            book_checkin: p.book_checkin,
            book_checkout: p.book_checkout,
            rooms_count: p.rooms.len() as i64,
        };
        assert!(!existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_returns_false_when_total_differs() {
        let p = sample_projection();
        let ex = ExistingBooking {
            book_id_serial: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            book_status: Some("confirmed".into()),
            book_cust_id: 100,
            book_total_amount: Some(900.0),
            book_deposit_amount: Some(0.0),
            book_checkin: p.book_checkin,
            book_checkout: p.book_checkout,
            rooms_count: p.rooms.len() as i64,
        };
        assert!(!existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_returns_false_when_rooms_count_differs() {
        let p = sample_projection();
        let ex = ExistingBooking {
            book_id_serial: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            book_status: Some("confirmed".into()),
            book_cust_id: 100,
            book_total_amount: Some(890.0),
            book_deposit_amount: Some(0.0),
            book_checkin: p.book_checkin,
            book_checkout: p.book_checkout,
            rooms_count: (p.rooms.len() + 1) as i64,
        };
        assert!(!existing_matches(&ex, &p));
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
