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

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
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

/// `HT_Book_H` projection for the booking aggregate loader.
///
/// Track J1 — held as a module-private const so the projection-lock
/// test can pin every column against the authoritative HF Hotel schema
/// dump. Mirrors what the booking_create recipe writes (cheatsheet §3.3
/// + writeback/recipes/booking_create.rs). The watcher does NOT need to
/// project EVERY column — only the ones the canonical PG row mirrors.
const BOOK_H_PROJECTION: &[&str] = &[
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
    // 2026-06-11 — disambiguates HT_Book_Ds.Book_Room_Type semantics:
    // 1 = Ds lines carry a room-TYPE code (FrmAddBook, cheatsheet §3.3),
    // 2 = Ds lines carry the room NUMBER (FrmAddBook2, §3.4). The
    // booking projection drops Ds lines from the room-assignment set
    // when this is 1.
    "Book_room_type",
];

/// `HT_Book_Ds` projection. `Book_Room_Type` stores the room NUMBER
/// when `HT_Book_H.Book_room_type=2` (cheatsheet §3.4) and a room-TYPE
/// code when it is 1 (§3.3). Track J1 projection-lock test pins every
/// column.
const BOOK_DS_PROJECTION: &[&str] = &[
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
];

/// `HT_Book_Date` projection. Track J1 projection-lock test pins every
/// column.
const BOOK_DATE_PROJECTION: &[&str] = &[
    "id",
    "Book_no",
    "Book_type",
    "Book_date_ds",
    "Book_Num",
    "Book_USE",
    "Book_ok",
];

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
        BOOK_H_PROJECTION,
        // Header table — single row expected, no ORDER BY required.
        None,
    )
    .await?;
    let header = header_rows.into_iter().next();

    let rooms = fetch_rows(
        mssql,
        "HT_Book_Ds",
        "Book_No",
        book_no,
        BOOK_DS_PROJECTION,
        // Track B2 / T2 HIGH-1 — stable iteration order across CT ticks
        // (legacy `id` is the IDENTITY PK; matches write order).
        Some("id ASC"),
    )
    .await?;

    // Track E1 / T2 MED-1 (audit 2026-05-13) — `HT_Book_Date` rows are
    // loaded here for aggregate completeness (the booking-aggregate
    // shape mirrors the legacy schema's three-table composition) but
    // the canonical projection in `sync/mappers/booking.rs::
    // project_aggregate` does NOT currently surface them. The per-night
    // `Book_ok` cancellation state would feed an "n nights cancelled
    // mid-stay" facet on `ht_bookings` (Track G surface), but until
    // that column lands the rows are dropped intentionally rather than
    // bag-on-the-floor accidentally. The data sits in `agg.nights`
    // ready for the projection to read when E2/G adds the column.
    let nights = fetch_rows(
        mssql,
        "HT_Book_Date",
        "Book_no",
        book_no,
        BOOK_DATE_PROJECTION,
        // Stable iteration so per-night accumulations are deterministic
        // (Track B2 / T2 HIGH-1).
        Some("id ASC"),
    )
    .await?;

    Ok(BookingAggregate {
        header,
        rooms,
        nights,
    })
}

/// `HT_CheckIn_Ds` projection used by `load_checkin_aggregate`.
///
/// Held as a module-private const so a unit test can lock the column
/// names against the authoritative HF Hotel schema dump without needing
/// a live MSSQL connection. See the comment on the projection's caller
/// for the schema-dump cross-reference and the PROD-CRIT post-mortem.
const CHECKIN_DS_PROJECTION: &[&str] = &[
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
    "Cin_Room_Dep",
    "Cin_Dep_Status",
    "Cin_Dep_return_date",
    "Cin_Dep_return_by",
];

/// `HT_CheckIn_H` projection used by `load_checkin_aggregate`. Mirrors
/// what walkin / checkin_to_booking recipes write (cheatsheet §3.6,
/// walkin/writes.txt). Track J1 projection-lock test pins every column.
const CHECKIN_H_PROJECTION: &[&str] = &[
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
];

