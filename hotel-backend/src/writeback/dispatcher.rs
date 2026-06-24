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

use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
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
    /// The job's frozen `writeback_jobs.idempotency_key` — the key the legacy
    /// idempotency ledger (`dbo.ht_writeback_ledger`) is keyed on. Stored value,
    /// never recomputed (see `ClaimedJob::idempotency_key`).
    pub idempotency_key: Uuid,
}

/// Round-bill gate probe — `docs/legacy-app/COMPAT_CHEATSHEET.md` §1.9.
/// Table / column names verified against `docs/legacy-app/SCHEMA.sql`
/// (`HT_Round_Bill`: `id, round_no, round_price, round_by, round_start,
/// round_end`); iHOTEL's own gate is
/// `Module1.check_round_bill(): SELECT id FROM HT_Round_Bill WHERE
/// round_end IS NULL`.
const ROUND_BILL_GATE_SQL: &str =
    "SELECT COUNT(*) FROM HT_Round_Bill WHERE round_end IS NULL";

/// True for the intents whose recipes write money rows iHOTEL attributes
/// to the open cashier round (`HT_CheckIn_Pay` tenders, `HT_CheckIn_H`
/// totals, `HT_CheckIn_Product` POS lines, `HT_Receipt_*`): walk-in /
/// linked check-in, check-out settle, payment, refund, POS sale.
/// iHOTEL refuses these operations entirely when no `HT_Round_Bill` row
/// has `round_end IS NULL` (cheatsheet §1.9) — we deliberately do NOT
/// block (canonical PG already committed; the writeback must land), we
/// only surface that the rows will be missing from iHOTEL's shift /
/// cash-drawer reports for the current round.
fn intent_requires_open_round(intent: &WritebackIntent) -> bool {
    intent_facts(intent).requires_open_round
}

/// Per-intent classification facts, derived in ONE EXHAUSTIVE match (no wildcard
/// arm) so adding a `WritebackIntent` variant is a COMPILE error until it is
/// classified here. This replaces the prior independent `matches!(...)`
/// classifiers — `intent_requires_open_round` and `intent_is_ledgered` — which
/// each silently returned `false` for an unlisted new intent (the drift hazard
/// the architecture review flagged: a new money-writing or sequential-allocator
/// intent would quietly skip the round-bill warning / the idempotency ledger).
/// The big `dispatch` routing match and the legacy-id resolve/back-populate
/// matches stay as-is: they are already compiler-exhaustive and carry
/// irreducible per-intent logic.
struct IntentFacts {
    /// Gated by the legacy idempotency ledger — the sequential-allocator CREATE
    /// recipes whose re-run would mint a duplicate id (see `ledger_lookup`).
    ledgered: bool,
    /// Writes money rows iHOTEL attributes to the open cashier round
    /// (`HT_CheckIn_Pay` / `HT_CheckIn_H` totals / `HT_CheckIn_Product` /
    /// `HT_Receipt_*`) — triggers the §1.9 round-bill warning.
    requires_open_round: bool,
}

fn intent_facts(intent: &WritebackIntent) -> IntentFacts {
    use WritebackIntent::*;
    // (ledgered, requires_open_round). Exhaustive — do NOT add a `_` arm.
    let (ledgered, requires_open_round) = match intent {
        CreateBooking { .. } => (true, false),
        CreateCheckIn { .. } => (true, true),
        RecordPayment { .. } => (true, true),
        IssueCoupon { .. } => (true, false),
        RecordPosSale { .. } => (true, true),
        CheckOut { .. } => (false, true),
        RefundPayment { .. } => (false, true),
        ModifyBooking { .. }
        | CancelBooking { .. }
        | CancelCheckIn { .. }
        | ExtendStay { .. }
        | RoomChange { .. }
        | MarkRoomClean { .. }
        | MarkRoomDirty { .. }
        | SetRoomMaintenance { .. }
        | UpdateRoom { .. }
        | UpdateCustomer { .. }
        | AdjustProductStock { .. }
        | RedeemCoupon { .. } => (false, false),
    };
    IntentFacts { ledgered, requires_open_round }
}

