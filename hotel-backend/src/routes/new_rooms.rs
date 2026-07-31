//! New Room API routes for HotelNew database
//!
//! - GET /api/new/rooms - List rooms from HT_Rooms_New
//! - GET /api/new/rooms/:id - Get single room
//! - POST /api/new/rooms - Create room
//! - PUT /api/new/rooms/:id - Update room
//!
//! Per `docs/architecture.md` §1, §6 (Phase 1b) the SQL has moved to
//! `repository::room`. This file owns request validation, response shaping,
//! and translates between the repository's row shape and the wire DTOs.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Row, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::outbox::intent::{UpdateRoomPayload, WritebackIntent};
use crate::outbox::{generate_idempotency_key, OutboxRepository};
use crate::repository::room::{RoomRow, RoomWrite};
use crate::service::ids::{aggregate_uuid, AggregateKind};

/// Room from HT_Rooms_New table
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoom {
    pub id: i32,
    pub room_no: String,
    pub room_type_id: Option<i32>,
    pub room_type_name: Option<String>,
    pub floor: Option<i32>,
    pub status: String,
    pub is_clean: bool,
    pub is_maintenance: bool,
    pub price_weekday: Option<f64>,
    pub price_weekend: Option<f64>,
    pub price_special: Option<f64>,
    pub notes: Option<String>,
    /// Receptionist-arranged board position (iHOTEL `HT_Rooms.Room_X`/`Room_y`,
    /// synced into `ht_rooms_new.room_x`/`room_y`). Serialized as `roomX`/`roomY`
    /// for the v2 spatial room grid (ADR 0003 U1). `None`/0 = unplaced.
    pub room_x: Option<i32>,
    pub room_y: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewRoom {
    fn from_row(row: RoomRow) -> Self {
        Self {
            id: row.room_id,
            room_no: row.room_no,
            room_type_id: row.room_type_id,
            room_type_name: row.type_name,
            floor: row.room_floor,
            status: row
                .room_status
                .unwrap_or_else(|| "available".to_string()),
            is_clean: row.room_clean.unwrap_or(true),
            is_maintenance: row.room_maintenance.unwrap_or(false),
            price_weekday: row.room_price_weekday,
            price_weekend: row.room_price_weekend,
            price_special: row.room_price_special,
            notes: row.room_notes,
            room_x: row.room_x,
            room_y: row.room_y,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Query parameters for rooms list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomsQuery {
    pub status: Option<String>,
    pub room_type_id: Option<i32>,
    pub floor: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    /// Branch selector: 'hfhotel' | 'hfville' | 'all' (HotelNew only contains hfhotel data)
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

impl NewRoomsQuery {
    /// Requested page, clamped to the first page.
    ///
    /// Both list paths MUST use this. `?page=0` used to mean `OFFSET -50` (a
    /// hard PG error) on the SQL path while the derived path silently served
    /// page 1 — and the response echoed the raw `"page": 0` alongside page-1
    /// data either way. `list_rooms` reports THIS value.
    pub fn clamped_page(&self) -> i32 {
        self.page.max(1)
    }

    /// Requested page size, clamped to non-negative. A negative `?limit=` was
    /// `LIMIT -20` (a PG error) on the SQL path and an empty page on the
    /// derived path; both now return an empty page. `0` is left as `0` on
    /// purpose — `LIMIT 0` and `take(0)` already agree.
    pub fn clamped_limit(&self) -> i32 {
        self.limit.max(0)
    }

    /// Row offset for the requested page, computed in `i64` so a large
    /// `page`x`limit` saturates instead of overflowing the `i32` multiply.
    pub fn clamped_offset(&self) -> i64 {
        (self.clamped_page() as i64 - 1).saturating_mul(self.clamped_limit() as i64)
    }
}

/// Response for rooms list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomsResponse {
    pub success: bool,
    pub data: Vec<NewRoom>,
    pub pagination: Pagination,
}

/// Response for single room
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomResponse {
    pub success: bool,
    pub room: NewRoom,
}

/// Request body for creating/updating room
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRoomRequest {
    pub room_no: String,
    pub room_type_id: Option<i32>,
    pub floor: Option<i32>,
    pub status: Option<String>,
    pub is_clean: Option<bool>,
    pub is_maintenance: Option<bool>,
    pub price_weekday: Option<f64>,
    pub price_weekend: Option<f64>,
    pub price_special: Option<f64>,
    pub notes: Option<String>,
}

/// Response for create/update operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// Flags scan feeding [`live_room_flags`].
///
/// Deliberately has NO `room_active` predicate: it must cover EVERY room
/// `repository::room`'s list queries can return, and neither `ROOM_SELECT_SQL`
/// nor `non_status_conditions` filters on `room_active`. It used to end
/// `WHERE r.room_active`, so an inactive room (or one with a NULL flag — the
/// column is `BOOLEAN DEFAULT true` with no NOT NULL) was listed but absent
/// from the map, defaulted to `(false, false, false)` and derived to
/// `available` — harmless while the derived status was display-only, but a
/// real mislabel now that `?status=available` filters on it.
///
/// The symmetric alternative — adding `COALESCE(r.room_active, true)` to the
/// LIST queries (the `hk.rs`/`inventory.rs` idiom) — was rejected: that would
/// DROP rooms from the unfiltered path, which three live callers consume
/// (`app/page.tsx`, `app/housekeeping/page.tsx`). Widening the flags scan
/// cannot change the returned row set, only make a row's flags truthful.
const LIVE_ROOM_FLAGS_SQL: &str = "SELECT r.room_id, \
            EXISTS(SELECT 1 FROM ht_checkins c \
                    WHERE c.cin_status='active' AND c.cin_checkout_time IS NULL \
                      AND (c.cin_room_id=r.room_id OR EXISTS( \
                          SELECT 1 FROM ht_checkin_rooms cr \
                           WHERE cr.cr_cin_id=c.cin_id AND cr.cr_room_id=r.room_id))) AS occupied, \
            EXISTS(SELECT 1 FROM ht_booking_rooms br \
                     JOIN ht_bookings b ON b.book_id=br.br_book_id \
                    WHERE br.br_room_id=r.room_id \
                      AND b.book_status IN ('confirmed','pending') \
                      AND b.book_checkin <= CURRENT_DATE AND b.book_checkout > CURRENT_DATE) AS booked, \
            EXISTS(SELECT 1 FROM ht_checkins c \
                    WHERE c.cin_status='active' AND c.cin_checkout_time IS NULL \
                      AND c.cin_expected_checkout = (now() AT TIME ZONE 'Asia/Bangkok')::date \
                      AND EXTRACT(hour FROM now() AT TIME ZONE 'Asia/Bangkok') >= 6 \
                      AND (c.cin_room_id=r.room_id OR EXISTS( \
                          SELECT 1 FROM ht_checkin_rooms cr \
                           WHERE cr.cr_cin_id=c.cin_id AND cr.cr_room_id=r.room_id))) AS checkout_due \
           FROM ht_rooms_new r";

/// Live occupancy/booking/checkout flags per `room_id`, derived from active
/// check-ins + bookings the SAME way the legacy `/api/rooms/board`
/// (`routes/rooms.rs`) does. The stored `ht_rooms_new.room_status` column is NOT
/// kept in sync with check-in/out, so reading it produced statuses that didn't
/// match iHOTEL or the classic grid (2026-06-27 fix). Tuple = (occupied,
/// booked, checkout_due).
async fn live_room_flags(pool: &crate::db::PgPool) -> ApiResult<HashMap<i32, (bool, bool, bool)>> {
    let rows = sqlx::query(LIVE_ROOM_FLAGS_SQL)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to derive live room status: {e}")))?;

    let mut map = HashMap::with_capacity(rows.len());
    for row in &rows {
        let id: i32 = row
            .try_get("room_id")
            .map_err(|e| ApiError::Internal(format!("room_id: {e}")))?;
        map.insert(
            id,
            (
                row.try_get("occupied").unwrap_or(false),
                row.try_get("booked").unwrap_or(false),
                row.try_get("checkout_due").unwrap_or(false),
            ),
        );
    }
    Ok(map)
}

/// Overlay the live derived status onto rows already mapped to [`NewRoom`].
///
/// iHOTEL/classic precedence, load-bearing:
/// `checkout_pending > maintenance > occupied > booked > available`.
/// `is_clean` / `is_maintenance` stay as independent flags.
///
/// Housekeeping cleanliness is deliberately NOT a derived-status tier:
/// iHOTEL doesn't gate check-in on the housekeeping clean flag, so neither do
/// we (`components/v2/RoomActionSheet.tsx`). A vacant room is `available`
/// whether or not a maid is in it; the frontend renders "รอทำความสะอาด" from
/// the separate `is_clean` flag.
fn overlay_live_status(rooms: &mut [NewRoom], live: &HashMap<i32, (bool, bool, bool)>) {
    for nr in rooms.iter_mut() {
        // `from_row` set status from the stored column; capture the
        // maintenance flag it carried so the v2 maintenance toggle (which
        // PATCHes room_status) keeps working alongside the synced
        // `room_maintenance` column (which the classic grid uses).
        let stored_maintenance = nr.status == "maintenance";
        let (occupied, booked, checkout_due) =
            live.get(&nr.id).copied().unwrap_or((false, false, false));
        nr.status = if checkout_due {
            "checkout_pending"
        } else if nr.is_maintenance || stored_maintenance {
            "maintenance"
        } else if occupied {
            "occupied"
        } else if booked {
            "booked"
        } else {
            "available"
        }
        .to_string();
    }
}

/// True when the request selects or orders by `status` and therefore cannot be
/// served by the SQL pagination path.
///
/// The stored `ht_rooms_new.room_status` column is a bypassed denormalization
/// (issue #200): filtering on it selects the WRONG SET of rooms and then
/// [`overlay_live_status`] relabels them with the RIGHT status — so a room
/// stored `occupied` with no active check-in was silently missing from
/// `GET /api/rooms?status=available`, i.e. from the room-change picker
/// (`components/ChangeRoomModal.tsx`). Such requests take the derived path:
/// scan (unpaginated, unfiltered-by-status) → overlay → filter/sort/paginate
/// in memory.
fn needs_derived_status_pass(params: &NewRoomsQuery) -> bool {
    params.status.is_some() || params.sort_by.as_deref() == Some("status")
}

/// Filter / sort / paginate rooms on their already-overlaid DERIVED status.
/// Returns `(page_of_rooms, total_matching)` — the total counts the FILTERED
/// set, not the scanned set, so `Pagination` stays truthful.
fn filter_sort_paginate(mut rooms: Vec<NewRoom>, params: &NewRoomsQuery) -> (Vec<NewRoom>, i32) {
    if let Some(wanted) = params.status.as_deref() {
        let wanted = wanted.trim();
        // Case-insensitive: the derived literals are ASCII lowercase and
        // `PATCH /rooms/:id/status` lowercases its input, so a mixed-case
        // query silently returning an empty picker would be a repeat of the
        // very bug this path fixes.
        rooms.retain(|r| r.status.eq_ignore_ascii_case(wanted));
    }

    if params.sort_by.as_deref() == Some("status") {
        let desc = params
            .sort_order
            .as_ref()
            .map(|s| s.to_lowercase() == "desc")
            .unwrap_or(false);
        // Tie-break on room_no ASC so the page is deterministic (the SQL sort
        // on the stored column had no tie-break at all).
        rooms.sort_by(|a, b| {
            let primary = if desc {
                b.status.cmp(&a.status)
            } else {
                a.status.cmp(&b.status)
            };
            primary.then_with(|| a.room_no.cmp(&b.room_no))
        });
    }

    let total = rooms.len() as i32;
    // Same clamp as the SQL path (`repository::room::list_with_count_sql`) and
    // as the `Pagination` the response reports — see `NewRoomsQuery`.
    let limit = params.clamped_limit();
    let offset = params.clamped_offset() as usize;

    let page_rooms = rooms.into_iter().skip(offset).take(limit as usize).collect();
    (page_rooms, total)
}

/// List one pool's rooms with the LIVE display status overlaid (matching iHOTEL
/// and the classic grid) instead of the stale stored `room_status` column.
///
/// Two paths, deliberately:
/// * no `status` filter/sort → unchanged SQL pagination + overlay (the three
///   unfiltered callers: dashboard, housekeeping, v2 grid);
/// * `status` filter and/or `sortBy=status` → derived path (see
///   [`needs_derived_status_pass`]). `total` is the filtered count in both.
async fn list_rooms_live(
    repo: &dyn crate::repository::room::RoomRepository,
    pool: &crate::db::PgPool,
    params: &NewRoomsQuery,
) -> ApiResult<(Vec<NewRoom>, i32)> {
    if needs_derived_status_pass(params) {
        let rows = repo.list_for_derived_status(pool, params).await?;
        let live = live_room_flags(pool).await?;
        let mut rooms: Vec<NewRoom> = rows.into_iter().map(NewRoom::from_row).collect();
        overlay_live_status(&mut rooms, &live);
        return Ok(filter_sort_paginate(rooms, params));
    }

    let (rows, total) = repo.list_with_count(pool, params).await?;
    let live = live_room_flags(pool).await?;
    let mut rooms: Vec<NewRoom> = rows.into_iter().map(NewRoom::from_row).collect();
    overlay_live_status(&mut rooms, &live);
    Ok((rooms, total))
}

/// GET /api/new/rooms - List rooms from HT_Rooms_New
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(params): Query<NewRoomsQuery>,
) -> ApiResult<Json<NewRoomsResponse>> {
    // Branch-aware: HF Hotel reads new_pool, HF Ville reads ville_pool
    // (hotelville's canonical ht_rooms_new is populated), All unions both —
    // mirroring routes/rooms.rs::list_rooms. Status is derived live per pool.
    //
    // `All` composes per-pool pages: each pool independently returns ITS page
    // `params.page` and ITS own total, concatenated / summed. That shape is
    // unchanged by the derived-status path — the only difference is that each
    // pool's total now counts its FILTERED rooms, so the summed total still
    // equals the number of rooms the union would yield across all pages.
    let repo = state.rooms.as_ref();
    let (rooms, total) = match params.branch.unwrap_or_default() {
        Branch::Hfhotel => list_rooms_live(repo, &state.new_pool, &params).await?,
        Branch::Hfville => list_rooms_live(repo, state.ville_pool()?, &params).await?,
        Branch::All => {
            let (mut r, mut t) = list_rooms_live(repo, &state.new_pool, &params).await?;
            if let Ok(vp) = state.ville_pool() {
                let (vr, vt) = list_rooms_live(repo, vp, &params).await?;
                r.extend(vr);
                t += vt;
            }
            (r, t)
        }
    };

    Ok(Json(NewRoomsResponse {
        success: true,
        data: rooms,
        // Report the CLAMPED page/limit — the raw values would claim
        // `"page": 0` next to page-1 data on the derived path.
        pagination: Pagination::new(params.clamped_page(), params.clamped_limit(), total),
    }))
}

/// Query parameters for single-room fetch (branch selector only).
///
/// `room_id` is a per-database SERIAL, so the id collides across the two
/// logical DBs — selecting the right pool is what makes the right record come
/// back. Mirrors `list_rooms`'s branch handling.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomQuery {
    pub branch: Option<Branch>,
}

/// GET /api/new/rooms/:id - Get single room
pub async fn get_room(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(params): Query<NewRoomQuery>,
) -> ApiResult<Json<NewRoomResponse>> {
    // Per-site pool via the unified write chokepoint. `All` is not meaningful
    // for a single id → it collapses to the HF Hotel pool.
    let pool = state.write_pool(params.branch)?;
    let row = state
        .rooms
        .get(pool, room_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    Ok(Json(NewRoomResponse {
        success: true,
        room: NewRoom::from_row(row),
    }))
}

/// POST /api/new/rooms - Create room
///
/// **Canonical-only — legacy `HT_Rooms` INSERT writeback is a deliberate TODO
/// (Task #51).** Unlike `update_room` (which UPDATEs an existing legacy row by
/// `Room_no` — safe, no id allocation), creating a room would require INSERTing
/// into `HT_Rooms`, whose `id` iHOTEL allocates app-side (MAX+1, race-prone —
/// see CLAUDE.md "Sole standing exception"). A blind INSERT here risks a
/// duplicate-id race with a concurrent iHOTEL save and has no back-population
/// anchor. New rooms are rare; create them in iHOTEL (the 15-minute room mirror
/// + `backfill_rooms` reconcile them inbound), or add the room here and let an
/// operator mirror it. Editing an existing room's price/type/notes DOES write
/// back (`update_room`).
pub async fn create_room(
    State(state): State<AppState>,
    Query(params): Query<NewRoomQuery>,
    Json(body): Json<CreateUpdateRoomRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Per-site pool via the unified write chokepoint (Ship-B). HF Ville
    // mutations stay gated by `ville_write_guard` until HFVILLE_WRITES_ENABLED.
    let pool = state.write_pool(params.branch)?;

    let room_no = body.room_no.trim();
    if room_no.is_empty() {
        return Err(ApiError::BadRequest("Room number is required".to_string()));
    }

    let existing = state.rooms.find_by_room_no(pool, room_no).await?;
    if existing.is_some() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    let mut tx = pool.begin().await?;
    let id = state
        .rooms
        .insert(
            &mut tx,
            RoomWrite {
                room_no,
                room_type_id: body.room_type_id,
                floor: body.floor,
                status,
                is_clean,
                is_maintenance,
                price_weekday: body.price_weekday,
                price_weekend: body.price_weekend,
                price_special: body.price_special,
                notes: body.notes.as_deref(),
            },
        )
        .await?;
    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room created successfully".to_string(),
        id: Some(id),
    }))
}

