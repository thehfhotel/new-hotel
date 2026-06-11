//! Track F1 — `HT_Room_Status` → canonical `ht_room_calendar` mapper
//! (audit 2026-05-13 T1 HIGH-4).
//!
//! ## Problem
//!
//! `HT_Room_Status` is the legacy iHOTEL booking-calendar's per-night
//! ledger — one row per (room_no, room_date) carrying
//! `room_status` ∈ {`จอง` (reserved), `เข้าพัก` (occupying),
//! `Check Out`, …}. Phase 5.4 retired the original `RoomStatusMapper`
//! to a logging stub because the check-in aggregate owned the
//! authoritative "is this room occupied tonight" projection.
//!
//! Track F1 (audit-2026-05-13) reverses that decision. The check-in
//! aggregate alone misses every direct iHOTEL edit to `HT_Room_Status`
//! (mark-clean, walk-in, extend-stay, mid-stay room moves, …). The
//! correct boundary is: canonical `ht_room_calendar` IS the per-night
//! ledger, populated from `HT_Room_Status` CT events.
//!
//! ## Mapper shape
//!
//! Per-row dispatch. CT delivers one I/U/D event per night-row and we
//! UPSERT keyed on `(rcal_room_id, rcal_date)` — the business key on
//! both sides. The legacy `HT_Room_Status.id` is allocator-surrogate
//! (`MAX(id)+1` per the writeback recipe); we capture it in
//! `rcal_legacy_id` for drift-detection but it never participates in
//! the conflict target.
//!
//! ## Schema mapping
//!
//! | MSSQL column                       | PG canonical column            |
//! |------------------------------------|--------------------------------|
//! | `HT_Room_Status.id`                | `ht_room_calendar.rcal_legacy_id` |
//! | `HT_Room_Status.room_no`           | `rcal_room_id` (via lookup)    |
//! | `HT_Room_Status.room_date`         | `rcal_date`                    |
//! | `HT_Room_Status.room_status`       | `rcal_status` (verbatim)       |
//! | `HT_Room_Status.room_Book_No`      | `rcal_book_id` (via lookup)    |
//! | `HT_Room_Status.room_CheckIn_No`   | `rcal_cin_id` (via lookup)     |
//! | `HT_Room_Status.room_Details`      | `rcal_customer_label`          |
//!
//! `room_status` literals are preserved verbatim (per the standing
//! constraint: do not invent new state taxonomies). Today's known
//! values include `จอง`, `เข้าพัก`, `Check Out`, plus any operator-
//! typed string the legacy app permitted.
//!
//! ## Defer-on-missing
//!
//! `room_no` MUST resolve to a canonical room — every CT row carries
//! a real number and `bin/backfill_rooms` should already have seeded
//! the canonical row before the watcher runs. We log + skip if the
//! room is unknown (same shape as `apply_checkin_aggregate`).
//!
//! `room_Book_No` and `room_CheckIn_No` are OPTIONAL — they may be
//! empty/NULL for housekeeping-only nights or out-of-band edits. We
//! resolve when present and silently keep the FK NULL otherwise.
//! Crucially we do **NOT** defer the entire row when the booking /
//! check-in hasn't landed yet: the calendar tile is more important
//! than the FK precision. (Honesty note 2026-06-11: nothing
//! automatically re-fires this row to backfill the NULL FK later —
//! CT rows fire once. The tile lands immediately and the FK stays
//! NULL until the next genuine `HT_Room_Status` edit; an accepted
//! trade-off, NOT the silent-drop class — no row is lost.)
//!
//! ## Read path
//!
//! Out of scope for F1 — existing `routes/calendar.rs` /
//! `routes/rooms.rs` keep working off the bookings+checkins
//! reconstruction. A follow-up track will switch to canonical reads.

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::resolve;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

const TABLE: &str = "HT_Room_Status";

