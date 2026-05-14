//! Schema fingerprint guard.
//!
//! Per `docs/architecture.md` §4e — the writeback worker queries MSSQL on
//! startup for the column shapes of the 10 tables it writes to, hashes the
//! `(table, ord, column, type)` tuples, and refuses to start if the hash
//! differs from a hardcoded baseline.
//!
//! ## Why
//!
//! The vendor app could push a schema change at any time. Adding a new column
//! is harmless; renaming or retyping one would silently corrupt our writebacks.
//! The fingerprint check is the safety net that stops us from writing into a
//! schema we don't understand.
//!
//! ## Source of truth
//!
//! [`EXPECTED_SCHEMA_BASELINE`] mirrors `docs/legacy-spike/schema/01-baseline-schema.txt`
//! lines 119–529 (the `==== Columns ====` block, filtered to the 10 tables we
//! touch). Any vendor change forces a re-capture and an update to this constant
//! — see `scripts/writeback-fingerprint.sh` for the regeneration recipe.

use sha2::{Digest, Sha256};
use tiberius::{Query, Row};

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use crate::db::DbPool;
use crate::writeback::error::{WritebackError, WritebackResult};

/// Tables the writeback worker touches. The fingerprint check covers exactly
/// these — adding a recipe that writes to a new table requires extending this
/// list AND regenerating [`EXPECTED_FINGERPRINT`].
///
/// Wave 5a item 5 added the trailing five: `HT_Cupon` (mark-printed UPDATE
/// from walkin / checkin_to_booking), `HT_CheckIn_Pay` (payment INSERT),
/// `HT_Receipt_Ds` (receipt line INSERT), `HT_POWER_LOG` (lights on/off
/// audit from walkin / checkin_to_booking / checkout / checkin_cancel),
/// `HT_Changed_Room` (room-move audit — not yet touched by a recipe but
/// the audit doc flagged it as in-scope for the next wave; fingerprinting
/// up-front avoids a separate baseline bump later).
///
/// Track D / T7 HIGH-3 (2026-05-13) renamed this list to
/// `WRITEBACK_FINGERPRINTED_TABLES` (was [`FINGERPRINTED_TABLES`]) to
/// disambiguate it from the new CT-side fingerprint
/// ([`CT_EXTRA_FINGERPRINTED_TABLES`]). The legacy alias is preserved
/// below for any out-of-tree consumer.
pub const WRITEBACK_FINGERPRINTED_TABLES: &[&str] = &[
    "HT_Customers",
    "HT_Book_H",
    "HT_Book_Ds",
    "HT_Book_Date",
    "HT_CheckIn_H",
    "HT_CheckIn_Ds",
    "HT_Receipt_H",
    "HT_Rooms",
    "HT_Room_Status",
    "HT_Rooms_Cancel",
    "HT_Cupon",
    "HT_CheckIn_Pay",
    "HT_Receipt_Ds",
    "HT_POWER_LOG",
    "HT_Changed_Room",
];

/// Back-compat alias for the writeback fingerprint list. Deprecated in
/// favour of [`WRITEBACK_FINGERPRINTED_TABLES`] — kept exported because
/// the constant name appears in spike + audit docs and some test fixtures.
#[allow(dead_code)]
pub const FINGERPRINTED_TABLES: &[&str] = WRITEBACK_FINGERPRINTED_TABLES;

/// Track D / T7 HIGH-3 — additional CT-watched tables the CT-side
/// fingerprint must cover but the writeback worker never touches.
/// Four mirror tables (`HT_CheckIn_Product`, `HT_Deposit`,
/// `HT_Bill_Debt_H`, `HT_Bill_Debt_Ds`) PLUS one user-impact table
/// (`HT_CheckIn_Other_People` — TM.30 immigration registry).
///
/// Track E1 (audit 2026-05-13 T2 HIGH-3) wired `HT_CheckIn_Other_People`
/// to a real `GuestRegistryMapper` — the table was already fingerprinted
/// here by Track D so no baseline bump was needed when the mapper
/// landed.
///
/// The full CT-side fingerprint set is the union of
/// [`WRITEBACK_FINGERPRINTED_TABLES`] + [`CT_EXTRA_FINGERPRINTED_TABLES`];
/// see [`ct_fingerprinted_tables`] for the convenience accessor.
pub const CT_EXTRA_FINGERPRINTED_TABLES: &[&str] = &[
    "HT_CheckIn_Product",
    "HT_Deposit",
    "HT_Bill_Debt_H",
    "HT_Bill_Debt_Ds",
    "HT_CheckIn_Other_People",
];

/// Union of the writeback + CT-extra fingerprint sets. Returned as
/// `Vec<&'static str>` so callers can ship it through SQL builders
/// without re-derivation.
pub fn ct_fingerprinted_tables() -> Vec<&'static str> {
    let mut out = Vec::with_capacity(
        WRITEBACK_FINGERPRINTED_TABLES.len() + CT_EXTRA_FINGERPRINTED_TABLES.len(),
    );
    out.extend_from_slice(WRITEBACK_FINGERPRINTED_TABLES);
    out.extend_from_slice(CT_EXTRA_FINGERPRINTED_TABLES);
    out
}

