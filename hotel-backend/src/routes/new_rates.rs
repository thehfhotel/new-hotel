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
    let mut conn = state.new_pool.get().await?;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(room_type_id) = params.room_type_id {
        conditions.push(format!("r.Rate_Room_Type_ID = {}", room_type_id));
    }

    if let Some(active) = params.active {
        conditions.push(format!("r.Rate_Active = {}", if active { 1 } else { 0 }));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
        r#"
        SELECT
            r.Rate_ID,
            r.Rate_Name,
            r.Rate_Room_Type_ID,
            rt.Type_Name,
            r.Rate_Type,
            r.Rate_Value,
            r.Rate_Valid_From,
            r.Rate_Valid_To,
            r.Rate_Days_Of_Week,
            r.Rate_Active,
            r.Rate_Created,
            r.Rate_Updated
        FROM HT_Rates r
        LEFT JOIN HT_Room_Types rt ON r.Rate_Room_Type_ID = rt.Type_ID
        {}
        ORDER BY r.Rate_Name ASC
        "#,
        where_clause
    );

    let rows = conn
        .simple_query(&query)
        .await?
        .into_first_result()
        .await?;

    let rates: Vec<Rate> = rows
        .iter()
        .map(|row| Rate {
            id: row.get::<i32, _>("Rate_ID").unwrap_or(0),
            name: row.get::<&str, _>("Rate_Name").unwrap_or_default().to_string(),
            room_type_id: row.get::<i32, _>("Rate_Room_Type_ID"),
            room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
            rate_type: row.get::<&str, _>("Rate_Type").unwrap_or("multiplier").to_string(),
            value: row.get::<f64, _>("Rate_Value").unwrap_or(1.0),
            valid_from: row.get::<NaiveDate, _>("Rate_Valid_From"),
            valid_to: row.get::<NaiveDate, _>("Rate_Valid_To"),
            days_of_week: row.get::<&str, _>("Rate_Days_Of_Week").map(String::from),
            active: row.get::<bool, _>("Rate_Active").unwrap_or(true),
            created_at: row.get::<chrono::NaiveDateTime, _>("Rate_Created"),
            updated_at: row.get::<chrono::NaiveDateTime, _>("Rate_Updated"),
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                r.Rate_ID,
                r.Rate_Name,
                r.Rate_Room_Type_ID,
                rt.Type_Name,
                r.Rate_Type,
                r.Rate_Value,
                r.Rate_Valid_From,
                r.Rate_Valid_To,
                r.Rate_Days_Of_Week,
                r.Rate_Active,
                r.Rate_Created,
                r.Rate_Updated
            FROM HT_Rates r
            LEFT JOIN HT_Room_Types rt ON r.Rate_Room_Type_ID = rt.Type_ID
            WHERE r.Rate_ID = @P1
            "#,
            &[&rate_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Rate not found".to_string()))?;

    let rate = Rate {
        id: row.get::<i32, _>("Rate_ID").unwrap_or(0),
        name: row.get::<&str, _>("Rate_Name").unwrap_or_default().to_string(),
        room_type_id: row.get::<i32, _>("Rate_Room_Type_ID"),
        room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
        rate_type: row.get::<&str, _>("Rate_Type").unwrap_or("multiplier").to_string(),
        value: row.get::<f64, _>("Rate_Value").unwrap_or(1.0),
        valid_from: row.get::<NaiveDate, _>("Rate_Valid_From"),
        valid_to: row.get::<NaiveDate, _>("Rate_Valid_To"),
        days_of_week: row.get::<&str, _>("Rate_Days_Of_Week").map(String::from),
        active: row.get::<bool, _>("Rate_Active").unwrap_or(true),
        created_at: row.get::<chrono::NaiveDateTime, _>("Rate_Created"),
        updated_at: row.get::<chrono::NaiveDateTime, _>("Rate_Updated"),
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

    let mut conn = state.new_pool.get().await?;

    let active = body.active.unwrap_or(true);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Rates (
                Rate_Name,
                Rate_Room_Type_ID,
                Rate_Type,
                Rate_Value,
                Rate_Valid_From,
                Rate_Valid_To,
                Rate_Days_Of_Week,
                Rate_Active
            )
            OUTPUT INSERTED.Rate_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8)
            "#,
            &[
                &name,
                &body.room_type_id,
                &rate_type.as_str(),
                &body.value,
                &body.valid_from.as_deref(),
                &body.valid_to.as_deref(),
                &body.days_of_week.as_deref(),
                &active,
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let rate_id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Rate_ID"))
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

    let mut conn = state.new_pool.get().await?;

    // Check if rate exists
    let check_rows = conn
        .query(
            "SELECT Rate_ID FROM HT_Rates WHERE Rate_ID = @P1",
            &[&rate_id],
        )
        .await?
        .into_first_result()
        .await?;

    if check_rows.is_empty() {
        return Err(ApiError::NotFound("Rate not found".to_string()));
    }

    let active = body.active.unwrap_or(true);

    conn.execute(
        r#"
        UPDATE HT_Rates
        SET Rate_Name = @P1,
            Rate_Room_Type_ID = @P2,
            Rate_Type = @P3,
            Rate_Value = @P4,
            Rate_Valid_From = @P5,
            Rate_Valid_To = @P6,
            Rate_Days_Of_Week = @P7,
            Rate_Active = @P8,
            Rate_Updated = GETDATE()
        WHERE Rate_ID = @P9
        "#,
        &[
            &name,
            &body.room_type_id,
            &rate_type.as_str(),
            &body.value,
            &body.valid_from.as_deref(),
            &body.valid_to.as_deref(),
            &body.days_of_week.as_deref(),
            &active,
            &rate_id,
        ],
    )
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
    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            "DELETE FROM HT_Rates WHERE Rate_ID = @P1",
            &[&rate_id],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Rate not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Rate deleted successfully".to_string(),
        id: Some(rate_id),
    }))
}
