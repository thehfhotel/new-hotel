//! Parent-aggregate re-loader.
//!
//! When CT delivers a row from one of the booking child tables
//! (`HT_Book_Ds`, `HT_Book_Date`), the watcher needs the *full* booking
//! aggregate — header + every line + every calendar night — to rebuild
//! the canonical PG projection.
//!
//! The simplest correct shape is "throw away whatever the CT row carried
//! and re-`SELECT *` the parent + children by `Book_no`". That eliminates
//! diffing logic entirely; the canonical UPSERT becomes a single
//! "this is the new aggregate state, replace the row" call.
//!
//! ## Re-use across phases
//!
//! Phase 5.4 brings `HT_CheckIn_H` + `HT_CheckIn_Ds` + `HT_CheckIn_Pay`
//! online. Those mappers will need an identical helper:
//! `load_checkin_aggregate(cin_no)`. The internal helper
//! [`fetch_rows`] is generic over `(table, where_col, where_val)` so the
//! check-in version is one extra public wrapper, no refactor.
//!
//! ## Why not use the existing `BookingRepository`?
//!
//! `BookingRepository` reads from canonical PG; this loader reads from
//! legacy MSSQL. Different driver, different row shapes, different table
//! names — they coincidentally share the word "booking" but otherwise
//! have nothing in common.

use crate::db::DbPool;
use crate::sync::row::test_support::{HashMapRow, MockValue};
use crate::sync::SyncError;

/// Owned snapshot of one booking aggregate as it currently lives in
/// legacy MSSQL.
///
/// `header` is `None` when the booking was deleted. `rooms` and `nights`
/// are the as-of-now state of the child tables for that `Book_no`.
#[derive(Debug, Clone)]
pub struct BookingAggregate {
    /// `HT_Book_H` row (one per booking). `None` when the header has
    /// been deleted/cancelled-with-purge — caller treats this as "the
    /// aggregate no longer exists" and emits a `BookingCancelled`.
    pub header: Option<HashMapRow>,
    /// `HT_Book_Ds` rows (one per assigned room). Empty when the
    /// booking has no detail rows (legitimate transient state during
    /// an edit / cancel-then-recreate).
    pub rooms: Vec<HashMapRow>,
    /// `HT_Book_Date` rows (one per calendar night). Empty when the
    /// booking is older than 60 days — frmMain1 startup prunes these
    /// (cheatsheet §3.7 "Startup prune"). Mappers must not infer
    /// "cancelled" from an empty `nights` vector.
    pub nights: Vec<HashMapRow>,
}

impl BookingAggregate {
    pub fn is_present(&self) -> bool {
        self.header.is_some()
    }
}

/// Owned snapshot of one check-in aggregate as it currently lives in
/// legacy MSSQL.
///
/// Mirrors the booking shape: `header` is `None` when the legacy header
/// has been deleted (caller emits `CheckInCancelled`). `rooms` are the
/// per-room detail lines (`HT_CheckIn_Ds`) and `payments` are the
/// payment ledger rows (`HT_CheckIn_Pay`) for that `Cin_no`.
///
/// Per the 5.4 spec the check-in aggregate's `payments` collection rolls
/// up into `ht_checkins.cin_paid_amount` so the `HT_CheckIn_Pay` mapper
/// can re-trigger an aggregate sync without a separate code path.
#[derive(Debug, Clone)]
pub struct CheckInAggregate {
    /// `HT_CheckIn_H` row (one per `Cin_no`). `None` when the header
    /// has been deleted — caller treats this as "the aggregate no
    /// longer exists" and emits a `CheckInCancelled`.
    pub header: Option<HashMapRow>,
    /// `HT_CheckIn_Ds` rows (one per assigned room). For multi-room
    /// stays this is >1; for the typical single-room flow it's exactly 1.
    pub rooms: Vec<HashMapRow>,
    /// `HT_CheckIn_Pay` rows (one per tender event). Empty for stays
    /// that haven't paid yet.
    pub payments: Vec<HashMapRow>,
}

impl CheckInAggregate {
    pub fn is_present(&self) -> bool {
        self.header.is_some()
    }
}

