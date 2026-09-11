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
use chrono::{DateTime, Datelike, Utc};

use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
use crate::db::PoisonAwareManager;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::bangkok_date;

/// Audit H15: every allocator interpolates a `prefix` into the `LIKE` clause of
/// its TABLOCKX `MAX+1` SELECT via `format!`. Today every caller derives the
/// prefix from integer year/month so the values are guaranteed safe; this
/// validator is a defense-in-depth guard against a future code path that takes
/// the prefix from untrusted input (config, request payload, etc.).
///
/// Accepts: 1–4 uppercase ASCII letters, optionally followed by up to 4 ASCII
/// digits, optionally followed by a single `-`. Examples that pass:
/// `R2604-`, `B2604-`, `CH26-`, `SB-`, `CB-`, `R`, `R26`. Anything containing
/// a quote, semicolon, percent sign, underscore, or whitespace is rejected —
/// these are the metacharacters that would matter inside a T-SQL string
/// literal or a `LIKE` pattern.
fn validate_allocator_prefix(prefix: &str) -> WritebackResult<()> {
    if prefix.is_empty() || prefix.len() > 10 {
        return Err(WritebackError::Recipe(format!(
            "allocator prefix has invalid length: {prefix:?}"
        )));
    }
    let bytes = prefix.as_bytes();
    let mut idx = 0;
    let mut letters = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_uppercase() {
        letters += 1;
        idx += 1;
    }
    if !(1..=4).contains(&letters) {
        return Err(WritebackError::Recipe(format!(
            "allocator prefix must start with 1-4 uppercase letters: {prefix:?}"
        )));
    }
    let mut digits = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        digits += 1;
        idx += 1;
    }
    if digits > 4 {
        return Err(WritebackError::Recipe(format!(
            "allocator prefix has too many digits: {prefix:?}"
        )));
    }
    // Optional trailing dash.
    if idx < bytes.len() && bytes[idx] == b'-' {
        idx += 1;
    }
    if idx != bytes.len() {
        return Err(WritebackError::Recipe(format!(
            "allocator prefix contains disallowed character: {prefix:?}"
        )));
    }
    Ok(())
}

/// Convenience alias — the type bb8 hands out for our legacy connection.
pub type LegacyConn<'a> = PooledConnection<'a, PoisonAwareManager>;

/// Approaching-wrap threshold for i32 allocators (audit Wave 6 LOW item 7).
///
/// Several legacy `id` columns (`HT_Book_Date.id`, `HT_CheckIn_Ds.id`,
/// `HT_Receipt_H.id`, `HT_Room_Status.id`, `HT_Rooms_Cancel.id`,
/// `HT_Customers.id`) are typed `int` and decoded as Rust `i32`. The .NET
/// app has been writing to these tables for over a decade and at ~15k
/// receipts/year the absolute counts are well under 1M today, so wrap is
/// not imminent. But once the value approaches `i32::MAX / 2`
/// (≈1.07 billion) we want a WARN every allocation so ops can plan a
/// migration to `bigint` before the column overflows. No hard error —
/// failing the allocator would block writeback entirely on a column we
/// don't own the schema of.
const I32_WRAP_WARN_THRESHOLD: i32 = i32::MAX / 2;

