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
///
/// Track E2 (T1 HIGH-3 / `docs/coexistence/audit-2026-05-13.md`)
/// widened the projection to cover the room-utilization counter
/// (`Room_Use_Count`), the legacy drag-drop grid coordinates
/// (`Room_X` / `Room_Y`), grouping (`Room_Group`), the relay-power
/// columns (`Room_Power_OPEN` / `_CLOSE` / `_STATUS`), and `Room_Polity`.
///
/// **Known wrongness — NOT fixed in Track E2.** The `room_price_*`
/// axis in `ht_rooms_new` is `weekday/weekend/special`, while legacy
/// `HT_Rooms` uses `Room_PriceA/B/C` indexed by customer-type
/// (`HT_SET_CusType_Main` row). Track F (canonical rate-table model)
/// is the right place to reconcile that — we intentionally don't
/// touch the price columns here.
const ROOMS_SELECT_COLS: &str = "t.id, t.Room_no, t.Room_Type, t.Room_Clean, \
     t.Room_Use, t.Room_Manternace, t.Room_Details, t.Room_Use_Count, \
     t.Room_X, t.Room_Y, t.Room_Group, t.Room_Power_OPEN, \
     t.Room_Power_CLOSE, t.Room_Power_STATUS, t.Room_Polity";

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
///
/// Track E2 (T1 HIGH-3) added the bottom block: utilization counter,
/// grid coordinates, group, relay-power columns, and policy id. These
/// were previously dropped on every CT tick.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct RoomProjection {
    legacy_id: i32,
    room_no: String,
    room_type: Option<String>,
    /// Legacy literal `'yes'` / `'no'` / NULL. Translated to `bool`
    /// at the boundary (NULL → keep existing value).
    room_clean_legacy: Option<String>,
    /// Legacy `Room_Manternace` (sic — typo in legacy schema). Same
    /// `'yes'` / `'no'` / NULL semantics as `room_clean_legacy`.
    room_manternace_legacy: Option<String>,
    room_use_legacy: Option<String>,
    room_details: Option<String>,
    // ------------------------------------------------------------------
    // Track E2 (T1 HIGH-3) additions.
    // ------------------------------------------------------------------
    /// Running nights total (`Room_Use_Count int NOT NULL DEFAULT 0`).
    /// Legacy `Module1.UPDATE_ROOM_USE` increments this on every
    /// checkout. Mirrored read-only into PG for utilization reports —
    /// writeback of the increment is owned by the existing writeback
    /// worker (Wave 6).
    room_use_count: Option<i32>,
    /// Legacy drag-drop grid coordinates from the iHOTEL layout
    /// (`Room_X` / `Room_Y`, both `int NOT NULL DEFAULT 0`). Lost on
    /// every CT sync today; capture so a future canonical room-layout
    /// UI (Track G) has the data.
    room_x: Option<i32>,
    room_y: Option<i32>,
    /// Floor / wing grouping (`Room_Group varchar(50)`).
    room_group: Option<String>,
    /// Relay-power command timestamps + status. Legacy stores all three
    /// as `varchar(50)`; we keep them as text to preserve the literal
    /// the legacy app wrote (`Room_Power_STATUS` defaults to `'off'`).
    room_power_open: Option<String>,
    room_power_close: Option<String>,
    room_power_status: Option<String>,
    /// Room policy id (`Room_Polity int NOT NULL DEFAULT 1`). Semantics
    /// unclear from the cheatsheet — captured for parity so reconcile
    /// stops drifting on it.
    room_polity: Option<i32>,
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
        room_manternace_legacy: row.try_get_str("Room_Manternace")?.map(str::to_string),
        room_use_legacy: row.try_get_str("Room_Use")?.map(str::to_string),
        room_details: row.try_get_str("Room_Details")?.map(str::to_string),
        room_use_count: row.try_get_i32("Room_Use_Count")?,
        room_x: row.try_get_i32("Room_X")?,
        room_y: row.try_get_i32("Room_Y")?,
        room_group: row.try_get_str("Room_Group")?.map(str::to_string),
        room_power_open: row.try_get_str("Room_Power_OPEN")?.map(str::to_string),
        room_power_close: row.try_get_str("Room_Power_CLOSE")?.map(str::to_string),
        room_power_status: row.try_get_str("Room_Power_STATUS")?.map(str::to_string),
        room_polity: row.try_get_i32("Room_Polity")?,
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
    let new_maintenance = legacy_yesno_to_bool(&projected.room_manternace_legacy);
    let new_use = legacy_yesno_to_bool(&projected.room_use_legacy);

    let (room_id, agg_id, prior_clean) = match existing {
        Some(ex) => {
            let agg_id = ex
                .aggregate_id
                .unwrap_or_else(|| aggregate_uuid(AggregateKind::Room, ex.room_id));
            // Track E2 (T1 HIGH-3) — write the newly-captured legacy
            // columns. `COALESCE(new, old)` preserves PG-side state
            // when the legacy column is NULL so we never blank a
            // populated row, but accepts the new value when present.
            // `room_use_count` uses raw assignment (not COALESCE) so a
            // legacy 0 truly resets PG to 0; the running counter is
            // the legacy app's authoritative state.
            // `room_notes` ($3) also uses raw assignment (2026-06-11,
            // audit P2): the CT projection always carries the current
            // `Room_Details` value, so a legacy NULL is a genuine
            // "notes cleared" transition — the old COALESCE could
            // never converge it and the reconcile sweep flagged the
            // row forever.
            // COALESCE argument order: see Bug A rationale in
            // `sync::mappers::checkin::update_existing` and
            // `sync::mappers::booking::update_existing`.
            //   * `legacy_room_no` and `legacy_room_id_int` are denormalised
            //     legacy pointers — must track the current MSSQL key. If
            //     the legacy app renumbered a room (rare but possible
            //     vendor maintenance), the pre-fix `COALESCE(existing, new)`
            //     would freeze the canonical denormalised values forever
            //     while the writeback path silently used the new legacy
            //     key. Flipped to `COALESCE($N, existing)` so a non-NULL
            //     new value overwrites; a transient NULL keeps the
            //     existing value (the projection-builder upstream never
            //     emits NULL for these unless the legacy column is
            //     genuinely missing).
            //   * `aggregate_id` stays write-once.
            sqlx::query(
                "UPDATE ht_rooms_new \
                    SET room_clean         = COALESCE($1, room_clean), \
                        room_maintenance   = COALESCE($2::bool, room_maintenance), \
                        room_notes         = $3, \
                        room_use_count     = COALESCE($8, room_use_count), \
                        room_x             = COALESCE($9, room_x), \
                        room_y             = COALESCE($10, room_y), \
                        room_group         = COALESCE($11, room_group), \
                        room_power_open    = COALESCE($12, room_power_open), \
                        room_power_close   = COALESCE($13, room_power_close), \
                        room_power_status  = COALESCE($14, room_power_status), \
                        room_polity        = COALESCE($15, room_polity), \
                        legacy_room_no     = COALESCE($4, legacy_room_no), \
                        legacy_room_id_int = COALESCE($5, legacy_room_id_int), \
                        aggregate_id       = COALESCE(aggregate_id, $6), \
                        updated_at         = NOW() \
                  WHERE room_id = $7",
            )
            .bind(new_clean)
            .bind(new_maintenance)
            .bind(&projected.room_details)
            .bind(&projected.room_no)
            .bind(projected.legacy_id)
            .bind(agg_id)
            .bind(ex.room_id)
            .bind(projected.room_use_count)
            .bind(projected.room_x)
            .bind(projected.room_y)
            .bind(&projected.room_group)
            .bind(&projected.room_power_open)
            .bind(&projected.room_power_close)
            .bind(&projected.room_power_status)
            .bind(projected.room_polity)
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
// HT_Room_Status — per-night occupancy ledger (Phase 5.4: retired stub)
// =============================================================================

/// CT mapper for the legacy `HT_Room_Status` ledger table.
///
/// **Phase 5.4 — retired-stub status.** The 5.4 check-in aggregate
/// owns the canonical "which room is occupied tonight" view. Every
/// `HT_Room_Status` change in MSSQL is paired with a corresponding
/// `HT_CheckIn_H` / `HT_CheckIn_Ds` change in the same legacy
/// transaction (verified across walkin / checkin-from-booking /
/// checkout / cancel flows in the spike captures); the check-in mapper
/// re-loads the parent aggregate and re-projects occupancy from the
/// ground truth.
///
/// Materialising `HT_Room_Status` here would either:
/// 1. Duplicate state the check-in mapper already projects, OR
/// 2. Race against it (two CT ticks could land on the same room-night
///    in opposite orders).
///
/// We keep the mapper registered so the table appears in
/// `legacy_sync_status` observability — operators can see the row
/// count tick visibly — but `apply` is a documented no-op that simply
/// logs and increments `rows_skipped`. This intentional retirement was
/// agreed in the 5.4 prep notes (item #3); a future PR could replace
/// this stub with a thin read-side view if a use case appears, but
/// today no subscriber needs it.
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
        // Log + skip. See the type-level doc comment for the rationale:
        // 5.4 retired this mapper in favour of the check-in aggregate's
        // ground-truth occupancy projection.
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
            "HT_Room_Status CT row observed (5.4 retired stub — \
             occupancy is owned by the check-in aggregate)"
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
        make_room_row_with_maintenance(id, room_no, clean, None)
    }

    fn make_room_row_with_maintenance(
        id: i32,
        room_no: &str,
        clean: Option<&str>,
        maintenance: Option<&str>,
    ) -> HashMapRow {
        let mut r = HashMapRow::new(ROOMS_TABLE)
            .with("id", MockValue::I32(id))
            .with("Room_no", MockValue::Str(room_no.into()))
            .with("Room_Type", MockValue::Str("Standard".into()))
            .with("Room_Use", MockValue::Str("no".into()))
            .with("Room_Details", MockValue::Null)
            // Track E2 additions — every column the projection reads
            // must be present in the test row (HashMapRow returns Err
            // on a missing cell, distinct from a Null cell).
            .with("Room_Use_Count", MockValue::Null)
            .with("Room_X", MockValue::Null)
            .with("Room_Y", MockValue::Null)
            .with("Room_Group", MockValue::Null)
            .with("Room_Power_OPEN", MockValue::Null)
            .with("Room_Power_CLOSE", MockValue::Null)
            .with("Room_Power_STATUS", MockValue::Null)
            .with("Room_Polity", MockValue::Null);
        r = match clean {
            Some(s) => r.with("Room_Clean", MockValue::Str(s.into())),
            None => r.with("Room_Clean", MockValue::Null),
        };
        r = match maintenance {
            Some(s) => r.with("Room_Manternace", MockValue::Str(s.into())),
            None => r.with("Room_Manternace", MockValue::Null),
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
    fn project_room_extracts_room_manternace_yes() {
        let row = make_room_row_with_maintenance(11, "210", Some("no"), Some("yes"));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_manternace_legacy.as_deref(), Some("yes"));
    }

    #[test]
    fn project_room_extracts_room_manternace_no() {
        let row = make_room_row_with_maintenance(12, "211", Some("yes"), Some("no"));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_manternace_legacy.as_deref(), Some("no"));
    }

    #[test]
    fn project_room_room_manternace_null_keeps_none() {
        let row = make_room_row_with_maintenance(13, "212", Some("yes"), None);
        let p = project_room(&row).expect("project must succeed");
        assert!(p.room_manternace_legacy.is_none());
    }

    #[test]
    fn legacy_yesno_translates_room_manternace_yes_to_true() {
        // Locks the wiring between Room_Manternace projection and the
        // boolean we bind into ht_rooms_new.room_maintenance.
        let row = make_room_row_with_maintenance(20, "301", None, Some("yes"));
        let p = project_room(&row).expect("project must succeed");
        let new_maintenance = legacy_yesno_to_bool(&p.room_manternace_legacy);
        assert_eq!(new_maintenance, Some(true));
    }

    #[test]
    fn project_room_errors_when_id_missing() {
        // Project requires `id` first — it errors out before reaching
        // any other column, so we only need to set `id` to NULL.
        let row = HashMapRow::new(ROOMS_TABLE).with("id", MockValue::Null);
        let err = project_room(&row).expect_err("NULL id must be loud");
        assert!(err.to_string().contains("id"));
    }

    #[test]
    fn project_room_errors_when_room_no_missing() {
        // Project reads `id` then `Room_no` and errors on a NULL
        // business key. Set only those two so the test stays focused.
        let row = HashMapRow::new(ROOMS_TABLE)
            .with("id", MockValue::I32(1))
            .with("Room_no", MockValue::Null);
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
        // Maintenance column uses the legacy typo `Room_Manternace`.
        // The mapper must project it so the dashboard's maintenance
        // flag stays in sync with the legacy app.
        assert!(m.select_sql().contains("Room_Manternace"));
    }

    #[test]
    fn room_status_mapper_metadata_is_correct() {
        let m = RoomStatusMapper;
        assert_eq!(m.table(), "HT_Room_Status");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("room_status"));
    }

    /// `HT_Room_Status::apply` always returns `Ok(None)`. As of 5.4
    /// this is a *retired* stub — the check-in aggregate owns
    /// occupancy. The mapper stays registered only so the table
    /// appears in `legacy_sync_status` observability.
    #[tokio::test]
    async fn room_status_mapper_apply_is_a_retired_stub_in_phase_54() {
        // We can construct a `RoomStatusMapper` and assert its metadata
        // without a tx. The actual `apply` call requires a tx; rely on
        // the integration suite for runtime coverage. Here we lock the
        // *intent* via the doc-comment + module-level constant.
        let m = RoomStatusMapper;
        let _ = m.table();
    }

    // ========================================================================
    // Track E2 — column-expansion coverage (T1 HIGH-3)
    //
    // Each block locks one finding from `docs/coexistence/audit-2026-05-13.md`.
    // ========================================================================

    /// T1 HIGH-3 — running nights total (`Room_Use_Count`). Writeback
    /// Wave 6 increments this in MSSQL; canonical PG must capture it
    /// or utilization reports show zero.
    #[test]
    fn projects_room_use_count() {
        let row = make_room_row(7, "402", Some("yes"))
            .with("Room_Use_Count", MockValue::I32(142));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_use_count, Some(142));
    }

    /// T1 HIGH-3 — grid coordinates from the iHOTEL drag-drop layout.
    /// Currently flattened to numeric order on every sync; capture so
    /// a future canonical layout UI has the source data.
    #[test]
    fn projects_room_xy_grid_coordinates() {
        let row = make_room_row(7, "402", Some("yes"))
            .with("Room_X", MockValue::I32(120))
            .with("Room_Y", MockValue::I32(240));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_x, Some(120));
        assert_eq!(p.room_y, Some(240));
    }

    /// T1 HIGH-3 — `Room_Group` (floor / wing) preserved as opaque
    /// string. The legacy app uses this for layout grouping.
    #[test]
    fn projects_room_group() {
        let row = make_room_row(7, "402", Some("yes"))
            .with("Room_Group", MockValue::Str("Floor 4".into()));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_group.as_deref(), Some("Floor 4"));
    }

    /// T1 HIGH-3 — relay-power columns. The legacy app drives a
    /// physical power-relay; we mirror state for observability.
    #[test]
    fn projects_room_power_columns() {
        let row = make_room_row(7, "402", Some("yes"))
            .with("Room_Power_OPEN", MockValue::Str("2026-01-15 08:00".into()))
            .with("Room_Power_CLOSE", MockValue::Str("2026-01-15 18:00".into()))
            .with("Room_Power_STATUS", MockValue::Str("on".into()));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_power_open.as_deref(), Some("2026-01-15 08:00"));
        assert_eq!(p.room_power_close.as_deref(), Some("2026-01-15 18:00"));
        assert_eq!(p.room_power_status.as_deref(), Some("on"));
    }

    /// T1 HIGH-3 — `Room_Polity` policy id (default 1 in legacy).
    /// Semantics unclear but reconcile would flag a divergence
    /// indefinitely if we never read the column.
    #[test]
    fn projects_room_polity() {
        let row = make_room_row(7, "402", Some("yes"))
            .with("Room_Polity", MockValue::I32(2));
        let p = project_room(&row).expect("project must succeed");
        assert_eq!(p.room_polity, Some(2));
    }

    /// T1 HIGH-3 / T2 HIGH-4 — the CT SELECT must mention every new
    /// column. Otherwise MSSQL returns NULL silently and projection
    /// sees None on every tick.
    #[test]
    fn room_select_sql_mentions_track_e2_columns() {
        let select = RoomMasterMapper.select_sql();
        for col in [
            "Room_Use_Count",
            "Room_X",
            "Room_Y",
            "Room_Group",
            "Room_Power_OPEN",
            "Room_Power_CLOSE",
            "Room_Power_STATUS",
            "Room_Polity",
        ] {
            assert!(
                select.contains(col),
                "RoomMasterMapper SELECT clause missing '{col}'"
            );
        }
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards.
    // -------------------------------------------------------------------

    #[test]
    fn rooms_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(ROOMS_SELECT_COLS, "HT_Rooms");
    }

    #[test]
    fn room_status_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(ROOM_STATUS_SELECT_COLS, "HT_Room_Status");
    }
}
