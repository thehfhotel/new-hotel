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
//! the parent booking should read `'completed'` on our side. NOTE
//! (corrected 2026-06-11): iHOTEL's own checkout flow NEVER touches
//! `HT_Book_H` — the `Book_Status='ออกแล้ว'` literal is written only by
//! OUR writeback checkout recipe (`writeback/recipes/checkout.rs`), so
//! a checkout performed inside iHOTEL leaves `HT_Book_H.Book_Status`
//! at its pre-checkout value. The one-shot re-projection of the parent
//! booking below simply re-mirrors whatever `HT_Book_H` currently says
//! (it picks up `'ออกแล้ว'` promptly when our own writeback stamped it;
//! for a pure-iHOTEL checkout the header keeps its prior status and the
//! re-projection is a no-op). The call uses the same TX so the two
//! UPSERTs commit atomically.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use crate::db::DbPool;
use crate::outbox::event::{CheckInSnapshot, DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::booking::apply_booking_aggregate;
use crate::sync::mappers::customer::{
    ensure_deleted_customer_sentinel, upsert_customer_from_row, DELETED_CUSTOMER_SENTINEL,
    EAGER_FETCH_COLUMNS as CUSTOMER_EAGER_FETCH_COLUMNS, TABLE as HT_CUSTOMERS,
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
    /// Denormalised customer pointer — compared by `existing_matches`
    /// since 2026-06-11 (audit P1 #6): iHOTEL's customer-delete cascade
    /// rewrites `HT_CheckIn_H.Cin_cust_no` to `'C0000'` WITHOUT touching
    /// status or amounts, so a status/amount-only comparison silently
    /// skipped the re-point and the canonical row kept referencing the
    /// deleted customer forever.
    legacy_cust_no: Option<String>,
}

/// Canonical PG-shape projection of the legacy aggregate. This is what
/// we UPSERT.
///
/// Exposed to `crate::scheduler::sync` (via `pub(crate)`) so the
/// reconcile auto-resolve sweep can call the SAME projection function
/// the mapper uses for canonical writes — eliminating the parallel
/// projection pipeline that was the root cause of Bug B / C / D
/// (2026-05-15..16). The sweep no longer maintains its own
/// "MSSQL → hash inputs" projection; it loads the aggregate and runs
/// `project_aggregate` exactly once. Mismatch with the canonical row
/// can then only signal real exogenous drift (CT miss, hand-edited
/// PG), not parallel-projection-pipeline divergence.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CanonicalCheckIn {
    pub(crate) legacy_cin_no: String,
    pub(crate) legacy_book_id: Option<String>,
    pub(crate) legacy_cust_no: Option<String>,
    pub(crate) legacy_room_no: Option<String>,
    /// Legacy literal verbatim (per user constraint) translated to our
    /// PG enum literal (`'active'`/`'checkedout'`/`'cancelled'`). The
    /// translation sits in [`legacy_status_to_pg`].
    pub(crate) cin_status: String,
    pub(crate) cin_checkin_time: NaiveDateTime,
    /// Set on checkout (the moment the last room flips to `'Check-Out'`).
    /// `None` for active stays.
    pub(crate) cin_checkout_time: Option<NaiveDateTime>,
    pub(crate) cin_expected_checkout: NaiveDate,
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
    /// Track B2 / T2 CRIT-1 — per-room projection. Each entry mirrors
    /// one `HT_CheckIn_Ds` row and lands as one `ht_checkin_rooms` row
    /// keyed on `(cr_cin_id, cr_room_id)`. The legacy header is one
    /// row per folio; the junction is one row per room. Pre-B2 the
    /// mapper collapsed this to `legacy_room_no` only (= first room),
    /// which lost all rooms 2..N.
    rooms: Vec<CanonicalRoom>,
}