/// Captured column tuples from `docs/legacy-spike/schema/01-baseline-schema.txt`
/// (lines 119–529). Format per row: `(table, ordinal, column, data_type)`.
///
/// Used by the [`tests::fingerprint_constant_matches_baseline`] test as the
/// canonical source of [`EXPECTED_FINGERPRINT`]. Marked `#[allow(dead_code)]`
/// because the runtime check ([`verify_schema_fingerprint`]) hashes the LIVE
/// schema and compares to [`EXPECTED_FINGERPRINT`] directly — the baseline
/// constant only exists so the test can re-derive the expected value when the
/// schema legitimately changes.
#[allow(dead_code)]
///
/// The fingerprint is `sha256(rows.join("\n"))`. We do NOT hash the
/// max_length / precision / scale / is_nullable columns because:
/// - vendor patches sometimes widen `varchar(50)` → `varchar(100)` for free
/// - is_nullable and is_identity have already been stripped from some columns
///   (e.g. `HT_Customers.id` per spike §2)
/// - These quirks would force a new fingerprint with every cosmetic patch
///
/// Including only `(table, ord, column, type)` is enough to detect:
/// - Renamed columns (`Cust_name` → `Customer_Name`)
/// - Retyped columns (`varchar` → `nvarchar`)
/// - Reordered columns
/// - Removed columns
const EXPECTED_SCHEMA_BASELINE: &[(&str, i32, &str, &str)] = &[
    // Live MSSQL ORDER BY TABLE_NAME, ORDINAL_POSITION sorts table names
    // case-sensitive on SQL Server's default collation. Verified order
    // against the schema dump in `docs/legacy-spike/schema/01-baseline-schema.txt`:
    //   HT_Book_*, HT_Changed_Room, HT_CheckIn_*, HT_Cupon, HT_Customers,
    //   HT_POWER_LOG, HT_Receipt_*, HT_Room_Status, HT_Rooms, HT_Rooms_Cancel.

    // HT_Book_Date — spike baseline lines 119-126
    ("HT_Book_Date", 1, "id", "int"),
    ("HT_Book_Date", 2, "Book_no", "varchar"),
    ("HT_Book_Date", 3, "Book_type", "varchar"),
    ("HT_Book_Date", 4, "Book_date_ds", "datetime"),
    ("HT_Book_Date", 5, "Book_Num", "int"),
    ("HT_Book_Date", 6, "Book_USE", "int"),
    ("HT_Book_Date", 7, "Book_ok", "int"),
    ("HT_Book_Date", 8, "Cin_no", "varchar"),
    // HT_Book_Ds — lines 127-137
    ("HT_Book_Ds", 1, "id", "int"),
    ("HT_Book_Ds", 2, "Book_No", "varchar"),
    ("HT_Book_Ds", 3, "Book_Room_Type", "varchar"),
    ("HT_Book_Ds", 4, "Book_Room_Start", "datetime"),
    ("HT_Book_Ds", 5, "Book_Room_End", "datetime"),
    ("HT_Book_Ds", 6, "Book_Room_Price", "float"),
    ("HT_Book_Ds", 7, "Book_Room_Night", "float"),
    ("HT_Book_Ds", 8, "Book_Room_Num", "float"),
    ("HT_Book_Ds", 9, "Book_Room_PriceToTal", "float"),
    ("HT_Book_Ds", 10, "Book_Room_Note", "varchar"),
    ("HT_Book_Ds", 11, "Book_status", "int"),
    // HT_Book_H — lines 146-163 (key columns through Book_Sale)
    ("HT_Book_H", 1, "Book_ID", "varchar"),
    ("HT_Book_H", 2, "Book_Date", "datetime"),
    ("HT_Book_H", 3, "Book_Date_in", "datetime"),
    ("HT_Book_H", 4, "Book_Date_out", "datetime"),
    ("HT_Book_H", 5, "Book_Cust_ID", "varchar"),
    ("HT_Book_H", 6, "Book_Cust_Name", "varchar"),
    ("HT_Book_H", 7, "Book_Cust_Name2", "varchar"),
    ("HT_Book_H", 8, "Book_Cust_Tel", "varchar"),
    ("HT_Book_H", 9, "Book_Price_Total", "float"),
    ("HT_Book_H", 10, "Book_Price_Pay", "float"),
    ("HT_Book_H", 11, "Book_Status", "varchar"),
    ("HT_Book_H", 12, "Book_by", "varchar"),
    ("HT_Book_H", 13, "Book_room_all", "text"),
    ("HT_Book_H", 14, "Book_room_note", "text"),
    ("HT_Book_H", 15, "Book_room_type", "int"),
    ("HT_Book_H", 16, "Book_Notify_Day", "int"),
    ("HT_Book_H", 17, "Book_Notify_Note", "varchar"),
    ("HT_Book_H", 18, "Book_Sale", "varchar"),
    // HT_Changed_Room — Wave 5a item 5. Spike baseline lines 200-207.
    // 8 columns. Not yet touched by a recipe (room-move audit) but
    // fingerprinted so a future recipe doesn't drift unobserved.
    ("HT_Changed_Room", 1, "id", "int"),
    ("HT_Changed_Room", 2, "cin_no", "varchar"),
    ("HT_Changed_Room", 3, "room_before", "varchar"),
    ("HT_Changed_Room", 4, "room_after", "varchar"),
    ("HT_Changed_Room", 5, "change_date", "datetime"),
    ("HT_Changed_Room", 6, "room_before_price", "float"),
    ("HT_Changed_Room", 7, "Note", "varchar"),
    ("HT_Changed_Room", 8, "ToPrice", "varchar"),
    // HT_CheckIn_Ds — lines 208-226
    ("HT_CheckIn_Ds", 1, "id", "int"),
    ("HT_CheckIn_Ds", 2, "Cin_No", "varchar"),
    ("HT_CheckIn_Ds", 3, "Cin_Room_No", "varchar"),
    ("HT_CheckIn_Ds", 4, "Cin_Room_Type", "varchar"),
    ("HT_CheckIn_Ds", 5, "Cin_Room_In", "datetime"),
    ("HT_CheckIn_Ds", 6, "Cin_Room_Out", "datetime"),
    ("HT_CheckIn_Ds", 7, "Cin_Room_Status", "varchar"),
    ("HT_CheckIn_Ds", 8, "Cin_Room_Dep", "float"),
    ("HT_CheckIn_Ds", 9, "Cin_Room_Price", "float"),
    ("HT_CheckIn_Ds", 10, "Cin_Room_Night", "float"),
    ("HT_CheckIn_Ds", 11, "Cin_Room_PriceToTal", "float"),
    ("HT_CheckIn_Ds", 12, "Cin_Room_Pay_Before", "float"),
    ("HT_CheckIn_Ds", 13, "Cin_Room_Pay_Total", "float"),
    ("HT_CheckIn_Ds", 14, "Cin_note", "varchar"),
    ("HT_CheckIn_Ds", 15, "Cin_Dep_Status", "varchar"),
    ("HT_CheckIn_Ds", 16, "Dep_by", "varchar"),
    ("HT_CheckIn_Ds", 17, "Cin_cupon", "int"),
    ("HT_CheckIn_Ds", 18, "Cin_Dep_return_date", "datetime"),
    ("HT_CheckIn_Ds", 19, "Cin_Dep_return_by", "varchar"),
    // HT_CheckIn_H — lines 227-248
    ("HT_CheckIn_H", 1, "Cin_no", "varchar"),
    ("HT_CheckIn_H", 2, "Cin_Date", "datetime"),
    ("HT_CheckIn_H", 3, "Cin_Book_no", "varchar"),
    ("HT_CheckIn_H", 4, "Cin_cust_no", "varchar"),
    ("HT_CheckIn_H", 5, "Cin_cust_price", "varchar"),
    ("HT_CheckIn_H", 6, "Cin_Car_type", "varchar"),
    ("HT_CheckIn_H", 7, "Cin_Car_id", "varchar"),
    ("HT_CheckIn_H", 8, "Cin_status", "varchar"),
    ("HT_CheckIn_H", 9, "Total_Price_Room", "float"),
    ("HT_CheckIn_H", 10, "Total_Price_Product", "float"),
    ("HT_CheckIn_H", 11, "Total_Price_Net", "float"),
    ("HT_CheckIn_H", 12, "Total_Price_Pay", "float"),
    ("HT_CheckIn_H", 13, "Total_Price_Balance", "float"),
    ("HT_CheckIn_H", 14, "Cin_Room_ALL", "varchar"),
    ("HT_CheckIn_H", 15, "Total_Price_vat", "float"),
    ("HT_CheckIn_H", 16, "Cin_by", "varchar"),
    ("HT_CheckIn_H", 17, "Cin_Date_in", "datetime"),
    ("HT_CheckIn_H", 18, "Cin_Date_out", "datetime"),
    ("HT_CheckIn_H", 19, "Cin_type", "int"),
    ("HT_CheckIn_H", 20, "Cin_note", "varchar"),
    ("HT_CheckIn_H", 21, "Cin_foreign", "varchar"),
    ("HT_CheckIn_H", 22, "Cin_Work_number", "int"),
    // HT_CheckIn_Pay — Wave 5a item 5. Spike baseline lines 253-274.
    // 22 columns. Payment recipe `INSERT INTO [HT_CheckIn_Pay] (...)`.
    ("HT_CheckIn_Pay", 1, "id", "int"),
    ("HT_CheckIn_Pay", 2, "Pay_no", "varchar"),
    ("HT_CheckIn_Pay", 3, "Cin_No", "varchar"),
    ("HT_CheckIn_Pay", 4, "Cin_Pay_Ds", "varchar"),
    ("HT_CheckIn_Pay", 5, "Cin_Pay_Cash", "float"),
    ("HT_CheckIn_Pay", 6, "Cin_Pay_Credit", "float"),
    ("HT_CheckIn_Pay", 7, "Cin_Pay_Date", "datetime"),
    ("HT_CheckIn_Pay", 8, "Cin_Pay_Ds_Name", "varchar"),
    ("HT_CheckIn_Pay", 9, "Cin_Pay_Ds_ID", "varchar"),
    ("HT_CheckIn_Pay", 10, "Cin_Pay_Ds_Price", "float"),
    ("HT_CheckIn_Pay", 11, "Cin_Pay_Ds_unit", "varchar"),
    ("HT_CheckIn_Pay", 12, "Cin_Pay_Ds_Num", "float"),
    ("HT_CheckIn_Pay", 13, "Cin_Pay_Ds_PriceOne", "float"),
    ("HT_CheckIn_Pay", 14, "Cin_Pay_Ds_PriceTotal", "float"),
    ("HT_CheckIn_Pay", 15, "Cin_Cust_no", "varchar"),
    ("HT_CheckIn_Pay", 16, "Cin_Status", "varchar"),
    ("HT_CheckIn_Pay", 17, "Cin_Pay_Note", "varchar"),
    ("HT_CheckIn_Pay", 18, "Pay_by", "varchar"),
    ("HT_CheckIn_Pay", 19, "Cin_Pay_Free", "float"),
    ("HT_CheckIn_Pay", 20, "Cin_Pay_Tran", "float"),
    ("HT_CheckIn_Pay", 21, "Branch", "varchar"),
    ("HT_CheckIn_Pay", 22, "Cin_Pay_web", "float"),
    // HT_Cupon — Wave 5a item 5. Spike baseline lines 292-298.
    // 7 columns. Walkin / checkin_to_booking emit the mark-printed
    // UPDATE via helpers::mark_cupon_printed.
    ("HT_Cupon", 1, "cupon_no", "int"),
    ("HT_Cupon", 2, "cupon_cin_no", "varchar"),
    ("HT_Cupon", 3, "cupon_cin_room", "varchar"),
    ("HT_Cupon", 4, "cupon_date", "datetime"),
    ("HT_Cupon", 5, "cupon_gen_date", "datetime"),
    ("HT_Cupon", 6, "cupon_by", "varchar"),
    ("HT_Cupon", 7, "cupon_print", "int"),
    // HT_Customers — lines 299-335 (35 columns)
    ("HT_Customers", 1, "id", "int"),
    ("HT_Customers", 2, "Cust_no", "varchar"),
    ("HT_Customers", 3, "Cust_perfix", "varchar"),
    ("HT_Customers", 4, "Cust_name", "varchar"),
    ("HT_Customers", 5, "Cust_name2", "varchar"),
    ("HT_Customers", 6, "Cust_sex", "varchar"),
    ("HT_Customers", 7, "Cust_IDcard", "varchar"),
    ("HT_Customers", 8, "Cust_Type", "varchar"),
    ("HT_Customers", 9, "Cust_Email", "varchar"),
    ("HT_Customers", 10, "Cust_Add_no", "varchar"),
    ("HT_Customers", 11, "Cust_Add_moo", "varchar"),
    ("HT_Customers", 12, "Cust_Add_soi", "varchar"),
    ("HT_Customers", 13, "Cust_Add_road", "varchar"),
    ("HT_Customers", 14, "Cust_Add_tambon", "varchar"),
    ("HT_Customers", 15, "Cust_Add_ampore", "varchar"),
    ("HT_Customers", 16, "Cust_Add_province", "varchar"),
    ("HT_Customers", 17, "Cust_Add_code", "varchar"),
    ("HT_Customers", 18, "Cust_Add_tel", "varchar"),
    ("HT_Customers", 19, "Cust_Add_fax", "varchar"),
    ("HT_Customers", 20, "Cust_Work_Name", "varchar"),
    ("HT_Customers", 21, "Cust_Work_no", "varchar"),
    ("HT_Customers", 22, "Cust_Work_moo", "varchar"),
    ("HT_Customers", 23, "Cust_Work_soi", "varchar"),
    ("HT_Customers", 24, "Cust_Work_road", "varchar"),
    ("HT_Customers", 25, "Cust_Work_tambon", "varchar"),
    ("HT_Customers", 26, "Cust_Work_ampore", "varchar"),
    ("HT_Customers", 27, "Cust_Work_province", "varchar"),
    ("HT_Customers", 28, "Cust_Work_code", "varchar"),
    ("HT_Customers", 29, "Cust_Work_tel", "varchar"),
    ("HT_Customers", 30, "Cust_Work_fax", "varchar"),
    ("HT_Customers", 31, "Cust_Last_Change", "datetime"),
    ("HT_Customers", 32, "Cust_Type_Main", "varchar"),
    ("HT_Customers", 33, "Cust_Price_Over", "float"),
    ("HT_Customers", 34, "Cust_Contry", "varchar"),
    ("HT_Customers", 35, "Cust_Work_Tax", "varchar"),
    // HT_POWER_LOG — Wave 5a item 5. Spike baseline lines 422-429.
    // 8 columns. Walkin / checkin_to_booking INSERT (lights on);
    // checkin_cancel / checkout UPDATE (lights off).
    ("HT_POWER_LOG", 1, "id", "int"),
    ("HT_POWER_LOG", 2, "ROOM_NO", "varchar"),
    ("HT_POWER_LOG", 3, "ROOM_POWER_START", "datetime"),
    ("HT_POWER_LOG", 4, "ROOM_POWER_END", "datetime"),
    ("HT_POWER_LOG", 5, "ROOM_POWER_START_BY", "varchar"),
    ("HT_POWER_LOG", 6, "ROOM_POWER_END_BY", "varchar"),
    ("HT_POWER_LOG", 7, "ROOM_POWER_NOTE", "varchar"),
    ("HT_POWER_LOG", 8, "ROOM_POWER_NOTE2", "varchar"),
    // HT_Receipt_Ds — Wave 5a item 5. Spike baseline lines 445-454.
    // 10 columns. Payment recipe INSERT (receipt line for room charge).
    ("HT_Receipt_Ds", 1, "id", "int"),
    ("HT_Receipt_Ds", 2, "S_Sale_id", "int"),
    ("HT_Receipt_Ds", 3, "S_Product_no", "varchar"),
    ("HT_Receipt_Ds", 4, "S_Product_name", "varchar"),
    ("HT_Receipt_Ds", 5, "S_Unit", "float"),
    ("HT_Receipt_Ds", 6, "S_UnitName", "varchar"),
    ("HT_Receipt_Ds", 7, "S_Price", "float"),
    ("HT_Receipt_Ds", 8, "S_Total", "float"),
    ("HT_Receipt_Ds", 9, "S_PriceDiscount", "float"),
    ("HT_Receipt_Ds", 10, "S_PriceDiscount_per", "varchar"),
    // HT_Receipt_H — lines 455-474 (20 columns)
    ("HT_Receipt_H", 1, "id", "int"),
    ("HT_Receipt_H", 2, "Receipt_no", "varchar"),
    ("HT_Receipt_H", 3, "Receipt_Date", "datetime"),
    ("HT_Receipt_H", 4, "Receipt_c_no", "varchar"),
    ("HT_Receipt_H", 5, "Receipt_Name", "varchar"),
    ("HT_Receipt_H", 6, "Receipt_Address", "varchar"),
    ("HT_Receipt_H", 7, "Receipt_Tel", "varchar"),
    ("HT_Receipt_H", 8, "Receipt_Fax", "varchar"),
    ("HT_Receipt_H", 9, "Receipt_Discount", "float"),
    ("HT_Receipt_H", 10, "Receipt_Total", "float"),
    ("HT_Receipt_H", 11, "Receipt_Vat", "float"),
    ("HT_Receipt_H", 12, "Receipt_BeforeVat", "float"),
    ("HT_Receipt_H", 13, "Receipt_VatIn", "varchar"),
    ("HT_Receipt_H", 14, "Receipt_VatPer", "float"),
    ("HT_Receipt_H", 15, "status_name", "varchar"),
    ("HT_Receipt_H", 16, "Receipt_Ref", "varchar"),
    ("HT_Receipt_H", 17, "Receipt_cin_vat_before", "float"),
    ("HT_Receipt_H", 18, "Receipt_note", "text"),
    ("HT_Receipt_H", 19, "Receipt_Tax", "varchar"),
    ("HT_Receipt_H", 20, "Receipt_noteUP", "text"),
    // HT_Room_Status — lines 493-500
    ("HT_Room_Status", 1, "id", "int"),
    ("HT_Room_Status", 2, "room_no", "varchar"),
    ("HT_Room_Status", 3, "room_date", "datetime"),
    ("HT_Room_Status", 4, "room_status", "varchar"),
    ("HT_Room_Status", 5, "room_Details", "varchar"),
    ("HT_Room_Status", 6, "room_Book_No", "varchar"),
    ("HT_Room_Status", 7, "room_CheckIn_No", "varchar"),
    ("HT_Room_Status", 8, "room_date_oa", "float"),
    // HT_Rooms — lines 501-523 (23 columns)
    ("HT_Rooms", 1, "id", "int"),
    ("HT_Rooms", 2, "Room_no", "varchar"),
    ("HT_Rooms", 3, "Room_Type", "varchar"),
    ("HT_Rooms", 4, "Room_Details", "varchar"),
    ("HT_Rooms", 5, "Room_PriceA", "float"),
    ("HT_Rooms", 6, "Room_PriceB", "float"),
    ("HT_Rooms", 7, "Room_PriceC", "float"),
    ("HT_Rooms", 8, "Room_Clean", "varchar"),
    ("HT_Rooms", 9, "Room_Use", "varchar"),
    ("HT_Rooms", 10, "Room_Manternace", "varchar"),
    ("HT_Rooms", 11, "Room_X", "int"),
    ("HT_Rooms", 12, "Room_Y", "int"),
    ("HT_Rooms", 13, "Room_Use_Count", "int"),
    ("HT_Rooms", 14, "Room_Polity", "int"),
    ("HT_Rooms", 15, "Room_Book", "varchar"),
    ("HT_Rooms", 16, "Room_Book_Name", "varchar"),
    ("HT_Rooms", 17, "Room_Book_Time", "varchar"),
    ("HT_Rooms", 18, "Room_Group", "varchar"),
    ("HT_Rooms", 19, "Room_Book_ds", "text"),
    ("HT_Rooms", 20, "Room_Power_OPEN", "varchar"),
    ("HT_Rooms", 21, "Room_Power_CLOSE", "varchar"),
    ("HT_Rooms", 22, "Room_Power_STATUS", "varchar"),
    ("HT_Rooms", 23, "Room_Clean_Time", "varchar"),
    // HT_Rooms_Cancel — lines 524-529
    ("HT_Rooms_Cancel", 1, "id", "int"),
    ("HT_Rooms_Cancel", 2, "room_no", "varchar"),
    ("HT_Rooms_Cancel", 3, "cin_no", "varchar"),
    ("HT_Rooms_Cancel", 4, "cancel_date", "datetime"),
    ("HT_Rooms_Cancel", 5, "cancel_by", "varchar"),
    ("HT_Rooms_Cancel", 6, "cancel_note", "text"),
];

