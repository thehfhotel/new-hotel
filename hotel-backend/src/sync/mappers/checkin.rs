//! Check-in aggregate Change Tracking mapper (Phase 5.4).
//!
//! ## Aggregate composition
//!
//! Three legacy tables form the check-in aggregate:
//!
//! | MSSQL table       | Role                       | Cardinality              |
//! |-------------------|----------------------------|--------------------------|
//! | `HT_CheckIn_H`    | Header (folio)             | 1 per `Cin_no`           |
//! | `HT_CheckIn_Ds`   | Per-room detail line       | 1+ per `Cin_no`          |
//! | `HT_CheckIn_Pay`  | Payment ledger             | 0+ per `Cin_no`          |
//!
//! Per `docs/architecture.md` §3.6d, every CT row from any of the three
//! tables resolves the parent `Cin_no`, re-loads the full aggregate via
//! [`crate::sync::parent_loader::load_checkin_aggregate`], and runs an
//! idempotent UPSERT into `ht_checkins`. Exactly one `DomainEvent` per
//! aggregate per tick — coalesced by the watcher (see
//! [`MssqlChangeMapper::coalesce_key`]).
//!
//! ## Status literals (verbatim from the legacy app)
//!
//! Per the user's standing constraint, legacy literals are passed
//! through unchanged:
//!
//! | Source column                            | Literal      | Meaning                    |
//! |------------------------------------------|--------------|----------------------------|
//! | `HT_CheckIn_H.Cin_status`                | `'ปกติ'`     | Normal (active)            |
//! | `HT_CheckIn_H.Cin_status`                | `'ยกเลิก'`   | Cancelled                  |
//! | `HT_CheckIn_Ds.Cin_Room_Status`          | `'เข้าพัก'`  | Currently occupying        |
//! | `HT_CheckIn_Ds.Cin_Room_Status`          | `'Check-Out'`| Departed (English, hyphen) |
//! | `HT_CheckIn_Ds.Cin_Room_Status`          | `'จอง'`      | Reserved (booking-linked)  |
//! | `HT_CheckIn_Ds.Cin_Room_Status`          | `'ยกเลิก'`   | Cancelled                  |
//!
//! ## Checkout side-effect on the parent booking
//!
//! When *every* `HT_CheckIn_Ds` row carries `Cin_Room_Status='Check-Out'`,
//! the legacy app also flips `HT_Book_H.Book_Status='ออกแล้ว'` on the
//! parent booking. Our PG-side `book_status` mirrors that via the
//! booking mapper's `legacy_status_to_pg('ออกแล้ว')` → `'completed'`.
//! To pick the change up promptly (instead of waiting for the next CT
//! tick that surfaces `HT_Book_H`), the check-in mapper triggers a
//! one-shot re-projection of the parent booking after publishing the
//! `CheckOutCompleted` event. The call uses the same TX so the two
//! UPSERTs commit atomically.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::db::DbPool;
use crate::outbox::event::{CheckInSnapshot, DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::booking::apply_booking_aggregate;
use crate::sync::mappers::customer::{
    upsert_customer_from_row, EAGER_FETCH_COLUMNS as CUSTOMER_EAGER_FETCH_COLUMNS,
    TABLE as HT_CUSTOMERS,
};
use crate::sync::parent_loader::{load_booking_aggregate, CheckInAggregate};
use crate::sync::resolve;
use crate::sync::row::test_support::{HashMapRow, MockValue};
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

// =============================================================================
// Legacy literals — keep verbatim per the user's standing constraint.
// =============================================================================

/// `HT_CheckIn_H.Cin_status` cancelled-folio literal.
const CIN_STATUS_CANCELLED: &str = "ยกเลิก";
/// `HT_CheckIn_Ds.Cin_Room_Status` checked-out literal (English, HYPHEN
/// — distinct from `HT_Room_Status.room_status='Check Out'` which uses
/// a SPACE per writeback recipe checkout.rs §3e).
const CIN_ROOM_STATUS_CHECKED_OUT: &str = "Check-Out";

const HT_CHECKIN_H: &str = "HT_CheckIn_H";
const HT_CHECKIN_DS: &str = "HT_CheckIn_Ds";

// =============================================================================
// Mapper trait impls — register once per CT-enabled table. Each impl is
// thin: it just declares the table/PK/projection and surfaces the
// coalesce key. The actual UPSERT lives in `apply_checkin_aggregate`.
// =============================================================================

/// CT mapper for `HT_CheckIn_H` (header). Every check-in I/U/D writes
/// here, so this is the most-frequent driver of an aggregate re-sync.
pub struct CheckInHeaderMapper;

const CHECKIN_H_SELECT_COLS: &str =
    "t.Cin_no, t.Cin_status, t.Cin_Book_no, t.Cin_cust_no";

#[async_trait]
impl MssqlChangeMapper for CheckInHeaderMapper {
    fn table(&self) -> &'static str {
        HT_CHECKIN_H
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // CT's PK projection for HT_CheckIn_H is the varchar `Cin_no`.
        // Migration 017 seeds this verbatim.
        &["Cin_no"]
    }

    fn select_sql(&self) -> &'static str {
        CHECKIN_H_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        // Watcher coalesces this through `apply_checkin_aggregate`. See
        // BookingHeaderMapper::apply for the full rationale.
        Ok(None)
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        row.try_get_str("Cin_no").ok().flatten().map(str::to_string)
    }
}

/// CT mapper for `HT_CheckIn_Ds` (per-room detail). Edits to room
/// status (e.g. `'เข้าพัก'` → `'Check-Out'`) drive an aggregate
/// re-sync that recomputes the canonical `cin_status` + emits the
/// appropriate event.
pub struct CheckInRoomsMapper;

const CHECKIN_DS_SELECT_COLS: &str =
    "t.id, t.Cin_No, t.Cin_Room_No, t.Cin_Room_Status";

#[async_trait]
impl MssqlChangeMapper for CheckInRoomsMapper {
    fn table(&self) -> &'static str {
        HT_CHECKIN_DS
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // Schema dump 2026-04-26 confirmed `id` is the SERIAL PK; CT
        // keys on it. `Cin_No` (capital N) is the parent FK and is
        // projected via SELECT for I/U; for D rows it's NULL and the
        // aggregate sweep picks the parent up via a sibling header CT
        // row (cancel cascade always touches the header).
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        CHECKIN_DS_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        Ok(None) // see CheckInHeaderMapper::apply
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        // I/U rows: the joined `Cin_No` is present.
        // D rows: NULL → return None and let a sibling CT row drive the
        //         aggregate sweep.
        row.try_get_str("Cin_No").ok().flatten().map(str::to_string)
    }
}

// =============================================================================
// Shared aggregate-sync helper — called once per (cin_no, tick) by the
// watcher's coalescing layer.
// =============================================================================

/// PG-side projection of an existing canonical row. Used to detect
/// Created vs Modified vs Cancelled and to skip publish-on-no-change.
#[derive(Debug, Clone)]
struct ExistingCheckIn {
    cin_id: i32,
    aggregate_id: Option<Uuid>,
    cin_status: Option<String>,
    cin_total_amount: Option<f64>,
    cin_paid_amount: Option<f64>,
    cin_checkout_time: Option<NaiveDateTime>,
}

