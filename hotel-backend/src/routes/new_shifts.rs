//! Shift API routes — Track F2 / T1 HIGH-5 (audit-2026-05-13.md).
//!
//! - `POST /api/new/shifts/open`     — open a cashier shift (one-per-site)
//! - `POST /api/new/shifts/close`    — close the open shift, return summary
//! - `GET  /api/new/shifts/current`  — peek at the open shift, 404 if none
//! - `GET  /api/new/shifts?limit=N`  — list recent shifts (default 50)
//!
//! All four endpoints delegate to `state.shifts_service`, which is bound
//! at startup to this binary's `SITE_ID` so per-site rounds stay isolated.
//! The payment gate in [`crate::service::PaymentService::record_payment`]
//! consults the same service handle, so closing a shift here immediately
//! locks the cash drawer to subsequent payment attempts.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::service::{CloseShiftCommand, OpenShiftCommand, Shift, ShiftSummary};

// =============================================================================
// Request / response shapes
// =============================================================================

/// Body for `POST /api/new/shifts/open`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenShiftRequest {
    /// Cashier opening the round. Required, non-blank after trim.
    pub opened_by: String,
    /// Opening cash float (baht). Must be `>= 0`.
    #[serde(default)]
    pub opening_float: f64,
    /// Optional free-form notes (variance, expected closer, etc.).
    #[serde(default)]
    pub notes: Option<String>,
}

/// Body for `POST /api/new/shifts/close`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseShiftRequest {
    pub closed_by: String,
    #[serde(default)]
    pub notes: Option<String>,
    /// Track J7c — optional physical cash-drawer count: a
    /// `{denomination: count}` map (e.g. `{"1000": 5, "100": 12}`). The
    /// service computes the counted total; the report shows the variance.
    #[serde(default)]
    pub cash_count: Option<serde_json::Value>,
}

/// `GET /api/new/shifts?limit=N` query string.
#[derive(Debug, Deserialize)]
pub struct ListShiftsQuery {
    /// Cap on rows returned. Clamped to `[1, 200]` by the service.
    pub limit: Option<i64>,
}

/// `GET /api/new/shifts/current?branch=<hfhotel|hfville|all>` query string.
#[derive(Debug, Deserialize)]
pub struct CurrentShiftQuery {
    /// Which site's open round to read. Defaults to HF Hotel.
    pub branch: Option<Branch>,
}

/// 201 body for a successful `open_shift`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenShiftResponse {
    pub success: bool,
    pub shift_id: i64,
    pub shift_no: i32,
}

/// 200 body for a successful `close_shift`. Mirrors
/// [`ShiftSummary`] field-for-field so the cashier UI can render the
/// cash-drawer reconciliation directly.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseShiftResponse {
    pub success: bool,
    pub shift_id: i64,
    pub shift_no: i32,
    pub opening_float: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
    pub payment_count: i64,
    pub total_collected: f64,
}

impl From<ShiftSummary> for CloseShiftResponse {
    fn from(s: ShiftSummary) -> Self {
        Self {
            success: true,
            shift_id: s.shift_id,
            shift_no: s.shift_no,
            opening_float: s.opening_float,
            opened_at: s.opened_at,
            closed_at: s.closed_at,
            payment_count: s.payment_count,
            total_collected: s.total_collected,
        }
    }
}

/// Single shift row in API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShiftDto {
    pub shift_id: i64,
    pub site_id: String,
    pub shift_no: i32,
    pub opening_float: f64,
    pub opened_by: String,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<String>,
    pub legacy_round_id: Option<i32>,
    pub notes: Option<String>,
}

impl From<Shift> for ShiftDto {
    fn from(s: Shift) -> Self {
        Self {
            shift_id: s.shift_id,
            site_id: s.site_id,
            shift_no: s.shift_no,
            opening_float: s.opening_float,
            opened_by: s.opened_by,
            opened_at: s.opened_at,
            closed_at: s.closed_at,
            closed_by: s.closed_by,
            legacy_round_id: s.legacy_round_id,
            notes: s.notes,
        }
    }
}

