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
        // `limit <= 0` short-circuits: `total / 0.0` is `inf`, and `inf as i32`
        // saturates, so `?limit=0` used to report `"totalPages": 2147483647`.
        // No page can be served at all in that case, so 0 is the honest answer.
        // Fixed here rather than per-caller because all eleven `Pagination::new`
        // call sites take `limit` straight from the query string, none can be
        // relying on the saturated value, and the arithmetic for `limit > 0` is
        // untouched.
        let total_pages = if limit > 0 {
            (total as f64 / limit as f64).ceil() as i32
        } else {
            0
        };
        Self {
            page,
            limit,
            total,
            total_pages,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_pages_rounds_up_for_a_positive_limit() {
        assert_eq!(Pagination::new(1, 50, 0).total_pages, 0);
        assert_eq!(Pagination::new(1, 50, 50).total_pages, 1);
        assert_eq!(Pagination::new(1, 50, 51).total_pages, 2);
    }

    /// `?limit=0` divided by zero, and `inf as i32` saturates — the payload
    /// claimed `"totalPages": 2147483647`. A non-positive limit serves no
    /// pages at all, so it must report none.
    #[test]
    fn non_positive_limit_reports_zero_total_pages() {
        assert_eq!(Pagination::new(1, 0, 120).total_pages, 0);
        assert_eq!(Pagination::new(1, -20, 120).total_pages, 0);
    }
}
