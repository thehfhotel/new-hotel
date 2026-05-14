//! Coupon recipes — Track G5.
//!
//! Mirrors the legacy `HT_Cupon` table from `docs/legacy-app/COMPAT_CHEATSHEET.md`
//! §`HT_Cupon` (food/breakfast coupon entitlement). One row per
//! (check-in × night × `Cin_cupon` count), allocated by iHOTEL's
//! `Module1.GEN_Cupon` at check-in time. The Track G5 canonical
//! `ht_coupons` table promotes the legacy mirror to a first-class
//! canonical owned table so receptionists can issue coupons without
//! falling back to iHOTEL.
//!
//! Two recipes (one per intent variant):
//!
//! - [`execute_issue`] — allocates a `cupon_no` under TABLOCKX+HOLDLOCK
//!   and INSERTs the legacy row. Mirrors `Module1.GEN_Cupon`'s shape:
//!   `(cupon_no, cupon_cin_no, cupon_cin_room, cupon_date,
//!     cupon_gen_date, cupon_by, cupon_print=0)`.
//! - [`execute_redeem`] — fires the same UPDATE iHOTEL emits from
//!   `Print_Report.cs:2552`: `UPDATE HT_Cupon SET cupon_print=1 WHERE
//!   cupon_no=...`. Requires `legacy_cupon_no` to be resolved — the
//!   worker keys this off `ht_coupons.legacy_cupon_no` which is
//!   back-populated after the issue recipe lands.
//!
//! Both recipes follow the project pattern from Track G4 / `room_change`:
//! pure `build_*` statement composer + thin `execute_*` wrapper that
//! holds the allocator + clock.

use crate::writeback::allocate::{allocate_cupon_no, LegacyConn};
use crate::writeback::dispatcher::{LegacyIds, ResolvedCoupon};
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{format_legacy_datetime, sql_quote};
use chrono::{DateTime, Utc};

/// Inputs for the coupon-issue recipe.
///
/// All strings are taken by reference so the recipe doesn't allocate
/// when threading them through `format!`. `cupon_no` is the freshly
/// allocated MAX+1 from [`allocate_cupon_no`].
#[derive(Debug, Clone)]
pub struct IssueInputs<'a> {
    pub cupon_no: i32,
    pub cupon_cin_no: &'a str,
    pub cupon_cin_room: &'a str,
    pub cupon_by: &'a str,
    pub created_at: DateTime<Utc>,
}

/// Build the legacy INSERT for a coupon. PURE — no I/O.
///
/// The column order mirrors the spike `COMPAT_CHEATSHEET.md` §`HT_Cupon`
/// schema:
///
/// ```text
/// cupon_no, cupon_cin_no varchar(50), cupon_cin_room varchar(50),
/// cupon_date datetime, cupon_gen_date datetime, cupon_by varchar(50),
/// cupon_print int NOT NULL DEFAULT 0
/// ```
///
/// `cupon_date` and `cupon_gen_date` both carry the issue timestamp —
/// `Module1.GEN_Cupon` always sets them to the same value (the spike
/// captures show identical values in every row).
///
/// Wrapped in `WHERE NOT EXISTS` on the freshly-allocated `cupon_no`
/// so a retry after a network drop on COMMIT doesn't double-write
/// (same idempotency contract as the payment recipe).
pub fn build_issue_statements(inputs: &IssueInputs<'_>) -> Vec<String> {
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
    let cin_no_q = sql_quote(inputs.cupon_cin_no);
    let room_q = sql_quote(inputs.cupon_cin_room);
    let by_q = sql_quote(inputs.cupon_by);
    let cupon_no = inputs.cupon_no;

    let mut statements = Vec::with_capacity(1);
    statements.push(format!(
        "INSERT INTO [HT_Cupon] ([cupon_no],[cupon_cin_no],[cupon_cin_room],\
         [cupon_date],[cupon_gen_date],[cupon_by],[cupon_print]) \
         SELECT {cupon_no},{cin_no_q},{room_q},{now_q},{now_q},{by_q},0 \
         WHERE NOT EXISTS (SELECT 1 FROM HT_Cupon WHERE cupon_no={cupon_no})"
    ));
    statements
}

/// Build the legacy UPDATE for a coupon redemption. PURE — no I/O.
///
/// Mirrors `Print_Report.cs:2552`: `UPDATE HT_Cupon SET cupon_print=1
/// WHERE cupon_no=<n>`. We target `cupon_no` (the legacy PK) rather
/// than `cupon_cin_no` because one folio may have N coupons and the
/// canonical row points to exactly one of them.
pub fn build_redeem_statements(legacy_cupon_no: i32) -> Vec<String> {
    vec![format!(
        "UPDATE HT_Cupon SET cupon_print=1 WHERE cupon_no={legacy_cupon_no}"
    )]
}

/// Issue recipe — allocate a `cupon_no` and INSERT the legacy row.
///
/// Returns `LegacyIds` carrying the allocated `cupon_no` so the
/// writeback worker's `back_populate_legacy_ids` step can stamp it
/// onto the canonical `ht_coupons.legacy_cupon_no` column.
pub async fn execute_issue(
    conn: &mut LegacyConn<'_>,
    resolved: &ResolvedCoupon,
) -> WritebackResult<LegacyIds> {
    // The recipe requires `cupon_cin_no` to be non-empty — the legacy
    // schema doesn't NULL the column and iHOTEL's filters
    // (`WHERE cupon_cin_no=...`) would silently miss our row otherwise.
    // Standalone promo coupons (no check-in) are a canonical-only
    // feature today; the writeback resolver must skip them or fall
    // back to a placeholder folio. We surface the gap loudly instead.
    if resolved.coupon_for_cin_no.is_empty() {
        return Err(WritebackError::Recipe(
            "IssueCoupon requires a non-empty coupon_for_cin_no — \
             standalone canonical-only coupons are not written back to legacy"
                .into(),
        ));
    }

    let cupon_no = allocate_cupon_no(conn).await?;
    let inputs = IssueInputs {
        cupon_no,
        cupon_cin_no: &resolved.coupon_for_cin_no,
        cupon_cin_room: &resolved.coupon_for_room_no,
        cupon_by: if resolved.issued_by.is_empty() {
            "Admin"
        } else {
            &resolved.issued_by
        },
        created_at: resolved.issued_at,
    };
    let statements = build_issue_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    Ok(LegacyIds::new().with_cupon_no(cupon_no))
}

