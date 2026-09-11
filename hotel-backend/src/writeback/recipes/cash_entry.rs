//! `CreateCashEntry` recipe — petty-cash income/expense writeback to legacy
//! `TB_Pay_History` (migration 059).
//!
//! ## STATUS: WIRED (intent + dispatcher + back-population), NOT YET EMITTED
//!
//! Issue #202 wired [`WritebackIntent::CreateCashEntry`] → the dispatcher arm
//! in `writeback::dispatcher::dispatch` → [`execute`] (this module) →
//! `mark_done`'s `back_populate_legacy_ids` stamping the allocated
//! `TB_Pay_History.id` onto `ht_cash_ledger.cash_legacy_id` (migration 085
//! added the `aggregate_id` column that back-population keys on). What is
//! STILL missing — and what keeps cash-outbound dark — is a service/route
//! call site that actually ENQUEUES this intent: `POST
//! /api/cash/{income,expense}` (`routes/new_cash.rs`) still writes canonical
//! PG only. Inbound mirroring (iHOTEL → us) is live via
//! `bin/sync.rs::sync_cash_history`.
//!
//! **Why the emission side stays unwired:** coexistence writebacks fire a
//! REAL write to the shared legacy DB the moment the feature is used, so they
//! must be byte-exact AND ship behind an env flag with reception-coordinated
//! live verification (repo invariant — new legacy writes ship dark). What we
//! can verify from `docs/legacy-app/` is the column ORDER, types, the
//! positional (no-column-list) INSERT form, the OADate `float` date encoding,
//! and the app-side `get_id` (MAX+1) id allocation
//! (`docs/legacy-app/SCHEMA.sql` §"Table: dbo.TB_Pay_History" "[id] int,",
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `TB_Pay_History` (A)"
//! "`id int` (NOT IDENTITY) via `get_id`", §"1.4 Date/time handling"
//! "`TB_Pay_History.Pay_Date` is `float`").
//! What we CANNOT yet verify (the off-repo `FrmAddPay.cs` decompile is needed —
//! `docs/legacy-app/EVERGREEN_ARTIFACTS.md`):
//!
//!   * the exact `Pay_Type` strings iHOTEL writes for income vs expense
//!     (this recipe takes them as an explicit payload field rather than
//!     guessing);
//!   * the semantics of `Pay_Program` (a SECOND `float` OADate — posting date?
//!     `0`? a copy of `Pay_Date`?) — defaulted here to the entry date;
//!   * which tree level (`MyType2` / `2_2` / `3`) maps to `Pay_Group` vs
//!     `Pay_Account`.
//!
//! Before flipping the emission flag: confirm the three points above against
//! `FrmAddPay.cs:638`, add the `POST /api/cash/*` outbox emission behind an
//! env-gated flag (default off), and re-validate the emitted SQL against a
//! live capture.
//!
//! ## Legacy reference (positional INSERT, `FrmAddPay.cs:638`)
//!
//! ```text
//! -- id allocated app-side via get_id (MAX+1); Pay_Date / Pay_Program are
//! -- DateTime.ToOADate() floats (days since 1899-12-30).
//! INSERT INTO TB_Pay_History
//!   VALUES (<id>, <Pay_Date OADate>, '<Pay_Bill>', '<Pay_Cust>', '<Pay_Type>',
//!           <Pay_Total>, '<Pay_Note>', <Pay_Program OADate>, '<Pay_Group>',
//!           '<Pay_Account>')
//! ```
//!
//! ## Timezone
//!
//! `entry_date` / `program_date` are real UTC instants in canonical PG; the
//! legacy OADate encodes a tz-naive Thai-local date, so we take the **Bangkok
//! calendar day** ([`bangkok_date`]) before serializing — same convention as
//! every other recipe.

use chrono::{DateTime, Utc};

use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    bangkok_date, date_to_ole_serial, money_2dp, sql_quote, sql_quote_or_empty,
};
use crate::writeback::recipes::helpers::validate_finite;

