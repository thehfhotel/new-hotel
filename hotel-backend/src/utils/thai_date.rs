//! Thai date formatting utilities

use chrono::{Datelike, NaiveDateTime};

/// Thai month names
pub const THAI_MONTHS: [&str; 12] = [
    "มกราคม",
    "กุมภาพันธ์",
    "มีนาคม",
    "เมษายน",
    "พฤษภาคม",
    "มิถุนายน",
    "กรกฎาคม",
    "สิงหาคม",
    "กันยายน",
    "ตุลาคม",
    "พฤศจิกายน",
    "ธันวาคม",
];

/// Format a NaiveDateTime as a Thai date string with Buddhist year
///
/// Example: "5 กุมภาพันธ์ 2569"
pub fn format_thai_date(dt: NaiveDateTime) -> String {
    let day = dt.day();
    let month = THAI_MONTHS[dt.month0() as usize];
    let year = dt.year() + 543; // Buddhist Era

    format!("{} {} {}", day, month, year)
}

/// Format a NaiveDateTime as a short Thai date string
///
/// Example: "5 ก.พ. 2569"
pub fn format_thai_date_short(dt: NaiveDateTime) -> String {
    const SHORT_MONTHS: [&str; 12] = [
        "ม.ค.", "ก.พ.", "มี.ค.", "เม.ย.", "พ.ค.", "มิ.ย.",
        "ก.ค.", "ส.ค.", "ก.ย.", "ต.ค.", "พ.ย.", "ธ.ค.",
    ];

    let day = dt.day();
    let month = SHORT_MONTHS[dt.month0() as usize];
    let year = dt.year() + 543;

    format!("{} {} {}", day, month, year)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_format_thai_date() {
        let dt = NaiveDate::from_ymd_opt(2024, 2, 5)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        assert_eq!(format_thai_date(dt), "5 กุมภาพันธ์ 2567");
    }

    #[test]
    fn test_format_thai_date_short() {
        let dt = NaiveDate::from_ymd_opt(2024, 2, 5)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        assert_eq!(format_thai_date_short(dt), "5 ก.พ. 2567");
    }
}