/// 200 body for `GET /api/new/shifts/current` (when a shift is open).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentShiftResponse {
    pub success: bool,
    pub shift: ShiftDto,
}

/// 200 body for `GET /api/new/shifts`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListShiftsResponse {
    pub success: bool,
    pub data: Vec<ShiftDto>,
}

// =============================================================================
// Handlers
// =============================================================================

/// `POST /api/new/shifts/open` — open a new cashier shift.
///
/// Returns `201 Created` with the new `shift_id` and per-site `shift_no`.
/// `400` when the body fails validation (blank `openedBy`, negative
/// float); `409`-equivalent (mapped to 400 today via
/// `ServiceError::Conflict → ApiError::BadRequest`) when a shift is
/// already open for this site.
pub async fn open_shift(
    State(state): State<AppState>,
    Json(body): Json<OpenShiftRequest>,
) -> ApiResult<(StatusCode, Json<OpenShiftResponse>)> {
    let outcome = state
        .shifts_service
        .open_shift(OpenShiftCommand {
            opened_by: body.opened_by,
            opening_float: body.opening_float,
            notes: body.notes,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(OpenShiftResponse {
            success: true,
            shift_id: outcome.shift_id,
            shift_no: outcome.shift_no,
        }),
    ))
}

/// `POST /api/new/shifts/close` — close the currently-open shift.
///
/// Returns `200 OK` with a cash-drawer reconciliation summary. `404`
/// when no shift is currently open for this site.
pub async fn close_shift(
    State(state): State<AppState>,
    Json(body): Json<CloseShiftRequest>,
) -> ApiResult<Json<CloseShiftResponse>> {
    let summary = state
        .shifts_service
        .close_shift(CloseShiftCommand {
            closed_by: body.closed_by,
            notes: body.notes,
            cash_count: body.cash_count,
        })
        .await?;

    Ok(Json(CloseShiftResponse::from(summary)))
}

/// `GET /api/new/shifts/current?branch=<…>` — return the currently-open shift.
///
/// Returns `404 Not Found` when no shift is open (rather than `200`
/// with a null body) so the cashier UI can branch on the HTTP status
/// without parsing the response.
///
/// Branch-aware: HF Ville's open round lives in the `hotelville`
/// canonical pool, HF Hotel's in the primary pool. Site-scoping is
/// connection-level (each logical DB holds exactly one site's shifts —
/// see the `canonical_pg_split` design), so the pool selection *is* the
/// site filter and `WHERE shift_closed_at IS NULL` returns that site's
/// open round (the `ht_shifts_one_open_per_site` partial unique index
/// guarantees at most one). Reads run as a runtime `sqlx::query` on the
/// resolved pool rather than via `shifts_service` (which is hardwired to
/// the primary pool at startup), so the same handler serves both sites.
pub async fn current_shift(
    State(state): State<AppState>,
    Query(query): Query<CurrentShiftQuery>,
) -> ApiResult<Json<CurrentShiftResponse>> {
    let pool = match query.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    let row = sqlx::query(
        "SELECT shift_id, shift_site_id, shift_no, \
                shift_opening_float::float8 AS shift_opening_float, \
                shift_opened_by, shift_opened_at, shift_closed_at, \
                shift_closed_by, shift_legacy_round_id, shift_notes \
           FROM ht_shifts \
          WHERE shift_closed_at IS NULL \
          ORDER BY shift_opened_at DESC \
          LIMIT 1",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to read current shift: {e}")))?
    .ok_or_else(|| ApiError::NotFound("no open shift for this site".to_string()))?;

    let shift = ShiftDto {
        shift_id: row.try_get("shift_id").map_err(map_shift_row_err)?,
        site_id: row.try_get("shift_site_id").map_err(map_shift_row_err)?,
        shift_no: row.try_get("shift_no").map_err(map_shift_row_err)?,
        opening_float: row.try_get("shift_opening_float").map_err(map_shift_row_err)?,
        opened_by: row.try_get("shift_opened_by").map_err(map_shift_row_err)?,
        opened_at: row.try_get("shift_opened_at").map_err(map_shift_row_err)?,
        closed_at: row.try_get("shift_closed_at").map_err(map_shift_row_err)?,
        closed_by: row.try_get("shift_closed_by").map_err(map_shift_row_err)?,
        legacy_round_id: row.try_get("shift_legacy_round_id").map_err(map_shift_row_err)?,
        notes: row.try_get("shift_notes").map_err(map_shift_row_err)?,
    };

    Ok(Json(CurrentShiftResponse {
        success: true,
        shift,
    }))
}

/// Map a column-decode failure on the `ht_shifts` read to a 500. A
/// decode error here means the schema drifted from `ShiftDto`, not a
/// client mistake — surface it as an internal error.
fn map_shift_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map shift row: {e}"))
}

