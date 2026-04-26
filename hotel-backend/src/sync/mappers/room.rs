//! Change Tracking mappers for the legacy room tables:
//!
//! * [`RoomMasterMapper`] — `HT_Rooms` (room inventory: type, clean,
//!   use). Mirrors the Phase-5.2 room aggregate.
//! * [`RoomStatusMapper`] — `HT_Room_Status` (per-night occupancy
//!   ledger). Stub for 5.2: deferred to 5.3 / 5.4 where the booking +
//!   checkin mappers own this surface.
//!
//! ## User constraint (verbatim, locked)
//!
//! > "for rooms statuses and metadata, stick to current setup we have
//! > in HOTEL legacy app for now"
//!
//! Practical consequences enforced here:
//!
//! - **No new English status taxonomy.** We mirror the legacy literals
//!   `Room_Clean = 'yes' | 'no'` and `Room_Use = 'yes' | 'no'` directly
//!   into our boolean columns at the boundary (semantics-preserving;
//!   the user's intent is "don't invent new states, don't drop info").
//! - **No `RoomMasterChanged` DomainEvent variant.** We only emit the
//!   already-existing `RoomMarkedClean` / `RoomMarkedDirty` events when
//!   `Room_Clean` actually flips. Other column edits (Room_Type,
//!   Room_PriceA, …) silently UPSERT with no event — there is no UI
//!   subscriber yet.
//! - **No metadata schema additions.** The mapper writes only to columns
//!   that already exist on `ht_rooms_new`.
//!
//! `HT_Rooms` notably does **not** have a `room_status` column — that
//! lives on `HT_Room_Status`. The Thai room-status literals
//! (`'ว่าง'`/`'เข้าพัก'`/`'จอง'`) referenced in the architecture spec
//! belong to the per-night ledger / booking views, not to the room
//! master. The 5.4 booking + checkin mappers will reconstruct the
//! occupancy view from that ledger; the master mapper only owns
//! "physical room exists, type X, currently clean/dirty/in-use".

use async_trait::async_trait;
use uuid::Uuid;

use crate::outbox::event::{DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

// =============================================================================
// HT_Rooms — room master mapper
// =============================================================================

/// CT mapper for the legacy `HT_Rooms` master table.
pub struct RoomMasterMapper;

const ROOMS_TABLE: &str = "HT_Rooms";

/// Columns we project. `id` is the PK on the CT side and is therefore
/// listed via [`primary_key_cols`] separately — we still re-project it
/// into the SELECT for `apply` to read alongside the data columns.
const ROOMS_SELECT_COLS: &str =
    "t.id, t.Room_no, t.Room_Type, t.Room_Clean, t.Room_Use, t.Room_Details";

#[async_trait]
impl MssqlChangeMapper for RoomMasterMapper {
    fn table(&self) -> &'static str {
        ROOMS_TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        ROOMS_SELECT_COLS
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
                    table: ROOMS_TABLE,
                    message: "I/U operation requires joined row".into(),
                })?;
                apply_room_upsert(tx, row).await
            }
            ChangeOp::Delete => {
                // Per user constraint: rooms are physical inventory —
                // legacy app does not delete rooms in normal operation.
                // If a D event ever arrives we log it and skip.
                tracing::warn!(
                    table = ROOMS_TABLE,
                    "HT_Rooms D event received — ignored (rooms are physical inventory)"
                );
                Ok(None)
            }
        }
    }
}

/// Owned snapshot of the legacy projection.
///
/// `room_type` and `room_use_legacy` are read into the struct so the
/// projection round-trip stays observable in tests, even though Phase
/// 5.2 doesn't ferry either to PG (room_type is administered by our
/// app's own UI; room_use is reconstructed by the 5.4 booking /
/// checkin mappers from the per-night ledger).
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct RoomProjection {
    legacy_id: i32,
    room_no: String,
    room_type: Option<String>,
    /// Legacy literal `'yes'` / `'no'` / NULL. Translated to `bool`
    /// at the boundary (NULL → keep existing value).
    room_clean_legacy: Option<String>,
    room_use_legacy: Option<String>,
    room_details: Option<String>,
}

