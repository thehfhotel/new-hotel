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
    extract::{Query, State},
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