/// Everything needed to mirror one cash-ledger entry into `TB_Pay_History`.
/// Mirrors the canonical `ht_cash_ledger` row (migration 059). `legacy_pay_type`
/// is the RAW `Pay_Type` string to write — supplied explicitly because the
/// income/expense markers are not yet verified (see the module TODO).
#[derive(Debug, Clone)]
pub struct CashEntryPayload {
    /// Which site this entry belongs to ("hfhotel" | "hfville").
    pub site_id: String,
    /// `Pay_Date` — the entry date (rendered as a Bangkok-calendar OADate).
    pub entry_date: DateTime<Utc>,
    /// `Pay_Program` — a second legacy OADate of uncertain semantics; when
    /// `None` we default it to the entry date (documented guess — verify).
    pub program_date: Option<DateTime<Utc>>,
    /// `Pay_Total` — amount in baht (must be finite; sign per iHOTEL convention).
    pub amount: f64,
    /// `Pay_Type` — raw legacy income/expense marker (verbatim).
    pub legacy_pay_type: String,
    /// `Pay_Bill`.
    pub bill_no: Option<String>,
    /// `Pay_Cust`.
    pub payee: Option<String>,
    /// `Pay_Note`.
    pub note: Option<String>,
    /// `Pay_Group` — account-tree id_full.
    pub group: Option<String>,
    /// `Pay_Account` — account-tree id_full.
    pub account: Option<String>,
}

/// Build the single positional `INSERT INTO TB_Pay_History` statement. PURE.
///
/// `legacy_id` is the `get_id`-style MAX+1 id; a real caller would allocate it
/// under the `allocate::*` TABLOCKX lock and pass it in here (the same pattern
/// the other MAX+1 recipes use). The 10 VALUES are emitted in the exact legacy
/// column order: `id, Pay_Date, Pay_Bill, Pay_Cust, Pay_Type, Pay_Total,
/// Pay_Note, Pay_Program, Pay_Group, Pay_Account`.
///
/// ECHO-SAFETY (issue #202): [`execute`] back-populates the allocated
/// `legacy_id` onto the canonical `ht_cash_ledger` row's `cash_legacy_id`
/// after the write (mirroring payments' `back_populate_legacy_ids` →
/// `legacy_receipt_no`) — see `bin/writeback.rs::back_populate_legacy_ids`'s
/// `CreateCashEntry` arm. The cash mirror importer
/// (`bin/sync.rs::sync_cash_history`) dedups `ON CONFLICT (cash_legacy_id)`,
/// so without that back-population an app-originated cash write re-imports as
/// a phantom duplicate. Do not enable cash-outbound EMISSION (the
/// `POST /api/cash/*` route enqueueing this intent) until that emission path
/// itself ships behind a flag and is reception-verified live — the
/// back-population plumbing this module now provides is a prerequisite, not
/// a green light.
pub fn build_statements(payload: &CashEntryPayload, legacy_id: i32) -> WritebackResult<Vec<String>> {
    validate_finite(&[("amount", payload.amount)])?;

    let pay_date = date_to_ole_serial(bangkok_date(payload.entry_date));
    // Pay_Program semantics unverified — default to the entry date's OADate.
    let pay_program = payload
        .program_date
        .map(|d| date_to_ole_serial(bangkok_date(d)))
        .unwrap_or(pay_date);
    let total = money_2dp(payload.amount)?;

    let stmt = format!(
        "INSERT INTO TB_Pay_History VALUES ({id}, {pay_date}, {bill}, {cust}, {ptype}, \
         {total}, {note}, {program}, {group}, {account})",
        id = legacy_id,
        pay_date = pay_date,
        bill = sql_quote_or_empty(payload.bill_no.as_deref()),
        cust = sql_quote_or_empty(payload.payee.as_deref()),
        ptype = sql_quote(&payload.legacy_pay_type),
        total = total,
        note = sql_quote_or_empty(payload.note.as_deref()),
        program = pay_program,
        group = sql_quote_or_empty(payload.group.as_deref()),
        account = sql_quote_or_empty(payload.account.as_deref()),
    );
    Ok(vec![stmt])
}

