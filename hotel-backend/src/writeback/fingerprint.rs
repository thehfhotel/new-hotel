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

use crate::db::DbPool;
use crate::writeback::error::{WritebackError, WritebackResult};

/// Tables the writeback worker touches. The fingerprint check covers exactly
/// these — adding a recipe that writes to a new table requires extending this
/// list AND regenerating [`EXPECTED_FINGERPRINT`].
pub const FINGERPRINTED_TABLES: &[&str] = &[
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
];

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
    ("HT_Book_H", 15, "book_room_type", "int"),
    ("HT_Book_H", 16, "Book_Notify_Day", "int"),
    ("HT_Book_H", 17, "Book_Notify_Note", "varchar"),
    ("HT_Book_H", 18, "Book_Sale", "varchar"),
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
    "5f2c17bc402edfc80e04fecb9dd741e26ed4cf1036f16855626051cd276376d2";

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

/// Query MSSQL for the live column tuples of [`FINGERPRINTED_TABLES`] and
/// compare to the captured baseline.
///
/// On match: returns Ok(()) and the worker proceeds.
/// On mismatch: returns [`WritebackError::SchemaDrift`] — caller logs and exits.
pub async fn verify_schema_fingerprint(pool: &DbPool) -> WritebackResult<()> {
    let live_rows = fetch_live_columns(pool).await?;
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

    tracing::info!(fingerprint = %actual, "Schema fingerprint OK");
    Ok(())
}

/// Pull the live `(table, ord, column, type)` tuples for the fingerprinted
/// tables. Ordered to match [`EXPECTED_SCHEMA_BASELINE`] iteration order.
async fn fetch_live_columns(pool: &DbPool) -> WritebackResult<Vec<(String, i32, String, String)>> {
    let mut conn = pool.get().await?;

    // information_schema.COLUMNS gives us everything we need without any
    // vendor-specific catalog views. The `IN (...)` builds a fixed list per
    // FINGERPRINTED_TABLES — small, hardcoded, no injection surface.
    let table_list = FINGERPRINTED_TABLES
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

    let stream = conn.simple_query(sql).await?;
    let rows: Vec<Row> = stream.into_first_result().await?;

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
}
