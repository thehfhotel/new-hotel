//! `ModifyBooking` recipe — spike `findings.md` §3c.
//!
//! **Critical departure from the legacy app.** Spike §3c documents that the
//! .NET app does a destructive DELETE-everything + RE-INSERT pattern (no
//! transaction). Per spike §3c recommendation:
//!
//! > **For our writeback**: we should **NOT** replicate this destructive
//! > pattern. Use targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` instead,
//! > and add/remove `HT_Book_Date` rows as needed. The legacy app reads what's
//! > in the tables, so our targeted UPDATEs are equivalent and safer.
//!
//! ## What we DO emit (safe, targeted)
//!
//! - `UPDATE HT_Book_H SET …` — only the changed fields (dates, customer info,
//!   notes). Keeps `book_room_type=2`, `Book_Notify_Day=3`, etc. untouched
//!   (they're set at create time per §3k).
//! - `UPDATE HT_Book_Ds SET …` — when stay range or price changed.
//! - `DELETE FROM HT_Book_Date WHERE Book_no=… AND Book_date_ds NOT IN (…)`
//!   — drop nights that no longer apply.
//! - `INSERT INTO HT_Book_Date …` — add new nights that didn't exist before.
//!
//! ## What we DO NOT emit (the legacy app's destructive pattern)
//!
//! - `DELETE FROM HT_Book_H WHERE Book_ID=…` — would lose the booking ID and
//!   the customer linkage if the subsequent INSERT failed.
//! - `DELETE FROM HT_Book_Ds WHERE Book_no=…` — same risk.
//! - Mass DELETE+REINSERT of `HT_Book_Date` — only diff what changed.
//!
//! Reference SQL (the destructive legacy pattern, for context only —
//! `booking-checkin-20260424-101838/writes.txt` lines 5-13):
//!
//! ```text
//! UPDATE HT_Customers SET ... WHERE Cust_no='C21610'                 -- preserved
//! UPDATE HT_Rooms SET room_book_*='', Room_Book='' WHERE room_book IN
//!     (SELECT id FROM ht_book_date WHERE Book_no='R014810')          -- preserved
//! DELETE FROM HT_Book_Date WHERE Book_no='R014810'                   -- ❌ skipped
//! DELETE FROM HT_Book_H    WHERE Book_ID='R014810'                   -- ❌ skipped
//! DELETE FROM HT_Book_Ds   WHERE Book_no='R014810'                   -- ❌ skipped
//! [INSERT all 3 tables again with the new dates]                     -- ❌ skipped
//! ```
//!
//! Our targeted approach achieves the same end state without the data-loss
//! window the legacy app's no-transaction implementation has (§3c warning).

use super::helpers::build_customer_resave_update;
use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
use crate::outbox::intent::BookingChanges;
use crate::writeback::allocate::{allocate_book_date_id, LegacyConn};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    end_of_stay_at_almost_noon, enumerate_calendar_nights, format_legacy_date,
    format_legacy_datetime, sql_quote,
};
use chrono::NaiveDate;

/// Inputs for the modify-booking recipe — already-resolved legacy IDs +
/// the diff from the `BookingChanges` payload.
#[derive(Debug, Clone)]
pub struct ModifyBookingInputs<'a> {
    pub book_id: &'a str,
    /// Caller pre-allocates the FIRST id to use for any new HT_Book_Date INSERTs.
    /// If no nights are added, this is unused.
    pub book_date_id_base: i32,
    pub changes: &'a BookingChanges,
    /// Calendar nights the *new* stay range covers (after applying the change).
    /// Empty if `changes.new_stay` is None — we don't touch HT_Book_Date.
    pub new_nights_calendar: Vec<NaiveDate>,
    /// Room number on the booking *before* this modification. Used as a
    /// fallback when the user extends the stay without moving rooms — every
    /// new HT_Book_Date row needs `Book_type` populated with a room number,
    /// otherwise the booking disappears from the .NET app's calendar grid
    /// (audit HIGH-1). Resolved by `execute()` from HT_Book_Date.
    pub existing_room_no: Option<&'a str>,
    /// Customer name on the booking *before* this modification — used as a
    /// fallback to render the calendar caption when only dates change (H8).
    /// Resolved by `execute()` from HT_Book_H.
    pub existing_customer_name: Option<&'a str>,
    /// Stay range (`Book_Room_Start`, `Book_Room_End`) on the booking
    /// *before* this modification — Bangkok wall-clock naive datetimes as
    /// stored in `HT_Book_Ds`. Used to render the calendar-caption rewrite
    /// when the payload doesn't change the stay (2026-06-11 audit P1-10a:
    /// step 0b clears the caption on EVERY modify, so the rewrite must
    /// also fire on every modify). Resolved by `execute()` from
    /// HT_Book_Ds when `changes.new_stay` is None.
    pub existing_stay: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
    /// `HT_Book_Ds.Book_Room_Night` on the booking *before* this
    /// modification — used when computing `Book_Room_PriceToTal` for a
    /// price-only modify (H10). Without it the total collapses to
    /// `price * 1` and corrupts multi-night bookings.
    /// Resolved by `execute()` from HT_Book_Ds.
    pub book_room_night_existing: Option<i32>,
}

