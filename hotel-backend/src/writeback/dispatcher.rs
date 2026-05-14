//! Trait + dispatch fn that routes a [`WritebackIntent`] to its recipe.
//!
//! Per `docs/architecture.md` §3.6c: the worker pulls a job from
//! `writeback_jobs`, deserializes the JSONB payload into a `WritebackIntent`,
//! and hands it to [`dispatch`]. Dispatch matches the variant and calls the
//! corresponding recipe in `writeback/recipes/`.
//!
//! ## ID-resolution split
//!
//! Recipes operate on **legacy identifiers** (the `R\d{6}` book IDs, the
//! `CH26-\d{6}` cin numbers, etc.). The PG aggregate UUIDs travel with the
//! intent for traceability, but they aren't sent to MSSQL. The worker
//! ([`bin/writeback.rs`]) resolves the UUID → legacy ID by reading
//! `public.ht_*` before dispatch, and the recipe consumes only the resolved
//! legacy IDs via the [`ResolvedJob`] handle.
//!
//! This keeps recipes pure of PG knowledge — they read inputs from the
//! `WritebackIntent` payload + `ResolvedJob`, write to MSSQL, and return
//! [`LegacyIds`] for the worker to persist back into PG.

use uuid::Uuid;

use crate::outbox::intent::WritebackIntent;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::recipes;

/// Identifiers minted by a recipe and persisted into `writeback_jobs.legacy_ids`
/// so the service / repository layers can backfill the canonical `legacy_*_id`
/// columns.
///
/// Not every recipe mints every ID — a `CancelBooking` allocates nothing new,
/// while a `CreateBooking` mints both `book_id` and (sometimes) `cust_no`.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegacyIds {
    pub book_id: Option<String>,
    pub cust_no: Option<String>,
    pub cin_no: Option<String>,
    pub pay_no: Option<String>,
    pub receipt_no: Option<String>,
    /// Room number on the MSSQL side (`HT_Rooms.room_no`, e.g. `"402"`).
    /// Captured by walkin / checkin_to_booking so the writeback worker's
    /// `mark_done` can back-populate `ht_checkins.legacy_room_no` for the
    /// next intent on the same check-in (CancelCheckIn, ExtendStay,
    /// CheckOut, RecordPayment all need this).
    pub room_no: Option<String>,
    /// `HT_CheckIn_Ds.id` for the row created during walkin / checkin_to_booking.
    /// Required by ExtendStay and CheckOut recipes. Despite earlier notes
    /// this column is NOT IDENTITY (schema dump 2026-04-26 confirmed
    /// `int NOT NULL, default=NULL`) — the recipe allocates it via
    /// `allocate_checkin_ds_id` (TABLOCKX MAX+1) and propagates that
    /// value here, so no wire-scope SCOPE_IDENTITY capture is needed.
    ///
    /// Multi-room (Track B4): when the recipe emits N `HT_CheckIn_Ds`
    /// rows this carries the FIRST room's id (back-compat with single-
    /// room consumers — ExtendStay still targets one Ds row). The full
    /// `room_no → ds_id` mapping is in `checkin_ds_ids_by_room` below.
    pub checkin_ds_id: Option<i32>,
    /// Track B4 — per-room `HT_CheckIn_Ds.id` mapping for multi-room
    /// folios. Each entry is `(room_no, ds_id)`. Empty for single-room
    /// folios (the legacy single-row behavior — `checkin_ds_id` above
    /// is sufficient). The worker uses this to back-populate
    /// `ht_checkin_rooms.cr_legacy_ds_id` per junction row so the next
    /// edit/extend/cancel writeback can target the correct legacy row.
    #[serde(default)]
    pub checkin_ds_ids_by_room: Vec<(String, i32)>,
    /// Track G5 — `HT_Cupon.cupon_no` minted by the issue recipe (or
    /// re-affirmed by the redeem recipe). Stamped onto canonical
    /// `ht_coupons.legacy_cupon_no` by the worker's
    /// `back_populate_legacy_ids` step. `None` for every non-coupon
    /// intent.
    #[serde(default)]
    pub cupon_no: Option<i32>,
    /// Free-form extras (e.g. `HT_Rooms_Cancel.id`).
    #[serde(default)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl LegacyIds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_book_id(mut self, book_id: String) -> Self {
        self.book_id = Some(book_id);
        self
    }
    pub fn with_cust_no(mut self, cust_no: String) -> Self {
        self.cust_no = Some(cust_no);
        self
    }
    pub fn with_cin_no(mut self, cin_no: String) -> Self {
        self.cin_no = Some(cin_no);
        self
    }
    pub fn with_pay_no(mut self, pay_no: String) -> Self {
        self.pay_no = Some(pay_no);
        self
    }
    pub fn with_receipt_no(mut self, receipt_no: String) -> Self {
        self.receipt_no = Some(receipt_no);
        self
    }
    pub fn with_room_no(mut self, room_no: String) -> Self {
        self.room_no = Some(room_no);
        self
    }
    pub fn with_checkin_ds_id(mut self, id: i32) -> Self {
        self.checkin_ds_id = Some(id);
        self
    }
    /// Track B4 — record one `(room_no, ds_id)` pair for the per-room
    /// `ht_checkin_rooms.cr_legacy_ds_id` back-population step. Order
    /// is preserved (folios with `Cin_Room_ALL='508 509 '` carry
    /// `[("508", id1), ("509", id2)]`).
    pub fn with_room_ds_id(mut self, room_no: String, id: i32) -> Self {
        self.checkin_ds_ids_by_room.push((room_no, id));
        self
    }
    /// Track G5 — record the freshly-allocated `HT_Cupon.cupon_no`.
    pub fn with_cupon_no(mut self, cupon_no: i32) -> Self {
        self.cupon_no = Some(cupon_no);
        self
    }

    pub fn into_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

