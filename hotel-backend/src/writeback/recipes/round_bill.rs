//! `OpenRound` / `CloseRound` recipes — Track J6 (round-bill coexistence
//! step 2). Faithful translation of iHOTEL's `FrmDueBill.cs` round handling,
//! captured in `docs/legacy-app/COMPAT_CHEATSHEET.md` §960-970 / §3.20-3.21.
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
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{format_legacy_datetime, sql_quote};
use crate::writeback::recipes::helpers::validate_finite;

/// SQL Server error numbers for a key collision on the `INSERT`:
/// `2627` = PRIMARY KEY / UNIQUE constraint violation, `2601` = unique
/// index violation. Either means iHOTEL grabbed our allocated
/// `HT_Round_Bill.id` first (the same-instant double-open race). Pure
/// helper mirroring `error::is_transient_pg_sqlstate` so the
/// classification is unit-testable without constructing a tiberius error.
fn is_round_id_collision(code: Option<u32>) -> bool {
    matches!(code, Some(2627) | Some(2601))
}

/// Map an open-INSERT failure: a `HT_Round_Bill.id` PK/unique collision
/// (iHOTEL won the same-instant open race) is **deterministic** — the id is
/// fixed in the payload, so every retry re-collides. Convert it to a
/// non-retryable [`WritebackError::Recipe`] so the job dead-letters at once
/// (fail-loud) instead of burning the retry budget. The outcome is still
/// fail-SAFE — the PK already prevented a duplicate round, and
/// `sync_round_bills` reconciles `ht_shifts` to iHOTEL's row on the next
/// tick. Any other error passes through unchanged (real transient I/O stays
/// retryable).
fn map_open_collision(err: WritebackError, legacy_round_id: i32) -> WritebackError {
    if let WritebackError::Tiberius(ref tib) = err {
        if is_round_id_collision(tib.code()) {
            return WritebackError::Recipe(format!(
                "HT_Round_Bill id {legacy_round_id} already exists (SQL Server error {} — \
                 iHOTEL won the same-instant open race); not retryable. sync_round_bills \
                 will reconcile ht_shifts to iHOTEL's open round on the next tick.",
                tib.code().unwrap_or(0),
            ));
        }
    }
    err
}

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
    // A `HT_Round_Bill.id` PK/unique collision means iHOTEL grabbed our
    // allocated id first (the same-instant double-open race). The id is fixed
    // in the payload, so a retry would re-collide forever — map it to a
    // non-retryable Recipe error for an immediate fail-loud dead-letter
    // (fail-SAFE: no duplicate round, and sync_round_bills self-heals
    // ht_shifts to iHOTEL's round). Real transient I/O passes through
    // retryable. The race is rated Low (one open round per site).
    super::execute_all(conn, &statements)
        .await
        .map_err(|e| map_open_collision(e, payload.legacy_round_id))?;
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

    #[test]
    fn pk_unique_collision_codes_are_classified() {
        // 2627 = PK/UNIQUE constraint violation, 2601 = unique index violation
        // — both mean iHOTEL won the same-instant open race.
        assert!(is_round_id_collision(Some(2627)));
        assert!(is_round_id_collision(Some(2601)));
        // Anything else (transient I/O, other server errors, no code) is NOT a
        // collision and must stay retryable.
        assert!(!is_round_id_collision(Some(1205))); // deadlock victim — retry
        assert!(!is_round_id_collision(Some(0)));
        assert!(!is_round_id_collision(None));
    }

    #[test]
    fn map_open_collision_only_rewrites_round_id_collisions() {
        // A non-tiberius error (e.g. a Recipe error) passes through untouched.
        let recipe = WritebackError::Recipe("boom".into());
        assert!(matches!(map_open_collision(recipe, 4778), WritebackError::Recipe(m) if m == "boom"));
    }
}