/// `HT_CheckIn_Pay` projection used by `load_checkin_aggregate`.
///
/// Track C — T2 CRIT-2 (`docs/coexistence/audit-2026-05-13.md`):
/// `Cin_Status` projected so the check-in aggregate sweep can exclude
/// cancelled payment rows (`'ยกเลิก'`) from `cin_paid_amount`. Before
/// this fix, a folio cancelled in iHOTEL kept the cascade-marked
/// payment rows visible in our canonical `ht_payments` aggregation —
/// `cin_paid_amount` diverged from the legacy view.
///
/// Track J1 projection-lock test pins every column.
const CHECKIN_PAY_PROJECTION: &[&str] = &[
    "id",
    "Cin_No",
    "Cin_Pay_Date",
    "Cin_Pay_Cash",
    "Cin_Pay_Credit",
    "Cin_Pay_Tran",
    "Cin_Pay_Free",
    "Cin_Pay_web",
    "Pay_No",
    "Cin_Status",
    // Track J7a — line-detail columns for the canonical `ht_payment_ledger`
    // mirror (income-by-tender reconciliation + iHOTEL-equivalent shift
    // report). All exist on the legacy `HT_CheckIn_Pay` schema
    // (COMPAT_CHEATSHEET §`HT_CheckIn_Pay` lines 485-497). Pulled on the same
    // read `apply_payment_aggregate` already issues — no extra query. The
    // check-in aggregate sweep ignores them; only the ledger mirror reads them.
    "Cin_Pay_Ds_Price", // line total (= sum of the 5 tenders)
    "Cin_Pay_Ds_ID",    // category code (P001 = room-rent line)
    "Cin_Pay_Ds_Name",  // human description (ค่าห้อง, น้ำดื่ม, …)
    "Cin_Pay_Ds",       // short label (often room no)
    "Cin_Pay_Ds_Num",   // qty / nights
    "Cin_Cust_no",
    "Branch",
    "Pay_by",
    "Cin_Pay_Note",
];

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
        CHECKIN_H_PROJECTION,
        // Header table — at most one row, no ORDER BY required.
        None,
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
        // Track B2 / T2 CRIT-1 — deposit columns mirrored into
        // `ht_checkin_rooms.cr_dep_*` by `sync::mappers::checkin::
        // project_rooms`. Legacy column names per the authoritative
        // schema dump (`docs/legacy-spike/schema/01-baseline-schema.txt`
        // lines 215, 222, 225, 226) and `writeback::fingerprint`
        // SCHEMA_COLUMNS lines 192/199/202/203:
        //   `Cin_Room_Dep`        (float,    col 8)
        //   `Cin_Dep_Status`      (varchar,  col 15)
        //   `Cin_Dep_return_date` (datetime, col 18)
        //   `Cin_Dep_return_by`   (varchar,  col 19)
        // PROD-CRIT: an earlier revision of this projection used the
        // names `Cin_dep` / `Cin_dep_status` / `Cin_dep_returned` /
        // `Cin_dep_returned_by`, none of which exist on the HF Hotel
        // legacy schema. tiberius returned `Invalid column name` and
        // every check-in CT row silently dropped on the floor (the
        // watcher tolerates row-shape errors and advances watermark).
        // Locked by `tests::checkin_ds_projection_names_match_legacy_schema`.
        CHECKIN_DS_PROJECTION,
        // Track B2 / T2 HIGH-1 (audit 2026-05-13) — deterministic
        // iteration order. Without this, `first_room_no` flipped
        // non-deterministically across CT ticks; the room-set diff in
        // `apply_checkin_aggregate` relied on iteration shape too. `id`
        // is the IDENTITY PK and matches insert order.
        Some("id ASC"),
    )
    .await?;

    let payments = fetch_rows(
        mssql,
        "HT_CheckIn_Pay",
        "Cin_No",
        cin_no,
        CHECKIN_PAY_PROJECTION,
        // Stable iteration for payment aggregation (Track B2 / T2 HIGH-1).
        Some("id ASC"),
    )
    .await?;

    Ok(CheckInAggregate {
        header,
        rooms,
        payments,
    })
}

