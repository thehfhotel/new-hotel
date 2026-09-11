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
//! Track E2 (`docs/coexistence/audit-2026-05-13.md` T1 HIGH-2 + T2
//! HIGH-4) widened the projection from 8 columns to the full legacy
//! surface so corporate-invoice, RR.4-export, and Module1.UPDATE_MONEY
//! debt-balance flows have canonical PG state to read from.
//!
//! | MSSQL `HT_Customers`    | PG `ht_customers`            |
//! |-------------------------|-------------------------------|
//! | `id`                    | `legacy_id` (migration 055 — CT D-rows carry ONLY this key, so hard-deletes resolve by it) |
//! | `Cust_no`               | `legacy_cust_no` (also derives `aggregate_id`) |
//! | `Cust_name`             | `cust_firstname` (NOT NULL — empty string fallback) |
//! | `Cust_name2`            | `cust_name2` (English/secondary name; FrmReportRR4) |
//! | `Cust_perfix`           | `cust_title` |
//! | `Cust_sex`              | `cust_sex` |
//! | `Cust_IDcard`           | `cust_idcard` |
//! | `Cust_Type`             | `cust_price_tier` (rate-tier label, e.g. `'ราคาปกติ'`) |
//! | `Cust_Type_Main`        | `cust_type` (Thai customer-category literal preserved) |
//! | `Cust_Email`            | `cust_email` |
//! | `Cust_Add_no`           | `cust_add_no` (door number; also kept on the legacy `cust_address` mirror column for backwards compat) |
//! | `Cust_Add_moo`          | `cust_add_moo` |
//! | `Cust_Add_soi`          | `cust_add_soi` |
//! | `Cust_Add_road`         | `cust_add_road` |
//! | `Cust_Add_tambon`       | `cust_add_tambon` |
//! | `Cust_Add_ampore`       | `cust_add_ampore` |
//! | `Cust_Add_province`     | `cust_add_province` |
//! | `Cust_Add_code`         | `cust_add_code` (postal code) |
//! | `Cust_Add_tel`          | `cust_phone` |
//! | `Cust_Add_fax`          | `cust_add_fax` |
//! | `Cust_Work_Name`        | `cust_work_name` |
//! | `Cust_Work_no`          | `cust_work_no` |
//! | `Cust_Work_moo`         | `cust_work_moo` |
//! | `Cust_Work_soi`         | `cust_work_soi` |
//! | `Cust_Work_road`        | `cust_work_road` |
//! | `Cust_Work_tambon`      | `cust_work_tambon` |
//! | `Cust_Work_ampore`      | `cust_work_ampore` |
//! | `Cust_Work_province`    | `cust_work_province` |
//! | `Cust_Work_code`        | `cust_work_code` |
//! | `Cust_Work_tel`         | `cust_work_tel` |
//! | `Cust_Work_fax`         | `cust_work_fax` |
//! | `Cust_Work_Tax`         | `cust_work_tax` (corporate tax id) |
//! | `Cust_Last_Change`      | `cust_last_change` (legacy edit timestamp) |
//! | `Cust_Contry`           | `cust_contry` (sic — preserved legacy spelling) |
//! | `Cust_Price_Over`       | `cust_price_over` (running debt balance — read-only mirror; Module1.UPDATE_MONEY writeback deferred to Track G) |
//!
//! ## Idempotency
//!
//! Before publishing an event, the mapper compares the canonical PG row
//! to the projected legacy row. If every mirrored column already matches,
//! the UPSERT runs (cheap NO-OP) but `Ok(None)` is returned so no
//! `event_log` row is written. This idempotency IS the echo-absorption
//! mechanism: CT rows produced by our own writeback re-enter here and
//! converge to a no-op. (`SET CONTEXT_INFO` never populated
//! `SYS_CHANGE_CONTEXT`, so there was never a SQL-layer filter — see
//! `db/mssql_session.rs`.)
//!
//! The comparison must cover every reconcile-hash input, or a row can
//! hash as unconverged while the gate reports "no change" — and since
//! `force_converge_reconcile_row` repairs by re-driving this same gate,
//! the self-heal path dies with it. `cust_address` was that gap until
//! 2026-07-28; see the field doc on [`ExistingEqualityKeys`].
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
use crate::sync::gate_guard::{self, GateField, HashInput, HashInputContract};
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

/// CT mapper for the legacy `HT_Customers` table.
pub struct CustomerMapper;

pub(crate) const TABLE: &str = "HT_Customers";

/// Column list (without aliases or commas) used by the eager-mirror
/// fetch in [`crate::sync::mappers::checkin`] when it pulls a single
/// `HT_Customers` row by `Cust_no`. Kept here so the projection layer
/// (see [`project`]) and the eager fetcher always reach for the same
/// set of columns.
pub(crate) const EAGER_FETCH_COLUMNS: &[&str] = &[
    // Legacy SERIAL PK — persisted as ht_customers.legacy_id (migration
    // 055) so CT D-rows, which carry ONLY this key, can resolve the
    // canonical row for the soft delete.
    "id",
    "Cust_no",
    "Cust_name",
    "Cust_name2",
    "Cust_perfix",
    "Cust_sex",
    "Cust_IDcard",
    "Cust_Type",
    "Cust_Type_Main",
    "Cust_Email",
    "Cust_Add_no",
    "Cust_Add_moo",
    "Cust_Add_soi",
    "Cust_Add_road",
    "Cust_Add_tambon",
    "Cust_Add_ampore",
    "Cust_Add_province",
    "Cust_Add_code",
    "Cust_Add_tel",
    "Cust_Add_fax",
    "Cust_Work_Name",
    "Cust_Work_no",
    "Cust_Work_moo",
    "Cust_Work_soi",
    "Cust_Work_road",
    "Cust_Work_tambon",
    "Cust_Work_ampore",
    "Cust_Work_province",
    "Cust_Work_code",
    "Cust_Work_tel",
    "Cust_Work_fax",
    "Cust_Work_Tax",
    "Cust_Last_Change",
    "Cust_Contry",
    "Cust_Price_Over",
];

/// Columns we project into the CT JOIN. Must match the field names the
/// `apply` body reads via `try_get_str` etc.
const SELECT_COLS: &str =
    "t.Cust_no, t.Cust_name, t.Cust_name2, t.Cust_perfix, t.Cust_sex, \
     t.Cust_IDcard, t.Cust_Type, t.Cust_Type_Main, t.Cust_Email, \
     t.Cust_Add_no, t.Cust_Add_moo, t.Cust_Add_soi, t.Cust_Add_road, \
     t.Cust_Add_tambon, t.Cust_Add_ampore, t.Cust_Add_province, \
     t.Cust_Add_code, t.Cust_Add_tel, t.Cust_Add_fax, \
     t.Cust_Work_Name, t.Cust_Work_no, t.Cust_Work_moo, t.Cust_Work_soi, \
     t.Cust_Work_road, t.Cust_Work_tambon, t.Cust_Work_ampore, \
     t.Cust_Work_province, t.Cust_Work_code, t.Cust_Work_tel, \
     t.Cust_Work_fax, t.Cust_Work_Tax, t.Cust_Last_Change, \
     t.Cust_Contry, t.Cust_Price_Over";

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
/// and event payload construction. Track E2 widened from 8 to 33 columns
/// to cover the full legacy `HT_Customers` surface (T1 HIGH-2 / T2 HIGH-4).
#[derive(Debug, Clone, PartialEq)]
struct CustomerProjection {
    /// Legacy `HT_Customers.id` (SERIAL PK). Persisted as
    /// `ht_customers.legacy_id` (migration 055) so CT D-rows — which
    /// carry ONLY this key — can resolve the canonical row. `None` for
    /// fixture rows that pre-date the widening; the UPSERT writes it
    /// through `COALESCE` so a transient `None` never blanks a stored
    /// value.
    legacy_id: Option<i32>,
    cust_no: String,
    cust_name: String,
    cust_name2: Option<String>,
    cust_title: Option<String>,
    cust_sex: Option<String>,
    cust_idcard: Option<String>,
    cust_price_tier: Option<String>,
    cust_type: Option<String>,
    cust_email: Option<String>,
    /// Door number from `Cust_Add_no`. Also written to legacy compat
    /// column `cust_address` for callers still reading the collapsed
    /// single-line address.
    cust_add_no: Option<String>,
    cust_add_moo: Option<String>,
    cust_add_soi: Option<String>,
    cust_add_road: Option<String>,
    cust_add_tambon: Option<String>,
    cust_add_ampore: Option<String>,
    cust_add_province: Option<String>,
    cust_add_code: Option<String>,
    /// Phone (`Cust_Add_tel`). Continues to populate `cust_phone`.
    cust_phone: Option<String>,
    cust_add_fax: Option<String>,
    cust_work_name: Option<String>,
    cust_work_no: Option<String>,
    cust_work_moo: Option<String>,
    cust_work_soi: Option<String>,
    cust_work_road: Option<String>,
    cust_work_tambon: Option<String>,
    cust_work_ampore: Option<String>,
    cust_work_province: Option<String>,
    cust_work_code: Option<String>,
    cust_work_tel: Option<String>,
    cust_work_fax: Option<String>,
    cust_work_tax: Option<String>,
    cust_last_change: Option<chrono::NaiveDateTime>,
    cust_contry: Option<String>,
    /// Running debt balance (`Cust_Price_Over`, `float NOT NULL DEFAULT 0`
    /// in legacy). Mirrored read-only — Module1.UPDATE_MONEY writeback
    /// is deferred to Track G.
    cust_price_over: Option<f64>,
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
        // `.ok().flatten()` — fixture rows / pre-055 loads may not carry
        // the `id` cell at all; treat "missing column" like NULL. The
        // watcher's CT projection always aliases the PK in (see
        // `bin/sync.rs::build_materialised_row`).
        legacy_id: row.try_get_i32("id").ok().flatten(),
        cust_no,
        cust_name,
        cust_name2: row.try_get_str("Cust_name2")?.map(str::to_string),
        cust_title: row.try_get_str("Cust_perfix")?.map(str::to_string),
        cust_sex: row.try_get_str("Cust_sex")?.map(str::to_string),
        cust_idcard: row.try_get_str("Cust_IDcard")?.map(str::to_string),
        cust_price_tier: row.try_get_str("Cust_Type")?.map(str::to_string),
        cust_type: row.try_get_str("Cust_Type_Main")?.map(str::to_string),
        cust_email: row.try_get_str("Cust_Email")?.map(str::to_string),
        cust_add_no: row.try_get_str("Cust_Add_no")?.map(str::to_string),
        cust_add_moo: row.try_get_str("Cust_Add_moo")?.map(str::to_string),
        cust_add_soi: row.try_get_str("Cust_Add_soi")?.map(str::to_string),
        cust_add_road: row.try_get_str("Cust_Add_road")?.map(str::to_string),
        cust_add_tambon: row.try_get_str("Cust_Add_tambon")?.map(str::to_string),
        cust_add_ampore: row.try_get_str("Cust_Add_ampore")?.map(str::to_string),
        cust_add_province: row.try_get_str("Cust_Add_province")?.map(str::to_string),
        cust_add_code: row.try_get_str("Cust_Add_code")?.map(str::to_string),
        cust_phone: row.try_get_str("Cust_Add_tel")?.map(str::to_string),
        cust_add_fax: row.try_get_str("Cust_Add_fax")?.map(str::to_string),
        cust_work_name: row.try_get_str("Cust_Work_Name")?.map(str::to_string),
        cust_work_no: row.try_get_str("Cust_Work_no")?.map(str::to_string),
        cust_work_moo: row.try_get_str("Cust_Work_moo")?.map(str::to_string),
        cust_work_soi: row.try_get_str("Cust_Work_soi")?.map(str::to_string),
        cust_work_road: row.try_get_str("Cust_Work_road")?.map(str::to_string),
        cust_work_tambon: row.try_get_str("Cust_Work_tambon")?.map(str::to_string),
        cust_work_ampore: row.try_get_str("Cust_Work_ampore")?.map(str::to_string),
        cust_work_province: row.try_get_str("Cust_Work_province")?.map(str::to_string),
        cust_work_code: row.try_get_str("Cust_Work_code")?.map(str::to_string),
        cust_work_tel: row.try_get_str("Cust_Work_tel")?.map(str::to_string),
        cust_work_fax: row.try_get_str("Cust_Work_fax")?.map(str::to_string),
        cust_work_tax: row.try_get_str("Cust_Work_Tax")?.map(str::to_string),
        cust_last_change: row.try_get_datetime("Cust_Last_Change")?,
        cust_contry: row.try_get_str("Cust_Contry")?.map(str::to_string),
        cust_price_over: row.try_get_f64("Cust_Price_Over")?,
    })
}

