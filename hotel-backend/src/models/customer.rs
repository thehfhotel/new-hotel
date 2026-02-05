//! Customer models

use chrono::NaiveDateTime;
use serde::Serialize;

/// Customer data from View_Customers
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub customer_type: Option<String>,
    pub phone: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_visit: Option<NaiveDateTime>,
}

/// Customers list response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomersResponse {
    pub success: bool,
    pub customers: Vec<Customer>,
    pub total: i32,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
}

/// Customer booking for booking history
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerBooking {
    pub id: String,
    pub room_number: String,
    pub room_type: String,
    pub check_in_date: Option<NaiveDateTime>,
    pub check_out_date: Option<NaiveDateTime>,
    pub status: String,
    pub total_amount: f64,
}

/// Customer bookings response
#[derive(Debug, Serialize)]
pub struct CustomerBookingsResponse {
    pub success: bool,
    pub bookings: Vec<CustomerBooking>,
}

/// Customer statistics
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerStats {
    pub total_bookings: i32,
    pub total_stays: i32,
    pub first_visit: Option<NaiveDateTime>,
    pub last_visit: Option<NaiveDateTime>,
    pub favorite_room_type: Option<String>,
    pub avg_stay_days: Option<f64>,
}

/// Customer stats response
#[derive(Debug, Serialize)]
pub struct CustomerStatsResponse {
    pub success: bool,
    pub stats: CustomerStats,
}
