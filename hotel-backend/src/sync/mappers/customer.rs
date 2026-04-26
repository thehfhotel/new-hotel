//! `HT_Customers` Change Tracking mapper.
//!
//! Per `docs/architecture.md` §3.6d, §11. Translates one CT row from the
//! legacy MSSQL `HT_Customers` table into:
//!
//! * a UPSERT into `public.ht_customers` (I/U), or
//! * a soft-delete via `cust_deleted_at = now()` (D, no event today),
//!
//! and emits `DomainEvent::CustomerCreated` / `CustomerModified` for I/U
//! when the row genuinely changed (idempotent skip otherwise).
//!
//! ## Column mapping
//!
//! | MSSQL `HT_Customers`    | PG `ht_customers`            |
//! |-------------------------|-------------------------------|
//! | `Cust_no`               | `legacy_cust_no` (also derives `aggregate_id`) |
//! | `Cust_name`             | `cust_firstname` (NOT NULL — empty string fallback) |
//! | `Cust_perfix`           | `cust_title` |
//! | `Cust_IDcard`           | `cust_idcard` |
//! | `Cust_Type_Main`        | `cust_type` (Thai literal preserved) |
//! | `Cust_Email`            | `cust_email` |
//! | `Cust_Add_no`           | `cust_address` (single combined column on our side; we mirror just the door-number for now — full address join lives in writeback's `customer_address::format_address`) |
//! | `Cust_Add_tel`          | `cust_phone` |
//!
//! ## Idempotency
//!
//! Before publishing an event, the mapper compares the canonical PG row
//! to the projected legacy row. If every mirrored column already matches,
//! the UPSERT runs (cheap NO-OP) but `Ok(None)` is returned so no
//! `event_log` row is written. This belt-and-suspenders us against a
//! missed `0x4E48` CONTEXT_INFO tag.
//!
//! ## Aggregate UUID
//!
//! `aggregate_id` is derived once via
//! `service::ids::aggregate_uuid(AggregateKind::Customer, cust_id)` and
//! pinned to the row. Subsequent updates reuse the same UUID so
//! subscribers can deduplicate.

use async_trait::async_trait;
use uuid::Uuid;

use crate::outbox::event::{CustomerSnapshot, DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

/// CT mapper for the legacy `HT_Customers` table.
pub struct CustomerMapper;

const TABLE: &str = "HT_Customers";

/// Columns we project into the CT JOIN. Must match the field names the
/// `apply` body reads via `try_get_str` etc.
const SELECT_COLS: &str =
    "t.Cust_no, t.Cust_name, t.Cust_perfix, t.Cust_IDcard, \
     t.Cust_Type_Main, t.Cust_Email, t.Cust_Add_no, t.Cust_Add_tel";

#[async_trait]
impl MssqlChangeMapper for CustomerMapper {
    fn table(&self) -> &'static str {
        TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // CT's primary key for HT_Customers is the SERIAL `id` integer
        // (not Cust_no — Cust_no is a unique business key, while `id`
        // is what CT keys its CHANGES projection on).
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        match op {
            ChangeOp::Insert | ChangeOp::Update => {
                let row = row.ok_or_else(|| SyncError::Mapper {
                    table: TABLE,
                    message: "I/U operation requires joined row".into(),
                })?;
                apply_upsert(tx, op, row).await
            }
            ChangeOp::Delete => {
                let row = row.ok_or_else(|| SyncError::Mapper {
                    table: TABLE,
                    message: "D operation requires PK row from CT".into(),
                })?;
                apply_soft_delete(tx, row).await
            }
        }
    }
}

/// Owned snapshot of the columns we mirror — used for idempotency check
/// and event payload construction.
#[derive(Debug, Clone, PartialEq)]
struct CustomerProjection {
    cust_no: String,
    cust_name: String,
    cust_title: Option<String>,
    cust_idcard: Option<String>,
    cust_type: Option<String>,
    cust_email: Option<String>,
    cust_address: Option<String>,
    cust_phone: Option<String>,
}

