//! Room models

use chrono::NaiveDateTime;
use serde::Serialize;

/// Room data from HT_Rooms table
#[derive(Debug, Serialize)]
pub struct Room {
    #[serde(rename = "Room_no")]
    pub room_no: String,
    #[serde(rename = "Room_Type")]
    pub room_type: Option<String>,
    #[serde(rename = "Room_Details")]
    pub room_details: Option<String>,
    #[serde(rename = "Room_Clean")]
    pub room_clean: Option<String>,
    #[serde(rename = "Room_Use")]
    pub room_use: Option<String>,
    #[serde(rename = "Room_Book")]
    pub room_book: Option<String>,
    #[serde(rename = "Room_Manternace")]
    pub room_manternace: Option<String>,
    #[serde(rename = "Room_PriceA")]
    pub room_price_a: Option<f64>,
    #[serde(rename = "Room_PriceB")]
    pub room_price_b: Option<f64>,
    #[serde(rename = "Room_PriceC")]
    pub room_price_c: Option<f64>,
    #[serde(rename = "Room_Group")]
    pub room_group: Option<String>,
    #[serde(rename = "Room_Book_Name")]
    pub room_book_name: Option<String>,
}

/// Room list response
#[derive(Debug, Serialize)]
pub struct RoomsResponse {
    pub success: bool,
    pub data: Vec<Room>,
    pub total: usize,
}

/// Current guest information for a room
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGuest {
    pub name: Option<String>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
}

/// Detailed room information with current guest
#[derive(Debug, Serialize)]
pub struct RoomDetail {
    #[serde(rename = "Room_no")]
    pub room_no: String,
    #[serde(rename = "Room_Type")]
    pub room_type: Option<String>,
    #[serde(rename = "Room_Details")]
    pub room_details: Option<String>,
    #[serde(rename = "Room_Clean")]
    pub room_clean: Option<String>,
    #[serde(rename = "Room_Use")]
    pub room_use: Option<String>,
    #[serde(rename = "Room_Book")]
    pub room_book: Option<String>,
    #[serde(rename = "Room_Manternace")]
    pub room_manternace: Option<String>,
    #[serde(rename = "Room_PriceA")]
    pub room_price_a: Option<f64>,
    #[serde(rename = "Room_PriceB")]
    pub room_price_b: Option<f64>,
    #[serde(rename = "Room_PriceC")]
    pub room_price_c: Option<f64>,
    #[serde(rename = "Room_Group")]
    pub room_group: Option<String>,
    #[serde(rename = "Room_Book_Name")]
    pub room_book_name: Option<String>,
    #[serde(rename = "Room_Book_Time")]
    pub room_book_time: Option<NaiveDateTime>,
    #[serde(rename = "currentGuest")]
    pub current_guest: Option<CurrentGuest>,
}

/// Room detail response
#[derive(Debug, Serialize)]
pub struct RoomDetailResponse {
    pub success: bool,
    pub room: RoomDetail,
}

/// Room status from View_Room_status
#[derive(Debug, Serialize)]
pub struct RoomStatus {
    pub room_no: String,
    pub room_date: Option<NaiveDateTime>,
    pub room_status: Option<String>,
    pub room_details: Option<String>,
    pub room_checkin_no: Option<String>,
    #[serde(rename = "Room_Type")]
    pub room_type: Option<String>,
}

/// Room status response
#[derive(Debug, Serialize)]
pub struct RoomStatusResponse {
    pub success: bool,
    pub data: Vec<RoomStatus>,
    pub total: usize,
}

/// Today's checkout rooms response
#[derive(Debug, Serialize)]
pub struct CheckoutsTodayResponse {
    pub success: bool,
    pub data: Vec<String>,
}
