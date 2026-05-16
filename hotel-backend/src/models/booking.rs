//! Booking models

use chrono::{DateTime, Utc};
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
    pub book_date: Option<DateTime<Utc>>,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
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
    pub book_date: Option<DateTime<Utc>>,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
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

/// Map legacy status code (INT, from `ht_bookings_legacy.book_status`) to
/// Thai display text. Kept for the cache-fallback path; the canonical
/// path uses [`map_canonical_status_to_thai`] instead.
pub fn map_status(status: Option<i32>) -> String {
    match status {
        Some(1) => "จอง".to_string(),
        Some(2) => "เข้าพัก".to_string(),
        Some(3) => "เสร็จสิ้น".to_string(),
        Some(4) => "ยกเลิก".to_string(),
        _ => "ไม่ทราบ".to_string(),
    }
}

/// Map Thai status text to legacy INT status code. Used to translate
/// frontend filter input into the cache schema's `book_status` column.
pub fn get_status_code(status_text: &str) -> Option<i32> {
    match status_text {
        "จอง" => Some(1),
        "เข้าพัก" => Some(2),
        "เสร็จสิ้น" => Some(3),
        "ยกเลิก" => Some(4),
        _ => None,
    }
}

/// Map canonical English status (`ht_bookings.book_status` — written by
/// the CT booking mapper from `HT_Book_H.Book_Status`) to Thai display
/// text. The canonical schema stores `'confirmed'` / `'checked_in'` /
/// `'completed'` / `'cancelled'` / `'pending'` per
/// `sync::mappers::booking::legacy_status_to_pg`.
pub fn map_canonical_status_to_thai(status: Option<&str>) -> String {
    match status {
        Some("confirmed") => "จอง".to_string(),
        Some("checked_in") => "เข้าพัก".to_string(),
        Some("completed") => "เสร็จสิ้น".to_string(),
        Some("cancelled") => "ยกเลิก".to_string(),
        // `pending` is the catch-all in `legacy_status_to_pg` —
        // anything that didn't parse cleanly on the legacy side. Map
        // to the same "unknown" literal the legacy INT path uses.
        _ => "ไม่ทราบ".to_string(),
    }
}

/// Map Thai status text to canonical English. Inverse of
/// [`map_canonical_status_to_thai`] — used to translate frontend
/// filter input into the canonical schema's `book_status` column.
pub fn get_canonical_status_from_thai(status_text: &str) -> Option<&'static str> {
    match status_text {
        "จอง" => Some("confirmed"),
        "เข้าพัก" => Some("checked_in"),
        "เสร็จสิ้น" => Some("completed"),
        "ยกเลิก" => Some("cancelled"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_canonical_status_round_trips_via_get_canonical_status_from_thai() {
        // Every Thai literal the frontend may filter on must map to a
        // canonical English status that, when piped back through
        // map_canonical_status_to_thai, yields the original Thai. If
        // this regresses, the bookings list status filter silently
        // drops to "ไม่ทราบ" for valid input.
        for thai in ["จอง", "เข้าพัก", "เสร็จสิ้น", "ยกเลิก"] {
            let english = get_canonical_status_from_thai(thai)
                .unwrap_or_else(|| panic!("missing canonical mapping for {thai}"));
            assert_eq!(
                map_canonical_status_to_thai(Some(english)),
                thai,
                "round-trip failed for canonical={english} thai={thai}"
            );
        }
    }

    #[test]
    fn map_canonical_status_returns_unknown_literal_for_pending_or_other() {
        // canonical 'pending' is the catch-all `legacy_status_to_pg`
        // assigns to anything it doesn't recognise on the legacy side.
        // The bookings UI shows "ไม่ทราบ" (unknown) for these, matching
        // the legacy INT path's default arm.
        assert_eq!(map_canonical_status_to_thai(Some("pending")), "ไม่ทราบ");
        assert_eq!(map_canonical_status_to_thai(Some("bogus")), "ไม่ทราบ");
        assert_eq!(map_canonical_status_to_thai(None), "ไม่ทราบ");
    }

    #[test]
    fn get_canonical_status_from_thai_returns_none_for_unknown_input() {
        // A NULL return from get_canonical_status_from_thai tells the
        // list endpoint to ignore the status filter (don't add a WHERE
        // clause for unrecognised Thai literals). Verifies that
        // contract.
        assert!(get_canonical_status_from_thai("ไม่ทราบ").is_none());
        assert!(get_canonical_status_from_thai("").is_none());
        assert!(get_canonical_status_from_thai("something else").is_none());
    }
}
