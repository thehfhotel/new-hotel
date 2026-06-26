//! Data models for API responses

mod booking;
mod checkin;
mod customer;
mod note;

pub use booking::*;
pub use checkin::*;
pub use customer::*;
pub use note::*;

use serde::Serialize;

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Pagination information for list responses
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i32,
    pub limit: i32,
    pub total: i32,
    pub total_pages: i32,
}

impl Pagination {
    pub fn new(page: i32, limit: i32, total: i32) -> Self {
        Self {
            page,
            limit,
            total,
            total_pages: (total as f64 / limit as f64).ceil() as i32,
        }
    }
}

/// Response with data and pagination
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub pagination: Pagination,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: T, page: i32, limit: i32, total: i32) -> Self {
        Self {
            success: true,
            data,
            pagination: Pagination::new(page, limit, total),
        }
    }
}
