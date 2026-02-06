//! Check-in models

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::Pagination;

/// Check-in record from View_CheckIn_Ds
///
/// Note: DateTime fields use `DateTime<Utc>` so they serialize with "Z" suffix.
/// The legacy SQL Server stores Thai local time as naive datetime. We treat these
/// as UTC for serialization so the frontend can display them as-is with `timeZone: 'UTC'`.
#[derive(Debug, Serialize)]
pub struct CheckIn {
    #[serde(rename = "Cin_no")]
    pub cin_no: Option<String>,
    #[serde(rename = "Cin_Room_No")]
    pub cin_room_no: Option<String>,
    #[serde(rename = "Cin_Room_In")]
    pub cin_room_in: Option<DateTime<Utc>>,
    #[serde(rename = "Cin_Room_Out")]
    pub cin_room_out: Option<DateTime<Utc>>,
    #[serde(rename = "Cin_cust_name")]
    pub cin_cust_name: Option<String>,
    #[serde(rename = "Cin_status")]
    pub cin_status: Option<String>,
}

/// Check-ins list response
#[derive(Debug, Serialize)]
pub struct CheckInsResponse {
    pub success: bool,
    pub data: Vec<CheckIn>,
    pub pagination: Pagination,
}