/// Build the targeted-UPDATE statements. PURE — no I/O.
pub fn build_statements(inputs: &ModifyBookingInputs<'_>) -> Vec<String> {
    let book_id_q = sql_quote(inputs.book_id);
    let mut statements = Vec::new();

    // 0a. Customer re-save — spike §3c capture lines 5,16,28. The .NET app
    //     re-saves the customer record on every booking modify. Without it,
    //     phone/address edits don't propagate to the customer master.
    if let Some(resave) = inputs.changes.customer_resave.as_ref() {
        statements.push(build_customer_resave_update(resave));
    }

    // 0b. Clear stale HT_Rooms display fields BEFORE the date diff —
    //     spike §3c capture lines 6,14,17. Otherwise after a date change the
    //     calendar grid keeps stale "booked" captions.
    statements.push(format!(
        "update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' \
         where room_book in (select id from ht_book_date  where Book_no={book_id_q})"
    ));

    // 1. UPDATE HT_Book_H — set only the changed fields. Builds the SET clause
    //    incrementally; if no header changes, we skip the UPDATE entirely.
    let mut header_sets: Vec<String> = Vec::new();
    if let Some(stay) = &inputs.changes.new_stay {
        // H9: HT_Book_H carries date-only forms (matches `booking_create`
        // and the latest legacy capture — see `booking_create.rs:105-114`
        // and the byte-parity test at `booking_create.rs::book_h_matches_legacy_capture_byte_for_byte`).
        // The earlier midnight-suffix shape (`'4/25/2026 12:00:00 AM'`)
        // diverged from create and broke the .NET app's booking-list view
        // when both code paths fired in the same writeback session.
        let in_q = sql_quote(&format_legacy_date(
            crate::writeback::format::bangkok_date(stay.start),
        ));
        let out_q = sql_quote(&format_legacy_date(
            crate::writeback::format::bangkok_date(stay.end),
        ));
        header_sets.push(format!("[Book_Date_in]={in_q}"));
        header_sets.push(format!("[Book_Date_out]={out_q}"));
    }
    if let Some(price) = &inputs.changes.new_price {
        let baht = (price.as_satang() as f64) / 100.0;
        // Wave 6 LOW item 4: 2dp matches HT_Book_H's create-side formatting.
        header_sets.push(format!("[Book_Price_Total]={baht:.2}"));
    }
    if let Some(phone) = &inputs.changes.new_customer_phone {
        let q = sql_quote(phone);
        header_sets.push(format!("[Book_Cust_Tel]={q}"));
    }
    if let Some(notes) = &inputs.changes.new_notes {
        let q = sql_quote(notes);
        // Wave 5b item 5: write `HT_Book_H.[Book_room_note]` (lowercase r —
        // header level, used by reports) for parity with the existing flow.
        // The matching `HT_Book_Ds.[Book_Room_Note]` (capital R) is pushed
        // into `ds_sets` below — iHOTEL's edit-booking form binds the visible
        // note column to the Ds row (`SCHEMA.sql:6` for HT_Book_Ds,
        // `COMPAT_CHEATSHEET.md:671` for the column name), so without that
        // write a note edit stayed invisible in iHOTEL until the user
        // re-saved.
        header_sets.push(format!("[Book_room_note]={q}"));
    }
    if !header_sets.is_empty() {
        statements.push(format!(
            "UPDATE [HT_Book_H] SET {sets} WHERE Book_ID={book_id_q}",
            sets = header_sets.join(",")
        ));
    }

    // 2. UPDATE HT_Book_Ds — set only the changed detail fields.
    let mut ds_sets: Vec<String> = Vec::new();
    if let Some(stay) = &inputs.changes.new_stay {
        // HT_Book_Ds carries actual stay times — start as picked, end snapped
        // to 11:59:59 AM per spike §3b convention.
        let start_q = sql_quote(&format_legacy_datetime(stay.start));
        let end_actual = end_of_stay_at_almost_noon(stay.end);
        let end_q = sql_quote(&format_legacy_datetime(end_actual));
        let nights = inputs.new_nights_calendar.len().max(1) as i32;
        ds_sets.push(format!("[Book_Room_Start]={start_q}"));
        ds_sets.push(format!("[Book_Room_End]={end_q}"));
        ds_sets.push(format!("[Book_Room_Night]={nights}"));
    }
    if let Some(room_no) = &inputs.changes.new_room_no {
        // Misleading column name: stores the room NUMBER (per spike §3b).
        let q = sql_quote(room_no);
        ds_sets.push(format!("[Book_Room_Type]={q}"));
    }
    if let Some(price) = &inputs.changes.new_price {
        let baht = (price.as_satang() as f64) / 100.0;
        // H10: when `new_stay` is None, the `new_nights_calendar` vec is
        // empty — falling back to `.len().max(1)=1` would corrupt a
        // multi-night booking's total (`Book_Room_PriceToTal` collapses to
        // a single-night value). Prefer the existing night count fetched
        // from MSSQL; fall back to the new calendar length when the stay
        // is genuinely changing.
        let nights = if inputs.changes.new_stay.is_some() {
            inputs.new_nights_calendar.len().max(1) as i32
        } else {
            inputs.book_room_night_existing.unwrap_or(1).max(1)
        };
        // Wave 6 LOW item 4: 2dp matches HT_Book_Ds's create-side formatting.
        ds_sets.push(format!("[Book_Room_Price]={baht:.2}"));
        ds_sets.push(format!(
            "[Book_Room_PriceToTal]={total:.2}",
            total = baht * nights as f64
        ));
    }
    if let Some(notes) = &inputs.changes.new_notes {
        // Wave 5b item 5: also write `HT_Book_Ds.[Book_Room_Note]` (capital R
        // — distinct column from the header-level `Book_room_note`). iHOTEL's
        // edit-booking form binds the visible note input to the Ds row, so
        // without this write a note edit was invisible in iHOTEL until the
        // user re-saved. `SCHEMA.sql` line 6 / `COMPAT_CHEATSHEET.md:671`
        // confirm the column name + casing.
        let q = sql_quote(notes);
        ds_sets.push(format!("[Book_Room_Note]={q}"));
    }
    if !ds_sets.is_empty() {
        statements.push(format!(
            "UPDATE [HT_Book_Ds] SET {sets} WHERE Book_No={book_id_q}",
            sets = ds_sets.join(",")
        ));
    }

    // 3. HT_Book_Date diff — only when stay range changed.
    if inputs.changes.new_stay.is_some() {
        // Drop nights that aren't in the new calendar — Wave 5b item 6:
        // `Book_date_ds` is `datetime`; the kept-list is rendered as date-only
        // strings (`'4/25/2026'`). Casting BOTH sides to DATE guarantees the
        // comparison ignores any stored-time component (the legacy app stores
        // bookings at midnight, but a future row written by a different path
        // could carry a non-midnight time and silently slip past the filter).
        let kept_dates_q = inputs
            .new_nights_calendar
            .iter()
            .map(|d| sql_quote(&format_legacy_date(*d)))
            .map(|q| format!("CAST({q} AS DATE)"))
            .collect::<Vec<_>>()
            .join(",");
        if kept_dates_q.is_empty() {
            statements.push(format!(
                "DELETE FROM HT_Book_Date WHERE Book_no={book_id_q}"
            ));
        } else {
            statements.push(format!(
                "DELETE FROM HT_Book_Date WHERE Book_no={book_id_q} \
                 AND CAST(Book_date_ds AS DATE) NOT IN ({kept_dates_q})"
            ));
        }
        // Re-insert any nights that don't exist yet. We can't know what's in
        // MSSQL without a SELECT, so we use INSERT with NOT EXISTS to stay
        // idempotent.
        // HIGH-1 fix: when only dates change, fall back to the existing
        // room_no resolved from MSSQL. Empty Book_type would make the
        // booking disappear from the .NET app's calendar grid.
        let room_no_for_inserts = inputs
            .changes
            .new_room_no
            .as_deref()
            .or(inputs.existing_room_no)
            .unwrap_or("");
        let room_no_q = sql_quote(room_no_for_inserts);
        for (i, day) in inputs.new_nights_calendar.iter().enumerate() {
            let id = inputs.book_date_id_base + i as i32;
            let date_q = sql_quote(&format_legacy_date(*day));
            statements.push(format!(
                "INSERT INTO [HT_Book_Date]([id],[Book_no],[Book_type],[Book_date_ds],\
                 [Book_Num],[Book_USE]) \
                 SELECT {id},{book_id_q},{room_no_q},{date_q},1,0 \
                 WHERE NOT EXISTS (SELECT 1 FROM HT_Book_Date \
                 WHERE Book_no={book_id_q} AND Book_date_ds={date_q})"
            ));
        }
    }

    // Re-write the HT_Rooms display caption — spike §3c capture line 26.
    // Mirrors `booking_create`'s caption format (commit 0179f81).
    //
    // 2026-06-11 audit P1-10a (extends H8): step 0b clears the caption on
    // EVERY modify, so the rewrite must also fire on every modify — not
    // only when dates change. A notes-only edit on a same-day booking
    // previously blanked the iHOTEL room caption permanently. The stay /
    // customer name / room number fall back to the existing MSSQL values
    // (`existing_*`, resolved by `execute()`) when the payload doesn't
    // carry them.
    {
        let customer_name = inputs
            .changes
            .new_customer_name
            .as_deref()
            .or(inputs.existing_customer_name);
        let room_no = inputs
            .changes
            .new_room_no
            .as_deref()
            .or(inputs.existing_room_no);
        // Stay short-forms: payload stay (UTC → Bangkok, end snapped to
        // 11:59:59 AM) when it changed; otherwise the stored HT_Book_Ds
        // wall-clock values verbatim (Book_Room_End already carries the
        // 11:59:59 AM convention).
        let stay_short: Option<(String, String)> = match inputs.changes.new_stay.as_ref() {
            Some(stay) => Some((
                crate::writeback::format::format_bangkok(stay.start, "%d/%m %H:%M"),
                crate::writeback::format::format_bangkok(
                    end_of_stay_at_almost_noon(stay.end),
                    "%d/%m %H:%M",
                ),
            )),
            None => inputs.existing_stay.map(|(start, end)| {
                (
                    start.format("%d/%m %H:%M").to_string(),
                    end.format("%d/%m %H:%M").to_string(),
                )
            }),
        };
        if let (Some(customer_name), Some(room_no), Some((in_short, out_short))) =
            (customer_name, room_no, stay_short)
        {
            let phone = inputs.changes.new_customer_phone.as_deref().unwrap_or("");
            let notes = inputs.changes.new_notes.as_deref().unwrap_or("");
            let room_book_ds_q = sql_quote(&format!(
                "{name}  {phone} เวลาเข้าพัก : 00:00  1. {room}  ({in_short}) ถึง ({out_short})  หมายเหตุ : {notes} ",
                name = customer_name,
                phone = phone,
                room = room_no,
                in_short = in_short,
                out_short = out_short,
                notes = notes,
            ));
            // P1-10b: `Room_Book` must point at a REAL `HT_Book_Date.id`
            // for this booking (cheatsheet HT_Rooms invariant:
            // "Room_Book<>'' ⇒ a row exists in HT_Book_Date with that
            // id"). The previous `book_date_id_base` literal was
            // speculative — when every guarded night-INSERT above skips
            // (rows already exist), no row with that id is ever created
            // and the caption points at a dangling id. `MAX(id)` always
            // resolves to an existing row; ISNULL falls back to `''`
            // (the legacy "not booked" sentinel) if the booking somehow
            // has no night rows at all.
            let room_book_name_q = sql_quote(customer_name);
            let room_book_time_q = sql_quote("00:00");
            let room_no_q = sql_quote(room_no);
            statements.push(format!(
                "update HT_Rooms set room_book_ds={room_book_ds_q}, \
                 Room_Book=ISNULL(CAST((SELECT MAX(id) FROM HT_Book_Date \
                 WHERE Book_no={book_id_q}) AS varchar(50)),''),\
                 Room_Book_Name={room_book_name_q},Room_Book_Time={room_book_time_q} \
                 where room_no={room_no_q}"
            ));
        }
    }

    statements
}

