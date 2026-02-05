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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("firstName") => "Cust_FirstName",
        Some("lastName") => "Cust_LastName",
        Some("phone") => "Cust_Phone",
        Some("email") => "Cust_Email",
        Some("createdAt") => "Created_At",
        _ => "Cust_ID",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if params.active_only {
        conditions.push("Cust_Active = 1".to_string());
    }

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(Cust_FirstName LIKE '%{}%' OR Cust_LastName LIKE '%{}%' OR Cust_Phone LIKE '%{}%' OR Cust_Email LIKE '%{}%' OR Cust_IDCard LIKE '%{}%')",
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
        "SELECT COUNT(*) as total FROM HT_Customers {}",
        where_clause
    );

    let count_rows = conn
        .simple_query(&count_query)
        .await?
        .into_first_result()
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.get::<i32, _>("total"))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            Cust_ID,
            Cust_FirstName,
            Cust_LastName,
            Cust_Phone,
            Cust_Email,
            Cust_IDCard,
            Cust_Address,
            Cust_Type,
            Cust_Notes,
            Cust_Active,
            Created_At,
            Updated_At
        FROM HT_Customers
        {}
        ORDER BY {} {}
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, order_by_column, sort_order, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let customers: Vec<NewCustomer> = rows
        .iter()
        .map(|row| NewCustomer {
            id: row.get::<i32, _>("Cust_ID").unwrap_or(0),
            first_name: row.get::<&str, _>("Cust_FirstName").map(String::from),
            last_name: row.get::<&str, _>("Cust_LastName").map(String::from),
            phone: row.get::<&str, _>("Cust_Phone").map(String::from),
            email: row.get::<&str, _>("Cust_Email").map(String::from),
            id_card: row.get::<&str, _>("Cust_IDCard").map(String::from),
            address: row.get::<&str, _>("Cust_Address").map(String::from),
            customer_type: row.get::<&str, _>("Cust_Type").map(String::from),
            notes: row.get::<&str, _>("Cust_Notes").map(String::from),
            active: row.get::<bool, _>("Cust_Active").unwrap_or(true),
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                Cust_ID,
                Cust_FirstName,
                Cust_LastName,
                Cust_Phone,
                Cust_Email,
                Cust_IDCard,
                Cust_Address,
                Cust_Type,
                Cust_Notes,
                Cust_Active,
                Created_At,
                Updated_At
            FROM HT_Customers
            WHERE Cust_ID = @P1
            "#,
            &[&cust_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    let customer = NewCustomer {
        id: row.get::<i32, _>("Cust_ID").unwrap_or(0),
        first_name: row.get::<&str, _>("Cust_FirstName").map(String::from),
        last_name: row.get::<&str, _>("Cust_LastName").map(String::from),
        phone: row.get::<&str, _>("Cust_Phone").map(String::from),
        email: row.get::<&str, _>("Cust_Email").map(String::from),
        id_card: row.get::<&str, _>("Cust_IDCard").map(String::from),
        address: row.get::<&str, _>("Cust_Address").map(String::from),
        customer_type: row.get::<&str, _>("Cust_Type").map(String::from),
        notes: row.get::<&str, _>("Cust_Notes").map(String::from),
        active: row.get::<bool, _>("Cust_Active").unwrap_or(true),
        created_at: row.get::<NaiveDateTime, _>("Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
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

    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Customers (
                Cust_FirstName,
                Cust_LastName,
                Cust_Phone,
                Cust_Email,
                Cust_IDCard,
                Cust_Address,
                Cust_Type,
                Cust_Notes
            )
            OUTPUT INSERTED.Cust_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8)
            "#,
            &[
                &first_name,
                &body.last_name.as_deref(),
                &body.phone.as_deref(),
                &body.email.as_deref(),
                &body.id_card.as_deref(),
                &body.address.as_deref(),
                &body.customer_type.as_deref(),
                &body.notes.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Cust_ID"))
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

    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            r#"
            UPDATE HT_Customers
            SET Cust_FirstName = @P1,
                Cust_LastName = @P2,
                Cust_Phone = @P3,
                Cust_Email = @P4,
                Cust_IDCard = @P5,
                Cust_Address = @P6,
                Cust_Type = @P7,
                Cust_Notes = @P8,
                Updated_At = GETDATE()
            WHERE Cust_ID = @P9
            "#,
            &[
                &first_name,
                &body.last_name.as_deref(),
                &body.phone.as_deref(),
                &body.email.as_deref(),
                &body.id_card.as_deref(),
                &body.address.as_deref(),
                &body.customer_type.as_deref(),
                &body.notes.as_deref(),
                &cust_id,
            ],
        )
        .await?;

    if result.total() == 0 {
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
    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            r#"
            UPDATE HT_Customers
            SET Cust_Active = 0,
                Updated_At = GETDATE()
            WHERE Cust_ID = @P1
            "#,
            &[&cust_id],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Customer not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer deleted successfully".to_string(),
        id: Some(cust_id),
    }))
}
