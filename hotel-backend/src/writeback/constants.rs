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

/// `HT_Book_H.Book_Status` literal for "booked" / "reserved" — the initial
/// state set on every fresh booking-create. The .NET app's future-date
/// room view (`View_Book_Date`) filters by this exact value:
/// `WHERE book_status='จอง'`. Set to empty string previously, which kept
/// our writeback bookings invisible in the future room view (the
/// 2026-04-26 R014838/R014839 incident — bookings 0008/0009 in our app).
pub const BOOK_STATUS_BOOKED: &str = "จอง";

/// `HT_Book_H.Book_Status` literal for "cancelled". Spike §3g-bis.
pub const BOOK_STATUS_CANCELLED: &str = "ยกเลิก";

/// `HT_Book_H.Book_Status` literal for "occupying" — set when a check-in
/// is created against an existing booking (spike §3d).
pub const BOOK_STATUS_OCCUPYING: &str = "เข้าพัก";

/// `HT_CheckIn_H.cin_status` literal for "cancelled". Spike §3i.
pub const CIN_STATUS_CANCELLED: &str = "ยกเลิก";

/// `HT_CheckIn_H.Cin_status` literal for "normal" (Thai: "ปกติ") — the
/// state set on every fresh check-in. Distinct from
/// [`CIN_ROOM_STATUS_OCCUPYING`] (`'เข้าพัก'`), which is the
/// `HT_CheckIn_Ds.Cin_Room_Status` value. Verified from
/// `/tmp/legacy-events-full.log` (every captured `HT_CheckIn_H` INSERT
/// emits `'ปกติ'` for the header status).
pub const CIN_STATUS_NORMAL: &str = "ปกติ";

/// `HT_CheckIn_Ds.Cin_dep_status` literal for "no deposit collected".
/// Verified from `/tmp/legacy-events-full.log` (every captured
/// `HT_CheckIn_Ds` INSERT emits `'ไม่เก็บค่ามัดจำ'`).
pub const CIN_DEP_STATUS_NONE: &str = "ไม่เก็บค่ามัดจำ";

/// `HT_CheckIn_Ds.Cin_dep_status` literal for "deposit collected, not yet
/// returned" — the initial value iHOTEL sets when a deposit is taken at
/// check-in (`docs/legacy-app/COMPAT_CHEATSHEET.md` §HT_CheckIn_Ds:
/// "`'ยังไม่คืนค่ามัดจำ'` (deposit not yet returned) — initial when deposit
/// collected", FrmCheckIn). The recipe writes this **only** when
/// `Cin_Room_Dep > 0`; otherwise [`CIN_DEP_STATUS_NONE`] is emitted (iHOTEL's
/// own `Cin_Room_Dep<=0 ⇒ 'ไม่เก็บค่ามัดจำ'` rule), preserving byte-parity for
/// the common no-deposit walk-in.
pub const CIN_DEP_STATUS_COLLECTED: &str = "ยังไม่คืนค่ามัดจำ";

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

/// `HT_Room_Status.room_status` value for a reserved (booked but not yet
/// occupying) night. Mirrors [`BOOK_STATUS_BOOKED`] but lives on the
/// per-room-per-day occupancy ledger. The legacy app's calendar grid
/// queries `where (room_status='จอง' or room_status='เข้าพัก')` to render
/// reserved + occupied nights — so without this row the booking is
/// invisible in the grid. Per `COMPAT_CHEATSHEET.md` §`HT_Room_Status` "status='จอง', room_Book_No=Book_ID"
/// (was: cheatsheet 347, the schema fence header of that same section).
pub const ROOM_STATUS_RESERVED: &str = "จอง";

/// `HT_Room_Status.room_status` value after check-out — **English with a
/// space**. Spike §3e + §4c.
///
/// ## Documented decision: `'Check Out'` (space) vs `'Check-Out'` (hyphen)
///
/// The decompile cheatsheet (`COMPAT_CHEATSHEET.md` §8.11) shows MOST
/// iHOTEL code paths read/write the hyphen form on this table and argues
/// a new app "should write 'Check-Out' consistently" — the space form is
/// a bug in `FrmCheckOut.cs:6246` whose rows escape frmMain1's
/// startup `DELETE ... WHERE room_status='Check-Out'` until the next
/// startup's `Room_Use='no'` fix-up converts them.
///
/// We deliberately follow the **capture** (`'Check Out'`, findings §3e —
/// `checkout-20260424-100323/writes.txt`), NOT the cheatsheet's
/// recommendation, because the capture is observed production behavior:
/// it is the exact byte sequence the live iHOTEL emits on every real
/// check-out, and 14 years of production data carry it. Matching it makes
/// our rows indistinguishable from iHOTEL's own — including inheriting
/// the same (benign, self-healing) startup-cleanup timing iHOTEL's rows
/// get. Writing the hyphen instead would make our checkout rows behave
/// DIFFERENTLY from iHOTEL's (deleted at next startup rather than after
/// the fix-up pass), a divergence nobody has observed or tested.
///
/// Re-affirmed by the 2026-06-11 coexistence audit. To switch to the
/// hyphen form, you would need: (a) a fresh capture showing iHOTEL itself
/// emitting `'Check-Out'` on this table (e.g. after a vendor patch), or
/// (b) a receptionist-verified live test on HF Ville demonstrating that
/// hyphen rows render/clean up at least as well as space rows across an
/// iHOTEL restart. Update the byte-parity tests in `checkout.rs` in the
/// same change.
pub const ROOM_STATUS_CHECKED_OUT: &str = "Check Out";