// `build_customer_resave_update` moved to `recipes::helpers` (coexistence
// audit 2026-06-11 P2) so the standalone `update_customer` recipe shares the
// byte-identical 31-field UPDATE. Imported via the `use` at the top.

/// Execute the modify-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
    changes: &BookingChanges,
) -> WritebackResult<LegacyIds> {
    // Only allocate book_date IDs if stay changed (we may insert new nights).
    let book_date_id_base = if changes.new_stay.is_some() {
        allocate_book_date_id(conn).await?
    } else {
        0 // unused
    };
    // Wave 6 LOW item 6: enumerate_calendar_nights now returns Result —
    // empty range surfaces as Recipe error, cap-truncate logs WARN.
    let new_nights = if let Some(stay) = &changes.new_stay {
        enumerate_calendar_nights(stay.start, stay.end)?
    } else {
        Vec::new()
    };
    // P1-10a (extends H8 / HIGH-1): the caption rewrite now fires on
    // EVERY modify (step 0b always clears), so the existing room_no /
    // customer name / stay must be resolved from MSSQL whenever the
    // payload doesn't carry them — not only when the stay changes. The
    // room_no doubles as the `Book_type` for any HT_Book_Date INSERTs.
    // Three TOP-1 SELECTs inside the already-open recipe transaction.
    let existing_room_no = if changes.new_room_no.is_none() {
        fetch_existing_room_no(conn, book_id).await?
    } else {
        None
    };
    let existing_customer_name = if changes.new_customer_name.is_none() {
        fetch_existing_customer_name(conn, book_id).await?
    } else {
        None
    };
    // P1-10a: when the stay isn't changing, the caption renders the stored
    // HT_Book_Ds stay range.
    let existing_stay = if changes.new_stay.is_none() {
        fetch_existing_stay(conn, book_id).await?
    } else {
        None
    };
    // H10: a price-only modify (no `new_stay`) collapses the
    // `Book_Room_PriceToTal` to `price * 1` unless we know the existing
    // night count. Mirror the `existing_room_no` lookup pattern.
    let book_room_night_existing = if changes.new_price.is_some() && changes.new_stay.is_none() {
        fetch_existing_book_room_night(conn, book_id).await?
    } else {
        None
    };

    // MED-1: reject NaN/Infinity before SQL formatting. Today the price comes
    // from `Money` (always finite), but `build_statements` also interpolates
    // `baht * nights` into `[Book_Room_PriceToTal]` — defense-in-depth against
    // a future code path that introduces a non-Money f64 or an arithmetic
    // overflow producing infinity.
    if let Some(price) = &changes.new_price {
        let room_price_baht = (price.as_satang() as f64) / 100.0;
        // H10: mirror the same precedence `build_statements` uses — when
        // `new_stay` is None we use the existing night count for the total.
        let nights = if changes.new_stay.is_some() {
            new_nights.len().max(1) as f64
        } else {
            book_room_night_existing.unwrap_or(1).max(1) as f64
        };
        super::helpers::validate_finite(&[
            ("room_price_baht", room_price_baht),
            ("room_price_total_baht", room_price_baht * nights),
        ])?;
    }

    let inputs = ModifyBookingInputs {
        book_id,
        book_date_id_base,
        book_room_night_existing,
        changes,
        new_nights_calendar: new_nights,
        existing_room_no: existing_room_no.as_deref(),
        existing_customer_name: existing_customer_name.as_deref(),
        existing_stay,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_book_id(book_id.to_string()))
}

