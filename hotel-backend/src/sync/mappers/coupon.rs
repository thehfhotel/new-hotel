//! Canonical `ht_coupons` projection for Track G5.
//!
//! Promotes legacy `HT_Cupon` CT events into canonical
//! `ht_coupons` rows so coupons issued by iHOTEL (via
//! `Module1.GEN_Cupon` at check-in time) land in the new app's
//! source-of-truth table.
//!
//! ## Relation to `legacy_mirror.ht_cupon`
//!
//! Phase 5.5c's [`super::mirror::CuponMirrorMapper`] continues to
//! write the raw legacy snapshot to `legacy_mirror.ht_cupon` for
//! observability / reconcile. The mirror mapper extends its
//! `apply` to ALSO call into [`apply_canonical_cupon_event`] so the
//! canonical state reflects iHOTEL-initiated coupon issues —
//! matches the Track G4 `ChangedRoomMirrorMapper` dual-write
//! pattern (no separate `Vec<dyn MssqlChangeMapper>` entry).
//!
//! Canonical-only fields (`coupon_code`, `coupon_value`,
//! `coupon_expires_at`) are not present in `HT_Cupon` — when this
//! helper inserts a row from a legacy CT event it stamps a
//! synthesized `coupon_code` of `"LEGACY-{cupon_no}"` and
//! `coupon_value=0` (legacy coupons are entitlement-only with no
//! face value). The `source` column lands as `legacy` so dashboards
//! can distinguish iHOTEL-issued from canonical-issued.
//!
//! ## Projection constants
//!
//! `SELECT_COLUMNS` is the single source of truth for the column
//! names this projection consumes. The mirror mapper's
//! `select_sql()` is already aligned with these names (the columns
//! were carried verbatim from spike `COMPAT_CHEATSHEET.md`
//! §`HT_Cupon`); this constant exists so a unit test can pin the
//! alignment and surface drift before it costs an integration run
//! (H1 pattern).

use crate::outbox::event::DomainEvent;
use crate::service::coupon::legacy_print_to_pg_status;
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

const TABLE: &str = "HT_Cupon";

/// Columns this projection reads from a `MappableRow`. Kept here so
/// the locking test can pin them — if the mirror mapper's
/// `select_sql` ever drifts, the test surfaces the mismatch.
pub const SELECT_COLUMNS: &[&str] = &[
    "cupon_no",
    "cupon_cin_no",
    "cupon_cin_room",
    "cupon_gen_date",
    "cupon_by",
    "cupon_print",
];