/// Canonical PG-shape projection of the legacy aggregate. This is what
/// we UPSERT.
#[derive(Debug, Clone, PartialEq)]
struct CanonicalCheckIn {
    legacy_cin_no: String,
    legacy_book_id: Option<String>,
    legacy_cust_no: Option<String>,
    legacy_room_no: Option<String>,
    /// Legacy literal verbatim (per user constraint) translated to our
    /// PG enum literal (`'active'`/`'checkedout'`/`'cancelled'`). The
    /// translation sits in [`legacy_status_to_pg`].
    cin_status: String,
    cin_checkin_time: NaiveDateTime,
    /// Set on checkout (the moment the last room flips to `'Check-Out'`).
    /// `None` for active stays.
    cin_checkout_time: Option<NaiveDateTime>,
    cin_expected_checkout: NaiveDate,
    total_amount: Option<f64>,
    paid_amount: Option<f64>,
    /// `HT_CheckIn_Ds.id` for the first room — kept so the writeback
    /// resolver has a fast path back into MSSQL even when the next CT
    /// tick deletes the parent header.
    legacy_checkin_ds_id: Option<i32>,
    /// Whether *every* `HT_CheckIn_Ds` row has `Cin_Room_Status =
    /// 'Check-Out'`. When true the mapper emits `CheckOutCompleted`
    /// AND triggers a one-shot re-projection of the parent booking
    /// (per the 5.4 prep #6 side-effect).
    is_fully_checked_out: bool,
}

/// Re-sync one check-in aggregate. Idempotent — safe to call multiple
/// times for the same `cin_no` per tick.
///
/// `mssql` is needed only for the optional checkout side-effect that
/// re-projects the parent booking. Pass `Some(&pool)` from the
/// watcher; pass `None` from tests / contexts that don't have legacy
/// access. The side-effect is best-effort — it logs and skips when
/// the pool is `None` or when the booking aggregate fails to load.
///
/// Returns:
/// * `Ok(Some(DomainEvent))` when the canonical row genuinely changed.
/// * `Ok(None)` when the canonical row already mirrors the legacy
///   aggregate (idempotent skip), OR when an FK resolver deferred the
///   apply (caller's `legacy_sync_status` counters reflect this as a
///   skipped row, not a failure).
pub async fn apply_checkin_aggregate(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    aggregate: &CheckInAggregate,
    cin_no: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    if !aggregate.is_present() {
        return apply_cancelled(tx, cin_no).await;
    }

    let projection = project_aggregate(aggregate, cin_no)?;
    let existing = fetch_existing(tx, cin_no).await?;

    // Idempotent skip — the canonical row already mirrors the legacy
    // aggregate.
    if let Some(ex) = existing.as_ref() {
        if existing_matches(ex, &projection) && ex.aggregate_id.is_some() {
            return Ok(None);
        }
    }

    // Short-circuit: header is still present but says cancelled AND
    // every `HT_CheckIn_Ds` row was deleted (so `legacy_room_no=None`,
    // `derive_room_state` line ~504-515). FK resolution below would
    // defer indefinitely on `resolve_room_id(None)`, permanently
    // stranding the canonical row in `cin_status='active'` while the
    // legacy is cancelled. Skip straight to the UPDATE on the existing
    // row — there's nothing else to mirror.
    //
    // Guard: only short-circuit when the canonical row already exists.
    // If it doesn't, the cancellation is legitimately deferred (we need
    // to wait for the original INSERT CT row to land first).
    if projection.cin_status == "cancelled" {
        if let Some(ex) = existing.as_ref() {
            return apply_cancelled_for_present_header(tx, ex, cin_no).await;
        }
    }

    // Resolve FKs. Defer-on-missing per `sync::resolve` contract — with
    // one targeted exception for customers (see
    // `resolve_customer_or_eager_mirror` rationale): a deferred customer
    // FK previously stranded the checkin row forever because the
    // watermark advanced past the deferred row and never re-read it.
    // The eager-mirror path pulls the matching `HT_Customers` row from
    // MSSQL synchronously, INSERTs it into canonical `ht_customers` in
    // the same TX, then retries the FK lookup.
    let cust_id =
        match resolve_customer_or_eager_mirror(tx, mssql, projection.legacy_cust_no.as_deref())
            .await?
        {
            Some(id) => id,
            None => return Ok(None),
        };
    let room_id = match resolve::resolve_room_id(tx, projection.legacy_room_no.as_deref())
        .await?
    {
        Some(id) => id,
        None => {
            tracing::warn!(
                cin_no,
                legacy_room_no = ?projection.legacy_room_no,
                "ht_checkins apply deferred: room not yet mirrored \
                 (run bin/backfill_rooms?)"
            );
            return Ok(None);
        }
    };
    // Booking is OPTIONAL — walk-ins write `Cin_Book_no=''`. Only defer
    // when the legacy carries a non-empty `Cin_Book_no` AND the parent
    // booking row hasn't landed yet.
    //
    // Track E1 / T2 MED-4 (audit 2026-05-13) — the walk-in short-circuit
    // matches BOTH `Some("")` (legitimate walk-in) AND `None` (the
    // legacy column was NULL OR a parse failure upstream set
    // `legacy_book_id=None`). The two are operationally distinct:
    // a walk-in is normal flow; a parse failure is a sync-quality
    // signal worth investigating. The debug log below lets operators
    // distinguish the two cases in production trace output.
    let book_id_opt = match projection.legacy_book_id.as_deref() {
        Some(id) if !id.is_empty() => {
            match resolve::resolve_booking_id(tx, Some(id)).await? {
                Some(found) => Some(found),
                None => {
                    tracing::warn!(
                        cin_no,
                        legacy_book_id = id,
                        "ht_checkins apply deferred: parent booking not yet mirrored"
                    );
                    return Ok(None);
                }
            }
        }
        other => {
            tracing::debug!(
                target: "sync::checkin",
                cin_no,
                legacy_book_id = ?other,
                "walk-in short-circuit (no parent booking lookup): \
                 distinguishes Some(\"\") = legitimate walk-in vs None = \
                 NULL/parse-failure on Cin_Book_no"
            );
            None
        }
    };

    let (cin_id_serial, agg_id, was_insert) = match existing {
        Some(ex) => {
            let agg_id = ex
                .aggregate_id
                .unwrap_or_else(|| aggregate_uuid(AggregateKind::CheckIn, ex.cin_id));
            update_existing(tx, ex.cin_id, cust_id, room_id, book_id_opt, &projection, agg_id)
                .await?;
            (ex.cin_id, agg_id, false)
        }
        None => {
            let new_id = insert_new(tx, cust_id, room_id, book_id_opt, &projection).await?;
            let agg_id = aggregate_uuid(AggregateKind::CheckIn, new_id);
            sqlx::query("UPDATE ht_checkins SET aggregate_id = $1 WHERE cin_id = $2")
                .bind(agg_id)
                .bind(new_id)
                .execute(&mut **tx)
                .await?;
            (new_id, agg_id, true)
        }
    };

    let event = build_event(
        was_insert,
        cust_id,
        room_id,
        book_id_opt,
        agg_id,
        &projection,
    );

    // Side-effect: when this transition completes a check-out, also
    // re-project the parent booking so its `Book_Status='ออกแล้ว'`
    // syncs into `ht_bookings.book_status='completed'` in the same TX.
    // No-op for walk-ins (`legacy_book_id` empty/None) and for
    // contexts without legacy access (`mssql=None`).
    if projection.is_fully_checked_out {
        if let (Some(book_id), Some(pool)) = (projection.legacy_book_id.as_deref(), mssql) {
            if !book_id.is_empty() {
                reproject_parent_booking(tx, pool, book_id).await?;
            }
        }
    }

    let _ = cin_id_serial; // kept for readability; aggregate_id is the public handle.
    Ok(Some(event))
}

