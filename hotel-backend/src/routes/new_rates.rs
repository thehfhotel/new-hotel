//! Rate API routes for the HotelNew database.
//!
//! Track F4 / T1 CRIT-4 (`docs/coexistence/audit-2026-05-13.md`) split
//! the surface in two:
//!
//! * **Canonical pricing matrix** — `ht_rate_tiers`, keyed on
//!   `(Room_Type, Cust_Type)`, mirrored from legacy `HT_Rooms_Price`
//!   by the periodic-poll mapper in
//!   `sync/mappers/rate_tiers.rs`. The read path (`GET /api/new/rates`
//!   and `GET /api/new/rate-tiers`) projects from this table.
//!
//! * **Legacy `ht_rates` table** — DEPRECATED. Its `(weekday / weekend /
//!   special)` axis is structurally wrong (legacy iHOTEL prices by
//!   customer-type tier, not by day-of-week category). It stays in
//!   place so the existing CRUD form on `/rates` can continue to write
//!   without breaking the UI, but no canonical read path consumes it
//!   anymore. A follow-on migration will DROP `ht_rates` once we are
//!   sure no reader remains.
//!
//! ## Endpoint matrix
//!
//! | Endpoint                              | Source            | Status     |
//! |---------------------------------------|-------------------|------------|
//! | `GET    /api/new/rates`               | `ht_rate_tiers`   | F4-migrated |
//! | `GET    /api/new/rates/:id`           | `ht_rate_tiers`   | F4-migrated |
//! | `GET    /api/new/rate-tiers`          | `ht_rate_tiers`   | F4 (new)    |
//! | `POST   /api/new/rates`               | `ht_rates`        | deprecated  |
//! | `PUT    /api/new/rates/:id`           | `ht_rates`        | deprecated  |
//! | `DELETE /api/new/rates/:id`           | `ht_rates`        | deprecated  |
//!
//! The `GET /api/new/rates` endpoint preserves the legacy `Rate`
//! response shape so the existing `app/rates/page.tsx` continues to
//! render. It picks the "default" customer-type tier (`ราคาปกติ`) so the
//! single-row-per-room-type list shape stays intact. Callers that need
//! per-tier pricing (e.g. corporate vs walk-in) hit
//! `GET /api/new/rate-tiers` instead.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};

/// `Cust_Type` literal that selects the "standard" pricing tier in
/// legacy iHOTEL. Used as the implicit default when callers query the
/// legacy `GET /api/new/rates` endpoint without specifying a
/// customer-type filter.
const DEFAULT_CUST_TYPE: &str = "ราคาปกติ";

/// Rate type enum (legacy ht_rates shape — kept for response
/// compatibility with the existing /rates frontend page).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RateType {
    Multiplier,
    Fixed,
}

impl RateType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fixed" => RateType::Fixed,
            _ => RateType::Multiplier,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RateType::Multiplier => "multiplier",
            RateType::Fixed => "fixed",
        }
    }
}

/// Response row for `GET /api/new/rates` and `GET /api/new/rates/:id`.
///
/// Shape preserved from the pre-F4 `ht_rates` projection so
/// `app/rates/page.tsx` continues to render without a frontend change.
/// Post-F4 the `value` field carries the per-night price from
/// `ht_rate_tiers.rate_tier_price` (default customer-type tier), and
/// the `rate_type` is always `"fixed"`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub id: i32,
    pub name: String,
    pub room_type_id: Option<i32>,
    pub room_type_name: Option<String>,
    pub rate_type: String,
    pub value: f64,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub days_of_week: Option<String>,
    pub active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// One row from the F4 canonical `ht_rate_tiers` table — exposed
/// verbatim via `GET /api/new/rate-tiers`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateTier {
    pub id: i64,
    pub room_type: String,
    pub cust_type: String,
    pub price: f64,
    pub price_hourly: Option<f64>,
    pub price_monthly: Option<f64>,
    pub legacy_id: Option<i32>,
    pub active: bool,
}

/// Query parameters for `GET /api/new/rates`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatesQuery {
    pub room_type_id: Option<i32>,
    pub active: Option<bool>,
}

/// Query parameters for `GET /api/new/rate-tiers` (F4 — the canonical
/// composite-key lookup).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateTierQuery {
    pub room_type: Option<String>,
    pub cust_type: Option<String>,
}