/// PUT /api/new/rooms/:id - Update room
///
/// Inside one PG transaction:
/// 1. UPDATE `ht_rooms_new` with the request body (existing behavior).
/// 2. Enqueue [`WritebackIntent::UpdateRoom`] so the writeback worker
///    mirrors price / type / notes onto legacy `HT_Rooms`. Closes the
///    admin-edit divergence between canonical PG and the legacy mirror
///    that previously left rows like room A2-1 stuck with
///    `Room_PriceA = 0.00` on MSSQL while PG showed `price_weekday = 890.00`.
///
/// The writeback targets `HT_Rooms.Room_no` of the row BEFORE the rename
/// (captured via [`load_legacy_room_no`]) so a price-and-rename edit
/// still lands the price update. Renaming `Room_no` itself is out of
/// scope for this recipe — see the recipe's module-level doc comment.
///
/// The PG UPDATE and outbox enqueue commit atomically — either both
/// land or neither does, preserving the "writeback queued ⇔ canonical
/// state mutated" invariant from `architecture.md` §3.6c.
pub async fn update_room(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(params): Query<NewRoomQuery>,
    Json(body): Json<CreateUpdateRoomRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Per-site pool via the unified write chokepoint (Ship-B). HF Ville
    // mutations stay gated by `ville_write_guard` until HFVILLE_WRITES_ENABLED.
    let pool = state.write_pool(params.branch)?;

    let room_no = body.room_no.trim();
    if room_no.is_empty() {
        return Err(ApiError::BadRequest("Room number is required".to_string()));
    }

    let existing = state
        .rooms
        .find_by_room_no_excluding(pool, room_no, room_id)
        .await?;
    if existing.is_some() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    // Resolve the legacy `room_type_name` (Thai literal) BEFORE opening
    // the TX — the lookup is read-only and a missing FK should surface
    // as a 400 immediately rather than rollback a half-applied edit.
    // `None` is propagated as `None` so the recipe skips the
    // `Room_Type=` SET fragment (preserves the iHOTEL-side value).
    let resolved_type_name =
        load_room_type_name(pool, body.room_type_id).await?;

    let mut tx = pool.begin().await?;

    // Capture the legacy `Room_no` BEFORE the UPDATE so the writeback
    // recipe targets the existing MSSQL row even when the operator is
    // renaming the room in the same edit (the new room_no won't exist
    // on the legacy side until a separate `Room_no` mirror lands —
    // tracked separately). Also capture the prior maintenance flag so
    // the `SetRoomMaintenance` mirror below only fires on an actual
    // state CHANGE (audit 2026-06-11 P2).
    let (legacy_room_no, was_maintenance) =
        load_room_writeback_context(&mut tx, room_id).await?;

    let rows_affected = state
        .rooms
        .update(
            &mut tx,
            room_id,
            RoomWrite {
                room_no,
                room_type_id: body.room_type_id,
                floor: body.floor,
                status,
                is_clean,
                is_maintenance,
                price_weekday: body.price_weekday,
                price_weekend: body.price_weekend,
                price_special: body.price_special,
                notes: body.notes.as_deref(),
            },
        )
        .await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::NotFound("Room not found".to_string()));
    }

    // Enqueue the writeback intent inside the SAME TX as the PG UPDATE
    // (architecture.md §3.6c — atomic-with-canonical-write invariant).
    // Aggregate UUID is the deterministic v5 derivation from `room_id`
    // (matches the value stamped onto `ht_rooms_new.aggregate_id` by
    // `backfill_rooms`); the writeback resolver no-ops for `UpdateRoom`
    // because the payload carries `room_no` directly.
    let aggregate_id = aggregate_uuid(AggregateKind::Room, room_id);
    let intent = WritebackIntent::UpdateRoom {
        room_id: aggregate_id,
        payload: UpdateRoomPayload {
            room_no: legacy_room_no,
            room_type_name: resolved_type_name,
            price_weekday: body.price_weekday,
            price_weekend: body.price_weekend,
            price_special: body.price_special,
            notes: body.notes.clone(),
        },
    };
    // Per-event discriminator key (2026-06-12 audit follow-up): rooms are
    // edited repeatedly over their lifetime and completed jobs persist as
    // status='done' rows, so the deterministic (intent, aggregate) key
    // would hard-fail the SECOND edit of the same room with a unique
    // violation on `writeback_jobs.idempotency_key`.
    let idempotency_key = generate_idempotency_key(&intent, Uuid::new_v4());
    OutboxRepository::enqueue(&mut tx, &intent, idempotency_key)
        .await
        .map_err(ApiError::from)?;

    // Audit 2026-06-11 P2 — mirror the maintenance flag onto legacy
    // `HT_Rooms.Room_Manternace` (sic) when the edit actually CHANGES it,
    // so a room taken out of service here stops looking rentable in
    // iHOTEL (cheatsheet §3.15/§3.16). Deliberately NOT folded into the
    // `UpdateRoom` payload: that recipe owns master data (price / type /
    // notes) and documents `Room_Manternace` as owned by the maintenance
    // flow — this dedicated intent IS that flow (the shape the
    // `update_room_status` doc comment below anticipated).
    if was_maintenance != is_maintenance {
        let maintenance_intent = WritebackIntent::SetRoomMaintenance {
            room_id: aggregate_id,
            maintenance: is_maintenance,
        };
        // Per-event discriminator key: the same room legitimately goes
        // in and out of maintenance over its lifetime, and
        // `writeback_jobs.idempotency_key` is permanently UNIQUE — a
        // payload-independent `(intent, aggregate)` key would hard-fail
        // the second flip after the first job completed. The enqueue is
        // atomic with the PG UPDATE in this TX (see the discriminator
        // note in `outbox::idempotency`).
        let maintenance_key =
            generate_idempotency_key(&maintenance_intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &maintenance_intent, maintenance_key)
            .await
            .map_err(ApiError::from)?;
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room updated successfully".to_string(),
        id: Some(room_id),
    }))
}