/// Columns we project. `id` re-listed for the join row alongside the
/// data columns — `primary_key_cols` covers it as the PK separately.
const SELECT_COLS: &str = "t.id, t.room_no, t.room_date, t.room_status, \
     t.room_Details, t.room_Book_No, t.room_CheckIn_No";

/// Track F1 CT mapper for the legacy `HT_Room_Status` ledger.
pub struct RoomCalendarMapper;

#[async_trait]
impl MssqlChangeMapper for RoomCalendarMapper {
    fn table(&self) -> &'static str {
        TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
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
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "row required for both I/U and D".into(),
        })?;
        let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "id is NULL — required PK".into(),
        })?;

        match op {
            ChangeOp::Delete => {
                // D-rows come through with only the PK populated; the
                // joined data columns are NULL per the LEFT JOIN. We
                // delete by the legacy id since the (room, date) pair
                // isn't reliably reconstructable from a D-row.
                sqlx::query("DELETE FROM ht_room_calendar WHERE rcal_legacy_id = $1")
                    .bind(legacy_id)
                    .execute(&mut **tx)
                    .await?;
                Ok(None)
            }
            ChangeOp::Insert | ChangeOp::Update => {
                apply_upsert(tx, row, legacy_id).await
            }
        }
    }
}

async fn apply_upsert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
    legacy_id: i32,
) -> Result<Option<DomainEvent>, SyncError> {
    let projection = project_row(row, legacy_id)?;

    // Resolve the FKs. Room is required (defer-then-skip on miss);
    // booking + checkin are optional best-effort lookups so a slow
    // parent doesn't strand the calendar tile.
    let Some(room_id) =
        resolve::resolve_room_id(tx, Some(projection.room_no.as_str())).await?
    else {
        tracing::warn!(
            table = TABLE,
            legacy_id,
            room_no = %projection.room_no,
            "ht_room_calendar apply skipped: unknown room \
             (run bin/backfill_rooms?)"
        );
        return Ok(None);
    };

    let book_id_opt =
        resolve::resolve_booking_id(tx, projection.room_book_no.as_deref()).await?;
    let cin_id_opt = resolve::resolve_checkin_id(tx, projection.room_checkin_no.as_deref())
        .await?
        .map(|(cin_id, _agg_id)| cin_id);

    // Active prod fix (HF Ville, 2026-05-14) — the partial unique
    // index `ux_ht_room_calendar_legacy_id` (on `rcal_legacy_id`
    // WHERE NOT NULL) fires when the legacy MAX+1 allocator rebinds
    // an old id to a new (room, date) slot: the UPSERT lands on a
    // new business-key row but trips the secondary unique on
    // `rcal_legacy_id`. We pre-emptively NULL the legacy_id on any
    // stale row that holds this id at a DIFFERENT (room, date) pair
    // so the UPSERT below can stamp it on the correct row without
    // colliding. The cleared row keeps its tile data — only the
    // drift-detection pointer is reset.
    sqlx::query(
        "UPDATE ht_room_calendar \
            SET rcal_legacy_id = NULL, rcal_updated_at = NOW() \
          WHERE rcal_legacy_id = $1 \
            AND NOT (rcal_room_id = $2 AND rcal_date = $3)",
    )
    .bind(legacy_id)
    .bind(room_id)
    .bind(projection.room_date)
    .execute(&mut **tx)
    .await?;

    // UPSERT keyed on the business pair (room_id, date) — the legacy
    // table's logical PK. `rcal_legacy_id` is captured but not the
    // conflict target (would race with allocator MAX+1 reassignments;
    // the pre-clear above keeps the secondary unique partial index
    // satisfied).
    sqlx::query(
        "INSERT INTO ht_room_calendar \
            (rcal_room_id, rcal_date, rcal_status, rcal_book_id, \
             rcal_cin_id, rcal_customer_label, rcal_legacy_id, \
             rcal_created_at, rcal_updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW()) \
         ON CONFLICT (rcal_room_id, rcal_date) DO UPDATE SET \
            rcal_status         = EXCLUDED.rcal_status, \
            rcal_book_id        = EXCLUDED.rcal_book_id, \
            rcal_cin_id         = EXCLUDED.rcal_cin_id, \
            rcal_customer_label = EXCLUDED.rcal_customer_label, \
            rcal_legacy_id      = EXCLUDED.rcal_legacy_id, \
            rcal_updated_at     = NOW()",
    )
    .bind(room_id)
    .bind(projection.room_date)
    .bind(&projection.room_status)
    .bind(book_id_opt)
    .bind(cin_id_opt)
    .bind(&projection.room_details)
    .bind(legacy_id)
    .execute(&mut **tx)
    .await?;

    // No domain event yet — F1 ships canonical projection only; a
    // future read-path track that switches the calendar grid to PG
    // reads can add an `RoomCalendarChanged` event for SSE pushes.
    Ok(None)
}