/// Existing canonical-row snapshot read back from PG for idempotency
/// comparison. Track E2 expanded this to mirror every column the mapper
/// writes so a re-applied row really is a no-op.
///
/// The struct uses a single `eq_keys` tuple via the [`equality_keys`]
/// helper so we don't have to enumerate every column twice (once here,
/// once in `matches`).
struct ExistingRow {
    cust_id: i32,
    aggregate_id: Option<Uuid>,
    /// Stored `legacy_id` (migration 055). Outside `keys` because the
    /// comparison is guarded (see [`matches`]): a `None` projection
    /// must not force a re-apply, but a row whose stored value is still
    /// NULL must re-apply once so the backfill lands.
    legacy_id: Option<i32>,
    keys: ExistingEqualityKeys,
}

/// Subset of `ht_customers` used for idempotency comparison. Keep the
/// field order identical to [`projection_equality_keys`] so the two
/// tuples compare directly.
#[derive(Debug, Clone, PartialEq)]
struct ExistingEqualityKeys {
    cust_name: String,
    cust_name2: Option<String>,
    cust_title: Option<String>,
    cust_sex: Option<String>,
    cust_idcard: Option<String>,
    cust_price_tier: Option<String>,
    cust_type: Option<String>,
    cust_email: Option<String>,
    cust_add_no: Option<String>,
    /// Legacy single-line mirror of `cust_add_no`. Compared since
    /// 2026-07-28 — same structural class as the d09e756 checkin-gate
    /// gap: a reconcile-hash input invisible to the skip comparator.
    ///
    /// The UPSERT writes the two in lock-step (`cust_address = $9`, the
    /// `cust_add_no` bind), so a mapper-written row can never diverge —
    /// but other canonical writers break the invariant. The Thai-ID
    /// check-in prefill (`repository::customer::update`) stores the full
    /// unsplit address here and leaves `cust_add_no` alone; historical
    /// backfills and pre-lock-step rows left their own residue. Because
    /// `scheduler::sync::customer_canonical_hash` reads `cust_address`
    /// on the canonical side and `Cust_Add_no` on the MSSQL side, such a
    /// row hashes as unconverged while this gate reported "no change" —
    /// and `force_converge_reconcile_row` repairs by re-driving this
    /// same gate, so self-heal was disabled along with it.
    ///
    /// Compared UNGUARDED (contrast `legacy_id` on [`ExistingRow`]):
    /// neither branch COALESCEs it, so one re-apply always converges and
    /// a `None` projection must be allowed to blank a stale value.
    cust_address: Option<String>,
    cust_add_moo: Option<String>,
    cust_add_soi: Option<String>,
    cust_add_road: Option<String>,
    cust_add_tambon: Option<String>,
    cust_add_ampore: Option<String>,
    cust_add_province: Option<String>,
    cust_add_code: Option<String>,
    cust_phone: Option<String>,
    cust_add_fax: Option<String>,
    cust_work_name: Option<String>,
    cust_work_no: Option<String>,
    cust_work_moo: Option<String>,
    cust_work_soi: Option<String>,
    cust_work_road: Option<String>,
    cust_work_tambon: Option<String>,
    cust_work_ampore: Option<String>,
    cust_work_province: Option<String>,
    cust_work_code: Option<String>,
    cust_work_tel: Option<String>,
    cust_work_fax: Option<String>,
    cust_work_tax: Option<String>,
    cust_last_change: Option<chrono::NaiveDateTime>,
    cust_contry: Option<String>,
    /// Decimal compared as `f64` bit-identical via `to_bits` — see
    /// [`matches`] which uses `==` on the `Option<f64>` wrapper. The
    /// `Cust_Price_Over` column defaults to 0.0 in legacy.
    cust_price_over: Option<f64>,
}

/// The idempotency gate, as a table of NAMED comparators.
///
/// [`matches`] is nothing but `.all()` over this table, so removing a
/// name removes the comparison — there is no separate list that can
/// drift away from the behaviour. Names are the CANONICAL (PG) column,
/// which is also what `scheduler::sync::customer_canonical_hash` reads,
/// so [`HASH_INPUTS`] can cite them directly. See
/// [`crate::sync::gate_guard`] for the invariant this exists to keep.
const GATE_FIELDS: [GateField<ExistingRow, CustomerProjection>; 35] = [
    // -- identity / name ---------------------------------------------
    GateField {
        name: "cust_firstname",
        guarded: false,
        matches: |ex, p| ex.keys.cust_name == p.cust_name,
    },
    GateField {
        name: "cust_name2",
        guarded: false,
        matches: |ex, p| ex.keys.cust_name2 == p.cust_name2,
    },
    GateField {
        name: "cust_title",
        guarded: false,
        matches: |ex, p| ex.keys.cust_title == p.cust_title,
    },
    GateField {
        name: "cust_sex",
        guarded: false,
        matches: |ex, p| ex.keys.cust_sex == p.cust_sex,
    },
    GateField {
        name: "cust_idcard",
        guarded: false,
        matches: |ex, p| ex.keys.cust_idcard == p.cust_idcard,
    },
    GateField {
        name: "cust_price_tier",
        guarded: false,
        matches: |ex, p| ex.keys.cust_price_tier == p.cust_price_tier,
    },
    GateField {
        name: "cust_type",
        guarded: false,
        matches: |ex, p| ex.keys.cust_type == p.cust_type,
    },
    GateField {
        name: "cust_email",
        guarded: false,
        matches: |ex, p| ex.keys.cust_email == p.cust_email,
    },
    // -- home address ------------------------------------------------
    GateField {
        name: "cust_add_no",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_no == p.cust_add_no,
    },
    // Legacy single-line mirror. Compares against the SAME projection
    // field as `cust_add_no` because the UPSERT binds $9 to both columns
    // (see UPDATE_SQL / INSERT_SQL) — the lock-step write is why
    // `HASH_INPUTS`'s `cust_address` entry names both terms.
    GateField {
        name: "cust_address",
        guarded: false,
        matches: |ex, p| ex.keys.cust_address == p.cust_add_no,
    },
    GateField {
        name: "cust_add_moo",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_moo == p.cust_add_moo,
    },
    GateField {
        name: "cust_add_soi",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_soi == p.cust_add_soi,
    },
    GateField {
        name: "cust_add_road",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_road == p.cust_add_road,
    },
    GateField {
        name: "cust_add_tambon",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_tambon == p.cust_add_tambon,
    },
    GateField {
        name: "cust_add_ampore",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_ampore == p.cust_add_ampore,
    },
    GateField {
        name: "cust_add_province",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_province == p.cust_add_province,
    },
    GateField {
        name: "cust_add_code",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_code == p.cust_add_code,
    },
    GateField {
        name: "cust_phone",
        guarded: false,
        matches: |ex, p| ex.keys.cust_phone == p.cust_phone,
    },
    GateField {
        name: "cust_add_fax",
        guarded: false,
        matches: |ex, p| ex.keys.cust_add_fax == p.cust_add_fax,
    },
    // -- work address ------------------------------------------------
    GateField {
        name: "cust_work_name",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_name == p.cust_work_name,
    },
    GateField {
        name: "cust_work_no",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_no == p.cust_work_no,
    },
    GateField {
        name: "cust_work_moo",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_moo == p.cust_work_moo,
    },
    GateField {
        name: "cust_work_soi",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_soi == p.cust_work_soi,
    },
    GateField {
        name: "cust_work_road",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_road == p.cust_work_road,
    },
    GateField {
        name: "cust_work_tambon",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_tambon == p.cust_work_tambon,
    },
    GateField {
        name: "cust_work_ampore",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_ampore == p.cust_work_ampore,
    },
    GateField {
        name: "cust_work_province",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_province == p.cust_work_province,
    },
    GateField {
        name: "cust_work_code",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_code == p.cust_work_code,
    },
    GateField {
        name: "cust_work_tel",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_tel == p.cust_work_tel,
    },
    GateField {
        name: "cust_work_fax",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_fax == p.cust_work_fax,
    },
    GateField {
        name: "cust_work_tax",
        guarded: false,
        matches: |ex, p| ex.keys.cust_work_tax == p.cust_work_tax,
    },
    // -- misc ---------------------------------------------------------
    GateField {
        name: "cust_last_change",
        guarded: false,
        matches: |ex, p| ex.keys.cust_last_change == p.cust_last_change,
    },
    GateField {
        name: "cust_contry",
        guarded: false,
        matches: |ex, p| ex.keys.cust_contry == p.cust_contry,
    },
    GateField {
        name: "cust_price_over",
        guarded: false,
        matches: |ex, p| ex.keys.cust_price_over == p.cust_price_over,
    },
    // Guarded (migration 055): a `None` projection (sparse fixture /
    // pre-widening load) must not force a re-apply, but a `Some`
    // projection against a still-NULL stored value MUST mismatch once so
    // the UPSERT backfills the column.
    GateField {
        name: "legacy_id",
        guarded: true,
        matches: |ex, p| p.legacy_id.is_none() || ex.legacy_id == p.legacy_id,
    },
];