fn project(row: &dyn MappableRow) -> Result<CustomerProjection, SyncError> {
    let cust_no_opt = row.try_get_str("Cust_no")?;
    let cust_no = cust_no_opt
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "Cust_no is NULL — required PK".into(),
        })?
        .to_string();

    // cust_firstname is NOT NULL in PG; legacy can be NULL (rare). Fall
    // back to empty string so the UPSERT succeeds and the legacy
    // anomaly stays observable in PG without blocking sync.
    let cust_name = row.try_get_str("Cust_name")?.unwrap_or("").to_string();

    Ok(CustomerProjection {
        cust_no,
        cust_name,
        cust_title: row.try_get_str("Cust_perfix")?.map(str::to_string),
        cust_idcard: row.try_get_str("Cust_IDcard")?.map(str::to_string),
        cust_type: row.try_get_str("Cust_Type_Main")?.map(str::to_string),
        cust_email: row.try_get_str("Cust_Email")?.map(str::to_string),
        cust_address: row.try_get_str("Cust_Add_no")?.map(str::to_string),
        cust_phone: row.try_get_str("Cust_Add_tel")?.map(str::to_string),
    })
}

/// Existing canonical-row snapshot read back from PG for idempotency
/// comparison.
struct ExistingRow {
    cust_id: i32,
    aggregate_id: Option<Uuid>,
    cust_name: String,
    cust_title: Option<String>,
    cust_idcard: Option<String>,
    cust_type: Option<String>,
    cust_email: Option<String>,
    cust_address: Option<String>,
    cust_phone: Option<String>,
}

async fn fetch_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cust_no: &str,
) -> Result<Option<ExistingRow>, SyncError> {
    // Dynamic query (not query!) — keeps this file out of the .sqlx
    // offline cache, which would otherwise need regenerating on every
    // schema tweak during 5.x development.
    let row = sqlx::query_as::<_, (
        i32,
        Option<Uuid>,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>(
        "SELECT cust_id, aggregate_id, cust_firstname, cust_title, \
                cust_idcard, cust_type, cust_email, cust_address, cust_phone \
           FROM ht_customers \
          WHERE legacy_cust_no = $1 \
          LIMIT 1",
    )
    .bind(cust_no)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(
        |(
            cust_id,
            aggregate_id,
            cust_name,
            cust_title,
            cust_idcard,
            cust_type,
            cust_email,
            cust_address,
            cust_phone,
        )| ExistingRow {
            cust_id,
            aggregate_id,
            cust_name,
            cust_title,
            cust_idcard,
            cust_type,
            cust_email,
            cust_address,
            cust_phone,
        },
    ))
}

/// Returns true when every mirrored column already matches the legacy
/// projection. Used to skip event publication on a re-applied row.
fn matches(existing: &ExistingRow, projected: &CustomerProjection) -> bool {
    existing.cust_name == projected.cust_name
        && existing.cust_title == projected.cust_title
        && existing.cust_idcard == projected.cust_idcard
        && existing.cust_type == projected.cust_type
        && existing.cust_email == projected.cust_email
        && existing.cust_address == projected.cust_address
        && existing.cust_phone == projected.cust_phone
}