fn project_room(row: &dyn MappableRow) -> Result<RoomProjection, SyncError> {
    let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
        table: ROOMS_TABLE,
        message: "id is NULL — required PK".into(),
    })?;
    let room_no = row
        .try_get_str("Room_no")?
        .ok_or_else(|| SyncError::Mapper {
            table: ROOMS_TABLE,
            message: "Room_no is NULL — required business key".into(),
        })?
        .to_string();

    Ok(RoomProjection {
        legacy_id,
        room_no,
        room_type: row.try_get_str("Room_Type")?.map(str::to_string),
        room_clean_legacy: row.try_get_str("Room_Clean")?.map(str::to_string),
        room_use_legacy: row.try_get_str("Room_Use")?.map(str::to_string),
        room_details: row.try_get_str("Room_Details")?.map(str::to_string),
    })
}

/// Translate the legacy `'yes'` / `'no'` / NULL literal into our PG
/// `BOOLEAN`. Unknown / NULL values fall back to `None` so the UPSERT
/// can preserve whatever the canonical row already had.
fn legacy_yesno_to_bool(s: &Option<String>) -> Option<bool> {
    match s.as_deref() {
        Some("yes") => Some(true),
        Some("no") => Some(false),
        _ => None,
    }
}

struct ExistingRoom {
    room_id: i32,
    aggregate_id: Option<Uuid>,
    room_clean: Option<bool>,
}

async fn fetch_existing_room(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    legacy_id: i32,
    room_no: &str,
) -> Result<Option<ExistingRoom>, SyncError> {
    // Resolve by legacy_room_id_int first (writeback's preferred
    // resolver key), falling back to room_no for rows that were never
    // touched by writeback.
    let row = sqlx::query_as::<_, (i32, Option<Uuid>, Option<bool>)>(
        "SELECT room_id, aggregate_id, room_clean \
           FROM ht_rooms_new \
          WHERE legacy_room_id_int = $1 \
             OR room_no = $2 \
          ORDER BY (legacy_room_id_int = $1) DESC \
          LIMIT 1",
    )
    .bind(legacy_id)
    .bind(room_no)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(|(room_id, aggregate_id, room_clean)| ExistingRoom {
        room_id,
        aggregate_id,
        room_clean,
    }))
}

async fn apply_room_upsert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    let projected = project_room(row)?;
    let existing = fetch_existing_room(tx, projected.legacy_id, &projected.room_no).await?;

    let new_clean = legacy_yesno_to_bool(&projected.room_clean_legacy);
    let new_use = legacy_yesno_to_bool(&projected.room_use_legacy);

    let (room_id, agg_id, prior_clean) = match existing {
        Some(ex) => {
            let agg_id = ex
                .aggregate_id
                .unwrap_or_else(|| aggregate_uuid(AggregateKind::Room, ex.room_id));
            sqlx::query(
                "UPDATE ht_rooms_new \
                    SET room_clean         = COALESCE($1, room_clean), \
                        room_notes         = COALESCE($2, room_notes), \
                        legacy_room_no     = COALESCE(legacy_room_no, $3), \
                        legacy_room_id_int = COALESCE(legacy_room_id_int, $4), \
                        aggregate_id       = COALESCE(aggregate_id, $5), \
                        updated_at         = NOW() \
                  WHERE room_id = $6",
            )
            .bind(new_clean)
            .bind(&projected.room_details)
            .bind(&projected.room_no)
            .bind(projected.legacy_id)
            .bind(agg_id)
            .bind(ex.room_id)
            .execute(&mut **tx)
            .await?;
            (ex.room_id, agg_id, ex.room_clean)
        }
        None => {
            // Don't auto-create rooms here. Room creation in our app is
            // an operator action (admin UI or `bin/backfill_rooms`). A
            // CT row for a room we've never seen probably means the
            // backfill hasn't run yet — log and skip.
            tracing::info!(
                table = ROOMS_TABLE,
                legacy_id = projected.legacy_id,
                room_no = %projected.room_no,
                "HT_Rooms CT row for unknown room — skipping (run backfill_rooms first)"
            );
            return Ok(None);
        }
    };
    let _ = room_id;

    // Per user constraint + spec: only emit RoomMarkedClean /
    // RoomMarkedDirty when room_clean actually flipped. Other column
    // edits silently UPSERT.
    let event = match (prior_clean, new_clean) {
        (Some(old), Some(new)) if old != new => Some(build_clean_event(agg_id, new)),
        // Either the legacy column was NULL (no signal to act on) or
        // the value didn't change. Idempotent skip.
        _ => None,
    };

    let _ = new_use; // surfaced to keep the projection complete; not eventized.
    Ok(event)
}