/// Generic single-table read by `<col> = <val>`, optionally with an
/// `ORDER BY` clause.
///
/// Returns the rows materialised through the same `MappableRow` impl
/// the rest of the sync layer uses, so consumers (mappers, tests) only
/// know one row shape.
///
/// `order_by` accepts an SQL fragment (e.g. `"id ASC"`) appended after
/// `WHERE`. Track B2 / T2 HIGH-1 (audit 2026-05-13) — without a stable
/// ORDER BY the `HT_CheckIn_Ds` walk yielded non-deterministic
/// `first_room_no`, which in turn caused `existing_matches` to
/// idempotency-skip on pure room re-orderings. Callers should always
/// pin order on multi-row child tables.
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
    order_by: Option<&'static str>,
) -> Result<Vec<HashMapRow>, SyncError> {
    let mut conn = mssql.get().await?;

    // SQL-quote the WHERE value (single-quote doubling). This is the
    // same boundary as the writeback dispatcher uses; we cannot reach
    // for a parameterised tiberius query without losing the streaming
    // shape that `simple_query` provides.
    let where_q = sql_quote_inline(where_val);
    let select_list = projection.join(", ");
    let order_clause = match order_by {
        Some(clause) => format!(" ORDER BY {clause}"),
        None => String::new(),
    };
    let sql = format!(
        "SELECT {select_list} FROM {table} WHERE {where_col} = {where_q}{order_clause}"
    );

    // R2 (2026-05-14): wrap in per-op read timeout so a row-locked
    // legacy table (e.g. `HT_Book_H` during an iHOTEL cancel) can't
    // freeze the parent-aggregate fetch indefinitely. The CT watcher
    // re-polls on the next tick, so a transient timeout costs at
    // most one tick of latency.
    let raw_rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;

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

    /// PROD-CRIT regression guard. Every column the
    /// `HT_CheckIn_Ds` projection asks tiberius to materialise MUST
    /// exist on the HF Hotel legacy schema; otherwise the SELECT
    /// aborts with `Invalid column name`, the watcher tolerates the
    /// error, advances watermark, and silently drops every check-in
    /// CT row.
    ///
    /// The whitelist below is the verbatim column list from
    /// `docs/legacy-spike/schema/01-baseline-schema.txt` rows 208-226
    /// (table `HT_CheckIn_Ds`, columns 1-19) — the authoritative
    /// `INFORMATION_SCHEMA.COLUMNS` dump captured against the live
    /// HF Hotel database during the legacy spike. Any new entry in
    /// `CHECKIN_DS_PROJECTION` that is NOT in the whitelist fails
    /// this test at compile-time-of-test-run (well before reaching
    /// production).
    ///
    /// To intentionally widen the projection: add the column to the
    /// whitelist AFTER verifying via `01-baseline-schema.txt` (or a
    /// fresh `INFORMATION_SCHEMA.COLUMNS` query against legacy
    /// MSSQL) that the column truly exists.
    #[test]
    fn checkin_ds_projection_names_match_legacy_schema() {
        let legacy_columns: &[&str] = &[
            "id",
            "Cin_No",
            "Cin_Room_No",
            "Cin_Room_Type",
            "Cin_Room_In",
            "Cin_Room_Out",
            "Cin_Room_Status",
            "Cin_Room_Dep",
            "Cin_Room_Price",
            "Cin_Room_Night",
            "Cin_Room_PriceToTal",
            "Cin_Room_Pay_Before",
            "Cin_Room_Pay_Total",
            "Cin_note",
            "Cin_Dep_Status",
            "Dep_by",
            "Cin_cupon",
            "Cin_Dep_return_date",
            "Cin_Dep_return_by",
        ];
        for col in CHECKIN_DS_PROJECTION {
            assert!(
                legacy_columns.contains(col),
                "projection column `{col}` is not on the HF Hotel \
                 legacy `HT_CheckIn_Ds` schema — see \
                 docs/legacy-spike/schema/01-baseline-schema.txt rows \
                 208-226. Adding a nonexistent column makes the entire \
                 SELECT fail with `Invalid column name` and silently \
                 drops every check-in CT row."
            );
        }
    }

    /// The four deposit columns added by Track B2 must be present in
    /// the projection — if anyone removes them the canonical
    /// `ht_checkin_rooms.cr_dep_*` columns stop updating. Pairs with
    /// the schema-name lock above so the projection can be neither
    /// "renamed to a bogus name" nor "silently shrunk".
    #[test]
    fn checkin_ds_projection_carries_deposit_columns() {
        for required in [
            "Cin_Room_Dep",
            "Cin_Dep_Status",
            "Cin_Dep_return_date",
            "Cin_Dep_return_by",
        ] {
            assert!(
                CHECKIN_DS_PROJECTION.contains(&required),
                "deposit column `{required}` missing from \
                 CHECKIN_DS_PROJECTION — Track B2 / T2 CRIT-1 \
                 `ht_checkin_rooms.cr_dep_*` mapping will break."
            );
        }
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards for the remaining parent_loader
    // projections. The existing `CHECKIN_DS_PROJECTION` lock test above
    // is the hand-rolled prototype this batch generalises; we leave it
    // as-is so its PROD-CRIT post-mortem documentation stays attached
    // to the exact projection it guards.
    // -------------------------------------------------------------------

    #[test]
    fn book_h_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(BOOK_H_PROJECTION, "HT_Book_H");
    }

    #[test]
    fn book_ds_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(BOOK_DS_PROJECTION, "HT_Book_Ds");
    }

    #[test]
    fn book_date_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(BOOK_DATE_PROJECTION, "HT_Book_Date");
    }

    #[test]
    fn checkin_h_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(CHECKIN_H_PROJECTION, "HT_CheckIn_H");
    }

    #[test]
    fn checkin_pay_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(CHECKIN_PAY_PROJECTION, "HT_CheckIn_Pay");
    }
}
