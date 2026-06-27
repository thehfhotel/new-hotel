//! Rate API routes for the HotelNew database.
//!
//! Track F4 / T1 CRIT-4 (`docs/coexistence/audit-2026-05-13.md`) split
//! the surface in two:
//!
//! * **Canonical pricing matrix** — `ht_rate_tiers`, keyed on
//!   `(Room_Type, Cust_Type)`, mirrored from legacy `HT_Rooms_Price`
//!   by the periodic-poll mapper in
//!   `sync/mappers/rate_tiers.rs`. The read path (`GET /api/new/rates`)
//!   projects from this table.
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
//! | `POST   /api/new/rates`               | `ht_rates`        | deprecated  |
//! | `PUT    /api/new/rates/:id`           | `ht_rates`        | deprecated  |
//! | `DELETE /api/new/rates/:id`           | `ht_rates`        | deprecated  |
//!
//! The `GET /api/new/rates` endpoint preserves the legacy `Rate`
//! response shape so the existing `app/rates/page.tsx` continues to
//! render. It picks the "default" customer-type tier (`ราคาปกติ`) so the
//! single-row-per-room-type list shape stays intact. Callers that need
//! per-tier pricing (e.g. corporate vs walk-in) use the service-layer
//! [`lookup_by_room_and_cust_type`] helper.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::outbox::intent::{rate_price_aggregate, RatePricePayload, WritebackIntent};
use crate::outbox::{generate_idempotency_key, OutboxRepository};

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
    /// JSON field name pinned to `rateName` to match the frontend's
    /// `RateRecord` contract (`app/rates/page.tsx`). Without the
    /// rename the API ships `name`, the frontend reads
    /// `rate.rateName` → `undefined`, and downstream displays of the
    /// rate name read as blank or trip optional-chain fallbacks.
    #[serde(rename = "rateName")]
    pub name: String,
    pub room_type_id: Option<i32>,
    pub room_type_name: Option<String>,
    pub rate_type: String,
    /// JSON field name pinned to `rateValue` to match the frontend's
    /// `RateRecord` contract. Without the rename the API ships
    /// `value`, the frontend reads `rate.rateValue` → `undefined`,
    /// and `.toLocaleString()` on undefined crashes the rates page
    /// table render (Array.map → TypeError, 2026-05-20 production
    /// incident).
    #[serde(rename = "rateValue")]
    pub value: f64,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    /// Day-of-week filter as `[0..=6]` (Sun=0..Sat=6). Always a JSON
    /// array, never `null`, so the frontend's `formatDaysOfWeek(days)`
    /// which calls `days.length` / `days.includes(...)` never trips a
    /// null/string-shape crash. Stored in legacy SQL as a comma-
    /// separated VARCHAR(50); converted at the rate-from-row boundary.
    /// Empty array means "every day applies" (legacy: NULL).
    pub days_of_week: Vec<i32>,
    pub active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// One row from the F4 canonical `ht_rate_tiers` table — returned by
/// the [`lookup_by_room_and_cust_type`] service-layer helper.
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

/// Request body for creating/updating a deprecated `ht_rates` row.
///
/// Kept verbatim so the existing /rates form continues to POST/PUT
/// without a contract change. Writes target `ht_rates`, which is now
/// disconnected from the canonical read path. Documented in the
/// module-level docstring.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRateRequest {
    #[serde(rename = "rateName")]
    pub name: String,
    pub room_type_id: Option<i32>,
    pub rate_type: String,
    #[serde(rename = "rateValue")]
    pub value: f64,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    /// Day-of-week filter from the form (`[0..=6]`). Joined to a
    /// comma-separated string before binding to the SQL VARCHAR(50)
    /// column. Empty array → NULL (legacy "every day").
    #[serde(default)]
    pub days_of_week: Vec<i32>,
    pub active: Option<bool>,
}