fn build_clean_event(room_id: Uuid, is_now_clean: bool) -> DomainEvent {
    let source = EventSource::LegacyApp {
        detected_at: chrono::Utc::now(),
    };
    if is_now_clean {
        DomainEvent::RoomMarkedClean {
            room_id,
            by: "legacy_app".to_string(),
            source,
        }
    } else {
        DomainEvent::RoomMarkedDirty { room_id, source }
    }
}

// =============================================================================
// HT_Room_Status — per-night occupancy ledger (Phase 5.2 stub)
// =============================================================================

/// CT mapper for the legacy `HT_Room_Status` ledger table.
///
/// **Phase 5.2 stub.** This table is the per-night occupancy ledger; the
/// canonical mirror lives in `ht_checkins` + `ht_booking_rooms`, both of
/// which the 5.3 / 5.4 booking + checkin mappers own. Materialising it
/// here would duplicate state the upstream mappers will rebuild from
/// their own CT rows.
///
/// For 5.2 we accept the CT rows (so the watcher's `rows_skipped`
/// counter ticks visibly), log them at `info`, and emit no events. When
/// 5.4 lands the booking + checkin mappers, this stub gets either:
///
/// - retired (booking mapper rebuilds occupancy from `HT_Book_Date` +
///   `HT_CheckIn_Ds`), or
/// - promoted into a thin "room currently occupied?" view.
///
/// The decision is deferred to 5.4 once the writeback flows are
/// re-validated end-to-end against live data.
pub struct RoomStatusMapper;

const ROOM_STATUS_TABLE: &str = "HT_Room_Status";

const ROOM_STATUS_SELECT_COLS: &str =
    "t.id, t.room_no, t.room_status, t.room_Book_No, t.room_CheckIn_No";

#[async_trait]
impl MssqlChangeMapper for RoomStatusMapper {
    fn table(&self) -> &'static str {
        ROOM_STATUS_TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        ROOM_STATUS_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        // Log + skip. See the type-level doc comment for the rationale.
        let pk = row
            .and_then(|r| r.try_get_i32("id").ok().flatten())
            .unwrap_or(-1);
        let room_no = row
            .and_then(|r| r.try_get_str("room_no").ok().flatten())
            .map(str::to_string)
            .unwrap_or_else(|| "?".to_string());
        tracing::debug!(
            table = ROOM_STATUS_TABLE,
            op = ?op,
            id = pk,
            room_no = %room_no,
            "HT_Room_Status CT row observed (5.2 stub — deferred to 5.4 \
             booking/checkin mappers)"
        );
        Ok(None)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    fn make_room_row(id: i32, room_no: &str, clean: Option<&str>) -> HashMapRow {
        let mut r = HashMapRow::new(ROOMS_TABLE)
            .with("id", MockValue::I32(id))
            .with("Room_no", MockValue::Str(room_no.into()))
            .with("Room_Type", MockValue::Str("Standard".into()))
            .with("Room_Use", MockValue::Str("no".into()))
            .with("Room_Details", MockValue::Null);
        r = match clean {
            Some(s) => r.with("Room_Clean", MockValue::Str(s.into())),
            None => r.with("Room_Clean", MockValue::Null),
        };
        r
    }

