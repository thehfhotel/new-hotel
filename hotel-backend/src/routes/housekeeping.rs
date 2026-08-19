//! Housekeeping API routes — clean / dirty / maintenance toggles.
//!
//! Thin wrappers over [`crate::service::HousekeepingService`], which owns the
//! canonical `ht_rooms_new.room_clean` / `room_maintenance` flips, the legacy
//! writeback enqueue (`MarkRoomClean` / `MarkRoomDirty` / `SetRoomMaintenance`
//! — spike §3j + coexistence audit 2026-06-11 P2), and the domain-event
//! publish. Until this module the service was wired into `AppState` but
//! **unrouted**: the housekeeping board's "mark dirty" / "mark clean" actions
//! PATCHed `/api/rooms/:id/status` with a string that never touched
//! `room_clean` and never reached iHOTEL (a no-op), and the out-of-service
//! toggle likewise never mirrored to legacy `HT_Rooms.Room_Manternace`, so the
//! legacy grid kept renting a room taken out of service in the new app.
//!
//! **Branch-aware:** `?branch=hfville` targets the `hotelville` canonical pool,
//! everything else the primary HF Hotel pool — mirroring `routes::new_shifts`.
//! A fresh `HousekeepingService` is built per request bound to the resolved
//! pool (the AppState-wired instance is hardwired to the primary pool at
//! startup); construction is cheap (Arc clones + a pool handle clone). HF Ville
//! mutations are still gated by the `ville_write_guard` middleware in `main.rs`
//! until `HFVILLE_WRITES_ENABLED` is flipped — this module relies on that
//! existing safety net rather than re-checking the flag.
//!
//! **Reception read (wave-4 B6):** [`list_cleaning_progress`]
//! (`GET /api/housekeeping/cleaning`) exposes today's maid-reported progress
//! from `ht_hk_cleaning_events` so the แผนกแม่บ้าน board can show who is in a
//! room and since when. Until it existed, that table was read by nothing
//! outside `routes::hk` — the maid reported progress and reception could not
//! see it.
//!
//! **Reception truth (wave-5 IF-1):** the same endpoint additionally serves
//! `legacyStatusStale`, `legacyClean[]` and the `housekeeping[]` axis — the
//! iHOTEL-wins merged cleanliness the `/hk` maid surface already shows, plus
//! the divergence it deliberately hides from her. Reception is the desk that
//! reconciles the two boards, so it is the one audience the disagreement is
//! actionable for. All read-side: ZERO new write shapes, and `cleaning` has no
//! legacy counterpart to mirror.
//!
//! All SQL here and in the service is RUNTIME `sqlx::query` (no compile-time
//! macro), so these routes need no `.sqlx/` cache regeneration.

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::domain::user::User;
use crate::error::ApiResult;
use crate::legacy_room_status::{RoomFlagsOutcome, RoomFlagsReaders};
use crate::outbox::event::EventSource;
use crate::service::{
    HousekeepingService, MarkCleanCommand, MarkDirtyCommand, MarkMaintenanceCommand,
};

/// Operator label stamped into the legacy `HT_Housewife` audit row when the
/// caller doesn't supply one. Auth context isn't wired into these routes yet
/// (`AUTH_ENABLED` is off in production), so we fall back to a stable sentinel
/// rather than tripping the service's non-empty `by` validation.
const DEFAULT_BY: &str = "Front Desk";

/// Branch selector shared by every housekeeping route.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HousekeepingQuery {
    pub branch: Option<Branch>,
}

/// Body for the clean / dirty actions. `by` lands in the legacy `HT_Housewife`
/// audit row (the housekeeper / operator). Optional — defaults to
/// [`DEFAULT_BY`] when absent or blank.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HousekeepingActionBody {
    pub by: Option<String>,
}

/// Body for the maintenance toggle.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceBody {
    /// `true` ⇒ take the room out of service (legacy `Room_Manternace='yes'`);
    /// `false` ⇒ return it to service.
    pub maintenance: bool,
}

/// Uniform success envelope for the housekeeping mutations.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HousekeepingResponse {
    pub success: bool,
    pub message: String,
    pub room_id: i32,
}