/// Redeem recipe — flip `cupon_print=1` on the legacy row.
///
/// Requires `legacy_cupon_no` to be resolved (the issue recipe
/// back-populated it). If the worker hasn't yet stamped the legacy id
/// onto the canonical row, the redeem is a no-op on the legacy side —
/// the canonical row already carries `coupon_status='redeemed'`. The
/// scheduler-driven `mirror.rs::CuponMirrorMapper` will reconcile any
/// drift once the issue back-population completes.
pub async fn execute_redeem(
    conn: &mut LegacyConn<'_>,
    resolved: &ResolvedCoupon,
) -> WritebackResult<LegacyIds> {
    let Some(legacy_cupon_no) = resolved.legacy_cupon_no else {
        // Deferred — the canonical state already reflects the redemption
        // (status='redeemed' was set before this intent was enqueued).
        // The worker will retry once the parent IssueCoupon's
        // back-population lands. Surface as a recipe error so the
        // dispatcher returns the job to the retry queue with backoff.
        return Err(WritebackError::Recipe(format!(
            "RedeemCoupon waiting on parent IssueCoupon back-population \
             (coupon_id={coupon_id})",
            coupon_id = resolved.coupon_id
        )));
    };
    let statements = build_redeem_statements(legacy_cupon_no);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_cupon_no(legacy_cupon_no))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> IssueInputs<'static> {
        IssueInputs {
            cupon_no: 17_895,
            cupon_cin_no: "CH26-005228",
            cupon_cin_room: "508",
            cupon_by: "Admin",
            created_at: Utc.with_ymd_and_hms(2026, 5, 14, 3, 1, 11).unwrap(),
        }
    }

    /// `build_issue_statements` must be deterministic for a fixed instant
    /// — keeps the recipe a pure function the dispatcher can test
    /// without an MSSQL connection.
    #[test]
    fn build_issue_statements_is_pure_with_fixed_instant() {
        let first = build_issue_statements(&sample_inputs());
        let second = build_issue_statements(&sample_inputs());
        assert_eq!(first, second);
    }

    /// The recipe emits exactly one INSERT — no header re-aggregation,
    /// no receipt rows. Drift on this count means the recipe shape
    /// changed.
    #[test]
    fn build_issue_statements_emits_one_statement() {
        let s = build_issue_statements(&sample_inputs());
        assert_eq!(s.len(), 1);
    }

    /// The INSERT must mention `HT_Cupon`, the freshly allocated
    /// `cupon_no`, the folio key, and the operator — pinning the
    /// statement shape to the legacy schema documented in
    /// `COMPAT_CHEATSHEET.md` §`HT_Cupon`.
    #[test]
    fn issue_statement_targets_ht_cupon_with_required_columns() {
        let s = build_issue_statements(&sample_inputs());
        let sql = &s[0];
        assert!(sql.contains("HT_Cupon"), "must target HT_Cupon: {sql}");
        assert!(sql.contains("17895"), "must embed cupon_no: {sql}");
        assert!(sql.contains("'CH26-005228'"), "must embed cupon_cin_no: {sql}");
        assert!(sql.contains("'508'"), "must embed cupon_cin_room: {sql}");
        assert!(sql.contains("'Admin'"), "must embed cupon_by: {sql}");
        // `cupon_print` is initialized to 0 (unprinted). The redeem
        // recipe flips it to 1.
        assert!(sql.contains(",0"), "must initialize cupon_print=0: {sql}");
    }

    /// Idempotency guard: a retry after a transient network drop on
    /// COMMIT MUST NOT double-write the same `cupon_no`. The recipe
    /// wraps the INSERT in `WHERE NOT EXISTS` on the freshly allocated
    /// id (mirrors the payment recipe's audit-MED guard).
    #[test]
    fn issue_statement_carries_idempotency_guard() {
        let s = build_issue_statements(&sample_inputs());
        let sql = &s[0];
        assert!(
            sql.contains("WHERE NOT EXISTS"),
            "must guard against double-INSERT on retry: {sql}"
        );
        assert!(
            sql.contains("FROM HT_Cupon WHERE cupon_no=17895"),
            "guard must filter on the allocated cupon_no: {sql}"
        );
    }

    /// `build_redeem_statements` produces exactly the legacy UPDATE
    /// shape iHOTEL fires from `Print_Report.cs:2552`.
    #[test]
    fn build_redeem_statements_matches_legacy_shape() {
        let s = build_redeem_statements(17_895);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "UPDATE HT_Cupon SET cupon_print=1 WHERE cupon_no=17895");
    }

    /// Different `cupon_no` values must produce different statements —
    /// trivial but catches a refactor that accidentally hardcodes a
    /// number.
    #[test]
    fn redeem_statement_embeds_cupon_no() {
        let one = build_redeem_statements(42);
        let two = build_redeem_statements(43);
        assert_ne!(one, two);
    }
}