/// One tile move in a `PUT /api/rooms/layout` body (#236 จัดผัง).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomLayoutMove {
    /// Canonical `ht_rooms_new.room_id`.
    pub id: i32,
    /// Legacy free-pixel board coordinates (see `RoomTileMove`). The
    /// frontend derives these NEIGHBOR-style so they round-trip through
    /// `computeSpatialLayout` into the intended cell (decision 2); a swap
    /// carries the two rooms' existing pairs verbatim.
    pub room_x: i32,
    pub room_y: i32,
}

/// Request body for `PUT /api/rooms/layout`: 1 entry = move/place,
/// 2 entries = swap.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomLayoutRequest {
    pub moves: Vec<RoomLayoutMove>,
}

/// Response for `PUT /api/rooms/layout`. `ok` is the locked #236 wire
/// contract; `success`/`message` follow the sibling `MutationResponse`
/// style so generic clients keep working.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomLayoutResponse {
    pub ok: bool,
    pub success: bool,
    pub message: String,
}

/// PUT /api/rooms/layout — layout-edit mode (จัดผัง) drop (#236).
///
/// Writes the SHARED board position: canonical `ht_rooms_new.room_x`/`room_y`
/// UPDATE(s) + ONE `MoveRoomTiles` writeback intent enqueued in the SAME PG
/// transaction (architecture.md §3.6c), mirroring iHOTEL FormRoomMain's
/// `update HT_Rooms set Room_X=<x>,Room_y=<y> where Room_no='<room>'`
/// byte-shape asynchronously via the writeback worker — never inline.
///
/// Shipped DARK behind `LAYOUT_WRITEBACK_ENABLED`: when the flag is off the
/// endpoint 409s with the stable code `LAYOUT_WRITEBACK_DISABLED` and no
/// write happens on EITHER side — a canonical-only rearrange would fork the
/// two boards (governing decision 3).
pub async fn update_room_layout(
    State(state): State<AppState>,
    Query(params): Query<NewRoomQuery>,
    Json(body): Json<RoomLayoutRequest>,
) -> ApiResult<Json<RoomLayoutResponse>> {
    if !crate::config::layout_writeback_enabled() {
        return Err(ApiError::Conflict(
            "LAYOUT_WRITEBACK_DISABLED — จัดผัง layout writes are shipped dark \
             (LAYOUT_WRITEBACK_ENABLED=false); flip the flag only after a \
             reception-coordinated live verification"
                .to_string(),
        ));
    }

    // 1 = move/place, 2 = swap. Anything else is a client bug — the drop
    // gesture can only ever touch one or two tiles.
    if body.moves.is_empty() || body.moves.len() > 2 {
        return Err(ApiError::BadRequest(
            "moves must contain 1 (move/place) or 2 (swap) entries".to_string(),
        ));
    }
    if body.moves.len() == 2 && body.moves[0].id == body.moves[1].id {
        return Err(ApiError::BadRequest(
            "swap entries must reference two different rooms".to_string(),
        ));
    }
    for m in &body.moves {
        // Negative coords are LEGAL: iHOTEL's free-pixel WinForms canvas can
        // persist them, and a verbatim swap with such a tile must round-trip
        // the exact values (review finding 2026-07-10). Only the (0,0)
        // unplaced sentinel is forbidden — a drop must never write it, or
        // the tile silently falls off both boards.
        if m.room_x == 0 && m.room_y == 0 {
            return Err(ApiError::BadRequest(format!(
                "(0,0) is the unplaced sentinel and cannot be written (room {})",
                m.id
            )));
        }
    }

    // Per-site pool via the unified write chokepoint (Ship-B). HF Ville
    // mutations stay gated by `ville_write_guard` until HFVILLE_WRITES_ENABLED.
    let pool = state.write_pool(params.branch)?;
    let mut tx = pool.begin().await?;

    let mut tile_moves: Vec<crate::outbox::intent::RoomTileMove> =
        Vec::with_capacity(body.moves.len());
    for m in &body.moves {
        // Resolve the legacy `Room_no` business key inside the TX (the
        // canonical `ht_rooms_new.room_no` mirrors `HT_Rooms.Room_no` 1:1).
        // Reuses the `update_room` context loader; the maintenance flag it
        // also returns is irrelevant here.
        let (legacy_room_no, _) = load_room_writeback_context(&mut tx, m.id).await?;

        // Absolute SET — deliberately NOT COALESCE'd: this endpoint owns the
        // coordinate pair. The sync mapper's `room_x = COALESCE($9, room_x)`
        // means the CT echo of our own legacy write converges to a no-op
        // because both sides carry identical bytes (decision 2).
        let rows_affected = sqlx::query(
            "UPDATE ht_rooms_new SET room_x = $1, room_y = $2, updated_at = NOW() \
             WHERE room_id = $3",
        )
        .bind(m.room_x)
        .bind(m.room_y)
        .bind(m.id)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if rows_affected == 0 {
            tx.rollback().await?;
            return Err(ApiError::NotFound(format!("Room {} not found", m.id)));
        }

        tile_moves.push(crate::outbox::intent::RoomTileMove {
            room_no: legacy_room_no,
            room_x: m.room_x,
            room_y: m.room_y,
        });
    }

    // ONE intent carries the whole drop (both halves of a swap) so the
    // legacy board can never persist half-swapped across a worker crash.
    // Aggregate id = first moved room, matching the intent's contract.
    let aggregate_id = aggregate_uuid(AggregateKind::Room, body.moves[0].id);
    let intent = WritebackIntent::MoveRoomTiles {
        room_id: aggregate_id,
        moves: tile_moves,
    };
    // Per-event discriminator key: the same room is moved repeatedly over a
    // จัดผัง session and completed jobs persist as status='done' rows — the
    // deterministic (intent, aggregate) key would unique-violate the second
    // move of the same room (same rationale as `update_room` above).
    let idempotency_key = generate_idempotency_key(&intent, Uuid::new_v4());
    OutboxRepository::enqueue(&mut tx, &intent, idempotency_key)
        .await
        .map_err(ApiError::from)?;

    tx.commit().await?;

    Ok(Json(RoomLayoutResponse {
        ok: true,
        success: true,
        message: "Room layout updated".to_string(),
    }))
}