/// `GET /api/new/shifts?limit=N` — most-recent shifts, newest first.
///
/// Defaults `limit` to 50 when unspecified. The service clamps the
/// effective limit to `[1, 200]` so a runaway client cannot DoS the
/// dashboard.
pub async fn list_shifts(
    State(state): State<AppState>,
    Query(query): Query<ListShiftsQuery>,
) -> ApiResult<Json<ListShiftsResponse>> {
    let limit = query.limit.unwrap_or(50);
    let rows = state.shifts_service.recent_shifts(limit).await?;
    let data = rows.into_iter().map(ShiftDto::from).collect();

    Ok(Json(ListShiftsResponse {
        success: true,
        data,
    }))
}

// =============================================================================
// Track J7b — round report (income by tender + sales-by-category).
//
// Computed from canonical `ht_payment_ledger` (the per-line mirror of legacy
// `HT_CheckIn_Pay`, populated by the sync worker's PaymentMapper) over the
// round's window. Mirrors the NUMBERS in iHOTEL's `ReportShipCash` /
// `ReportIncome2` (tender split + sales summary) in a clean layout — the
// ledger holds BOTH apps' payments, so the totals match what iHOTEL reports.
// =============================================================================

/// `GET /api/shifts/{shift_id}/report?branch=<…>` query string.
#[derive(Debug, Deserialize)]
pub struct RoundReportQuery {
    pub branch: Option<Branch>,
}

/// Income split by payment tender over the round window. Amounts in baht;
/// refunds net via negative tenders (legacy negation convention). `total` is
/// `SUM(ledger_amount)` = `cash+credit+free+transfer+web` (the legacy
/// per-line invariant) so it reconciles against the tender sum.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenderBreakdown {
    /// `round_price_rec` — cash RECEIVED (`ledger_cash` where `> 0`).
    pub cash_received: f64,
    /// `round_price_pay` — cash PAID OUT / refunds (`-SUM(ledger_cash)` where
    /// `< 0`, as a positive number). iHOTEL splits cash this way.
    pub cash_paid: f64,
    /// `cash_received - cash_paid` (net cash that actually entered the drawer).
    pub cash_net: f64,
    /// `round_price_credit` — credit-card (`ledger_credit` where `> 0`, matching
    /// iHOTEL's positive-only filter).
    pub credit: f64,
    /// `round_price_tran` — bank transfer (`ledger_tran` where `> 0`).
    pub transfer: f64,
    /// Discount/comp tender (`ledger_free`). Extra vs iHOTEL's round screen
    /// (it folds these into the line total); surfaced here so no info is lost.
    pub free: f64,
    /// Online/web tender (`ledger_web`). Extra vs iHOTEL's round screen.
    pub web: f64,
    /// Active ledger lines counted in the window.
    pub line_count: i64,
    /// Distinct `Pay_no` values (≈ number of payment receipts) in the window.
    pub payment_count: i64,
}