/// SHA-256 hex of [`EXPECTED_SCHEMA_BASELINE`] joined as
/// `"{table}|{ord}|{column}|{type}\n"` per row, no trailing newline.
///
/// Computed at build-time by the test [`fingerprint_constant_matches_baseline`]
/// — that test is the canonical source. This constant is the snapshot value
/// committed alongside `01-baseline-schema.txt` so the worker can refuse to
/// start without re-running the test.
pub const EXPECTED_FINGERPRINT: &str =
    "8e076342babe5394b149c6e5aea5801348329e4a6a227118b31714e5e5d504b0";

/// Track D / T7 HIGH-3 — schema baseline for the 5 additional
/// CT-watched tables not covered by [`EXPECTED_SCHEMA_BASELINE`].
/// Format identical: `(table, ordinal, column, data_type)` — derived
/// from `docs/legacy-spike/schema/01-baseline-schema.txt` lines for
/// each table (Bill_Debt_Ds: 92-100, Bill_Debt_H: 101-118,
/// CheckIn_Other_People: 249-252, CheckIn_Product: 275-286,
/// Deposit: 334-341).
///
/// Hashed independently of the writeback baseline so the writeback
/// startup check stays orthogonal to the CT startup check.
#[allow(dead_code)]
const CT_EXTRA_SCHEMA_BASELINE: &[(&str, i32, &str, &str)] = &[
    // HT_Bill_Debt_Ds — baseline lines 92-100, 9 columns
    ("HT_Bill_Debt_Ds", 1, "id", "int"),
    ("HT_Bill_Debt_Ds", 2, "Bill_No", "varchar"),
    ("HT_Bill_Debt_Ds", 3, "DS_ID", "int"),
    ("HT_Bill_Debt_Ds", 4, "DS_NO", "varchar"),
    ("HT_Bill_Debt_Ds", 5, "DS_NAME", "varchar"),
    ("HT_Bill_Debt_Ds", 6, "DS_UNIT", "varchar"),
    ("HT_Bill_Debt_Ds", 7, "DS_NUM", "float"),
    ("HT_Bill_Debt_Ds", 8, "DS_PRICE", "float"),
    ("HT_Bill_Debt_Ds", 9, "DS_PRICE_TOTAL", "float"),
    // HT_Bill_Debt_H — baseline lines 101-118, 18 columns
    ("HT_Bill_Debt_H", 1, "Bill_No", "varchar"),
    ("HT_Bill_Debt_H", 2, "Bill_Cust_ID", "varchar"),
    ("HT_Bill_Debt_H", 3, "Bill_Cust_Name", "varchar"),
    ("HT_Bill_Debt_H", 4, "Bill_Cust_Address", "varchar"),
    ("HT_Bill_Debt_H", 5, "Bill_Cust_Tel", "varchar"),
    ("HT_Bill_Debt_H", 6, "Bill_Cust_Fax", "varchar"),
    ("HT_Bill_Debt_H", 7, "Bill_Date", "datetime"),
    ("HT_Bill_Debt_H", 8, "Bill_Ref", "varchar"),
    ("HT_Bill_Debt_H", 9, "Bill_Price_Type", "varchar"),
    ("HT_Bill_Debt_H", 10, "Bill_Type", "varchar"),
    ("HT_Bill_Debt_H", 11, "Bill_Total", "float"),
    ("HT_Bill_Debt_H", 12, "Bill_Pay", "float"),
    ("HT_Bill_Debt_H", 13, "Bill_Debt", "float"),
    ("HT_Bill_Debt_H", 14, "Bill_Pay_CASH", "float"),
    ("HT_Bill_Debt_H", 15, "Bill_Pay_CREDIT", "float"),
    ("HT_Bill_Debt_H", 16, "Bill_Status", "varchar"),
    ("HT_Bill_Debt_H", 17, "Bill_by", "varchar"),
    ("HT_Bill_Debt_H", 18, "Bill_Note", "varchar"),
    // HT_CheckIn_Other_People — baseline lines 249-252, 4 columns
    // (TM.30 immigration registry — Track E HIGH-3 will add a mapper)
    ("HT_CheckIn_Other_People", 1, "id", "int"),
    ("HT_CheckIn_Other_People", 2, "Cin_no", "varchar"),
    ("HT_CheckIn_Other_People", 3, "Cin_name", "varchar"),
    ("HT_CheckIn_Other_People", 4, "Cin_contry", "varchar"),
    // HT_CheckIn_Product — baseline lines 275-286, 12 columns
    ("HT_CheckIn_Product", 1, "id", "int"),
    ("HT_CheckIn_Product", 2, "Cin_No", "varchar"),
    ("HT_CheckIn_Product", 3, "Cin_Room_no", "varchar"),
    ("HT_CheckIn_Product", 4, "Cin_Ds_date", "datetime"),
    ("HT_CheckIn_Product", 5, "Cin_Pro_id", "varchar"),
    ("HT_CheckIn_Product", 6, "Cin_Pro_name", "varchar"),
    ("HT_CheckIn_Product", 7, "Cin_Pro_Unit", "varchar"),
    ("HT_CheckIn_Product", 8, "Cin_Pro_num", "float"),
    ("HT_CheckIn_Product", 9, "Cin_Pro_price", "float"),
    ("HT_CheckIn_Product", 10, "Cin_Pro_priceTotal", "float"),
    ("HT_CheckIn_Product", 11, "Cin_Pro_pay", "float"),
    ("HT_CheckIn_Product", 12, "Cin_Pro_note", "varchar"),
    // HT_Deposit — baseline lines 334-341, 8 columns
    ("HT_Deposit", 1, "id", "int"),
    ("HT_Deposit", 2, "Dep_no", "varchar"),
    ("HT_Deposit", 3, "Dep_Date", "datetime"),
    ("HT_Deposit", 4, "Dep_Room", "varchar"),
    ("HT_Deposit", 5, "Dep_Name", "varchar"),
    ("HT_Deposit", 6, "Dep_Price", "float"),
    ("HT_Deposit", 7, "Dep_Status", "varchar"),
    ("HT_Deposit", 8, "Dep_ref", "varchar"),
];

