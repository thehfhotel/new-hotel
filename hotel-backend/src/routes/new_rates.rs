//! New Rate API routes for HotelNew database
//!
//! - GET /api/new/rates - List all rates (with optional room_type_id filter)
//! - POST /api/new/rates - Create a rate
//! - GET /api/new/rates/:id - Get single rate
//! - PUT /api/new/rates/:id - Update rate
//! - DELETE /api/new/rates/:id - Delete rate

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};

/// Rate type enum
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

/// Rate from HT_Rates table
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

/// Query parameters for rates list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatesQuery {
    pub room_type_id: Option<i32>,
    pub active: Option<bool>,
}

/// Response for rates list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RatesResponse {
    pub success: bool,
    pub data: Vec<Rate>,
    pub total: i32,
}

/// Response for single rate
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateResponse {
    pub success: bool,
    pub rate: Rate,
}

/// Request body for creating/updating rate
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

/// Response for mutation operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/rates - List all rates
pub async fn list_rates(
    State(state): State<AppState>,
    Query(params): Query<RatesQuery>,
) -> ApiResult<Json<RatesResponse>> {
    let pool = &state.new_pool;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(room_type_id) = params.room_type_id {
        conditions.push(format!("r.rate_room_type_id = {}", room_type_id));
    }

    if let Some(active) = params.active {
        conditions.push(format!("r.rate_active = {}", if active { "true" } else { "false" }));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
        r#"
        SELECT
            r.rate_id,
            r.rate_name,
            r.rate_room_type_id,
            rt.type_name,
            r.rate_type,
            r.rate_value,
            r.rate_valid_from,
            r.rate_valid_to,
            r.rate_days_of_week,
            r.rate_active,
            r.rate_created,
            r.rate_updated
        FROM ht_rates r
        LEFT JOIN ht_room_types rt ON r.rate_room_type_id = rt.type_id
        {}
        ORDER BY r.rate_name ASC
        "#,
        where_clause
    );

    let rows = sqlx::query(&query).fetch_all(pool).await?;

    let rates: Vec<Rate> = rows
        .iter()
        .map(|row| Rate {
            id: row.try_get::<i32, _>("rate_id").unwrap_or(0),
            name: row.try_get::<String, _>("rate_name").unwrap_or_default(),
            room_type_id: row.try_get::<i32, _>("rate_room_type_id").ok(),
            room_type_name: row.try_get::<String, _>("type_name").ok(),
            rate_type: row.try_get::<String, _>("rate_type").unwrap_or_else(|_| "multiplier".to_string()),
            value: row.try_get::<f64, _>("rate_value").unwrap_or(1.0),
            valid_from: row.try_get::<NaiveDate, _>("rate_valid_from").ok(),
            valid_to: row.try_get::<NaiveDate, _>("rate_valid_to").ok(),
            days_of_week: row.try_get::<String, _>("rate_days_of_week").ok(),
            active: row.try_get::<bool, _>("rate_active").unwrap_or(true),
            created_at: row.try_get::<chrono::NaiveDateTime, _>("rate_created").ok(),
            updated_at: row.try_get::<chrono::NaiveDateTime, _>("rate_updated").ok(),
        })
        .collect();

    let total = rates.len() as i32;

    Ok(Json(RatesResponse {
        success: true,
        data: rates,
        total,
    }))
}