/// Room deposits over the round window — iHOTEL's `Dep_Rec` / `Dep_pay`
/// (from `HT_CheckIn_Ds.Cin_room_dep`; our `ht_checkin_rooms.cr_dep_amount`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositSummary {
    /// `Dep_Rec` — deposits TAKEN this round (room checked in within the window).
    pub received: f64,
    /// `Dep_pay` — deposits RETURNED this round (`cr_dep_returned_at` in window).
    pub returned: f64,
}

/// One sales-summary row — `room` (legacy `Cin_Pay_Ds_ID='P001'`) vs `product`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesCategory {
    pub category: String,
    pub amount: f64,
    pub lines: i64,
}

/// `200` body for the round report.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundReportResponse {
    pub success: bool,
    pub shift: ShiftDto,
    /// Window actually summed: `[opened_at, closed_at or now)`.
    pub window_from: DateTime<Utc>,
    pub window_to: DateTime<Utc>,
    /// True when the round is still open (upper bound = `now()`, a live preview).
    pub open: bool,
    pub income: TenderBreakdown,
    /// Room deposits received / returned this round (iHOTEL `Dep_Rec`/`Dep_pay`).
    pub deposits: DepositSummary,
    /// iHOTEL's round "total" (`FrmDueBill` Labeltotal): opening float +
    /// cash_received - cash_paid + credit + transfer.
    pub grand_total: f64,
    pub sales: Vec<SalesCategory>,
    /// Cash the drawer SHOULD hold: `shift_opening_float + net cash` over the
    /// window (only the cash tender touches the drawer).
    pub expected_cash: f64,
    /// Physical cash the cashier counted at close (server-computed from the
    /// denomination map). `None` until a close supplies a count.
    pub counted_cash: Option<f64>,
    /// `counted_cash - expected_cash` (over/short). `None` when uncounted.
    pub cash_variance: Option<f64>,
}