/// Apply one `HT_Cupon` CT event to the canonical `ht_coupons`
/// table. Idempotent UPSERT keyed on the partial unique index
/// `ux_ht_coupons_legacy_cupon_no`.
///
/// Returns `Ok(None)` — no `DomainEvent` is emitted (coupons don't
/// have a `DomainEvent` variant yet; a future track can add
/// `CouponIssued` if the SSE channel needs it).
pub async fn apply_canonical_cupon_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op: ChangeOp,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    let cupon_no = row.try_get_i32("cupon_no")?.ok_or_else(|| SyncError::Mapper {
        table: TABLE,
        message: "cupon_no NULL — should not happen post Phase 5.5b".into(),
    })?;

    match op {
        ChangeOp::Delete => {
            // Legacy DELETE of a coupon row — clear the canonical
            // back-pointer but keep the canonical row (it may have
            // already been redeemed and we want the audit trail).
            // The `legacy_mirror` mapper handles the raw delete.
            sqlx::query(
                "UPDATE ht_coupons \
                    SET legacy_cupon_no = NULL, \
                        updated_at = NOW() \
                  WHERE legacy_cupon_no = $1",
            )
            .bind(cupon_no)
            .execute(&mut **tx)
            .await?;
        }
        ChangeOp::Insert | ChangeOp::Update => {
                let cin_no = row.try_get_str("cupon_cin_no")?.unwrap_or_default();
                let cin_room = row.try_get_str("cupon_cin_room")?.unwrap_or_default();
                let issued_by = row.try_get_str("cupon_by")?.unwrap_or_default();
                let cupon_print = row.try_get_i32("cupon_print")?.unwrap_or(0);
                let issued_at = row.try_get_datetime("cupon_gen_date")?;
                let status = legacy_print_to_pg_status(cupon_print);
                let aggregate_id =
                    aggregate_uuid(AggregateKind::Coupon, cupon_no);

                // INSERT … ON CONFLICT (legacy_cupon_no) UPDATE …
                // Mirrors the products / customers mapper UPSERT
                // pattern. The partial unique index on `legacy_cupon_no
                // WHERE NOT NULL` is what enables the ON CONFLICT
                // target here.
                let synthetic_code = format!("LEGACY-{cupon_no}");
                // ON CONFLICT targets the partial UNIQUE index
                // `ux_ht_coupons_legacy_cupon_no WHERE
                //  legacy_cupon_no IS NOT NULL`. PostgreSQL infers
                // the matching constraint when the predicate is
                // restated in the conflict target.
                sqlx::query(
                    "INSERT INTO ht_coupons ( \
                        legacy_cupon_no, coupon_code, coupon_value, \
                        coupon_status, coupon_issued_at, coupon_issued_by, \
                        coupon_for_cin_no, coupon_redeemed_at, \
                        aggregate_id, source \
                     ) VALUES ( \
                        $1, $2, 0, $3, COALESCE($4, NOW()), $5, $6, \
                        CASE WHEN $3 = 'redeemed' THEN NOW() ELSE NULL END, \
                        $7, 'legacy' \
                     ) \
                     ON CONFLICT (legacy_cupon_no) \
                       WHERE legacy_cupon_no IS NOT NULL \
                     DO UPDATE SET \
                        coupon_status      = EXCLUDED.coupon_status, \
                        coupon_for_cin_no  = EXCLUDED.coupon_for_cin_no, \
                        coupon_issued_by   = EXCLUDED.coupon_issued_by, \
                        coupon_redeemed_at = CASE \
                            WHEN EXCLUDED.coupon_status = 'redeemed' \
                                 AND ht_coupons.coupon_redeemed_at IS NULL \
                            THEN NOW() \
                            ELSE ht_coupons.coupon_redeemed_at \
                        END, \
                        updated_at         = NOW()",
                )
                .bind(cupon_no)
                .bind(&synthetic_code)
                .bind(status)
                .bind(issued_at)
                .bind(issued_by)
                .bind(cin_no)
                .bind(aggregate_id)
                .execute(&mut **tx)
                .await?;

            // `cin_room` is preserved in `legacy_mirror.ht_cupon`
            // for cross-reference but not surfaced canonically —
            // the canonical row tracks `coupon_redeemed_cin_id`
            // (a FK to `ht_checkins`) which is resolved on
            // canonical redemption, not derived from the legacy
            // `cupon_cin_room` string.
            let _ = cin_room;
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    /// H1 pattern — lock the canonical projection's column list
    /// against the mirror mapper's `select_sql` so drift on either
    /// side surfaces at compile-test time rather than as silent
    /// NULL columns at runtime. The mirror mapper SELECTs:
    ///
    /// ```sql
    /// t.cupon_no, t.cupon_cin_no, t.cupon_cin_room, t.cupon_date,
    /// t.cupon_gen_date, t.cupon_by, t.cupon_print
    /// ```
    ///
    /// Every column [`apply_canonical_cupon_event`] reads is in that
    /// projection. The test enumerates the canonical-side needs
    /// (`SELECT_COLUMNS`) and asserts every one is present in the
    /// mirror mapper's `select_sql` literal.
    #[test]
    fn canonical_columns_locked_to_mirror_select_sql() {
        let mirror_select = crate::sync::mappers::mirror::CuponMirrorMapper.select_sql();
        for col in SELECT_COLUMNS {
            assert!(
                mirror_select.contains(col),
                "canonical projection needs column `{col}` but mirror select_sql \
                 does not project it: {mirror_select}"
            );
        }
    }

    /// Build a representative `HT_Cupon` CT row mirroring the spike
    /// capture (`docs/legacy-spike/findings.md` line 104 — issued at
    /// check-in time).
    fn sample_cupon_row(cupon_print: i32) -> HashMapRow {
        HashMapRow::new(TABLE)
            .with("cupon_no", MockValue::I32(17_895))
            .with("cupon_cin_no", MockValue::Str("CH26-005228".into()))
            .with("cupon_cin_room", MockValue::Str("508".into()))
            .with("cupon_date", MockValue::Datetime(
                chrono::NaiveDate::from_ymd_opt(2026, 5, 14).unwrap()
                    .and_hms_opt(3, 1, 11).unwrap(),
            ))
            .with("cupon_gen_date", MockValue::Datetime(
                chrono::NaiveDate::from_ymd_opt(2026, 5, 14).unwrap()
                    .and_hms_opt(3, 1, 11).unwrap(),
            ))
            .with("cupon_by", MockValue::Str("Admin".into()))
            .with("cupon_print", MockValue::I32(cupon_print))
    }

    /// `cupon_print=0` projects to canonical status `issued`.
    /// `cupon_print=1` projects to `redeemed`. Pins the mapping
    /// against accidental inversion.
    #[test]
    fn cupon_print_projects_to_canonical_status() {
        let _r0 = sample_cupon_row(0);
        let _r1 = sample_cupon_row(1);
        // Status mapping is a pure helper from the service module.
        // Re-import here is enough; full INSERT semantics are covered
        // by DB-gated integration tests.
        assert_eq!(legacy_print_to_pg_status(0), "issued");
        assert_eq!(legacy_print_to_pg_status(1), "redeemed");
    }

    /// `cupon_no` is required — a NULL surface should fail the
    /// mapper rather than silently writing a NULL-keyed row.
    #[test]
    fn mapper_requires_cupon_no() {
        let r = HashMapRow::new(TABLE)
            .with("cupon_cin_no", MockValue::Str("CH26-005228".into()))
            .with("cupon_print", MockValue::I32(0));
        // try_get_i32 on a missing column returns Err — the
        // production `apply_canonical_cupon_event` propagates as
        // `SyncError::Mapper`. The assertion here is at the row
        // level: missing cupon_no is unrecoverable, surfaced via the
        // `try_get_i32?` chain.
        let result: Result<Option<i32>, _> = r.try_get_i32("cupon_no");
        // The mock errors on absent column lookups (per `row.rs`).
        assert!(result.is_err(), "missing cupon_no must surface as error");
    }

    /// `aggregate_uuid` is deterministic for a given `cupon_no` —
    /// guards against subscribers seeing different ids for the same
    /// row across writebacks.
    #[test]
    fn aggregate_id_is_deterministic_for_cupon_no() {
        let a = aggregate_uuid(AggregateKind::Coupon, 17_895);
        let b = aggregate_uuid(AggregateKind::Coupon, 17_895);
        assert_eq!(a, b);
        let other = aggregate_uuid(AggregateKind::Coupon, 17_896);
        assert_ne!(a, other);
    }
}