/// Cancellation short-circuit for the case where the legacy header
/// is still present (`Cin_status='ยกเลิก'`) but every `HT_CheckIn_Ds`
/// row has been deleted. In that state `derive_room_state` returns
/// `first_room_no=None`, so the normal `apply_checkin_aggregate` path
/// can't resolve a room FK and defers indefinitely — stranding the
/// canonical row in `cin_status='active'`.
///
/// Same UPDATE semantics as [`apply_cancelled`] (the header-gone path),
/// but skipped if the canonical row is already cancelled. Returns
/// `Ok(None)` for the event: a domain event would mismatch the bus
/// invariant that `CheckInCancelled` carries a real cancellation
/// transition, and these strands are recoveries, not transitions.
async fn apply_cancelled_for_present_header(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    existing: &ExistingCheckIn,
    cin_no: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    if existing.cin_status.as_deref() == Some("cancelled") {
        return Ok(None);
    }
    tracing::debug!(
        cin_no,
        cin_id = existing.cin_id,
        "ht_checkins apply: short-circuit cancellation \
         (legacy header present but all HT_CheckIn_Ds rows deleted)"
    );
    sqlx::query(
        "UPDATE ht_checkins \
            SET cin_status = 'cancelled', \
                updated_at = NOW() \
          WHERE cin_id = $1",
    )
    .bind(existing.cin_id)
    .execute(&mut **tx)
    .await?;
    Ok(None)
}

/// Mark the canonical row cancelled when the legacy header has gone.
/// Emits `CheckInCancelled` if a row existed; `Ok(None)` otherwise.
async fn apply_cancelled(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_no: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    let existing = fetch_existing(tx, cin_no).await?;
    let Some(ex) = existing else {
        return Ok(None);
    };
    if ex.cin_status.as_deref() == Some("cancelled") {
        return Ok(None);
    }
    sqlx::query(
        "UPDATE ht_checkins \
            SET cin_status = 'cancelled', \
                updated_at = NOW() \
          WHERE cin_id = $1",
    )
    .bind(ex.cin_id)
    .execute(&mut **tx)
    .await?;
    let agg_id = ex
        .aggregate_id
        .unwrap_or_else(|| aggregate_uuid(AggregateKind::CheckIn, ex.cin_id));
    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };
    Ok(Some(DomainEvent::CheckInCancelled {
        id: agg_id,
        source,
        reason: Some("legacy app cancelled or deleted check-in".into()),
    }))
}