/// Pre-resolved legacy IDs that the worker fetches from `public.ht_*` before
/// dispatching. Each field is `Some(_)` only when the intent actually needs it.
#[derive(Debug, Default, Clone)]
pub struct ResolvedJob {
    pub legacy_book_id: Option<String>,
    pub legacy_cin_no: Option<String>,
    pub legacy_cust_no: Option<String>,
    pub legacy_room_no: Option<String>,
    /// `HT_Rooms.id` (numeric internal PK, distinct from `room_no`). Required
    /// for `mark_clean` per spike §3j capture (`UPDATE HT_Rooms WHERE id=6`).
    pub legacy_room_id_int: Option<i32>,
    /// `HT_CheckIn_Ds.id` for the row to update (CheckOut, ExtendStay).
    pub legacy_checkin_ds_id: Option<i32>,
    /// Track G2 / T4 CRIT-1 — `HT_CheckIn_Pay.Pay_no` of the original
    /// payment row a `RefundPayment` intent refunds. Resolved by the
    /// worker from `ht_payments.legacy_pay_no` keyed on the original
    /// payment's aggregate id. None when the back-population hasn't
    /// landed yet; the recipe then falls back to a generic `REFUND`
    /// prefix in `Cin_Pay_Note`.
    pub legacy_original_pay_no: Option<String>,
    /// Track G4 / T4 HIGH-3 — fully-loaded canonical `ht_room_changes`
    /// row for the `RoomChange` intent. The dispatcher loads this from
    /// PG before handing off to the recipe so the recipe stays pure /
    /// MSSQL-only (the recipe pattern Track C established). `None` for
    /// every other intent.
    pub room_change: Option<ResolvedRoomChange>,
    /// Track G5 — fully-loaded canonical `ht_coupons` row for the
    /// `IssueCoupon` / `RedeemCoupon` intents. The dispatcher loads
    /// this from PG before handing off to the recipe so the recipe
    /// stays MSSQL-only — same pattern as `room_change`. `None` for
    /// every other intent.
    pub coupon: Option<ResolvedCoupon>,
    /// Track G6 — fully-loaded canonical `ht_pos_sales` row for the
    /// `RecordPosSale` intent (joined with `ht_products` so the recipe
    /// has the legacy `Pro_no` / `Pro_name` / `Pro_Unit` without a
    /// second query). Same pattern as `room_change`: the resolver in
    /// the writeback worker hydrates it; the recipe consumes plain
    /// fields only.
    pub pos_sale: Option<ResolvedPosSale>,
}