/// Build a [`HousekeepingService`] bound to the branch's pool. Reuses the
/// AppState repository / outbox / event-bus handles (cheap Arc clones) and the
/// resolved pool handle — same construction shape as `AppState::wire_services`.
fn service_for(state: &AppState, branch: Option<Branch>) -> ApiResult<HousekeepingService> {
    // Delegate the Hfville→ville_pool decision to the unified write chokepoint.
    let pool = state.write_pool(branch)?.clone();
    Ok(HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Resolve the operator label: prefer the authenticated session user
/// (Task #40), then the body-supplied value, then [`DEFAULT_BY`]. Auth
/// ships dark, so `actor` is `None` and the body value wins exactly as
/// before; once auth is on the housekeeper's login is recorded instead.
fn resolve_by(actor: Option<&User>, by: Option<String>) -> String {
    super::resolve_actor(actor, by.as_deref()).unwrap_or_else(|| DEFAULT_BY.to_string())
}

/// `EventSource` for an HTTP-originated housekeeping mutation. Mirrors
/// `routes::new_bookings` — a real `user_id` lands here once auth is wired.
fn http_source() -> EventSource {
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

/// POST /api/housekeeping/rooms/{id}/clean — mark a room clean.
pub async fn mark_clean(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    actor: Option<Extension<User>>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(actor.as_deref(), body.and_then(|Json(b)| b.by));
    let outcome = svc
        .mark_clean(MarkCleanCommand {
            room_id,
            by,
            source: http_source(),
        })
        .await?;
    Ok(Json(HousekeepingResponse {
        success: true,
        message: "Room marked clean".to_string(),
        room_id: outcome.room_id,
    }))
}

/// POST /api/housekeeping/rooms/{id}/dirty — mark a room dirty.
pub async fn mark_dirty(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    actor: Option<Extension<User>>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(actor.as_deref(), body.and_then(|Json(b)| b.by));
    let outcome = svc
        .mark_dirty(MarkDirtyCommand {
            room_id,
            by,
            source: http_source(),
        })
        .await?;
    Ok(Json(HousekeepingResponse {
        success: true,
        message: "Room marked dirty".to_string(),
        room_id: outcome.room_id,
    }))
}

/// POST /api/housekeeping/rooms/{id}/maintenance — toggle out-of-service.
pub async fn set_maintenance(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    Json(body): Json<MaintenanceBody>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let outcome = svc
        .set_maintenance(MarkMaintenanceCommand {
            room_id,
            maintenance: body.maintenance,
            source: http_source(),
        })
        .await?;
    let message = if body.maintenance {
        "Room sent to maintenance".to_string()
    } else {
        "Room returned to service".to_string()
    };
    Ok(Json(HousekeepingResponse {
        success: true,
        message,
        room_id: outcome.room_id,
    }))
}

// ============================================================================
// Reception read — today's maid cleaning progress
// ============================================================================

/// One room's LATEST maid-reported cleaning event today (Thai day).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomCleaningProgress {
    pub room_id: i32,
    pub room_no: String,
    /// `started` | `done` | `dirty`.
    pub status: String,
    /// Verified HF ID badge — ALWAYS present.
    pub badge: String,
    /// Display-name snapshot; usually `null` today (the CF IdP forwards only
    /// `apps` + `badge`). Render `name ?? badge`.
    pub name: Option<String>,
    /// A real instant (`TIMESTAMPTZ`), NOT a naive legacy MSSQL datetime.
    /// Render it with normal local-time formatting — the `timeZone:'UTC'` rule
    /// applies ONLY to values mirrored from legacy (see `app/hk/hk-lib.ts`).
    pub at: DateTime<Utc>,
}

/// What iHOTEL said about one room's cleanliness, in CANONICAL polarity
/// (`true` = IS clean). Only rooms iHOTEL had a usable value for appear —
/// an unrecognised legacy literal is UNKNOWN, never guessed
/// (`legacy_room_status::legacy_clean_to_is_clean`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyRoomClean {
    pub room_no: String,
    pub clean: bool,
}

/// One room on the HOUSEKEEPING axis — the second axis reception reads
/// alongside `/api/rooms`' availability `status`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomHousekeeping {
    pub room_no: String,
    /// `clean` | `cleaning` | `dirty` — see [`derive_hk_status`].
    pub hk_status: String,
    /// iHOTEL and canonical PG disagree about this room's cleanliness.
    ///
    /// SHOWN to reception, deliberately — the opposite of the `/hk` maid
    /// surface, which suppresses it (`routes::hk`'s
    /// `the_maid_never_receives_the_canonical_second_opinion`). The maid has
    /// exactly one job per room and no action to take about which database is
    /// behind; the receptionist is the person who reconciles the two boards,
    /// so for her the disagreement IS the actionable fact. Matching is the
    /// norm, divergence is the anomaly.
    pub divergent: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningProgressResponse {
    pub success: bool,
    pub data: Vec<RoomCleaningProgress>,
    /// `true` when the iHOTEL read could not answer, so every `housekeeping[]`
    /// entry below is derived from the canonical PG MIRROR rather than iHOTEL
    /// truth. Same meaning and same four collapsed failure modes as
    /// `routes::hk`'s `RoomsResponse::legacy_status_stale`.
    ///
    /// ADDITIVE and ALWAYS serialized, so a client branches on the VALUE
    /// rather than on whether the key exists — a rollback must not be able to
    /// paint a permanent stale banner.
    pub legacy_status_stale: bool,
    /// Raw iHOTEL answer per room. Empty when `legacy_status_stale`.
    pub legacy_clean: Vec<LegacyRoomClean>,
    /// EVERY active room on the housekeeping axis (unlike `data`, which only
    /// carries rooms a maid reported on today).
    pub housekeeping: Vec<RoomHousekeeping>,
}

/// The housekeeping axis. A SECOND AXIS, never an availability tier.
///
/// `overlay_live_status` (`routes::new_rooms`) deliberately excludes
/// cleanliness from availability — iHOTEL does not gate check-in on the clean
/// flag, so neither do we — and that exclusion is pinned by
/// `room_stored_cleaning_is_still_derived_available`. This axis therefore sits
/// BESIDE `status`, is computed at READ TIME (no second writer, no stored
/// denormalization to distrust), and is exposed on THIS reception endpoint
/// only — never on `/api/rooms`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkStatus {
    Clean,
    Cleaning,
    Dirty,
}