/// Resolve the pre-UPDATE writeback context for `room_id` from PG:
///
/// * the legacy `HT_Rooms.Room_no` business key — the canonical
///   `ht_rooms_new.room_no` mirrors the legacy column 1:1 (per
///   `bin/backfill_rooms.rs:370`), so the canonical value IS the legacy
///   key;
/// * the prior `room_maintenance` flag, so the caller can detect an
///   actual maintenance-state CHANGE and emit
///   [`WritebackIntent::SetRoomMaintenance`] only then (audit 2026-06-11
///   P2). A `NULL` flag is treated as `false` (matches the read DTO's
///   `unwrap_or(false)`).
///
/// Returns `NotFound` when the row does not exist — surfaces the same
/// failure as the PG UPDATE's `rows_affected==0` branch but earlier
/// (before the recipe goes to allocate the outbox row).
///
/// Uses dynamic `sqlx::query` (not the compile-time macro) to keep the
/// crate buildable without regenerating the `.sqlx/` cache.
async fn load_room_writeback_context(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
) -> ApiResult<(String, bool)> {
    let row = sqlx::query(
        "SELECT room_no, room_maintenance FROM ht_rooms_new WHERE room_id = $1",
    )
    .bind(room_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;
    let room_no: String = row
        .try_get("room_no")
        .map_err(|err| ApiError::Internal(format!("room_no column missing: {err}")))?;
    let was_maintenance: bool = row
        .try_get::<Option<bool>, _>("room_maintenance")
        .map_err(|err| {
            ApiError::Internal(format!("room_maintenance column missing: {err}"))
        })?
        .unwrap_or(false);
    Ok((room_no, was_maintenance))
}

/// Resolve `ht_room_types.type_name` for the given `room_type_id`.
/// Returns `None` when `room_type_id` is `None` (so the writeback recipe
/// skips the `Room_Type=` fragment) and a `BadRequest` when the FK
/// points at a non-existent type (surfaces the FK violation to the
/// operator instead of failing the writeback later).
///
/// Uses dynamic `sqlx::query` (not the compile-time macro) to keep the
/// crate buildable without regenerating the `.sqlx/` cache.
async fn load_room_type_name(
    pool: &sqlx::PgPool,
    room_type_id: Option<i32>,
) -> ApiResult<Option<String>> {
    let Some(type_id) = room_type_id else {
        return Ok(None);
    };
    let row = sqlx::query("SELECT type_name FROM ht_room_types WHERE type_id = $1")
        .bind(type_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            ApiError::BadRequest(format!("Room type {type_id} does not exist"))
        })?;
    let type_name: String = row
        .try_get("type_name")
        .map_err(|err| ApiError::Internal(format!("type_name column missing: {err}")))?;
    Ok(Some(type_name))
}