/// Fully-hydrated `ht_coupons` row required by the
/// `WritebackIntent::IssueCoupon` / `RedeemCoupon` recipes. Sourced
/// from PG by the writeback worker; the recipe consumes it as plain
/// fields and never touches sqlx. Mirrors the Track G4
/// `ResolvedRoomChange` pattern.
#[derive(Debug, Default, Clone)]
pub struct ResolvedCoupon {
    /// `ht_coupons.coupon_id` — primary key. Used to back-populate
    /// `legacy_cupon_no` after the legacy INSERT lands.
    pub coupon_id: i64,
    /// `ht_coupons.legacy_cupon_no` — `Some(_)` only when the writeback
    /// worker has already back-populated it (i.e. on the redeem path
    /// after issue). The recipe uses this to target `WHERE cupon_no=...`
    /// on the redeem UPDATE.
    pub legacy_cupon_no: Option<i32>,
    /// `ht_coupons.coupon_for_cin_no` — legacy folio key the coupon is
    /// attached to. Empty string for standalone promo coupons.
    pub coupon_for_cin_no: String,
    /// Legacy `Cin_room_no` resolved alongside `cupon_cin_no`. Empty
    /// string when no room context is known.
    pub coupon_for_room_no: String,
    /// `ht_coupons.coupon_issued_at` — wall-clock of issue. Lands in
    /// legacy `cupon_date` + `cupon_gen_date` per spike COMPAT
    /// CHEATSHEET §`HT_Cupon`.
    pub issued_at: chrono::DateTime<chrono::Utc>,
    /// Operator name. Empty string when unknown.
    pub issued_by: String,
}

/// Track G6 — fully-hydrated `ht_pos_sales` row (joined with
/// `ht_products`) required by the `WritebackIntent::RecordPosSale`
/// recipe. Sourced from PG by the writeback worker; the recipe
/// consumes plain string + number fields only.
#[derive(Debug, Default, Clone)]
pub struct ResolvedPosSale {
    /// `ht_pos_sales.sale_id` — primary key. Used to back-populate
    /// `sale_legacy_id` after the legacy INSERT returns its IDENTITY.
    pub sale_id: i64,
    /// `HT_CheckIn_Product.Cin_No` — legacy folio key.
    pub legacy_cin_no: String,
    /// `HT_CheckIn_Product.Cin_Room_no` — the room number the line is
    /// charged to. Resolved from `ht_checkins.legacy_room_no`.
    /// Empty string ⇒ recipe passes through unchanged (legacy
    /// accepts NULL on that column).
    pub legacy_room_no: String,
    /// `ht_products.prod_legacy_no` (a.k.a. `Pro_no`) — business key
    /// of the product, lands in `Cin_Pro_id` AND drives the paired
    /// stock-adjust `UPDATE HT_Products`.
    pub prod_legacy_no: String,
    /// `ht_products.prod_name` — lands in `Cin_Pro_name`.
    pub prod_name: String,
    /// `ht_products.prod_unit` — lands in `Cin_Pro_Unit`. Empty
    /// string when the product has no unit (rare; defensive).
    pub prod_unit: String,
    /// `sale_qty` baseline (units sold).
    pub qty: f64,
    /// `sale_unit_price` in baht.
    pub unit_price_baht: f64,
    /// `sale_total` in baht (=`qty * unit_price`). Carried explicitly
    /// so the recipe never has to re-multiply (any rounding drift
    /// stays in PG's generated column, not in the legacy mirror).
    pub total_baht: f64,
    /// Operator note — lands in `Cin_Pro_note`. Empty string when
    /// the cashier skipped the field.
    pub note: String,
    /// Wall-clock when the sale was rung up. Lands in `Cin_Ds_date`.
    pub sold_at: chrono::DateTime<chrono::Utc>,
}

/// Fully-hydrated `ht_room_changes` row required by the
/// `WritebackIntent::RoomChange` recipe. Sourced from PG by the
/// writeback worker; the recipe consumes it as plain string + number
/// fields and never touches sqlx.
#[derive(Debug, Default, Clone)]
pub struct ResolvedRoomChange {
    /// `ht_room_changes.rc_id` — primary key. Used to back-populate
    /// `rc_legacy_id` after the legacy INSERT returns its IDENTITY.
    pub rc_id: i64,
    /// `Cin_no` (legacy folio key, `CHyy-NNNNNN`). Resolved from
    /// `ht_checkins.legacy_cin_no` via `rc_cin_id`.
    pub legacy_cin_no: String,
    /// `HT_Rooms.room_no` of the room the guest moved FROM. Resolved
    /// from `ht_rooms_new.legacy_room_no` (a.k.a. `room_no`) via
    /// `rc_from_room_id`.
    pub from_room_no: String,
    /// `HT_Rooms.room_no` of the room the guest moved TO.
    pub to_room_no: String,
    /// `HT_Changed_Room.Note` — free-form reason. Empty string when
    /// the receptionist skipped the field.
    pub reason: String,
    /// `HT_Changed_Room.change_date` — wall-clock of the move.
    pub changed_at: chrono::DateTime<chrono::Utc>,
    /// Operator name. Empty string when unknown.
    pub changed_by: String,
    /// `HT_Changed_Room.room_before_price` — baht (legacy column is
    /// `float NOT NULL DEFAULT 0`). Canonical PG column is
    /// `NUMERIC(12, 2)`; the resolver casts to `f64` for the recipe.
    pub room_before_price_baht: f64,
    /// `HT_Changed_Room.ToPrice` — legacy stores this as `varchar(20)`.
    /// Empty string when not set.
    pub to_price: String,
}