/// `GET /api/shifts/{shift_id}/report` — income-by-tender + sales summary for
/// a round, computed from canonical `ht_payment_ledger` over the shift's
/// window. Works for an open round (preview to `now()`) or a closed one.
/// Branch-aware (per-site pool), same as [`current_shift`].
pub async fn round_report(
    State(state): State<AppState>,
    Path(shift_id): Path<i64>,
    Query(query): Query<RoundReportQuery>,
) -> ApiResult<Json<RoundReportResponse>> {
    let pool = match query.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    let row = sqlx::query(
        "SELECT shift_id, shift_site_id, shift_no, \
                shift_opening_float::float8 AS shift_opening_float, \
                shift_opened_by, shift_opened_at, shift_closed_at, \
                shift_closed_by, shift_legacy_round_id, shift_notes, \
                shift_counted_cash::float8 AS counted_cash \
           FROM ht_shifts WHERE shift_id = $1",
    )
    .bind(shift_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to read shift: {e}")))?
    .ok_or_else(|| ApiError::NotFound(format!("shift {shift_id} not found")))?;

    let counted_cash: Option<f64> = row.try_get("counted_cash").map_err(map_shift_row_err)?;

    let shift = ShiftDto {
        shift_id: row.try_get("shift_id").map_err(map_shift_row_err)?,
        site_id: row.try_get("shift_site_id").map_err(map_shift_row_err)?,
        shift_no: row.try_get("shift_no").map_err(map_shift_row_err)?,
        opening_float: row.try_get("shift_opening_float").map_err(map_shift_row_err)?,
        opened_by: row.try_get("shift_opened_by").map_err(map_shift_row_err)?,
        opened_at: row.try_get("shift_opened_at").map_err(map_shift_row_err)?,
        closed_at: row.try_get("shift_closed_at").map_err(map_shift_row_err)?,
        closed_by: row.try_get("shift_closed_by").map_err(map_shift_row_err)?,
        legacy_round_id: row.try_get("shift_legacy_round_id").map_err(map_shift_row_err)?,
        notes: row.try_get("shift_notes").map_err(map_shift_row_err)?,
    };

    let window_from = shift.opened_at;
    let open = shift.closed_at.is_none();
    let window_to = shift.closed_at.unwrap_or_else(Utc::now);

    // Income by tender. `ledger_pay_date` and the window bounds are all
    // TIMESTAMPTZ (UTC) — the ledger mapper converts the legacy Thai-naive
    // Cin_Pay_Date the same way ht_shifts converts round_start/end, so the
    // comparison is apples-to-apples. Active lines only ('1'); cancelled
    // ('ยกเลิก') excluded — matching iHOTEL's shift report.
    // Cash is split RECEIVED (>0) vs PAID-OUT (<0, negated) exactly like
    // iHOTEL's `View_RBill_H` (`round_price_rec` / `round_price_pay`);
    // credit/transfer use iHOTEL's positive-only filter (`round_price_credit`
    // / `round_price_tran`). free/web are summed net as our extras.
    let inc = sqlx::query(
        "SELECT COALESCE(SUM(ledger_cash) FILTER (WHERE ledger_cash > 0),0)::float8     AS cash_received, \
                COALESCE(-SUM(ledger_cash) FILTER (WHERE ledger_cash < 0),0)::float8    AS cash_paid, \
                COALESCE(SUM(ledger_credit) FILTER (WHERE ledger_credit > 0),0)::float8 AS credit, \
                COALESCE(SUM(ledger_tran) FILTER (WHERE ledger_tran > 0),0)::float8     AS transfer, \
                COALESCE(SUM(ledger_free),0)::float8   AS free, \
                COALESCE(SUM(ledger_web),0)::float8    AS web, \
                COUNT(*)::bigint                       AS line_count, \
                COUNT(DISTINCT ledger_pay_no)::bigint  AS payment_count \
           FROM ht_payment_ledger \
          WHERE ledger_status = '1' \
            AND ledger_pay_date >= $1 AND ledger_pay_date < $2",
    )
    .bind(window_from)
    .bind(window_to)
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to sum round income: {e}")))?;

    let cash_received: f64 = inc.try_get("cash_received").map_err(map_shift_row_err)?;
    let cash_paid: f64 = inc.try_get("cash_paid").map_err(map_shift_row_err)?;
    let credit: f64 = inc.try_get("credit").map_err(map_shift_row_err)?;
    let transfer: f64 = inc.try_get("transfer").map_err(map_shift_row_err)?;
    let income = TenderBreakdown {
        cash_received,
        cash_paid,
        cash_net: cash_received - cash_paid,
        credit,
        transfer,
        free: inc.try_get("free").map_err(map_shift_row_err)?,
        web: inc.try_get("web").map_err(map_shift_row_err)?,
        line_count: inc.try_get("line_count").map_err(map_shift_row_err)?,
        payment_count: inc.try_get("payment_count").map_err(map_shift_row_err)?,
    };

    // Room deposits (iHOTEL `Dep_Rec`/`Dep_pay`): from `ht_checkin_rooms`
    // (mirror of `HT_CheckIn_Ds.Cin_room_dep`), not the payment ledger.
    // Received = room checked in within the window; returned = deposit returned
    // within the window. Active folios only (`cin_status <> 'cancelled'` =
    // iHOTEL's `Cin_status='ปกติ'`).
    let dep = sqlx::query(
        "SELECT COALESCE(SUM(cr.cr_dep_amount) FILTER \
                    (WHERE cr.cr_room_in >= $1 AND cr.cr_room_in < $2),0)::float8 AS dep_received, \
                COALESCE(SUM(cr.cr_dep_amount) FILTER \
                    (WHERE cr.cr_dep_returned_at >= $1 AND cr.cr_dep_returned_at < $2),0)::float8 AS dep_returned \
           FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE cr.cr_dep_amount > 0 AND c.cin_status <> 'cancelled'",
    )
    .bind(window_from)
    .bind(window_to)
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to sum round deposits: {e}")))?;

    let deposits = DepositSummary {
        received: dep.try_get("dep_received").map_err(map_shift_row_err)?,
        returned: dep.try_get("dep_returned").map_err(map_shift_row_err)?,
    };

    let sales_rows = sqlx::query(
        "SELECT CASE WHEN ledger_ds_id = 'P001' THEN 'room' ELSE 'product' END AS category, \
                COALESCE(SUM(ledger_amount),0)::float8 AS amount, \
                COUNT(*)::bigint AS lines \
           FROM ht_payment_ledger \
          WHERE ledger_status = '1' \
            AND ledger_pay_date >= $1 AND ledger_pay_date < $2 \
          GROUP BY 1 ORDER BY 1",
    )
    .bind(window_from)
    .bind(window_to)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to summarise round sales: {e}")))?;

    let mut sales = Vec::with_capacity(sales_rows.len());
    for r in sales_rows {
        sales.push(SalesCategory {
            category: r.try_get("category").map_err(map_shift_row_err)?,
            amount: r.try_get("amount").map_err(map_shift_row_err)?,
            lines: r.try_get("lines").map_err(map_shift_row_err)?,
        });
    }

    let round2 = |v: f64| (v * 100.0).round() / 100.0;
    // iHOTEL's round "total" (FrmDueBill Labeltotal): opening + net cash +
    // credit + transfer. Deposits are reported separately (as iHOTEL does).
    let grand_total =
        round2(shift.opening_float + income.cash_net + income.credit + income.transfer);
    // Cash reconciliation: only cash touches the drawer → opening + net cash.
    let expected_cash = round2(shift.opening_float + income.cash_net);
    let cash_variance = counted_cash.map(|c| round2(c - expected_cash));

    Ok(Json(RoundReportResponse {
        success: true,
        shift,
        window_from,
        window_to,
        open,
        income,
        deposits,
        grand_total,
        sales,
        expected_cash,
        counted_cash,
        cash_variance,
    }))
}

// =============================================================================
// Track J7f — round SUMMARY / analytics (multi-round, over a date range).
//
// iHOTEL equivalent: the `FrmDueBill` history list — `SELECT TOP 100 * FROM
// View_RBill_H WHERE round_end IS NOT NULL AND round_start BETWEEN <from> AND
// <to>` — a table of closed rounds, each with the full per-round breakdown.
// We add the aggregate roll-up (period totals + by-cashier) for analytics.
// =============================================================================

/// `GET /api/shifts/summary?branch=&from=&to=` query string. `from`/`to` are
/// RFC3339; default to the last 30 days. Range capped at 92 days (the deposit
/// per-round subqueries scan `ht_checkin_rooms`).
#[derive(Debug, Deserialize)]
pub struct RoundSummaryQuery {
    pub branch: Option<Branch>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// One closed round in the summary — the same per-round breakdown as the
/// single-round report (iHOTEL `View_RBill_H` columns).
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoundSummaryRow {
    /// PG `shift_id` — lets the UI open the single-round report
    /// (`GET /api/shifts/{shift_id}/report`) on row click.
    pub shift_id: i64,
    pub shift_no: i32,
    pub legacy_round_id: Option<i32>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub opened_by: String,
    pub closed_by: Option<String>,
    pub opening: f64,
    pub cash_received: f64,
    pub cash_paid: f64,
    pub cash_net: f64,
    pub credit: f64,
    pub transfer: f64,
    pub free: f64,
    pub web: f64,
    pub deposits_received: f64,
    pub deposits_returned: f64,
    pub grand_total: f64,
}

/// Period roll-up across the rounds (the analytics layer).
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoundSummaryTotals {
    pub round_count: i64,
    pub cash_received: f64,
    pub cash_paid: f64,
    pub cash_net: f64,
    pub credit: f64,
    pub transfer: f64,
    pub free: f64,
    pub web: f64,
    pub deposits_received: f64,
    pub deposits_returned: f64,
    pub grand_total: f64,
}

/// Per-cashier tally (by `closed_by`) — analytics dimension.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashierTally {
    pub cashier: String,
    pub round_count: i64,
    pub grand_total: f64,
}

/// `200` body for the round summary.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundSummaryResponse {
    pub success: bool,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub rounds: Vec<RoundSummaryRow>,
    pub totals: RoundSummaryTotals,
    pub by_cashier: Vec<CashierTally>,
}