/// Join a `Vec<i32>` day-of-week filter into the comma-separated
/// VARCHAR(50) the legacy `ht_rates.rate_days_of_week` column expects.
/// Returns `None` for an empty vec (legacy NULL convention = "every
/// day"), matching what `ht_rates`'s pre-F4 form has always written.
fn days_of_week_to_csv(days: &[i32]) -> Option<String> {
    if days.is_empty() {
        return None;
    }
    Some(
        days.iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
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

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query)).fetch_all(pool).await?;
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

    let days_of_week_csv = days_of_week_to_csv(&body.days_of_week);
    let rec = sqlx::query!(
        r#"INSERT INTO ht_rates (rate_name, rate_room_type_id, rate_type, rate_value, rate_valid_from, rate_valid_to, rate_days_of_week, rate_active)
        VALUES ($1, $2, $3, $4::float8, $5, $6, $7, $8)
        RETURNING rate_id"#,
        name, body.room_type_id, rate_type.as_str(), body.value,
        valid_from, valid_to,
        days_of_week_csv.as_deref(), active
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

    let days_of_week_csv = days_of_week_to_csv(&body.days_of_week);
    sqlx::query!(
        r#"UPDATE ht_rates SET rate_name = $1, rate_room_type_id = $2, rate_type = $3, rate_value = $4::float8,
        rate_valid_from = $5, rate_valid_to = $6, rate_days_of_week = $7, rate_active = $8, rate_updated = NOW()
        WHERE rate_id = $9"#,
        name, body.room_type_id, rate_type.as_str(), body.value,
        valid_from, valid_to,
        days_of_week_csv.as_deref(), active, rate_id
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
        // rate_tiers don't track per-day filters — empty array means
        // "applies every day" on both the frontend and the legacy
        // CSV-NULL convention.
        days_of_week: Vec::new(),
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

// ============================================================================
// Task #51 — canonical ht_rate_tiers WRITE path (replaces the dead ht_rates
// writes) + legacy HT_Rooms_Price writeback.
// ============================================================================
//
// The pre-#51 POST/PUT/DELETE above write the DEPRECATED `ht_rates` table,
// which no canonical reader consumes and which never reaches iHOTEL. The
// endpoints below write the LIVE `ht_rate_tiers` matrix (the mirror of legacy
// `HT_Rooms_Price`) and enqueue a `WritebackIntent::UpsertRatePrice` so the
// edit lands on iHOTEL's pricing matrix too. Branch-aware: the canonical write
// and the outbox enqueue both target the per-branch pool (HF Ville mutations
// are gated upstream by `ville_write_guard`).

/// Resolve the canonical pool + the diagnostic site string for a branch.
/// `Branch::All` is not meaningful for a write → treated as HF Hotel.
fn pool_and_site(state: &AppState, branch: Branch) -> ApiResult<(&crate::db::PgPool, &'static str)> {
    // Pool selection delegates to the unified per-site write chokepoint; the
    // diagnostic site string stays local (only `Hfville` is distinct).
    let site = if matches!(branch, Branch::Hfville) { "hfville" } else { "hfhotel" };
    Ok((state.write_pool(Some(branch))?, site))
}

/// Query parameters for `GET /api/rate-tiers`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateTiersQuery {
    pub branch: Option<Branch>,
    /// Filter to a single `rate_tier_room_type` (Thai literal).
    pub room_type: Option<String>,
    /// `false` (default) returns every tier; `true` only active ones.
    #[serde(default)]
    pub active_only: bool,
}

/// Response for `GET /api/rate-tiers`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateTiersResponse {
    pub success: bool,
    pub data: Vec<RateTier>,
    pub total: i32,
}