/// Response for `GET /api/new/rates`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatesResponse {
    pub success: bool,
    pub data: Vec<Rate>,
    pub total: i32,
}

/// Response for `GET /api/new/rates/:id`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateResponse {
    pub success: bool,
    pub rate: Rate,
}

/// Response for `GET /api/new/rate-tiers`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateTiersResponse {
    pub success: bool,
    pub data: Vec<RateTier>,
    pub total: i32,
}

/// Request body for creating/updating a deprecated `ht_rates` row.
///
/// Kept verbatim so the existing /rates form continues to POST/PUT
/// without a contract change. Writes target `ht_rates`, which is now
/// disconnected from the canonical read path. Documented in the
/// module-level docstring.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRateRequest {
    pub name: String,
    pub room_type_id: Option<i32>,
    pub rate_type: String,
    pub value: f64,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub days_of_week: Option<String>,
    pub active: Option<bool>,
}

/// Response for mutation operations.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

// ============================================================================
// F4 read path — canonical ht_rate_tiers
// ============================================================================

/// GET /api/new/rates — backwards-compatible legacy-shape list.
///
/// Post-F4 the rows come from `ht_rate_tiers` filtered by
/// `rate_tier_active = true` and the default customer-type tier
/// (`ราคาปกติ`). One row per `(room_type, default_cust_type)` —
/// matches the pre-F4 single-row-per-rate cardinality. Callers needing
/// the full per-tier matrix hit `GET /api/new/rate-tiers` instead.
pub async fn list_rates(
    State(state): State<AppState>,
    Query(params): Query<RatesQuery>,
) -> ApiResult<Json<RatesResponse>> {
    let pool = &state.new_pool;

    // Build dynamic WHERE without sql injection — only known filters.
    let mut conditions: Vec<String> = vec![
        "t.rate_tier_active = true".to_string(),
        format!("t.rate_tier_cust_type = '{}'", DEFAULT_CUST_TYPE.replace('\'', "''")),
    ];
    if let Some(active) = params.active {
        // Override of the default active filter for the rare admin
        // case where the operator wants to inspect disabled tiers.
        conditions.clear();
        conditions.push(format!(
            "t.rate_tier_cust_type = '{}'",
            DEFAULT_CUST_TYPE.replace('\'', "''")
        ));
        conditions.push(format!(
            "t.rate_tier_active = {}",
            if active { "true" } else { "false" }
        ));
    }
    if let Some(room_type_id) = params.room_type_id {
        conditions.push(format!("rt.type_id = {}", room_type_id));
    }

    let where_clause = format!("WHERE {}", conditions.join(" AND "));

    let query = format!(
        r#"
        SELECT
            t.rate_tier_id,
            t.rate_tier_room_type,
            t.rate_tier_cust_type,
            t.rate_tier_price::float8 AS rate_tier_price,
            t.rate_tier_active,
            t.rate_tier_created_at,
            t.rate_tier_updated_at,
            rt.type_id          AS rt_type_id,
            rt.type_name        AS rt_type_name
        FROM ht_rate_tiers t
        LEFT JOIN ht_room_types rt ON rt.type_name = t.rate_tier_room_type
        {}
        ORDER BY t.rate_tier_room_type ASC
        "#,
        where_clause
    );

    let rows = sqlx::query(&query).fetch_all(pool).await?;
    let rates: Vec<Rate> = rows.iter().map(rate_from_row).collect();
    let total = rates.len() as i32;

    Ok(Json(RatesResponse {
        success: true,
        data: rates,
        total,
    }))
}