/// Pull `HT_Book_H` + all `HT_Book_Ds` rows + all `HT_Book_Date` rows
/// for one booking by `Book_no`.
///
/// Returns `Ok(BookingAggregate { header: None, … })` when the booking
/// header is missing — caller emits `BookingCancelled`.
///
/// Errors are transient: the caller (the watcher) records them in
/// `legacy_sync_status.last_error` and retries on the next tick.
pub async fn load_booking_aggregate(
    mssql: &DbPool,
    book_no: &str,
) -> Result<BookingAggregate, SyncError> {
    // Header is keyed by `Book_ID`; details by `Book_No`. Same business
    // value, different column case — verified from `_SCHEMA.sql`.
    let header_rows = fetch_rows(
        mssql,
        "HT_Book_H",
        "Book_ID",
        book_no,
        // Mirror what the booking_create recipe writes (cheatsheet §3.3 +
        // writeback/recipes/booking_create.rs). The watcher does NOT need
        // to project EVERY column — only the ones the canonical PG row
        // mirrors. Add columns here as the canonical projection grows.
        &[
            "Book_ID",
            "Book_Date",
            "Book_Cust_ID",
            "Book_Cust_Name",
            "Book_Cust_Tel",
            "Book_Price_Total",
            "Book_Price_Pay",
            "Book_Status",
            "Book_Date_in",
            "Book_Date_out",
            "Book_by",
            "Book_room_note",
        ],
    )
    .await?;
    let header = header_rows.into_iter().next();

    let rooms = fetch_rows(
        mssql,
        "HT_Book_Ds",
        "Book_No",
        book_no,
        // `Book_Room_Type` stores the room NUMBER per cheatsheet §3.4.
        &[
            "id",
            "Book_No",
            "Book_Room_Type",
            "Book_Room_Start",
            "Book_Room_End",
            "Book_Room_Price",
            "Book_Room_Night",
            "Book_Room_Num",
            "Book_Room_PriceToTal",
            "Book_status",
        ],
    )
    .await?;

    let nights = fetch_rows(
        mssql,
        "HT_Book_Date",
        "Book_no",
        book_no,
        &[
            "id",
            "Book_no",
            "Book_type",
            "Book_date_ds",
            "Book_Num",
            "Book_USE",
            "Book_ok",
        ],
    )
    .await?;

    Ok(BookingAggregate {
        header,
        rooms,
        nights,
    })
}

/// Pull `HT_CheckIn_H` + all `HT_CheckIn_Ds` rows + all `HT_CheckIn_Pay`
/// rows for one check-in by `Cin_no`.
///
/// Returns `Ok(CheckInAggregate { header: None, … })` when the check-in
/// header is missing — caller emits `CheckInCancelled`.
///
/// Note the WHERE-column casing per cheatsheet §3.4 / §3.4 schema dump:
/// `HT_CheckIn_H.Cin_no` (lowercase n), `HT_CheckIn_Ds.Cin_No` (capital
/// N — the discrepancy is verbatim from the legacy schema), and
/// `HT_CheckIn_Pay.Cin_No` (also capital N).
pub async fn load_checkin_aggregate(
    mssql: &DbPool,
    cin_no: &str,
) -> Result<CheckInAggregate, SyncError> {
    let header_rows = fetch_rows(
        mssql,
        "HT_CheckIn_H",
        "Cin_no",
        cin_no,
        // Mirror what walkin / checkin_to_booking recipes write
        // (cheatsheet §3.6, walkin/writes.txt). Add columns here as the
        // canonical PG projection grows.
        &[
            "Cin_no",
            "Cin_Date",
            "Cin_Book_no",
            "Cin_cust_no",
            "Cin_status",
            "Total_Price_Room",
            "Total_Price_Net",
            "Total_Price_Pay",
            "Total_Price_Balance",
            "Cin_Date_in",
            "Cin_Date_Out",
            "Cin_by",
            "Cin_Room_ALL",
        ],
    )
    .await?;
    let header = header_rows.into_iter().next();

    let rooms = fetch_rows(
        mssql,
        "HT_CheckIn_Ds",
        // Legacy schema uses capital N here (cheatsheet §3.4 schema
        // dump: `Cin_No varchar(50)`). Locked test in
        // sync/mappers/checkin.rs guards against accidental rename.
        "Cin_No",
        cin_no,
        &[
            "id",
            "Cin_No",
            "Cin_Room_No",
            "Cin_Room_Type",
            "Cin_Room_In",
            "Cin_Room_Out",
            "Cin_Room_Status",
            "Cin_Room_Price",
            "Cin_Room_Night",
            "Cin_Room_PriceToTal",
            "Cin_Room_Pay_Total",
        ],
    )
    .await?;

    let payments = fetch_rows(
        mssql,
        "HT_CheckIn_Pay",
        "Cin_No",
        cin_no,
        &[
            "id",
            "Cin_No",
            "Cin_Pay_Date",
            "Cin_Pay_Cash",
            "Cin_Pay_Credit",
            "Cin_Pay_Tran",
            "Pay_No",
            "Cin_Pay_Status",
        ],
    )
    .await?;

    Ok(CheckInAggregate {
        header,
        rooms,
        payments,
    })
}