async fn apply_upsert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op: ChangeOp,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    let projected = project(row)?;
    let existing = fetch_existing(tx, &projected.cust_no).await?;

    // Idempotency — re-applied row with no change → skip event.
    if let Some(ref ex) = existing {
        if matches(ex, &projected) && ex.aggregate_id.is_some() {
            return Ok(None);
        }
    }

    // UPSERT. Two branches:
    //  - existing row → UPDATE columns, set legacy_cust_no/aggregate_id
    //    if NULL.
    //  - no existing row → INSERT new (aggregate_id derived after the
    //    INSERT once we know the SERIAL cust_id).
    let (final_cust_id, final_agg_id, was_insert) = match existing {
        Some(ex) => {
            let agg_id = ex
                .aggregate_id
                .unwrap_or_else(|| aggregate_uuid(AggregateKind::Customer, ex.cust_id));
            sqlx::query(
                "UPDATE ht_customers \
                    SET cust_firstname = $1, \
                        cust_title     = $2, \
                        cust_idcard    = $3, \
                        cust_type      = $4, \
                        cust_email     = $5, \
                        cust_address   = $6, \
                        cust_phone     = $7, \
                        legacy_cust_no = COALESCE(legacy_cust_no, $8), \
                        aggregate_id   = COALESCE(aggregate_id, $9), \
                        cust_deleted_at = NULL, \
                        updated_at     = NOW() \
                  WHERE cust_id = $10",
            )
            .bind(&projected.cust_name)
            .bind(&projected.cust_title)
            .bind(&projected.cust_idcard)
            .bind(&projected.cust_type)
            .bind(&projected.cust_email)
            .bind(&projected.cust_address)
            .bind(&projected.cust_phone)
            .bind(&projected.cust_no)
            .bind(agg_id)
            .bind(ex.cust_id)
            .execute(&mut **tx)
            .await?;
            (ex.cust_id, agg_id, false)
        }
        None => {
            // INSERT, RETURNING so we can derive aggregate_id from the
            // freshly-allocated SERIAL cust_id, then UPDATE that.
            let row = sqlx::query_as::<_, (i32,)>(
                "INSERT INTO ht_customers \
                     (cust_firstname, cust_title, cust_idcard, cust_type, \
                      cust_email, cust_address, cust_phone, legacy_cust_no) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
                 RETURNING cust_id",
            )
            .bind(&projected.cust_name)
            .bind(&projected.cust_title)
            .bind(&projected.cust_idcard)
            .bind(&projected.cust_type)
            .bind(&projected.cust_email)
            .bind(&projected.cust_address)
            .bind(&projected.cust_phone)
            .bind(&projected.cust_no)
            .fetch_one(&mut **tx)
            .await?;

            let cust_id = row.0;
            let agg_id = aggregate_uuid(AggregateKind::Customer, cust_id);
            sqlx::query("UPDATE ht_customers SET aggregate_id = $1 WHERE cust_id = $2")
                .bind(agg_id)
                .bind(cust_id)
                .execute(&mut **tx)
                .await?;
            (cust_id, agg_id, true)
        }
    };
    let _ = final_cust_id; // currently unused beyond UPSERT, keep for clarity.

    let event = build_event(op, was_insert, final_agg_id, &projected);
    Ok(Some(event))
}

async fn apply_soft_delete(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    // For D, CT only carries the PK columns from the projection (no
    // joined row data). The watcher embeds the legacy SERIAL `id` PK
    // and, when available, the unique business key `Cust_no` so we can
    // resolve the canonical row even if the I-event-driven UPSERT had
    // not yet populated `legacy_cust_no` (a benign race window of one
    // tick).
    //
    // Resolution order:
    //   1. Try `Cust_no` if the D-row carries it (preferred — matches
    //      the partial unique index on `legacy_cust_no`).
    //   2. Fall back to the legacy `id` (numeric); the row may not yet
    //      have a column on our side that mirrors it, in which case
    //      this is a no-op tombstone and the next mapper tick will
    //      reconcile via the I/U path.
    //
    // Per spec: emit no DomainEvent for D in 5.2 (no UI subscriber yet).
    let _ = row.try_get_i32("id"); // surface the column existence; not used for resolution today.
    let cust_no_opt = row.try_get_str("Cust_no").unwrap_or(None);
    if let Some(cust_no) = cust_no_opt {
        sqlx::query(
            "UPDATE ht_customers \
                SET cust_deleted_at = NOW(), \
                    updated_at      = NOW() \
              WHERE legacy_cust_no = $1",
        )
        .bind(cust_no)
        .execute(&mut **tx)
        .await?;
    }
    // No Cust_no on the D row → silent no-op; the row never existed in
    // canonical PG, or its legacy_cust_no will land on the next I/U
    // tick and a subsequent D will succeed.
    Ok(None)
}