/// Per-room slice of the canonical projection — one entry per
/// `HT_CheckIn_Ds` row in the legacy aggregate, mirroring
/// `ht_checkin_rooms` cardinality (junction keyed on
/// `(cr_cin_id, cr_room_id)`). Track B2 / T2 CRIT-1.
#[derive(Debug, Clone, PartialEq)]
struct CanonicalRoom {
    /// `HT_CheckIn_Ds.Cin_Room_No` — resolved to `cr_room_id` at apply
    /// time. Carried as a string here so projection stays pure (no PG
    /// access). `None` is invalid for a row that landed in the
    /// aggregate; we filter zero-room-no rows out at projection.
    legacy_room_no: String,
    /// `HT_CheckIn_Ds.Cin_Room_Status` — Thai/English literal preserved
    /// verbatim per user constraint (`'เข้าพัก'` / `'Check-Out'` /
    /// `'จอง'` / `'ยกเลิก'`).
    cr_room_status: String,
    cr_room_in: Option<NaiveDateTime>,
    cr_room_out: Option<NaiveDateTime>,
    cr_rate_per_night: f64,
    cr_nights: i32,
    cr_room_total: f64,
    cr_dep_amount: f64,
    cr_dep_status: Option<String>,
    cr_dep_returned_at: Option<NaiveDateTime>,
    cr_dep_returned_by: Option<String>,
    /// `HT_CheckIn_Ds.id` (legacy IDENTITY) — partial-indexed in PG when
    /// non-NULL so writeback can resolve back to the legacy row.
    cr_legacy_ds_id: Option<i32>,
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
/// * `Ok(None)` ONLY for genuine idempotent skips (the canonical row
///   already mirrors the legacy aggregate).
/// * `Err(SyncError::Mapper)` when an FK cannot be resolved even after
///   the eager-mirror fallback. The watcher's per-key handler records
///   the error and HOLDS the watermark so the row is loudly retried —
///   never silently dropped. (Pre-2026-06-11 these paths returned
///   `Ok(None)`, which the watcher counted as `skipped` while the
///   watermark advanced: the June-3 silent-drop class.)
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
    // aggregate. Track B2 / T2 HIGH-2 (audit 2026-05-13) widened this
    // check to include the per-room junction set: pre-B2 a pure
    // room-change (rooms added / dropped / swapped without touching
    // header amounts) idempotency-skipped because `existing_matches`
    // only compared header fields. We now also fetch the current
    // `ht_checkin_rooms` rows for the folio and compare the
    // `(legacy_room_no, cr_room_status)` SET against the projection's
    // `rooms` slice. Any divergence forces a re-apply.
    if let Some(ex) = existing.as_ref() {
        if existing_matches(ex, &projection) && ex.aggregate_id.is_some() {
            let existing_rooms = fetch_existing_room_set(tx, ex.cin_id).await?;
            if rooms_match(&existing_rooms, &projection.rooms) {
                return Ok(None);
            }
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
    // When it does NOT exist, this is a cancellation of a folio we never
    // mirrored. We cannot INSERT a cancelled canonical row (the legacy
    // cancel cascade deletes every Ds row, so there is no room FK to
    // satisfy `cin_room_id NOT NULL`), and erroring would wedge the
    // watermark forever on a folio that carries no recoverable state.
    // This is a deliberate, logged skip — NOT the silent FK-defer class:
    // there is nothing to converge (legacy says cancelled, PG has no
    // row; absence ≙ cancelled for every reader). Under the 2026-06-11
    // error-on-defer regime the original INSERT can no longer be
    // silently dropped, so this branch only fires for folios created and
    // cancelled before their first successful mirror (e.g. within one
    // tick window, or pre-CT-bootstrap history).
    if projection.cin_status == "cancelled" {
        if let Some(ex) = existing.as_ref() {
            return apply_cancelled_for_present_header(tx, ex, cin_no).await;
        }
        if projection.legacy_room_no.is_none() {
            tracing::info!(
                cin_no,
                "ht_checkins apply: cancelled folio was never mirrored and \
                 carries no HT_CheckIn_Ds rows — skipping (no canonical row \
                 to cancel, no room FK left to mirror one)"
            );
            return Ok(None);
        }
        // Cancelled, never mirrored, but Ds rows still present (header
        // cancel with stale Ds rows) — fall through to FK resolution so
        // the cancelled canonical row is inserted for audit completeness.
    }

    // Resolve FKs. A miss MUST NOT fall through to `Ok(None)` — the
    // watcher counts that as `skipped` and advances the watermark, so
    // the row would be permanently dropped once CT retention (2 days)
    // ages it out (the 2026-06-03 incident class). Strategy per FK:
    //
    // * customer — eager-mirror from `HT_Customers` via MSSQL in the
    //   same TX, retry the lookup; error only if the customer is
    //   genuinely absent in MSSQL too (or no pool is available).
    // * room — no eager-mirror path exists (room creation is an
    //   operator action); error so the watermark holds until the
    //   operator runs `bin/backfill_rooms`.
    // * booking — eager-mirror the parent booking aggregate from MSSQL
    //   (it may itself eager-mirror its customer); error on miss.
    let cust_id =
        match resolve_customer_or_eager_mirror(tx, mssql, projection.legacy_cust_no.as_deref())
            .await?
        {
            Some(id) => id,
            None => {
                return Err(SyncError::Mapper {
                    table: HT_CHECKIN_H,
                    message: format!(
                        "customer FK unresolvable for cin_no={cin_no} \
                         legacy_cust_no={:?} — eager-mirror failed or no MSSQL \
                         pool; holding watermark for loud retry",
                        projection.legacy_cust_no
                    ),
                });
            }
        };
    let room_id = match resolve::resolve_room_id(tx, projection.legacy_room_no.as_deref())
        .await?
    {
        Some(id) => id,
        None => {
            return Err(SyncError::Mapper {
                table: HT_CHECKIN_H,
                message: format!(
                    "room FK unresolvable for cin_no={cin_no} \
                     legacy_room_no={:?} — run bin/backfill_rooms; holding \
                     watermark for loud retry",
                    projection.legacy_room_no
                ),
            });
        }
    };
    // Booking is OPTIONAL — walk-ins write `Cin_Book_no=''`. Only the
    // case where the legacy carries a non-empty `Cin_Book_no` AND the
    // parent booking row hasn't landed yet needs resolution work.
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
            match resolve_booking_or_eager_mirror(tx, mssql, cin_no, id).await? {
                Some(found) => Some(found),
                None => {
                    return Err(SyncError::Mapper {
                        table: HT_CHECKIN_H,
                        message: format!(
                            "parent booking FK unresolvable for cin_no={cin_no} \
                             legacy_book_id={id} — eager-mirror failed or no \
                             MSSQL pool; holding watermark for loud retry"
                        ),
                    });
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

    // Track B2 / T2 CRIT-1 — resolve every per-room FK BEFORE we mutate
    // canonical state. If any room is missing in `ht_rooms_new` the
    // entire apply errors so the watermark holds and the next tick
    // retries against a fully resolvable aggregate (rooms have no
    // eager-mirror path — operator must run `bin/backfill_rooms`).
    let resolved_rooms = match resolve_canonical_rooms(tx, &projection.rooms).await? {
        Some(rs) => rs,
        None => {
            return Err(SyncError::Mapper {
                table: HT_CHECKIN_DS,
                message: format!(
                    "per-room FK unresolvable for cin_no={cin_no} — at least \
                     one HT_CheckIn_Ds room is missing in ht_rooms_new (run \
                     bin/backfill_rooms); holding watermark for loud retry"
                ),
            });
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

    // Track B2 / T2 CRIT-1 — fan the projection's per-room slice out
    // into `ht_checkin_rooms`. UPSERT each row, then DELETE any
    // junction row whose `cr_room_id` is no longer in the current
    // aggregate (covers iHOTEL dropping a room from a multi-room
    // folio).
    apply_checkin_rooms(tx, cin_id_serial, &resolved_rooms).await?;

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
    // Track B2 — the legacy app deletes every `HT_CheckIn_Ds` row on
    // cancellation. Clear the junction so canonical state mirrors that.
    // The `ON DELETE CASCADE` from `ht_checkins` only fires when the
    // parent `ht_checkins` row is DELETEd; here we keep the parent row
    // (status='cancelled') and explicitly clear the junction.
    delete_dropped_checkin_rooms(tx, existing.cin_id, &[]).await?;
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
    // Track B2 — header gone means every Ds row has gone too. Clear
    // the junction so canonical state mirrors that. Keeps the parent
    // `ht_checkins` row in place so reporting / audit trails can still
    // resolve the cancelled folio by `legacy_cin_no`.
    delete_dropped_checkin_rooms(tx, ex.cin_id, &[]).await?;
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
/// are logged but don't roll back the check-in apply. Unlike an FK
/// defer, a failure here is NOT a lost row — the canonical booking row
/// already exists (the check-in resolved its FK against it); only its
/// status convergence is delayed until any future `HT_Book_H` CT touch
/// or reconcile sweep.
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
                "checkout side-effect: failed to reload parent booking \
                 (best-effort; canonical booking row already exists — only \
                 status convergence is delayed)"
            );
            return Ok(());
        }
    };
    if let Err(err) = apply_booking_aggregate(tx, Some(mssql), &agg, book_id).await {
        tracing::warn!(
            book_id,
            error = %err,
            "checkout side-effect: failed to re-project parent booking \
             (best-effort only — the canonical booking row already exists, \
             so this is a stale-status window, not a lost row; any future \
             HT_Book_H CT touch re-converges it)"
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

/// Resolve a customer FK via the standard canonical lookup; on miss,
/// eagerly mirror the referenced `HT_Customers` row from MSSQL into
/// `ht_customers` and retry. Returns `Ok(Some(id))` on success and
/// `Ok(None)` when no resolution is possible (no `legacy_cust_no`, no
/// MSSQL pool available, or the customer is truly missing in MSSQL
/// too). **Callers must convert that `None` into an error** so the
/// watcher holds the watermark — see `sync::resolve` module doc.
///
/// Shared by the check-in AND booking aggregate appliers (`pub(crate)`
/// since 2026-06-11 — the booking mapper's bare defer was the June-3
/// silent-drop root cause).
///
/// ## Why eager-mirror instead of defer
///
/// The CT watermark advances past a deferred row, so a `legacy_cust_no`
/// that hasn't been mirrored yet would never trigger a re-read of the
/// dependent row. Recovery was accidental — it depended on a later CT
/// update for the same aggregate re-firing the load. Production
/// log lines like `ht_checkins apply deferred: customer not yet
/// mirrored cin_no="CH26-001061" legacy_cust_no=Some("C1951")` are the
/// observable form of that bug. The eager-mirror path closes the
/// window: if the checkin/booking references a customer, that customer
/// MUST exist in MSSQL (the legacy app inserts it before pointing the
/// dependent row at it), so a synchronous fetch is always safe.
pub(crate) async fn resolve_customer_or_eager_mirror(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    legacy_cust_no: Option<&str>,
) -> Result<Option<i32>, SyncError> {
    // Fast path — canonical row already there.
    if let Some(id) = resolve::resolve_customer_id(tx, legacy_cust_no).await? {
        return Ok(Some(id));
    }

    // iHOTEL's customer-delete cascade rewrites dependent rows to the
    // reserved `'C0000'` sentinel, which has NO real `HT_Customers` row
    // (cheatsheet §3.24) — an eager MSSQL fetch can never satisfy it
    // and erroring would wedge the watermark on every delete cascade.
    // Materialise the canonical tombstone placeholder instead.
    if legacy_cust_no == Some(DELETED_CUSTOMER_SENTINEL) {
        let id = ensure_deleted_customer_sentinel(tx).await?;
        return Ok(Some(id));
    }

    // Need a Cust_no AND an MSSQL pool to attempt the eager fetch.
    let (Some(cust_no), Some(pool)) = (legacy_cust_no, mssql) else {
        tracing::warn!(
            legacy_cust_no = ?legacy_cust_no,
            "customer FK unresolvable: not yet mirrored and eager-mirror \
             impossible (no legacy_cust_no or no MSSQL pool) — caller will \
             error to hold the watermark"
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
        // message so production can tell the two cases apart. The
        // caller converts this `None` into an error (watermark hold).
        tracing::warn!(
            legacy_cust_no = cust_no,
            "customer FK unresolvable: not in MSSQL either — caller will \
             error to hold the watermark"
        );
        return Ok(None);
    };

    let cust_id = upsert_customer_from_row(tx, &row).await?;
    tracing::debug!(
        legacy_cust_no = cust_no,
        cust_id,
        "eagerly mirrored referenced customer from MSSQL"
    );
    Ok(Some(cust_id))
}

/// Resolve a check-in's parent-booking FK; on miss, eagerly mirror the
/// whole booking aggregate from MSSQL (header + Ds + Date) in the same
/// TX via [`apply_booking_aggregate`] and retry the lookup. The booking
/// apply may in turn eager-mirror ITS customer — the full dependency
/// chain materialises in one watcher tick instead of silently dropping.
///
/// Returns `Ok(None)` when the booking cannot be resolved even after
/// the eager attempt (no MSSQL pool, the booking is genuinely absent in
/// MSSQL, or its own FK chain is unresolvable). **Callers must convert
/// that `None` into an error** so the watermark holds (see
/// `sync::resolve` module doc — the pre-2026-06-11 `Ok(None)` defer
/// here was a member of the June-3 silent-drop class).
async fn resolve_booking_or_eager_mirror(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    cin_no: &str,
    legacy_book_id: &str,
) -> Result<Option<i32>, SyncError> {
    // Fast path — canonical row already there.
    if let Some(id) = resolve::resolve_booking_id(tx, Some(legacy_book_id)).await? {
        return Ok(Some(id));
    }

    let Some(pool) = mssql else {
        tracing::warn!(
            cin_no,
            legacy_book_id,
            "parent booking FK unresolvable: not yet mirrored and no MSSQL \
             pool for eager-mirror — caller will error to hold the watermark"
        );
        return Ok(None);
    };

    let agg = match load_booking_aggregate(pool, legacy_book_id).await {
        Ok(a) => a,
        Err(err) => {
            tracing::warn!(
                cin_no,
                legacy_book_id,
                error = %err,
                "parent booking eager-mirror: aggregate load from MSSQL \
                 failed — caller will error to hold the watermark"
            );
            return Ok(None);
        }
    };
    if let Err(err) = apply_booking_aggregate(tx, Some(pool), &agg, legacy_book_id).await {
        tracing::warn!(
            cin_no,
            legacy_book_id,
            error = %err,
            "parent booking eager-mirror: apply failed — caller will error \
             to hold the watermark"
        );
        return Ok(None);
    }

    let resolved = resolve::resolve_booking_id(tx, Some(legacy_book_id)).await?;
    if resolved.is_some() {
        tracing::debug!(
            cin_no,
            legacy_book_id,
            "eagerly mirrored parent booking aggregate from MSSQL"
        );
    }
    Ok(resolved)
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

    // R2 (2026-05-14): wrap in per-op read timeout — the eager
    // customer fetch can fan out under retry and a single stuck
    // row-lock would otherwise serialize-block the watcher tick.
    let raw_rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
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

/// Project a legacy aggregate into its canonical PG shape.
///
/// Exposed `pub(crate)` so the reconcile auto-resolve sweep can call
/// the SAME projection function the mapper uses for canonical writes.
/// See [`CanonicalCheckIn`] doc for the architectural rationale.
pub(crate) fn project_aggregate(
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

    // Track B2 / T2 CRIT-1 — full per-room projection. One entry per
    // legacy `HT_CheckIn_Ds` row; lands as one `ht_checkin_rooms` row.
    let rooms = project_rooms(&agg.rooms)?;

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
        rooms,
    })
}

/// Project every `HT_CheckIn_Ds` row into a [`CanonicalRoom`]. Rows
/// without a `Cin_Room_No` are skipped (transient mid-edit state — the
/// junction enforces `cr_room_id NOT NULL`).
///
/// Track B2 / T2 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`):
/// pre-B2 the mapper kept only the FIRST room. That bug is the head of
/// every multi-room cardinality failure documented in audit Theme 1.
///
/// Missing-column probe semantics: `try_get_*` errors when a column is
/// not present in the row. Production rows always carry the full
/// projection from `parent_loader::load_checkin_aggregate`, but legacy
/// fixtures (pre-B2 test rows) may omit columns the B2 widening added.
/// The helpers below collapse "missing" and "NULL" to the same
/// `Option::None` so fixtures using a sparse `ds_row` shape remain
/// projectable without per-test backfill.
fn project_rooms(rooms: &[HashMapRow]) -> Result<Vec<CanonicalRoom>, SyncError> {
    let mut out = Vec::with_capacity(rooms.len());
    for r in rooms {
        let Some(legacy_room_no) = r.try_get_str("Cin_Room_No")?.map(str::to_string) else {
            // Pre-B2 callers tolerated this by collapsing to
            // first_room_no; B2 just drops the row from the junction.
            // The drift is logged so the operator can investigate.
            tracing::debug!(
                target: "sync::checkin",
                "project_rooms: HT_CheckIn_Ds row carries NULL Cin_Room_No — skipping"
            );
            continue;
        };
        if legacy_room_no.is_empty() {
            tracing::debug!(
                target: "sync::checkin",
                "project_rooms: HT_CheckIn_Ds row carries empty Cin_Room_No — skipping"
            );
            continue;
        }

        let cr_room_status = r
            .try_get_str("Cin_Room_Status")?
            .unwrap_or_default()
            .to_string();
        let cr_room_in = optional_datetime(r, "Cin_Room_In");
        let cr_room_out = optional_datetime(r, "Cin_Room_Out");
        let cr_rate_per_night = optional_decimal(r, "Cin_Room_Price").unwrap_or(0.0);
        // The legacy column is `Cin_Room_Night` (capital N on Night per
        // schema dump §3.4) — keep verbatim.
        let cr_nights = optional_i32(r, "Cin_Room_Night").unwrap_or(1);
        let cr_room_total = optional_decimal(r, "Cin_Room_PriceToTal").unwrap_or(0.0);
        // Legacy column names per the authoritative schema dump
        // (`docs/legacy-spike/schema/01-baseline-schema.txt` cols 8 /
        // 15 / 18 / 19). The lowercase-`dep` spelling does not exist
        // on HF Hotel — keep these verbatim (`parent_loader::
        // load_checkin_aggregate` writes them under these keys).
        let cr_dep_amount = optional_decimal(r, "Cin_Room_Dep").unwrap_or(0.0);
        let cr_dep_status = optional_str(r, "Cin_Dep_Status");
        let cr_dep_returned_at = optional_datetime(r, "Cin_Dep_return_date");
        let cr_dep_returned_by = optional_str(r, "Cin_Dep_return_by");
        let cr_legacy_ds_id = optional_i32(r, "id");

        out.push(CanonicalRoom {
            legacy_room_no,
            cr_room_status,
            cr_room_in,
            cr_room_out,
            cr_rate_per_night,
            cr_nights,
            cr_room_total,
            cr_dep_amount,
            cr_dep_status,
            cr_dep_returned_at,
            cr_dep_returned_by,
            cr_legacy_ds_id,
        });
    }
    Ok(out)
}

/// Collapse "column missing" and "column NULL" to `None`. Production
/// `parent_loader::materialise` writes every projection column (NULL
/// when absent in MSSQL), so the missing-column branch only fires for
/// test fixtures that pre-date a projection widening.
fn optional_str(row: &HashMapRow, col: &str) -> Option<String> {
    row.try_get_str(col).ok().flatten().map(str::to_string)
}

fn optional_i32(row: &HashMapRow, col: &str) -> Option<i32> {
    row.try_get_i32(col).ok().flatten()
}

fn optional_decimal(row: &HashMapRow, col: &str) -> Option<f64> {
    row.try_get_decimal(col).ok().flatten()
}

fn optional_datetime(row: &HashMapRow, col: &str) -> Option<NaiveDateTime> {
    row.try_get_datetime(col).ok().flatten()
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
///
/// NULL-tolerance on `Cin_Date_Out` (2026-06-11, audit P2): a single
/// malformed legacy folio with `Cin_Date_Out IS NULL` used to hard-error
/// here. Under the error-on-defer regime a hard error holds the
/// HT_CheckIn_* watermark, so ONE bad folio would wedge ALL check-in
/// sync permanently. Fall back instead — loudly — through:
///
/// 1. max `Cin_Room_Out` across still-active Ds rows (normal path),
/// 2. max `Cin_Room_Out` across ALL Ds rows (even checked-out ones),
/// 3. `Cin_Date_in`'s date (zero-night shape — the row stays visible
///    and the reconcile sweep flags it if iHOTEL later repairs it).
///
/// Only a NULL `Cin_Date_in` (truly unusable — no stay start) still
/// errors.
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
    let header_date_out: Option<NaiveDateTime> = header.try_get_datetime("Cin_Date_Out")?;

    let expected_checkout = match (max_room_out_among_active(rooms)?, header_date_out) {
        (Some(active_max), _) => active_max.date(),
        (None, Some(h)) => h.date(),
        (None, None) => {
            // Malformed legacy folio — keep it loud but never wedge the
            // watermark over one bad row.
            let any_room_out = max_room_out_any(rooms)?;
            tracing::warn!(
                fallback = ?any_room_out,
                "HT_CheckIn_H.Cin_Date_Out is NULL — falling back to max \
                 Cin_Room_Out across all Ds rows, else Cin_Date_in (malformed \
                 legacy folio; pre-2026-06-11 this hard-errored and would now \
                 wedge the checkin watermark)"
            );
            any_room_out.map(|dt| dt.date()).unwrap_or_else(|| date_in.date())
        }
    };

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

/// Largest `Cin_Room_Out` across ALL Ds rows regardless of status. Last
/// resort for the NULL-`Cin_Date_Out` malformed-folio fallback in
/// [`derive_stay_range`] — checked-out rows carry the actual departure
/// timestamp, which beats inventing a date.
fn max_room_out_any(rooms: &[HashMapRow]) -> Result<Option<NaiveDateTime>, SyncError> {
    let mut latest: Option<NaiveDateTime> = None;
    for r in rooms {
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
        Option<String>,
    )>(
        "SELECT cin_id, aggregate_id, cin_status, \
                cin_total_amount::float8, cin_paid_amount::float8, cin_checkout_time, \
                legacy_cust_no \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(
        |(cin_id, aggregate_id, cin_status, total, paid, checkout, legacy_cust_no)| {
            ExistingCheckIn {
                cin_id,
                aggregate_id,
                cin_status,
                cin_total_amount: total,
                cin_paid_amount: paid,
                cin_checkout_time: checkout,
                legacy_cust_no,
            }
        },
    ))
}

fn existing_matches(ex: &ExistingCheckIn, p: &CanonicalCheckIn) -> bool {
    // `legacy_cust_no` comparison is guarded on the projection carrying a
    // value: `update_existing` writes it through `COALESCE($12,
    // legacy_cust_no)`, so a transient NULL on the legacy side never
    // overwrites — comparing a None projection against a Some canonical
    // value would force a re-apply every tick without ever converging.
    // A Some projection (including the `'C0000'` delete-cascade sentinel)
    // MUST match, or the apply re-runs and re-points the FK.
    ex.cin_status.as_deref() == Some(p.cin_status.as_str())
        && ex.cin_total_amount == p.total_amount
        && ex.cin_paid_amount == p.paid_amount
        && ex.cin_checkout_time == p.cin_checkout_time
        && (p.legacy_cust_no.is_none() || ex.legacy_cust_no == p.legacy_cust_no)
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
    // COALESCE argument order matters:
    //   * `legacy_cin_no` (PK) and `aggregate_id` (write-once UUID) keep
    //     `COALESCE(existing, new)` semantics — once set, never overwritten.
    //   * `legacy_room_no`, `legacy_cust_no`, `legacy_checkin_ds_id` are
    //     denormalised pointers that MUST track the current MSSQL state.
    //     Pre-fix they used `COALESCE(existing, new)` which made them
    //     write-once, so a room change (legacy_app inserts HT_Changed_Room
    //     and rewrites HT_CheckIn_Ds.Cin_Room_No) updated the junction
    //     `ht_checkin_rooms` correctly but left `ht_checkins.legacy_room_no`
    //     pinned to the original room forever. `COALESCE(new, existing)`
    //     restores write-through semantics. The transient `first_room_no=None`
    //     scenario that the old COALESCE was guarding is already caught
    //     upstream by the `resolve_room_id` defer at line ~332 — if no room
    //     can be resolved, apply returns Ok(None) and never reaches this
    //     UPDATE, so the guard here was load-bearing only on dead code.
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
                legacy_room_no         = COALESCE($11, legacy_room_no), \
                legacy_cust_no         = COALESCE($12, legacy_cust_no), \
                legacy_checkin_ds_id   = COALESCE($13, legacy_checkin_ds_id), \
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

// -----------------------------------------------------------------------------
// ht_checkin_rooms junction (Track B2 / T2 CRIT-1)
//
// Per the audit (`docs/coexistence/audit-2026-05-13.md` Theme 1) the
// pre-B2 mapper collapsed every multi-room folio to its first room —
// rooms 2..N never landed in canonical state. B2 fans the projection's
// per-room slice out into `ht_checkin_rooms`, keyed on the junction's
// `UNIQUE (cr_cin_id, cr_room_id)`. Cardinality changes (a 3-room folio
// dropping to 2 via iHOTEL edit) are handled by an explicit DELETE of
// the room IDs that left the set.
// -----------------------------------------------------------------------------

/// Resolved per-room slice — the projection's `CanonicalRoom` plus the
/// canonical `room_id` FK. Returned by [`resolve_canonical_rooms`].
#[derive(Debug, Clone)]
struct ResolvedRoom {
    room_id: i32,
    canonical: CanonicalRoom,
}

/// Resolve `cr_room_id` for every projection room. Returns `None` if
/// any room is still missing in `ht_rooms_new` — the caller converts
/// that into an error so the watermark holds and the next tick retries
/// against a fully resolvable aggregate (rooms have no eager-mirror
/// path; the operator must run `bin/backfill_rooms`). Empty input →
/// empty output (a folio with no `HT_CheckIn_Ds` rows is legitimate
/// transient state).
async fn resolve_canonical_rooms(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    projection_rooms: &[CanonicalRoom],
) -> Result<Option<Vec<ResolvedRoom>>, SyncError> {
    let mut out = Vec::with_capacity(projection_rooms.len());
    for room in projection_rooms {
        match resolve::resolve_room_id(tx, Some(room.legacy_room_no.as_str())).await? {
            Some(id) => out.push(ResolvedRoom {
                room_id: id,
                canonical: room.clone(),
            }),
            None => {
                tracing::warn!(
                    legacy_room_no = %room.legacy_room_no,
                    "ht_checkin_rooms room not mirrored (run \
                     bin/backfill_rooms) — caller will error to hold the \
                     watermark"
                );
                return Ok(None);
            }
        }
    }
    Ok(Some(out))
}

/// UPSERT each resolved room into `ht_checkin_rooms`, then DELETE any
/// stale junction row whose `cr_room_id` is no longer in the current
/// aggregate. The DELETE scope is bound to `cr_cin_id = $1`, so other
/// folios' rows are never touched — even if two folios coincidentally
/// share a room number (impossible under the legacy `HT_CheckIn_Ds`
/// uniqueness rules but enforced defensively).
async fn apply_checkin_rooms(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_id: i32,
    resolved_rooms: &[ResolvedRoom],
) -> Result<(), SyncError> {
    // UPSERT keyed on `(cr_cin_id, cr_room_id)` — same shape as the
    // junction's `UNIQUE` constraint.
    for room in resolved_rooms {
        upsert_checkin_room(tx, cin_id, room).await?;
    }

    // DELETE rooms that dropped from the aggregate (3-room → 2-room
    // edit on the legacy side). Empty `resolved_rooms` means EVERY
    // junction row for this folio leaves — a legitimate transient
    // state during a multi-statement legacy edit, where the
    // re-projection on the next CT tick will re-insert them.
    let kept_room_ids: Vec<i32> = resolved_rooms.iter().map(|r| r.room_id).collect();
    delete_dropped_checkin_rooms(tx, cin_id, &kept_room_ids).await?;

    Ok(())
}

/// UPSERT one `ht_checkin_rooms` row. The `ON CONFLICT` clause keys on
/// the junction's `UNIQUE (cr_cin_id, cr_room_id)`.
async fn upsert_checkin_room(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_id: i32,
    room: &ResolvedRoom,
) -> Result<(), SyncError> {
    let c = &room.canonical;
    sqlx::query(
        "INSERT INTO ht_checkin_rooms ( \
             cr_cin_id, cr_room_id, cr_room_in, cr_room_out, cr_room_status, \
             cr_rate_per_night, cr_nights, cr_room_total, cr_dep_amount, \
             cr_dep_status, cr_dep_returned_at, cr_dep_returned_by, \
             cr_legacy_ds_id \
         ) VALUES ($1, $2, $3, $4, $5, $6::float8, $7, $8::float8, $9::float8, \
                   $10, $11, $12, $13) \
         ON CONFLICT (cr_cin_id, cr_room_id) DO UPDATE SET \
             cr_room_in         = EXCLUDED.cr_room_in, \
             cr_room_out        = EXCLUDED.cr_room_out, \
             cr_room_status     = EXCLUDED.cr_room_status, \
             cr_rate_per_night  = EXCLUDED.cr_rate_per_night, \
             cr_nights          = EXCLUDED.cr_nights, \
             cr_room_total      = EXCLUDED.cr_room_total, \
             cr_dep_amount      = EXCLUDED.cr_dep_amount, \
             cr_dep_status      = EXCLUDED.cr_dep_status, \
             cr_dep_returned_at = EXCLUDED.cr_dep_returned_at, \
             cr_dep_returned_by = EXCLUDED.cr_dep_returned_by, \
             cr_legacy_ds_id    = COALESCE(EXCLUDED.cr_legacy_ds_id, \
                                           ht_checkin_rooms.cr_legacy_ds_id), \
             cr_updated_at      = NOW()",
    )
    .bind(cin_id)
    .bind(room.room_id)
    .bind(c.cr_room_in)
    .bind(c.cr_room_out)
    .bind(&c.cr_room_status)
    .bind(c.cr_rate_per_night)
    .bind(c.cr_nights)
    .bind(c.cr_room_total)
    .bind(c.cr_dep_amount)
    .bind(&c.cr_dep_status)
    .bind(c.cr_dep_returned_at)
    .bind(&c.cr_dep_returned_by)
    .bind(c.cr_legacy_ds_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// DELETE every `ht_checkin_rooms` row for this folio whose
/// `cr_room_id` is NOT in `kept`. Handles the 3-room → 2-room edit case
/// (and the legitimate header-still-present-but-all-Ds-deleted case,
/// where `kept` is empty and every junction row drops).
async fn delete_dropped_checkin_rooms(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_id: i32,
    kept: &[i32],
) -> Result<(), SyncError> {
    if kept.is_empty() {
        sqlx::query("DELETE FROM ht_checkin_rooms WHERE cr_cin_id = $1")
            .bind(cin_id)
            .execute(&mut **tx)
            .await?;
    } else {
        sqlx::query(
            "DELETE FROM ht_checkin_rooms \
              WHERE cr_cin_id = $1 \
                AND cr_room_id <> ALL($2)",
        )
        .bind(cin_id)
        .bind(kept)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

/// Existing room slice as it currently lives in `ht_checkin_rooms`.
/// Pairs the `legacy_room_no` (looked up via the `ht_rooms_new` join)
/// with the per-row status so [`rooms_match`] can compare against the
/// projection's `CanonicalRoom` set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ExistingRoom {
    legacy_room_no: String,
    cr_room_status: String,
}

async fn fetch_existing_room_set(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_id: i32,
) -> Result<Vec<ExistingRoom>, SyncError> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT r.room_no, cr.cr_room_status \
           FROM ht_checkin_rooms cr \
           JOIN ht_rooms_new r ON r.room_id = cr.cr_room_id \
          WHERE cr.cr_cin_id = $1",
    )
    .bind(cin_id)
    .fetch_all(&mut **tx)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(legacy_room_no, cr_room_status)| ExistingRoom {
            legacy_room_no,
            cr_room_status,
        })
        .collect())
}

/// True when the projection's per-room set already matches what lives
/// in `ht_checkin_rooms`. Compares the SET (not the sequence) of
/// `(legacy_room_no, cr_room_status)` pairs — order-independent per
/// Track B2 / T2 HIGH-2.
fn rooms_match(existing: &[ExistingRoom], projection: &[CanonicalRoom]) -> bool {
    use std::collections::HashSet;
    if existing.len() != projection.len() {
        return false;
    }
    let ex_set: HashSet<(&str, &str)> = existing
        .iter()
        .map(|r| (r.legacy_room_no.as_str(), r.cr_room_status.as_str()))
        .collect();
    let proj_set: HashSet<(&str, &str)> = projection
        .iter()
        .map(|r| (r.legacy_room_no.as_str(), r.cr_room_status.as_str()))
        .collect();
    ex_set == proj_set
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

/// Convert a legacy MSSQL datetime — Bangkok wall-clock stored without
/// timezone info — to a real UTC instant for `DomainEvent` snapshots.
///
/// Fixed 2026-06-11 (audit P2 "timezone mislabels"): the previous
/// implementation stamped the Bangkok wall-clock digits as UTC
/// (`Utc.from_utc_datetime`), so every event snapshot was 7 hours in
/// the future. Subscribers re-fetch canonical rows for display, so no
/// stored data was wrong — but anything comparing snapshot instants
/// against real `Utc::now()` (e.g. event-lag dashboards) saw phantom
/// skew. `+07:00` is a fixed offset (Thailand has no DST), so
/// `single()` is always `Some`; the `expect` documents that invariant.
fn naive_dt_to_utc(dt: NaiveDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    let bangkok = chrono::FixedOffset::east_opt(7 * 3600).expect("+07:00 is a valid offset");
    bangkok
        .from_local_datetime(&dt)
        .single()
        .expect("fixed offsets have no DST gaps/folds")
        .with_timezone(&chrono::Utc)
}

/// Date-only variant of [`naive_dt_to_utc`]: midnight Bangkok → UTC
/// (= 17:00 the previous day). See that function for the 2026-06-11
/// mislabel fix rationale.
fn naive_date_to_utc(date: NaiveDate) -> chrono::DateTime<chrono::Utc> {
    let midnight = chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight");
    naive_dt_to_utc(date.and_time(midnight))
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
            // Track B2 — per-room operational columns the projection
            // now mirrors into `ht_checkin_rooms`. Default values mirror
            // the writeback recipe's `walkin::build_statements` payload
            // (1 night, 890.00, no deposit).
            .with("Cin_Room_Price", MockValue::Decimal(890.0))
            .with("Cin_Room_Night", MockValue::I32(1))
            .with("Cin_Room_PriceToTal", MockValue::Decimal(890.0))
            .with("Cin_Room_Dep", MockValue::Decimal(0.0))
            .with("Cin_Dep_Status", MockValue::Null)
            .with("Cin_Dep_return_date", MockValue::Null)
            .with("Cin_Dep_return_by", MockValue::Null)
            .with("Cin_Room_In", MockValue::Null)
    }

    /// Multi-room aggregates need distinct `HT_CheckIn_Ds.id` values so
    /// the per-room projection carries unique `cr_legacy_ds_id`s. The
    /// default `ds_row` helper hardcodes 25001 (single-room captures);
    /// pass a `ds_id` here to override.
    fn ds_row_with_id(cin_no: &str, room_no: &str, status: &str, ds_id: i32) -> HashMapRow {
        ds_row(cin_no, room_no, status).with("id", MockValue::I32(ds_id))
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

    // ----- NULL Cin_Date_Out tolerance (2026-06-11, audit P2) -------------

    /// A malformed legacy folio with `Cin_Date_Out IS NULL` used to
    /// hard-error projection. Under error-on-defer that would wedge the
    /// HT_CheckIn_* watermark on ONE bad folio forever. Fallback chain:
    /// max Ds `Cin_Room_Out` first…
    #[test]
    fn derive_stay_range_null_header_out_falls_back_to_ds_room_out() {
        let mut header = header_row("CH26-009999", "C21607", "ปกติ");
        header.cells.insert("Cin_Date_Out".into(), MockValue::Null);
        let rooms = vec![ds_row_with_room_out(
            "CH26-009999",
            "402",
            CIN_ROOM_STATUS_CHECKED_OUT,
            dt(2026, 6, 2, 11, 0),
        )];
        let (_, expected_out) =
            derive_stay_range(&header, &rooms).expect("NULL Cin_Date_Out must not error");
        assert_eq!(
            expected_out,
            chrono::NaiveDate::from_ymd_opt(2026, 6, 2).unwrap(),
            "checked-out Ds Cin_Room_Out is the best available fallback"
        );
    }

    /// …then `Cin_Date_in` when no Ds row carries a date either
    /// (zero-night shape — row stays mirrored and visible).
    #[test]
    fn derive_stay_range_null_header_out_falls_back_to_date_in() {
        let mut header = header_row("CH26-009998", "C21607", "ปกติ");
        header.cells.insert("Cin_Date_Out".into(), MockValue::Null);
        let rooms = vec![ds_row("CH26-009998", "402", "เข้าพัก")]; // Cin_Room_Out NULL
        let (date_in, expected_out) =
            derive_stay_range(&header, &rooms).expect("NULL Cin_Date_Out must not error");
        assert_eq!(expected_out, date_in.date());
    }

    /// NULL `Cin_Date_in` is still a hard error — there is genuinely no
    /// stay start to mirror.
    #[test]
    fn derive_stay_range_null_date_in_still_errors() {
        let mut header = header_row("CH26-009997", "C21607", "ปกติ");
        header.cells.insert("Cin_Date_in".into(), MockValue::Null);
        let err = derive_stay_range(&header, &[]).expect_err("no stay start → unusable");
        assert!(err.to_string().contains("Cin_Date_in"));
    }

    // ----- DomainEvent snapshot timezone (2026-06-11, audit P2) -----------

    /// Legacy datetimes are Bangkok wall-clock. The snapshot conversion
    /// must shift them −7h to a real UTC instant (pre-fix the digits
    /// were stamped as UTC verbatim, putting every snapshot 7h in the
    /// future).
    #[test]
    fn naive_dt_to_utc_shifts_bangkok_wall_clock_minus_seven_hours() {
        let wall = dt(2026, 6, 11, 14, 30); // 14:30 Bangkok
        let utc = naive_dt_to_utc(wall);
        assert_eq!(
            utc,
            chrono::DateTime::parse_from_rfc3339("2026-06-11T07:30:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc)
        );
    }

    /// Midnight Bangkok = 17:00 UTC the previous day.
    #[test]
    fn naive_date_to_utc_is_previous_day_seventeen_hundred_utc() {
        let utc = naive_date_to_utc(chrono::NaiveDate::from_ymd_opt(2026, 6, 11).unwrap());
        assert_eq!(
            utc,
            chrono::DateTime::parse_from_rfc3339("2026-06-10T17:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc)
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

    // ----- project_rooms (Track B2 / T2 CRIT-1) -------------------------

    /// Multi-room aggregate: projection MUST carry one `CanonicalRoom`
    /// per `HT_CheckIn_Ds` row. Pre-B2 the mapper collapsed to first
    /// room only — this test locks the headline correctness fix.
    #[test]
    fn project_rooms_emits_one_canonical_room_per_legacy_ds_row() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![
                ds_row_with_id("CH26-005228", "402", "เข้าพัก", 25001),
                ds_row_with_id("CH26-005228", "403", "เข้าพัก", 25002),
                ds_row_with_id("CH26-005228", "404", "เข้าพัก", 25003),
            ],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.rooms.len(), 3, "one CanonicalRoom per HT_CheckIn_Ds row");
        let room_nos: Vec<&str> = p.rooms.iter().map(|r| r.legacy_room_no.as_str()).collect();
        assert_eq!(room_nos, vec!["402", "403", "404"]);
        let ds_ids: Vec<Option<i32>> = p.rooms.iter().map(|r| r.cr_legacy_ds_id).collect();
        assert_eq!(ds_ids, vec![Some(25001), Some(25002), Some(25003)]);
    }

    /// Thai literal `'เข้าพัก'` (in-stay) MUST round-trip verbatim — the
    /// user constraint forbids translating per-room status into a
    /// canonical enum.
    #[test]
    fn project_rooms_preserves_thai_literal_in_room_status() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![
                ds_row_with_id("CH26-005228", "402", "เข้าพัก", 25001),
                ds_row_with_id("CH26-005228", "403", "Check-Out", 25002),
                ds_row_with_id("CH26-005228", "404", "จอง", 25003),
                ds_row_with_id("CH26-005228", "405", "ยกเลิก", 25004),
            ],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        let statuses: Vec<&str> = p.rooms.iter().map(|r| r.cr_room_status.as_str()).collect();
        assert_eq!(
            statuses,
            vec!["เข้าพัก", "Check-Out", "จอง", "ยกเลิก"],
            "every legacy literal must pass through unchanged"
        );
    }

    /// Single-room flow still works — the default `ds_row` carries the
    /// 890.00/1-night writeback recipe shape.
    #[test]
    fn project_rooms_carries_rate_nights_total() {
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![ds_row("CH26-005228", "402", "เข้าพัก")],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.rooms.len(), 1);
        let only = &p.rooms[0];
        assert_eq!(only.cr_rate_per_night, 890.0);
        assert_eq!(only.cr_nights, 1);
        assert_eq!(only.cr_room_total, 890.0);
        assert_eq!(only.cr_dep_amount, 0.0);
    }

    /// Empty `Cin_Room_No` (transient mid-edit state) must be skipped
    /// from the projection — the junction enforces `cr_room_id NOT NULL`.
    #[test]
    fn project_rooms_drops_rows_with_empty_cin_room_no() {
        let mut empty = ds_row("CH26-005228", "", "เข้าพัก");
        empty.cells.insert("id".into(), MockValue::I32(25099));
        let agg = CheckInAggregate {
            header: Some(header_row("CH26-005228", "C21607", "ปกติ")),
            rooms: vec![
                ds_row_with_id("CH26-005228", "402", "เข้าพัก", 25001),
                empty,
            ],
            payments: vec![],
        };
        let p = project_aggregate(&agg, "CH26-005228").unwrap();
        assert_eq!(p.rooms.len(), 1, "empty Cin_Room_No row must drop");
        assert_eq!(p.rooms[0].legacy_room_no, "402");
    }

    // ----- rooms_match (Track B2 / T2 HIGH-2) ----------------------------

    /// Pure room-change idempotency check — pre-B2 a swap of 402→403
    /// (same status, same amounts) was wrongly skipped because the
    /// header-level `existing_matches` matched.
    #[test]
    fn rooms_match_detects_pure_room_swap() {
        let existing = vec![ExistingRoom {
            legacy_room_no: "402".into(),
            cr_room_status: "เข้าพัก".into(),
        }];
        let projection = vec![CanonicalRoom {
            legacy_room_no: "403".into(),
            cr_room_status: "เข้าพัก".into(),
            cr_room_in: None,
            cr_room_out: None,
            cr_rate_per_night: 890.0,
            cr_nights: 1,
            cr_room_total: 890.0,
            cr_dep_amount: 0.0,
            cr_dep_status: None,
            cr_dep_returned_at: None,
            cr_dep_returned_by: None,
            cr_legacy_ds_id: Some(25001),
        }];
        assert!(
            !rooms_match(&existing, &projection),
            "room swap MUST force a re-apply (pre-B2 bug: idempotency-skipped)"
        );
    }

    #[test]
    fn rooms_match_is_order_independent() {
        let existing = vec![
            ExistingRoom {
                legacy_room_no: "402".into(),
                cr_room_status: "เข้าพัก".into(),
            },
            ExistingRoom {
                legacy_room_no: "403".into(),
                cr_room_status: "เข้าพัก".into(),
            },
        ];
        // Same set, different order.
        let projection = vec![
            CanonicalRoom {
                legacy_room_no: "403".into(),
                cr_room_status: "เข้าพัก".into(),
                cr_room_in: None,
                cr_room_out: None,
                cr_rate_per_night: 890.0,
                cr_nights: 1,
                cr_room_total: 890.0,
                cr_dep_amount: 0.0,
                cr_dep_status: None,
                cr_dep_returned_at: None,
                cr_dep_returned_by: None,
                cr_legacy_ds_id: Some(25002),
            },
            CanonicalRoom {
                legacy_room_no: "402".into(),
                cr_room_status: "เข้าพัก".into(),
                cr_room_in: None,
                cr_room_out: None,
                cr_rate_per_night: 890.0,
                cr_nights: 1,
                cr_room_total: 890.0,
                cr_dep_amount: 0.0,
                cr_dep_status: None,
                cr_dep_returned_at: None,
                cr_dep_returned_by: None,
                cr_legacy_ds_id: Some(25001),
            },
        ];
        assert!(
            rooms_match(&existing, &projection),
            "set semantics — order must not matter"
        );
    }

    /// A dropped room (3→2) MUST force a re-apply so the
    /// junction-DELETE cleanup runs.
    #[test]
    fn rooms_match_detects_dropped_room() {
        let existing = vec![
            ExistingRoom {
                legacy_room_no: "402".into(),
                cr_room_status: "เข้าพัก".into(),
            },
            ExistingRoom {
                legacy_room_no: "403".into(),
                cr_room_status: "เข้าพัก".into(),
            },
        ];
        let projection = vec![CanonicalRoom {
            legacy_room_no: "402".into(),
            cr_room_status: "เข้าพัก".into(),
            cr_room_in: None,
            cr_room_out: None,
            cr_rate_per_night: 890.0,
            cr_nights: 1,
            cr_room_total: 890.0,
            cr_dep_amount: 0.0,
            cr_dep_status: None,
            cr_dep_returned_at: None,
            cr_dep_returned_by: None,
            cr_legacy_ds_id: Some(25001),
        }];
        assert!(!rooms_match(&existing, &projection));
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
            rooms: vec![],
        }
    }

    /// Existing canonical row exactly mirroring `p` — tests mutate one
    /// field at a time.
    fn make_existing(p: &CanonicalCheckIn) -> ExistingCheckIn {
        ExistingCheckIn {
            cin_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cin_status: Some(p.cin_status.clone()),
            cin_total_amount: p.total_amount,
            cin_paid_amount: p.paid_amount,
            cin_checkout_time: p.cin_checkout_time,
            legacy_cust_no: p.legacy_cust_no.clone(),
        }
    }

    #[test]
    fn existing_matches_is_true_for_unchanged_row() {
        let p = sample_canonical();
        let ex = make_existing(&p);
        assert!(existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_is_false_when_status_differs() {
        let p = sample_canonical();
        let mut ex = make_existing(&p);
        ex.cin_status = Some("checkedout".into());
        assert!(!existing_matches(&ex, &p));
    }

    #[test]
    fn existing_matches_is_false_when_paid_amount_differs() {
        let p = sample_canonical();
        let mut ex = make_existing(&p);
        ex.cin_paid_amount = Some(100.0);
        assert!(!existing_matches(&ex, &p));
    }

    /// Audit 2026-06-11 P1 #6 — iHOTEL's customer-delete cascade
    /// (`UPDATE HT_CheckIn_H SET Cin_cust_no='C0000'`, cheatsheet
    /// §3.24) changes ONLY the customer pointer; pre-fix the
    /// status/amount-only comparison silently idempotency-skipped it.
    #[test]
    fn existing_matches_is_false_when_only_cust_no_changed() {
        let p = sample_canonical(); // legacy_cust_no = Some("C21607")
        let mut ex = make_existing(&p);
        ex.legacy_cust_no = Some("C0000".into());
        assert!(
            !existing_matches(&ex, &p),
            "a cust_no-only change MUST force a re-apply (C0000 cascade)"
        );
    }

    /// Guard: a None-cust_no projection against a populated canonical
    /// value stays idempotent — the UPDATE writes it through
    /// `COALESCE($12, legacy_cust_no)`, so it could never converge and
    /// would re-apply every tick.
    #[test]
    fn existing_matches_guards_none_cust_no_projection() {
        let mut p = sample_canonical();
        p.legacy_cust_no = None;
        let mut ex = make_existing(&p);
        ex.legacy_cust_no = Some("C21607".into());
        assert!(existing_matches(&ex, &p));
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

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards.
    //
    // The exact class of bug that drove PR #90 (`HT_CheckIn_Ds` typo →
    // nine-hour silent drop of every check-in CT row). The check-in
    // *aggregate* projection in `parent_loader::CHECKIN_DS_PROJECTION`
    // already has its own lock test there; these two guards cover the
    // *CT mapper* SELECT projections (the JOIN against CHANGETABLE).
    // -------------------------------------------------------------------

    #[test]
    fn checkin_h_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(CHECKIN_H_SELECT_COLS, "HT_CheckIn_H");
    }

    #[test]
    fn checkin_ds_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(CHECKIN_DS_SELECT_COLS, "HT_CheckIn_Ds");
    }
}