/// Generic single-table read by `<col> = <val>`.
///
/// Returns the rows materialised through the same `MappableRow` impl
/// the rest of the sync layer uses, so consumers (mappers, tests) only
/// know one row shape.
///
/// Kept generic so 5.4's `load_checkin_aggregate(cin_no)` can call into
/// the same helper with `("HT_CheckIn_H", "Cin_no", cin_no, …)` without
/// duplicating the SQL-build / row-materialise plumbing.
async fn fetch_rows(
    mssql: &DbPool,
    table: &'static str,
    where_col: &'static str,
    where_val: &str,
    projection: &[&'static str],
) -> Result<Vec<HashMapRow>, SyncError> {
    let mut conn = mssql.get().await?;

    // SQL-quote the WHERE value (single-quote doubling). This is the
    // same boundary as the writeback dispatcher uses; we cannot reach
    // for a parameterised tiberius query without losing the streaming
    // shape that `simple_query` provides.
    let where_q = sql_quote_inline(where_val);
    let select_list = projection.join(", ");
    let sql = format!("SELECT {select_list} FROM {table} WHERE {where_col} = {where_q}");

    let stream = conn.simple_query(&sql).await?;
    let raw_rows = stream.into_first_result().await?;

    let mut out = Vec::with_capacity(raw_rows.len());
    for r in &raw_rows {
        out.push(materialise(r, table, projection));
    }
    Ok(out)
}

/// SQL-quote a value for inline interpolation. Mirrors
/// `crate::writeback::format::sql_quote` semantics; copied here to keep
/// `sync` independent of `writeback`'s public surface.
fn sql_quote_inline(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Convert a tiberius row into the shared `HashMapRow`. Mirrors the
/// boundary translator in `bin/sync.rs::materialise_row` — same probe
/// order, same fall-throughs.
fn materialise(
    row: &tiberius::Row,
    table: &'static str,
    projection: &[&'static str],
) -> HashMapRow {
    let mut h = HashMapRow::new(table);
    for col in projection {
        let cell = read_cell(row, col).unwrap_or(MockValue::Null);
        h.cells.insert((*col).to_string(), cell);
    }
    h
}

/// Probe a tiberius cell as the most-specific type that succeeds.
/// `f64` is probed BEFORE `i64` so a `numeric`/`float` column doesn't
/// silently coerce to integer.
fn read_cell(row: &tiberius::Row, col: &str) -> Option<MockValue> {
    if let Ok(Some(s)) = tiberius::Row::try_get::<&str, _>(row, col) {
        return Some(MockValue::Str(s.to_string()));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i32, _>(row, col) {
        return Some(MockValue::I32(n));
    }
    if let Ok(Some(d)) = tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, col) {
        return Some(MockValue::DateTime(d));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<f64, _>(row, col) {
        return Some(MockValue::Decimal(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i64, _>(row, col) {
        return Some(MockValue::I64(n));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `BookingAggregate::is_present` mirrors `header.is_some()` — used
    /// by the booking mapper to choose between `BookingCancelled` and
    /// `BookingCreated` / `BookingModified`.
    #[test]
    fn is_present_is_true_when_header_set() {
        let agg = BookingAggregate {
            header: Some(HashMapRow::new("HT_Book_H")),
            rooms: vec![],
            nights: vec![],
        };
        assert!(agg.is_present());
    }

    #[test]
    fn is_present_is_false_when_header_missing() {
        let agg = BookingAggregate {
            header: None,
            rooms: vec![HashMapRow::new("HT_Book_Ds")],
            nights: vec![HashMapRow::new("HT_Book_Date")],
        };
        assert!(!agg.is_present());
    }

    #[test]
    fn sql_quote_inline_doubles_embedded_quotes() {
        assert_eq!(sql_quote_inline("R014810"), "'R014810'");
        assert_eq!(sql_quote_inline("O'Brien"), "'O''Brien'");
        assert_eq!(sql_quote_inline(""), "''");
    }

    /// Phase 5.4 — `CheckInAggregate::is_present` mirrors the booking
    /// version. Used by the check-in mapper to choose between
    /// `CheckInCancelled` (header gone) and `CheckInCreated` /
    /// `CheckInModified` (header present).
    #[test]
    fn checkin_is_present_is_true_when_header_set() {
        let agg = CheckInAggregate {
            header: Some(HashMapRow::new("HT_CheckIn_H")),
            rooms: vec![],
            payments: vec![],
        };
        assert!(agg.is_present());
    }

    #[test]
    fn checkin_is_present_is_false_when_header_missing() {
        let agg = CheckInAggregate {
            header: None,
            rooms: vec![HashMapRow::new("HT_CheckIn_Ds")],
            payments: vec![HashMapRow::new("HT_CheckIn_Pay")],
        };
        assert!(!agg.is_present());
    }
}
