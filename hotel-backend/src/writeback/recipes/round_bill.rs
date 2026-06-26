//! `OpenRound` / `CloseRound` recipes — Track J6 (round-bill coexistence
//! step 2). Faithful translation of iHOTEL's `FrmDueBill.cs` round handling,
//! captured in `docs/legacy-app/COMPAT_CHEATSHEET.md` §946-956 / §3.20-3.21.
//!
//! iHOTEL reference SQL (the only two statements it fires against
//! `HT_Round_Bill`):
//!
//! ```text
//! -- open (FrmDueBill.cs:1653) — id allocated app-side via get_id (MAX+1)
//! INSERT INTO HT_Round_Bill (id, round_start, round_price, round_by)
//! VALUES (<id>, '<now>', <float>, '<loginName>')           -- round_end NULL
//!
//! -- close (FrmDueBill.cs:1670)
//! UPDATE HT_Round_Bill SET round_end='<now>', round_by='<emp>'
//!  WHERE round_end IS NULL
//! ```
//!
//! ## Departures from the literal iHOTEL SQL (both safe, both deliberate)
//!
//! - **`id` is explicit, not re-derived.** `ShiftService::open_shift`
//!   allocates it as `MAX(shift_no)+1` and mirrors it into
//!   `ht_shifts.shift_legacy_round_id`; we pass that exact value so the
//!   canonical row and the legacy row stay on the same id (and the read-only
//!   `sync_round_bills` mirror, which keys `ht_shifts` on `shift_no = legacy
//!   id`, lines up on the next tick). We do NOT take iHOTEL's `TABLOCKX`
//!   `MAX+1` path: the `ht_shifts_one_open_per_site` partial unique index
//!   already serializes *our* openers (you cannot have two open rounds), so a
//!   write-time allocator would buy nothing. The legacy `HT_Round_Bill.id`
//!   PRIMARY KEY (added by the CT-prerequisite DDL, migration 020-022) is the
//!   collision backstop against iHOTEL's concurrent `MAX+1`: a same-instant
//!   double-open hard-fails one INSERT (2627/2601) rather than minting a
//!   duplicate round. OpenRound is ledgered, so a crash-after-commit retry is
//!   a no-op before it can even reach the PK.
//!
//! - **close pins `id`.** iHOTEL closes "whatever is open"
//!   (`WHERE round_end IS NULL`); we know our id, so we add `id=<legacy
//!   round_id> AND round_end IS NULL`. This is strictly safer (cannot close a
//!   round we didn't mean to) and idempotent: a second apply — or one racing
//!   iHOTEL's own close of the same round — matches 0 rows and is a no-op.
//!
//! ## Timezone
//!
//! `round_start` / `round_end` are tz-naive **Thai local** on the legacy side
//! (CLAUDE.md "Timezone Handling"). The canonical `ht_shifts` timestamps are
//! `TIMESTAMPTZ` (UTC); [`format_legacy_datetime`] renders the Bangkok
//! wall-clock naive literal, identical to every other recipe.

use crate::outbox::intent::{CloseRoundPayload, OpenRoundPayload};
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};
use crate::writeback::recipes::helpers::validate_finite;

/// Build the single `INSERT HT_Round_Bill` statement that opens a round. PURE.
///
/// `round_no` is intentionally omitted (NULL) — it is unused in the live data
/// (every `HT_Round_Bill.round_no` is NULL at both sites) and iHOTEL's open
/// does not set it either. `round_end` defaults to NULL = "open".
pub fn build_open_statement(payload: &OpenRoundPayload) -> Vec<String> {
    let start_q = sql_quote(&format_legacy_datetime(payload.round_start));
    let by_q = sql_quote(&payload.round_by);
    // `round_price` is a float column; render 2dp for parity with the money
    // formatting used elsewhere (iHOTEL writes e.g. `3000`).
    let price = format!("{:.2}", payload.round_price);
    vec![format!(
        "INSERT INTO HT_Round_Bill (id, round_start, round_price, round_by) \
         VALUES ({id}, {start_q}, {price}, {by_q})",
        id = payload.legacy_round_id,
    )]
}

/// Build the single `UPDATE HT_Round_Bill` statement that closes a round. PURE.
pub fn build_close_statement(payload: &CloseRoundPayload) -> Vec<String> {
    let end_q = sql_quote(&format_legacy_datetime(payload.round_end));
    let by_q = sql_quote(&payload.round_by);
    vec![format!(
        "UPDATE HT_Round_Bill SET round_end={end_q}, round_by={by_q} \
         WHERE id={id} AND round_end IS NULL",
        id = payload.legacy_round_id,
    )]
}