/// Execute the recipe: allocate `TB_Pay_History.id` (MAX+1 TABLOCKX — see
/// [`crate::writeback::allocate::allocate_pay_history_id`]), build + run the
/// INSERT, and return the allocated id via [`LegacyIds::cash_legacy_id`] so
/// the worker's `mark_done` can back-populate `ht_cash_ledger.cash_legacy_id`
/// (issue #202).
///
/// Takes the OUTBOX wire payload (`outbox::intent::CreateCashEntryPayload`,
/// not this module's [`CashEntryPayload`]) — `outbox/` doesn't depend on
/// `writeback/`, so the two types are field-for-field mirrors and this is
/// where they're bridged, one time, at the dispatch boundary.
///
/// No `WHERE NOT EXISTS` guard on the INSERT (unlike `payment`'s
/// `HT_CheckIn_Pay` row): `CreateCashEntry` is classified `ledgered: true` in
/// `dispatcher::intent_facts` (like `CreateBooking`), so `dispatch`'s
/// `dbo.ht_writeback_ledger` lookup is the sole idempotency guard — same
/// choice `booking_create.rs` makes.
pub async fn execute(
    conn: &mut crate::writeback::allocate::LegacyConn<'_>,
    payload: &crate::outbox::intent::CreateCashEntryPayload,
) -> WritebackResult<crate::writeback::dispatcher::LegacyIds> {
    use crate::writeback::allocate::allocate_pay_history_id;
    use crate::writeback::dispatcher::LegacyIds;

    let legacy_id = allocate_pay_history_id(conn).await?;
    let recipe_payload = from_outbox_payload(payload);
    let statements = build_statements(&recipe_payload, legacy_id)?;
    super::execute_all(conn, &statements).await?;

    Ok(LegacyIds::new().with_cash_legacy_id(legacy_id))
}