/// GET /api/new/rates/:id — single rate by `ht_rate_tiers.rate_tier_id`.
///
/// Returns legacy-shape `Rate`. Only rows where `rate_tier_cust_type` is
/// the default tier are returned via the legacy path; the multi-tier
/// detail view is `GET /api/new/rate-tiers?room_type=...`.
pub async fn get_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
) -> ApiResult<Json<RateResponse>> {
    let pool = &state.new_pool;

    let row = sqlx::query(
        r#"
        SELECT
            t.rate_tier_id,
            t.rate_tier_room_type,
            t.rate_tier_cust_type,
            t.rate_tier_price::float8 AS rate_tier_price,
            t.rate_tier_active,
            t.rate_tier_created_at,
            t.rate_tier_updated_at,
            rt.type_id          AS rt_type_id,
            rt.type_name        AS rt_type_name
        FROM ht_rate_tiers t
        LEFT JOIN ht_room_types rt ON rt.type_name = t.rate_tier_room_type
        WHERE t.rate_tier_id = $1
        "#,
    )
    .bind(rate_id as i64)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Rate not found".to_string()))?;

    let rate = rate_from_row(&row);

    Ok(Json(RateResponse {
        success: true,
        rate,
    }))
}

/// GET /api/new/rate-tiers — F4 canonical composite-key lookup.
///
/// Query params (all optional):
/// * `roomType` — filter to one room-type label
/// * `custType` — filter to one customer-type tier
///
/// Returns one `RateTier` per matched row. Use this when the caller
/// needs full per-customer-type pricing (corporate vs walk-in, hourly
/// extension, monthly). For the simpler one-row-per-room-type list
/// shape the legacy frontend renders, hit `GET /api/new/rates`.
pub async fn list_rate_tiers(
    State(state): State<AppState>,
    Query(params): Query<RateTierQuery>,
) -> ApiResult<Json<RateTiersResponse>> {
    let pool = &state.new_pool;

    let mut sql = String::from(
        r#"
        SELECT
            rate_tier_id,
            rate_tier_room_type,
            rate_tier_cust_type,
            rate_tier_price::float8         AS rate_tier_price,
            rate_tier_price_hourly::float8  AS rate_tier_price_hourly,
            rate_tier_price_monthly::float8 AS rate_tier_price_monthly,
            rate_tier_legacy_id,
            rate_tier_active
        FROM ht_rate_tiers
        WHERE rate_tier_active = true
        "#,
    );

    let mut bind_room_type: Option<String> = None;
    let mut bind_cust_type: Option<String> = None;
    if let Some(rt) = params.room_type.as_deref() {
        if !rt.trim().is_empty() {
            sql.push_str(" AND rate_tier_room_type = $1");
            bind_room_type = Some(rt.to_string());
        }
    }
    if let Some(ct) = params.cust_type.as_deref() {
        if !ct.trim().is_empty() {
            let placeholder = if bind_room_type.is_some() { "$2" } else { "$1" };
            sql.push_str(&format!(" AND rate_tier_cust_type = {}", placeholder));
            bind_cust_type = Some(ct.to_string());
        }
    }
    sql.push_str(" ORDER BY rate_tier_room_type, rate_tier_cust_type");

    let mut q = sqlx::query(&sql);
    if let Some(rt) = bind_room_type {
        q = q.bind(rt);
    }
    if let Some(ct) = bind_cust_type {
        q = q.bind(ct);
    }
    let rows = q.fetch_all(pool).await?;
    let data: Vec<RateTier> = rows.iter().map(rate_tier_from_row).collect();
    let total = data.len() as i32;

    Ok(Json(RateTiersResponse {
        success: true,
        data,
        total,
    }))
}

/// Service-layer helper for the matrix lookup used by booking /
/// check-in price resolution. Returns `Ok(None)` when no row matches.
/// Public so future callers in `routes/new_bookings.rs` and the
/// writeback pricing logic can switch from per-room defaults to
/// per-tier matrix lookup without re-implementing the SELECT.
pub async fn lookup_by_room_and_cust_type(
    pool: &crate::db::PgPool,
    room_type: &str,
    cust_type: &str,
) -> Result<Option<RateTier>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            rate_tier_id,
            rate_tier_room_type,
            rate_tier_cust_type,
            rate_tier_price::float8         AS rate_tier_price,
            rate_tier_price_hourly::float8  AS rate_tier_price_hourly,
            rate_tier_price_monthly::float8 AS rate_tier_price_monthly,
            rate_tier_legacy_id,
            rate_tier_active
        FROM ht_rate_tiers
        WHERE rate_tier_active = true
          AND rate_tier_room_type = $1
          AND rate_tier_cust_type = $2
        "#,
    )
    .bind(room_type)
    .bind(cust_type)
    .fetch_optional(pool)
    .await?;

    Ok(row.as_ref().map(rate_tier_from_row))
}