/// Build the appropriate `CustomerCreated` / `CustomerModified` event
/// for a successful upsert.
fn build_event(
    op: ChangeOp,
    was_insert: bool,
    agg_id: Uuid,
    projected: &CustomerProjection,
) -> DomainEvent {
    use crate::domain::customer::CustomerType;

    // Map MSSQL Cust_Type_Main Thai literal back to canonical enum.
    // Unknown → Other (mirrors how the writeback path treats it).
    let customer_type = match projected.cust_type.as_deref() {
        Some(s) if s == CustomerType::Individual.legacy_literal() => {
            CustomerType::Individual
        }
        Some(s) if s == CustomerType::Company.legacy_literal() => CustomerType::Company,
        Some(s) if s == CustomerType::Government.legacy_literal() => {
            CustomerType::Government
        }
        _ => CustomerType::Other,
    };

    let snapshot = CustomerSnapshot {
        id: agg_id,
        legacy_cust_no: Some(projected.cust_no.clone()),
        name: projected.cust_name.clone(),
        customer_type,
        phone: projected.cust_phone.clone(),
    };

    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };

    if was_insert || matches!(op, ChangeOp::Insert) {
        DomainEvent::CustomerCreated {
            id: agg_id,
            source,
            snapshot,
        }
    } else {
        DomainEvent::CustomerModified {
            id: agg_id,
            source,
            // 5.2 doesn't try to compute a precise diff — the snapshot
            // payload carries the new state, and `changed_fields` is
            // best-effort. Leave empty so subscribers fall back to
            // re-fetching the canonical row.
            changed_fields: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    fn make_row_full(cust_no: &str, name: &str) -> HashMapRow {
        HashMapRow::new(TABLE)
            .with("Cust_no", MockValue::Str(cust_no.into()))
            .with("Cust_name", MockValue::Str(name.into()))
            .with("Cust_perfix", MockValue::Str("นาย".into()))
            .with("Cust_IDcard", MockValue::Str("***REMOVED***90123".into()))
            .with(
                "Cust_Type_Main",
                MockValue::Str("บุคคลธรรมดา".into()),
            )
            .with("Cust_Email", MockValue::Str("a@b.co".into()))
            .with("Cust_Add_no", MockValue::Str("123/4".into()))
            .with("Cust_Add_tel", MockValue::Str("08***REMOVED***".into()))
    }

    #[test]
    fn project_extracts_all_columns_from_full_row() {
        let row = make_row_full("C00001", "ทดสอบ");
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_no, "C00001");
        assert_eq!(p.cust_name, "ทดสอบ");
        assert_eq!(p.cust_title.as_deref(), Some("นาย"));
        assert_eq!(p.cust_idcard.as_deref(), Some("***REMOVED***90123"));
        assert_eq!(p.cust_type.as_deref(), Some("บุคคลธรรมดา"));
        assert_eq!(p.cust_email.as_deref(), Some("a@b.co"));
        assert_eq!(p.cust_address.as_deref(), Some("123/4"));
        assert_eq!(p.cust_phone.as_deref(), Some("08***REMOVED***"));
    }

    #[test]
    fn project_tolerates_null_optional_columns() {
        let row = HashMapRow::new(TABLE)
            .with("Cust_no", MockValue::Str("C00099".into()))
            .with("Cust_name", MockValue::Str("Anon".into()))
            .with("Cust_perfix", MockValue::Null)
            .with("Cust_IDcard", MockValue::Null)
            .with("Cust_Type_Main", MockValue::Null)
            .with("Cust_Email", MockValue::Null)
            .with("Cust_Add_no", MockValue::Null)
            .with("Cust_Add_tel", MockValue::Null);
        let p = project(&row).expect("nullable cols must project to None");
        assert_eq!(p.cust_no, "C00099");
        assert_eq!(p.cust_name, "Anon");
        assert!(p.cust_title.is_none());
        assert!(p.cust_phone.is_none());
    }

    #[test]
    fn project_falls_back_to_empty_name_on_null_legacy_cust_name() {
        let row = HashMapRow::new(TABLE)
            .with("Cust_no", MockValue::Str("C99999".into()))
            .with("Cust_name", MockValue::Null)
            .with("Cust_perfix", MockValue::Null)
            .with("Cust_IDcard", MockValue::Null)
            .with("Cust_Type_Main", MockValue::Null)
            .with("Cust_Email", MockValue::Null)
            .with("Cust_Add_no", MockValue::Null)
            .with("Cust_Add_tel", MockValue::Null);
        let p = project(&row).expect("legacy NULL Cust_name must NOT abort");
        assert_eq!(p.cust_name, "");
    }

    #[test]
    fn project_errors_when_cust_no_is_null() {
        let row = HashMapRow::new(TABLE)
            .with("Cust_no", MockValue::Null)
            .with("Cust_name", MockValue::Str("x".into()))
            .with("Cust_perfix", MockValue::Null)
            .with("Cust_IDcard", MockValue::Null)
            .with("Cust_Type_Main", MockValue::Null)
            .with("Cust_Email", MockValue::Null)
            .with("Cust_Add_no", MockValue::Null)
            .with("Cust_Add_tel", MockValue::Null);
        let err = project(&row).expect_err("NULL Cust_no must be loud");
        assert!(err.to_string().contains("Cust_no"));
    }

    /// Two projections with identical content compare equal — the
    /// idempotency check relies on this.
    #[test]
    fn matches_returns_true_for_identical_projections() {
        let p = CustomerProjection {
            cust_no: "C1".into(),
            cust_name: "n".into(),
            cust_title: Some("t".into()),
            cust_idcard: Some("i".into()),
            cust_type: Some("ty".into()),
            cust_email: Some("e".into()),
            cust_address: Some("a".into()),
            cust_phone: Some("ph".into()),
        };
        let ex = ExistingRow {
            cust_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cust_name: "n".into(),
            cust_title: Some("t".into()),
            cust_idcard: Some("i".into()),
            cust_type: Some("ty".into()),
            cust_email: Some("e".into()),
            cust_address: Some("a".into()),
            cust_phone: Some("ph".into()),
        };
        assert!(matches(&ex, &p));
    }

    #[test]
    fn matches_returns_false_when_phone_differs() {
        let p = CustomerProjection {
            cust_no: "C1".into(),
            cust_name: "n".into(),
            cust_title: None,
            cust_idcard: None,
            cust_type: None,
            cust_email: None,
            cust_address: None,
            cust_phone: Some("0888".into()),
        };
        let ex = ExistingRow {
            cust_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            cust_name: "n".into(),
            cust_title: None,
            cust_idcard: None,
            cust_type: None,
            cust_email: None,
            cust_address: None,
            cust_phone: Some("0999".into()),
        };
        assert!(!matches(&ex, &p));
    }

    #[test]
    fn build_event_for_insert_emits_customer_created() {
        let p = CustomerProjection {
            cust_no: "C77".into(),
            cust_name: "Alice".into(),
            cust_title: None,
            cust_idcard: None,
            cust_type: Some("บุคคลธรรมดา".into()),
            cust_email: None,
            cust_address: None,
            cust_phone: Some("0801112222".into()),
        };
        let agg = aggregate_uuid(AggregateKind::Customer, 77);
        let event = build_event(ChangeOp::Insert, true, agg, &p);
        assert_eq!(event.type_name(), "CustomerCreated");
        assert_eq!(event.aggregate_id(), agg);
    }

    #[test]
    fn build_event_for_update_emits_customer_modified() {
        let p = CustomerProjection {
            cust_no: "C77".into(),
            cust_name: "Alice".into(),
            cust_title: None,
            cust_idcard: None,
            cust_type: Some("บริษัท".into()),
            cust_email: None,
            cust_address: None,
            cust_phone: None,
        };
        let agg = aggregate_uuid(AggregateKind::Customer, 77);
        let event = build_event(ChangeOp::Update, false, agg, &p);
        assert_eq!(event.type_name(), "CustomerModified");
        assert_eq!(event.aggregate_id(), agg);
    }

    #[test]
    fn build_event_classifies_unknown_cust_type_as_other() {
        let p = CustomerProjection {
            cust_no: "C77".into(),
            cust_name: "X".into(),
            cust_title: None,
            cust_idcard: None,
            cust_type: Some("???".into()),
            cust_email: None,
            cust_address: None,
            cust_phone: None,
        };
        let event = build_event(
            ChangeOp::Insert,
            true,
            aggregate_uuid(AggregateKind::Customer, 1),
            &p,
        );
        // Pull the snapshot back out via JSON round-trip — keeps the
        // assertion independent of the enum's debug format.
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(
            json["data"]["snapshot"]["customer_type"],
            serde_json::Value::String("other".into())
        );
    }

    #[test]
    fn customer_mapper_advertises_correct_table_and_select() {
        let m = CustomerMapper;
        assert_eq!(m.table(), "HT_Customers");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("Cust_no"));
        assert!(m.select_sql().contains("Cust_Type_Main"));
        // No leading SELECT / FROM — caller wraps it.
        assert!(!m.select_sql().to_uppercase().starts_with("SELECT"));
    }
}
