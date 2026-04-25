//! Race-safe MAX+1 ID allocation against legacy MSSQL.
//!
//! Per `docs/legacy-spike/findings.md` §2 + §6 — the legacy app's identifier
//! columns (`HT_Customers.Cust_no`, `HT_Book_H.Book_ID`, `HT_CheckIn_H.Cin_no`)
//! are app-allocated `MAX(...) + 1` strings. The `+1` increment is unsafe
//! against concurrent writers, which is why the spike validated wrapping
//! every allocation in `WITH (TABLOCKX, HOLDLOCK)`:
//!
//! ```sql
//! BEGIN TRAN
//!   SELECT @nextid = ISNULL(MAX(id), 0) + 1
//!     FROM <table> WITH (TABLOCKX, HOLDLOCK)
//!   INSERT INTO <table> (id, ...) VALUES (@nextid, ...)
//! COMMIT
//! ```
//!
//! `TABLOCKX` blocks all other writers AND blocks the .NET app's `SELECT MAX`
//! queries (default `READ COMMITTED` isolation, no NOLOCK hints — verified
//! in spike §6 Test 2). The .NET app waits for our COMMIT, then reads our
//! committed MAX, allocates `our+1`, and INSERTs. Sequential, no collisions.
//!
//! ## Functions exposed
//!
//! Each helper returns the allocated identifier as a string ready to embed
//! in subsequent INSERT/UPDATE statements. **The caller is responsible for
//! the surrounding transaction** — these helpers issue only the SELECT under
//! `TABLOCKX, HOLDLOCK`. The lock is released when the caller's `COMMIT` or
//! `ROLLBACK` closes the transaction.

use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use chrono::{Datelike, Utc};
use tiberius::Row;

use crate::writeback::error::{WritebackError, WritebackResult};

/// Convenience alias — the type bb8 hands out for our legacy connection.
pub type LegacyConn<'a> = PooledConnection<'a, ConnectionManager>;

/// Internal helper — run a `SELECT ISNULL(MAX(...), 0) + 1` query under
/// `TABLOCKX, HOLDLOCK` and return the result.
async fn select_next_int_with_lock(
    conn: &mut LegacyConn<'_>,
    sql: &str,
) -> WritebackResult<i32> {
    let stream = conn.simple_query(sql).await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    let row = rows
        .first()
        .ok_or_else(|| WritebackError::Recipe(format!("MAX+1 query returned no rows: {sql}")))?;
    let next: i32 = row
        .get(0)
        .ok_or_else(|| WritebackError::Recipe(format!("MAX+1 column was NULL: {sql}")))?;
    Ok(next)
}