/// `GET /api/shifts/summary` — closed rounds over a date range, each with the
/// iHOTEL `View_RBill_H` per-round breakdown, plus period totals + a
/// by-cashier roll-up. Branch-aware (per-site pool), same as the report.
pub async fn round_summary(
    State(state): State<AppState>,
    Query(query): Query<RoundSummaryQuery>,
) -> ApiResult<Json<RoundSummaryResponse>> {
    let pool = match query.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    let to = query.to.unwrap_or_else(Utc::now);
    let mut from = query.from.unwrap_or_else(|| to - chrono::Duration::days(30));
    // Cap the span so the per-round deposit subqueries stay bounded.
    let max_from = to - chrono::Duration::days(92);
    if from < max_from {
        from = max_from;
    }

    // One pass: per closed round, the ledger tender splits (FILTER) + deposit
    // received/returned (correlated subqueries scoped to that round's window).
    let rows = sqlx::query(
        "SELECT s.shift_id, s.shift_no, s.shift_legacy_round_id, s.shift_opened_at, s.shift_closed_at, \
                s.shift_opened_by, s.shift_closed_by, s.shift_opening_float::float8 AS opening, \
                COALESCE(SUM(l.ledger_cash) FILTER (WHERE l.ledger_cash > 0),0)::float8     AS cash_received, \
                COALESCE(-SUM(l.ledger_cash) FILTER (WHERE l.ledger_cash < 0),0)::float8    AS cash_paid, \
                COALESCE(SUM(l.ledger_credit) FILTER (WHERE l.ledger_credit > 0),0)::float8 AS credit, \
                COALESCE(SUM(l.ledger_tran) FILTER (WHERE l.ledger_tran > 0),0)::float8     AS transfer, \
                COALESCE(SUM(l.ledger_free),0)::float8 AS free, \
                COALESCE(SUM(l.ledger_web),0)::float8  AS web, \
                (SELECT COALESCE(SUM(cr.cr_dep_amount),0)::float8 FROM ht_checkin_rooms cr \
                    JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
                   WHERE cr.cr_dep_amount > 0 AND c.cin_status <> 'cancelled' \
                     AND cr.cr_room_in >= s.shift_opened_at AND cr.cr_room_in < s.shift_closed_at) AS dep_received, \
                (SELECT COALESCE(SUM(cr.cr_dep_amount),0)::float8 FROM ht_checkin_rooms cr \
                    JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
                   WHERE cr.cr_dep_amount > 0 AND c.cin_status <> 'cancelled' \
                     AND cr.cr_dep_returned_at >= s.shift_opened_at AND cr.cr_dep_returned_at < s.shift_closed_at) AS dep_returned \
           FROM ht_shifts s \
           LEFT JOIN ht_payment_ledger l \
             ON l.ledger_status = '1' \
            AND l.ledger_pay_date >= s.shift_opened_at AND l.ledger_pay_date < s.shift_closed_at \
          WHERE s.shift_closed_at IS NOT NULL \
            AND s.shift_opened_at >= $1 AND s.shift_opened_at < $2 \
          GROUP BY s.shift_id \
          ORDER BY s.shift_opened_at DESC",
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to build round summary: {e}")))?;

    let round2 = |v: f64| (v * 100.0).round() / 100.0;
    let mut rounds = Vec::with_capacity(rows.len());
    let mut totals = RoundSummaryTotals::default();
    let mut tally: std::collections::HashMap<String, (i64, f64)> = std::collections::HashMap::new();

    for r in rows {
        let opening: f64 = r.try_get("opening").map_err(map_shift_row_err)?;
        let cash_received: f64 = r.try_get("cash_received").map_err(map_shift_row_err)?;
        let cash_paid: f64 = r.try_get("cash_paid").map_err(map_shift_row_err)?;
        let credit: f64 = r.try_get("credit").map_err(map_shift_row_err)?;
        let transfer: f64 = r.try_get("transfer").map_err(map_shift_row_err)?;
        let free: f64 = r.try_get("free").map_err(map_shift_row_err)?;
        let web: f64 = r.try_get("web").map_err(map_shift_row_err)?;
        let dep_received: f64 = r.try_get("dep_received").map_err(map_shift_row_err)?;
        let dep_returned: f64 = r.try_get("dep_returned").map_err(map_shift_row_err)?;
        let cash_net = cash_received - cash_paid;
        let grand_total = round2(opening + cash_net + credit + transfer);
        let closed_by: Option<String> = r.try_get("shift_closed_by").map_err(map_shift_row_err)?;

        totals.round_count += 1;
        totals.cash_received += cash_received;
        totals.cash_paid += cash_paid;
        totals.credit += credit;
        totals.transfer += transfer;
        totals.free += free;
        totals.web += web;
        totals.deposits_received += dep_received;
        totals.deposits_returned += dep_returned;
        totals.grand_total += grand_total;

        let e = tally.entry(closed_by.clone().unwrap_or_else(|| "—".into())).or_insert((0, 0.0));
        e.0 += 1;
        e.1 += grand_total;

        rounds.push(RoundSummaryRow {
            shift_id: r.try_get("shift_id").map_err(map_shift_row_err)?,
            shift_no: r.try_get("shift_no").map_err(map_shift_row_err)?,
            legacy_round_id: r.try_get("shift_legacy_round_id").map_err(map_shift_row_err)?,
            opened_at: r.try_get("shift_opened_at").map_err(map_shift_row_err)?,
            closed_at: r.try_get("shift_closed_at").map_err(map_shift_row_err)?,
            opened_by: r.try_get("shift_opened_by").map_err(map_shift_row_err)?,
            closed_by,
            opening,
            cash_received,
            cash_paid,
            cash_net,
            credit,
            transfer,
            free,
            web,
            deposits_received: dep_received,
            deposits_returned: dep_returned,
            grand_total,
        });
    }

    // Round period totals; sort cashiers by total desc.
    totals.cash_received = round2(totals.cash_received);
    totals.cash_paid = round2(totals.cash_paid);
    totals.cash_net = round2(totals.cash_received - totals.cash_paid);
    totals.credit = round2(totals.credit);
    totals.transfer = round2(totals.transfer);
    totals.free = round2(totals.free);
    totals.web = round2(totals.web);
    totals.deposits_received = round2(totals.deposits_received);
    totals.deposits_returned = round2(totals.deposits_returned);
    totals.grand_total = round2(totals.grand_total);

    let mut by_cashier: Vec<CashierTally> = tally
        .into_iter()
        .map(|(cashier, (round_count, gt))| CashierTally {
            cashier,
            round_count,
            grand_total: round2(gt),
        })
        .collect();
    by_cashier.sort_by(|a, b| b.grand_total.partial_cmp(&a.grand_total).unwrap_or(std::cmp::Ordering::Equal));

    Ok(Json(RoundSummaryResponse {
        success: true,
        from,
        to,
        rounds,
        totals,
        by_cashier,
    }))
}