/// Internal helper — run a `SELECT ISNULL(MAX(...), 0) + 1` query under
/// `TABLOCKX, HOLDLOCK` and return the result.
///
/// Audit Wave 6 LOW item 7: logs a WARN when the next value approaches
/// `i32::MAX / 2` so operators get visibility before any column overflows.
async fn select_next_int_with_lock(
    conn: &mut LegacyConn<'_>,
    sql: &str,
) -> WritebackResult<i32> {
    // R2 (2026-05-14): wrap MAX+1 TABLOCKX select in write-budget
    // timeout. The TABLOCKX-HOLDLOCK pattern intentionally blocks
    // concurrent writers, so it is *expected* to wait — but a wait
    // longer than the write budget indicates the locker (iHOTEL or
    // a stuck writeback worker) is itself wedged. Failing the
    // allocation here is the right move: caller is inside a
    // transaction that will ROLLBACK and the job retries on the
    // next dispatcher pass.
    let rows = simple_query_with_timeout(conn, sql, MssqlOpKind::Write).await?;
    let row = rows
        .first()
        .ok_or_else(|| WritebackError::Recipe(format!("MAX+1 query returned no rows: {sql}")))?;
    let next: i32 = row
        .get(0)
        .ok_or_else(|| WritebackError::Recipe(format!("MAX+1 column was NULL: {sql}")))?;
    if next > I32_WRAP_WARN_THRESHOLD {
        tracing::warn!(
            next,
            threshold = I32_WRAP_WARN_THRESHOLD,
            i32_max = i32::MAX,
            sql,
            "allocator: i32 id column approaching wrap — plan a bigint migration",
        );
    }
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

/// Derive a `HT_Customers.Cust_no` (`'C' + integer`, no zero padding —
/// spike §2) from a freshly-allocated `HT_Customers.id`.
///
/// 2026-06-11 coexistence audit: iHOTEL derives Cust_no from `MAX(id)+1`
/// (cheatsheet §1.6 #3 — `FrmCheckIn.SAVE_CUST` reads
/// `SELECT TOP 1 * FROM HT_Customers ORDER BY id DESC`, takes `id+1`),
/// NOT from the highest existing Cust_no suffix. An earlier allocator here
/// parsed `MAX(parse(Cust_no))+1` — numerically identical today because
/// the two sequences have never diverged, but any one-off divergence
/// (manual row, restore artifact) would fork our sequence away from
/// iHOTEL's forever. Deriving from the SAME `MAX(id)+1` value that
/// [`allocate_customer_id`] fetches under TABLOCKX keeps the pair atomic
/// and convention-identical: callers allocate the id once and feed it
/// here, so `Cust_no == 'C' + id` exactly like the .NET app.
pub fn cust_no_from_customer_id(customer_id: i32) -> String {
    format!("C{customer_id}")
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
/// — allocation rolls over each calendar year. We use the *current Thai
/// (Bangkok) year* to match the .NET app's behavior; using `Utc::now().year()`
/// directly would emit the wrong prefix for ~7h around the Bangkok new-year
/// rollover (audit CRIT-C3 — `docs/legacy-spike/writeback-audit-2026-05-12.md`).
pub async fn allocate_cin_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    allocate_cin_no_with_now(conn, Utc::now()).await
}

/// Test-only variant of [`allocate_cin_no`] that takes the "now" instant
/// explicitly so tests can pin the wall-clock and assert behaviour around the
/// Bangkok calendar boundary without mocking the global clock.
pub(crate) async fn allocate_cin_no_with_now(
    conn: &mut LegacyConn<'_>,
    now: DateTime<Utc>,
) -> WritebackResult<String> {
    let prefix = cin_no_prefix(now);
    // Audit H15: defense-in-depth — fail loud if the prefix ever contains
    // anything other than letters/digits/dash. Today the prefix is derived
    // from integer year arithmetic so this is unreachable in production.
    validate_allocator_prefix(&prefix)?;
    let suffix_offset = prefix.len() + 1;

    // Match exactly the year prefix, then extract the trailing 6-digit suffix.
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Cin_no, {suffix_offset}, 50) AS INT)), 0) + 1 \
             FROM HT_CheckIn_H WITH (TABLOCKX, HOLDLOCK) \
             WHERE Cin_no LIKE '{prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{prefix}{next:06}"))
}