// ============================================================================
// Deprecated ht_rates write path — POST / PUT / DELETE
// ============================================================================
//
// These endpoints write to the legacy `ht_rates` table, which is
// disconnected from the canonical read path post-F4. They are kept in
// place so the existing /rates form continues to mutate without a
// frontend change. iHOTEL is the source of pricing edits; writeback of
// our app's `ht_rates` changes back to legacy is intentionally out of
// F4 scope. A follow-on track will either DROP `ht_rates` entirely
// (after frontend retirement) or wire writeback into iHOTEL's
// `HT_Rooms_Price` via the `HT_Rooms_Price` UPSERT recipe.

/// POST /api/new/rates — DEPRECATED. Writes to `ht_rates`.
pub async fn create_rate(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateRateRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Rate name is required".to_string()));
    }
    let rate_type = body.rate_type.to_lowercase();
    if rate_type != "multiplier" && rate_type != "fixed" {
        return Err(ApiError::BadRequest(
            "Rate type must be 'multiplier' or 'fixed'".to_string(),
        ));
    }

    let pool = &state.new_pool;
    let active = body.active.unwrap_or(true);
    let valid_from = parse_optional_date(&body.valid_from, "valid_from")?;
    let valid_to = parse_optional_date(&body.valid_to, "valid_to")?;

    let rec = sqlx::query!(
        r#"INSERT INTO ht_rates (rate_name, rate_room_type_id, rate_type, rate_value, rate_valid_from, rate_valid_to, rate_days_of_week, rate_active)
        VALUES ($1, $2, $3, $4::float8, $5, $6, $7, $8)
        RETURNING rate_id"#,
        name, body.room_type_id, rate_type.as_str(), body.value,
        valid_from, valid_to,
        body.days_of_week.as_deref(), active
    )
    .fetch_one(pool)
    .await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate created successfully".to_string(),
        id: Some(rec.rate_id),
    }))
}

/// PUT /api/new/rates/:id — DEPRECATED. Writes to `ht_rates`.
pub async fn update_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
    Json(body): Json<CreateUpdateRateRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Rate name is required".to_string()));
    }
    let rate_type = body.rate_type.to_lowercase();
    if rate_type != "multiplier" && rate_type != "fixed" {
        return Err(ApiError::BadRequest(
            "Rate type must be 'multiplier' or 'fixed'".to_string(),
        ));
    }

    let pool = &state.new_pool;

    let exists = sqlx::query!("SELECT rate_id FROM ht_rates WHERE rate_id = $1", rate_id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound("Rate not found".to_string()));
    }

    let active = body.active.unwrap_or(true);
    let valid_from = parse_optional_date(&body.valid_from, "valid_from")?;
    let valid_to = parse_optional_date(&body.valid_to, "valid_to")?;

    sqlx::query!(
        r#"UPDATE ht_rates SET rate_name = $1, rate_room_type_id = $2, rate_type = $3, rate_value = $4::float8,
        rate_valid_from = $5, rate_valid_to = $6, rate_days_of_week = $7, rate_active = $8, rate_updated = NOW()
        WHERE rate_id = $9"#,
        name, body.room_type_id, rate_type.as_str(), body.value,
        valid_from, valid_to,
        body.days_of_week.as_deref(), active, rate_id
    )
    .execute(pool)
    .await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate updated successfully".to_string(),
        id: Some(rate_id),
    }))
}

/// DELETE /api/new/rates/:id — DEPRECATED. Writes to `ht_rates`.
pub async fn delete_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    let result = sqlx::query!("DELETE FROM ht_rates WHERE rate_id = $1", rate_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Rate not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate deleted successfully".to_string(),
        id: Some(rate_id),
    }))
}

// ============================================================================
// Helpers
// ============================================================================

/// Parse an optional ISO date string. Returns `None` if the input is
/// absent; `BadRequest` if present but malformed.
fn parse_optional_date(
    value: &Option<String>,
    field: &'static str,
) -> Result<Option<NaiveDate>, ApiError> {
    match value {
        None => Ok(None),
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(Some)
            .map_err(|_| {
                ApiError::BadRequest(format!(
                    "Invalid {field} date format (expected YYYY-MM-DD)"
                ))
            }),
    }
}