/// Look up the current room_no on the booking by reading any existing
/// `HT_Book_Date.Book_type` row. Returns `None` if no rows match (e.g.
/// brand-new booking that hasn't allocated its first night yet).
async fn fetch_existing_room_no(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
) -> WritebackResult<Option<String>> {
    let book_id_q = sql_quote(book_id);
    let sql = format!(
        "SELECT TOP 1 Book_type FROM HT_Book_Date \
         WHERE Book_no={book_id_q} AND Book_type IS NOT NULL AND Book_type<>'' \
         ORDER BY id DESC"
    );
    // R2 (2026-05-14): SELECT runs inside the recipe's BEGIN TRAN —
    // write budget. Same envelope as the surrounding INSERT/UPDATEs.
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    match rows.first() {
        Some(r) => Ok(r.get::<&str, _>(0).map(|s| s.to_string())),
        None => Ok(None),
    }
}

/// Look up the current customer name on the booking from `HT_Book_H`.
/// Mirrors `fetch_existing_room_no` — used by H8 to render the calendar
/// caption when only dates change and the payload doesn't carry the name.
async fn fetch_existing_customer_name(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
) -> WritebackResult<Option<String>> {
    let book_id_q = sql_quote(book_id);
    let sql = format!(
        "SELECT TOP 1 Book_Cust_Name FROM HT_Book_H \
         WHERE Book_ID={book_id_q}"
    );
    // R2: same envelope as fetch_existing_room_no above.
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    match rows.first() {
        Some(r) => Ok(r.get::<&str, _>(0).map(|s| s.to_string())),
        None => Ok(None),
    }
}

