//! Booking models

use chrono::NaiveDateTime;
use serde::Serialize;

use super::note::Note;
use super::Pagination;

/// Booking room info
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRoom {
    pub room_no: String,
    pub room_type: String,
}

/// Customer info in booking
#[derive(Debug, Serialize)]
pub struct BookingCustomer {
    pub name: String,
}

/// Booking data for list view
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Booking {
    pub book_no: String,
    pub book_date: Option<NaiveDateTime>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub customer: BookingCustomer,
    pub status: String,
    pub rooms: Vec<BookingRoom>,
    pub room_count: usize,
}

/// Bookings list response
#[derive(Debug, Serialize)]
pub struct BookingsResponse {
    pub success: bool,
    pub data: Vec<Booking>,
    pub pagination: Pagination,
}

/// Detailed customer info for booking detail
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingCustomerDetail {
    pub full_name: String,
}

/// Room with total for booking detail
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRoomDetail {
    pub room_no: String,
    pub room_type: String,
    pub total: f64,
}

/// Detailed booking information
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingDetail {
    pub book_no: String,
    pub book_date: Option<NaiveDateTime>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub status: String,
    pub status_code: Option<i32>,
    pub customer: BookingCustomerDetail,
    pub rooms: Vec<BookingRoomDetail>,
    pub room_count: usize,
    pub total_amount: f64,
    pub notes: Vec<Note>,
}

/// Booking detail response
#[derive(Debug, Serialize)]
pub struct BookingDetailResponse {
    pub success: bool,
    pub booking: BookingDetail,
}

/// Map status code to Thai text
pub fn map_status(status: Option<i32>) -> String {
    match status {
        Some(1) => "จอง".to_string(),
        Some(2) => "เข้าพัก".to_string(),
        Some(3) => "เสร็จสิ้น".to_string(),
        Some(4) => "ยกเลิก".to_string(),
        _ => "ไม่ทราบ".to_string(),
    }
}

/// Map Thai status text to status code
pub fn get_status_code(status_text: &str) -> Option<i32> {
    match status_text {
        "จอง" => Some(1),
        "เข้าพัก" => Some(2),
        "เสร็จสิ้น" => Some(3),
        "ยกเลิก" => Some(4),
        _ => None,
    }
}
