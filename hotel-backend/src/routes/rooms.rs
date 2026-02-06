//! Room API routes
//!
//! - GET /api/rooms - List all rooms
//! - GET /api/rooms/:id - Get room details with current guest
//! - GET /api/rooms/status - Get room status history
//! - GET /api/rooms/checkouts-today - Get rooms with checkout today

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use tiberius::Row;

use crate::db::DbPool;
use crate::error::{ApiError, ApiResult};
use crate::models::{
    CheckoutsTodayResponse, CurrentGuest, Room, RoomDetail, RoomDetailResponse, RoomStatus,
    RoomStatusResponse, RoomsResponse,
};

/// GET /api/rooms - List all rooms
pub async fn list_rooms(State(pool): State<DbPool>) -> ApiResult<Json<RoomsResponse>> {
    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Room_no,
                Room_Type,
                Room_Details,
                Room_Clean,
                Room_Use,
                Room_Book,
                Room_Manternace,
                Room_PriceA,
                Room_PriceB,
                Room_PriceC,
                Room_Group,
                Room_Book_Name
            FROM HT_Rooms
            ORDER BY Room_no
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let rooms: Vec<Room> = rows
        .iter()
        .map(|row| Room {
            room_no: row.get::<&str, _>("Room_no").unwrap_or_default().to_string(),
            room_type: row.get::<&str, _>("Room_Type").map(String::from),
            room_details: row.get::<&str, _>("Room_Details").map(String::from),
            room_clean: row.get::<&str, _>("Room_Clean").map(String::from),
            room_use: row.get::<&str, _>("Room_Use").map(String::from),
            room_book: row.get::<&str, _>("Room_Book").map(String::from),
            room_manternace: row.get::<&str, _>("Room_Manternace").map(String::from),
            room_price_a: row.get::<f64, _>("Room_PriceA"),
            room_price_b: row.get::<f64, _>("Room_PriceB"),
            room_price_c: row.get::<f64, _>("Room_PriceC"),
            room_group: row.get::<&str, _>("Room_Group").map(String::from),
            room_book_name: row.get::<&str, _>("Room_Book_Name").map(String::from),
        })
        .collect();

    let total = rooms.len();

    Ok(Json(RoomsResponse {
        success: true,
        data: rooms,
        total,
    }))
}

/// GET /api/rooms/:id - Get room details with current guest
pub async fn get_room(
    State(pool): State<DbPool>,
    Path(room_no): Path<String>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let mut conn = pool.get().await?;

    // Get room details
    let room_rows = conn
        .query(
            r#"
            SELECT
                Room_no,
                Room_Type,
                Room_Details,
                Room_Clean,
                Room_Use,
                Room_Book,
                Room_Manternace,
                Room_PriceA,
                Room_PriceB,
                Room_PriceC,
                Room_Group,
                Room_Book_Name,
                Room_Book_Time
            FROM HT_Rooms
            WHERE Room_no = @P1
            "#,
            &[&room_no],
        )
        .await?
        .into_first_result()
        .await?;

    let room_row = room_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    // Get current/recent check-in
    let checkin_rows = conn
        .query(
            r#"
            SELECT TOP 1
                Cin_Cust_Name,
                Cin_Room_In,
                Cin_Room_Out
            FROM View_CheckIn_Ds
            WHERE Cin_Room_No = @P1
            ORDER BY Cin_Room_In DESC
            "#,
            &[&room_no],
        )
        .await?
        .into_first_result()
        .await?;

    let current_guest = checkin_rows.first().map(|row| CurrentGuest {
        name: row.get::<&str, _>("Cin_Cust_Name").map(String::from),
        check_in: row.try_get::<NaiveDateTime, _>("Cin_Room_In").ok().flatten().map(|dt| dt.and_utc()),
        check_out: row.try_get::<NaiveDateTime, _>("Cin_Room_Out").ok().flatten().map(|dt| dt.and_utc()),
    });

    let room = RoomDetail {
        room_no: room_row
            .get::<&str, _>("Room_no")
            .unwrap_or_default()
            .to_string(),
        room_type: room_row.get::<&str, _>("Room_Type").map(String::from),
        room_details: room_row.get::<&str, _>("Room_Details").map(String::from),
        room_clean: room_row.get::<&str, _>("Room_Clean").map(String::from),
        room_use: room_row.get::<&str, _>("Room_Use").map(String::from),
        room_book: room_row.get::<&str, _>("Room_Book").map(String::from),
        room_manternace: room_row.get::<&str, _>("Room_Manternace").map(String::from),
        room_price_a: room_row.get::<f64, _>("Room_PriceA"),
        room_price_b: room_row.get::<f64, _>("Room_PriceB"),
        room_price_c: room_row.get::<f64, _>("Room_PriceC"),
        room_group: room_row.get::<&str, _>("Room_Group").map(String::from),
        room_book_name: room_row.get::<&str, _>("Room_Book_Name").map(String::from),
        room_book_time: room_row.try_get::<NaiveDateTime, _>("Room_Book_Time").ok().flatten().map(|dt| dt.and_utc()),
        current_guest,
    };

    Ok(Json(RoomDetailResponse {
        success: true,
        room,
    }))
}