/// Track D / T7 HIGH-3 — SHA-256 fingerprint of
/// [`CT_EXTRA_SCHEMA_BASELINE`] (same encoding as
/// [`EXPECTED_FINGERPRINT`]). Verified by the
/// [`tests::ct_extra_fingerprint_constant_matches_baseline`] test.
pub const CT_EXTRA_EXPECTED_FINGERPRINT: &str =
    "6def7fd6b56a24cc9d29fcce32086ead827a1a8f1c48306a3972cb84b8549f56";

/// Compute the fingerprint of a column-tuple slice.
///
/// Public so `scripts/writeback-fingerprint.sh` can re-derive the expected
/// constant by including this module in a one-shot script.
pub fn compute_fingerprint(rows: &[(&str, i32, &str, &str)]) -> String {
    let mut hasher = Sha256::new();
    for (i, (table, ord, column, ty)) in rows.iter().enumerate() {
        if i > 0 {
            hasher.update(b"\n");
        }
        hasher.update(format!("{table}|{ord}|{column}|{ty}").as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Query MSSQL for the live column tuples of
/// [`WRITEBACK_FINGERPRINTED_TABLES`] and compare to the captured baseline.
///
/// On match: returns Ok(()) and the worker proceeds.
/// On mismatch: returns [`WritebackError::SchemaDrift`] — caller logs and exits.
pub async fn verify_schema_fingerprint(pool: &DbPool) -> WritebackResult<()> {
    let live_rows = fetch_live_columns(pool, WRITEBACK_FINGERPRINTED_TABLES).await?;
    let live_refs: Vec<(&str, i32, &str, &str)> = live_rows
        .iter()
        .map(|(t, o, c, ty)| (t.as_str(), *o, c.as_str(), ty.as_str()))
        .collect();
    let actual = compute_fingerprint(&live_refs);

    if actual != EXPECTED_FINGERPRINT {
        tracing::error!(
            expected = EXPECTED_FINGERPRINT,
            actual = %actual,
            "Schema fingerprint mismatch — refusing to write. \
             Re-run scripts/writeback-fingerprint.sh after auditing changes."
        );
        return Err(WritebackError::SchemaDrift {
            expected: EXPECTED_FINGERPRINT.to_string(),
            actual,
        });
    }

    tracing::info!(fingerprint = %actual, "Schema fingerprint OK (writeback)");
    Ok(())
}

/// Track D / T7 HIGH-3 — CT-watcher fingerprint guard.
///
/// Verifies BOTH the writeback baseline AND the CT-extra baseline,
/// because the CT watcher pulls CT rows from every table the writeback
/// touches PLUS the 5 additional mirror / TM.30 tables. The two
/// fingerprints are hashed independently so a vendor drift on a CT-only
/// table doesn't force a writeback baseline bump (and vice versa).
///
/// On any mismatch: returns [`WritebackError::SchemaDrift`] — caller
/// logs and exits. Same shape as [`verify_schema_fingerprint`] so the
/// CT watcher's startup gate can share the error-handling code path.
pub async fn verify_ct_schema_fingerprint(pool: &DbPool) -> WritebackResult<()> {
    // 1. Writeback baseline — same check the writeback worker runs.
    //    Sharing this guard means a drift on a writeback-touched table
    //    pages BOTH workers, which is what we want (correlated outage).
    verify_schema_fingerprint(pool).await?;

    // 2. CT-extra baseline — 5 tables the writeback never touches but
    //    the CT watcher mirrors / will mirror.
    let live_rows = fetch_live_columns(pool, CT_EXTRA_FINGERPRINTED_TABLES).await?;
    let live_refs: Vec<(&str, i32, &str, &str)> = live_rows
        .iter()
        .map(|(t, o, c, ty)| (t.as_str(), *o, c.as_str(), ty.as_str()))
        .collect();
    let actual = compute_fingerprint(&live_refs);

    if actual != CT_EXTRA_EXPECTED_FINGERPRINT {
        tracing::error!(
            expected = CT_EXTRA_EXPECTED_FINGERPRINT,
            actual = %actual,
            "CT-extra fingerprint mismatch — refusing to start the CT watcher. \
             A vendor change on HT_CheckIn_Product / HT_Deposit / HT_Bill_Debt_* \
             / HT_CheckIn_Other_People would silently corrupt CT-mapped \
             legacy_mirror.* rows. Re-audit + regenerate the baseline before retrying."
        );
        return Err(WritebackError::SchemaDrift {
            expected: CT_EXTRA_EXPECTED_FINGERPRINT.to_string(),
            actual,
        });
    }

    tracing::info!(fingerprint = %actual, "CT-extra fingerprint OK");
    Ok(())
}

/// Pull the live `(table, ord, column, type)` tuples for an arbitrary
/// list of fingerprinted tables. Ordered to match the baseline
/// iteration order. Used by both the writeback and CT fingerprint
/// guards via different `tables` slices.
async fn fetch_live_columns(
    pool: &DbPool,
    tables: &[&str],
) -> WritebackResult<Vec<(String, i32, String, String)>> {
    let mut conn = pool.get().await?;

    // information_schema.COLUMNS gives us everything we need without any
    // vendor-specific catalog views. The `IN (...)` builds a fixed list per
    // caller — small, hardcoded, no injection surface.
    let table_list = tables
        .iter()
        .map(|t| format!("'{t}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT TABLE_NAME, ORDINAL_POSITION, COLUMN_NAME, DATA_TYPE \
         FROM INFORMATION_SCHEMA.COLUMNS \
         WHERE TABLE_SCHEMA = 'dbo' AND TABLE_NAME IN ({table_list}) \
         ORDER BY TABLE_NAME, ORDINAL_POSITION",
    );

    // R2 (2026-05-14): wrap fingerprint metadata read in per-op
    // timeout — runs at writeback worker startup, so a wedged catalog
    // (rare but possible during a sys metadata rebuild) would
    // otherwise block worker boot indefinitely.
    let rows: Vec<Row> =
        simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;

    let mut out = Vec::with_capacity(rows.len());
    for row in &rows {
        let table: &str = row
            .get(0)
            .ok_or_else(|| WritebackError::Config("TABLE_NAME column missing".into()))?;
        let ord: i32 = row
            .get(1)
            .ok_or_else(|| WritebackError::Config("ORDINAL_POSITION column missing".into()))?;
        let column: &str = row
            .get(2)
            .ok_or_else(|| WritebackError::Config("COLUMN_NAME column missing".into()))?;
        let ty: &str = row
            .get(3)
            .ok_or_else(|| WritebackError::Config("DATA_TYPE column missing".into()))?;
        out.push((table.to_string(), ord, column.to_string(), ty.to_string()));
    }
    Ok(out)
}

/// Audit Wave 6 LOW item 8 — Ville cutover safety predicate.
///
/// Returns `Err` when the collation name contains `_CS_` (case-sensitive).
/// Pure function extracted so unit tests can exercise the rejection logic
/// without a live MSSQL connection.
fn collation_safety_check(collation: &str) -> WritebackResult<()> {
    if collation.contains("_CS_") {
        return Err(WritebackError::Config(format!(
            "Legacy server collation is case-sensitive ({collation}) — \
             writeback recipes assume case-insensitive (Thai_CI_AS). Refusing \
             to start to prevent silent SQL filter mismatches on a fresh Ville \
             cutover. Restore the database with COLLATE Thai_CI_AS or another \
             _CI_ collation before retrying."
        )));
    }
    Ok(())
}

/// Audit Wave 6 LOW item 8 — Ville cutover safety check.
///
/// Refuse to start the writeback worker if the server-level collation is
/// **case-sensitive** (`_CS_`). Every string literal in our recipes is
/// pinned to the case the .NET app emits (e.g. `'จอง'`, `'เข้าพัก'`,
/// `Cust_no` not `cust_no`); a case-sensitive collation would silently
/// fork our writes from the legacy app's filters on the day someone
/// stands up a fresh Ville instance with the wrong collation. This check
/// fails fast at startup so the misconfiguration is impossible to miss.
///
/// HF Hotel + HF Ville both run `Thai_CI_AS` (case-insensitive, accent-
/// sensitive) per the Ville-upgrade decision documented in
/// `MEMORY.md::ville_constraint`. We treat anything else with `_CS_` in
/// the name as a hard refusal; other deviations (different language
/// prefix, `_AS` vs `_AI`) log a WARN but don't block — the case
/// distinction is the one that breaks recipes byte-for-byte.
pub async fn verify_legacy_collation_safety(pool: &DbPool) -> WritebackResult<()> {
    let mut conn = pool.get().await?;
    // R2: per-op timeout — SERVERPROPERTY is a single round-trip but
    // runs at startup, same blocking concern as `fetch_live_columns`.
    let rows: Vec<Row> = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT CAST(SERVERPROPERTY('Collation') AS NVARCHAR(128)) AS c",
        MssqlOpKind::Read,
    )
    .await?;
    let row = rows.first().ok_or_else(|| {
        WritebackError::Config("SERVERPROPERTY('Collation') returned no rows".into())
    })?;
    let collation: &str = row
        .get(0)
        .ok_or_else(|| WritebackError::Config("Collation column missing".into()))?;
    collation_safety_check(collation)?;
    tracing::info!(collation, "Legacy MSSQL collation OK (not case-sensitive)");
    Ok(())
}

/// Suppress the unused-import warning on `Query` — kept for the future
/// CT-watcher bin that lives in the same crate.
#[allow(dead_code)]
fn _suppress_unused_query_import(_: Query<'static>) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The constant must equal what the function computes. If this test fails,
    /// either:
    /// - You modified `EXPECTED_SCHEMA_BASELINE` (intentionally, after a
    ///   vendor schema change). → Update `EXPECTED_FINGERPRINT` to the
    ///   value reported by this test's failure message.
    /// - The constant was edited without changing the baseline. → Restore the
    ///   constant from git history.
    #[test]
    fn fingerprint_constant_matches_baseline() {
        let computed = compute_fingerprint(EXPECTED_SCHEMA_BASELINE);
        assert_eq!(
            computed, EXPECTED_FINGERPRINT,
            "EXPECTED_FINGERPRINT must equal sha256 of EXPECTED_SCHEMA_BASELINE.\n\
             Recompute and update the constant if the baseline changed: '{computed}'"
        );
    }

    /// Track D / T7 HIGH-3 — the CT-extra constant must equal what the
    /// function computes over [`CT_EXTRA_SCHEMA_BASELINE`]. Same
    /// rationale as `fingerprint_constant_matches_baseline` for the
    /// writeback side. On failure the message reports the value to
    /// paste into [`CT_EXTRA_EXPECTED_FINGERPRINT`].
    #[test]
    fn ct_extra_fingerprint_constant_matches_baseline() {
        let computed = compute_fingerprint(CT_EXTRA_SCHEMA_BASELINE);
        assert_eq!(
            computed, CT_EXTRA_EXPECTED_FINGERPRINT,
            "CT_EXTRA_EXPECTED_FINGERPRINT must equal sha256 of CT_EXTRA_SCHEMA_BASELINE.\n\
             Recompute and update the constant if the baseline changed: '{computed}'"
        );
    }

    /// Track D / T7 HIGH-3 — ct_fingerprinted_tables() returns the
    /// union of writeback + CT-extra tables (20 tables today).
    #[test]
    fn ct_fingerprinted_tables_returns_union_of_writeback_and_ct_extra() {
        let union = ct_fingerprinted_tables();
        assert_eq!(
            union.len(),
            WRITEBACK_FINGERPRINTED_TABLES.len() + CT_EXTRA_FINGERPRINTED_TABLES.len()
        );
        for t in WRITEBACK_FINGERPRINTED_TABLES {
            assert!(union.contains(t), "writeback table {t} must be in CT union");
        }
        for t in CT_EXTRA_FINGERPRINTED_TABLES {
            assert!(union.contains(t), "CT-extra table {t} must be in CT union");
        }
    }

    /// Track D / T7 HIGH-3 — the 5 CT-extra tables are exactly the
    /// ones the audit named. Locking the list so a future refactor
    /// can't silently drop one (`HT_CheckIn_Other_People` gained its
    /// mapper in Track E1 but stays in this list — the fingerprint
    /// guard is independent of mapper presence).
    #[test]
    fn ct_extra_fingerprinted_tables_matches_audit_set() {
        let expected = [
            "HT_CheckIn_Product",
            "HT_Deposit",
            "HT_Bill_Debt_H",
            "HT_Bill_Debt_Ds",
            "HT_CheckIn_Other_People",
        ];
        assert_eq!(CT_EXTRA_FINGERPRINTED_TABLES, &expected);
    }

    /// Track D / T7 HIGH-3 — baseline row counts per table. Locks the
    /// column counts so a copy-paste regression doesn't silently drop
    /// a column from the baseline.
    #[test]
    fn ct_extra_baseline_column_counts_match_schema() {
        let cases: &[(&str, usize)] = &[
            ("HT_Bill_Debt_Ds", 9),
            ("HT_Bill_Debt_H", 18),
            ("HT_CheckIn_Other_People", 4),
            ("HT_CheckIn_Product", 12),
            ("HT_Deposit", 8),
        ];
        for (table, expected_count) in cases {
            let rows: Vec<_> = CT_EXTRA_SCHEMA_BASELINE
                .iter()
                .filter(|(t, _, _, _)| *t == *table)
                .collect();
            assert_eq!(
                rows.len(),
                *expected_count,
                "expected {expected_count} columns for {table}, got {}",
                rows.len(),
            );
        }
    }

    #[test]
    fn fingerprint_is_order_sensitive() {
        let mut shuffled = EXPECTED_SCHEMA_BASELINE.to_vec();
        shuffled.swap(0, 1);
        assert_ne!(compute_fingerprint(EXPECTED_SCHEMA_BASELINE), compute_fingerprint(&shuffled));
    }

    #[test]
    fn fingerprint_detects_renamed_column() {
        let mut altered = EXPECTED_SCHEMA_BASELINE.to_vec();
        altered[0] = (altered[0].0, altered[0].1, "Cust_NEWNAME", altered[0].3);
        assert_ne!(
            compute_fingerprint(EXPECTED_SCHEMA_BASELINE),
            compute_fingerprint(&altered),
        );
    }

    #[test]
    fn fingerprint_detects_retyped_column() {
        let mut altered = EXPECTED_SCHEMA_BASELINE.to_vec();
        altered[0] = (altered[0].0, altered[0].1, altered[0].2, "nvarchar");
        assert_ne!(
            compute_fingerprint(EXPECTED_SCHEMA_BASELINE),
            compute_fingerprint(&altered),
        );
    }

    /// Wave 5a item 5 — the recipes touch five additional tables that
    /// the baseline list missed: `HT_Cupon` (loyalty coupons),
    /// `HT_CheckIn_Pay` (payments), `HT_Receipt_Ds` (receipt lines),
    /// `HT_POWER_LOG` (light on/off audit), and `HT_Changed_Room`
    /// (room-move audit). A vendor rename on any of these would
    /// previously silently corrupt our writeback; now the startup
    /// fingerprint check refuses to run.
    #[test]
    fn fingerprinted_tables_includes_wave_5a_additions() {
        for table in [
            "HT_Cupon",
            "HT_CheckIn_Pay",
            "HT_Receipt_Ds",
            "HT_POWER_LOG",
            "HT_Changed_Room",
        ] {
            assert!(
                FINGERPRINTED_TABLES.contains(&table),
                "Wave 5a expects {table} to be fingerprinted"
            );
        }
    }

    #[test]
    fn tracks_ht_cupon_table() {
        // The HT_Cupon mark-printed UPDATE fires from walkin +
        // checkin_to_booking recipes via helpers::mark_cupon_printed.
        // Baseline must include every column the vendor schema lists.
        let cupon_rows: Vec<_> = EXPECTED_SCHEMA_BASELINE
            .iter()
            .filter(|(t, _, _, _)| *t == "HT_Cupon")
            .collect();
        assert_eq!(
            cupon_rows.len(),
            7,
            "HT_Cupon has 7 columns per docs/legacy-spike/schema/01-baseline-schema.txt"
        );
        assert!(cupon_rows.iter().any(|(_, _, c, _)| *c == "cupon_print"));
    }

    #[test]
    fn tracks_ht_checkin_pay_table() {
        let pay_rows: Vec<_> = EXPECTED_SCHEMA_BASELINE
            .iter()
            .filter(|(t, _, _, _)| *t == "HT_CheckIn_Pay")
            .collect();
        // 22 columns per baseline schema lines 253-274.
        assert_eq!(pay_rows.len(), 22);
        assert!(pay_rows.iter().any(|(_, _, c, _)| *c == "Cin_Pay_Tran"));
        assert!(pay_rows.iter().any(|(_, _, c, _)| *c == "Cin_Pay_web"));
    }

    #[test]
    fn tracks_ht_receipt_ds_table() {
        let ds_rows: Vec<_> = EXPECTED_SCHEMA_BASELINE
            .iter()
            .filter(|(t, _, _, _)| *t == "HT_Receipt_Ds")
            .collect();
        // 10 columns per baseline schema lines 445-454.
        assert_eq!(ds_rows.len(), 10);
        assert!(ds_rows.iter().any(|(_, _, c, _)| *c == "S_PriceDiscount"));
    }

    #[test]
    fn tracks_ht_power_log_table() {
        let pl_rows: Vec<_> = EXPECTED_SCHEMA_BASELINE
            .iter()
            .filter(|(t, _, _, _)| *t == "HT_POWER_LOG")
            .collect();
        // 8 columns per baseline schema lines 422-429.
        assert_eq!(pl_rows.len(), 8);
        assert!(pl_rows.iter().any(|(_, _, c, _)| *c == "ROOM_POWER_NOTE"));
    }

    #[test]
    fn tracks_ht_changed_room_table() {
        let rows: Vec<_> = EXPECTED_SCHEMA_BASELINE
            .iter()
            .filter(|(t, _, _, _)| *t == "HT_Changed_Room")
            .collect();
        // 8 columns per baseline schema lines 200-207.
        assert_eq!(rows.len(), 8);
        assert!(rows.iter().any(|(_, _, c, _)| *c == "room_before"));
        assert!(rows.iter().any(|(_, _, c, _)| *c == "room_after"));
    }

    // --- Wave 6 LOW item 8 — collation safety check ---

    #[test]
    fn collation_safety_accepts_case_insensitive_thai() {
        // Production collation on both HF Hotel + HF Ville (verified
        // 2026-04-29 Ville upgrade — MEMORY.md::ville_constraint).
        assert!(collation_safety_check("Thai_CI_AS").is_ok());
    }

    #[test]
    fn collation_safety_accepts_other_case_insensitive_variants() {
        // Other plausible _CI_ collations a future env might use. None
        // contain `_CS_` so all must pass.
        for ok in [
            "Thai_CI_AI",
            "SQL_Latin1_General_CP1_CI_AS",
            "Latin1_General_100_CI_AS_SC",
        ] {
            assert!(
                collation_safety_check(ok).is_ok(),
                "expected {ok:?} to be accepted (no `_CS_`)"
            );
        }
    }

    #[test]
    fn collation_safety_rejects_case_sensitive_thai() {
        let err = collation_safety_check("Thai_CS_AS").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("case-sensitive"));
        assert!(msg.contains("Thai_CS_AS"));
        assert!(msg.contains("Refusing"));
    }

    #[test]
    fn collation_safety_rejects_any_cs_variant() {
        for bad in [
            "Thai_CS_AS",
            "SQL_Latin1_General_CP1_CS_AS",
            "Latin1_General_100_CS_AS_SC",
        ] {
            assert!(
                collation_safety_check(bad).is_err(),
                "expected {bad:?} to be rejected (contains `_CS_`)"
            );
        }
    }
}
