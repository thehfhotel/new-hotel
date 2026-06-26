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
    pub cash: f64,
    pub credit: f64,
    pub transfer: f64,
    pub free: f64,
    pub web: f64,
    pub total: f64,
    /// Active ledger lines counted in the window.
    pub line_count: i64,
    /// Distinct `Pay_no` values (≈ number of payment receipts) in the window.
    pub payment_count: i64,
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
    pub sales: Vec<SalesCategory>,
    /// Cash the drawer SHOULD hold: `shift_opening_float + cash tenders` over
    /// the window (only the cash tender touches the drawer).
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
    let inc = sqlx::query(
        "SELECT COALESCE(SUM(ledger_cash),0)::float8   AS cash, \
                COALESCE(SUM(ledger_credit),0)::float8 AS credit, \
                COALESCE(SUM(ledger_tran),0)::float8   AS transfer, \
                COALESCE(SUM(ledger_free),0)::float8   AS free, \
                COALESCE(SUM(ledger_web),0)::float8    AS web, \
                COALESCE(SUM(ledger_amount),0)::float8 AS total, \
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

    let income = TenderBreakdown {
        cash: inc.try_get("cash").map_err(map_shift_row_err)?,
        credit: inc.try_get("credit").map_err(map_shift_row_err)?,
        transfer: inc.try_get("transfer").map_err(map_shift_row_err)?,
        free: inc.try_get("free").map_err(map_shift_row_err)?,
        web: inc.try_get("web").map_err(map_shift_row_err)?,
        total: inc.try_get("total").map_err(map_shift_row_err)?,
        line_count: inc.try_get("line_count").map_err(map_shift_row_err)?,
        payment_count: inc.try_get("payment_count").map_err(map_shift_row_err)?,
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

    // Cash reconciliation: only the cash tender touches the drawer.
    let round2 = |v: f64| (v * 100.0).round() / 100.0;
    let expected_cash = round2(shift.opening_float + income.cash);
    let cash_variance = counted_cash.map(|c| round2(c - expected_cash));

    Ok(Json(RoundReportResponse {
        success: true,
        shift,
        window_from,
        window_to,
        open,
        income,
        sales,
        expected_cash,
        counted_cash,
        cash_variance,
    }))
}