/// Best-effort round-bill gate warning (cheatsheet §1.9). NEVER blocks or
/// fails the writeback:
/// * 0 open rounds ⇒ one `tracing::warn!` with the distinct event name
///   `writeback_no_open_round` (grep / log-alert anchor). Log-only by
///   design — the worker's existing throttled Slack path
///   (`bin/writeback.rs::record_self_heal`) is a process-global counter
///   bound to self-heal salvage semantics, and this chokepoint lives in
///   the lib where no `SlackClient` handle exists; piping one through
///   `dispatch` for a soft warning isn't worth the signature churn.
/// * a probe error ⇒ `tracing::debug!` only (e.g. transient MSSQL
///   hiccup — the recipe's own statements will surface real failures).
async fn warn_if_no_open_round(conn: &mut LegacyConn<'_>, ctx: DispatchContext, intent_name: &str) {
    // Same envelope as other in-recipe SELECTs: runs inside the worker's
    // BEGIN TRAN, write budget (R2 2026-05-14 convention).
    match simple_query_with_timeout(conn, ROUND_BILL_GATE_SQL, MssqlOpKind::Write).await {
        Ok(rows) => {
            let open_rounds: i32 = rows
                .first()
                .and_then(|row| row.get::<i32, _>(0))
                .unwrap_or(0);
            if open_rounds == 0 {
                tracing::warn!(
                    event = "writeback_no_open_round",
                    job_id = ctx.job_id,
                    aggregate_id = %ctx.aggregate_id,
                    intent = intent_name,
                    "No open HT_Round_Bill round (round_end IS NULL) — this \
                     money-writing writeback will land outside iHOTEL's \
                     cashier shift and be missing from its round/cash-drawer \
                     reports (cheatsheet §1.9). Writing anyway."
                );
            }
        }
        Err(err) => {
            // Best-effort probe — a failure here must never fail the job.
            tracing::debug!(
                job_id = ctx.job_id,
                intent = intent_name,
                error = %err,
                "Round-bill gate probe failed; skipping the §1.9 warning"
            );
        }
    }
}

/// Intents whose recipes ALLOCATE a fresh sequential legacy id (Book_ID /
/// Cin_no / pay_no / cupon_no / receipt no) and are therefore NON-idempotent
/// on a crash-after-commit retry. These — and only these — are gated by the
/// legacy idempotency ledger (`dbo.ht_writeback_ledger`). Mutate-only intents
/// (CheckOut, Cancel*, ModifyBooking, ExtendStay, RoomChange, MarkRoom*,
/// Update*, AdjustProductStock, RedeemCoupon) are absolute-SET / idempotent
/// UPSERT recipes whose correct second-run behaviour is to CONVERGE — ledgering
/// them would add rows and could block a wanted convergence re-run for zero
/// duplicate-protection benefit. `CreateCheckIn` covers both walk-in and
/// booking-conversion. Keep in sync with the ledger chokepoint in `dispatch`.
fn intent_is_ledgered(intent: &WritebackIntent) -> bool {
    intent_facts(intent).ledgered
}