impl HkStatus {
    /// Wire literal. Kept lowercase and stable — the frontend's
    /// `HK_STATUS_LABELS` keys off exactly these three.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::Cleaning => "cleaning",
            Self::Dirty => "dirty",
        }
    }
}

/// Derive one room's housekeeping status. PURE — unit-tested below.
///
/// `room_clean` is the MERGED value (iHOTEL wins where it has an opinion),
/// not the raw canonical column. `latest_event_today` is the room's most
/// recent `ht_hk_cleaning_events` literal for the Thai day, or `None` when the
/// maid has not touched it today.
///
/// ## Evaluation order is load-bearing
///
/// `clean` is tested FIRST and unconditionally; `cleaning` and `dirty` then
/// partition the not-clean space. That is what the locked design's table says:
/// its `dirty` row carries the qualifier "latest not started" while its
/// `clean` row carries no qualifier at all. If `cleaning` outranked `clean`,
/// the `clean` row would need the same qualifier — it does not, so it wins.
///
/// The two only disagree in an anomaly anyway: the normal lifecycle is
/// dirty → `started` (still `room_clean=false`) → `done` (flips it true), so a
/// room that is clean AND mid-`started` means someone marked it clean out of
/// band. Showing สะอาด there follows the merged truth rather than a stale
/// event, which is the same precedence the rest of this endpoint applies.
pub fn derive_hk_status(room_clean: bool, latest_event_today: Option<&str>) -> HkStatus {
    if room_clean {
        HkStatus::Clean
    } else if latest_event_today == Some("started") {
        // "latest event is 'started'" is exactly the design's "'started' with
        // no later done/dirty" — the query already takes only the newest row.
        HkStatus::Cleaning
    } else {
        HkStatus::Dirty
    }
}

/// One room as read from PG, before the iHOTEL merge. Internal to the axis
/// build so the merge can be unit-tested with no database and no legacy server.
#[derive(Debug, Clone)]
pub struct RoomCleanRow {
    pub room_no: String,
    pub room_clean: bool,
    pub latest_event_today: Option<String>,
}

