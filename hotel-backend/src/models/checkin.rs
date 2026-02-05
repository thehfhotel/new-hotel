//! Check-in models

use chrono::NaiveDateTime;
use serde::Serialize;

use super::Pagination;

/// Check-in record from View_CheckIn_Ds
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckIn {
    pub cin_no: Option<String>,
    pub cin_room_no: Option<String>,
    pub cin_room_in: Option<NaiveDateTime>,
    pub cin_room_out: Option<NaiveDateTime>,
    pub cin_cust_name: Option<String>,
    pub cin_status: Option<String>,
}

/// Check-ins list response
#[derive(Debug, Serialize)]
pub struct CheckInsResponse {
    pub success: bool,
    pub data: Vec<CheckIn>,
    pub pagination: Pagination,
}