/// Request body for `POST /api/rate-tiers`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRateTierRequest {
    pub branch: Option<Branch>,
    pub room_type: String,
    pub cust_type: String,
    pub price: f64,
    #[serde(default)]
    pub price_hourly: Option<f64>,
    #[serde(default)]
    pub price_monthly: Option<f64>,
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Request body for `PUT /api/rate-tiers/:id`. The composite key
/// `(room_type, cust_type)` is the tier's identity and is NOT editable here —
/// only the prices + active flag. Read back from the existing row.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRateTierRequest {
    pub branch: Option<Branch>,
    pub price: f64,
    #[serde(default)]
    pub price_hourly: Option<f64>,
    #[serde(default)]
    pub price_monthly: Option<f64>,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

/// GET /api/rate-tiers — the full canonical `(room_type, cust_type)` pricing
/// matrix (every customer-type tier, not just the default). Branch-aware.
pub async fn list_rate_tiers(
    State(state): State<AppState>,
    Query(params): Query<RateTiersQuery>,
) -> ApiResult<Json<RateTiersResponse>> {
    let (pool, _site) = pool_and_site(&state, params.branch.unwrap_or_default())?;

    let mut conditions: Vec<String> = Vec::new();
    if params.active_only {
        conditions.push("rate_tier_active = true".to_string());
    }
    if let Some(rt) = params.room_type.as_deref() {
        conditions.push(format!("rate_tier_room_type = '{}'", rt.replace('\'', "''")));
    }
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
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
        {where_clause}
        ORDER BY rate_tier_room_type ASC, rate_tier_cust_type ASC
        "#
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query)).fetch_all(pool).await?;
    let data: Vec<RateTier> = rows.iter().map(rate_tier_from_row).collect();
    let total = data.len() as i32;

    Ok(Json(RateTiersResponse {
        success: true,
        data,
        total,
    }))
}

/// POST /api/rate-tiers — create a new `(room_type, cust_type)` tier in
/// canonical PG and enqueue the legacy `HT_Rooms_Price` UPSERT.
pub async fn create_rate_tier(
    State(state): State<AppState>,
    Json(body): Json<CreateRateTierRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let room_type = body.room_type.trim();
    let cust_type = body.cust_type.trim();
    if room_type.is_empty() {
        return Err(ApiError::BadRequest("Room type is required".to_string()));
    }
    if cust_type.is_empty() {
        return Err(ApiError::BadRequest("Customer type is required".to_string()));
    }
    if !body.price.is_finite() || body.price < 0.0 {
        return Err(ApiError::BadRequest("Price must be a non-negative number".to_string()));
    }

    let (pool, site) = pool_and_site(&state, body.branch.unwrap_or_default())?;
    let mut tx = pool.begin().await?;

    // Reject a duplicate composite key with a 400 rather than letting the
    // UNIQUE constraint surface as a 500.
    let existing = sqlx::query(
        "SELECT 1 FROM ht_rate_tiers WHERE rate_tier_room_type = $1 AND rate_tier_cust_type = $2",
    )
    .bind(room_type)
    .bind(cust_type)
    .fetch_optional(&mut *tx)
    .await?;
    if existing.is_some() {
        return Err(ApiError::BadRequest(
            "A rate tier already exists for this room type / customer type".to_string(),
        ));
    }

    let rec = sqlx::query(
        "INSERT INTO ht_rate_tiers \
             (rate_tier_room_type, rate_tier_cust_type, rate_tier_price, \
              rate_tier_price_hourly, rate_tier_price_monthly, rate_tier_active) \
         VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric, $6) \
         RETURNING rate_tier_id",
    )
    .bind(room_type)
    .bind(cust_type)
    .bind(body.price)
    .bind(body.price_hourly)
    .bind(body.price_monthly)
    .bind(body.active)
    .fetch_one(&mut *tx)
    .await?;
    let rate_tier_id: i64 = rec.try_get("rate_tier_id")?;

    enqueue_rate_writeback(
        &mut tx,
        site,
        room_type,
        cust_type,
        body.price,
        body.price_hourly,
        body.price_monthly,
        body.active,
    )
    .await?;

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate tier created successfully".to_string(),
        id: Some(rate_tier_id as i32),
    }))
}

