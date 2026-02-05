//! Check-in models

use chrono::NaiveDateTime;
use serde::Serialize;

use super::Pagination;

/// Check-in record from View_CheckIn_Ds
#[derive(Debug, Serialize)]
pub struct CheckIn {
    #[serde(rename = "Cin_no")]
    pub cin_no: Option<String>,
    #[serde(rename = "Cin_Room_No")]
    pub cin_room_no: Option<String>,
    #[serde(rename = "Cin_Room_In")]
    pub cin_room_in: Option<NaiveDateTime>,
    #[serde(rename = "Cin_Room_Out")]
    pub cin_room_out: Option<NaiveDateTime>,
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