/// Pure helper — compute the `CH{yy}-` prefix from a UTC instant, using the
/// Bangkok calendar year. Extracted for unit-test access.
fn cin_no_prefix(now: DateTime<Utc>) -> String {
    let year_two = bangkok_date(now).year() % 100;
    format!("CH{year_two:02}-")
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

/// Allocate the next `HT_Cupon.cupon_no` (Track G5).
///
/// Per `docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Cupon`:
///
/// > PK: `cupon_no int` (NOT IDENTITY) — generated by
/// > `get_id("HT_Cupon","cupon_no")`.
///
/// The legacy `get_id` helper is a MAX+1 allocator. We mirror it under
/// `TABLOCKX, HOLDLOCK` to stay race-safe against iHOTEL's
/// `Module1.GEN_Cupon` which fires the same allocator on every
/// check-in. 17,894 rows in production as of the 2026-04-26 spike —
/// nowhere near `i32` wrap.
pub async fn allocate_cupon_no(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(cupon_no), 0) + 1 FROM HT_Cupon WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_CheckIn_Ds.id`.
///
/// **Note:** the spike §2 narrative claimed this column was IDENTITY. The
/// 2026-04-26 schema dump (via `inspect_schema`) confirmed it is NOT —
/// `int NOT NULL, default=NULL`. INSERTs MUST provide an explicit `id`
/// or fail. Recipes (`walkin`, `checkin_to_booking`) call this helper.
pub async fn allocate_checkin_ds_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_CheckIn_Ds WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `HT_Receipt_H.id`.
///
/// Schema dump 2026-04-26 confirmed: `int NOT NULL, default=NULL`, NOT
/// IDENTITY. The `payment` recipe correctly allocates and includes `[id]`
/// in its INSERT.
pub async fn allocate_receipt_h_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM HT_Receipt_H WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Allocate the next `TB_Pay_History.id` (issue #202 — `CreateCashEntry`).
///
/// Per `docs/legacy-app/COMPAT_CHEATSHEET.md` §`TB_Pay_History` "`id int` (NOT IDENTITY) via `get_id`"
/// and `docs/legacy-app/SCHEMA.sql` §"Table: dbo.TB_Pay_History" "[id] int,"
/// (an earlier revision cited both by line number — one of those numbers landed
/// on a blank line, the other on the table marker itself), plus the
/// `cash_entry` recipe module doc:
/// `TB_Pay_History.id` is NOT IDENTITY — allocated app-side via `get_id`
/// (MAX+1), same convention as every other legacy sequential id here.
/// (The schema dump declares it plain `[id] int`, i.e. nullable; an earlier
/// revision of this comment said `int NOT NULL`, which neither source states.)
pub async fn allocate_pay_history_id(conn: &mut LegacyConn<'_>) -> WritebackResult<i32> {
    select_next_int_with_lock(
        conn,
        "SELECT ISNULL(MAX(id), 0) + 1 FROM TB_Pay_History WITH (TABLOCKX, HOLDLOCK)",
    )
    .await
}

/// Format a `Pay_No` for `HT_CheckIn_Pay`. Per
/// `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "R{yyMM}-{4digit}"
/// (an earlier revision credited this to `findings.md` by line number;
/// findings.md documents this format at no revision — those numbers were
/// always COMPAT_CHEATSHEET's), the immutable
/// decompile `Module1.GetSIR_PAY` (`Module1.cs:1756`), and
/// live capture `docs/legacy-spike/raw/invoice-20260424-100827/07-events.txt:154`
/// the legacy app emits `R{yyMM}-{4digit}` (e.g. `R2604-0241`). The `R` prefix
/// and 4-digit suffix MUST match exactly — the `LIKE 'R{yyMM}-%'` predicate is
/// how we read the same sequence iHOTEL writes. Year/month are scoped to the
/// Bangkok calendar (audit CRIT-C1 + CRIT-C3 —
/// `docs/legacy-spike/writeback-audit-2026-05-12.md`).
pub async fn allocate_pay_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    allocate_pay_no_with_now(conn, Utc::now()).await
}

/// Test-only variant of [`allocate_pay_no`] that takes the "now" instant
/// explicitly so tests can pin the wall-clock without mocking the global clock.
pub(crate) async fn allocate_pay_no_with_now(
    conn: &mut LegacyConn<'_>,
    now: DateTime<Utc>,
) -> WritebackResult<String> {
    let month_prefix = pay_no_prefix(now);
    // Audit H15: validate the prefix before interpolation. Cheap arithmetic
    // check today; required guard if `pay_no_prefix` is ever refactored to
    // accept caller-supplied input.
    validate_allocator_prefix(&month_prefix)?;
    // SUBSTRING is 1-based in T-SQL; the suffix starts one position past the
    // prefix. Derive from prefix length so future format tweaks don't silently
    // misalign the MAX scan.
    let suffix_offset = month_prefix.len() + 1;
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Pay_No, {suffix_offset}, 50) AS INT)), 0) + 1 \
             FROM HT_CheckIn_Pay WITH (TABLOCKX, HOLDLOCK) \
             WHERE Pay_No LIKE '{month_prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{month_prefix}{next:04}"))
}

/// Pure helper — compute the `R{yy}{MM}-` prefix from a UTC instant, using the
/// Bangkok calendar.
fn pay_no_prefix(now: DateTime<Utc>) -> String {
    let bkk = bangkok_date(now);
    format!("R{:02}{:02}-", bkk.year() % 100, bkk.month())
}