/// Merge iHOTEL over canonical PG and build both additive axes.
///
/// Returns `(legacy_status_stale, legacy_clean, housekeeping)`.
///
/// Same three CR-1 rules as `routes::hk::merge_legacy_room_flags` — iHOTEL
/// wins per room it has a usable value for; a room it has no usable value for
/// keeps its canonical value SILENTLY (a mapping gap is not a staleness
/// event); an unanswerable read is stale, not an error — with ONE deliberate
/// difference: the disagreement is RETURNED here instead of only logged.
///
/// PURE apart from the summary log line.
pub fn build_housekeeping_axes(
    rooms: &[RoomCleanRow],
    outcome: &RoomFlagsOutcome,
    branch_id: &str,
) -> (bool, Vec<LegacyRoomClean>, Vec<RoomHousekeeping>) {
    let legacy = match outcome {
        RoomFlagsOutcome::Available(map) => Some(map),
        RoomFlagsOutcome::Unavailable => None,
    };

    let mut legacy_clean = Vec::new();
    let mut housekeeping = Vec::with_capacity(rooms.len());
    let mut divergences = 0usize;

    for room in rooms {
        // `.is_clean` ONLY. The CR-1 read also carries `Room_Use` occupancy
        // now, but reception's board derives occupancy from its own
        // availability axis (`/api/rooms`) — pulling a second opinion in here
        // would change this surface's contract, which this change deliberately
        // does not do.
        let ihotel = legacy.and_then(|m| m.get(room.room_no.trim()).and_then(|f| f.is_clean));
        if let Some(clean) = ihotel {
            legacy_clean.push(LegacyRoomClean {
                room_no: room.room_no.clone(),
                clean,
            });
        }
        let divergent = matches!(ihotel, Some(clean) if clean != room.room_clean);
        if divergent {
            divergences += 1;
        }
        // iHOTEL wins where it has an opinion; otherwise canonical stands.
        let merged = ihotel.unwrap_or(room.room_clean);
        housekeeping.push(RoomHousekeeping {
            room_no: room.room_no.clone(),
            hk_status: derive_hk_status(merged, room.latest_event_today.as_deref())
                .as_str()
                .to_string(),
            divergent,
        });
    }

    if divergences > 0 {
        // SUMMARY only, no per-room line: reception's board polls on a timer
        // and on every SSE cleaning event, so a per-room warn here would emit
        // the same rows every few seconds. The per-room detail already exists
        // on the `/hk` path, and — the actual fix — it is now IN the response
        // for the person who can act on it.
        tracing::warn!(
            branch = branch_id,
            divergences,
            rooms = rooms.len(),
            "reception housekeeping board: iHOTEL and canonical PG disagree — \
             showing iHOTEL, flagging the rooms as divergent"
        );
    }

    (legacy.is_none(), legacy_clean, housekeeping)
}