/// Request body for updating room status only
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoomStatusRequest {
    pub status: String,
}

/// PATCH /api/new/rooms/:id/status - Update room status only
///
/// **Intentionally does NOT emit a `WritebackIntent::UpdateRoom`** —
/// this endpoint mutates canonical `ht_rooms_new.room_status` (one of
/// `available`/`occupied`/`maintenance`/`cleaning`), which has no
/// direct counterpart on legacy `HT_Rooms`. The legacy `Room_Use`
/// column is the *occupancy* flag (`'yes'`/`'no'`) and is owned by
/// the walk-in / check-in-to-booking / cancel / check-out recipes;
/// `Room_Clean` / `Room_Manternace` are owned by `mark_clean` and the
/// maintenance request flow. Mirroring `room_status` here would race
/// those flows and corrupt the legacy occupancy state.
///
/// The dedicated-intent shape this comment anticipated now exists for
/// the maintenance flag: [`WritebackIntent::SetRoomMaintenance`],
/// emitted by `update_room` above when the canonical `room_maintenance`
/// boolean actually changes — the column-ownership split is preserved.
pub async fn update_room_status(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(params): Query<NewRoomQuery>,
    Json(body): Json<UpdateRoomStatusRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Per-site pool via the unified write chokepoint (Ship-B). HF Ville
    // mutations stay gated by `ville_write_guard` until HFVILLE_WRITES_ENABLED.
    let pool = state.write_pool(params.branch)?;

    let status = body.status.trim().to_lowercase();

    let valid_statuses = ["available", "occupied", "maintenance", "cleaning"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        )));
    }

    let mut tx = pool.begin().await?;
    let rows_affected = state
        .rooms
        .update_status(&mut tx, room_id, status.as_str())
        .await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::NotFound("Room not found".to_string()));
    }
    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Room status updated to '{}'", status),
        id: Some(room_id),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A room as the repository would hand it over: `stored_status` is the
    /// (drift-prone) `ht_rooms_new.room_status` column.
    fn room(id: i32, room_no: &str, stored_status: &str) -> NewRoom {
        NewRoom {
            id,
            room_no: room_no.to_string(),
            room_type_id: None,
            room_type_name: None,
            floor: None,
            status: stored_status.to_string(),
            is_clean: true,
            is_maintenance: false,
            price_weekday: None,
            price_weekend: None,
            price_special: None,
            notes: None,
            room_x: None,
            room_y: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn query(status: Option<&str>) -> NewRoomsQuery {
        NewRoomsQuery {
            status: status.map(|s| s.to_string()),
            room_type_id: None,
            floor: None,
            page: default_page(),
            limit: default_limit(),
            sort_by: None,
            sort_order: None,
            branch: None,
        }
    }

    /// Derived-status overlay: the live flags win over the stored column.
    fn overlaid(rooms: &mut [NewRoom], live: &[(i32, (bool, bool, bool))]) {
        let map: HashMap<i32, (bool, bool, bool)> = live.iter().copied().collect();
        overlay_live_status(rooms, &map);
    }

    /// Issue #200, the user-facing half: HF Ville rooms stored `occupied` with
    /// no active check-in were silently missing from the room-change picker
    /// (`/api/rooms?status=available`). They must now come back.
    #[test]
    fn stale_occupied_room_with_no_checkin_is_returned_as_available() {
        let mut rooms = vec![room(1, "109", "occupied"), room(2, "112", "occupied")];
        // No live flags at all → nobody is actually checked in.
        overlaid(&mut rooms, &[]);

        let (page, total) = filter_sort_paginate(rooms, &query(Some("available")));

        assert_eq!(total, 2, "both stale-occupied rooms are really available");
        let nos: Vec<&str> = page.iter().map(|r| r.room_no.as_str()).collect();
        assert_eq!(nos, vec!["109", "112"]);
        assert!(page.iter().all(|r| r.status == "available"));
    }

    /// The other half of the same drift: a room stored `available` that IS
    /// occupied must never be offered as a move-to target.
    #[test]
    fn stale_available_room_with_active_checkin_is_not_returned() {
        let mut rooms = vec![room(3, "201", "available"), room(4, "203", "available")];
        // Room 201 has an active check-in; 203 does not.
        overlaid(&mut rooms, &[(3, (true, false, false))]);

        let (page, total) = filter_sort_paginate(rooms, &query(Some("available")));

        assert_eq!(total, 1);
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].room_no, "203");
    }

    /// Requests with no `status` filter/sort keep the SQL pagination path —
    /// the three unfiltered callers (dashboard, housekeeping, v2 grid) must
    /// not change shape.
    #[test]
    fn unfiltered_requests_do_not_take_the_derived_path() {
        assert!(!needs_derived_status_pass(&query(None)));

        let mut q = query(None);
        q.room_type_id = Some(3);
        q.floor = Some(2);
        q.sort_by = Some("roomNo".to_string());
        assert!(
            !needs_derived_status_pass(&q),
            "non-status filters/sorts stay in SQL"
        );

        assert!(needs_derived_status_pass(&query(Some("available"))));
        let mut sorted = query(None);
        sorted.sort_by = Some("status".to_string());
        assert!(needs_derived_status_pass(&sorted));
    }

    /// The unfiltered overlay itself is unchanged: every scanned room is
    /// returned, relabelled, in the SQL order.
    #[test]
    fn overlay_relabels_without_dropping_rooms() {
        let mut rooms = vec![
            room(1, "101", "available"),
            room(2, "102", "occupied"),
            room(3, "103", "available"),
        ];
        overlaid(
            &mut rooms,
            &[(1, (true, false, false)), (3, (false, true, false))],
        );

        let statuses: Vec<&str> = rooms.iter().map(|r| r.status.as_str()).collect();
        assert_eq!(statuses, vec!["occupied", "available", "booked"]);
    }

    /// `total` counts the FILTERED set, not the scanned set, and not the page.
    #[test]
    fn reported_total_matches_filtered_count_not_page_size() {
        let mut rooms: Vec<NewRoom> = (1..=10)
            .map(|i| room(i, &format!("1{:02}", i), "occupied"))
            .collect();
        // Rooms 1..=4 are genuinely occupied; 5..=10 are free.
        let live: Vec<(i32, (bool, bool, bool))> =
            (1..=4).map(|i| (i, (true, false, false))).collect();
        overlaid(&mut rooms, &live);

        let mut q = query(Some("available"));
        q.limit = 2;
        q.page = 1;
        let (page, total) = filter_sort_paginate(rooms.clone(), &q);
        assert_eq!(total, 6, "6 rooms are derived-available");
        assert_eq!(page.len(), 2, "page is capped by limit");
        assert_eq!(page[0].room_no, "105");

        // Page 3 of 6 rooms at limit 2 → the last two.
        q.page = 3;
        let (page3, total3) = filter_sort_paginate(rooms, &q);
        assert_eq!(total3, 6);
        let nos: Vec<&str> = page3.iter().map(|r| r.room_no.as_str()).collect();
        assert_eq!(nos, vec!["109", "110"]);
    }

    /// `Branch::All` concatenates per-pool pages and SUMS per-pool totals; the
    /// filtered totals must still add up to the union's filtered size.
    #[test]
    fn branch_all_sums_filtered_totals_per_pool() {
        let mut hotel = vec![room(1, "101", "occupied"), room(2, "102", "available")];
        overlaid(&mut hotel, &[(2, (true, false, false))]);
        let mut ville = vec![room(1, "201", "occupied"), room(2, "203", "occupied")];
        overlaid(&mut ville, &[]);

        let q = query(Some("available"));
        let (mut page, mut total) = filter_sort_paginate(hotel, &q);
        let (vpage, vtotal) = filter_sort_paginate(ville, &q);
        page.extend(vpage);
        total += vtotal;

        assert_eq!(total, 3, "1 at HF Hotel + 2 at HF Ville");
        assert_eq!(page.len() as i32, total, "one page holds all of them");
    }

    /// Precedence is pinned: checkout_pending > maintenance > occupied >
    /// booked > available (iHOTEL/classic grid order).
    #[test]
    fn derived_status_precedence_is_unchanged() {
        let mut rooms = vec![
            room(1, "101", "available"),
            room(2, "102", "maintenance"),
            room(3, "103", "available"),
            room(4, "104", "available"),
            room(5, "105", "occupied"),
        ];
        rooms[2].is_maintenance = true;
        overlaid(
            &mut rooms,
            &[
                // checkout_due beats everything, including the stored flag.
                (1, (true, true, true)),
                (2, (true, true, false)),
                (3, (true, true, false)),
                (4, (false, true, false)),
            ],
        );

        let statuses: Vec<&str> = rooms.iter().map(|r| r.status.as_str()).collect();
        assert_eq!(
            statuses,
            vec![
                "checkout_pending",
                "maintenance",
                "maintenance",
                "booked",
                "available"
            ]
        );
    }

    /// `sortBy=status` orders on the DERIVED value (the stored column is not
    /// what the client sees), with room_no as a deterministic tie-break.
    #[test]
    fn status_sort_orders_on_derived_value() {
        let mut rooms = vec![
            room(1, "101", "available"),
            room(2, "102", "occupied"),
            room(3, "103", "available"),
        ];
        // 101 is really occupied, 102 is really free, 103 is booked.
        overlaid(
            &mut rooms,
            &[(1, (true, false, false)), (3, (false, true, false))],
        );

        let mut q = query(None);
        q.sort_by = Some("status".to_string());
        let (asc, total) = filter_sort_paginate(rooms.clone(), &q);
        assert_eq!(total, 3);
        let nos: Vec<&str> = asc.iter().map(|r| r.room_no.as_str()).collect();
        // available(102) < booked(103) < occupied(101)
        assert_eq!(nos, vec!["102", "103", "101"]);

        q.sort_order = Some("desc".to_string());
        let (desc, _) = filter_sort_paginate(rooms, &q);
        let nos: Vec<&str> = desc.iter().map(|r| r.room_no.as_str()).collect();
        assert_eq!(nos, vec!["101", "103", "102"]);
    }

    /// A mixed-case / padded `?status=` must not silently empty the picker.
    #[test]
    fn status_filter_is_case_insensitive_and_trimmed() {
        let mut rooms = vec![room(1, "101", "occupied")];
        overlaid(&mut rooms, &[]);
        let (page, total) = filter_sort_paginate(rooms, &query(Some(" Available ")));
        assert_eq!(total, 1);
        assert_eq!(page.len(), 1);
    }

    /// A vacant room stored `cleaning` is genuinely free and MUST still be
    /// offered as a move-to target: iHOTEL doesn't gate check-in on the
    /// housekeeping clean flag, so neither do we (the principle recorded at
    /// `components/v2/RoomActionSheet.tsx`). Housekeeping progress rides the
    /// separate `is_clean` flag, never the derived status.
    #[test]
    fn room_stored_cleaning_is_still_derived_available() {
        let mut rooms = vec![room(1, "101", "cleaning"), room(2, "102", "available")];
        rooms[0].is_clean = false; // housekeeper is in there, room still dirty
        overlaid(&mut rooms, &[]);

        assert_eq!(rooms[0].status, "available", "stored literal is not a tier");
        assert!(!rooms[0].is_clean, "cleanliness stays an independent flag");

        let (page, total) = filter_sort_paginate(rooms.clone(), &query(Some("available")));
        assert_eq!(total, 2, "both vacant rooms are pickable");
        let nos: Vec<&str> = page.iter().map(|r| r.room_no.as_str()).collect();
        assert_eq!(nos, vec!["101", "102"]);

        // ...and `cleaning` is never emitted, so it matches nothing.
        let (page, total) = filter_sort_paginate(rooms, &query(Some("cleaning")));
        assert_eq!(total, 0);
        assert!(page.is_empty());
    }

    /// Review finding 2: the flags scan must cover EVERY room the list queries
    /// return. It used to end `WHERE r.room_active`, so an inactive room (or one
    /// with a NULL flag) was listed, missed the map, defaulted to no live flags
    /// and derived to `available` — i.e. it showed up in `?status=available`.
    /// Aligning the other way (filtering the LIST) would drop rooms from the
    /// unfiltered path that `app/page.tsx` + `app/housekeeping/page.tsx` consume.
    #[test]
    fn live_flags_scan_does_not_filter_rooms_the_list_keeps() {
        assert!(
            !LIVE_ROOM_FLAGS_SQL.contains("room_active"),
            "flags scan must not narrow the room set: {LIVE_ROOM_FLAGS_SQL}"
        );
        assert!(
            LIVE_ROOM_FLAGS_SQL.trim_end().ends_with("FROM ht_rooms_new r"),
            "no trailing predicate: {LIVE_ROOM_FLAGS_SQL}"
        );
    }

    /// Review finding 3: `?page=0` served page-1 data on the derived path while
    /// the SQL path built `OFFSET -50`. Both clamp now, and the response reports
    /// the clamped value instead of echoing `"page": 0`.
    #[test]
    fn out_of_range_pagination_is_clamped_on_the_derived_path() {
        let mut q = query(Some("available"));
        q.page = 0;
        q.limit = 2;
        assert_eq!(q.clamped_page(), 1);
        assert_eq!(q.clamped_limit(), 2);
        assert_eq!(q.clamped_offset(), 0);

        let mut rooms: Vec<NewRoom> = (1..=4)
            .map(|i| room(i, &format!("10{i}"), "occupied"))
            .collect();
        overlaid(&mut rooms, &[]);
        let (page, total) = filter_sort_paginate(rooms, &q);
        assert_eq!(total, 4);
        let nos: Vec<&str> = page.iter().map(|r| r.room_no.as_str()).collect();
        assert_eq!(nos, vec!["101", "102"], "page 0 serves page 1");

        // A negative limit was `LIMIT -20` (PG error) on the SQL path; both
        // paths now return an empty page with a truthful total.
        let mut q = query(Some("available"));
        q.limit = -20;
        assert_eq!(q.clamped_limit(), 0);
        assert_eq!(q.clamped_offset(), 0);
        let mut rooms = vec![room(1, "101", "occupied")];
        overlaid(&mut rooms, &[]);
        let (page, total) = filter_sort_paginate(rooms, &q);
        assert!(page.is_empty());
        assert_eq!(total, 1);
    }

    /// A huge `?page=` must saturate rather than overflow the `i32` multiply
    /// the SQL path used to do (`(page - 1) * limit`).
    #[test]
    fn absurd_page_saturates_instead_of_overflowing() {
        let mut q = query(None);
        q.page = i32::MAX;
        q.limit = i32::MAX;
        let offset = q.clamped_offset();
        assert!(offset > 0, "no wraparound: {offset}");
        assert_eq!(offset, (i32::MAX as i64 - 1) * i32::MAX as i64);
    }
}