/// Query parameters for room status
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomStatusQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// GET /api/rooms/status - Get room status history
pub async fn get_room_status(
    State(pool): State<DbPool>,
    Query(params): Query<RoomStatusQuery>,
) -> ApiResult<Json<RoomStatusResponse>> {
    let mut conn = pool.get().await?;

    // Build dynamic query
    let mut query = String::from(
        r#"
        SELECT
            room_no,
            room_date,
            room_status,
            room_Details,
            room_CheckIn_No,
            Room_Type
        FROM View_Room_status
        "#,
    );

    let mut conditions: Vec<String> = Vec::new();

    if params.start_date.is_some() {
        conditions.push("room_date >= @P1".to_string());
    }

    if params.end_date.is_some() {
        conditions.push(format!(
            "room_date <= @P{}",
            if params.start_date.is_some() { 2 } else { 1 }
        ));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY room_no, room_date");

    // Execute based on parameters
    let rows: Vec<Row> = match (&params.start_date, &params.end_date) {
        (Some(start), Some(end)) => {
            conn.query(&query, &[&start.as_str(), &end.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (Some(start), None) => {
            conn.query(&query, &[&start.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (None, Some(end)) => {
            conn.query(&query, &[&end.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (None, None) => conn.simple_query(&query).await?.into_first_result().await?,
    };

    let statuses: Vec<RoomStatus> = rows
        .iter()
        .map(|row| RoomStatus {
            room_no: row.get::<&str, _>("room_no").unwrap_or_default().to_string(),
            room_date: row.get::<NaiveDateTime, _>("room_date").map(|dt| dt.and_utc()),
            room_status: row.get::<&str, _>("room_status").map(String::from),
            room_details: row.get::<&str, _>("room_Details").map(String::from),
            room_checkin_no: row.get::<&str, _>("room_CheckIn_No").map(String::from),
            room_type: row.get::<&str, _>("Room_Type").map(String::from),
        })
        .collect();

    let total = statuses.len();

    Ok(Json(RoomStatusResponse {
        success: true,
        data: statuses,
        total,
    }))
}

/// GET /api/rooms/checkouts-today - Get rooms with checkout today
pub async fn get_checkouts_today(
    State(pool): State<DbPool>,
) -> ApiResult<Json<CheckoutsTodayResponse>> {
    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT DISTINCT c.Cin_Room_no as room_no
            FROM View_CheckIn_Ds c
            INNER JOIN HT_Rooms r ON c.Cin_Room_No = r.Room_no
            WHERE CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
                AND r.Room_Use = 'yes'
                AND c.Cin_Room_In = (
                    SELECT MAX(c2.Cin_Room_In)
                    FROM View_CheckIn_Ds c2
                    WHERE c2.Cin_Room_No = c.Cin_Room_No
                )
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let room_numbers: Vec<String> = rows
        .iter()
        .filter_map(|row| row.get::<&str, _>("room_no").map(String::from))
        .collect();

    Ok(Json(CheckoutsTodayResponse {
        success: true,
        data: room_numbers,
    }))
}
