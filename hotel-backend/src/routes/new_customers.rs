//! New Customer API routes for HotelNew database
//!
//! - GET /api/new/customers - List customers from HT_Customers
//! - GET /api/new/customers/:id - Get single customer
//! - POST /api/new/customers - Create customer
//! - PUT /api/new/customers/:id - Update customer
//! - DELETE /api/new/customers/:id - Soft delete (set Cust_Active=0)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

/// Customer from HT_Customers table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Query parameters for customers list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomersQuery {
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    #[serde(default = "default_active_only")]
    pub active_only: bool,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }
fn default_active_only() -> bool { true }

/// Response for customers list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomersResponse {
    pub success: bool,
    pub data: Vec<NewCustomer>,
    pub pagination: Pagination,
}

/// Response for single customer
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerResponse {
    pub success: bool,
    pub customer: NewCustomer,
}

/// Request body for creating/updating customer
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateCustomerRequest {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
}

/// Response for create/update/delete operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/customers - List customers from HT_Customers
pub async fn list_customers(
    State(state): State<AppState>,
    Query(params): Query<NewCustomersQuery>,
) -> ApiResult<Json<NewCustomersResponse>> {
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("firstName") => "cust_firstname",
        Some("lastName") => "cust_lastname",
        Some("phone") => "cust_phone",
        Some("email") => "cust_email",
        Some("createdAt") => "created_at",
        _ => "cust_id",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if params.active_only {
        conditions.push("cust_active = true".to_string());
    }

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(cust_firstname LIKE '%{}%' OR cust_lastname LIKE '%{}%' OR cust_phone LIKE '%{}%' OR cust_email LIKE '%{}%' OR cust_idcard LIKE '%{}%')",
            escaped, escaped, escaped, escaped, escaped
        ));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_customers {}",
        where_clause
    );

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            cust_id,
            cust_firstname,
            cust_lastname,
            cust_phone,
            cust_email,
            cust_idcard,
            cust_address,
            cust_type,
            cust_notes,
            cust_active,
            created_at,
            updated_at
        FROM ht_customers
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, order_by_column, sort_order, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let customers: Vec<NewCustomer> = rows
        .iter()
        .map(|row| NewCustomer {
            id: row.try_get::<i32, _>("cust_id").unwrap_or(0),
            first_name: row.try_get::<String, _>("cust_firstname").ok(),
            last_name: row.try_get::<String, _>("cust_lastname").ok(),
            phone: row.try_get::<String, _>("cust_phone").ok(),
            email: row.try_get::<String, _>("cust_email").ok(),
            id_card: row.try_get::<String, _>("cust_idcard").ok(),
            address: row.try_get::<String, _>("cust_address").ok(),
            customer_type: row.try_get::<String, _>("cust_type").ok(),
            notes: row.try_get::<String, _>("cust_notes").ok(),
            active: row.try_get::<bool, _>("cust_active").unwrap_or(true),
            created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
        })
        .collect();

    Ok(Json(NewCustomersResponse {
        success: true,
        data: customers,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/customers/:id - Get single customer
pub async fn get_customer(
    State(state): State<AppState>,
    Path(cust_id): Path<i32>,
) -> ApiResult<Json<NewCustomerResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                cust_id,
                cust_firstname,
                cust_lastname,
                cust_phone,
                cust_email,
                cust_idcard,
                cust_address,
                cust_type,
                cust_notes,
                cust_active,
                created_at,
                updated_at
            FROM ht_customers
            WHERE cust_id = $1
            "#,
        )
        .bind(&cust_id)
        .fetch_all(pool)
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    let customer = NewCustomer {
        id: row.try_get::<i32, _>("cust_id").unwrap_or(0),
        first_name: row.try_get::<String, _>("cust_firstname").ok(),
        last_name: row.try_get::<String, _>("cust_lastname").ok(),
        phone: row.try_get::<String, _>("cust_phone").ok(),
        email: row.try_get::<String, _>("cust_email").ok(),
        id_card: row.try_get::<String, _>("cust_idcard").ok(),
        address: row.try_get::<String, _>("cust_address").ok(),
        customer_type: row.try_get::<String, _>("cust_type").ok(),
        notes: row.try_get::<String, _>("cust_notes").ok(),
        active: row.try_get::<bool, _>("cust_active").unwrap_or(true),
        created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
        updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
    };

    Ok(Json(NewCustomerResponse {
        success: true,
        customer,
    }))
}

/// POST /api/new/customers - Create customer
pub async fn create_customer(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateCustomerRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let first_name = body.first_name.trim();
    if first_name.is_empty() {
        return Err(ApiError::BadRequest("First name is required".to_string()));
    }

    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_customers (
                cust_firstname,
                cust_lastname,
                cust_phone,
                cust_email,
                cust_idcard,
                cust_address,
                cust_type,
                cust_notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING cust_id
            "#,
        )
        .bind(&first_name)
        .bind(&body.last_name.as_deref())
        .bind(&body.phone.as_deref())
        .bind(&body.email.as_deref())
        .bind(&body.id_card.as_deref())
        .bind(&body.address.as_deref())
        .bind(&body.customer_type.as_deref())
        .bind(&body.notes.as_deref())
        .fetch_all(pool)
        .await?;

    let id = rows
        .first()
        .map(|r| r.try_get::<i32, _>("cust_id").ok())
        .flatten()
        .ok_or_else(|| ApiError::Internal("Failed to create customer".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer created successfully".to_string(),
        id: Some(id),
    }))
}

/// PUT /api/new/customers/:id - Update customer
pub async fn update_customer(
    State(state): State<AppState>,
    Path(cust_id): Path<i32>,
    Json(body): Json<CreateUpdateCustomerRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let first_name = body.first_name.trim();
    if first_name.is_empty() {
        return Err(ApiError::BadRequest("First name is required".to_string()));
    }

    let pool = &state.new_pool;

    let result = sqlx::query(
            r#"
            UPDATE ht_customers
            SET cust_firstname = $1,
                cust_lastname = $2,
                cust_phone = $3,
                cust_email = $4,
                cust_idcard = $5,
                cust_address = $6,
                cust_type = $7,
                cust_notes = $8,
                updated_at = NOW()
            WHERE cust_id = $9
            "#,
        )
        .bind(&first_name)
        .bind(&body.last_name.as_deref())
        .bind(&body.phone.as_deref())
        .bind(&body.email.as_deref())
        .bind(&body.id_card.as_deref())
        .bind(&body.address.as_deref())
        .bind(&body.customer_type.as_deref())
        .bind(&body.notes.as_deref())
        .bind(&cust_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Customer not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer updated successfully".to_string(),
        id: Some(cust_id),
    }))
}

/// DELETE /api/new/customers/:id - Soft delete (set Cust_Active=0)
pub async fn delete_customer(
    State(state): State<AppState>,
    Path(cust_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    let result = sqlx::query(
            r#"
            UPDATE ht_customers
            SET cust_active = false,
                updated_at = NOW()
            WHERE cust_id = $1
            "#,
        )
        .bind(&cust_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Customer not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer deleted successfully".to_string(),
        id: Some(cust_id),
    }))
}