/// GET /api/new/rates/:id - Get single rate
pub async fn get_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
) -> ApiResult<Json<RateResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                r.rate_id,
                r.rate_name,
                r.rate_room_type_id,
                rt.type_name,
                r.rate_type,
                r.rate_value,
                r.rate_valid_from,
                r.rate_valid_to,
                r.rate_days_of_week,
                r.rate_active,
                r.rate_created,
                r.rate_updated
            FROM ht_rates r
            LEFT JOIN ht_room_types rt ON r.rate_room_type_id = rt.type_id
            WHERE r.rate_id = $1
            "#,
        )
        .bind(&rate_id)
        .fetch_all(pool)
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Rate not found".to_string()))?;

    let rate = Rate {
        id: row.try_get::<i32, _>("rate_id").unwrap_or(0),
        name: row.try_get::<String, _>("rate_name").unwrap_or_default(),
        room_type_id: row.try_get::<i32, _>("rate_room_type_id").ok(),
        room_type_name: row.try_get::<String, _>("type_name").ok(),
        rate_type: row.try_get::<String, _>("rate_type").unwrap_or_else(|_| "multiplier".to_string()),
        value: row.try_get::<f64, _>("rate_value").unwrap_or(1.0),
        valid_from: row.try_get::<NaiveDate, _>("rate_valid_from").ok(),
        valid_to: row.try_get::<NaiveDate, _>("rate_valid_to").ok(),
        days_of_week: row.try_get::<String, _>("rate_days_of_week").ok(),
        active: row.try_get::<bool, _>("rate_active").unwrap_or(true),
        created_at: row.try_get::<chrono::NaiveDateTime, _>("rate_created").ok(),
        updated_at: row.try_get::<chrono::NaiveDateTime, _>("rate_updated").ok(),
    };

    Ok(Json(RateResponse {
        success: true,
        rate,
    }))
}

/// POST /api/new/rates - Create a rate
pub async fn create_rate(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateRateRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Rate name is required".to_string()));
    }

    // Validate rate_type
    let rate_type = body.rate_type.to_lowercase();
    if rate_type != "multiplier" && rate_type != "fixed" {
        return Err(ApiError::BadRequest("Rate type must be 'multiplier' or 'fixed'".to_string()));
    }

    let pool = &state.new_pool;

    let active = body.active.unwrap_or(true);

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_rates (
                rate_name,
                rate_room_type_id,
                rate_type,
                rate_value,
                rate_valid_from,
                rate_valid_to,
                rate_days_of_week,
                rate_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING rate_id
            "#,
        )
        .bind(&name)
        .bind(&body.room_type_id)
        .bind(&rate_type.as_str())
        .bind(&body.value)
        .bind(&body.valid_from.as_deref())
        .bind(&body.valid_to.as_deref())
        .bind(&body.days_of_week.as_deref())
        .bind(&active)
        .fetch_all(pool)
        .await?;

    let rate_id = rows
        .first()
        .map(|r| r.try_get::<i32, _>("rate_id").ok())
        .flatten()
        .ok_or_else(|| ApiError::Internal("Failed to create rate".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate created successfully".to_string(),
        id: Some(rate_id),
    }))
}

/// PUT /api/new/rates/:id - Update rate
pub async fn update_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
    Json(body): Json<CreateUpdateRateRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Rate name is required".to_string()));
    }

    // Validate rate_type
    let rate_type = body.rate_type.to_lowercase();
    if rate_type != "multiplier" && rate_type != "fixed" {
        return Err(ApiError::BadRequest("Rate type must be 'multiplier' or 'fixed'".to_string()));
    }

    let pool = &state.new_pool;

    // Check if rate exists
    let check_rows = sqlx::query(
            "SELECT rate_id FROM ht_rates WHERE rate_id = $1",
        )
        .bind(&rate_id)
        .fetch_all(pool)
        .await?;

    if check_rows.is_empty() {
        return Err(ApiError::NotFound("Rate not found".to_string()));
    }

    let active = body.active.unwrap_or(true);

    sqlx::query(
        r#"
        UPDATE ht_rates
        SET rate_name = $1,
            rate_room_type_id = $2,
            rate_type = $3,
            rate_value = $4,
            rate_valid_from = $5,
            rate_valid_to = $6,
            rate_days_of_week = $7,
            rate_active = $8,
            rate_updated = NOW()
        WHERE rate_id = $9
        "#,
    )
    .bind(&name)
    .bind(&body.room_type_id)
    .bind(&rate_type.as_str())
    .bind(&body.value)
    .bind(&body.valid_from.as_deref())
    .bind(&body.valid_to.as_deref())
    .bind(&body.days_of_week.as_deref())
    .bind(&active)
    .bind(&rate_id)
    .execute(pool)
    .await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate updated successfully".to_string(),
        id: Some(rate_id),
    }))
}

/// DELETE /api/new/rates/:id - Delete rate
pub async fn delete_rate(
    State(state): State<AppState>,
    Path(rate_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    let result = sqlx::query(
            "DELETE FROM ht_rates WHERE rate_id = $1",
        )
        .bind(&rate_id)
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