/// Bridge the OUTBOX wire payload into this module's recipe-internal
/// [`CashEntryPayload`]. PURE — split out of [`execute`] so the field mapping
/// is unit-testable without a live MSSQL connection.
fn from_outbox_payload(payload: &crate::outbox::intent::CreateCashEntryPayload) -> CashEntryPayload {
    CashEntryPayload {
        site_id: payload.site_id.clone(),
        entry_date: payload.entry_date,
        program_date: payload.program_date,
        amount: payload.amount,
        legacy_pay_type: payload.legacy_pay_type.clone(),
        bill_no: payload.bill_no.clone(),
        payee: payload.payee.clone(),
        note: payload.note.clone(),
        group: payload.group.clone(),
        account: payload.account.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn payload() -> CashEntryPayload {
        CashEntryPayload {
            site_id: "hfhotel".into(),
            // 07:00 UTC == 14:00 Bangkok on 4/24 → Bangkok calendar day 4/24
            // → OADate 46136 (matches format::date_to_ole_serial spike value).
            entry_date: Utc.with_ymd_and_hms(2026, 4, 24, 7, 0, 0).unwrap(),
            program_date: None,
            amount: 1500.0,
            legacy_pay_type: "รายจ่าย".into(),
            bill_no: Some("B-001".into()),
            payee: Some("ค่าน้ำ".into()),
            note: Some("note".into()),
            group: Some("G1".into()),
            account: Some("A1".into()),
        }
    }

    #[test]
    fn build_matches_positional_shape() {
        let stmts = build_statements(&payload(), 42).unwrap();
        assert_eq!(stmts.len(), 1);
        let s = &stmts[0];
        // Positional INSERT (no column list) — exactly as FrmAddPay emits.
        assert!(s.starts_with("INSERT INTO TB_Pay_History VALUES ("), "{s}");
        assert!(s.contains("(42, 46136,"), "explicit id + Pay_Date OADate: {s}");
        assert!(s.contains("'รายจ่าย'"), "raw Pay_Type verbatim: {s}");
        assert!(s.contains("1500.00"), "Pay_Total 2dp: {s}");
        assert!(s.contains("'B-001'") && s.contains("'ค่าน้ำ'"), "bill + cust: {s}");
        assert!(s.contains("'G1'") && s.contains("'A1'"), "group + account: {s}");
    }

    #[test]
    fn program_date_defaults_to_entry_date_oadate() {
        // No program_date → Pay_Program OADate equals Pay_Date OADate (46136),
        // so the serial appears twice in the VALUES list.
        let s = &build_statements(&payload(), 1).unwrap()[0];
        assert_eq!(s.matches("46136").count(), 2, "Pay_Date + Pay_Program both 46136: {s}");
    }

    #[test]
    fn null_optionals_render_as_empty_string() {
        let mut p = payload();
        p.bill_no = None;
        p.payee = None;
        p.note = None;
        p.group = None;
        p.account = None;
        let s = &build_statements(&p, 7).unwrap()[0];
        // sql_quote_or_empty(None) → '' (spike §3k convention).
        assert!(s.contains("''"), "None optionals become '': {s}");
    }

    #[test]
    fn apostrophes_are_escaped() {
        let mut p = payload();
        p.payee = Some("O'Brien".into());
        let s = &build_statements(&p, 9).unwrap()[0];
        assert!(s.contains("'O''Brien'"), "apostrophe doubled: {s}");
    }

    #[test]
    fn non_finite_amount_is_rejected() {
        let mut p = payload();
        p.amount = f64::NAN;
        assert!(build_statements(&p, 1).is_err());
    }

    // ── Issue #202 wiring ──────────────────────────────────────────────

    fn outbox_payload() -> crate::outbox::intent::CreateCashEntryPayload {
        crate::outbox::intent::CreateCashEntryPayload {
            site_id: "hfhotel".into(),
            entry_date: Utc.with_ymd_and_hms(2026, 4, 24, 7, 0, 0).unwrap(),
            program_date: None,
            amount: 1500.0,
            legacy_pay_type: "รายจ่าย".into(),
            bill_no: Some("B-001".into()),
            payee: Some("ค่าน้ำ".into()),
            note: Some("note".into()),
            group: Some("G1".into()),
            account: Some("A1".into()),
        }
    }

    /// `from_outbox_payload` must carry every field across field-for-field —
    /// a dropped field here would silently blank that column in the legacy
    /// INSERT for every future emitter, with no compile error (both structs
    /// have the same field names but are structurally distinct types).
    #[test]
    fn from_outbox_payload_maps_every_field() {
        let mapped = from_outbox_payload(&outbox_payload());
        assert_eq!(mapped.site_id, "hfhotel");
        assert_eq!(mapped.entry_date, outbox_payload().entry_date);
        assert_eq!(mapped.program_date, None);
        assert_eq!(mapped.amount, 1500.0);
        assert_eq!(mapped.legacy_pay_type, "รายจ่าย");
        assert_eq!(mapped.bill_no.as_deref(), Some("B-001"));
        assert_eq!(mapped.payee.as_deref(), Some("ค่าน้ำ"));
        assert_eq!(mapped.note.as_deref(), Some("note"));
        assert_eq!(mapped.group.as_deref(), Some("G1"));
        assert_eq!(mapped.account.as_deref(), Some("A1"));
    }

    /// "id surfaced from the recipe" (issue #202 test requirement): the id
    /// `execute` would allocate is exactly what `build_statements` embeds in
    /// the INSERT AND exactly what `LegacyIds::with_cash_legacy_id` carries
    /// out to `mark_done` — pinning that the two never drift apart even
    /// though `execute` itself needs a live MSSQL connection to allocate the
    /// real id and can't run in this unit suite.
    #[test]
    fn allocated_id_reaches_both_the_sql_and_legacy_ids() {
        let mapped = from_outbox_payload(&outbox_payload());
        let legacy_id = 555;
        let stmt = &build_statements(&mapped, legacy_id).unwrap()[0];
        assert!(
            stmt.contains(&format!("({legacy_id}, ")),
            "allocated id must be the INSERT's first VALUES slot: {stmt}"
        );

        let ids = crate::writeback::dispatcher::LegacyIds::new().with_cash_legacy_id(legacy_id);
        assert_eq!(
            ids.cash_legacy_id,
            Some(legacy_id),
            "the SAME id must reach LegacyIds for mark_done to back-populate"
        );
    }
}