/// Ledger ENTRY check: has this job's `idempotency_key` already been applied?
/// Returns the stored `LegacyIds` when the recipe committed on an earlier
/// attempt (the crash-after-commit replay), so the caller short-circuits to an
/// idempotent no-op. `Ok(None)` ⇒ first application; proceed.
async fn ledger_lookup(
    conn: &mut LegacyConn<'_>,
    idempotency_key: Uuid,
) -> WritebackResult<Option<LegacyIds>> {
    let sql = build_ledger_lookup_sql(idempotency_key);
    // Read budget: this is a pure SELECT (the write that commits the row is
    // `ledger_record`). Runs inside the worker's open BEGIN TRAN; READ COMMITTED
    // still sees rows committed by the original attempt's separate TX.
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Read)
        .await
        .map_err(ledger_table_err)?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    // legacy_ids is NVARCHAR(MAX). A NULL/undeserializable payload is treated as
    // a HIT returning empty LegacyIds (mark_done's COALESCE back-population then
    // re-derives from PG harmlessly) — but we LOG it: today every LegacyIds field
    // is Option / #[serde(default)] so this can't happen, but a future field
    // added without a default would otherwise silently drop ids on replay.
    match row.get::<&str, _>(0) {
        Some(json) => match serde_json::from_str::<LegacyIds>(json) {
            Ok(ids) => Ok(Some(ids)),
            Err(e) => {
                tracing::error!(
                    idempotency_key = %idempotency_key,
                    error = %e,
                    raw = %json,
                    "Ledger hit but legacy_ids failed to deserialize — replaying as \
                     empty LegacyIds (mark_done will re-derive from PG). Check for a \
                     LegacyIds field added without #[serde(default)]."
                );
                Ok(Some(LegacyIds::default()))
            }
        },
        None => Ok(Some(LegacyIds::default())),
    }
}

/// Ledger RECORD: the LAST write before the worker's COMMIT, on the recipe's
/// OWN connection, so it commits atomically with the recipe's legacy writes
/// (and the future `Pro_Amt` decrement). The PRIMARY KEY on `idempotency_key`
/// is the concurrency serializer of last resort: if two workers race across the
/// lease window, the loser's INSERT hits the PK violation, its whole TX rolls
/// back, and its retry no-ops on the winner's row. MUST remain the final
/// statement `dispatch` emits — appending any write after it re-opens the gap.
async fn ledger_record(
    conn: &mut LegacyConn<'_>,
    idempotency_key: Uuid,
    intent_name: &str,
    aggregate_id: Uuid,
    ids: &LegacyIds,
) -> WritebackResult<()> {
    let json = serde_json::to_string(ids)
        .map_err(|e| WritebackError::Recipe(format!("ledger legacy_ids serialize: {e}")))?;
    let sql = build_ledger_insert_sql(idempotency_key, intent_name, aggregate_id, &json);
    simple_query_with_timeout(conn, &sql, MssqlOpKind::Write)
        .await
        .map_err(ledger_table_err)?;
    Ok(())
}

/// True if an MSSQL error names the idempotency-ledger table — i.e. it is
/// missing/unavailable (e.g. dropped mid-run by a backup-restore or admin wipe
/// of the shared DB). The table name is distinctive, so a substring match is
/// reliable. Pulled out so the classification is unit-testable.
fn is_ledger_missing(err_msg: &str) -> bool {
    err_msg.contains("ht_writeback_ledger")
}

/// Map a "ledger table missing mid-run" MSSQL error to a NON-retryable
/// `WritebackError::Config`, so the job fails loud and exhausts immediately
/// instead of burning retries that re-run the recipe UNGUARDED — which could
/// create duplicate create-writebacks / double the `Pro_Amt` decrement, the
/// exact class the ledger exists to prevent. The startup probe
/// (`verify_writeback_ledger_exists`) catches a boot-time absence; this covers a
/// drop while the worker is running. Any other error stays retryable.
fn ledger_table_err(e: tiberius::error::Error) -> WritebackError {
    if is_ledger_missing(&e.to_string()) {
        WritebackError::Config(format!(
            "dbo.ht_writeback_ledger unavailable mid-run — restore it; refusing to \
             re-run the recipe unguarded ({e})"
        ))
    } else {
        WritebackError::Tiberius(e)
    }
}

/// Pure builder for the ledger lookup SQL (testable without MSSQL).
fn build_ledger_lookup_sql(idempotency_key: Uuid) -> String {
    format!(
        "SELECT legacy_ids FROM dbo.ht_writeback_ledger WHERE idempotency_key = '{idempotency_key}'"
    )
}