/// Execute the open-round recipe. Returns empty [`LegacyIds`] — the canonical
/// `ht_shifts.shift_legacy_round_id` is already set by `open_shift`, so there
/// is nothing for the worker to back-populate.
pub async fn execute_open(
    conn: &mut LegacyConn<'_>,
    payload: &OpenRoundPayload,
) -> WritebackResult<LegacyIds> {
    validate_finite(&[("round_price", payload.round_price)])?;
    let statements = build_open_statement(payload);
    // Collision note (pre-flip hardening, P2): if iHOTEL grabbed our allocated
    // `id` in a same-instant double-open, this INSERT hits the legacy PK
    // (SQL Server 2627/2601). That surfaces as `WritebackError::Tiberius`,
    // which `is_retryable()` currently treats as retryable — so the fixed-id
    // job re-fails identically until it dead-letters. The outcome is
    // fail-SAFE (the PK prevents a duplicate round, and `sync_round_bills`
    // reconciles `ht_shifts` to iHOTEL's row on the next tick), just noisy.
    // Before flipping `ROUND_WRITEBACK_ENABLED` on, map 2627/2601 here to a
    // non-retryable `WritebackError::Recipe` for an immediate fail-loud
    // dead-letter. Left as-is while the feature ships dark + the race is rated
    // Low (one open round per site; the close→open is a deliberate human step).
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new())
}

/// Execute the close-round recipe. Returns empty [`LegacyIds`].
pub async fn execute_close(
    conn: &mut LegacyConn<'_>,
    payload: &CloseRoundPayload,
) -> WritebackResult<LegacyIds> {
    let statements = build_close_statement(payload);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn open_payload() -> OpenRoundPayload {
        OpenRoundPayload {
            site_id: "hfhotel".into(),
            legacy_round_id: 4778,
            round_price: 3000.0,
            round_by: "Admin".into(),
            // 07:17:24 UTC == 14:17:24 Bangkok.
            round_start: Utc.with_ymd_and_hms(2026, 6, 26, 7, 17, 24).unwrap(),
        }
    }

    fn close_payload() -> CloseRoundPayload {
        CloseRoundPayload {
            site_id: "hfhotel".into(),
            legacy_round_id: 4778,
            round_by: "Admin".into(),
            // 15:00:00 UTC == 22:00:00 Bangkok.
            round_end: Utc.with_ymd_and_hms(2026, 6, 26, 15, 0, 0).unwrap(),
        }
    }

    #[test]
    fn open_matches_ihotel_shape() {
        let stmts = build_open_statement(&open_payload());
        assert_eq!(stmts.len(), 1);
        let s = &stmts[0];
        assert!(s.starts_with("INSERT INTO HT_Round_Bill (id, round_start, round_price, round_by)"));
        assert!(s.contains("VALUES (4778,"), "explicit id: {s}");
        assert!(s.contains("3000.00"), "round_price 2dp: {s}");
        assert!(s.contains("'Admin'"), "round_by: {s}");
        // Bangkok wall-clock (UTC + 7h), tz-naive, rendered in the legacy
        // .NET `M/D/YYYY h:mm:ss AM/PM` shape by `format_legacy_datetime`
        // (07:17:24 UTC → 14:17:24 Bangkok → 2:17:24 PM).
        assert!(s.contains("'6/26/2026 2:17:24 PM'"), "Bangkok naive round_start: {s}");
        // round_no / round_end intentionally absent (left NULL).
        assert!(!s.to_lowercase().contains("round_no"), "round_no must stay NULL: {s}");
        assert!(!s.to_lowercase().contains("round_end"), "round_end must stay NULL: {s}");
    }

    #[test]
    fn close_pins_id_and_is_idempotent() {
        let stmts = build_close_statement(&close_payload());
        assert_eq!(stmts.len(), 1);
        let s = &stmts[0];
        assert!(s.starts_with("UPDATE HT_Round_Bill SET round_end="));
        // 15:00:00 UTC → 22:00:00 Bangkok → 10:00:00 PM (legacy .NET shape).
        assert!(s.contains("'6/26/2026 10:00:00 PM'"), "Bangkok naive round_end: {s}");
        assert!(s.contains("round_by='Admin'"), "round_by re-stamped: {s}");
        // Pinned id AND the open-only guard → precise + idempotent.
        assert!(s.contains("WHERE id=4778 AND round_end IS NULL"), "guarded UPDATE: {s}");
    }

    #[test]
    fn open_quotes_round_by_safely() {
        let mut p = open_payload();
        p.round_by = "O'Brien".into();
        let s = &build_open_statement(&p)[0];
        assert!(s.contains("'O''Brien'"), "apostrophe must be doubled: {s}");
    }

    #[test]
    fn open_rejects_non_finite_price() {
        let mut p = open_payload();
        p.round_price = f64::NAN;
        // build_statement is pure (no validation); execute_open validates.
        // Assert the guard the recipe uses rejects NaN.
        assert!(validate_finite(&[("round_price", p.round_price)]).is_err());
    }
}