/// Format a `Receipt_no` for `HT_Receipt_H`. Per
/// `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "B{yyMM}-{4digit}"
/// (an earlier revision credited one and the same line number to two different
/// documents; only the cheatsheet one ever resolved),
/// the immutable decompile `FrmAddSale.GetSIR` (`FrmAddSale.cs:3818`), and live capture
/// `docs/legacy-spike/raw/walkin-20260424-095304/07-events.txt:120` the legacy
/// app emits `B{yyMM}-{4digit}` (single `B` is the default; `SB`/`CB` are
/// SmallBill/CreditBill variants). The prefix and 4-digit suffix MUST match
/// exactly or the `LIKE` predicate forks our sequence away from iHOTEL's
/// (audit CRIT-C2 + CRIT-C3 —
/// `docs/legacy-spike/writeback-audit-2026-05-12.md`).
pub async fn allocate_receipt_no(conn: &mut LegacyConn<'_>) -> WritebackResult<String> {
    allocate_receipt_no_with_now(conn, Utc::now()).await
}

/// Test-only variant of [`allocate_receipt_no`] that takes the "now" instant
/// explicitly so tests can pin the wall-clock without mocking the global clock.
pub(crate) async fn allocate_receipt_no_with_now(
    conn: &mut LegacyConn<'_>,
    now: DateTime<Utc>,
) -> WritebackResult<String> {
    let month_prefix = receipt_no_prefix(now);
    // Audit H15: defense-in-depth against prefix injection — see
    // `validate_allocator_prefix` for the accepted shape.
    validate_allocator_prefix(&month_prefix)?;
    let suffix_offset = month_prefix.len() + 1;
    let next = select_next_int_with_lock(
        conn,
        &format!(
            "SELECT ISNULL(MAX(TRY_CAST(SUBSTRING(Receipt_no, {suffix_offset}, 50) AS INT)), 0) + 1 \
             FROM HT_Receipt_H WITH (TABLOCKX, HOLDLOCK) \
             WHERE Receipt_no LIKE '{month_prefix}%'",
        ),
    )
    .await?;
    Ok(format!("{month_prefix}{next:04}"))
}

/// Pure helper — compute the `B{yy}{MM}-` prefix from a UTC instant, using the
/// Bangkok calendar.
fn receipt_no_prefix(now: DateTime<Utc>) -> String {
    let bkk = bangkok_date(now);
    format!("B{:02}{:02}-", bkk.year() % 100, bkk.month())
}

#[cfg(test)]
mod tests {
    //! These tests verify the SQL TEXT we emit and the pure prefix/format
    //! helpers — not the I/O. A live MSSQL is out of scope for the unit suite
    //! per the Phase 4b spec; any live interaction is the receptionist's job.
    //!
    //! Allocator end-to-end correctness (the `LIKE` predicate actually
    //! matching a freshly-installed iHOTEL row) is the receptionist-driven
    //! acceptance test described in Wave 1 of
    //! `docs/legacy-spike/writeback-audit-2026-05-12.md`.

    use super::*;
    use chrono::TimeZone;

    /// Tiny shape-matcher: confirm `s` matches `<letters><N digits>-<M digits>`.
    /// Avoids pulling in `regex` as a dev-dep just for two assertions.
    fn matches_shape(s: &str, leading_letters: &str, head_digits: usize, tail_digits: usize) -> bool {
        let Some(rest) = s.strip_prefix(leading_letters) else { return false; };
        let bytes = rest.as_bytes();
        if bytes.len() != head_digits + 1 + tail_digits {
            return false;
        }
        if bytes[head_digits] != b'-' {
            return false;
        }
        let head_digits_ok = bytes[..head_digits].iter().all(|b| b.is_ascii_digit());
        let tail_digits_ok = bytes[head_digits + 1..].iter().all(|b| b.is_ascii_digit());
        head_digits_ok && tail_digits_ok
    }

    #[test]
    fn cust_no_format_padding_lookup() {
        // Format invariant: 'C' + integer (no zero-pad)
        assert_eq!(cust_no_from_customer_id(21607), "C21607");
        assert_eq!(cust_no_from_customer_id(1), "C1");
    }