/// Gate term names, for [`crate::sync::gate_guard::reconcile_entity_contracts`].
pub(crate) fn gate_field_names() -> Vec<&'static str> {
    gate_guard::gate_field_names(&GATE_FIELDS)
}

/// The inputs `scheduler::sync::customer_canonical_hash` consumes, as a
/// descriptor table over the SAME projection the gate compares.
///
/// Order IS the hash-body order — `hash_body` joins these with `|` and
/// the result must stay byte-identical to the pre-table `format!`. That
/// is pinned by `customers_hash_bytes_unchanged_for_golden_inputs`.
const HASH_INPUTS: [HashInput<CustomerProjection>; 6] = [
    HashInput {
        name: "legacy_cust_no",
        // Row identity: `fetch_existing` SELECTs `WHERE legacy_cust_no =
        // $1`, so a changed `Cust_no` resolves a different canonical row
        // (or none → INSERT) instead of reaching any comparator.
        gated_by: &[],
        segmented: true,
        lookup_key: true,
        segment: |p| p.cust_no.clone(),
        mutate: |p| p.cust_no = "C99999".into(),
    },
    HashInput {
        name: "cust_firstname",
        gated_by: &["cust_firstname"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.cust_name.clone(),
        mutate: |p| p.cust_name = "เปลี่ยนชื่อ".into(),
    },
    HashInput {
        name: "cust_type",
        gated_by: &["cust_type"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.cust_type.clone().unwrap_or_default(),
        mutate: |p| p.cust_type = Some("นิติบุคคล".into()),
    },
    HashInput {
        name: "cust_phone",
        gated_by: &["cust_phone"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.cust_phone.clone().unwrap_or_default(),
        mutate: |p| p.cust_phone = Some("0899999999".into()),
    },
    HashInput {
        name: "cust_idcard",
        gated_by: &["cust_idcard"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.cust_idcard.clone().unwrap_or_default(),
        mutate: |p| p.cust_idcard = Some("9999999999999".into()),
    },
    HashInput {
        name: "cust_address",
        // ALIAS, declared explicitly (2026-07-28 incident). The hash
        // reads canonical `cust_address` on the PG side and
        // `Cust_Add_no` on the MSSQL side; the UPSERT writes the two in
        // lock-step from ONE projection field, so both gate terms cover
        // this input and both must exist.
        gated_by: &["cust_add_no", "cust_address"],
        segmented: true,
        lookup_key: false,
        segment: |p| p.cust_add_no.clone().unwrap_or_default(),
        mutate: |p| p.cust_add_no = Some("999/9".into()),
    },
];

/// Name-level hash contract, for
/// [`crate::sync::gate_guard::reconcile_entity_contracts`].
pub(crate) fn hash_input_contract() -> Vec<HashInputContract> {
    gate_guard::hash_input_contracts(&HASH_INPUTS)
}

/// Render the `customers` reconcile-hash body for a projection, from
/// [`HASH_INPUTS`]. Test-only: production hashes both sides from loose
/// column values in `scheduler::sync`, and the golden-vector test pins
/// this against that function byte-for-byte.
#[cfg(test)]
fn hash_body(p: &CustomerProjection) -> String {
    gate_guard::hash_body(&HASH_INPUTS, p)
}

#[cfg(test)]
fn projection_equality_keys(p: &CustomerProjection) -> ExistingEqualityKeys {
    ExistingEqualityKeys {
        cust_name: p.cust_name.clone(),
        cust_name2: p.cust_name2.clone(),
        cust_title: p.cust_title.clone(),
        cust_sex: p.cust_sex.clone(),
        cust_idcard: p.cust_idcard.clone(),
        cust_price_tier: p.cust_price_tier.clone(),
        cust_type: p.cust_type.clone(),
        cust_email: p.cust_email.clone(),
        cust_add_no: p.cust_add_no.clone(),
        // Same source value as `cust_add_no` — the UPSERT binds $9 to
        // both columns, so the projected `cust_address` IS `Cust_Add_no`.
        cust_address: p.cust_add_no.clone(),
        cust_add_moo: p.cust_add_moo.clone(),
        cust_add_soi: p.cust_add_soi.clone(),
        cust_add_road: p.cust_add_road.clone(),
        cust_add_tambon: p.cust_add_tambon.clone(),
        cust_add_ampore: p.cust_add_ampore.clone(),
        cust_add_province: p.cust_add_province.clone(),
        cust_add_code: p.cust_add_code.clone(),
        cust_phone: p.cust_phone.clone(),
        cust_add_fax: p.cust_add_fax.clone(),
        cust_work_name: p.cust_work_name.clone(),
        cust_work_no: p.cust_work_no.clone(),
        cust_work_moo: p.cust_work_moo.clone(),
        cust_work_soi: p.cust_work_soi.clone(),
        cust_work_road: p.cust_work_road.clone(),
        cust_work_tambon: p.cust_work_tambon.clone(),
        cust_work_ampore: p.cust_work_ampore.clone(),
        cust_work_province: p.cust_work_province.clone(),
        cust_work_code: p.cust_work_code.clone(),
        cust_work_tel: p.cust_work_tel.clone(),
        cust_work_fax: p.cust_work_fax.clone(),
        cust_work_tax: p.cust_work_tax.clone(),
        cust_last_change: p.cust_last_change,
        cust_contry: p.cust_contry.clone(),
        cust_price_over: p.cust_price_over,
    }
}

async fn fetch_existing(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cust_no: &str,
) -> Result<Option<ExistingRow>, SyncError> {
    use sqlx::Row;

    // Dynamic query — keeps this file out of the `.sqlx` offline cache
    // (which would have to be regenerated for every column tweak). The
    // 37 identifiers we read mirror everything the UPSERT writes
    // (including the `cust_address` lock-step mirror), plus
    // (cust_id, aggregate_id) for FK resolution.
    let opt = sqlx::query(
        "SELECT cust_id, aggregate_id, legacy_id, cust_firstname, cust_name2, \
                cust_title, cust_sex, cust_idcard, cust_price_tier, \
                cust_type, cust_email, cust_add_no, cust_address, cust_add_moo, \
                cust_add_soi, cust_add_road, cust_add_tambon, cust_add_ampore, \
                cust_add_province, cust_add_code, cust_phone, cust_add_fax, \
                cust_work_name, cust_work_no, cust_work_moo, cust_work_soi, \
                cust_work_road, cust_work_tambon, cust_work_ampore, \
                cust_work_province, cust_work_code, cust_work_tel, \
                cust_work_fax, cust_work_tax, cust_last_change, cust_contry, \
                cust_price_over \
           FROM ht_customers \
          WHERE legacy_cust_no = $1 \
          LIMIT 1",
    )
    .bind(cust_no)
    .fetch_optional(&mut **tx)
    .await?;

    let Some(row) = opt else {
        return Ok(None);
    };
    Ok(Some(ExistingRow {
        cust_id: row.try_get("cust_id")?,
        aggregate_id: row.try_get("aggregate_id")?,
        legacy_id: row.try_get("legacy_id")?,
        keys: ExistingEqualityKeys {
            cust_name: row.try_get("cust_firstname")?,
            cust_name2: row.try_get("cust_name2")?,
            cust_title: row.try_get("cust_title")?,
            cust_sex: row.try_get("cust_sex")?,
            cust_idcard: row.try_get("cust_idcard")?,
            cust_price_tier: row.try_get("cust_price_tier")?,
            cust_type: row.try_get("cust_type")?,
            cust_email: row.try_get("cust_email")?,
            cust_add_no: row.try_get("cust_add_no")?,
            cust_address: row.try_get("cust_address")?,
            cust_add_moo: row.try_get("cust_add_moo")?,
            cust_add_soi: row.try_get("cust_add_soi")?,
            cust_add_road: row.try_get("cust_add_road")?,
            cust_add_tambon: row.try_get("cust_add_tambon")?,
            cust_add_ampore: row.try_get("cust_add_ampore")?,
            cust_add_province: row.try_get("cust_add_province")?,
            cust_add_code: row.try_get("cust_add_code")?,
            cust_phone: row.try_get("cust_phone")?,
            cust_add_fax: row.try_get("cust_add_fax")?,
            cust_work_name: row.try_get("cust_work_name")?,
            cust_work_no: row.try_get("cust_work_no")?,
            cust_work_moo: row.try_get("cust_work_moo")?,
            cust_work_soi: row.try_get("cust_work_soi")?,
            cust_work_road: row.try_get("cust_work_road")?,
            cust_work_tambon: row.try_get("cust_work_tambon")?,
            cust_work_ampore: row.try_get("cust_work_ampore")?,
            cust_work_province: row.try_get("cust_work_province")?,
            cust_work_code: row.try_get("cust_work_code")?,
            cust_work_tel: row.try_get("cust_work_tel")?,
            cust_work_fax: row.try_get("cust_work_fax")?,
            cust_work_tax: row.try_get("cust_work_tax")?,
            cust_last_change: row.try_get("cust_last_change")?,
            cust_contry: row.try_get("cust_contry")?,
            cust_price_over: row.try_get("cust_price_over")?,
        },
    }))
}

/// Returns true when every mirrored column already matches the legacy
/// projection. Used to skip event publication on a re-applied row.
///
/// The comparison IS [`GATE_FIELDS`] — one named comparator per mirrored
/// column, `.all()`-ed. Deleting a term from that table deletes the
/// comparison, which is what makes the gate/hash superset invariant
/// mechanically checkable (see [`crate::sync::gate_guard`]).
///
/// `legacy_id` (migration 055) is the one guarded term: a `None`
/// projection (fixture / pre-widening load) must not force a re-apply,
/// but a `Some` projection against a still-NULL stored value MUST
/// mismatch once so the UPSERT backfills the column.
///
/// Every other term is UNGUARDED, because the UPSERT writes those
/// columns through plainly. A guard would be wrong for `cust_address`:
/// the write carries no COALESCE, so a `None` `Cust_Add_no` has to be
/// allowed to blank a stale stored value.
fn matches(existing: &ExistingRow, projected: &CustomerProjection) -> bool {
    GATE_FIELDS
        .iter()
        .all(|f| (f.matches)(existing, projected))
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
            bind_projected_columns(
                sqlx::query(UPDATE_SQL),
                &projected,
            )
            .bind(&projected.cust_no)   // $34
            .bind(agg_id)               // $35
            .bind(ex.cust_id)           // $36
            .bind(projected.legacy_id)  // $37
            .execute(&mut **tx)
            .await?;
            (ex.cust_id, agg_id, false)
        }
        None => {
            // INSERT, RETURNING so we can derive aggregate_id from the
            // freshly-allocated SERIAL cust_id, then UPDATE that.
            let row = bind_projected_columns(
                sqlx::query_as::<_, (i32,)>(INSERT_SQL),
                &projected,
            )
            .bind(&projected.cust_no)   // $34
            .bind(projected.legacy_id)  // $35
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

/// SQL for the UPDATE branch — column list mirrors [`bind_projected_columns`]
/// in $1..$33 order, with $34 = legacy_cust_no, $35 = aggregate_id,
/// $36 = cust_id WHERE-key, $37 = legacy_id (migration 055; COALESCE'd
/// new-over-old: the legacy id of a Cust_no never changes, but a `None`
/// projection from a sparse fixture must not blank a stored value).
/// `cust_address` is kept in lock-step with `cust_add_no` so legacy
/// single-line readers keep working.
const UPDATE_SQL: &str =
    "UPDATE ht_customers \
        SET cust_firstname     = $1, \
            cust_name2         = $2, \
            cust_title         = $3, \
            cust_sex           = $4, \
            cust_idcard        = $5, \
            cust_price_tier    = $6, \
            cust_type          = $7, \
            cust_email         = $8, \
            cust_add_no        = $9, \
            cust_add_moo       = $10, \
            cust_add_soi       = $11, \
            cust_add_road      = $12, \
            cust_add_tambon    = $13, \
            cust_add_ampore    = $14, \
            cust_add_province  = $15, \
            cust_add_code      = $16, \
            cust_phone         = $17, \
            cust_add_fax       = $18, \
            cust_work_name     = $19, \
            cust_work_no       = $20, \
            cust_work_moo      = $21, \
            cust_work_soi      = $22, \
            cust_work_road     = $23, \
            cust_work_tambon   = $24, \
            cust_work_ampore   = $25, \
            cust_work_province = $26, \
            cust_work_code     = $27, \
            cust_work_tel      = $28, \
            cust_work_fax      = $29, \
            cust_work_tax      = $30, \
            cust_last_change   = $31, \
            cust_contry        = $32, \
            cust_price_over    = $33, \
            cust_address       = $9, \
            legacy_cust_no     = COALESCE(legacy_cust_no, $34), \
            aggregate_id       = COALESCE(aggregate_id, $35), \
            legacy_id          = COALESCE($37, legacy_id), \
            cust_deleted_at    = NULL, \
            updated_at         = NOW() \
      WHERE cust_id = $36";

/// SQL for the INSERT branch — same $1..$33 column order as
/// [`bind_projected_columns`], $34 = legacy_cust_no, $35 = legacy_id.
const INSERT_SQL: &str =
    "INSERT INTO ht_customers \
         (cust_firstname, cust_name2, cust_title, cust_sex, cust_idcard, \
          cust_price_tier, cust_type, cust_email, cust_add_no, cust_add_moo, \
          cust_add_soi, cust_add_road, cust_add_tambon, cust_add_ampore, \
          cust_add_province, cust_add_code, cust_phone, cust_add_fax, \
          cust_work_name, cust_work_no, cust_work_moo, cust_work_soi, \
          cust_work_road, cust_work_tambon, cust_work_ampore, \
          cust_work_province, cust_work_code, cust_work_tel, cust_work_fax, \
          cust_work_tax, cust_last_change, cust_contry, cust_price_over, \
          cust_address, legacy_cust_no, legacy_id) \
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, \
             $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, \
             $27, $28, $29, $30, $31, $32, $33, $9, $34, $35) \
     RETURNING cust_id";

/// Binds the 33 projected columns to a sqlx query in $1..$33 order.
/// Generic over the query type so it works for both `Query` and
/// `QueryAs`. Returns the query so the caller can chain further binds.
fn bind_projected_columns<'q, Q>(q: Q, p: &'q CustomerProjection) -> Q
where
    Q: BindCustomer<'q>,
{
    q.bind_str(&p.cust_name)
        .bind_optstr(&p.cust_name2)
        .bind_optstr(&p.cust_title)
        .bind_optstr(&p.cust_sex)
        .bind_optstr(&p.cust_idcard)
        .bind_optstr(&p.cust_price_tier)
        .bind_optstr(&p.cust_type)
        .bind_optstr(&p.cust_email)
        .bind_optstr(&p.cust_add_no)
        .bind_optstr(&p.cust_add_moo)
        .bind_optstr(&p.cust_add_soi)
        .bind_optstr(&p.cust_add_road)
        .bind_optstr(&p.cust_add_tambon)
        .bind_optstr(&p.cust_add_ampore)
        .bind_optstr(&p.cust_add_province)
        .bind_optstr(&p.cust_add_code)
        .bind_optstr(&p.cust_phone)
        .bind_optstr(&p.cust_add_fax)
        .bind_optstr(&p.cust_work_name)
        .bind_optstr(&p.cust_work_no)
        .bind_optstr(&p.cust_work_moo)
        .bind_optstr(&p.cust_work_soi)
        .bind_optstr(&p.cust_work_road)
        .bind_optstr(&p.cust_work_tambon)
        .bind_optstr(&p.cust_work_ampore)
        .bind_optstr(&p.cust_work_province)
        .bind_optstr(&p.cust_work_code)
        .bind_optstr(&p.cust_work_tel)
        .bind_optstr(&p.cust_work_fax)
        .bind_optstr(&p.cust_work_tax)
        .bind_optdt(&p.cust_last_change)
        .bind_optstr(&p.cust_contry)
        .bind_optf64(&p.cust_price_over)
}

/// Bind-helper trait so [`bind_projected_columns`] works for both
/// `sqlx::query::Query` and `sqlx::query::QueryAs`. Avoids type-erasing
/// the query while still letting us pass it through a generic helper.
trait BindCustomer<'q>: Sized {
    fn bind_str(self, s: &'q str) -> Self;
    fn bind_optstr(self, s: &'q Option<String>) -> Self;
    fn bind_optdt(self, d: &'q Option<chrono::NaiveDateTime>) -> Self;
    fn bind_optf64(self, f: &'q Option<f64>) -> Self;
}

impl<'q> BindCustomer<'q>
    for sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>
{
    fn bind_str(self, s: &'q str) -> Self {
        self.bind(s)
    }
    fn bind_optstr(self, s: &'q Option<String>) -> Self {
        self.bind(s)
    }
    fn bind_optdt(self, d: &'q Option<chrono::NaiveDateTime>) -> Self {
        self.bind(d)
    }
    fn bind_optf64(self, f: &'q Option<f64>) -> Self {
        self.bind(f)
    }
}

impl<'q, O>
    BindCustomer<'q>
    for sqlx::query::QueryAs<'q, sqlx::Postgres, O, sqlx::postgres::PgArguments>
{
    fn bind_str(self, s: &'q str) -> Self {
        self.bind(s)
    }
    fn bind_optstr(self, s: &'q Option<String>) -> Self {
        self.bind(s)
    }
    fn bind_optdt(self, d: &'q Option<chrono::NaiveDateTime>) -> Self {
        self.bind(d)
    }
    fn bind_optf64(self, f: &'q Option<f64>) -> Self {
        self.bind(f)
    }
}

async fn apply_soft_delete(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    // For D, CT only carries the PK columns (no joined row data) — for
    // `HT_Customers` that is the legacy SERIAL `id` ALONE; the joined
    // `Cust_no` is NULL because the row no longer exists. Pre-migration
    // 055 this function resolved exclusively by `Cust_no`, so EVERY
    // iHOTEL customer delete (FrmManageCustomersNew, cheatsheet §3.24)
    // was a silent no-op and the canonical row stayed live forever
    // (audit 2026-06-11 P1 #6).
    //
    // Resolution order:
    //   1. `legacy_id` (migration 055) — the only key a D-row reliably
    //      carries. Populated by every I/U apply since 2026-06-11.
    //   2. `Cust_no` fallback — fires only when the materialised row
    //      somehow carries it (it never does for real CT D-rows, but
    //      fixtures and a hypothetical future projection might).
    //   3. Neither resolves → loud WARN, not an error: the MSSQL row is
    //      already gone, so a retry can never learn more — erroring
    //      would wedge the watermark permanently on deletes of rows
    //      mirrored before migration 055 backfilled their legacy_id.
    //      This is a deliberate, logged residual gap (see migration 055
    //      header).
    //
    // Per spec: emit no DomainEvent for D in 5.2 (no UI subscriber yet).
    let legacy_id = row.try_get_i32("id").unwrap_or(None);
    if let Some(id) = legacy_id {
        let affected = sqlx::query(
            "UPDATE ht_customers \
                SET cust_deleted_at = NOW(), \
                    updated_at      = NOW() \
              WHERE legacy_id = $1",
        )
        .bind(id)
        .execute(&mut **tx)
        .await?
        .rows_affected();
        if affected > 0 {
            tracing::info!(
                legacy_id = id,
                "ht_customers soft-deleted via legacy_id (iHOTEL hard delete)"
            );
            return Ok(None);
        }
    }

    let cust_no_opt = row.try_get_str("Cust_no").unwrap_or(None);
    if let Some(cust_no) = cust_no_opt {
        let affected = sqlx::query(
            "UPDATE ht_customers \
                SET cust_deleted_at = NOW(), \
                    updated_at      = NOW() \
              WHERE legacy_cust_no = $1",
        )
        .bind(cust_no)
        .execute(&mut **tx)
        .await?
        .rows_affected();
        if affected > 0 {
            return Ok(None);
        }
    }

    tracing::warn!(
        legacy_id = ?legacy_id,
        "HT_Customers D-row could not be resolved to a canonical row \
         (legacy_id not yet backfilled on rows mirrored before migration \
         055, or the customer was never mirrored) — soft delete skipped; \
         deliberate no-op because the MSSQL row is gone and retrying can \
         never learn more"
    );
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

/// Eager-mirror a single `HT_Customers` row into canonical `ht_customers`.
///
/// Used by the check-in AND booking mappers when a CT-driven row
/// references a `Cust_no` that hasn't been mirrored yet — fetching the
/// customer in-band (rather than "deferring", which silently drops the
/// dependent row once the watermark advances — the June-3 2026 class)
/// is the only self-healing path. See
/// `sync::mappers::checkin::resolve_customer_or_eager_mirror`.
///
/// Semantics:
/// - Projects `row` via the same shared [`project`] helper the I/U path
///   uses, so column extraction stays consistent across both entry points.
/// - INSERTs with `ON CONFLICT (legacy_cust_no) DO NOTHING` against the
///   partial unique index defined in migration 018. Concurrent inserts
///   from another tick or another aggregate apply in the same TX collapse
///   silently to the existing row.
/// - Sets `aggregate_id` on the freshly-inserted row in a follow-up
///   UPDATE (matching the I/U path's pattern of deriving the UUID from
///   the SERIAL `cust_id`).
/// - Returns the canonical `cust_id` so the caller can resolve its FK
///   without an extra round-trip.
///
/// Emits no `DomainEvent` — the caller is reconstructing a missing FK,
/// not observing a customer-level transition. The next CT tick on
/// `HT_Customers` (if any) will surface a normal `CustomerCreated` via
/// the I/U path; subscribers de-dup by `aggregate_id`.
pub(crate) async fn upsert_customer_from_row(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
) -> Result<i32, SyncError> {
    let projected = project(row)?;

    // Race-safe INSERT: another concurrent tick (or a sibling aggregate
    // apply within the same outer TX) may have just inserted the row.
    // `ON CONFLICT DO NOTHING` against the partial unique index on
    // `legacy_cust_no` collapses that case to a no-op so we don't error
    // out on the duplicate. The matching `WHERE legacy_cust_no IS NOT
    // NULL` predicate is required because `ON CONFLICT (col)` only
    // matches a partial unique index when the same predicate is named
    // explicitly (PostgreSQL 42P10 otherwise). The RETURNING clause is
    // therefore optional (returns 0 rows on conflict), and we follow
    // up with a SELECT to resolve the canonical `cust_id` in both
    // branches.
    // Same column layout as [`INSERT_SQL`] but with an `ON CONFLICT
    // DO NOTHING` clause so concurrent eager-mirror calls within the
    // same outer TX collapse safely.
    const EAGER_INSERT_SQL: &str =
        "INSERT INTO ht_customers \
             (cust_firstname, cust_name2, cust_title, cust_sex, cust_idcard, \
              cust_price_tier, cust_type, cust_email, cust_add_no, cust_add_moo, \
              cust_add_soi, cust_add_road, cust_add_tambon, cust_add_ampore, \
              cust_add_province, cust_add_code, cust_phone, cust_add_fax, \
              cust_work_name, cust_work_no, cust_work_moo, cust_work_soi, \
              cust_work_road, cust_work_tambon, cust_work_ampore, \
              cust_work_province, cust_work_code, cust_work_tel, cust_work_fax, \
              cust_work_tax, cust_last_change, cust_contry, cust_price_over, \
              cust_address, legacy_cust_no, legacy_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, \
                 $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, \
                 $27, $28, $29, $30, $31, $32, $33, $9, $34, $35) \
         ON CONFLICT (legacy_cust_no) WHERE legacy_cust_no IS NOT NULL \
             DO NOTHING \
         RETURNING cust_id";

    let inserted: Option<(i32,)> = bind_projected_columns(
        sqlx::query_as::<_, (i32,)>(EAGER_INSERT_SQL),
        &projected,
    )
    .bind(&projected.cust_no)   // $34
    .bind(projected.legacy_id)  // $35
    .fetch_optional(&mut **tx)
    .await?;

    let cust_id = match inserted {
        Some((id,)) => {
            // Fresh INSERT — pin aggregate_id to the SERIAL id so
            // subsequent subscribers can deduplicate by UUID.
            let agg_id = aggregate_uuid(AggregateKind::Customer, id);
            sqlx::query("UPDATE ht_customers SET aggregate_id = $1 WHERE cust_id = $2")
                .bind(agg_id)
                .bind(id)
                .execute(&mut **tx)
                .await?;
            id
        }
        None => {
            // Concurrent insert raced us — resolve the existing row by
            // legacy_cust_no.
            let existing: (i32,) = sqlx::query_as(
                "SELECT cust_id FROM ht_customers WHERE legacy_cust_no = $1 LIMIT 1",
            )
            .bind(&projected.cust_no)
            .fetch_one(&mut **tx)
            .await?;
            existing.0
        }
    };

    Ok(cust_id)
}

/// iHOTEL's reserved "deleted customer" sentinel (cheatsheet §3.24 /
/// §`HT_Customers` invariants). The delete cascade rewrites every FK-style
/// reference (`Cin_cust_no`, `Book_Cust_ID`, …) to `'C0000'`, and NO
/// real `HT_Customers` row with that `Cust_no` exists — so the
/// eager-mirror fetch can never satisfy it.
pub(crate) const DELETED_CUSTOMER_SENTINEL: &str = "C0000";

/// Ensure a canonical placeholder row exists for the `'C0000'` sentinel
/// and return its `cust_id`.
///
/// Added 2026-06-11 (audit P1 #6): when iHOTEL deletes a customer, its
/// cascade re-points dependent rows at `'C0000'`. Mirroring that
/// re-point requires `cin_cust_id` / `book_cust_id` (both `NOT NULL`)
/// to have somewhere to land. Per the cheatsheet, `'C0000'` "should be
/// treated as a sentinel and not as a valid customer reference" — the
/// canonical placeholder mirrors exactly that: a tombstone target, not
/// a customer anyone created.
///
/// Idempotent: `ON CONFLICT (legacy_cust_no) DO NOTHING` + follow-up
/// SELECT, same race-safe shape as [`upsert_customer_from_row`].
pub(crate) async fn ensure_deleted_customer_sentinel(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<i32, SyncError> {
    let inserted: Option<(i32,)> = sqlx::query_as(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no) \
         VALUES ('(deleted customer)', $1) \
         ON CONFLICT (legacy_cust_no) WHERE legacy_cust_no IS NOT NULL \
             DO NOTHING \
         RETURNING cust_id",
    )
    .bind(DELETED_CUSTOMER_SENTINEL)
    .fetch_optional(&mut **tx)
    .await?;

    let cust_id = match inserted {
        Some((id,)) => {
            let agg_id = aggregate_uuid(AggregateKind::Customer, id);
            sqlx::query("UPDATE ht_customers SET aggregate_id = $1 WHERE cust_id = $2")
                .bind(agg_id)
                .bind(id)
                .execute(&mut **tx)
                .await?;
            id
        }
        None => {
            let existing: (i32,) = sqlx::query_as(
                "SELECT cust_id FROM ht_customers WHERE legacy_cust_no = $1 LIMIT 1",
            )
            .bind(DELETED_CUSTOMER_SENTINEL)
            .fetch_one(&mut **tx)
            .await?;
            existing.0
        }
    };
    Ok(cust_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    /// Populate every column the projection touches with NULL. Callers
    /// override specific cells via `.with(...)` to focus the test on the
    /// behaviour under inspection. Without this helper the test setup
    /// would have to enumerate all 33 legacy columns even for a single-
    /// field assertion.
    fn make_row_with_nulls(cust_no: &str) -> HashMapRow {
        let mut r = HashMapRow::new(TABLE)
            .with("Cust_no", MockValue::Str(cust_no.into()));
        // Every other EAGER_FETCH column starts as Null so try_get_str
        // returns Ok(None) instead of erroring on a missing cell.
        for col in EAGER_FETCH_COLUMNS.iter().copied().filter(|c| *c != "Cust_no") {
            r = r.with(col, MockValue::Null);
        }
        r
    }

    fn make_row_full(cust_no: &str, name: &str) -> HashMapRow {
        make_row_with_nulls(cust_no)
            .with("Cust_name", MockValue::Str(name.into()))
            .with("Cust_perfix", MockValue::Str("นาย".into()))
            .with("Cust_IDcard", MockValue::Str("REDACTED-sa-pw90123".into()))
            .with(
                "Cust_Type_Main",
                MockValue::Str("บุคคลธรรมดา".into()),
            )
            .with("Cust_Email", MockValue::Str("a@b.co".into()))
            .with("Cust_Add_no", MockValue::Str("123/4".into()))
            .with("Cust_Add_tel", MockValue::Str("08REDACTED-sa-pw".into()))
    }

    /// Build a `CustomerProjection` with every column populated to a
    /// distinct sentinel value, so tests can detect when the projection
    /// loses a field. Used by the equality / event tests.
    fn make_projection_all_set() -> CustomerProjection {
        CustomerProjection {
            legacy_id: Some(21607),
            cust_no: "C00001".into(),
            cust_name: "Alice".into(),
            cust_name2: Some("Alice EN".into()),
            cust_title: Some("นาย".into()),
            cust_sex: Some("ชาย".into()),
            cust_idcard: Some("REDACTED-sa-pw90123".into()),
            cust_price_tier: Some("ราคาปกติ".into()),
            cust_type: Some("บุคคลธรรมดา".into()),
            cust_email: Some("a@b.co".into()),
            cust_add_no: Some("123/4".into()),
            cust_add_moo: Some("5".into()),
            cust_add_soi: Some("ซ.1".into()),
            cust_add_road: Some("ถ.สุขุมวิท".into()),
            cust_add_tambon: Some("คลองตัน".into()),
            cust_add_ampore: Some("วัฒนา".into()),
            cust_add_province: Some("กรุงเทพ".into()),
            cust_add_code: Some("10110".into()),
            cust_phone: Some("08REDACTED-sa-pw".into()),
            cust_add_fax: Some("02REDACTED-sa-pw".into()),
            cust_work_name: Some("Acme Co".into()),
            cust_work_no: Some("99".into()),
            cust_work_moo: Some("2".into()),
            cust_work_soi: Some("ซ.2".into()),
            cust_work_road: Some("ถ.เพชรบุรี".into()),
            cust_work_tambon: Some("ราชเทวี".into()),
            cust_work_ampore: Some("ราชเทวี".into()),
            cust_work_province: Some("กรุงเทพ".into()),
            cust_work_code: Some("10400".into()),
            cust_work_tel: Some("02-555-1111".into()),
            cust_work_fax: Some("02-555-2222".into()),
            cust_work_tax: Some("REDACTED-tax-id".into()),
            cust_last_change: Some(
                chrono::NaiveDate::from_ymd_opt(2026, 1, 15)
                    .unwrap()
                    .and_hms_opt(10, 30, 0)
                    .unwrap(),
            ),
            cust_contry: Some("Thai".into()),
            cust_price_over: Some(1234.5),
        }
    }

    /// Build an `ExistingRow` whose keys match `p` exactly — used by the
    /// idempotency-match tests.
    fn make_existing_matching(p: &CustomerProjection) -> ExistingRow {
        ExistingRow {
            cust_id: 1,
            aggregate_id: Some(uuid::Uuid::nil()),
            legacy_id: p.legacy_id,
            keys: projection_equality_keys(p),
        }
    }

    #[test]
    fn project_extracts_all_columns_from_full_row() {
        let row = make_row_full("C00001", "ทดสอบ");
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_no, "C00001");
        assert_eq!(p.cust_name, "ทดสอบ");
        assert_eq!(p.cust_title.as_deref(), Some("นาย"));
        assert_eq!(p.cust_idcard.as_deref(), Some("REDACTED-sa-pw90123"));
        assert_eq!(p.cust_type.as_deref(), Some("บุคคลธรรมดา"));
        assert_eq!(p.cust_email.as_deref(), Some("a@b.co"));
        assert_eq!(p.cust_add_no.as_deref(), Some("123/4"));
        assert_eq!(p.cust_phone.as_deref(), Some("08REDACTED-sa-pw"));
    }

    #[test]
    fn project_tolerates_null_optional_columns() {
        let row = make_row_with_nulls("C00099")
            .with("Cust_name", MockValue::Str("Anon".into()));
        let p = project(&row).expect("nullable cols must project to None");
        assert_eq!(p.cust_no, "C00099");
        assert_eq!(p.cust_name, "Anon");
        assert!(p.cust_title.is_none());
        assert!(p.cust_phone.is_none());
    }

    #[test]
    fn project_falls_back_to_empty_name_on_null_legacy_cust_name() {
        let row = make_row_with_nulls("C99999");
        let p = project(&row).expect("legacy NULL Cust_name must NOT abort");
        assert_eq!(p.cust_name, "");
    }

    #[test]
    fn project_errors_when_cust_no_is_null() {
        let row = make_row_with_nulls("ignored")
            // Override Cust_no to NULL on top of the all-NULL baseline.
            .with("Cust_no", MockValue::Null)
            .with("Cust_name", MockValue::Str("x".into()));
        let err = project(&row).expect_err("NULL Cust_no must be loud");
        assert!(err.to_string().contains("Cust_no"));
    }

    /// Two projections with identical content compare equal — the
    /// idempotency check relies on this.
    #[test]
    fn matches_returns_true_for_identical_projections() {
        let p = make_projection_all_set();
        let ex = make_existing_matching(&p);
        assert!(matches(&ex, &p));
    }

    #[test]
    fn matches_returns_false_when_phone_differs() {
        let p = make_projection_all_set();
        let mut ex = make_existing_matching(&p);
        ex.keys.cust_phone = Some("0999999999".into());
        assert!(!matches(&ex, &p));
    }

    /// Track E2 / T1 HIGH-2 — `Cust_Price_Over` is the legacy running
    /// debt balance. The mapper must capture changes so subsequent
    /// idempotency checks don't silently skip a debt mutation.
    #[test]
    fn matches_returns_false_when_cust_price_over_differs() {
        let p = make_projection_all_set();
        let mut ex = make_existing_matching(&p);
        ex.keys.cust_price_over = Some(0.0);
        assert!(!matches(&ex, &p));
    }

    // ----- cust_address lock-step mirror (2026-07-28 gate gap) ----------

    /// The gate must see a canonical `cust_address` that drifted away
    /// from the projected `Cust_Add_no`. This mapper writes the two in
    /// lock-step, but other canonical writers do not: the Thai-ID
    /// check-in prefill (`repository::customer::update`) stores the full
    /// unsplit address in `cust_address` and leaves `cust_add_no` alone.
    /// `cust_address` is a reconcile-hash input, so before this test the
    /// row hashed as unconverged forever while the gate said "no change"
    /// — and `force_converge_reconcile_row` re-drives this same gate, so
    /// the self-heal path was dead too. Same class as d09e756.
    #[test]
    fn matches_returns_false_when_cust_address_drifted_from_add_no() {
        let p = make_projection_all_set(); // cust_add_no = Some("123/4")
        let mut ex = make_existing_matching(&p);
        // What a check-in prefill leaves behind: the collapsed
        // single-line address, while cust_add_no still holds the door no.
        ex.keys.cust_address =
            Some("123/4 ม.5 ซ.1 ถ.สุขุมวิท คลองตัน วัฒนา กรุงเทพ 10110".into());
        assert_eq!(
            ex.keys.cust_add_no, p.cust_add_no,
            "only cust_address may differ in this fixture"
        );
        assert!(
            !matches(&ex, &p),
            "a cust_address diverged from cust_add_no MUST force a re-apply"
        );
    }

    /// Steady state stays idempotent: `cust_address` mirroring
    /// `cust_add_no` is exactly what the UPSERT leaves behind
    /// (`cust_address = $9`, the `cust_add_no` bind), so the re-apply
    /// converges in ONE pass instead of re-firing every tick.
    #[test]
    fn matches_returns_true_when_cust_address_mirrors_add_no() {
        let p = make_projection_all_set();
        let ex = make_existing_matching(&p);
        assert_eq!(ex.keys.cust_address, p.cust_add_no);
        assert!(
            matches(&ex, &p),
            "the lock-step steady state must not force a spurious re-apply"
        );
    }

    /// Unguarded on purpose: a legacy row whose `Cust_Add_no` went NULL
    /// must still defeat the gate so the UPSERT blanks the stale
    /// canonical `cust_address`. A Some-only guard (the `legacy_id`
    /// shape) would leave the stale value hashing as unconverged.
    #[test]
    fn matches_returns_false_when_add_no_null_but_address_still_stored() {
        let mut p = make_projection_all_set();
        p.cust_add_no = None;
        let mut ex = make_existing_matching(&p); // both keys now None
        ex.keys.cust_address = Some("123/4".into());
        assert!(
            !matches(&ex, &p),
            "a NULL Cust_Add_no must re-apply so the stale mirror is blanked"
        );
    }

    // ----- legacy_id (migration 055, customer hard-delete handling) ------

    /// The projection must capture the legacy SERIAL `id` so the UPSERT
    /// can persist it — CT D-rows carry ONLY this key.
    #[test]
    fn project_captures_legacy_id_from_pk_alias() {
        let row = make_row_with_nulls("C21607")
            .with("Cust_name", MockValue::Str("x".into()))
            .with("id", MockValue::I32(21607));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.legacy_id, Some(21607));
    }

    /// Sparse fixtures without an `id` cell must still project (None).
    #[test]
    fn project_tolerates_missing_legacy_id() {
        let row = make_row_with_nulls("C21607").with("Cust_name", MockValue::Str("x".into()));
        let p = project(&row).expect("missing id cell must not abort");
        // make_row_with_nulls seeds `id` (it's in EAGER_FETCH_COLUMNS)
        // as Null → None.
        assert!(p.legacy_id.is_none());
    }

    /// A row mirrored before migration 055 carries NULL legacy_id; the
    /// first CT touch after the migration must mismatch ONCE so the
    /// UPSERT backfills the column.
    #[test]
    fn matches_returns_false_when_stored_legacy_id_still_null() {
        let p = make_projection_all_set(); // legacy_id = Some(21607)
        let mut ex = make_existing_matching(&p);
        ex.legacy_id = None;
        assert!(!matches(&ex, &p), "must re-apply once to backfill legacy_id");
    }

    /// A None-projection legacy_id (sparse fixture) must not force a
    /// re-apply against a populated stored value.
    #[test]
    fn matches_guards_none_legacy_id_projection() {
        let mut p = make_projection_all_set();
        p.legacy_id = None;
        let mut ex = make_existing_matching(&p);
        ex.legacy_id = Some(21607);
        assert!(matches(&ex, &p));
    }

    // ----- gate ⊇ reconcile-hash (see `crate::sync::gate_guard`) ---------

    /// THE behavioural half of the gate/hash invariant, for customers.
    ///
    /// For every reconcile-hash input: apply a realistic legacy-side
    /// edit, then assert (i) the hashed segment actually moved — so a
    /// no-op mutator can't fake a pass — and (ii) the PRODUCTION gate
    /// refuses to idempotency-skip it.
    ///
    /// This cannot be satisfied by editing a list: it executes
    /// `matches()` against a genuinely mutated projection. It catches
    /// both directions — a hash input added without a gate term, and a
    /// gate term deleted while still hashed. The 2026-07-28
    /// `cust_address` gap (hashed on both sides, invisible to the gate)
    /// is exactly what it would have caught.
    #[test]
    fn customers_hash_mutations_all_defeat_the_idempotency_gate() {
        let base = make_projection_all_set();
        let ex = make_existing_matching(&base);
        assert!(
            matches(&ex, &base),
            "fixture must start converged, else the test proves nothing"
        );

        for input in HASH_INPUTS.iter() {
            if input.lookup_key {
                // Identity, not a compared field — `fetch_existing`
                // resolves BY it. See `HashInput::lookup_key`.
                continue;
            }
            let before = (input.segment)(&base);
            let mut mutated = base.clone();
            (input.mutate)(&mut mutated);
            let after = (input.segment)(&mutated);
            assert_ne!(
                before, after,
                "hash input `{}`: mutator did not move the hashed segment — \
                 a self-validating mutator is what stops this test passing \
                 vacuously",
                input.name,
            );
            assert!(
                !matches(&ex, &mutated),
                "GATE/HASH INVARIANT VIOLATED — customers: a legacy edit that \
                 moves reconcile-hash input `{}` is idempotency-SKIPPED by the \
                 mapper. The CT delta then ages out inside the 2-day retention \
                 window and the reconcile sweep flags a row it can never close \
                 (force_converge re-drives this same gate). Widen GATE_FIELDS. \
                 Mechanism: d09e756.",
                input.name,
            );
        }
    }

    /// Byte-parity pin. `ht_reconcile_log.mssql_hash` and every
    /// `ht_customers_legacy.sync_hash` ack are stored SHA256s of this
    /// exact body — one byte of drift invalidates all of them and
    /// triggers a full re-diff storm on the next tick.
    ///
    /// The expected value is built from the LITERAL pre-table format
    /// string (`"{}|{}|{}|{}|{}|{}"`), not from the new code, and is
    /// checked against BOTH the production hash function and the
    /// descriptor-table join.
    #[test]
    fn customers_hash_bytes_unchanged_for_golden_inputs() {
        use crate::scheduler::sync::{customer_canonical_hash, sha256};

        // Pin explicit values rather than reusing the shared fixture, so
        // a fixture tweak can never move the golden vector.
        let mut p = make_projection_all_set();
        p.cust_no = "C00001".into();
        p.cust_name = "Alice".into();
        p.cust_type = Some("บุคคลธรรมดา".into());
        p.cust_phone = Some("0899999999".into());
        p.cust_idcard = Some("9876543210987".into());
        p.cust_add_no = Some("123/4".into());

        // Literal body under the format string this table replaced:
        //   format!("{}|{}|{}|{}|{}|{}", cust_no, name, type, phone,
        //           idcard, address)  with `.unwrap_or("")` per Option.
        let expected = sha256("C00001|Alice|บุคคลธรรมดา|0899999999|9876543210987|123/4");

        assert_eq!(
            customer_canonical_hash(
                &p.cust_no,
                &p.cust_name,
                p.cust_type.as_deref(),
                p.cust_phone.as_deref(),
                p.cust_idcard.as_deref(),
                // Canonical side reads `cust_address`; the projection
                // holds it as `cust_add_no` (lock-step mirror).
                p.cust_add_no.as_deref(),
            ),
            expected,
            "production customer hash changed bytes"
        );
        assert_eq!(
            sha256(&hash_body(&p)),
            expected,
            "HASH_INPUTS join no longer reproduces the production hash body — \
             an entry was added, removed, reordered, or mis-rendered"
        );
    }

    /// Compile-time completeness: every mirrored column has a gate term.
    ///
    /// The destructuring below has NO `..` rest pattern, so adding a
    /// column to `ExistingEqualityKeys` stops this test COMPILING
    /// (E0027). The engineer must then add it here, and the set-equality
    /// assert forces the matching [`GATE_FIELDS`] entry. That closes the
    /// loop a plain name list leaves open.
    #[test]
    fn every_equality_key_column_has_a_gate_term() {
        use std::collections::HashSet;

        let ExistingEqualityKeys {
            cust_name: _,
            cust_name2: _,
            cust_title: _,
            cust_sex: _,
            cust_idcard: _,
            cust_price_tier: _,
            cust_type: _,
            cust_email: _,
            cust_add_no: _,
            cust_address: _,
            cust_add_moo: _,
            cust_add_soi: _,
            cust_add_road: _,
            cust_add_tambon: _,
            cust_add_ampore: _,
            cust_add_province: _,
            cust_add_code: _,
            cust_phone: _,
            cust_add_fax: _,
            cust_work_name: _,
            cust_work_no: _,
            cust_work_moo: _,
            cust_work_soi: _,
            cust_work_road: _,
            cust_work_tambon: _,
            cust_work_ampore: _,
            cust_work_province: _,
            cust_work_code: _,
            cust_work_tel: _,
            cust_work_fax: _,
            cust_work_tax: _,
            cust_last_change: _,
            cust_contry: _,
            cust_price_over: _,
        } = projection_equality_keys(&make_projection_all_set());

        // `cust_name` holds PG `cust_firstname`; the gate term is named
        // after the PG column so the hash table can cite it directly.
        let expected: HashSet<&str> = [
            "cust_firstname",
            "cust_name2",
            "cust_title",
            "cust_sex",
            "cust_idcard",
            "cust_price_tier",
            "cust_type",
            "cust_email",
            "cust_add_no",
            "cust_address",
            "cust_add_moo",
            "cust_add_soi",
            "cust_add_road",
            "cust_add_tambon",
            "cust_add_ampore",
            "cust_add_province",
            "cust_add_code",
            "cust_phone",
            "cust_add_fax",
            "cust_work_name",
            "cust_work_no",
            "cust_work_moo",
            "cust_work_soi",
            "cust_work_road",
            "cust_work_tambon",
            "cust_work_ampore",
            "cust_work_province",
            "cust_work_code",
            "cust_work_tel",
            "cust_work_fax",
            "cust_work_tax",
            "cust_last_change",
            "cust_contry",
            "cust_price_over",
            // Not an `ExistingEqualityKeys` column — lives on
            // `ExistingRow` because its comparison is guarded.
            "legacy_id",
        ]
        .into_iter()
        .collect();

        let actual: HashSet<&str> = gate_field_names().into_iter().collect();
        assert_eq!(
            actual, expected,
            "GATE_FIELDS drifted from the mirrored-column set"
        );
    }

    /// No customer hash input rests on a guarded (Some-only) term, so
    /// every hashed movement — including a legacy value being cleared —
    /// is visible to the gate. `legacy_id` is the sole guarded term and
    /// is not hashed.
    #[test]
    fn no_customer_hash_input_rests_on_a_guarded_gate_term() {
        use std::collections::HashSet;

        let guarded: HashSet<&str> = gate_guard::guarded_gate_field_names(&GATE_FIELDS)
            .into_iter()
            .collect();
        assert_eq!(
            guarded,
            ["legacy_id"].into_iter().collect::<HashSet<&str>>(),
            "guarding a term is a decision with a residual Some→None \
             weakness — record it here deliberately"
        );
        for input in HASH_INPUTS.iter() {
            for name in input.gated_by {
                assert!(
                    !guarded.contains(name),
                    "hash input `{}` is gated only via guarded term `{}` — a \
                     legacy NULL would then be invisible to the gate but \
                     visible to the hash",
                    input.name,
                    name,
                );
            }
        }
    }

    #[test]
    fn build_event_for_insert_emits_customer_created() {
        let p = make_projection_all_set();
        let agg = aggregate_uuid(AggregateKind::Customer, 77);
        let event = build_event(ChangeOp::Insert, true, agg, &p);
        assert_eq!(event.type_name(), "CustomerCreated");
        assert_eq!(event.aggregate_id(), agg);
    }

    #[test]
    fn build_event_for_update_emits_customer_modified() {
        let p = make_projection_all_set();
        let agg = aggregate_uuid(AggregateKind::Customer, 77);
        let event = build_event(ChangeOp::Update, false, agg, &p);
        assert_eq!(event.type_name(), "CustomerModified");
        assert_eq!(event.aggregate_id(), agg);
    }

    #[test]
    fn build_event_classifies_unknown_cust_type_as_other() {
        let mut p = make_projection_all_set();
        p.cust_type = Some("???".into());
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

    // ========================================================================
    // Track E2 — column-expansion coverage (T1 HIGH-2 + T2 HIGH-4)
    //
    // Each block locks one finding from `docs/coexistence/audit-2026-05-13.md`
    // so the projection + SELECT contract can't quietly regress.
    // ========================================================================

    /// T2 HIGH-4 — eager-fetch must mention every column the projection
    /// reads. Otherwise the `fetch_customer_row_from_mssql` in
    /// `checkin.rs` builds a SELECT that omits these columns, and
    /// `try_get_str` errors at the row boundary with "column not
    /// present" — silent breakage at runtime.
    #[test]
    fn eager_fetch_columns_cover_track_e2_additions() {
        for col in [
            "Cust_name2",
            "Cust_sex",
            "Cust_Type",
            "Cust_Add_moo",
            "Cust_Add_soi",
            "Cust_Add_road",
            "Cust_Add_tambon",
            "Cust_Add_ampore",
            "Cust_Add_province",
            "Cust_Add_code",
            "Cust_Add_fax",
            "Cust_Work_Name",
            "Cust_Work_no",
            "Cust_Work_moo",
            "Cust_Work_soi",
            "Cust_Work_road",
            "Cust_Work_tambon",
            "Cust_Work_ampore",
            "Cust_Work_province",
            "Cust_Work_code",
            "Cust_Work_tel",
            "Cust_Work_fax",
            "Cust_Work_Tax",
            "Cust_Last_Change",
            "Cust_Contry",
            "Cust_Price_Over",
        ] {
            assert!(
                EAGER_FETCH_COLUMNS.contains(&col),
                "EAGER_FETCH_COLUMNS missing required column '{col}' — \
                 Track E2 widened projection but eager fetch was not updated"
            );
        }
    }

    /// T1 HIGH-2 — the running debt balance must be projected. Locks
    /// the float boundary translation (Cust_Price_Over is `float NOT
    /// NULL DEFAULT 0` in MSSQL).
    #[test]
    fn projects_cust_price_over_from_legacy_float() {
        let row = make_row_with_nulls("C00001")
            .with("Cust_Price_Over", MockValue::F64(2_500.75));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_price_over, Some(2_500.75));
    }

    #[test]
    fn projects_cust_price_over_null_when_legacy_null() {
        let row = make_row_with_nulls("C00001");
        let p = project(&row).expect("project must succeed");
        assert!(p.cust_price_over.is_none());
    }

    /// T1 HIGH-2 — full address tuple. Each of the 8 components must
    /// round-trip from MSSQL column → projection field.
    #[test]
    fn projects_address_tuple() {
        let row = make_row_with_nulls("C00002")
            .with("Cust_Add_no", MockValue::Str("123/4".into()))
            .with("Cust_Add_moo", MockValue::Str("5".into()))
            .with("Cust_Add_soi", MockValue::Str("ซ.1".into()))
            .with("Cust_Add_road", MockValue::Str("ถ.สุขุมวิท".into()))
            .with("Cust_Add_tambon", MockValue::Str("คลองตัน".into()))
            .with("Cust_Add_ampore", MockValue::Str("วัฒนา".into()))
            .with("Cust_Add_province", MockValue::Str("กรุงเทพ".into()))
            .with("Cust_Add_code", MockValue::Str("10110".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_add_no.as_deref(), Some("123/4"));
        assert_eq!(p.cust_add_moo.as_deref(), Some("5"));
        assert_eq!(p.cust_add_soi.as_deref(), Some("ซ.1"));
        assert_eq!(p.cust_add_road.as_deref(), Some("ถ.สุขุมวิท"));
        assert_eq!(p.cust_add_tambon.as_deref(), Some("คลองตัน"));
        assert_eq!(p.cust_add_ampore.as_deref(), Some("วัฒนา"));
        assert_eq!(p.cust_add_province.as_deref(), Some("กรุงเทพ"));
        assert_eq!(p.cust_add_code.as_deref(), Some("10110"));
    }

    /// T1 HIGH-2 — full work-address tuple. Same 10 fields legacy
    /// captures for corporate invoices.
    #[test]
    fn projects_work_address_tuple() {
        let row = make_row_with_nulls("C00003")
            .with("Cust_Work_Name", MockValue::Str("Acme Co".into()))
            .with("Cust_Work_no", MockValue::Str("99".into()))
            .with("Cust_Work_moo", MockValue::Str("2".into()))
            .with("Cust_Work_soi", MockValue::Str("ซ.2".into()))
            .with("Cust_Work_road", MockValue::Str("ถ.เพชรบุรี".into()))
            .with("Cust_Work_tambon", MockValue::Str("ราชเทวี".into()))
            .with("Cust_Work_ampore", MockValue::Str("ราชเทวี".into()))
            .with("Cust_Work_province", MockValue::Str("กรุงเทพ".into()))
            .with("Cust_Work_code", MockValue::Str("10400".into()))
            .with("Cust_Work_tel", MockValue::Str("02-555-1111".into()))
            .with("Cust_Work_fax", MockValue::Str("02-555-2222".into()))
            .with("Cust_Work_Tax", MockValue::Str("REDACTED-tax-id".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_work_name.as_deref(), Some("Acme Co"));
        assert_eq!(p.cust_work_no.as_deref(), Some("99"));
        assert_eq!(p.cust_work_moo.as_deref(), Some("2"));
        assert_eq!(p.cust_work_soi.as_deref(), Some("ซ.2"));
        assert_eq!(p.cust_work_road.as_deref(), Some("ถ.เพชรบุรี"));
        assert_eq!(p.cust_work_tambon.as_deref(), Some("ราชเทวี"));
        assert_eq!(p.cust_work_ampore.as_deref(), Some("ราชเทวี"));
        assert_eq!(p.cust_work_province.as_deref(), Some("กรุงเทพ"));
        assert_eq!(p.cust_work_code.as_deref(), Some("10400"));
        assert_eq!(p.cust_work_tel.as_deref(), Some("02-555-1111"));
        assert_eq!(p.cust_work_fax.as_deref(), Some("02-555-2222"));
        assert_eq!(p.cust_work_tax.as_deref(), Some("REDACTED-tax-id"));
    }

    /// T1 HIGH-2 — Cust_name2 (English/secondary name) is required for
    /// FrmReportRR4 rendering.
    #[test]
    fn projects_cust_name2() {
        let row = make_row_with_nulls("C1")
            .with("Cust_name2", MockValue::Str("Alice EN".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_name2.as_deref(), Some("Alice EN"));
    }

    /// T1 HIGH-2 — Cust_sex captured for demographic reports.
    #[test]
    fn projects_cust_sex() {
        let row = make_row_with_nulls("C1")
            .with("Cust_sex", MockValue::Str("ชาย".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_sex.as_deref(), Some("ชาย"));
    }

    /// T1 HIGH-2 — Cust_Contry preserves the legacy spelling (no "u").
    #[test]
    fn projects_cust_contry_preserving_legacy_spelling() {
        let row = make_row_with_nulls("C1")
            .with("Cust_Contry", MockValue::Str("Thai".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_contry.as_deref(), Some("Thai"));
    }

    /// T1 HIGH-2 — Cust_Type maps to `cust_price_tier` (rate-tier
    /// label, e.g. `'ราคาปกติ'`), distinct from `Cust_Type_Main` /
    /// `cust_type` which is the customer category. Keeping both
    /// avoids the existing `cust_type` column's semantics drifting.
    #[test]
    fn projects_cust_type_as_price_tier() {
        let row = make_row_with_nulls("C1")
            .with("Cust_Type", MockValue::Str("ราคาปกติ".into()));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_price_tier.as_deref(), Some("ราคาปกติ"));
    }

    /// T1 HIGH-2 — Cust_Last_Change as datetime, captured for audit.
    #[test]
    fn projects_cust_last_change_datetime() {
        let dt = chrono::NaiveDate::from_ymd_opt(2026, 1, 15)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        let row = make_row_with_nulls("C1")
            .with("Cust_Last_Change", MockValue::DateTime(dt));
        let p = project(&row).expect("project must succeed");
        assert_eq!(p.cust_last_change, Some(dt));
    }

    /// T2 HIGH-4 — the column list emitted into the CT JOIN must
    /// mention every legacy column the projection reads. Otherwise
    /// MSSQL returns NULL for the missing columns and the projection
    /// silently sees None.
    #[test]
    fn select_sql_lists_track_e2_columns() {
        let select = CustomerMapper.select_sql();
        for col in [
            "Cust_name2",
            "Cust_sex",
            "Cust_Type",
            "Cust_Add_moo",
            "Cust_Work_Name",
            "Cust_Work_Tax",
            "Cust_Last_Change",
            "Cust_Contry",
            "Cust_Price_Over",
        ] {
            assert!(
                select.contains(col),
                "SELECT clause missing '{col}' — projection will silently \
                 see None at runtime"
            );
        }
    }

    /// Track J1 — projection-lock guard. Every column the customer CT
    /// mapper SELECTs must exist on the live HF Hotel `HT_Customers`
    /// schema. A typo here (e.g. the PR #101-class `Cust_Type` vs
    /// `Cust_Type_Main` confusion) silently breaks the JOIN at runtime
    /// and drops every customer CT row.
    #[test]
    fn select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(SELECT_COLS, "HT_Customers");
    }
}