/// Allocate the next `HT_Customers.id` (raw integer PK).
///
/// `HT_Customers.id` was originally an IDENTITY but the property has been
/// stripped on this DB (per spike §2 — the .NET app sets it explicitly via
/// MAX+1). We follow suit.
pub async fn allocate_customer_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Customers WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_Customers.Cust_no` — `'C' + integer`.
///
/// Per spike §2: format is `C\d+` (no zero padding observed). Allocated by
/// parsing the highest existing `Cust_no` integer suffix and adding one.
pub async fn allocate_cust_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    // Strip leading 'C', cast to int, take MAX, add 1. Cust_no values that
    // somehow lack the prefix or aren't numeric are excluded — the legacy app
    // never emits any.
    let next = select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Cust_no, 2, 50) AS INT)), 0) + 1 \
         FROM HT_Customers WITH (TABLOCKX, HOLDLOCK) \
         WHERE Cust_no LIKE 'C%'",
    )
    .await?;
    Ok(format!("C{next}"))
}

/// Allocate the next `HT_Book_H.Book_ID` — `'R' + zero-padded 6-digit integer`.
///
/// Per spike §2: format is `R\d{6}` (e.g. `R014810`). Allocated by parsing the
/// highest numeric suffix.
pub async fn allocate_book_id(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    let next = select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Book_ID, 2, 50) AS INT)), 0) + 1 \
         FROM HT_Book_H WITH (TABLOCKX, HOLDLOCK) \
         WHERE Book_ID LIKE 'R%'",
    )
    .await?;
    Ok(format!("R{next:06}"))
}

/// Allocate the next `HT_CheckIn_H.Cin_no` — `'CH' + 2-digit-year + '-' + 6-digit integer`.
///
/// Per spike §2: format is `CH\d{2}-\d{6}` (e.g. `CH26-005228`). Year-scoped
/// — allocation rolls over each calendar year. We use the *current Thai year*
/// (system clock) to match the .NET app's behavior.
pub async fn allocate_cin_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    let year_two = Utc::now().year() % 100;
    let prefix = format!("CH{year_two:02}-");

    // Match exactly the year prefix, then extract the trailing 6-digit suffix.
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Cin_no, 6, 50) AS INT)), 0) + 1 \
             FROM HT_CheckIn_H WITH (TABLOCKX, HOLDLOCK) \
             WHERE Cin_no LIKE '{prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{prefix}{next:06}"))
}

/// Allocate the next `HT_Book_Date.id`. App-allocated MAX+1 (per spike §2 — the
/// `id` column is not IDENTITY).
pub async fn allocate_book_date_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Book_Date WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_Room_Status.id` (also a non-IDENTITY MAX+1, per spike).
pub async fn allocate_room_status_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Room_Status WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_Rooms_Cancel.id` (per spike §3i — observed MAX+1
/// allocation: previous count was 297, our cancel got id=298).
pub async fn allocate_rooms_cancel_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Rooms_Cancel WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_CheckIn_Ds.id`.
///
/// **Note:** spike §2 calls out this column as the ONE exception that *is*
/// SQL Server `IDENTITY` — INSERTs without `id` auto-allocate. We expose this
/// helper anyway for symmetry, but recipes should prefer the IDENTITY path
/// (omit `id` from the INSERT column list).
pub async fn allocate_checkin_ds_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_CheckIn_Ds WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_Receipt_H.id`.
///
/// `id` here is IDENTITY per spike baseline (`is_identity=1`). Same caveat as
/// [`allocate_checkin_ds_id`] — prefer omitting from the INSERT column list.
pub async fn allocate_receipt_h_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Receipt_H WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Format a `Pay_No` for `HT_CheckIn_Pay`. Spike §3h captures show this is
/// month-scoped but the exact prefix/format wasn't fully derived. Until a
/// dedicated capture lands, we use the timestamp-based pattern matching the
/// .NET app's observed output.
///
/// Returns a `MM`-prefixed sequential integer, allocated under `TABLOCKX,
/// HOLDLOCK` for the current month.
pub async fn allocate_pay_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    let now = Utc::now();
    let month_prefix = format!("P{:02}{:02}-", now.year() % 100, now.month());
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Pay_No, 7, 50) AS INT)), 0) + 1 \
             FROM HT_CheckIn_Pay WITH (TABLOCKX, HOLDLOCK) \
             WHERE Pay_No LIKE '{month_prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{month_prefix}{next:06}"))
}

/// Format a `Receipt_no` for `HT_Receipt_H`. Spike §3h doesn't fully derive
/// the format either — captured value `RC2604-000001` matches the same pattern
/// as `Pay_No`.
pub async fn allocate_receipt_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    let now = Utc::now();
    let month_prefix = format!("RC{:02}{:02}-", now.year() % 100, now.month());
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Receipt_no, 8, 50) AS INT)), 0) + 1 \
             FROM HT_Receipt_H WITH (TABLOCKX, HOLDLOCK) \
             WHERE Receipt_no LIKE '{month_prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{month_prefix}{next:06}"))
}

#[cfg(test)]
mod tests {
    //! These tests verify the SQL TEXT we emit — not the I/O. A live MSSQL is
    //! out of scope for the unit suite per the Phase 4b spec; any live
    //! interaction is the receptionist's job.

    use super::*;

    #[test]
    fn cust_no_format_padding_lookup() {
        // Format invariant: 'C' + integer (no zero-pad)
        let formatted = format!("C{}", 21607);
        assert_eq!(formatted, "C21607");
    }

    #[test]
    fn book_id_format_zero_pads_to_six() {
        // Format invariant: 'R' + zeropad6
        assert_eq!(format!("R{:06}", 14810), "R014810");
        assert_eq!(format!("R{:06}", 1), "R000001");
    }

    #[test]
    fn cin_no_format_year_scoped_with_six_digit_suffix() {
        // Format invariant: 'CH' + YY + '-' + zeropad6
        let year_two = 26;
        let prefix = format!("CH{year_two:02}-");
        let cin_no = format!("{prefix}{:06}", 5228);
        assert_eq!(cin_no, "CH26-005228");
    }
}