/// Re-project the parent booking aggregate. Best-effort: failures here
/// are logged but don't roll back the check-in apply (the next CT tick
/// will pick up the booking change anyway).
async fn reproject_parent_booking(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: &DbPool,
    book_id: &str,
) -> Result<(), SyncError> {
    let agg = match load_booking_aggregate(mssql, book_id).await {
        Ok(a) => a,
        Err(err) => {
            tracing::warn!(
                book_id,
                error = %err,
                "checkout side-effect: failed to reload parent booking; \
                 next CT tick will catch it"
            );
            return Ok(());
        }
    };
    if let Err(err) = apply_booking_aggregate(tx, &agg, book_id).await {
        tracing::warn!(
            book_id,
            error = %err,
            "checkout side-effect: failed to re-project parent booking; \
             next CT tick will catch it"
        );
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Customer FK resolution with eager mirror (defer-forever recovery)
// -----------------------------------------------------------------------------

/// Strategy for sourcing a `HT_Customers` row when the canonical FK
/// lookup misses. The production strategy hits MSSQL via tiberius; tests
/// inject a closure that returns a synthesised `HashMapRow` so the
/// eager-mirror path is exercised without a live legacy connection.
///
/// Kept module-private — callers route through
/// [`resolve_customer_or_eager_mirror`].
///
/// The `Stub` variant is gated behind `#[doc(hidden)]` rather than
/// `#[cfg(test)]` so the integration suite (a separate crate, compiled
/// without the lib's `test` cfg) can also reach it via
/// [`resolve_customer_via_eager_mirror_for_test`].
#[doc(hidden)]
pub enum CustomerSource<'a> {
    /// Production: borrow a live MSSQL pool and `SELECT` one row by
    /// `Cust_no`.
    Mssql(&'a DbPool),
    /// Test injection: deterministic stub returning the row a real MSSQL
    /// fetch would have returned.
    Stub(Box<dyn Fn(&str) -> Option<HashMapRow> + Send + Sync + 'a>),
}

/// Resolve `cin_cust_id` via the standard canonical lookup; on miss,
/// eagerly mirror the referenced `HT_Customers` row from MSSQL into
/// `ht_customers` and retry. Returns `Ok(Some(id))` on success and
/// `Ok(None)` only for legitimate defers (no `legacy_cust_no` on the
/// projection, no MSSQL pool available, or the customer is truly
/// missing in MSSQL too).
///
/// ## Why eager-mirror instead of defer
///
/// The CT watermark advances past a deferred row, so a `legacy_cust_no`
/// that hasn't been mirrored yet would never trigger a re-read of the
/// checkin row. Recovery was accidental — it depended on a later CT
/// update for the same checkin re-firing the aggregate load. Production
/// log lines like `ht_checkins apply deferred: customer not yet
/// mirrored cin_no="CH26-001061" legacy_cust_no=Some("C1951")` are the
/// observable form of that bug. The eager-mirror path closes the
/// window: if the checkin references a customer, that customer MUST
/// exist in MSSQL (the legacy app inserts it before pointing the
/// checkin at it), so a synchronous fetch is always safe.
async fn resolve_customer_or_eager_mirror(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    legacy_cust_no: Option<&str>,
) -> Result<Option<i32>, SyncError> {
    // Fast path — canonical row already there.
    if let Some(id) = resolve::resolve_customer_id(tx, legacy_cust_no).await? {
        return Ok(Some(id));
    }

    // Need a Cust_no AND an MSSQL pool to attempt the eager fetch.
    let (Some(cust_no), Some(pool)) = (legacy_cust_no, mssql) else {
        tracing::warn!(
            legacy_cust_no = ?legacy_cust_no,
            "ht_checkins apply deferred: customer not yet mirrored \
             (no legacy_cust_no or no MSSQL pool available)"
        );
        return Ok(None);
    };

    resolve_customer_via_eager_mirror(tx, CustomerSource::Mssql(pool), cust_no).await
}

/// Inner of [`resolve_customer_or_eager_mirror`] that takes a
/// [`CustomerSource`] so tests can inject a row-supplier closure. Holds
/// the actual mirror + retry plumbing.
async fn resolve_customer_via_eager_mirror(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    source: CustomerSource<'_>,
    cust_no: &str,
) -> Result<Option<i32>, SyncError> {
    let fetched = match &source {
        CustomerSource::Mssql(pool) => fetch_customer_row_from_mssql(pool, cust_no).await?,
        CustomerSource::Stub(f) => f(cust_no),
    };

    let Some(row) = fetched else {
        // Truly missing in MSSQL — should not happen under the legacy
        // app's own ordering invariant, but be defensive. Distinct log
        // message so production can tell the two cases apart.
        tracing::warn!(
            legacy_cust_no = cust_no,
            "ht_checkins apply deferred: customer not in MSSQL \
             — leaving checkin deferred"
        );
        return Ok(None);
    };

    let cust_id = upsert_customer_from_row(tx, &row).await?;
    tracing::debug!(
        legacy_cust_no = cust_no,
        cust_id,
        "ht_checkins apply: eagerly mirrored referenced customer from MSSQL"
    );
    Ok(Some(cust_id))
}

/// Test seam for integration tests: lets the integration suite exercise
/// the eager-mirror path without a live MSSQL connection by injecting a
/// stub row-supplier. Gated `#[doc(hidden)]` so it does not appear in
/// the public-facing rustdoc surface — the suite calls it but
/// production never should.
///
/// Returns the resolved `cust_id` on the eager-mirror hit; `Ok(None)`
/// when the stub returns no row (i.e. customer truly missing in MSSQL).
#[doc(hidden)]
pub async fn resolve_customer_via_eager_mirror_for_test<F>(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cust_no: &str,
    supplier: F,
) -> Result<Option<i32>, SyncError>
where
    F: Fn(&str) -> Option<HashMapRow> + Send + Sync + 'static,
{
    let source = CustomerSource::Stub(Box::new(supplier));
    resolve_customer_via_eager_mirror(tx, source, cust_no).await
}

/// Pull one `HT_Customers` row by `Cust_no`. Mirrors
/// `parent_loader::fetch_rows` shape — a single-table `SELECT` via
/// `simple_query` with inline-quoted WHERE value.
///
/// Returns `Ok(None)` when the row genuinely doesn't exist in MSSQL —
/// caller treats that as the defensive "leave deferred" branch.
async fn fetch_customer_row_from_mssql(
    pool: &DbPool,
    cust_no: &str,
) -> Result<Option<HashMapRow>, SyncError> {
    let mut conn = pool.get().await?;
    let where_q = sql_quote_inline(cust_no);
    let select_list = CUSTOMER_EAGER_FETCH_COLUMNS.join(", ");
    let sql = format!(
        "SELECT {select_list} FROM {HT_CUSTOMERS} WHERE Cust_no = {where_q}"
    );

    let stream = conn.simple_query(&sql).await?;
    let raw_rows = stream.into_first_result().await?;
    let Some(raw) = raw_rows.first() else {
        return Ok(None);
    };

    let mut h = HashMapRow::new(HT_CUSTOMERS);
    for col in CUSTOMER_EAGER_FETCH_COLUMNS {
        let cell = read_cell(raw, col).unwrap_or(MockValue::Null);
        h.cells.insert((*col).to_string(), cell);
    }
    Ok(Some(h))
}

/// SQL-quote a value for inline interpolation. Same semantics as
/// `parent_loader::sql_quote_inline` — duplicated here so this module
/// stays self-contained for the eager-fetch path.
fn sql_quote_inline(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Probe a tiberius cell as the most-specific type that succeeds.
/// Mirrors `parent_loader::read_cell` — kept private here so the
/// boundary translator stays close to the eager-fetch caller.
fn read_cell(row: &tiberius::Row, col: &str) -> Option<MockValue> {
    if let Ok(Some(s)) = tiberius::Row::try_get::<&str, _>(row, col) {
        return Some(MockValue::Str(s.to_string()));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i32, _>(row, col) {
        return Some(MockValue::I32(n));
    }
    if let Ok(Some(d)) = tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, col) {
        return Some(MockValue::DateTime(d));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<f64, _>(row, col) {
        return Some(MockValue::Decimal(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i64, _>(row, col) {
        return Some(MockValue::I64(n));
    }
    None
}

// -----------------------------------------------------------------------------
// Projection helpers
// -----------------------------------------------------------------------------

fn project_aggregate(
    agg: &CheckInAggregate,
    cin_no: &str,
) -> Result<CanonicalCheckIn, SyncError> {
    let header = agg.header.as_ref().ok_or_else(|| SyncError::Mapper {
        table: HT_CHECKIN_H,
        message: "project_aggregate called with header=None".into(),
    })?;

    let legacy_cust_no = header.try_get_str("Cin_cust_no")?.map(str::to_string);
    let legacy_book_id = header
        .try_get_str("Cin_Book_no")?
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let legacy_status_raw = header
        .try_get_str("Cin_status")?
        .unwrap_or_default()
        .to_string();

    // Total amounts on the header.
    let total_amount = header
        .try_get_decimal("Total_Price_Net")?
        .or(header.try_get_decimal("Total_Price_Room").ok().flatten());
    // `cin_paid_amount` mirrors `HT_CheckIn_H.Total_Price_Pay`, NOT the
    // sum of `HT_CheckIn_Pay.Cin_Pay_*` rows. The header is the legacy
    // app's source of truth (recipe `payment.rs::execute_all` keeps it
    // synchronised with the ledger).
    let paid_amount = header.try_get_decimal("Total_Price_Pay")?;

    let (cin_checkin_time, cin_expected_checkout) =
        derive_stay_range(header, &agg.rooms)?;

    // Per-room state determines status + checkout-completion.
    let room_state = derive_room_state(&agg.rooms, &legacy_status_raw)?;

    Ok(CanonicalCheckIn {
        legacy_cin_no: cin_no.to_string(),
        legacy_book_id,
        legacy_cust_no,
        legacy_room_no: room_state.first_room_no,
        cin_status: room_state.canonical_status,
        cin_checkin_time,
        cin_checkout_time: room_state.checkout_time,
        cin_expected_checkout,
        total_amount,
        paid_amount,
        legacy_checkin_ds_id: room_state.first_ds_id,
        is_fully_checked_out: room_state.is_fully_checked_out,
    })
}

/// Translate the legacy `Cin_status` literal to the PG canonical
/// literal already in use by `routes/new_checkins`.
///
/// User constraint: legacy literals stay verbatim — this fn is the
/// *one* boundary translator; everywhere else passes the legacy string
/// through unchanged.
fn legacy_status_to_pg(legacy_header: &str) -> &'static str {
    match legacy_header {
        CIN_STATUS_CANCELLED => "cancelled",
        // `'ปกติ'` is the active default. Treat anything non-cancelled
        // as `'active'` until the per-room state machine tells us
        // otherwise (handled in `derive_room_state`).
        _ => "active",
    }
}

/// Output of [`derive_room_state`]. Five orthogonal facts the
/// projection layer needs to assemble a `CanonicalCheckIn`.
struct RoomState {
    canonical_status: String,
    is_fully_checked_out: bool,
    checkout_time: Option<NaiveDateTime>,
    first_room_no: Option<String>,
    first_ds_id: Option<i32>,
}

/// Examine all `HT_CheckIn_Ds` rows and decide:
///
/// * the canonical PG status (`'active'` / `'checkedout'` /
///   `'cancelled'`)
/// * whether ALL rooms have flipped to `'Check-Out'` (drives the
///   booking re-projection side-effect)
/// * the checkout timestamp (the latest `Cin_Room_Out` across rooms,
///   when fully checked out)
/// * the first room number (for `legacy_room_no` denormalisation)
/// * the first `HT_CheckIn_Ds.id` (for `legacy_checkin_ds_id`)
fn derive_room_state(
    rooms: &[crate::sync::row::test_support::HashMapRow],
    legacy_header_status: &str,
) -> Result<RoomState, SyncError> {
    // Header-level cancel always wins.
    if legacy_header_status == CIN_STATUS_CANCELLED {
        return Ok(RoomState {
            canonical_status: "cancelled".into(),
            is_fully_checked_out: false,
            checkout_time: None,
            first_room_no: rooms
                .first()
                .and_then(|r| r.try_get_str("Cin_Room_No").ok().flatten())
                .map(str::to_string),
            first_ds_id: rooms.first().and_then(|r| r.try_get_i32("id").ok().flatten()),
        });
    }

    if rooms.is_empty() {
        // No room rows yet — transient state during a multi-statement
        // legacy edit. Fall back to header-derived status; no checkout.
        return Ok(RoomState {
            canonical_status: legacy_status_to_pg(legacy_header_status).into(),
            is_fully_checked_out: false,
            checkout_time: None,
            first_room_no: None,
            first_ds_id: None,
        });
    }

    let mut all_checked_out = true;
    let mut latest_out: Option<NaiveDateTime> = None;
    let mut first_room_no: Option<String> = None;
    let mut first_ds_id: Option<i32> = None;

    for r in rooms {
        let status = r.try_get_str("Cin_Room_Status")?.unwrap_or_default();
        if status != CIN_ROOM_STATUS_CHECKED_OUT {
            all_checked_out = false;
        }
        if let Some(out) = r.try_get_datetime("Cin_Room_Out")? {
            latest_out = Some(latest_out.map_or(out, |existing| existing.max(out)));
        }
        if first_room_no.is_none() {
            first_room_no = r.try_get_str("Cin_Room_No")?.map(str::to_string);
        }
        if first_ds_id.is_none() {
            first_ds_id = r.try_get_i32("id")?;
        }
    }

    let canonical_status = if all_checked_out {
        "checkedout"
    } else {
        legacy_status_to_pg(legacy_header_status)
    };

    let checkout_time = if all_checked_out { latest_out } else { None };

    Ok(RoomState {
        canonical_status: canonical_status.into(),
        is_fully_checked_out: all_checked_out,
        checkout_time,
        first_room_no,
        first_ds_id,
    })
}

/// Derive the canonical stay range — `(cin_checkin_time,
/// cin_expected_checkout)` — from the legacy aggregate.
///
/// `cin_checkin_time` is always sourced from `HT_CheckIn_H.Cin_Date_in`.
/// That value is set on insert and never moves on extension.
///
/// `cin_expected_checkout` is the trickier piece. The legacy iHOTEL app
/// writes extensions to `HT_CheckIn_Ds.Cin_Room_Out`, NOT to
/// `HT_CheckIn_H.Cin_Date_Out`. Per
/// `docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_CheckIn_Ds` (line
/// "Update on extend (ClickUSE.cs:1146): updates Cin_Room_Out for stay
/// extension."), the Ds-row value is the source of truth for the
/// current expected checkout once a stay has been extended.
///
/// Strategy:
/// 1. Compute `max(Cin_Room_Out)` across Ds rows that are NOT already
///    `'Check-Out'`. Already-checked-out rooms carry the actual
///    departure timestamp, which is stale relative to a later
///    extension on other still-active rooms.
/// 2. Fall back to `HT_CheckIn_H.Cin_Date_Out` when:
///    - no Ds rows are loaded yet (transient mid-edit state), OR
///    - every Ds row is fully checked out (status is `'checkedout'`
///      anyway; keep the date stable rather than backwards-jumping to
///      a stale Ds value), OR
///    - the still-active Ds rows have no `Cin_Room_Out` populated yet
///      (the legacy app sets `Cin_Room_Out` only after the user picks
///      a checkout time — until then it's NULL).
fn derive_stay_range(
    header: &dyn MappableRow,
    rooms: &[HashMapRow],
) -> Result<(NaiveDateTime, NaiveDate), SyncError> {
    let date_in: NaiveDateTime = header
        .try_get_datetime("Cin_Date_in")?
        .ok_or_else(|| SyncError::Mapper {
            table: HT_CHECKIN_H,
            message: "Cin_Date_in is NULL on header".into(),
        })?;
    let header_date_out: NaiveDateTime = header
        .try_get_datetime("Cin_Date_Out")?
        .ok_or_else(|| SyncError::Mapper {
            table: HT_CHECKIN_H,
            message: "Cin_Date_Out is NULL on header".into(),
        })?;

    let expected_checkout = max_room_out_among_active(rooms)?
        .map(|dt| dt.date())
        .unwrap_or_else(|| header_date_out.date());

    Ok((date_in, expected_checkout))
}

/// Largest `Cin_Room_Out` across Ds rows whose `Cin_Room_Status` is NOT
/// `'Check-Out'`. Returns `None` when no still-active row carries a
/// populated `Cin_Room_Out` — caller falls back to the header date.
fn max_room_out_among_active(
    rooms: &[HashMapRow],
) -> Result<Option<NaiveDateTime>, SyncError> {
    let mut latest: Option<NaiveDateTime> = None;
    for r in rooms {
        let status = r.try_get_str("Cin_Room_Status")?.unwrap_or_default();
        if status == CIN_ROOM_STATUS_CHECKED_OUT {
            continue;
        }
        if let Some(out) = r.try_get_datetime("Cin_Room_Out")? {
            latest = Some(latest.map_or(out, |existing| existing.max(out)));
        }
    }
    Ok(latest)
}

// -----------------------------------------------------------------------------
// PG access
// -----------------------------------------------------------------------------

async fn fetch_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    legacy_cin_no: &str,
) -> Result<Option<ExistingCheckIn>, SyncError> {
    let row = sqlx::query_as::<_, (
        i32,
        Option<Uuid>,
        Option<String>,
        Option<f64>,
        Option<f64>,
        Option<NaiveDateTime>,
    )>(
        "SELECT cin_id, aggregate_id, cin_status, \
                cin_total_amount::float8, cin_paid_amount::float8, cin_checkout_time \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(
        |(cin_id, aggregate_id, cin_status, total, paid, checkout)| ExistingCheckIn {
            cin_id,
            aggregate_id,
            cin_status,
            cin_total_amount: total,
            cin_paid_amount: paid,
            cin_checkout_time: checkout,
        },
    ))
}

fn existing_matches(ex: &ExistingCheckIn, p: &CanonicalCheckIn) -> bool {
    ex.cin_status.as_deref() == Some(p.cin_status.as_str())
        && ex.cin_total_amount == p.total_amount
        && ex.cin_paid_amount == p.paid_amount
        && ex.cin_checkout_time == p.cin_checkout_time
}

#[allow(clippy::too_many_arguments)]
async fn update_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_id_serial: i32,
    cust_id: i32,
    room_id: i32,
    book_id: Option<i32>,
    p: &CanonicalCheckIn,
    agg_id: Uuid,
) -> Result<(), SyncError> {
    sqlx::query(
        "UPDATE ht_checkins \
            SET cin_cust_id            = $1, \
                cin_room_id            = $2, \
                cin_book_id            = COALESCE($3, cin_book_id), \
                cin_status             = $4, \
                cin_checkin_time       = $5, \
                cin_checkout_time      = $6, \
                cin_expected_checkout  = $7, \
                cin_total_amount       = $8::float8, \
                cin_paid_amount        = $9::float8, \
                legacy_cin_no          = COALESCE(legacy_cin_no, $10), \
                legacy_room_no         = COALESCE(legacy_room_no, $11), \
                legacy_cust_no         = COALESCE(legacy_cust_no, $12), \
                legacy_checkin_ds_id   = COALESCE(legacy_checkin_ds_id, $13), \
                aggregate_id           = COALESCE(aggregate_id, $14), \
                updated_at             = NOW() \
          WHERE cin_id = $15",
    )
    .bind(cust_id)
    .bind(room_id)
    .bind(book_id)
    .bind(&p.cin_status)
    .bind(p.cin_checkin_time)
    .bind(p.cin_checkout_time)
    .bind(p.cin_expected_checkout)
    .bind(p.total_amount)
    .bind(p.paid_amount)
    .bind(&p.legacy_cin_no)
    .bind(&p.legacy_room_no)
    .bind(&p.legacy_cust_no)
    .bind(p.legacy_checkin_ds_id)
    .bind(agg_id)
    .bind(cin_id_serial)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_new(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cust_id: i32,
    room_id: i32,
    book_id: Option<i32>,
    p: &CanonicalCheckIn,
) -> Result<i32, SyncError> {
    // `cin_no` is NOT NULL UNIQUE in our schema; reuse the legacy
    // `Cin_no` as the canonical `cin_no` to keep the human reference
    // stable across both apps.
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO ht_checkins \
             (cin_no, cin_book_id, cin_cust_id, cin_room_id, \
              cin_checkin_time, cin_checkout_time, cin_expected_checkout, \
              cin_status, cin_total_amount, cin_paid_amount, \
              legacy_cin_no, legacy_room_no, legacy_cust_no, legacy_checkin_ds_id, source) \
         VALUES \
             ($1, $2, $3, $4, $5, $6, $7, $8, $9::float8, $10::float8, \
              $11, $12, $13, $14, 'legacy_app') \
         RETURNING cin_id",
    )
    .bind(&p.legacy_cin_no)
    .bind(book_id)
    .bind(cust_id)
    .bind(room_id)
    .bind(p.cin_checkin_time)
    .bind(p.cin_checkout_time)
    .bind(p.cin_expected_checkout)
    .bind(&p.cin_status)
    .bind(p.total_amount)
    .bind(p.paid_amount)
    .bind(&p.legacy_cin_no)
    .bind(&p.legacy_room_no)
    .bind(&p.legacy_cust_no)
    .bind(p.legacy_checkin_ds_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(row.0)
}

/// Build the appropriate `CheckInCreated` / `CheckInCancelled` /
/// `CheckOutCompleted` event.
///
/// Decision tree:
/// 1. canonical status `'cancelled'` → `CheckInCancelled`
/// 2. fully-checked-out (every room flipped) → `CheckOutCompleted`
/// 3. brand-new row → `CheckInCreated`
/// 4. otherwise (in-flight modification) → `CheckInCreated` re-emitted
///    with the latest snapshot. We don't have a `CheckInModified`
///    variant in the current event taxonomy, so we treat repeated
///    apply calls as idempotent re-publications of the create event;
///    subscribers de-dup by `aggregate_id`.
fn build_event(
    was_insert: bool,
    cust_id: i32,
    room_id: i32,
    book_id: Option<i32>,
    agg_id: Uuid,
    p: &CanonicalCheckIn,
) -> DomainEvent {
    use crate::domain::checkin::CheckInState;
    use crate::domain::shared::Money;

    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };

    if p.cin_status == "cancelled" {
        return DomainEvent::CheckInCancelled {
            id: agg_id,
            source,
            reason: Some("legacy app set Cin_status='ยกเลิก'".into()),
        };
    }

    if p.is_fully_checked_out {
        return DomainEvent::CheckOutCompleted {
            id: agg_id,
            source,
        };
    }

    // For both fresh inserts and updates we emit `CheckInCreated`
    // carrying the latest snapshot. The event taxonomy doesn't currently
    // have a `CheckInModified` variant — subscribers that care about
    // updates re-fetch on every CheckInCreated for the same aggregate.
    let _ = was_insert; // Documented above; future-proof for a Modified variant.

    let booking_uuid = book_id.map(|id| aggregate_uuid(AggregateKind::Booking, id));
    let _ = room_id; // room_no is the public surface in CheckInSnapshot.

    let snapshot = CheckInSnapshot {
        id: agg_id,
        legacy_cin_no: Some(p.legacy_cin_no.clone()),
        booking_id: booking_uuid,
        customer_id: aggregate_uuid(AggregateKind::Customer, cust_id),
        status: match p.cin_status.as_str() {
            "checkedout" => CheckInState::CheckedOut,
            "cancelled" => CheckInState::Cancelled,
            _ => CheckInState::Active,
        },
        room_no: p.legacy_room_no.clone().unwrap_or_default(),
        stay_start: naive_dt_to_utc(p.cin_checkin_time),
        stay_end: naive_date_to_utc(p.cin_expected_checkout),
        total_price_net: p
            .total_amount
            .map(|f| Money::from_satang((f * 100.0).round() as i64))
            .unwrap_or(Money::ZERO),
    };

    DomainEvent::CheckInCreated {
        id: agg_id,
        source,
        snapshot,
    }
}

fn naive_dt_to_utc(dt: NaiveDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    chrono::Utc.from_utc_datetime(&dt)
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

    fn header_row(cin_no: &str, cust_no: &str, status: &str) -> HashMapRow {
        HashMapRow::new(HT_CHECKIN_H)
            .with("Cin_no", MockValue::Str(cin_no.into()))
            .with("Cin_status", MockValue::Str(status.into()))
            .with("Cin_Book_no", MockValue::Str(String::new()))
            .with("Cin_cust_no", MockValue::Str(cust_no.into()))
            .with(
                "Cin_Date_in",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                        .unwrap()
                        .and_hms_opt(14, 30, 0)
                        .unwrap(),
                ),
            )
            .with(
                "Cin_Date_Out",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 27)
                        .unwrap()
                        .and_hms_opt(12, 0, 0)
                        .unwrap(),
                ),
            )
            .with("Total_Price_Room", MockValue::Decimal(890.0))
            .with("Total_Price_Net", MockValue::Decimal(890.0))
            .with("Total_Price_Pay", MockValue::Decimal(0.0))
            .with("Total_Price_Balance", MockValue::Decimal(890.0))
    }

    fn ds_row(cin_no: &str, room_no: &str, status: &str) -> HashMapRow {
        HashMapRow::new(HT_CHECKIN_DS)
            .with("id", MockValue::I32(25001))
            .with("Cin_No", MockValue::Str(cin_no.into()))
            .with("Cin_Room_No", MockValue::Str(room_no.into()))
            .with("Cin_Room_Status", MockValue::Str(status.into()))
            .with("Cin_Room_Out", MockValue::Null)
    }

    fn ds_row_checked_out(cin_no: &str, room_no: &str) -> HashMapRow {
        ds_row(cin_no, room_no, CIN_ROOM_STATUS_CHECKED_OUT).with(
            "Cin_Room_Out",
            MockValue::DateTime(
                chrono::NaiveDate::from_ymd_opt(2026, 4, 27)
                    .unwrap()
                    .and_hms_opt(11, 30, 0)
                    .unwrap(),
            ),
        )
    }

    // ----- legacy_status_to_pg -------------------------------------------

    #[test]
    fn legacy_status_normal_maps_to_active() {
        assert_eq!(legacy_status_to_pg("ปกติ"), "active");
    }

    #[test]
    fn legacy_status_yokleek_maps_to_cancelled() {
        // User constraint: 'ยกเลิก' is the verbatim cancelled literal.
        assert_eq!(legacy_status_to_pg("ยกเลิก"), "cancelled");
    }

    #[test]
    fn legacy_status_unknown_falls_back_to_active() {
        // Per-room state may still flip this to 'checkedout' in
        // derive_room_state — header status alone never returns
        // 'checkedout'.
        assert_eq!(legacy_status_to_pg(""), "active");
        assert_eq!(legacy_status_to_pg("?"), "active");
    }

    // ----- project_aggregate ---------------------------------------------

    #[test]
    fn project_active_walkin_carries_room_and_amounts() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![ds_row("CH26-005228", "402", "เข้าพัก")],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.legacy_cin_no, "CH26-005228");
        assert_eq!(p.legacy_cust_no.as_deref(), Some("C21607"));
        assert!(p.legacy_book_id.is_none(), "walk-in must carry no book id");
        assert_eq!(p.legacy_room_no.as_deref(), Some("402"));
        assert_eq!(p.cin_status, "active");
        assert!(!p.is_fully_checked_out);
        assert!(p.cin_checkout_time.is_none());
        assert_eq!(p.total_amount, Some(890.0));
        assert_eq!(p.paid_amount, Some(0.0));
        assert_eq!(p.legacy_checkin_ds_id, Some(25001));
    }

    #[test]
    fn project_cancelled_header_marks_status_cancelled_regardless_of_rooms() {
        // Even if a stale Ds row still says 'เข้าพัก', a header
        // cancel must dominate.
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ยกเลิก")),
            rooms: vec![ds_row("CH26-005228", "402", "เข้าพัก")],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.cin_status, "cancelled");
        assert!(!p.is_fully_checked_out);
    }

    #[test]
    fn project_all_rooms_checked_out_marks_fully_checked_out() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![
                ds_row_checked_out("CH26-005228", "402"),
                ds_row_checked_out("CH26-005228", "403"),
            ],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.cin_status, "checkedout");
        assert!(p.is_fully_checked_out);
        assert!(
            p.cin_checkout_time.is_some(),
            "checkout_time must be set when fully checked out"
        );
    }

    #[test]
    fn project_partial_checkout_keeps_active_status() {
        // Only one of two rooms checked out — stay is still active.
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![
                ds_row_checked_out("CH26-005228", "402"),
                ds_row("CH26-005228", "403", "เข้าพัก"),
            ],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.cin_status, "active");
        assert!(!p.is_fully_checked_out);
        assert!(p.cin_checkout_time.is_none());
    }

    #[test]
    fn project_booking_linked_carries_book_id() {
        let mut header = header_row("CH26-005231", "C21610", "ปกติ");
        header
            .cells
            .insert("Cin_Book_no".into(), MockValue::Str("R014810".into()));
        let agg = CheckInAggregate {
            header: Some(header),
            rooms: vec![ds_row("CH26-005231", "402", "เข้าพัก")],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005231").unwrap();
        assert_eq!(p.legacy_book_id.as_deref(), Some("R014810"));
    }

    /// Regression for the v2.63.0 strand bug: when the legacy header
    /// says cancelled AND every `HT_CheckIn_Ds` row was deleted, the
    /// projection MUST carry `cin_status='cancelled'` with
    /// `legacy_room_no=None`. The `apply_checkin_aggregate`
    /// short-circuit relies on this projection shape to skip FK
    /// resolution.
    #[test]
    fn project_cancelled_with_deleted_ds_rows_carries_no_room() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005252", "C21607", "ยกเลิก")),
            rooms: vec![], // legacy deleted every Ds row
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005252").unwrap();
        assert_eq!(p.cin_status, "cancelled");
        assert!(
            p.legacy_room_no.is_none(),
            "no Ds rows -> no legacy_room_no -> would have stranded the row \
             pre-fix"
        );
        assert!(!p.is_fully_checked_out);
    }

    /// Helper: build an active Ds row that carries an explicit
    /// `Cin_Room_Out` (the column the legacy app stamps on stay
    /// extension per `docs/legacy-app/COMPAT_CHEATSHEET.md`
    /// §`HT_CheckIn_Ds`).
    fn ds_row_with_room_out(
        cin_no: &str,
        room_no: &str,
        status: &str,
        room_out: NaiveDateTime,
    ) -> HashMapRow {
        ds_row(cin_no, room_no, status).with("Cin_Room_Out", MockValue::DateTime(room_out))
    }

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, min, 0)
            .unwrap()
    }

    // ----- derive_stay_range (stay-extension propagation) ----------------

    /// Regression for the stay-extension bug
    /// (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_CheckIn_Ds`,
    /// "Update on extend (ClickUSE.cs:1146)"): the legacy app writes
    /// extensions to `HT_CheckIn_Ds.Cin_Room_Out`, NOT to
    /// `HT_CheckIn_H.Cin_Date_Out`. The mapper must surface the Ds
    /// value as `cin_expected_checkout`.
    #[test]
    fn derive_stay_range_uses_max_cin_room_out_from_active_ds_rows() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        // Header says 4/27 but receptionist extended the stay; the Ds
        // row carries the new checkout date.
        let extended_out = dt(2026, 4, 30, 12, 0);
        let rooms = vec![ds_row_with_room_out(
            "CH26-005351",
            "402",
            "เข้าพัก",
            extended_out,
        )];
        let (date_in, expected_out) = derive_stay_range(&header, &rooms).unwrap();
        assert_eq!(date_in, dt(2026, 4, 26, 14, 30), "checkin time stays on header");
        assert_eq!(expected_out, extended_out.date(), "extension must propagate");
    }

    /// Multi-room aggregate: pick the maximum `Cin_Room_Out` across all
    /// still-active rooms (the latest extension wins).
    #[test]
    fn derive_stay_range_picks_latest_room_out_when_multiple_active() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        let rooms = vec![
            ds_row_with_room_out("CH26-005351", "402", "เข้าพัก", dt(2026, 4, 28, 12, 0)),
            ds_row_with_room_out("CH26-005351", "403", "เข้าพัก", dt(2026, 5, 2, 12, 0)),
            ds_row_with_room_out("CH26-005351", "404", "เข้าพัก", dt(2026, 4, 30, 12, 0)),
        ];
        let (_, expected_out) = derive_stay_range(&header, &rooms).unwrap();
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
            "must pick max across rooms"
        );
    }

    /// Empty rooms list (transient mid-edit state): fall back to the
    /// header `Cin_Date_Out`. Mirrors `derive_room_state`'s
    /// `rooms.is_empty()` branch.
    #[test]
    fn derive_stay_range_falls_back_to_header_when_no_rooms() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        let (_, expected_out) = derive_stay_range(&header, &[]).unwrap();
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
            "header date wins when no Ds rows are loaded"
        );
    }

    /// All rooms already `'Check-Out'`: their `Cin_Room_Out` carries
    /// the actual departure timestamp, which would backwards-jump the
    /// expected date relative to any later extension. Use header.
    #[test]
    fn derive_stay_range_falls_back_to_header_when_all_rooms_checked_out() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        // The dt here is the actual departure (a day BEFORE header's
        // 4/27 expected). We must NOT pick this — header wins.
        let rooms = vec![
            ds_row_with_room_out(
                "CH26-005351",
                "402",
                CIN_ROOM_STATUS_CHECKED_OUT,
                dt(2026, 4, 25, 11, 0),
            ),
            ds_row_with_room_out(
                "CH26-005351",
                "403",
                CIN_ROOM_STATUS_CHECKED_OUT,
                dt(2026, 4, 25, 11, 0),
            ),
        ];
        let (_, expected_out) = derive_stay_range(&header, &rooms).unwrap();
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
            "fully-checked-out aggregate must not regress to a stale Ds value"
        );
    }

    /// Mixed: one room checked out, another still active. Only the
    /// active room's `Cin_Room_Out` counts toward the max.
    #[test]
    fn derive_stay_range_ignores_checked_out_rows_when_others_active() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        let rooms = vec![
            // Already departed — its room_out is stale.
            ds_row_with_room_out(
                "CH26-005351",
                "402",
                CIN_ROOM_STATUS_CHECKED_OUT,
                dt(2026, 4, 25, 11, 0),
            ),
            // Still occupying with an extended checkout.
            ds_row_with_room_out("CH26-005351", "403", "เข้าพัก", dt(2026, 5, 1, 12, 0)),
        ];
        let (_, expected_out) = derive_stay_range(&header, &rooms).unwrap();
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            "checked-out room_out must be ignored, active extension must win"
        );
    }

    /// Active rooms exist but `Cin_Room_Out` is NULL on each (the
    /// legacy app hasn't stamped one yet — booking-only state, pre any
    /// extension or checkout). Fall back to header.
    #[test]
    fn derive_stay_range_falls_back_to_header_when_active_rooms_have_null_room_out() {
        let header = header_row("CH26-005351", "C21607", "ปกติ");
        // ds_row default sets Cin_Room_Out=Null.
        let rooms = vec![ds_row("CH26-005351", "402", "เข้าพัก")];
        let (_, expected_out) = derive_stay_range(&header, &rooms).unwrap();
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
            "NULL Cin_Room_Out on active rows falls through to header"
        );
    }

    /// End-to-end through `project_aggregate`: the extension date must
    /// land on the canonical projection's `cin_expected_checkout`.
    #[test]
    fn project_aggregate_carries_extended_checkout_from_ds_room_out() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005351", "C21607", "ปกติ")),
            rooms: vec![ds_row_with_room_out(
                "CH26-005351",
                "402",
                "เข้าพัก",
                dt(2026, 5, 3, 12, 0),
            )],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005351").unwrap();
        assert_eq!(
            p.cin_expected_checkout,
            chrono::NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
            "stay extension on Ds.Cin_Room_Out must surface as cin_expected_checkout"
        );
    }

    #[test]
    fn project_errors_when_checkin_dates_missing() {
        let mut header = header_row("CH26-005228", "C21607", "ปกติ");
        header.cells.insert("Cin_Date_in".into(), MockValue::Null);
        let agg = CheckInAggregate {
            header: Some(header),
            rooms: vec![],
            payments: vec![],
        };
        let err = project_aggregate(&agg, "CH26-005228").expect_err("missing date must error");
        assert!(err.to_string().contains("Cin_Date_in"));
    }

    // ----- existing_matches ----------------------------------------------

    fn sample_canonical() -> CanonicalCheckIn {
        CanonicalCheckIn {
            legacy_cin_no: "CH26-005228".into(),
            legacy_book_id: None,
            legacy_cust_no: Some("C21607".into()),
            legacy_room_no: Some("402".into()),
            cin_status: "active".into(),
            cin_checkin_time: chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                .unwrap()
                .and_hms_opt(14, 30, 0)
                .unwrap(),
            cin_checkout_time: None,
            cin_expected_checkout: chrono::NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
            total_amount: Some(890.0),
            paid_amount: Some(0.0),
            legacy_checkin_ds_id: Some(25001),
            is_fully_checked_out: false,
        }
    }

    #[test]
    fn existing_matches_is_true_for_unchanged_row() {
        let p = sample_canonical();
        let ex = ExistingCheckIn {
            cin_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cin_status: Some("active".into()),
            cin_total_amount: Some(890.0),
            cin_paid_amount: Some(0.0),
            cin_checkout_time: None,
        };
        assert!(existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_is_false_when_status_differs() {
        let p = sample_canonical();
        let ex = ExistingCheckIn {
            cin_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cin_status: Some("checkedout".into()),
            cin_total_amount: Some(890.0),
            cin_paid_amount: Some(0.0),
            cin_checkout_time: None,
        };
        assert!(!existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_is_false_when_paid_amount_differs() {
        let p = sample_canonical();
        let ex = ExistingCheckIn {
            cin_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cin_status: Some("active".into()),
            cin_total_amount: Some(890.0),
            cin_paid_amount: Some(100.0),
            cin_checkout_time: None,
        };
        assert!(!existing_matches(&ex, &p));
    }

    // ----- coalesce_key --------------------------------------------------

    #[test]
    fn header_mapper_coalesces_on_cin_no() {
        let m = CheckInHeaderMapper;
        let row = header_row("CH26-005228", "C21607", "ปกติ");
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("CH26-005228"));
    }

    #[test]
    fn rooms_mapper_coalesces_on_cin_no_when_present() {
        let m = CheckInRoomsMapper;
        let row = ds_row("CH26-005231", "402", "เข้าพัก");
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("CH26-005231"));
    }

    /// On a D row the joined `Cin_No` is NULL — the mapper returns
    /// `None` and the watcher relies on the sibling header CT row to
    /// drive the aggregate sweep.
    #[test]
    fn rooms_mapper_coalesce_returns_none_when_cin_no_null() {
        let m = CheckInRoomsMapper;
        let row = HashMapRow::new(HT_CHECKIN_DS)
            .with("id", MockValue::I32(25001))
            .with("Cin_No", MockValue::Null);
        assert!(m.coalesce_key(&row).is_none());
    }

    // ----- mapper metadata -----------------------------------------------

    #[test]
    fn header_mapper_metadata_is_correct() {
        let m = CheckInHeaderMapper;
        assert_eq!(m.table(), "HT_CheckIn_H");
        assert_eq!(m.primary_key_cols(), &["Cin_no"]);
        assert!(m.select_sql().contains("Cin_status"));
    }

    #[test]
    fn rooms_mapper_metadata_is_correct() {
        let m = CheckInRoomsMapper;
        assert_eq!(m.table(), "HT_CheckIn_Ds");
        assert_eq!(m.primary_key_cols(), &["id"]);
        // CRITICAL: capital N. Locks against an accidental rename to
        // lowercase that would silently break the parent FK lookup.
        assert!(m.select_sql().contains("t.Cin_No"));
        assert!(m.select_sql().contains("Cin_Room_Status"));
    }

    // ----- build_event ---------------------------------------------------

    #[test]
    fn build_event_for_active_walkin_emits_checkin_created() {
        let p = sample_canonical();
        let agg = aggregate_uuid(AggregateKind::CheckIn, 1);
        let ev = build_event(true, 100, 200, None, agg, &p);
        assert_eq!(ev.type_name(), "CheckInCreated");
        assert_eq!(ev.aggregate_id(), agg);
    }

    #[test]
    fn build_event_for_cancelled_emits_checkin_cancelled() {
        let mut p = sample_canonical();
        p.cin_status = "cancelled".into();
        let agg = aggregate_uuid(AggregateKind::CheckIn, 1);
        let ev = build_event(false, 100, 200, None, agg, &p);
        assert_eq!(ev.type_name(), "CheckInCancelled");
    }

    #[test]
    fn build_event_for_fully_checked_out_emits_checkout_completed() {
        let mut p = sample_canonical();
        p.cin_status = "checkedout".into();
        p.is_fully_checked_out = true;
        p.cin_checkout_time = Some(
            chrono::NaiveDate::from_ymd_opt(2026, 4, 27)
                .unwrap()
                .and_hms_opt(11, 30, 0)
                .unwrap(),
        );
        let agg = aggregate_uuid(AggregateKind::CheckIn, 1);
        let ev = build_event(false, 100, 200, None, agg, &p);
        assert_eq!(ev.type_name(), "CheckOutCompleted");
    }

    /// User-constraint regression: cancelled legacy literal `'ยกเลิก'`
    /// must round-trip to `CheckInCancelled`. Locks the verbatim
    /// translation in `legacy_status_to_pg`.
    #[test]
    fn cancelled_legacy_status_round_trips_to_checkin_cancelled() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ยกเลิก")),
            rooms: vec![],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.cin_status, "cancelled");
        let ev = build_event(
            false,
            100,
            200,
            None,
            aggregate_uuid(AggregateKind::CheckIn, 1),
            &p,
        );
        assert_eq!(ev.type_name(), "CheckInCancelled");
    }

    // ----- coalescing semantics ------------------------------------------

    /// Mirrors the watcher's coalescing pre-pass: 1 H + 2 Ds CT rows
    /// for the same check-in must yield exactly ONE unique key.
    #[test]
    fn coalescing_dedups_one_h_plus_two_ds_to_single_apply() {
        let header = CheckInHeaderMapper;
        let rooms = CheckInRoomsMapper;

        let h = header_row("CH26-005228", "C21607", "ปกติ");
        let ds1 = ds_row("CH26-005228", "402", "เข้าพัก");
        let ds2 = ds_row("CH26-005228", "403", "เข้าพัก");

        let mut keys = std::collections::HashSet::new();
        if let Some(k) = header.coalesce_key(&h) {
            keys.insert(k);
        }
        for r in &[&ds1, &ds2] {
            if let Some(k) = rooms.coalesce_key(*r) {
                keys.insert(k);
            }
        }
        assert_eq!(keys.len(), 1);
        assert!(keys.contains("CH26-005228"));
    }
}