/// `HT_Customers.Cust_Type` default. Spike §3a (and elsewhere).
pub const CUST_TYPE_NORMAL: &str = "ราคาปกติ";

/// `HT_Customers.Cust_Type_Main` default for the .NET app's INSERT
/// path — the legacy customer-create captures all show `'ราคาปกติ'`
/// (verified from `/tmp/legacy-events-full.log`, all 12 captured
/// `INSERT INTO [HT_Customers]` rows). The earlier
/// [`CUST_TYPE_MAIN_INDIVIDUAL`] (`'บุคคลธรรมดา'`) is what the .NET
/// app's UPDATE path emits — kept for that flow only.
pub const CUST_TYPE_MAIN_NORMAL: &str = "ราคาปกติ";

/// `HT_Customers.Cust_Type_Main` value emitted by the .NET app's
/// `UPDATE [HT_Customers]` path (booking-modify / checkin-to-booking).
/// Verified from `/tmp/legacy-events-full.log` `UPDATE` captures —
/// distinct from [`CUST_TYPE_MAIN_NORMAL`] which the INSERT path uses.
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

/// `HT_CheckIn_Pay.Branch` default — the legacy app fills this with
/// the literal Thai phrase for "head office" on every payment row.
/// Verified from `/tmp/legacy-events-full.log` (all 24 captured
/// `INSERT INTO [HT_CheckIn_Pay]` rows).
pub const BRANCH_HEAD_OFFICE: &str = "สำนักงานใหญ่";

/// `HT_CheckIn_Pay.Cin_Pay_Ds_Name` literal for room-charge payments
/// (Thai: "room cost"). Verified from `/tmp/legacy-events-full.log`
/// — all 24 captured payment rows use this exact string, NOT the
/// receipt-line label `'ค่าห้องพัก [room]'` which is HT_Receipt_Ds only.
pub const PAY_DS_NAME_ROOM: &str = "ค่าห้อง";

/// `HT_CheckIn_Pay.Cin_Pay_Ds_unit` literal (Thai: "item"). Verified
/// from `/tmp/legacy-events-full.log` — captured as the string
/// `'รายการ'`, not the integer `1`.
pub const PAY_DS_UNIT_ITEM: &str = "รายการ";

/// `HT_CheckIn_Pay.Cin_Pay_Ds_ID` literal — the legacy product code
/// emitted into the payment row (NOT the receipt-line `SEV-001`
/// service code). Verified from `/tmp/legacy-events-full.log`.
pub const PAY_DS_ID_ROOM: &str = "P001";

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

/// `HT_Receipt_H.Receipt_VatPer` value emitted by the legacy app — the
/// hotel's standard 7% Thai VAT. Verified from
/// `/tmp/legacy-events-full.log` (captured Receipt_H rows render
/// `Receipt_BeforeVat = round(Total / 1.07, 2)`,
/// `Receipt_Vat = Total - BeforeVat`, `Receipt_VatIn='True'`,
/// `Receipt_VatPer=7`).
pub const RECEIPT_VAT_PERCENT: i32 = 7;

/// `HT_Receipt_H.Receipt_VatIn` literal — the legacy app emits the
/// string `'True'` (not a SQL bit), indicating the headline `Total`
/// figure is VAT-inclusive. Verified from `/tmp/legacy-events-full.log`.
pub const RECEIPT_VAT_INCLUSIVE: &str = "True";

/// `HT_Receipt_H.status_name` value for an active receipt (Thai:
/// "normal"). Verified from `/tmp/legacy-events-full.log` — every
/// captured Receipt_H row emits `'ปกติ'`, never empty.
pub const RECEIPT_STATUS_NORMAL: &str = "ปกติ";

/// Compose the `HT_Receipt_H.Receipt_note` blurb the legacy app
/// renders for a stay (e.g. `'เข้าพัก วันที่ 25/04/26 ถึง 26/04/26'`).
/// Two-digit day/month/year matching the captured format.
pub fn receipt_stay_note(check_in: chrono::NaiveDate, check_out: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    format!(
        "เข้าพัก วันที่ {:02}/{:02}/{:02} ถึง {:02}/{:02}/{:02}",
        check_in.day(),
        check_in.month(),
        check_in.year() % 100,
        check_out.day(),
        check_out.month(),
        check_out.year() % 100,
    )
}

/// `HT_Receipt_H.Receipt_noteUP` literal for receipts that originated
/// from a booking (vs a walk-in). Captured forms: `'Booking'` when
/// `Cin_Book_no` is set, empty string otherwise.
pub const RECEIPT_NOTE_UP_BOOKING: &str = "Booking";