/// Look up the current stay range (`Book_Room_Start`, `Book_Room_End`) on
/// the booking from `HT_Book_Ds`. Bangkok wall-clock naive datetimes as
/// stored. Used by P1-10a to render the calendar-caption rewrite when the
/// payload doesn't change the stay. Returns `None` when the row or either
/// column is missing — the caption rewrite is then skipped.
async fn fetch_existing_stay(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
) -> WritebackResult<Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>> {
    let book_id_q = sql_quote(book_id);
    let sql = format!(
        "SELECT TOP 1 Book_Room_Start, Book_Room_End FROM HT_Book_Ds \
         WHERE Book_No={book_id_q}"
    );
    // R2: same envelope as fetch_existing_room_no above.
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    match rows.first() {
        Some(r) => {
            let start: Option<chrono::NaiveDateTime> = r.get(0);
            let end: Option<chrono::NaiveDateTime> = r.get(1);
            Ok(start.zip(end))
        }
        None => Ok(None),
    }
}

/// Look up the current `Book_Room_Night` on the booking from `HT_Book_Ds`.
/// Used by H10 to preserve the multi-night total when a price-only modify
/// fires (no `new_stay`, so `new_nights_calendar` is empty).
async fn fetch_existing_book_room_night(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
) -> WritebackResult<Option<i32>> {
    let book_id_q = sql_quote(book_id);
    let sql = format!(
        "SELECT TOP 1 Book_Room_Night FROM HT_Book_Ds \
         WHERE Book_No={book_id_q}"
    );
    // R2: same envelope as fetch_existing_room_no above.
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    match rows.first() {
        // HT_Book_Ds.Book_Room_Night is declared `int` in the schema dump
        // (2026-04-26); read as i32. Defensively widen None.
        Some(r) => Ok(r.get::<i32, _>(0)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shared::{DateRange, Money};
    use crate::outbox::intent::CustomerResave;
    use chrono::{TimeZone, Utc};

    #[test]
    fn no_destructive_delete_on_book_h_or_book_ds() {
        // Spike §3c rule: never DELETE from HT_Book_H / HT_Book_Ds — that's the
        // destructive pattern we explicitly skip.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
            )),
            new_room_no: Some("403".into()),
            new_room_type: None,
            new_price: Some(Money::from_baht(950)),
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let statements = build_statements(&inputs);
        for s in &statements {
            let lower = s.to_ascii_lowercase();
            assert!(
                !lower.contains("delete from ht_book_h")
                    && !lower.contains("delete from ht_book_ds"),
                "destructive pattern leaked: {s}"
            );
        }
    }

    #[test]
    fn empty_changes_only_emits_room_book_clear() {
        // Spike §3c capture lines 6/14/17: the .NET app fires the
        // `update HT_Rooms set room_book_*=''` clear on EVERY save, even
        // when no fields change. Mirroring keeps the calendar grid in sync.
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        // Without resolved existing context (name/room/stay all None) the
        // caption rewrite cannot render, so only the clear is emitted.
        // In production `execute()` always resolves the existing values,
        // so the rewrite fires — see
        // `caption_rewrite_fires_on_no_op_modify_with_existing_context`.
        assert_eq!(s.len(), 1);
        assert!(s[0].starts_with("update HT_Rooms set room_book_ds=''"));
        assert!(s[0].contains("Book_no='R014810'"));
    }

    /// 2026-06-11 audit P1-10a: step 0b clears the caption on EVERY modify,
    /// so the rewrite must fire on every modify too — including a modify
    /// that changes nothing (and, the original bug, a notes-only modify on
    /// a same-day booking). The stay / name / room render from the
    /// existing MSSQL values.
    #[test]
    fn caption_rewrite_fires_on_no_op_modify_with_existing_context() {
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: Some("402"),
            existing_customer_name: Some("SPIKE TEST GUEST"),
            existing_stay: Some((
                chrono::NaiveDate::from_ymd_opt(2026, 4, 25)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                    .unwrap()
                    .and_hms_opt(11, 59, 59)
                    .unwrap(),
            )),
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        assert_eq!(s.len(), 2, "clear + caption rewrite, got: {s:#?}");
        assert!(s[0].starts_with("update HT_Rooms set room_book_ds=''"));
        // Byte-pin the rewrite: existing stay rendered verbatim from the
        // stored wall-clock; Room_Book points at the live MAX(id) (P1-10b).
        assert_eq!(
            s[1],
            "update HT_Rooms set room_book_ds='SPIKE TEST GUEST   เวลาเข้าพัก : 00:00  \
             1. 402  (25/04 12:00) ถึง (26/04 11:59)  หมายเหตุ :  ', \
             Room_Book=ISNULL(CAST((SELECT MAX(id) FROM HT_Book_Date \
             WHERE Book_no='R014810') AS varchar(50)),''),\
             Room_Book_Name='SPIKE TEST GUEST',Room_Book_Time='00:00' \
             where room_no='402'"
        );
    }

    /// P1-10a (the original bug shape): a notes-only modify must re-write
    /// the caption — previously it cleared at 0b and never re-rendered,
    /// permanently blanking the iHOTEL room caption.
    #[test]
    fn caption_rewrite_fires_on_notes_only_modify() {
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: Some("late arrival".to_string()),
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: Some("402"),
            existing_customer_name: Some("SPIKE TEST GUEST"),
            existing_stay: Some((
                chrono::NaiveDate::from_ymd_opt(2026, 4, 25)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                    .unwrap()
                    .and_hms_opt(11, 59, 59)
                    .unwrap(),
            )),
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let caption = s
            .iter()
            .rev()
            .find(|stmt| stmt.contains("Room_Book_Name="))
            .expect("notes-only modify must re-write the caption");
        assert!(caption.contains("หมายเหตุ : late arrival "));
        assert!(caption.contains("where room_no='402'"));
        // And the caption must come AFTER the 0b clear. (The clear also
        // contains `Room_Book_Name=''`, so discriminate the rewrite by its
        // `where room_no=` filter — the clear filters by `room_book in`.)
        let clear_idx = s
            .iter()
            .position(|stmt| stmt.starts_with("update HT_Rooms set room_book_ds=''"))
            .unwrap();
        let caption_idx = s
            .iter()
            .position(|stmt| stmt.contains("Room_Book_Name=") && stmt.contains("where room_no="))
            .unwrap();
        assert!(clear_idx < caption_idx);
    }

    /// P1-10b: `Room_Book` must never be a speculative literal id — when
    /// the guarded night-INSERTs all skip (rows already exist), the
    /// `book_date_id_base` id is never inserted and the cheatsheet
    /// invariant "Room_Book<>'' ⇒ a row exists in HT_Book_Date with that
    /// id" breaks. The rewrite must point at the live MAX(id) instead.
    #[test]
    fn room_book_points_at_live_max_id_not_speculative_base() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            )),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: Some("SPIKE TEST WALKIN".into()),
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let caption = s
            .iter()
            .rev()
            .find(|stmt| stmt.contains("Room_Book_Name="))
            .expect("caption rewrite must be emitted");
        assert!(
            caption.contains(
                "Room_Book=ISNULL(CAST((SELECT MAX(id) FROM HT_Book_Date \
                 WHERE Book_no='R014810') AS varchar(50)),'')"
            ),
            "Room_Book must be the live MAX(id) subquery: {caption}"
        );
        assert!(
            !caption.contains("Room_Book='47300'"),
            "speculative book_date_id_base literal leaked: {caption}"
        );
    }

    #[test]
    fn book_date_inserts_use_existing_room_no_when_only_dates_change() {
        // HIGH-1 regression: previously, when new_stay was Some but
        // new_room_no was None, every HT_Book_Date INSERT was issued with
        // Book_type='', making the booking invisible in the .NET app's
        // calendar grid.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 5, 0, 0).unwrap(),
            )),
            new_room_no: None, // user only changed dates, not the room
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014812",
            book_date_id_base: 47301,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: Some("402"),
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let inserts: Vec<_> = s.iter().filter(|x| x.contains("INSERT INTO [HT_Book_Date]")).collect();
        assert_eq!(inserts.len(), 2, "expected one INSERT per night");
        for stmt in &inserts {
            assert!(
                stmt.contains(",'402',"),
                "Book_type must use existing_room_no, got: {stmt}"
            );
            assert!(
                !stmt.contains(",'',"),
                "Book_type must NOT be empty (HIGH-1): {stmt}"
            );
        }
    }

    #[test]
    fn book_date_inserts_use_new_room_no_when_room_changed() {
        // When the user moves rooms, new_room_no wins over existing_room_no.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            )),
            new_room_no: Some("510".to_string()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014812",
            book_date_id_base: 47302,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: Some("402"), // would be the wrong choice
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let insert = s.iter().find(|x| x.contains("INSERT INTO [HT_Book_Date]")).unwrap();
        assert!(insert.contains(",'510',"), "got: {insert}");
        assert!(!insert.contains(",'402',"));
    }

    #[test]
    fn updates_book_ds_with_actual_stay_times_and_almost_noon_end() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                // 5 AM UTC = noon Bangkok.
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let d = s.iter().find(|s| s.contains("[HT_Book_Ds]")).unwrap();
        assert!(d.contains("'4/25/2026 12:00:00 PM'"));
        assert!(d.contains("'4/26/2026 11:59:59 AM'"));
    }

    #[test]
    fn updates_book_h_with_phone_when_phone_changed() {
        // After fix #7 we always emit the room_book clear (statement 0). The
        // phone change adds the [Book_Cust_Tel] UPDATE on HT_Book_H.
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: Some("0900000099".into()),
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        // Statement 0: room_book clear; statement 1: HT_Book_H UPDATE.
        assert_eq!(s.len(), 2);
        let book_h = s.iter().find(|s| s.contains("[HT_Book_H]")).unwrap();
        assert!(book_h.contains("[Book_Cust_Tel]='0900000099'"));
    }

    #[test]
    fn clear_room_book_display_fires_before_date_diff() {
        // Spike §3c capture lines 6,14,17 — the clear must come BEFORE any
        // HT_Book_Date INSERT, otherwise the .NET calendar grid keeps stale
        // captions briefly even after a date change.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let clear_idx = s
            .iter()
            .position(|s| s.starts_with("update HT_Rooms set room_book_ds=''"))
            .unwrap();
        let date_insert_idx = s
            .iter()
            .position(|s| s.contains("INSERT INTO [HT_Book_Date]"))
            .unwrap();
        assert!(clear_idx < date_insert_idx, "clear must precede date INSERT");
    }

    #[test]
    fn customer_resave_emits_full_31_field_update() {
        // Verified from /tmp/legacy-events-full.log capture for
        // C21624 (line 3988): the .NET app's UPDATE includes the
        // trailing 5 fields Cust_Work_tax, Cust_perfix, Cust_sex,
        // Cust_IDcard, Cust_Contry — and the WHERE clause uses
        // lowercase `where` (not `WHERE`).
        let resave = CustomerResave {
            legacy_cust_no: "C21624".into(),
            cust_name: "Alberto Calvo Alvarez".into(),
            cust_type: "ราคาปกติ".into(),
            cust_type_main: "บุคคลธรรมดา".into(),
            cust_perfix: "925".into(),
            cust_sex: "ชาย".into(),
            ..CustomerResave::default()
        };
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: Some(resave),
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014820",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .expect("customer re-save UPDATE must be emitted when payload set");
        assert!(upd.contains("[Cust_name]='Alberto Calvo Alvarez'"));
        assert!(upd.contains("[Cust_Type_main]='บุคคลธรรมดา'"));
        assert!(upd.contains("[Cust_Work_tax]=''"));
        assert!(upd.contains("[Cust_perfix]='925'"));
        assert!(upd.contains("[Cust_sex]='ชาย'"));
        assert!(upd.contains("[Cust_IDcard]=''"));
        assert!(upd.contains("[Cust_Contry]=''"));
        // Lowercase `where` matches the legacy capture form.
        assert!(upd.contains(" where Cust_no='C21624'"));
        // Customer re-save must come BEFORE the room_book clear.
        let resave_idx = s
            .iter()
            .position(|s| s.starts_with("UPDATE [HT_Customers]"))
            .unwrap();
        let clear_idx = s
            .iter()
            .position(|s| s.starts_with("update HT_Rooms"))
            .unwrap();
        assert!(resave_idx < clear_idx);
    }

    #[test]
    fn caption_rewrite_uses_new_dates_when_full_context_supplied() {
        // Spike §3c capture line 26: re-write HT_Rooms display caption with
        // the new dates after a modify. Mirrors `booking_create`'s format.
        let stay = DateRange::new(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
        );
        let changes = BookingChanges {
            new_stay: Some(stay),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: Some("vip".into()),
            new_customer_phone: Some("0900000088".into()),
            new_customer_name: Some("SPIKE TEST WALKIN".into()),
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        // Caption rewrite is the LAST statement.
        let caption = s
            .iter()
            .rev()
            .find(|s| s.contains("Room_Book_Name="))
            .expect("caption rewrite must be emitted when full context set");
        assert!(caption.contains("'SPIKE TEST WALKIN'"));
        assert!(caption.contains("where room_no='402'"));
    }

    #[test]
    fn book_date_diff_uses_idempotent_insert() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        // The night INSERT should be guarded by NOT EXISTS for idempotency
        let insert = s
            .iter()
            .find(|s| s.contains("INSERT INTO [HT_Book_Date]"))
            .unwrap();
        assert!(insert.contains("WHERE NOT EXISTS"));
    }

    #[test]
    fn validate_finite_rejects_nan_room_price_baht() {
        // MED-1 guard: a non-finite f64 in the price would otherwise interpolate
        // as the literal string "NaN" / "inf" into SQL and fail the entire
        // legacy transaction. The execute() guard wraps the same call we
        // verify here directly — keeping this unit test pure (no MSSQL conn).
        let result = super::super::helpers::validate_finite(&[
            ("room_price_baht", f64::NAN),
            ("room_price_total_baht", 1780.0),
        ]);
        let err = result.expect_err("NaN must be rejected");
        let msg = err.to_string();
        assert!(msg.contains("room_price_baht"), "msg: {msg}");
        assert!(msg.contains("NaN"), "msg: {msg}");
    }

    #[test]
    fn validate_finite_rejects_infinity_in_room_price_total() {
        // MED-1 guard: defends the `baht * nights` interpolation specifically.
        // Even if `room_price_baht` is finite, the multiplication can overflow
        // to infinity for extreme inputs.
        let result = super::super::helpers::validate_finite(&[
            ("room_price_baht", 950.0),
            ("room_price_total_baht", f64::INFINITY),
        ]);
        let err = result.expect_err("Infinity must be rejected");
        assert!(err.to_string().contains("room_price_total_baht"));
    }

    /// H8 — caption rewrite must fire on a date-only edit. Previously only
    /// fired when `customer_name + room_no + stay` were ALL `Some`; a date
    /// change cleared the caption at step 0b but never re-wrote it, so the
    /// calendar grid lost the booking caption for the new date range.
    /// Fix: when `new_stay.is_some()`, resolve customer_name + room_no from
    /// existing rows and always emit the caption rewrite.
    #[test]
    fn caption_rewrite_fires_on_date_only_edit() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 5, 0, 0).unwrap(),
            )),
            new_room_no: None,         // user only changed dates
            new_customer_name: None,   // not in payload
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014812",
            book_date_id_base: 47301,
            book_room_night_existing: None,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: Some("402"),
            existing_customer_name: Some("SPIKE TEST GUEST"),
            existing_stay: None,
        };
        let s = build_statements(&inputs);
        // The rewrite is the LAST `update HT_Rooms` — distinguished from the
        // step-0b clear because the clear sets `room_book_ds=''` while the
        // rewrite sets a populated caption + `where room_no=…`.
        let caption = s
            .iter()
            .rev()
            .find(|stmt| stmt.contains("update HT_Rooms") && stmt.contains("where room_no="))
            .expect("caption rewrite must fire when stay changes, even without room/customer in payload");
        assert!(
            caption.contains("'SPIKE TEST GUEST'"),
            "caption must use existing customer name: {caption}"
        );
        assert!(
            caption.contains("where room_no='402'"),
            "caption must target existing room: {caption}"
        );
    }

    /// H9 — `Book_Date_in/out` on HT_Book_H must use date-only format
    /// (`'4/25/2026'`) to match `booking_create` and the latest legacy
    /// capture. Previously used midnight-suffix (`'4/25/2026 12:00:00 AM'`),
    /// creating two shapes for the same column in one writeback session.
    #[test]
    fn book_h_dates_use_date_only_format() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 11, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            book_room_night_existing: None,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
        };
        let s = build_statements(&inputs);
        let h = s.iter().find(|stmt| stmt.contains("[HT_Book_H]")).unwrap();
        assert!(
            h.contains("[Book_Date_in]='4/25/2026'"),
            "Book_Date_in must be date-only ('4/25/2026'), got: {h}"
        );
        assert!(
            h.contains("[Book_Date_out]='4/26/2026'"),
            "Book_Date_out must be date-only ('4/26/2026'), got: {h}"
        );
        // The old midnight-suffix form must be gone.
        assert!(
            !h.contains("12:00:00 AM"),
            "midnight-suffix must NOT appear in HT_Book_H update: {h}"
        );
    }

    /// H10 — price-only modify must preserve `Book_Room_Night` (and the
    /// computed `Book_Room_PriceToTal`). Previously fell back to
    /// `new_nights_calendar.len().max(1) = 1` when `new_stay` was None,
    /// corrupting a 3-night booking's total from `2670.00` to `890.00`.
    /// Fix: look up the existing night count from MSSQL when computing
    /// the total for a price-only modify.
    #[test]
    fn price_only_modify_preserves_multi_night_total() {
        let changes = BookingChanges {
            new_stay: None,                       // no date change
            new_price: Some(Money::from_baht(890)),
            new_room_no: None,
            new_room_type: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            book_room_night_existing: Some(3), // existing 3-night booking
            changes: &changes,
            new_nights_calendar: vec![], // empty because no stay change
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
        };
        let s = build_statements(&inputs);
        let ds = s
            .iter()
            .find(|stmt| stmt.contains("[HT_Book_Ds]"))
            .expect("HT_Book_Ds UPDATE must be emitted for a price-only modify");
        // 890 * 3 = 2670, formatted to 2dp per the canonical legacy shape.
        assert!(
            ds.contains("[Book_Room_PriceToTal]=2670"),
            "total must be price * existing_nights = 2670, got: {ds}"
        );
        assert!(
            !ds.contains("[Book_Room_PriceToTal]=890.00")
                && !ds.contains("[Book_Room_PriceToTal]=890,"),
            "total must NOT collapse to a single-night value: {ds}"
        );
    }

    #[test]
    fn book_date_keeps_only_new_calendar_dates() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
            book_room_night_existing: None,
        };
        let s = build_statements(&inputs);
        let del = s
            .iter()
            .find(|s| s.starts_with("DELETE FROM HT_Book_Date"))
            .unwrap();
        // Wave 5b item 6: both sides of NOT IN are cast to DATE.
        assert!(del.contains("CAST(Book_date_ds AS DATE) NOT IN (CAST('4/25/2026' AS DATE))"));
    }

    /// Wave 5b item 5: a notes-only edit must write BOTH
    /// `HT_Book_H.[Book_room_note]` (header, lowercase r) AND
    /// `HT_Book_Ds.[Book_Room_Note]` (capital R — what iHOTEL's edit form
    /// binds to). Without the Ds write, the visible note in iHOTEL stayed
    /// stale until the user re-saved.
    #[test]
    fn notes_change_updates_both_header_and_ds_columns() {
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: Some("VIP — please welcome".to_string()),
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            book_room_night_existing: None,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
            existing_customer_name: None,
            existing_stay: None,
        };
        let s = build_statements(&inputs);
        let header = s
            .iter()
            .find(|stmt| stmt.starts_with("UPDATE [HT_Book_H]"))
            .expect("HT_Book_H UPDATE must be emitted");
        assert!(
            header.contains("[Book_room_note]='VIP — please welcome'"),
            "header note (lowercase r) missing: {header}"
        );
        let ds = s
            .iter()
            .find(|stmt| stmt.starts_with("UPDATE [HT_Book_Ds]"))
            .expect("HT_Book_Ds UPDATE must be emitted for notes change");
        assert!(
            ds.contains("[Book_Room_Note]='VIP — please welcome'"),
            "ds note (capital R) missing: {ds}"
        );
    }

    /// Wave 5b item 6: NOT-IN list and `Book_date_ds` column must BOTH be
    /// cast to DATE — defends against any future row stored with a
    /// non-midnight datetime (the legacy app stores midnight, but a row
    /// written by another path could differ).
    #[test]
    fn book_date_ds_filter_casts_to_date() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 5, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            book_room_night_existing: None,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: Some("402"),
            existing_customer_name: None,
            existing_stay: None,
        };
        let s = build_statements(&inputs);
        let del = s
            .iter()
            .find(|stmt| stmt.starts_with("DELETE FROM HT_Book_Date"))
            .expect("DELETE FROM HT_Book_Date must be emitted");
        assert!(
            del.contains("CAST(Book_date_ds AS DATE)"),
            "column side must be CAST AS DATE: {del}"
        );
        assert!(
            del.contains("CAST('4/25/2026' AS DATE)") && del.contains("CAST('4/26/2026' AS DATE)"),
            "every kept-list entry must be CAST AS DATE: {del}"
        );
        // The old uncast literal form must be gone (would compare datetime
        // to string and silently miss non-midnight rows).
        assert!(
            !del.contains("NOT IN ('4/25/2026'"),
            "old uncast literal form leaked: {del}"
        );
    }
}