/// Translate a `ht_rate_tiers` row joined to `ht_room_types` (by
/// `type_name`) into the legacy `Rate` shape so the existing frontend
/// continues to render. `rate_id` is the `BIGSERIAL` cast back into
/// `i32` for compatibility; this is safe today (production has ~32
/// rows) but a follow-on can widen the response type when the
/// frontend allows.
fn rate_from_row(row: &sqlx::postgres::PgRow) -> Rate {
    let rate_tier_id: i64 = row.try_get("rate_tier_id").unwrap_or(0);
    Rate {
        id: rate_tier_id as i32,
        name: row.try_get::<String, _>("rate_tier_room_type").unwrap_or_default(),
        room_type_id: row.try_get::<i32, _>("rt_type_id").ok(),
        room_type_name: row.try_get::<String, _>("rt_type_name").ok(),
        rate_type: "fixed".to_string(),
        value: row.try_get::<f64, _>("rate_tier_price").unwrap_or(0.0),
        valid_from: None,
        valid_to: None,
        days_of_week: None,
        active: row.try_get::<bool, _>("rate_tier_active").unwrap_or(true),
        created_at: row
            .try_get::<chrono::DateTime<chrono::Utc>, _>("rate_tier_created_at")
            .ok()
            .map(|dt| dt.naive_utc()),
        updated_at: row
            .try_get::<chrono::DateTime<chrono::Utc>, _>("rate_tier_updated_at")
            .ok()
            .map(|dt| dt.naive_utc()),
    }
}

/// Translate a raw `ht_rate_tiers` row into the verbose `RateTier`
/// response shape used by the new `GET /api/new/rate-tiers` endpoint.
fn rate_tier_from_row(row: &sqlx::postgres::PgRow) -> RateTier {
    RateTier {
        id: row.try_get::<i64, _>("rate_tier_id").unwrap_or(0),
        room_type: row.try_get::<String, _>("rate_tier_room_type").unwrap_or_default(),
        cust_type: row.try_get::<String, _>("rate_tier_cust_type").unwrap_or_default(),
        price: row.try_get::<f64, _>("rate_tier_price").unwrap_or(0.0),
        price_hourly: row.try_get::<f64, _>("rate_tier_price_hourly").ok(),
        price_monthly: row.try_get::<f64, _>("rate_tier_price_monthly").ok(),
        legacy_id: row.try_get::<i32, _>("rate_tier_legacy_id").ok(),
        active: row.try_get::<bool, _>("rate_tier_active").unwrap_or(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_type_parse_round_trip() {
        assert_eq!(RateType::from_str("Fixed"), RateType::Fixed);
        assert_eq!(RateType::from_str("multiplier"), RateType::Multiplier);
        assert_eq!(RateType::from_str("garbage"), RateType::Multiplier);
        assert_eq!(RateType::Fixed.as_str(), "fixed");
        assert_eq!(RateType::Multiplier.as_str(), "multiplier");
    }

    #[test]
    fn default_cust_type_is_thai_standard_tier() {
        // The legacy `ราคาปกติ` label is the implicit "default" pricing
        // tier in iHOTEL. Pinned here so a future rename of the
        // constant breaks loudly via this test rather than silently
        // shifting which rows the legacy /rates endpoint returns.
        assert_eq!(DEFAULT_CUST_TYPE, "ราคาปกติ");
    }

    #[test]
    fn parse_optional_date_accepts_iso() {
        let parsed = parse_optional_date(&Some("2026-05-13".to_string()), "valid_from")
            .expect("ISO date should parse");
        assert_eq!(
            parsed,
            Some(NaiveDate::from_ymd_opt(2026, 5, 13).unwrap())
        );
    }

    #[test]
    fn parse_optional_date_returns_none_when_absent() {
        let parsed =
            parse_optional_date(&None, "valid_from").expect("None input should yield Ok(None)");
        assert_eq!(parsed, None);
    }

    #[test]
    fn parse_optional_date_rejects_malformed_input() {
        let err = parse_optional_date(&Some("13/05/2026".to_string()), "valid_to")
            .expect_err("Malformed date should error");
        assert!(matches!(err, ApiError::BadRequest(_)));
    }
}