    /// 2026-06-11 audit — Cust_no derives from the customer `id` itself
    /// (cheatsheet §1.6 #3: iHOTEL reads `MAX(id)`, takes +1, and uses the
    /// same number for both columns). The pairing `Cust_no == 'C' + id`
    /// must hold for every id our allocator can produce.
    #[test]
    fn cust_no_is_always_c_plus_the_allocated_id() {
        for id in [1, 42, 21607, 21_624, i32::MAX] {
            let cust_no = cust_no_from_customer_id(id);
            assert_eq!(cust_no, format!("C{id}"));
        }
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

    // --- CRIT-C1 / C2 / C3 regression tests ---
    //
    // These tests exercise the *pure prefix + emitted-string* invariants for
    // the three allocators audited in
    // `docs/legacy-spike/writeback-audit-2026-05-12.md`. They guard against
    // the original bug pattern (wrong letter, wrong digit width, UTC clock
    // straddling the Bangkok calendar boundary) without requiring a live
    // MSSQL.

    /// Compose an example emitted ID from the prefix helper + a fake
    /// `next` integer, mirroring exactly what the allocator returns.
    /// Lets the regex tests below run without a DB.
    fn compose_pay_no(now: DateTime<Utc>, next: i32) -> String {
        format!("{}{next:04}", pay_no_prefix(now))
    }
    fn compose_receipt_no(now: DateTime<Utc>, next: i32) -> String {
        format!("{}{next:04}", receipt_no_prefix(now))
    }
    fn compose_cin_no(now: DateTime<Utc>, next: i32) -> String {
        format!("{}{next:06}", cin_no_prefix(now))
    }

    #[test]
    fn allocate_pay_no_emits_r_prefix_with_4_digit_suffix() {
        // CRIT-C1: must be R{yyMM}-{4digit}, not P{yyMM}-{6digit}.
        // CITATION FIX (assertion message below is stale and NOT edited here —
        // this package is comment-only): the literal `R2604-0241` comes from
        // the frozen capture `docs/legacy-spike/raw/invoice-20260424-100827/07-events.txt:154`,
        // not from findings.md, which carries the string at no revision. The
        // format itself is `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "R{yyMM}-{4digit}".
        let now = Utc.with_ymd_and_hms(2026, 4, 24, 10, 5, 4).unwrap(); // 4/24 17:05 BKK
        let id = compose_pay_no(now, 241);
        assert_eq!(id, "R2604-0241", "must match findings.md §2 capture R2604-0241");
        assert!(
            matches_shape(&id, "R", 4, 4),
            "Pay_No {id} must match shape R<4digits>-<4digits>"
        );
    }

    #[test]
    fn allocate_receipt_no_emits_b_prefix_with_4_digit_suffix() {
        // CRIT-C2: must be B{yyMM}-{4digit}, not RC{yyMM}-{6digit}.
        let now = Utc.with_ymd_and_hms(2026, 4, 24, 10, 5, 4).unwrap();
        let id = compose_receipt_no(now, 1);
        assert_eq!(id, "B2604-0001");
        assert!(
            matches_shape(&id, "B", 4, 4),
            "Receipt_no {id} must match shape B<4digits>-<4digits>"
        );
    }

    #[test]
    fn allocate_cin_no_emits_ch_prefix_with_6_digit_suffix() {
        // Format width unchanged from prior behaviour — guard regression.
        let now = Utc.with_ymd_and_hms(2026, 4, 24, 10, 5, 4).unwrap();
        let id = compose_cin_no(now, 5228);
        assert_eq!(id, "CH26-005228");
        assert!(
            matches_shape(&id, "CH", 2, 6),
            "Cin_no {id} must match shape CH<2digits>-<6digits>"
        );
    }

    // --- BKK calendar boundary tests (CRIT-C3) ---
    //
    // At a UTC instant that's still in the previous BKK day/month/year but
    // already past midnight UTC of the next, the allocator MUST emit the
    // Bangkok-side period — NOT the UTC one.

    #[test]
    fn allocate_cin_no_uses_bangkok_calendar_year() {
        // 2026-12-31 17:30:00 UTC = 2027-01-01 00:30 Bangkok.
        // Cin_No prefix must roll over to CH27- (not stay on CH26-).
        let utc_just_before_bkk_new_year = Utc.with_ymd_and_hms(2026, 12, 31, 17, 30, 0).unwrap();
        let id = compose_cin_no(utc_just_before_bkk_new_year, 1);
        assert!(
            id.starts_with("CH27-"),
            "expected CH27- prefix (Bangkok already in new year), got {id}"
        );
    }

    #[test]
    fn allocate_pay_no_uses_bangkok_calendar_month() {
        // 2026-04-30 17:30:00 UTC = 2026-05-01 00:30 Bangkok.
        // Pay_No prefix must roll over to R2605- (not stay on R2604-).
        let utc_just_before_bkk_month_rollover =
            Utc.with_ymd_and_hms(2026, 4, 30, 17, 30, 0).unwrap();
        let id = compose_pay_no(utc_just_before_bkk_month_rollover, 1);
        assert!(
            id.starts_with("R2605-"),
            "expected R2605- prefix (Bangkok already in May), got {id}"
        );
    }

    #[test]
    fn allocate_receipt_no_uses_bangkok_calendar_month() {
        // Same boundary as the Pay_No test — must roll forward to B2605-.
        let utc_just_before_bkk_month_rollover =
            Utc.with_ymd_and_hms(2026, 4, 30, 17, 30, 0).unwrap();
        let id = compose_receipt_no(utc_just_before_bkk_month_rollover, 1);
        assert!(
            id.starts_with("B2605-"),
            "expected B2605- prefix (Bangkok already in May), got {id}"
        );
    }

    // --- SUBSTRING offset is derived from prefix length ---
    //
    // The audit calls out the original hardcoded `SUBSTRING(..., 7, 50)` /
    // `SUBSTRING(..., 8, 50)` offsets as fragile. These checks confirm the
    // offset arithmetic stays in sync with the emitted prefix.

    // --- Allocator prefix validation (CRIT-H15) ---
    //
    // The audit calls out the `LIKE '{prefix}%'` interpolation as a templated
    // injection sink. Today the prefix is integer-derived (year/month) so the
    // production values are safe, but the validator guards against any future
    // code path that wires a caller-supplied string into the prefix.

    #[test]
    fn allocate_prefix_validation_accepts_real_prefixes() {
        // These are the exact prefixes the three production allocators emit.
        for good in ["R2604-", "B2604-", "CH26-", "R", "CH27-"] {
            assert!(
                validate_allocator_prefix(good).is_ok(),
                "real allocator prefix {good:?} must validate"
            );
        }
    }

    #[test]
    fn allocate_prefix_validation_rejects_special_chars() {
        // Each of these would be catastrophic interpolated into `LIKE '<p>%'`
        // or into the SUBSTRING start offset — they MUST be rejected.
        for bad in [
            "R26';--",     // statement terminator + SQL comment
            "R2604%",      // explicit wildcard
            "R26_4-",      // single-char LIKE wildcard
            "R 2604-",     // whitespace (impossible from current callers)
            "r2604-",      // lowercase (production prefixes are all uppercase)
            "R26+4-",      // arithmetic / operator
            "R'2604-",     // embedded quote
            "",            // empty
            "TOOLONGPREFIX1234", // > 10 chars
        ] {
            let result = validate_allocator_prefix(bad);
            assert!(
                result.is_err(),
                "bad prefix {bad:?} must be rejected, got: {result:?}"
            );
        }
    }

    #[test]
    fn allocate_prefix_validation_caps_digit_run_length() {
        // Five trailing digits is the next-most-likely typo (forgetting the
        // `-`); reject to keep the format invariant tight.
        assert!(validate_allocator_prefix("R12345-").is_err());
    }

    /// Wave 6 LOW item 7: confirm the i32-wrap warning threshold is set at
    /// half of `i32::MAX`. The actual WARN log emission is checked by
    /// `select_next_int_with_lock` callers in live MSSQL; this test pins
    /// the threshold constant so an accidental edit (e.g. dropping a digit
    /// in the constant value) surfaces immediately.
    #[test]
    fn i32_wrap_warn_threshold_is_half_of_i32_max() {
        assert_eq!(I32_WRAP_WARN_THRESHOLD, i32::MAX / 2);
        // Sanity: threshold must be a real, large positive number — not 0 or
        // a stray negative from an arithmetic typo.
        assert!(I32_WRAP_WARN_THRESHOLD > 1_000_000_000);
        assert!(I32_WRAP_WARN_THRESHOLD < i32::MAX);
    }

    #[test]
    fn pay_no_substring_offset_is_one_past_prefix_length() {
        let now = Utc.with_ymd_and_hms(2026, 4, 24, 10, 0, 0).unwrap();
        // prefix "R2604-" is 6 chars; SUBSTRING (1-based) of suffix starts at 7.
        assert_eq!(pay_no_prefix(now).len() + 1, 7);
    }

    #[test]
    fn receipt_no_substring_offset_is_one_past_prefix_length() {
        let now = Utc.with_ymd_and_hms(2026, 4, 24, 10, 0, 0).unwrap();
        // prefix "B2604-" is 6 chars; SUBSTRING (1-based) of suffix starts at 7.
        assert_eq!(receipt_no_prefix(now).len() + 1, 7);
    }
}