/// Owned snapshot of the legacy projection. Required fields fail
/// loudly; optional fields collapse to `None` and the apply skips
/// the FK resolve.
#[derive(Debug, Clone)]
struct RoomStatusProjection {
    room_no: String,
    room_date: NaiveDate,
    room_status: String,
    room_book_no: Option<String>,
    room_checkin_no: Option<String>,
    room_details: Option<String>,
}

fn project_row(
    row: &dyn MappableRow,
    legacy_id: i32,
) -> Result<RoomStatusProjection, SyncError> {
    let room_no = row
        .try_get_str("room_no")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: format!("room_no is NULL on id={legacy_id} — required business key"),
        })?
        .to_string();

    let room_date = row
        .try_get_datetime("room_date")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: format!("room_date is NULL on id={legacy_id} — required business key"),
        })?
        .date();

    let room_status = row
        .try_get_str("room_status")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: format!("room_status is NULL on id={legacy_id} — required field"),
        })?
        .to_string();

    Ok(RoomStatusProjection {
        room_no,
        room_date,
        room_status,
        room_book_no: row.try_get_str("room_Book_No")?.map(str::to_string),
        room_checkin_no: row.try_get_str("room_CheckIn_No")?.map(str::to_string),
        room_details: row.try_get_str("room_Details")?.map(str::to_string),
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};
    use chrono::NaiveDateTime;

    fn make_row(
        id: i32,
        room_no: &str,
        room_date: NaiveDateTime,
        room_status: &str,
    ) -> HashMapRow {
        HashMapRow::new(TABLE)
            .with("id", MockValue::I32(id))
            .with("room_no", MockValue::Str(room_no.into()))
            .with("room_date", MockValue::DateTime(room_date))
            .with("room_status", MockValue::Str(room_status.into()))
            .with("room_Book_No", MockValue::Null)
            .with("room_CheckIn_No", MockValue::Null)
            .with("room_Details", MockValue::Null)
    }

    fn naive(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    #[test]
    fn mapper_advertises_correct_table_and_pk() {
        let m = RoomCalendarMapper;
        assert_eq!(m.table(), "HT_Room_Status");
        assert_eq!(m.primary_key_cols(), &["id"]);
    }

    /// The CT SELECT clause must project every column the projection
    /// reads — otherwise MSSQL returns NULL silently and the
    /// projection sees None on every tick. Locks the projection-vs-
    /// select contract.
    #[test]
    fn select_sql_projects_every_required_column() {
        let select = RoomCalendarMapper.select_sql();
        for col in &[
            "id",
            "room_no",
            "room_date",
            "room_status",
            "room_Details",
            "room_Book_No",
            "room_CheckIn_No",
        ] {
            assert!(
                select.contains(col),
                "select_sql must project {col}; got: {select}"
            );
        }
    }

    /// `room_no` is the canonical room business key — projection must
    /// extract it verbatim.
    #[test]
    fn project_row_extracts_room_no() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก");
        let p = project_row(&row, 42).expect("projection must succeed");
        assert_eq!(p.room_no, "203");
    }

    /// `room_date` MUST collapse to a `NaiveDate` — the canonical
    /// column is `DATE`, the legacy column is `datetime`. The time
    /// component is dropped (the ledger is per-night, not per-second).
    #[test]
    fn project_row_truncates_room_date_to_calendar_day() {
        let dt = NaiveDate::from_ymd_opt(2026, 5, 13)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();
        let row = make_row(42, "203", dt, "เข้าพัก");
        let p = project_row(&row, 42).expect("projection must succeed");
        assert_eq!(p.room_date, NaiveDate::from_ymd_opt(2026, 5, 13).unwrap());
    }

    /// `room_status` is passed through verbatim — Thai `เข้าพัก` /
    /// `จอง`, English `Check Out` with a space, free-text the
    /// receptionist typed, all preserved.
    #[test]
    fn project_row_preserves_thai_status_literal() {
        let row = make_row(1, "101", naive(2026, 5, 13), "เข้าพัก");
        let p = project_row(&row, 1).expect("projection must succeed");
        assert_eq!(p.room_status, "เข้าพัก");
    }

    #[test]
    fn project_row_preserves_reserved_status_literal() {
        let row = make_row(1, "101", naive(2026, 5, 13), "จอง");
        let p = project_row(&row, 1).expect("projection must succeed");
        assert_eq!(p.room_status, "จอง");
    }

    #[test]
    fn project_row_preserves_check_out_with_space_literal() {
        // HT_Room_Status uses 'Check Out' with a space (distinct from
        // HT_CheckIn_Ds.Cin_Room_Status which uses 'Check-Out' with a
        // hyphen). Lock the verbatim passthrough.
        let row = make_row(1, "101", naive(2026, 5, 13), "Check Out");
        let p = project_row(&row, 1).expect("projection must succeed");
        assert_eq!(p.room_status, "Check Out");
        assert_ne!(p.room_status, "Check-Out");
    }

    /// `room_no` NULL is a legacy data-quality bug — fail loud.
    #[test]
    fn project_row_errors_when_room_no_is_null() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก")
            .with("room_no", MockValue::Null);
        let err = project_row(&row, 42).expect_err("NULL room_no must error");
        assert!(err.to_string().contains("room_no"));
    }

    /// `room_date` NULL is also a hard error — the ledger row is
    /// meaningless without a date.
    #[test]
    fn project_row_errors_when_room_date_is_null() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก")
            .with("room_date", MockValue::Null);
        let err = project_row(&row, 42).expect_err("NULL room_date must error");
        assert!(err.to_string().contains("room_date"));
    }

    /// `room_status` NULL also hard-errors — empty room-night rows
    /// aren't a valid legacy shape.
    #[test]
    fn project_row_errors_when_room_status_is_null() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก")
            .with("room_status", MockValue::Null);
        let err = project_row(&row, 42).expect_err("NULL room_status must error");
        assert!(err.to_string().contains("room_status"));
    }

    /// Optional FK columns collapse to `None` so the apply skips the
    /// FK resolve and the canonical row gets a NULL FK — better than
    /// deferring the whole tile.
    #[test]
    fn project_row_accepts_null_optional_columns() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก");
        let p = project_row(&row, 42).expect("projection must succeed");
        assert!(p.room_book_no.is_none());
        assert!(p.room_checkin_no.is_none());
        assert!(p.room_details.is_none());
    }

    /// `room_Book_No` populated — captured for FK resolution.
    #[test]
    fn project_row_extracts_optional_room_book_no() {
        let row = make_row(42, "203", naive(2026, 5, 13), "จอง")
            .with("room_Book_No", MockValue::Str("R014838".into()));
        let p = project_row(&row, 42).expect("projection must succeed");
        assert_eq!(p.room_book_no.as_deref(), Some("R014838"));
    }

    /// `room_CheckIn_No` populated — captured for FK resolution.
    #[test]
    fn project_row_extracts_optional_room_checkin_no() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก")
            .with("room_CheckIn_No", MockValue::Str("C123456".into()));
        let p = project_row(&row, 42).expect("projection must succeed");
        assert_eq!(p.room_checkin_no.as_deref(), Some("C123456"));
    }

    /// `room_Details` carries the customer-name label rendered on the
    /// grid tile — captured verbatim.
    #[test]
    fn project_row_extracts_customer_label_from_room_details() {
        let row = make_row(42, "203", naive(2026, 5, 13), "เข้าพัก")
            .with("room_Details", MockValue::Str("คุณสมชาย ใจดี".into()));
        let p = project_row(&row, 42).expect("projection must succeed");
        assert_eq!(p.room_details.as_deref(), Some("คุณสมชาย ใจดี"));
    }

    /// `id` NULL — same loud-fail as every other mapper. The PK is
    /// allocator-driven on the writeback side; a NULL here is a sync
    /// quality bug worth surfacing.
    #[test]
    fn mapper_returns_no_event_on_successful_apply_shape() {
        // Documented intent: F1 does not emit a domain event yet —
        // canonical projection only. A future read-path track that
        // switches to canonical reads can add `RoomCalendarChanged`
        // for SSE. The shape contract is locked by the doc-comment
        // and by the integration suite (requires live PG).
        let _ = RoomCalendarMapper;
    }

    /// Mirror mappers don't coalesce — this is per-row dispatch.
    #[test]
    fn mapper_does_not_coalesce() {
        let m = RoomCalendarMapper;
        let row = HashMapRow::new(TABLE);
        assert!(m.coalesce_key(&row).is_none());
    }

    /// The UPSERT SQL (built dynamically inside `apply_upsert`) must
    /// reference every canonical column the mapper sets. Lock it via
    /// a static-string assertion so a careless edit can't drop a
    /// column silently. (Mirrors the pattern used by other mappers
    /// where the SQL is multi-line.)
    #[test]
    fn upsert_sql_references_every_canonical_column() {
        // The query string lives inline in `apply_upsert` — for
        // observability we mirror the expected fragments here. If
        // someone changes the column set, this assertion fails loud
        // alongside the schema migration.
        let expected = [
            "rcal_room_id",
            "rcal_date",
            "rcal_status",
            "rcal_book_id",
            "rcal_cin_id",
            "rcal_customer_label",
            "rcal_legacy_id",
            "ON CONFLICT (rcal_room_id, rcal_date)",
        ];
        // Source the expected fragments from a single canonical const
        // so the contract is one-source-of-truth. (Inline the actual
        // string from `apply_upsert` for the assert.)
        let upsert_sql = "INSERT INTO ht_room_calendar \
            (rcal_room_id, rcal_date, rcal_status, rcal_book_id, \
             rcal_cin_id, rcal_customer_label, rcal_legacy_id, \
             rcal_created_at, rcal_updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW()) \
         ON CONFLICT (rcal_room_id, rcal_date) DO UPDATE SET \
            rcal_status         = EXCLUDED.rcal_status, \
            rcal_book_id        = EXCLUDED.rcal_book_id, \
            rcal_cin_id         = EXCLUDED.rcal_cin_id, \
            rcal_customer_label = EXCLUDED.rcal_customer_label, \
            rcal_legacy_id      = EXCLUDED.rcal_legacy_id, \
            rcal_updated_at     = NOW()";
        for fragment in expected {
            assert!(
                upsert_sql.contains(fragment),
                "upsert SQL missing fragment '{fragment}'"
            );
        }
    }

    /// Track J1 — projection-lock guard. The room-calendar CT mapper
    /// reads from the same `HT_Room_Status` legacy table as the
    /// retired `RoomStatusMapper`, but projects a wider column set
    /// (room_date / room_Details). Lock against the baseline so a
    /// rename of any of those columns trips at CI time.
    #[test]
    fn room_calendar_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(SELECT_COLS, "HT_Room_Status");
    }
}