/// PUT /api/rate-tiers/:id — update a tier's prices (+ active flag) in
/// canonical PG and enqueue the legacy `HT_Rooms_Price` UPSERT.
pub async fn update_rate_tier(
    State(state): State<AppState>,
    Path(rate_tier_id): Path<i64>,
    Json(body): Json<UpdateRateTierRequest>,
) -> ApiResult<Json<MutationResponse>> {
    if !body.price.is_finite() || body.price < 0.0 {
        return Err(ApiError::BadRequest("Price must be a non-negative number".to_string()));
    }

    let (pool, site) = pool_and_site(&state, body.branch.unwrap_or_default())?;
    let mut tx = pool.begin().await?;

    // Capture the composite key from the existing row — it is the tier's
    // immutable identity and the legacy `HT_Rooms_Price` UPSERT key.
    let existing = sqlx::query(
        "SELECT rate_tier_room_type, rate_tier_cust_type FROM ht_rate_tiers WHERE rate_tier_id = $1",
    )
    .bind(rate_tier_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiError::NotFound("Rate tier not found".to_string()))?;
    let room_type: String = existing.try_get("rate_tier_room_type")?;
    let cust_type: String = existing.try_get("rate_tier_cust_type")?;

    sqlx::query(
        "UPDATE ht_rate_tiers SET \
             rate_tier_price         = $2::numeric, \
             rate_tier_price_hourly  = $3::numeric, \
             rate_tier_price_monthly = $4::numeric, \
             rate_tier_active        = $5, \
             rate_tier_updated_at    = NOW() \
         WHERE rate_tier_id = $1",
    )
    .bind(rate_tier_id)
    .bind(body.price)
    .bind(body.price_hourly)
    .bind(body.price_monthly)
    .bind(body.active)
    .execute(&mut *tx)
    .await?;

    enqueue_rate_writeback(
        &mut tx,
        site,
        &room_type,
        &cust_type,
        body.price,
        body.price_hourly,
        body.price_monthly,
        body.active,
    )
    .await?;

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate tier updated successfully".to_string(),
        id: Some(rate_tier_id as i32),
    }))
}

/// Enqueue the `HT_Rooms_Price` writeback inside the caller's TX (atomic with
/// the canonical `ht_rate_tiers` write — architecture.md §3.6c).
///
/// Only fires when `active` is true: a disabled tier has no representation in
/// legacy `HT_Rooms_Price` (the legacy matrix has no active flag), and pushing
/// the price of a just-disabled tier would mis-mirror it. Deactivation stays
/// canonical-only — iHOTEL keeps its own row until an operator edits it there
/// (delete-then-reinsert is iHOTEL's job, not ours; cheatsheet §`HT_Rooms_Price`).
#[allow(clippy::too_many_arguments)]
async fn enqueue_rate_writeback(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    site: &str,
    room_type: &str,
    cust_type: &str,
    price: f64,
    price_hourly: Option<f64>,
    price_monthly: Option<f64>,
    active: bool,
) -> ApiResult<()> {
    if !active {
        return Ok(());
    }
    let intent = WritebackIntent::UpsertRatePrice {
        rate_aggregate_id: rate_price_aggregate(site, room_type, cust_type),
        payload: RatePricePayload {
            site_id: site.to_string(),
            room_type: room_type.to_string(),
            cust_type: cust_type.to_string(),
            price,
            price_hourly,
            price_monthly,
        },
    };
    // Per-event discriminator key: a tier's price is edited repeatedly over its
    // lifetime and completed jobs persist as status='done' rows, so a
    // deterministic (intent, aggregate) key would hard-fail the second edit on
    // the UNIQUE `writeback_jobs.idempotency_key`. (Same rationale as the
    // room-edit writeback in `new_rooms::update_room`.)
    let idempotency_key = generate_idempotency_key(&intent, Uuid::new_v4());
    OutboxRepository::enqueue(tx, &intent, idempotency_key)
        .await
        .map_err(ApiError::from)?;
    Ok(())
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