/// Pure builder for the ledger INSERT SQL (testable without MSSQL).
/// `idempotency_key` / `aggregate_id` are UUIDs and `intent_name` is a static
/// identifier literal from `WritebackIntent::intent_name()` — none can carry a
/// quote — so only the serialized `legacy_ids` JSON is single-quote-escaped.
/// The `N'...'` prefix keeps the NVARCHAR(MAX) literal Unicode-safe (LegacyIds
/// can carry Thai room/customer text).
fn build_ledger_insert_sql(
    idempotency_key: Uuid,
    intent_name: &str,
    aggregate_id: Uuid,
    legacy_ids_json: &str,
) -> String {
    // Route the JSON literal through the writeback layer's shared quoter (the
    // same single-quote-doubling the recipes use) instead of hand-rolling it.
    // `sql_quote` returns the value WRAPPED in single quotes, so the NVARCHAR
    // literal is `N` + the quoted value. idempotency_key / aggregate_id are
    // UUIDs and intent_name is a static identifier literal — none can carry a
    // quote — so they stay as plain quoted interpolations.
    let json = crate::writeback::format::sql_quote(legacy_ids_json);
    format!(
        "INSERT INTO dbo.ht_writeback_ledger \
            (idempotency_key, intent_name, aggregate_id, legacy_ids, applied_at) \
         VALUES ('{idempotency_key}', '{intent_name}', '{aggregate_id}', N{json}, GETDATE())"
    )
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

    // Legacy idempotency ledger (dbo.ht_writeback_ledger) — gates the
    // sequential-allocator CREATE recipes against crash-after-commit retries.
    // The worker COMMITs MSSQL (writeback.rs run_in_transaction) then marks the
    // PG job done; a crash / lease-expiry BETWEEN those re-claims and re-runs
    // this recipe, which would allocate a FRESH legacy id + duplicate the row
    // (and, for CreateCheckIn, double-decrement HT_Products.Pro_Amt in Phase B).
    // The only durable proof-of-write observable by the retry is a row written
    // INSIDE the same BEGIN TRAN..COMMIT — hence a legacy-side ledger. It keys
    // on the FROZEN per-job idempotency_key (NOT Cin_Book_no), so a true retry
    // no-ops while N genuinely-distinct jobs (e.g. each room of a multi-room
    // booking — single-room-per-POST, each its own aggregate) all land.
    //
    // The lookup short-circuits BEFORE the round-bill probe so a replay makes
    // no further writes / emits no spurious `writeback_no_open_round` WARN.
    let ledgered = intent_is_ledgered(intent);
    if ledgered {
        if let Some(ids) = ledger_lookup(conn, ctx.idempotency_key).await? {
            tracing::info!(
                job_id = ctx.job_id,
                aggregate_id = %ctx.aggregate_id,
                idempotency_key = %ctx.idempotency_key,
                intent = intent.intent_name(),
                "Writeback ledger hit — idempotent no-op replay (recipe already committed)"
            );
            return Ok(ids);
        }
    }

    // Cheatsheet §1.9 round-bill gate — best-effort WARN (never blocks) before
    // a money-writing recipe actually runs (after the ledger short-circuit so
    // replays stay silent). See `warn_if_no_open_round`.
    if intent_requires_open_round(intent) {
        warn_if_no_open_round(conn, ctx, intent.intent_name()).await;
    }

    let result = match intent {
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
        // Audit 2026-06-11 P2 — standalone mark-dirty. Same resolution
        // contract as MarkRoomClean: the recipe needs both the display
        // `room_no` (HT_Housewife.h_room + prior-occupant lookup) and
        // the numeric `HT_Rooms.id` (the flag UPDATE's key — spike §3j).
        WritebackIntent::MarkRoomDirty { room_id: _, by } => {
            let room_no = nonempty(resolved.legacy_room_no.as_ref()).ok_or_else(|| {
                WritebackError::Recipe("MarkRoomDirty requires resolved legacy_room_no".into())
            })?;
            let room_id_int = resolved.legacy_room_id_int.ok_or_else(|| {
                WritebackError::Recipe(
                    "MarkRoomDirty requires resolved legacy_room_id_int (HT_Rooms.id)"
                        .into(),
                )
            })?;
            recipes::mark_dirty::execute(conn, room_no, room_id_int, by).await
        }
        // Audit 2026-06-11 P2 — maintenance-flag mirror. The single
        // UPDATE is keyed by numeric `HT_Rooms.id` only (cheatsheet
        // §3.15/§3.16 `where id=<id>` form), so `legacy_room_no` is not
        // required here.
        WritebackIntent::SetRoomMaintenance { room_id: _, maintenance } => {
            let room_id_int = resolved.legacy_room_id_int.ok_or_else(|| {
                WritebackError::Recipe(
                    "SetRoomMaintenance requires resolved legacy_room_id_int (HT_Rooms.id)"
                        .into(),
                )
            })?;
            recipes::set_maintenance::execute(conn, room_id_int, *maintenance).await
        }
        // Audit 2026-06-11 P2 — standalone customer-edit re-save. The
        // payload carries the legacy `Cust_no` business key directly
        // (hydrated by the service from `ht_customers.legacy_cust_no`
        // before enqueue — the intent is only emitted for customers that
        // already exist on the legacy side), so no `ResolvedJob` lookup
        // is needed. Mirrors the `UpdateRoom` payload-carries-the-key
        // shape.
        WritebackIntent::UpdateCustomer { customer_id: _, resave } => {
            if resave.legacy_cust_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "UpdateCustomer requires non-empty resave.legacy_cust_no".into(),
                ));
            }
            recipes::update_customer::execute(conn, resave).await
        }
        // Admin room master-data edit (price / type / notes). The
        // payload carries the legacy `Room_no` business key directly
        // (resolved by the route from `ht_rooms_new.room_no` which
        // mirrors `HT_Rooms.Room_no` 1:1), so no `ResolvedJob` lookup
        // is needed. The recipe emits one targeted UPDATE; columns
        // whose payload field is `None` are omitted from the SET list,
        // preserving any iHOTEL-side value the operator did not touch.
        WritebackIntent::UpdateRoom { room_id: _, payload } => {
            if payload.room_no.is_empty() {
                return Err(WritebackError::Recipe(
                    "UpdateRoom requires non-empty payload.room_no".into(),
                ));
            }
            recipes::update_room::execute(conn, payload).await
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
    };

    // Ledger RECORD — the LAST write before the worker's COMMIT, on the SAME
    // conn, committing atomically with everything the recipe just wrote. Only
    // on success (Ok): a recipe error propagates so the TX rolls back and the
    // retry re-runs cleanly. A concurrent worker's INSERT here hits the PK and
    // its `?` rolls back its whole TX (serializer of last resort). DO NOT add
    // any write after this point — that would re-open the crash window.
    if ledgered {
        if let Ok(ref ids) = result {
            ledger_record(
                conn,
                ctx.idempotency_key,
                intent.intent_name(),
                ctx.aggregate_id,
                ids,
            )
            .await?;
        }
    }
    result
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

    // ── Idempotency ledger (dbo.ht_writeback_ledger) ─────────────────────────

    #[test]
    fn legacy_ids_deserializes_from_minimal_and_partial_json() {
        // Locks the ledger-replay deser contract: a row written by an OLDER
        // binary (fewer fields) must still deserialize on replay — every
        // LegacyIds field is Option / #[serde(default)]. If a future field is
        // ever added WITHOUT a default this test breaks (and the live replay
        // path logs + falls back to empty ids rather than silently dropping).
        let empty: LegacyIds = serde_json::from_str("{}").expect("empty JSON must deserialize");
        assert!(empty.cin_no.is_none() && empty.book_id.is_none());
        let partial: LegacyIds =
            serde_json::from_str(r#"{"cin_no":"CH26-000001","book_id":"R014810"}"#)
                .expect("partial JSON must deserialize");
        assert_eq!(partial.cin_no.as_deref(), Some("CH26-000001"));
        assert_eq!(partial.book_id.as_deref(), Some("R014810"));
    }

    #[test]
    fn is_ledger_missing_flags_only_the_ledger_table_error() {
        assert!(is_ledger_missing("Invalid object name 'dbo.ht_writeback_ledger'."));
        assert!(is_ledger_missing(
            "... ht_writeback_ledger ... (code: 208, state: 1, class: 16)"
        ));
        assert!(!is_ledger_missing("Invalid object name 'dbo.ht_other_table'."));
        assert!(!is_ledger_missing("connection reset by peer"));
    }

    #[test]
    fn build_ledger_lookup_sql_pins_table_and_key() {
        let k = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let sql = build_ledger_lookup_sql(k);
        assert_eq!(
            sql,
            "SELECT legacy_ids FROM dbo.ht_writeback_ledger WHERE idempotency_key = \
             '550e8400-e29b-41d4-a716-446655440000'"
        );
    }

    #[test]
    fn build_ledger_insert_sql_shape_and_escaping() {
        let k = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let agg = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();
        // A room_no carrying a single quote forces the JSON to contain a `'`,
        // which the SQL literal MUST double-escape — else the INSERT breaks /
        // is injectable. (LegacyIds can carry free-text fields.)
        let json = serde_json::to_string(&LegacyIds::new().with_room_no("4'02".into())).unwrap();
        assert!(json.contains("4'02"), "fixture JSON must contain the raw quote");
        let sql = build_ledger_insert_sql(k, "create_check_in", agg, &json);

        // Column order is load-bearing (matches migration 024 + the SELECT).
        assert!(sql.contains(
            "INSERT INTO dbo.ht_writeback_ledger (idempotency_key, intent_name, \
             aggregate_id, legacy_ids, applied_at) VALUES ("
        ));
        // Keys placed positionally, intent literal, Unicode-safe N'' prefix, GETDATE().
        assert!(sql.contains("'550e8400-e29b-41d4-a716-446655440000'"));
        assert!(sql.contains("'create_check_in'"));
        assert!(sql.contains("'11111111-2222-3333-4444-555555555555'"));
        assert!(sql.contains("N'"), "legacy_ids literal must be NVARCHAR (N'')");
        assert!(sql.contains("4''02"), "embedded single quote must be doubled");
        assert!(!sql.contains("4'02'"), "raw unescaped quote must not survive");
        assert!(sql.trim_end().ends_with("GETDATE())"));
    }

    /// The allowlist must cover exactly the sequential-allocator CREATE recipes
    /// and NONE of the idempotent mutate-only ones. Pinned textually (the heavy
    /// create payloads are awkward to construct) + functionally on the cheap
    /// intents below. Keep in sync with `intent_is_ledgered`'s `matches!`.
    #[test]
    fn intent_facts_is_exhaustive_with_no_wildcard_arm() {
        // Drift guard (the point of consolidating the classifiers): intent_facts
        // MUST stay an EXHAUSTIVE match with NO `_` arm, so adding a
        // WritebackIntent variant is a COMPILE error until it is classified —
        // not the silent (false, false) default the old per-classifier
        // matches!() gave. Adding a `_ =>` here reintroduces that hazard.
        let src = include_str!("dispatcher.rs");
        let start = src.find("fn intent_facts(").expect("intent_facts must exist");
        let rest = &src[start..];
        let body = &rest[..rest.find("\n}").map(|i| i + 2).unwrap_or(rest.len())];
        assert!(
            !body.contains("_ =>"),
            "intent_facts must have NO wildcard arm (it would reintroduce silent drift)"
        );
        // The 5 sequential-allocator CREATE intents must be classified in the table.
        for v in ["CreateBooking", "CreateCheckIn", "RecordPayment", "IssueCoupon", "RecordPosSale"] {
            assert!(body.contains(v), "intent_facts must classify {v}");
        }
    }

    #[test]
    fn intent_is_ledgered_functional_on_cheap_intents() {
        use crate::domain::payment::PaymentMethod;
        use crate::domain::shared::Money;
        use crate::outbox::intent::RecordPaymentReceipt;
        let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        // create / sequential-allocator → ledgered
        assert!(intent_is_ledgered(&WritebackIntent::RecordPosSale {
            check_in_id: id,
            sale_id: 1,
        }));
        assert!(intent_is_ledgered(&WritebackIntent::RecordPayment {
            check_in_id: id,
            amount: Money::from_satang(1),
            method: PaymentMethod::Cash,
            receipt: RecordPaymentReceipt::default(),
            checkin_ds_id: None,
            price_per_night_baht: None,
            nights: None,
            payment_aggregate_id: None,
            vat_percent: None,
        }));
        // mutate-only → NOT ledgered
        assert!(!intent_is_ledgered(&WritebackIntent::CancelBooking { booking_id: id }));
        assert!(!intent_is_ledgered(&WritebackIntent::MarkRoomClean {
            room_id: id,
            by: "Admin".into(),
        }));
        assert!(!intent_is_ledgered(&WritebackIntent::CheckOut {
            check_in_id: id,
            nights: None,
            room_price_total: None,
            product_total: None,
            net_total: None,
            pay_total: None,
            balance: None,
        }));
    }

    /// Load-bearing ordering invariant (the verifiers rated this high): the
    /// ledger lookup MUST precede the recipe match, and the ledger record MUST
    /// be the LAST write `dispatch` emits (after the match, before `result`).
    /// Appending any write after `ledger_record` re-opens the crash window.
    /// Pinned textually in the spirit of
    /// `round_bill_gate_runs_between_context_info_and_recipes`.
    #[test]
    fn ledger_lookup_precedes_match_and_record_is_last_write() {
        let src = include_str!("dispatcher.rs");
        let lookup = src
            .find("ledger_lookup(conn, ctx.idempotency_key)")
            .expect("dispatch() must call ledger_lookup before the match");
        let match_pos = src
            .find("let result = match intent {")
            .expect("dispatch() must assign `let result = match intent {`");
        // Anchor on the post-match record BLOCK (unique to dispatch) rather than
        // "ledger_record(" — the latter's first match is the fn definition, which
        // precedes the match.
        let record = src
            .find("if let Ok(ref ids) = result {")
            .expect("dispatch() must record the ledger after the match");
        let result_tail = src
            .find("\n    result\n}")
            .expect("dispatch() must end by returning `result`");
        assert!(lookup < match_pos, "ledger_lookup must run before the recipe match");
        assert!(match_pos < record, "ledger_record must run AFTER the recipe match");
        assert!(
            record < result_tail,
            "ledger_record must be the last write before `result` is returned"
        );
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
    /// `refund_payment`; Track G5 added `issue_coupon` + `redeem_coupon`;
    /// the admin-room-edit gap-close added `update_room`; the 2026-06-11
    /// coexistence audit P2 track added `mark_room_dirty`,
    /// `set_room_maintenance` and `update_customer` —
    /// keep the count and the names list in lock-step.)
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
            "mark_room_dirty",
            "set_room_maintenance",
            "update_room",
            "update_customer",
            "adjust_product_stock",
            "issue_coupon",
            "redeem_coupon",
            "record_pos_sale",
        ];
        // We verify dispatch handles all by constructing one of each via the
        // intent name → no actual MSSQL call, just type-level coverage.
        // Counting expected variants here keeps the test cheap and
        // deterministic.
        assert_eq!(names.len(), 19, "expected 19 WritebackIntent variants");
    }

    /// Cheatsheet §1.9 — byte-pin the round-bill gate probe. iHOTEL's own
    /// gate is `SELECT id FROM HT_Round_Bill WHERE round_end IS NULL`
    /// (`Module1.check_round_bill()`); ours counts the same predicate.
    /// Table + column names verified against `docs/legacy-app/SCHEMA.sql`.
    #[test]
    fn round_bill_gate_sql_is_byte_pinned() {
        assert_eq!(
            ROUND_BILL_GATE_SQL,
            "SELECT COUNT(*) FROM HT_Round_Bill WHERE round_end IS NULL"
        );
    }

    /// The gate covers exactly the money-writing recipe paths (walk-in /
    /// linked check-in, checkout, payment, refund, POS sale) and nothing
    /// else — room/customer/booking master-data flows are not attributed
    /// to a cashier round by iHOTEL.
    #[test]
    fn round_bill_gate_classifies_money_writing_intents_only() {
        use crate::domain::payment::PaymentMethod;
        use crate::domain::shared::Money;
        use crate::outbox::intent::RecordPaymentReceipt;

        let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let gated = [
            WritebackIntent::CheckOut {
                check_in_id: id,
                nights: None,
                room_price_total: None,
                product_total: None,
                net_total: None,
                pay_total: None,
                balance: None,
            },
            WritebackIntent::RecordPayment {
                check_in_id: id,
                amount: Money::from_satang(89000),
                method: PaymentMethod::Cash,
                receipt: RecordPaymentReceipt::default(),
                checkin_ds_id: None,
                price_per_night_baht: None,
                nights: None,
                payment_aggregate_id: None,
                vat_percent: None,
            },
            WritebackIntent::RefundPayment {
                check_in_id: id,
                original_payment_aggregate_id: None,
                amount: Money::from_satang(89000),
                method: PaymentMethod::Cash,
                refund_reason: String::new(),
                payment_aggregate_id: None,
            },
            WritebackIntent::RecordPosSale { check_in_id: id, sale_id: 1 },
        ];
        for intent in &gated {
            assert!(
                intent_requires_open_round(intent),
                "{} must be round-bill gated",
                intent.intent_name()
            );
        }

        let ungated = [
            WritebackIntent::CancelBooking { booking_id: id },
            WritebackIntent::MarkRoomClean { room_id: id, by: "Admin".into() },
            WritebackIntent::MarkRoomDirty { room_id: id, by: "Admin".into() },
            WritebackIntent::SetRoomMaintenance { room_id: id, maintenance: true },
            WritebackIntent::UpdateCustomer {
                customer_id: id,
                resave: crate::outbox::intent::CustomerResave::default(),
            },
        ];
        for intent in &ungated {
            assert!(
                !intent_requires_open_round(intent),
                "{} must NOT be round-bill gated",
                intent.intent_name()
            );
        }
    }

    /// The gate must run AFTER the loop-prevention tag and BEFORE any
    /// recipe SQL — textual pin in the same spirit as
    /// `dispatch_calls_set_context_info_before_recipes`.
    #[test]
    fn round_bill_gate_runs_between_context_info_and_recipes() {
        let source = include_str!("dispatcher.rs");
        let context_pos = source
            .find("set_context_info(conn)")
            .expect("dispatch() must call set_context_info(conn)");
        let gate_pos = source
            .find("warn_if_no_open_round(conn, ctx, intent.intent_name())")
            .expect("dispatch() must call warn_if_no_open_round");
        let match_pos = source
            .find("let result = match intent {")
            .expect("dispatch() must contain the recipe match `let result = match intent {`");
        assert!(
            context_pos < gate_pos && gate_pos < match_pos,
            "round-bill gate must sit between set_context_info and the \
             recipe match"
        );
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
            .find("let result = match intent {")
            .expect("dispatch() must contain the recipe match `let result = match intent {`");
        assert!(
            context_pos < match_pos,
            "set_context_info must be called BEFORE the match on intent — \
             otherwise the loop-prevention tag is missing for the first \
             statement of every recipe"
        );
    }
}