/// GET /api/housekeeping/cleaning — reception's housekeeping truth.
///
/// The reception (แผนกแม่บ้าน) board's live feed. Before this, `ht_hk_cleaning_events`
/// was read by NOTHING outside `routes::hk` — the maid's progress existed and
/// reception could not see it, and the board's middle "กำลังทำความสะอาด" column
/// was permanently empty because it keyed off a `status='cleaning'` literal that
/// `/api/rooms` never emits (pinned by `routes::new_rooms`'
/// `room_stored_cleaning_is_still_derived_available`).
///
/// ## Wave-5: three ADDITIVE fields (IF-1)
///
/// `legacyStatusStale`, `legacyClean[]` and `housekeeping[{roomNo, hkStatus,
/// divergent}]`. Reception now gets the SAME iHOTEL-wins merged truth the maid
/// gets on `/hk`, so the two screens cannot disagree about a room — plus the
/// divergence itself, which the maid surface deliberately suppresses (see
/// [`RoomHousekeeping::divergent`]).
///
/// `hkStatus` is the second axis that finally makes กำลังทำความสะอาด real: it
/// is derived from a maid's `started` event rather than from the dead
/// `status='cleaning'` literal the old board matched on. Computed at READ
/// TIME — no second writer, no stored denormalization.
///
/// Deliberately a SEPARATE endpoint rather than widening `/api/rooms`:
/// `routes::new_rooms` is a large shared file whose live-flags scan and
/// cleanliness-is-not-a-tier rule are pinned by tests, and both stay untouched.
/// `overlay_live_status` is not modified by this change and must not be.
///
/// ZERO new write shapes: everything here is a read, and `cleaning` has no
/// legacy counterpart at all (iHOTEL has no in-progress state; `Room_Clean_Time`
/// is off limits — it drives a physical room-power countdown).
///
/// Rooms with no event today are ABSENT from `data` (the frontend reads absent
/// as "no progress reported"), so the response stays tiny — 0 rows on a quiet
/// morning, at most one row per active room.
///
/// `?branch=` is OPTIONAL here, matching its sibling housekeeping routes:
/// reception's `useBranchFetch` always sends one, and an omitted branch means
/// the primary site exactly as it does for clean/dirty/maintenance. (The `/hk`
/// MAID surface is the opposite — there the branch is mandatory, because a maid
/// picks her own property and a wrong guess files against the wrong hotel.)
pub async fn list_cleaning_progress(
    State(state): State<AppState>,
    Query(query): Query<HousekeepingQuery>,
    readers: Option<Extension<RoomFlagsReaders>>,
) -> ApiResult<Json<CleaningProgressResponse>> {
    // Same per-site chokepoint as the mutating siblings, so this read can never
    // disagree with the writes it renders.
    let pool = state.write_pool(query.branch)?;

    // `TODAY_BKK` is shared verbatim with `routes::hk` — ONE definition of
    // "today", so the maid's list and reception's board cannot disagree at a
    // day boundary.
    //
    // LEFT JOIN LATERAL (widened from INNER, wave-5): the housekeeping AXIS
    // needs every active room, not only the ones a maid touched today. `data`
    // is still filtered to rooms WITH an event below, so its published
    // contract — absent means "no progress reported" — is unchanged.
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            COALESCE(r.room_clean, true) AS room_clean,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {today}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        WHERE COALESCE(r.room_active, true) = true
        ORDER BY r.room_no
        "#,
        today = super::hk::TODAY_BKK
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pool)
        .await?;

    // Rooms a maid reported on today — the ORIGINAL `data` contract. A room
    // with no event today has a NULL `hkev_status` from the LEFT JOIN and is
    // filtered out here exactly as the INNER JOIN used to drop it.
    let data: Vec<RoomCleaningProgress> = rows
        .iter()
        .filter_map(|row| {
            let status = row
                .try_get::<Option<String>, _>("hkev_status")
                .ok()
                .flatten()?;
            Some(RoomCleaningProgress {
                room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
                room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
                status,
                badge: row
                    .try_get::<Option<String>, _>("hkev_badge")
                    .ok()
                    .flatten()
                    .unwrap_or_default(),
                name: row.try_get::<Option<String>, _>("hkev_name").ok().flatten(),
                at: row
                    .try_get::<DateTime<Utc>, _>("hkev_created_at")
                    .unwrap_or_else(|_| Utc::now()),
            })
        })
        .collect();

    let axis_rows: Vec<RoomCleanRow> = rows
        .iter()
        .map(|row| RoomCleanRow {
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            room_clean: row.try_get::<bool, _>("room_clean").unwrap_or(true),
            latest_event_today: row
                .try_get::<Option<String>, _>("hkev_status")
                .ok()
                .flatten(),
        })
        .collect();

    // Which site's iHOTEL to ask. `All` resolves to HF Hotel exactly as
    // `AppState::write_pool` does, so the legacy read and the PG read can never
    // describe two different properties.
    let branch_id = match query.branch.unwrap_or_default() {
        Branch::Hfville => "hfville",
        Branch::Hfhotel | Branch::All => "hfhotel",
    };
    let outcome = match readers.as_deref() {
        Some(readers) => readers.read(branch_id).await,
        // No Extension layered (direct-call tests, or a router built without
        // it): identical to an unreachable legacy — serve the canonical mirror
        // and say so.
        None => RoomFlagsOutcome::Unavailable,
    };

    let (legacy_status_stale, legacy_clean, housekeeping) =
        build_housekeeping_axes(&axis_rows, &outcome, branch_id);

    Ok(Json(CleaningProgressResponse {
        success: true,
        data,
        legacy_status_stale,
        legacy_clean,
        housekeeping,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::legacy_room_status::LegacyRoomFlags;

    // ---------------------------------------------------------------
    // Housekeeping axis (wave-5 IF-1) — pure, no DB, no legacy server
    // ---------------------------------------------------------------

    fn pg_room(room_no: &str, room_clean: bool, latest: Option<&str>) -> RoomCleanRow {
        RoomCleanRow {
            room_no: room_no.to_string(),
            room_clean,
            latest_event_today: latest.map(str::to_string),
        }
    }

    /// iHOTEL's answer for this surface. `occupied` is left UNKNOWN on
    /// purpose: reception's board must behave identically whether or not the
    /// widened read has an occupancy opinion.
    fn ihotel(pairs: &[(&str, bool)]) -> RoomFlagsOutcome {
        RoomFlagsOutcome::Available(
            pairs
                .iter()
                .map(|(no, clean)| {
                    (
                        (*no).to_string(),
                        LegacyRoomFlags {
                            is_clean: Some(*clean),
                            occupied: None,
                        },
                    )
                })
                .collect(),
        )
    }

    /// A room whose `Room_Clean` was junk but whose `Room_Use` read fine must
    /// be treated here exactly as a room iHOTEL never mentioned: the reception
    /// board keeps its canonical cleanliness and reports no legacy opinion.
    /// Pins that the widened read cannot leak occupancy into this surface.
    #[test]
    fn an_occupancy_only_answer_is_no_opinion_for_reception() {
        let rooms = vec![pg_room("104", true, None)];
        let outcome = RoomFlagsOutcome::Available(
            [(
                "104".to_string(),
                LegacyRoomFlags {
                    is_clean: None,
                    occupied: Some(true),
                },
            )]
            .into_iter()
            .collect(),
        );
        let (stale, legacy_clean, housekeeping) =
            build_housekeeping_axes(&rooms, &outcome, "hfhotel");
        assert!(!stale, "iHOTEL answered — nothing is stale");
        assert!(
            legacy_clean.is_empty(),
            "no cleanliness opinion ⇒ nothing on the legacy axis: {legacy_clean:?}"
        );
        assert!(!housekeeping[0].divergent);
        assert_eq!(housekeeping[0].hk_status, "clean", "canonical stands");
    }

    /// The three-way derivation, exactly as the locked design's table.
    #[test]
    fn hk_status_derivation_matches_the_design_table() {
        // dirty + a live `started` ⇒ the maid is in there NOW. This is the
        // case that was impossible to represent before wave-5 and the whole
        // reason the middle column existed but never filled.
        assert_eq!(derive_hk_status(false, Some("started")), HkStatus::Cleaning);
        // dirty, nothing started ⇒ waiting for a maid.
        assert_eq!(derive_hk_status(false, None), HkStatus::Dirty);
        assert_eq!(derive_hk_status(false, Some("done")), HkStatus::Dirty);
        assert_eq!(derive_hk_status(false, Some("dirty")), HkStatus::Dirty);
        // clean ⇒ clean.
        assert_eq!(derive_hk_status(true, None), HkStatus::Clean);
        assert_eq!(derive_hk_status(true, Some("done")), HkStatus::Clean);
    }

    /// The ONE case the two candidate orderings disagree on. Pinned because it
    /// is a deliberate reading of the design table (the `dirty` row carries the
    /// "latest not started" qualifier; the `clean` row carries none), not an
    /// accident of `if` order.
    #[test]
    fn merged_clean_outranks_a_stale_started_event() {
        assert_eq!(
            derive_hk_status(true, Some("started")),
            HkStatus::Clean,
            "merged truth wins over an out-of-band `started` event"
        );
    }

    /// Wire literals are the frontend's `HK_STATUS_LABELS` keys — pinned so a
    /// rename here cannot silently blank three columns on reception's board.
    #[test]
    fn hk_status_wire_literals_are_stable() {
        assert_eq!(HkStatus::Clean.as_str(), "clean");
        assert_eq!(HkStatus::Cleaning.as_str(), "cleaning");
        assert_eq!(HkStatus::Dirty.as_str(), "dirty");
    }

    /// iHOTEL wins in BOTH directions, and the disagreement is REPORTED (the
    /// deliberate difference from the maid surface, which suppresses it).
    #[test]
    fn ihotel_wins_and_the_divergence_is_reported_to_reception() {
        let rooms = vec![
            pg_room("301", true, None),  // canonical says clean…
            pg_room("302", false, None), // canonical says dirty…
        ];
        let (stale, legacy, hk) =
            build_housekeeping_axes(&rooms, &ihotel(&[("301", false), ("302", true)]), "hfhotel");

        assert!(!stale);
        assert_eq!(
            hk[0].hk_status, "dirty",
            "iHOTEL's dirty beats canonical clean"
        );
        assert_eq!(
            hk[1].hk_status, "clean",
            "iHOTEL's clean beats canonical dirty"
        );
        assert!(hk[0].divergent && hk[1].divergent);
        assert_eq!(legacy.len(), 2);
        assert!(!legacy[0].clean && legacy[1].clean);
    }

    /// Agreement is not divergence — the anomaly flag must stay rare enough to
    /// mean something.
    #[test]
    fn agreement_flags_nothing() {
        let rooms = vec![pg_room("301", true, None), pg_room("302", false, None)];
        let (stale, legacy, hk) =
            build_housekeeping_axes(&rooms, &ihotel(&[("301", true), ("302", false)]), "hfhotel");

        assert!(!stale);
        assert!(hk.iter().all(|h| !h.divergent));
        assert_eq!(legacy.len(), 2, "agreement still reports what iHOTEL said");
    }

    /// A room iHOTEL has no usable value for keeps its canonical value
    /// SILENTLY — a mapping gap is not a staleness event, and flagging the
    /// whole board for one unmatched room trains reception to ignore the note.
    #[test]
    fn a_room_ihotel_does_not_know_keeps_canonical_and_is_not_divergent() {
        let rooms = vec![pg_room("301", false, Some("started"))];
        let (stale, legacy, hk) = build_housekeeping_axes(&rooms, &ihotel(&[]), "hfhotel");

        assert!(!stale, "an empty answer is still an answer, not an outage");
        assert!(legacy.is_empty());
        assert_eq!(hk[0].hk_status, "cleaning");
        assert!(!hk[0].divergent);
    }

    /// An unreachable legacy is stale-but-usable: canonical values, no iHOTEL
    /// list, and — critically — NOTHING flagged divergent, because a read that
    /// did not happen cannot prove a disagreement.
    #[test]
    fn unavailable_legacy_is_stale_and_proves_no_divergence() {
        let rooms = vec![pg_room("301", false, None), pg_room("302", true, None)];
        let (stale, legacy, hk) =
            build_housekeeping_axes(&rooms, &RoomFlagsOutcome::Unavailable, "hfhotel");

        assert!(stale);
        assert!(legacy.is_empty());
        assert_eq!(hk[0].hk_status, "dirty");
        assert_eq!(hk[1].hk_status, "clean");
        assert!(hk.iter().all(|h| !h.divergent));
    }

    /// Legacy `varchar` room numbers are space-padded in places; the merge must
    /// match on the trimmed token or every room looks unknown.
    #[test]
    fn room_numbers_are_matched_trimmed() {
        let rooms = vec![pg_room("  301  ", true, None)];
        let (_, legacy, hk) =
            build_housekeeping_axes(&rooms, &ihotel(&[("301", false)]), "hfhotel");
        assert_eq!(legacy.len(), 1, "a padded canonical room_no still matches");
        assert_eq!(hk[0].hk_status, "dirty");
        assert!(hk[0].divergent);
    }

    /// The three new fields are ADDITIVE and ALWAYS serialized, in the agreed
    /// camelCase spelling. A client must be able to branch on the VALUE, so a
    /// rollback cannot paint a permanent stale banner by omitting the key.
    #[test]
    fn the_new_fields_are_always_serialized_in_camel_case() {
        let body = serde_json::to_string(&CleaningProgressResponse {
            success: true,
            data: vec![],
            legacy_status_stale: false,
            legacy_clean: vec![LegacyRoomClean {
                room_no: "301".to_string(),
                clean: true,
            }],
            housekeeping: vec![RoomHousekeeping {
                room_no: "301".to_string(),
                hk_status: "cleaning".to_string(),
                divergent: false,
            }],
        })
        .expect("response serializes");

        assert!(body.contains("\"legacyStatusStale\":false"), "{body}");
        assert!(body.contains("\"legacyClean\""), "{body}");
        assert!(body.contains("\"hkStatus\":\"cleaning\""), "{body}");
        assert!(body.contains("\"divergent\":false"), "{body}");
    }

    #[test]
    fn resolve_by_defaults_blank_and_missing() {
        // Auth off (`actor = None`): body value wins, blank/missing → sentinel.
        assert_eq!(resolve_by(None, None), DEFAULT_BY);
        assert_eq!(resolve_by(None, Some("   ".to_string())), DEFAULT_BY);
        assert_eq!(resolve_by(None, Some("Nok".to_string())), "Nok");
        assert_eq!(resolve_by(None, Some("  Nok  ".to_string())), "Nok");
    }

    #[test]
    fn resolve_by_prefers_authenticated_user() {
        use crate::domain::user::{Role, User};
        use chrono::NaiveDate;
        let u = User {
            user_id: 9,
            username: "housekeeper_a".to_string(),
            password_hash: String::new(),
            role: Role::Housekeeper,
            active: true,
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            last_login_at: None,
            email: None,
        };
        // Authenticated user overrides the body value.
        assert_eq!(
            resolve_by(Some(&u), Some("Nok".to_string())),
            "housekeeper_a"
        );
    }

    /// The clean/dirty body must accept an empty object (`{}`) — the frontend
    /// may omit `by` — so serde `default` keeps the `by` field optional.
    #[test]
    fn action_body_accepts_empty_object() {
        let body: HousekeepingActionBody = serde_json::from_str("{}").unwrap();
        assert!(body.by.is_none());
        let body: HousekeepingActionBody = serde_json::from_str(r#"{"by":"Nok"}"#).unwrap();
        assert_eq!(body.by.as_deref(), Some("Nok"));
    }

    /// Maintenance body round-trips the boolean in both directions.
    #[test]
    fn maintenance_body_parses_bool() {
        let on: MaintenanceBody = serde_json::from_str(r#"{"maintenance":true}"#).unwrap();
        assert!(on.maintenance);
        let off: MaintenanceBody = serde_json::from_str(r#"{"maintenance":false}"#).unwrap();
        assert!(!off.maintenance);
    }

    /// The reception feed must carry the LATEST event per room and must OMIT
    /// rooms with no event today (absent = "no progress reported"). DB-backed;
    /// skips gracefully without a local PG, same `try_pool` convention as
    /// `service::housekeeping::tests`.
    #[tokio::test]
    async fn cleaning_feed_returns_latest_event_per_room_and_omits_quiet_rooms() {
        use sqlx::PgPool;

        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        let Ok(pool) = PgPool::connect(&url).await else {
            eprintln!("skipping cleaning_feed_returns_latest_event_per_room — PG not reachable");
            return;
        };

        for marker in ["ZT-HKF1", "ZT-HKF2"] {
            let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
                .bind(marker)
                .execute(&pool)
                .await;
        }
        // ZT-HKF1 gets two events today; ZT-HKF2 stays quiet.
        let reported: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HKF1', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed reported room");
        let quiet: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HKF2', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed quiet room");

        for status in ["started", "done"] {
            sqlx::query(
                "INSERT INTO ht_hk_cleaning_events \
                     (hkev_room_id, hkev_status, hkev_badge, hkev_name) \
                 VALUES ($1, $2, 'Q1001', 'นก')",
            )
            .bind(reported)
            .bind(status)
            .execute(&pool)
            .await
            .expect("seed cleaning event");
        }

        let state = AppState::new(pool.clone());
        let Json(body) = list_cleaning_progress(
            axum::extract::State(state),
            axum::extract::Query(HousekeepingQuery { branch: None }),
            // No reader layered — the documented fallback: canonical mirror
            // plus the stale flag.
            None,
        )
        .await
        .expect("cleaning feed must answer");

        assert!(body.success);
        assert!(
            body.legacy_status_stale,
            "no iHOTEL reader must degrade to the canonical mirror, flagged stale"
        );
        assert!(
            body.legacy_clean.is_empty(),
            "a stale read has no iHOTEL values"
        );
        // The AXIS covers every active room, including the quiet one that is
        // (correctly) absent from `data`.
        assert!(
            body.housekeeping.iter().any(|h| h.room_no == "ZT-HKF2"),
            "the housekeeping axis must carry rooms with no event today"
        );
        assert!(
            body.housekeeping.iter().all(|h| !h.divergent),
            "a stale read can prove no divergence, so nothing may be flagged"
        );
        let row = body
            .data
            .iter()
            .find(|r| r.room_id == reported)
            .expect("the room with events today must appear");
        assert_eq!(row.status, "done", "the LATEST event wins");
        assert_eq!(row.badge, "Q1001");
        assert_eq!(row.name.as_deref(), Some("นก"));
        assert_eq!(row.room_no, "ZT-HKF1");
        assert!(
            !body.data.iter().any(|r| r.room_id == quiet),
            "a room with no event today must be ABSENT, not present with a null status"
        );

        for id in [reported, quiet] {
            let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
                .bind(id)
                .execute(&pool)
                .await;
        }
    }
}
