//! Legacy literal constants — copied verbatim from
//! `docs/legacy-spike/findings.md` §3 + §4c.
//!
//! **Do not edit these without re-validating against a fresh capture.** The
//! .NET app is case-insensitive in SQL but case-sensitive in some screens
//! (e.g. the booking-list filter checks `Book_Status='ยกเลิก'` exactly).
//!
//! Mixed Thai / English with inconsistent spacing is intentional: the legacy
//! app uses `'Check-Out'` with a hyphen on `HT_CheckIn_Ds.Cin_Room_Status`
//! but `'Check Out'` with a space on `HT_Room_Status.room_status` (spike §3e
//! captures verified). Mirror exactly.

// ───────────────────────────────────────────────────────────────────────────
// Status / state literals
// ───────────────────────────────────────────────────────────────────────────

/// `HT_Book_H.Book_Status` literal for "cancelled". Spike §3g-bis.
pub const BOOK_STATUS_CANCELLED: &str = "ยกเลิก";

/// `HT_Book_H.Book_Status` literal for "occupying" — set when a check-in
/// is created against an existing booking (spike §3d).
pub const BOOK_STATUS_OCCUPYING: &str = "เข้าพัก";

/// `HT_CheckIn_H.cin_status` literal for "cancelled". Spike §3i.
pub const CIN_STATUS_CANCELLED: &str = "ยกเลิก";

/// `HT_CheckIn_Ds.Cin_Room_Status` initial value for an active stay (Thai:
/// "occupying"). Spike §3a.
pub const CIN_ROOM_STATUS_OCCUPYING: &str = "เข้าพัก";

/// `HT_CheckIn_Ds.Cin_Room_Status` value after check-out — **English with a
/// hyphen**. Distinct from `HT_Room_Status.room_status` below (which uses a
/// space). Spike §3e + §4c.
pub const CIN_ROOM_STATUS_CHECKED_OUT: &str = "Check-Out";

/// `HT_Room_Status.room_status` value for an active stay (mirrors
/// [`CIN_ROOM_STATUS_OCCUPYING`]). Spike §3a.
pub const ROOM_STATUS_OCCUPYING: &str = "เข้าพัก";

/// `HT_Room_Status.room_status` value after check-out — **English with a
/// space**. Spike §3e + §4c.
pub const ROOM_STATUS_CHECKED_OUT: &str = "Check Out";

/// `HT_Customers.Cust_Type` default. Spike §3a (and elsewhere).
pub const CUST_TYPE_NORMAL: &str = "ราคาปกติ";

/// `HT_Customers.Cust_Type_Main` default. Spike §3a.
pub const CUST_TYPE_MAIN_INDIVIDUAL: &str = "บุคคลธรรมดา";

// ───────────────────────────────────────────────────────────────────────────
// Power-log note templates (HT_POWER_LOG.ROOM_POWER_NOTE / NOTE2)
// ───────────────────────────────────────────────────────────────────────────

/// Lights-on note template applied at check-in. Spike §3a.
/// Format: `เปิดไฟ อัตโนมัติ จากเช็คอิน No.{cin_no}`.
pub fn power_log_note_check_in(cin_no: &str) -> String {
    format!("เปิดไฟ อัตโนมัติ จากเช็คอิน No.{cin_no}")
}

/// Lights-off note template applied at check-out. Spike §3e Phase 2.
/// Format: `ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.{cin_no}`.
pub fn power_log_note_check_out(cin_no: &str) -> String {
    format!("ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.{cin_no}")
}

/// Lights-off note when a check-in is cancelled — distinct text from
/// check-out (verified in spike §3i capture). Has no cin_no suffix.
pub const POWER_LOG_NOTE_CHECKIN_CANCELLED: &str = "ปิดไฟ เนื่องจากยกเลิกห้องพัก";

// ───────────────────────────────────────────────────────────────────────────
// Numeric / shape constants for booking-list visibility (spike §3k)
// ───────────────────────────────────────────────────────────────────────────

/// `HT_Book_H.book_room_type` value that makes a booking visible in the
/// .NET app's main booking-list view. `2` = book by room number; `1` = book
/// by room type (anomaly). Per spike §3k, of 1178 real bookings 1176 use `2`.
pub const BOOK_ROOM_TYPE_BY_ROOM_NUMBER: i32 = 2;

/// `HT_Book_Ds.Book_status` numeric code for an active booking (the booking
/// list filters to `=1`). Spike §3k.
pub const BOOK_DS_STATUS_ACTIVE: i32 = 1;

/// `HT_Book_Ds.Book_status` numeric code for a cancelled booking. Spike §3g-bis.
pub const BOOK_DS_STATUS_CANCELLED: i32 = 3;

/// `HT_Book_H.Book_Notify_Day` default — payment-reminder lead time in days.
/// `3` matches the .NET app default per spike §3k.
pub const BOOK_NOTIFY_DAY_DEFAULT: i32 = 3;

/// Operator name attributed to writeback-issued statements. `'Admin'` is what
/// the legacy app's logged-in employee always shows in our captures.
pub const DEFAULT_OPERATOR: &str = "Admin";

// ───────────────────────────────────────────────────────────────────────────
// Receipt
// ───────────────────────────────────────────────────────────────────────────

/// `HT_Receipt_Ds.S_Product_no` for room charges. Spike §3h.
pub const RECEIPT_SERVICE_CODE_ROOM: &str = "SEV-001";

/// `HT_Receipt_Ds.S_UnitName` for room nights (Thai: "night"). Spike §3h.
pub const RECEIPT_UNIT_NIGHT: &str = "คืน";

/// Format a receipt line label for room charges: `ค่าห้องพัก [{room_no}]`.
/// Spike §3h capture: `'ค่าห้องพัก [414]'`.
pub fn receipt_room_label(room_no: &str) -> String {
    format!("ค่าห้องพัก [{room_no}]")
}