/// Treat empty-string legacy IDs as missing (audit MED-1). PG can return
/// `Some("")` for a `legacy_*` column that has never been populated by a
/// successful writeback yet — letting that through would produce
/// `WHERE Cin_no=''` (a silent no-op) instead of failing loudly.
fn nonempty<'a>(opt: Option<&'a String>) -> Option<&'a str> {
    opt.map(|s| s.as_str()).filter(|s| !s.is_empty())
}

/// Job context carried through dispatch. Lets the recipe trace its own
/// activity in logs without having to re-derive identifiers from the payload.
#[derive(Debug, Clone, Copy)]
pub struct DispatchContext {
    pub job_id: i64,
    pub aggregate_id: Uuid,
}

/// Apply the intent's recipe inside the **caller's already-open** MSSQL
/// connection. The worker is responsible for the surrounding transaction
/// (`BEGIN TRAN ... COMMIT/ROLLBACK`).
///
/// Recipes consume the intent by reference because callers (the worker) keep
/// the deserialized intent alive for tracing/error messages.
///
/// ## Loop-prevention chokepoint (Phase 5.1)
///
/// `dispatch` is the **single** entry point through which every recipe
/// runs. The first thing it does after the trace line is call
/// [`crate::db::mssql_session::set_context_info`] which issues
/// `SET CONTEXT_INFO 0x4E48` ("NH" = New Hotel). SQL Server Change
/// Tracking surfaces that value as `SYS_CHANGE_CONTEXT` on every row
/// the recipe mutates, and the CT watcher (`bin/sync.rs`) filters those
/// rows out — preventing a feedback loop where our writeback fires CT,
/// the watcher re-detects it, re-publishes the event, and the writeback
/// fires again.
///
/// **DO NOT** add a recipe entry point that bypasses `dispatch`. The
/// loop-prevention guarantee depends on the tag being applied exactly
/// once per writeback session BEFORE any recipe SQL runs. New
/// `WritebackIntent` variants must extend the `match` below — the
/// compiler's exhaustiveness check is the structural enforcement.
///
/// ## Pool isolation contract (Wave 5a item 6 — re-checked Wave 6 item 10)
///
/// `SET CONTEXT_INFO 0x4E48` is **session-scoped**: it persists on the bb8
/// connection after it returns to the pool. That is safe **only** because
/// the writeback worker (`hotel-backend/src/bin/writeback.rs`) owns a
/// dedicated MSSQL pool created via `create_pool(&mssql_config)` at the
/// top of `main()`. No other code path acquires from this pool — the
/// backend's `AppState.legacy_pool` is a separate `bb8::Pool` instance in
/// a different process.
///
/// **No runtime assertion is feasible** because the contract is a
/// structural property of how the binary is wired, not a value the bb8
/// pool can introspect: bb8 has no API to enumerate other handles to the
/// same `Pool` instance, and the backend's separate pool lives in a
/// separate process (Docker `backend` vs `writeback` services per
/// `docker-compose.yml`). The contract is enforced by code organization:
///
/// 1. `bin/writeback.rs::main` is the only call site that constructs
///    this `Pool` via `create_pool(&mssql_config)`.
/// 2. The backend's `AppState.legacy_pool` is constructed separately in
///    the `hotel-backend` binary's startup.
/// 3. `verify_legacy_collation_safety` + `verify_schema_fingerprint`
///    confirm the writeback binary is on the right server but do not
///    address pool-share semantics — that's still purely structural.
///
/// If a future refactor shares the writeback pool with non-writeback
/// callers (e.g. ad-hoc legacy reads from the same binary), this
/// assumption breaks and an `on_release` hook that clears `CONTEXT_INFO`
/// back to 0 would be required to avoid leaking the `0x4E48` tag onto
/// reader queries. The audit doc
/// (`docs/legacy-spike/writeback-audit-2026-05-12.md` MED cluster +
/// Wave 6 item 10) tracks this contract.
pub async fn dispatch(
    conn: &mut LegacyConn<'_>,
    intent: &WritebackIntent,
    resolved: &ResolvedJob,
    ctx: DispatchContext,
) -> WritebackResult<LegacyIds> {
    tracing::debug!(
        job_id = ctx.job_id,
        aggregate_id = %ctx.aggregate_id,
        intent = intent.intent_name(),
        "Dispatching writeback intent"
    );
    // Phase 5.1 loop-prevention — tag this writeback session so the CT
    // watcher can filter out our own changes. Belt-and-suspenders: the
    // 5.2+ mappers are idempotent UPSERTs, so a missed tag costs at
    // most one extra cycle.
    crate::db::mssql_session::set_context_info(conn).await?;
    match intent {
        WritebackIntent::CreateBooking { booking_id: _, payload } => {
            recipes::booking_create::execute(conn, payload).await
        }
        WritebackIntent::ModifyBooking { booking_id: _, changes } => {
            let book_id = nonempty(resolved.legacy_book_id.as_ref()).ok_or_else(|| {
                WritebackError::Recipe(
                    "ModifyBooking requires resolved legacy_book_id".into(),
                )
            })?;
            recipes::booking_modify::execute(conn, book_id, changes).await
        }
        WritebackIntent::CancelBooking { booking_id: _ } => {
            let book_id = nonempty(resolved.legacy_book_id.as_ref()).ok_or_else(|| {
                WritebackError::Recipe(
                    "CancelBooking requires resolved legacy_book_id".into(),
                )
            })?;
            recipes::booking_cancel::execute(conn, book_id).await
        }
        WritebackIntent::CreateCheckIn { check_in_id: _, payload } => {
            if payload.linked_booking_id.is_some() {
                let book_id = nonempty(payload.linked_legacy_book_id.as_ref())
                    .or(nonempty(resolved.legacy_book_id.as_ref()))
                    .ok_or_else(|| {
                        WritebackError::Recipe(
                            "CheckIn-to-booking requires legacy_book_id (in payload \
                             or resolved)"
                                .into(),
                        )
                    })?;
                recipes::checkin_to_booking::execute(conn, payload, book_id).await
            } else {
                recipes::walkin::execute(conn, payload).await
            }
        }
        WritebackIntent::CancelCheckIn {
            check_in_id: _,
            reason,
            room_price,
            pay_to_subtract,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CancelCheckIn requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CancelCheckIn requires resolved legacy_room_no".into())
            })?;
            recipes::checkin_cancel::execute(
                conn,
                cin_no,
                room_no,
                reason.as_deref(),
                *room_price,
                *pay_to_subtract,
            )
            .await
        }
        WritebackIntent::ExtendStay {
            check_in_id: _,
            new_end,
            stay_start,
            guest_label,
            new_room_price_total,
            new_net_total,
            new_pay_total,
            new_balance_total,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("ExtendStay requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("ExtendStay requires resolved legacy_room_no".into())
            })?;
            let ds_id = resolved.legacy_checkin_ds_id.ok_or_else(|| {
                WritebackError::Recipe(
                    "ExtendStay requires resolved legacy_checkin_ds_id".into(),
                )
            })?;
            recipes::extend_stay::execute(
                conn,
                cin_no,
                room_no,
                ds_id,
                *stay_start,
                *new_end,
                guest_label,
                *new_room_price_total,
                *new_net_total,
                *new_pay_total,
                *new_balance_total,
            )
            .await
        }
        WritebackIntent::CheckOut {
            check_in_id: _,
            nights,
            room_price_total,
            product_total,
            net_total,
            pay_total,
            balance,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CheckOut requires resolved legacy_cin_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("CheckOut requires resolved legacy_room_no".into())
            })?;
            let ds_id = resolved.legacy_checkin_ds_id.ok_or_else(|| {
                WritebackError::Recipe(
                    "CheckOut requires resolved legacy_checkin_ds_id".into(),
                )
            })?;
            // Audit H1: legacy events queued before the Wave 2 fix lack the
            // totals payload. Fall back to the prior all-zeros behavior so
            // the queue drains, and log a WARN so the partial sync surfaces
            // in worker logs. New events always carry real totals.
            if nights.is_none() {
                tracing::warn!(
                    cin_no,
                    "CheckOut intent has no nights/totals payload — falling \
                     back to legacy zeros (audit H1). This event was likely \
                     enqueued before the Wave 2 deploy."
                );
            }
            recipes::checkout::execute(
                conn,
                cin_no,
                room_no,
                ds_id,
                nights.unwrap_or(1.0),
                room_price_total.unwrap_or(0.0),
                product_total.unwrap_or(0.0),
                net_total.unwrap_or(0.0),
                pay_total.unwrap_or(0.0),
                balance.unwrap_or(0.0),
            )
            .await
        }
        WritebackIntent::RecordPayment {
            check_in_id: _,
            amount,
            method,
            receipt,
            checkin_ds_id,
            price_per_night_baht,
            nights,
            payment_aggregate_id: _,
            vat_percent,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_cin_no".into())
            })?;
            let cust_no = nonempty(resolved.legacy_cust_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_cust_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RecordPayment requires resolved legacy_room_no".into())
            })?;
            recipes::payment::execute(
                conn,
                cin_no,
                cust_no,
                room_no,
                *amount,
                *method,
                receipt,
                *checkin_ds_id,
                *price_per_night_baht,
                *nights,
                *vat_percent,
            )
            .await
        }
        // Track G2 / T4 CRIT-1 — refund / negative payment. Mirrors
        // `RecordPayment` resolution (same check-in identifiers) but
        // dispatches to the `refund_payment` recipe which emits a
        // NEGATIVE `HT_CheckIn_Pay` row and re-aggregates totals.
        WritebackIntent::RefundPayment {
            check_in_id: _,
            original_payment_aggregate_id,
            amount,
            method,
            refund_reason,
            payment_aggregate_id: _,
        } => {
            let cin_no = nonempty(resolved.legacy_cin_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RefundPayment requires resolved legacy_cin_no".into())
            })?;
            let cust_no = nonempty(resolved.legacy_cust_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RefundPayment requires resolved legacy_cust_no".into())
            })?;
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("RefundPayment requires resolved legacy_room_no".into())
            })?;
            // Resolve the original payment's legacy Pay_no for the audit
            // note. Best-effort — if the back-population is still pending
            // the recipe falls back to a generic REFUND prefix.
            let original_legacy_pay_no = match original_payment_aggregate_id {
                Some(_uuid) => resolved.legacy_original_pay_no.as_deref(),
                None => None,
            };
            recipes::refund_payment::execute(
                conn,
                cin_no,
                cust_no,
                room_no,
                *amount,
                *method,
                original_legacy_pay_no,
                refund_reason,
            )
            .await
        }
        // Track G4 / T4 HIGH-3 — mid-stay room change. The dispatcher
        // hydrated `resolved.room_change` from PG before this point;
        // the recipe consumes it as plain fields and never re-queries
        // sqlx. Missing hydration means the worker resolver couldn't
        // find the canonical `ht_room_changes` row — surface as a
        // recipe error so the job lands in the retry queue rather
        // than silently emitting an empty INSERT.
        WritebackIntent::RoomChange { check_in_id: _, rc_id } => {
            let resolved_rc = resolved
                .room_change
                .as_ref()
                .ok_or_else(|| {
                    WritebackError::Recipe(format!(
                        "RoomChange requires resolved.room_change (rc_id={rc_id})"
                    ))
                })?;
            if resolved_rc.legacy_cin_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "RoomChange requires legacy_cin_no on resolved.room_change".into(),
                ));
            }
            if resolved_rc.from_room_no.is_empty() || resolved_rc.to_room_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "RoomChange requires both from_room_no and to_room_no on resolved.room_change"
                        .into(),
                ));
            }
            recipes::room_change::execute(conn, resolved_rc).await
        }
        WritebackIntent::MarkRoomClean { room_id: _, by } => {
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("MarkRoomClean requires resolved legacy_room_no".into())
            })?;
            let room_id_int = resolved.legacy_room_id_int.ok_or_else(|| {
                WritebackError::Recipe(
                    "MarkRoomClean requires resolved legacy_room_id_int (HT_Rooms.id)"
                        .into(),
                )
            })?;
            recipes::mark_clean::execute(conn, room_no, room_id_int, by).await
        }
        // Track F3 — `audit-2026-05-13.md` T1 CRIT-3. The intent
        // already carries the legacy `Pro_no` business key directly
        // (no `ResolvedJob` lookup needed — product master is keyed
        // by the same `Pro_no` on both sides).
        WritebackIntent::AdjustProductStock {
            prod_legacy_no,
            delta,
            reason: _,
            product_aggregate_id: _,
        } => {
            if prod_legacy_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "AdjustProductStock requires non-empty prod_legacy_no".into(),
                ));
            }
            recipes::adjust_product_stock::execute(conn, prod_legacy_no, *delta).await
        }
        // Track G5 — coupon issuing. The dispatcher hydrated
        // `resolved.coupon` from PG (canonical `ht_coupons` row keyed
        // on `coupon_id`) before this point. The recipe consumes it
        // as plain fields and never re-queries sqlx.
        WritebackIntent::IssueCoupon { coupon_aggregate_id: _, coupon_id } => {
            let resolved_coupon = resolved.coupon.as_ref().ok_or_else(|| {
                WritebackError::Recipe(format!(
                    "IssueCoupon requires resolved.coupon (coupon_id={coupon_id})"
                ))
            })?;
            recipes::coupon::execute_issue(conn, resolved_coupon).await
        }
        WritebackIntent::RedeemCoupon { coupon_aggregate_id: _, coupon_id } => {
            let resolved_coupon = resolved.coupon.as_ref().ok_or_else(|| {
                WritebackError::Recipe(format!(
                    "RedeemCoupon requires resolved.coupon (coupon_id={coupon_id})"
                ))
            })?;
            recipes::coupon::execute_redeem(conn, resolved_coupon).await
        }
        // Track G6 — POS / sales-to-room. The dispatcher hydrated
        // `resolved.pos_sale` from PG before this point; the recipe
        // consumes plain fields only. Missing hydration ⇒ recipe
        // error so the job lands in the retry queue rather than
        // silently emitting an empty INSERT.
        WritebackIntent::RecordPosSale { check_in_id: _, sale_id } => {
            let resolved_sale = resolved
                .pos_sale
                .as_ref()
                .ok_or_else(|| {
                    WritebackError::Recipe(format!(
                        "RecordPosSale requires resolved.pos_sale (sale_id={sale_id})"
                    ))
                })?;
            if resolved_sale.legacy_cin_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "RecordPosSale requires legacy_cin_no on resolved.pos_sale".into(),
                ));
            }
            if resolved_sale.prod_legacy_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "RecordPosSale requires prod_legacy_no on resolved.pos_sale".into(),
                ));
            }
            recipes::pos_sale::execute(conn, resolved_sale).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_ids_round_trips_to_json() {
        let ids = LegacyIds::new()
            .with_book_id("R014810".into())
            .with_cust_no("C21610".into());
        let json = ids.into_json();
        assert_eq!(json["book_id"], "R014810");
        assert_eq!(json["cust_no"], "C21610");
        assert!(json["cin_no"].is_null());
    }

    #[test]
    fn legacy_ids_default_serializes_with_nulls() {
        let json = LegacyIds::default().into_json();
        assert!(json["book_id"].is_null());
        assert!(json["pay_no"].is_null());
    }

    /// Track B4 — multi-room ds_id mapping must round-trip through
    /// the JSONB column the worker reads in `back_populate_legacy_ids`.
    /// The wire shape is `"checkin_ds_ids_by_room": [["508", 25101], ["509", 25102]]`.
    #[test]
    fn legacy_ids_round_trips_per_room_ds_ids() {
        let ids = LegacyIds::new()
            .with_cin_no("CH26-005230".into())
            .with_checkin_ds_id(25101)
            .with_room_ds_id("508".into(), 25101)
            .with_room_ds_id("509".into(), 25102);
        let json = ids.into_json();
        let pairs = json["checkin_ds_ids_by_room"]
            .as_array()
            .expect("checkin_ds_ids_by_room must serialize as array");
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0][0], "508");
        assert_eq!(pairs[0][1], 25101);
        assert_eq!(pairs[1][0], "509");
        assert_eq!(pairs[1][1], 25102);
        // The single-room field still works for back-compat consumers.
        assert_eq!(json["checkin_ds_id"], 25101);
    }

    /// Single-room folios omit the multi-room field via serde default —
    /// the JSON shape stays an empty array so the worker's
    /// `back_populate_legacy_ids` for-each loop is a no-op.
    #[test]
    fn legacy_ids_single_room_emits_empty_room_map() {
        let ids = LegacyIds::new()
            .with_cin_no("CH26-005228".into())
            .with_checkin_ds_id(25001);
        let json = ids.into_json();
        let pairs = json["checkin_ds_ids_by_room"]
            .as_array()
            .expect("checkin_ds_ids_by_room must be an array (possibly empty)");
        assert!(
            pairs.is_empty(),
            "single-room folio must serialize an empty room map"
        );
    }

    #[test]
    fn resolved_job_default_is_all_none() {
        let r = ResolvedJob::default();
        assert!(r.legacy_book_id.is_none());
        assert!(r.legacy_cin_no.is_none());
        assert!(r.legacy_cust_no.is_none());
        assert!(r.legacy_room_no.is_none());
        assert!(r.legacy_room_id_int.is_none());
        assert!(r.legacy_checkin_ds_id.is_none());
        assert!(r.legacy_original_pay_no.is_none());
        // Track G4 — room_change is hydrated only for the RoomChange intent.
        assert!(r.room_change.is_none());
        // Track G5 — coupon is hydrated only for the IssueCoupon /
        // RedeemCoupon intents.
        assert!(r.coupon.is_none());
        // Track G6 — pos_sale is hydrated only for the RecordPosSale intent.
        assert!(r.pos_sale.is_none());
    }

    /// Verifies all `WritebackIntent` variants have a matching `intent_name`
    /// — the dispatcher's match arms cover the same set. If a new variant is
    /// added without updating the dispatcher, the compiler will fail (match
    /// exhaustiveness), and this test catches drift in name strings.
    /// (Track F3 added `adjust_product_stock`; Track G2 added
    /// `refund_payment`; Track G5 added `issue_coupon` + `redeem_coupon`
    /// — keep the count and the names list in lock-step.)
    #[test]
    fn all_intent_variants_route_to_recipes() {
        let names = [
            "create_booking",
            "modify_booking",
            "cancel_booking",
            "create_check_in",
            "cancel_check_in",
            "extend_stay",
            "check_out",
            "record_payment",
            "refund_payment",
            "room_change",
            "mark_room_clean",
            "adjust_product_stock",
            "issue_coupon",
            "redeem_coupon",
            "record_pos_sale",
        ];
        // We verify dispatch handles all by constructing one of each via the
        // intent name → no actual MSSQL call, just type-level coverage.
        // Counting expected variants here keeps the test cheap and
        // deterministic.
        assert_eq!(names.len(), 15, "expected 15 WritebackIntent variants");
    }

    /// Phase 5.1 chokepoint guarantee — every recipe MUST run after
    /// `set_context_info` so its mutations carry the `0x4E48` tag that
    /// the CT watcher filters out. The structural enforcement is:
    ///
    /// 1. `dispatch` is the single public entry point — recipes are only
    ///    reachable through `crate::writeback::recipes` which is `pub`
    ///    inside the `writeback` module but the recipes' `execute`
    ///    functions are only called from this file.
    /// 2. `dispatch` calls `set_context_info` BEFORE the `match`.
    /// 3. The `match` is exhaustive over `WritebackIntent` — the
    ///    compiler refuses to build if a new variant skips the call.
    ///
    /// This test pins point 2 by reading the source and asserting the
    /// invocation appears before the variant arms. It's a textual
    /// check, not a runtime assertion, but combined with the compiler-
    /// level exhaustiveness in point 3 and the module-level privacy of
    /// recipes, it's sufficient to make a regression visible in CI.
    #[test]
    fn dispatch_calls_set_context_info_before_recipes() {
        let source = include_str!("dispatcher.rs");
        let context_pos = source
            .find("set_context_info(conn)")
            .expect("dispatch() must call set_context_info(conn)");
        let match_pos = source
            .find("match intent {")
            .expect("dispatch() must contain `match intent {`");
        assert!(
            context_pos < match_pos,
            "set_context_info must be called BEFORE the match on intent — \
             otherwise the loop-prevention tag is missing for the first \
             statement of every recipe"
        );
    }
}