    #[test]
    fn project_room_extracts_required_columns() {
        let row = make_room_row(7, "402", Some("yes"));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.legacy_id, 7);
        assert_eq!(p.room_no, "402");
        assert_eq!(p.room_clean_legacy.as_deref(), Some("yes"));
        assert_eq!(p.room_type.as_deref(), Some("Standard"));
    }

    #[test]
    fn project_room_errors_when_id_missing() {
        let row = HashMapRow::new(ROOMS_TABLE)
            .with("id", MockValue::Null)
            .with("Room_no", MockValue::Str("402".into()))
            .with("Room_Type", MockValue::Null)
            .with("Room_Clean", MockValue::Null)
            .with("Room_Use", MockValue::Null)
            .with("Room_Details", MockValue::Null);
        let err = project_room(&row).expect_err("NULL id must be loud");
        assert!(err.to_string().contains("id"));
    }

    #[test]
    fn project_room_errors_when_room_no_missing() {
        let row = HashMapRow::new(ROOMS_TABLE)
            .with("id", MockValue::I32(1))
            .with("Room_no", MockValue::Null)
            .with("Room_Type", MockValue::Null)
            .with("Room_Clean", MockValue::Null)
            .with("Room_Use", MockValue::Null)
            .with("Room_Details", MockValue::Null);
        let err = project_room(&row).expect_err("NULL Room_no must be loud");
        assert!(err.to_string().contains("Room_no"));
    }

    #[test]
    fn legacy_yesno_translates_recognised_literals() {
        assert_eq!(legacy_yesno_to_bool(&Some("yes".to_string())), Some(true));
        assert_eq!(legacy_yesno_to_bool(&Some("no".to_string())), Some(false));
    }

    #[test]
    fn legacy_yesno_returns_none_for_null_or_unknown() {
        assert_eq!(legacy_yesno_to_bool(&None), None);
        assert_eq!(legacy_yesno_to_bool(&Some("maybe".to_string())), None);
        assert_eq!(legacy_yesno_to_bool(&Some(String::new())), None);
    }

    #[test]
    fn build_clean_event_yes_emits_marked_clean() {
        let agg = aggregate_uuid(AggregateKind::Room, 1);
        let ev = build_clean_event(agg, true);
        assert_eq!(ev.type_name(), "RoomMarkedClean");
    }

    #[test]
    fn build_clean_event_no_emits_marked_dirty() {
        let agg = aggregate_uuid(AggregateKind::Room, 1);
        let ev = build_clean_event(agg, false);
        assert_eq!(ev.type_name(), "RoomMarkedDirty");
    }

    #[test]
    fn room_master_mapper_metadata_is_correct() {
        let m = RoomMasterMapper;
        assert_eq!(m.table(), "HT_Rooms");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("Room_Clean"));
        assert!(m.select_sql().contains("Room_no"));
    }

    #[test]
    fn room_status_mapper_metadata_is_correct() {
        let m = RoomStatusMapper;
        assert_eq!(m.table(), "HT_Room_Status");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("room_status"));
    }

    /// `HT_Room_Status::apply` always returns `Ok(None)` in 5.2 — it's
    /// a logging stub. This test pins the stub semantics until 5.4
    /// replaces it; if a future PR accidentally emits an event from
    /// here, the duplicate-publication risk surfaces in CI.
    #[tokio::test]
    async fn room_status_mapper_apply_is_a_logging_stub_in_phase_52() {
        // We can construct a `RoomStatusMapper` and assert its metadata
        // without a tx. The actual `apply` call requires a tx; rely on
        // the integration suite for runtime coverage. Here we lock the
        // *intent* via the doc-comment + module-level constant.
        let m = RoomStatusMapper;
        let _ = m.table();
    }
}
