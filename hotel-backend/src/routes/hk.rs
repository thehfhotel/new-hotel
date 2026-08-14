//! Maid-facing housekeeping surface (`/api/hk/*`) — employee-login plan
//! Phase 4 (HF-erp `docs/employee-login-plan.md`).
//!
//! Serves the mobile `/hk` pages maids open from their LINE Role Menu:
//!
//! - `GET  /api/hk/me`                          — the verified maid identity.
//! - `GET  /api/hk/rooms`                       — room list + today's progress.
//! - `GET  /api/hk/rooms/{id}`                  — one room + today's events.
//! - `POST /api/hk/rooms/{id}/cleaning`         — report progress (`started`/`done`/`dirty`).
//! - `POST /api/hk/rooms/{id}/broken-items`     — RETIRED, answers `410 Gone`.
//! - `GET  /api/hk/broken-items/{id}/photo`     — stream a report's photo.
//!
//! ## `?branch=` is REQUIRED on every room endpoint (wave-4 A)
//!
//! It used to be optional, and `None` fell through to `Branch::default()` =
//! HF Hotel (`routes::mode`). The `/hk` page never sent it, so a HF Ville maid's
//! report was filed against HF Hotel — "pinned by omission", not by design.
//!
//! Now every room endpoint 400s without an explicit, exactly-spelled
//! `hfhotel` / `hfville`. `branch=all` is refused too: `write_pool(Some(All))`
//! returns the PRIMARY pool, so accepting it would re-open the identical bug
//! under a different query string. `GET /api/hk/me` is the ONE exception — it is
//! what tells the client which branches exist, so it must answer before a branch
//! is chosen.
//!
//! Which branches a maid may pick is [`HkPolicy::branches`], from `HK_BRANCHES`
//! (default `hfhotel`); a well-formed branch outside that list is `403`. Server
//! is the enforcement point, the picker is only UX — a stale cached bundle that
//! still omits `?branch=` fails loudly instead of silently mutating HF Hotel.
//!
//! The gate lives in the HANDLERS, i.e. INSIDE `require_hk_access`: an
//! unauthenticated caller gets 401 and can never probe branch configuration.
//!
//! ## Identity & auth
//!
//! The router is wrapped by [`crate::middleware::hk_access::require_hk_access`]
//! (its own Cloudflare Access application, HF ID silent login, grant key
//! `housekeeping`) which injects the verified [`HkIdentity`] — handlers NEVER
//! trust client-supplied reporter fields; the badge/name stamped into rows
//! come exclusively from the verified assertion. Fail closed: no valid
//! identity ⇒ the middleware already answered 401/403.
//!
//! ## Coexistence stance (ADR 0002 / invariant #6)
//!
//! **Changed 2026-08-11 (housekeeping-ops).** This surface used to be
//! PG-canonical-only with NO legacy coupling at all. It no longer is:
//!
//! - `ht_hk_cleaning_events` / `ht_hk_broken_reports` (migration 077) remain
//!   PG-canonical-only NEW data — no sync mapper, no legacy twin.
//! - **`cleaning` with `done` DOES now reach iHOTEL**: it delegates to
//!   [`crate::service::housekeeping::HousekeepingService::mark_clean_if_dirty`],
//!   which flips `ht_rooms_new.room_clean`, enqueues the proven
//!   `MarkRoomClean` writeback and publishes `RoomMarkedClean` — all in one
//!   transaction (PG first, mirror async; invariant #2). That is the whole
//!   point of the change: reception must see the finished room on iHOTEL's
//!   board without the maid touching the legacy app.
//! - `started` stays PG-only on purpose — iHOTEL's in-progress field
//!   `Room_Clean_Time` feeds its room-power countdown, so mirroring it is
//!   parity risk for no operational gain (housekeeping-ops plan, decision #3).
//!   It publishes [`crate::outbox::event::DomainEvent::RoomCleaningStarted`] so
//!   reception's board sees the room go in-progress live over SSE.
//! - **`cleaning` with `dirty` ships DARK (wave-4 B, invariant #6).** The
//!   `MarkRoomDirty` write SHAPE is already live-verified
//!   (`docs/coexistence/phase3-mark-dirty-runsheet.md`), but the TRIGGER is new
//!   — a maid's phone, unattended, able to flag an occupied room mid-stay. So it
//!   is gated by `HK_MARK_DIRTY_ENABLED` (default OFF ⇒ `403`) and needs
//!   reception-coordinated live verification before the flag flips. A flag flip
//!   here is NOT "just config".
//! - The repeat-tap guard lives in the service's conditional UPDATE, so a maid
//!   double-tapping เสร็จแล้ว cannot double-write `HT_Housewife`.
//! - The whole tap (event row + flag flip + writeback enqueue + domain event)
//!   commits in ONE transaction inside
//!   [`crate::service::housekeeping::HousekeepingService::report_cleaning_progress`]
//!   (invariants #1 + #2) — it used to be a bare pool INSERT followed by a
//!   separate service transaction.
//!
//! Branch-aware via `?branch=` resolved through the unified
//! [`AppState::write_pool`] chokepoint (Ship-B gate).
//!
//! ## HF Ville admission
//!
//! `HFVILLE_WRITES_ENABLED=true` since 2026-06-29 (Ville coequal writes are
//! LIVE), so `ville_write_guard` currently admits every branch and this surface
//! behaves identically at both properties.
//!
//! `ville_write_guard` additionally grants ONE narrow exemption —
//! `POST /api/hk/rooms/{id}/cleaning` for `branch=hfville`, regardless of the
//! flag ([`crate::middleware::ville_guard::is_ville_exempt_path`]). That is
//! inert today and matters only if Ville writes are ever turned back off: a
//! maid's cleaning report should not be collateral damage of a front-desk write
//! policy toggle. `broken-items` and every non-hk mutation are NOT exempt.
//! Pinned by `tests/test_hk_ville_guard.rs`.
//!
//! All SQL is RUNTIME `sqlx::query` (no compile-time macro), so this module
//! needs no `.sqlx/` cache regeneration — same policy as
//! `routes::guest_documents` / `routes::new_maintenance`.

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::middleware::hk_access::HkIdentity;
use crate::outbox::event::EventSource;
use crate::service::housekeeping::{
    CleaningProgressStatus, HousekeepingService, ReportCleaningCommand,
};

/// Cleaning-progress statuses a maid can report. `started` =
/// เริ่มทำความสะอาด, `done` = เสร็จแล้ว, `dirty` = ห้องยังไม่สะอาด. Matches the
/// CHECK constraint on `ht_hk_cleaning_events.hkev_status` (migration 077,
/// widened by migration 087) — keep the two in lock-step.
pub const VALID_CLEANING_STATUSES: [&str; 3] = ["started", "done", "dirty"];

/// Env var listing the branches the `/hk` surface may serve. Comma-separated;
/// unset ⇒ [`DEFAULT_HK_BRANCH`].
pub const HK_BRANCHES_ENV: &str = "HK_BRANCHES";

/// Env var gating `status:"dirty"` (invariant #6). Default OFF.
pub const HK_MARK_DIRTY_ENV: &str = "HK_MARK_DIRTY_ENABLED";

/// The one branch served when `HK_BRANCHES` is unset — HF Ville is admitted
/// only after its legacy-key repair is verified (`bin/repair_room_legacy_keys`).
pub const DEFAULT_HK_BRANCH: Branch = Branch::Hfhotel;

/// 400 body for a missing / malformed `?branch=`.
pub const BRANCH_REQUIRED_ERROR: &str = "branch query parameter is required (hfhotel|hfville)";

/// 403 body for a well-formed branch that `HK_BRANCHES` does not list.
pub const BRANCH_NOT_ENABLED_ERROR: &str = "branch not enabled for the housekeeping surface";

/// 403 body when `status:"dirty"` arrives while `HK_MARK_DIRTY_ENABLED` is off.
/// Thai, because a maid reads it (the frontend hides the button, so this is the
/// stale-bundle path).
pub const MARK_DIRTY_DISABLED_ERROR: &str =
    "ยังไม่เปิดใช้งานการแจ้งห้องไม่สะอาด กรุณาแจ้งแผนกต้อนรับ";

/// Runtime configuration of the `/hk` surface, resolved ONCE at startup and
/// carried as a request extension by [`router`].
///
/// It lives here (not on `AppState`) deliberately: this is surface-local policy
/// with no bearing on any other route, and keeping it out of `AppState` means
/// the integration tests can mount the same handlers under a DIFFERENT policy
/// without a process-global that two tests would fight over.
#[derive(Debug, Clone)]
pub struct HkPolicy {
    /// Branches a maid may select, in `HK_BRANCHES` order. Never empty.
    pub branches: Vec<Branch>,
    /// `HK_MARK_DIRTY_ENABLED` — ships DARK (default `false`).
    pub mark_dirty_enabled: bool,
}

impl HkPolicy {
    /// Read `HK_BRANCHES` + `HK_MARK_DIRTY_ENABLED`. Call ONCE at startup.
    pub fn from_env() -> Self {
        let raw = std::env::var(HK_BRANCHES_ENV).ok();
        let branches = parse_hk_branches(raw.as_deref());
        let mark_dirty_enabled = std::env::var(HK_MARK_DIRTY_ENV)
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        Self {
            branches,
            mark_dirty_enabled,
        }
    }

    /// Stable ids of the configured branches, for logging + `GET /api/hk/me`.
    pub fn branch_ids(&self) -> Vec<&'static str> {
        self.branches.iter().copied().map(branch_id).collect()
    }
}

impl Default for HkPolicy {
    fn default() -> Self {
        Self {
            branches: vec![DEFAULT_HK_BRANCH],
            mark_dirty_enabled: false,
        }
    }
}

/// Stable wire id of a branch (matches the `?branch=` spelling).
fn branch_id(branch: Branch) -> &'static str {
    match branch {
        Branch::Hfhotel => "hfhotel",
        Branch::Hfville => "hfville",
        Branch::All => "all",
    }
}

/// Thai label shown in the picker. FIXED values, matching the estate labels the
/// same maids already see in the Housekeeping ops app
/// (`~/HF/housekeeping/src/shared/labels.ts`) — do not localize them here.
fn branch_label_th(branch: Branch) -> &'static str {
    match branch {
        Branch::Hfhotel => "ฮาร์เบอร์ฟร้อนท์",
        Branch::Hfville => "วิลล์",
        Branch::All => "ทุกสาขา",
    }
}

/// Parse `HK_BRANCHES`. PURE — unit-tested below.
///
/// Comma-separated, case-insensitive, order preserved, duplicates collapsed.
/// An unknown token is dropped with a WARN, never a panic: a typo in an env var
/// must not take the surface down at boot. If nothing usable survives (unset,
/// blank, or every token unknown) the result is the documented default
/// `[hfhotel]` — which can never admit HF Ville, so the fallback is safe.
///
/// `all` is NOT accepted: it maps to the primary pool in `write_pool`, so
/// listing it would silently mean "HF Hotel" while reading as "both".
fn parse_hk_branches(raw: Option<&str>) -> Vec<Branch> {
    let mut out: Vec<Branch> = Vec::new();
    for token in raw.unwrap_or_default().split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        match token.to_ascii_lowercase().as_str() {
            "hfhotel" => {
                if !out.contains(&Branch::Hfhotel) {
                    out.push(Branch::Hfhotel);
                }
            }
            "hfville" => {
                if !out.contains(&Branch::Hfville) {
                    out.push(Branch::Hfville);
                }
            }
            other => tracing::warn!(
                token = other,
                "{HK_BRANCHES_ENV}: unknown branch token dropped (expected hfhotel|hfville)"
            ),
        }
    }
    if out.is_empty() {
        out.push(DEFAULT_HK_BRANCH);
    }
    out
}

/// Parse a `?branch=` value with NO defaulting and NO case-fudging. PURE.
///
/// `None` for absent / empty / unknown / `all` / wrong case — every one of
/// which the caller turns into a 400. Only surrounding whitespace is forgiven.
fn parse_branch_param(raw: Option<&str>) -> Option<Branch> {
    match raw.map(str::trim) {
        Some("hfhotel") => Some(Branch::Hfhotel),
        Some("hfville") => Some(Branch::Hfville),
        _ => None,
    }
}

/// The required-branch gate: 400 when absent/malformed, 403 when well-formed
/// but not enabled by `HK_BRANCHES`. PURE — unit-tested below.
fn require_branch(policy: &HkPolicy, raw: Option<&str>) -> ApiResult<Branch> {
    let branch = parse_branch_param(raw)
        .ok_or_else(|| ApiError::BadRequest(BRANCH_REQUIRED_ERROR.to_string()))?;
    if !policy.branches.contains(&branch) {
        return Err(ApiError::Forbidden(BRANCH_NOT_ENABLED_ERROR.to_string()));
    }
    Ok(branch)
}

/// Branch selector shared by every hk room route. Deliberately a raw `String`,
/// not `Option<Branch>`: serde would 400 with its OWN body shape for an unknown
/// value and would happily accept `all`, so the validation is ours
/// ([`require_branch`]) and every rejection carries the repo's
/// `{success:false,error}` envelope.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HkBranchQuery {
    pub branch: Option<String>,
}

/// Resolve the per-site canonical pool via the unified write chokepoint —
/// same idiom as `routes::new_maintenance::resolve_pool`. Takes a REQUIRED
/// branch: the "no branch ⇒ primary pool" fallthrough is the bug this stream
/// closes, so it is not expressible here.
fn resolve_pool(state: &AppState, branch: Branch) -> ApiResult<&PgPool> {
    state.write_pool(Some(branch))
}

/// Build a [`HousekeepingService`] bound to the branch's pool — identical
/// construction to `routes::housekeeping::service_for`, so the maid surface
/// and the front desk share one code path into the `MarkRoomClean` writeback
/// (and one `Hfville → ville_pool` decision, via `write_pool`).
fn service_for(state: &AppState, branch: Branch) -> ApiResult<HousekeepingService> {
    let pool = state.write_pool(Some(branch))?.clone();
    Ok(HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Max chars kept from a maid's display name for `HT_Housewife.h_name`.
///
/// `h_name` is `varchar(150)` in the legacy schema. Thai is single-byte under
/// the DB's TIS-620 collation but 3 bytes as UTF-8, and we cannot be certain
/// which budget a given driver/collation path spends, so bound BOTH: at most
/// [`MAX_H_NAME_CHARS`] characters AND at most [`MAX_H_NAME_BYTES`] UTF-8
/// bytes. That is comfortably inside 150 under either reading.
const MAX_H_NAME_CHARS: usize = 45;
const MAX_H_NAME_BYTES: usize = 140;

/// The label recorded as the housekeeper in `HT_Housewife.h_name`.
///
/// Prefers the verified HF ID display name so iHOTEL's housekeeping log names
/// the actual maid; falls back to the badge, which is ALWAYS present (the
/// middleware 401s without one). Today the CF Access IdP forwards only
/// `["apps", "badge"]`, so this resolves to the badge in production — adding
/// `name` to the forwarded claims upgrades the audit row with no code change
/// here. Never client-supplied: both fields come from the verified assertion.
///
/// The name is IdP-supplied and therefore unbounded, while `h_name` is
/// `varchar(150)`. An over-long name would make the legacy INSERT fail with
/// MSSQL 8152 (string truncation) and the writeback job would retry to
/// `exhausted` — a stuck queue caused purely by someone's long display name.
/// Truncation on a CHAR boundary avoids that and never splits a UTF-8
/// sequence (a byte-sliced Thai name would corrupt the literal).
fn maid_label(identity: &HkIdentity) -> String {
    let raw = identity
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or(&identity.badge);
    truncate_h_name(raw)
}

/// Clamp to [`MAX_H_NAME_CHARS`] chars and [`MAX_H_NAME_BYTES`] UTF-8 bytes,
/// always on a char boundary. PURE — unit-tested below.
fn truncate_h_name(raw: &str) -> String {
    let mut out = String::new();
    for (index, ch) in raw.chars().enumerate() {
        if index >= MAX_H_NAME_CHARS || out.len() + ch.len_utf8() > MAX_H_NAME_BYTES {
            break;
        }
        out.push(ch);
    }
    out
}

/// `EventSource` for a maid-originated cleaning event. Mirrors
/// `routes::housekeeping::http_source`; a real `user_id` would land here if
/// maids ever became PMS accounts (they are CF Access identities today).
fn hk_source() -> EventSource {
    EventSource::our_app(uuid::Uuid::nil(), uuid::Uuid::new_v4())
}

// ============================================================================
// Router
// ============================================================================

/// Build the fully-layered `/api/hk/*` router.
///
/// Lives here (rather than inline in `main`) so `main.rs` and the integration
/// tests mount the SAME stack — a test that rebuilt the layer order by hand
/// would be testing its own replica, not the shipped wiring. Same idiom as
/// `routes::admin_users::router()`.
///
/// Layer order is load-bearing, outermost last:
/// 1. `ville_write_guard` (OUTERMOST) — a disabled-Ville mutation is refused
///    up front, before any auth work, EXCEPT the exempt cleaning route.
/// 2. `require_hk_access` — Cloudflare Access gate; fails closed (401) when
///    `CF_ACCESS_HK_AUD` is unset, so the surface ships dark.
/// 3. body limit, [`HkPolicy`] extension, then the handlers — which is where
///    the required-`?branch=` gate lives. Putting it here rather than in a
///    layer is deliberate: an unauthenticated caller must not be able to tell
///    400 (malformed branch) from 403 (branch not offered) and enumerate which
///    properties this deployment serves.
///
/// That order is what `tests/test_hk_ville_guard.rs` asserts against: an
/// unauthenticated `branch=hfville` request returns 403 when the guard refuses
/// it and 401 when the guard admits it and auth then refuses — which
/// distinguishes the two layers without needing a valid Access assertion. It
/// also explains why `tests/test_hk_branch_required.rs` mounts
/// [`routes_inside_access`] for the branch gate's own status codes: through
/// this stack every unauthenticated probe is 401, by design.
pub fn router(state: AppState) -> Router {
    router_with_policy(state, HkPolicy::from_env())
}

/// [`router`] with an explicit [`HkPolicy`] — used by `main.rs` so the resolved
/// `HK_BRANCHES` / `HK_MARK_DIRTY_ENABLED` can be logged at startup from the
/// SAME value the router serves (parsing twice would let the log lie).
pub fn router_with_policy(state: AppState, policy: HkPolicy) -> Router {
    let ville_guard = axum::middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::ville_guard::ville_write_guard,
    );
    routes_inside_access(state, policy)
        .layer(axum::middleware::from_fn(
            crate::middleware::hk_access::require_hk_access,
        ))
        .layer(ville_guard)
}

/// The `/hk` route table and everything that runs INSIDE the Cloudflare Access
/// gate — handlers, the 8 MB body limit, and the [`HkPolicy`] extension.
///
/// ⚠️ **UNAUTHENTICATED.** This is NOT a mountable surface: it carries no
/// identity check, and `main.rs` must never mount it. It is `pub` for exactly
/// one reason — `tests/test_hk_branch_required.rs` has to observe the branch
/// gate's own status codes (400/403), and through the full [`router`] every
/// unauthenticated probe is 401 by design (the gate is INSIDE the Access layer
/// precisely so nobody can probe branch config without an identity). The test
/// injects its own `Extension<HkIdentity>` in the Access layer's place, then
/// separately asserts through [`router`] that the real stack still answers 401.
pub fn routes_inside_access(state: AppState, policy: HkPolicy) -> Router {
    Router::new()
        .route("/api/hk/me", get(me))
        .route("/api/hk/rooms", get(list_rooms))
        .route("/api/hk/rooms/{room_id}", get(room_detail))
        .route("/api/hk/rooms/{room_id}/cleaning", post(report_cleaning))
        .route(
            "/api/hk/rooms/{room_id}/broken-items",
            post(report_broken_item),
        )
        .route(
            "/api/hk/broken-items/{report_id}/photo",
            get(broken_item_photo),
        )
        .layer(axum::extract::DefaultBodyLimit::max(8 * 1024 * 1024))
        .layer(Extension(policy))
        .with_state(state)
}

// ============================================================================
// Response types
// ============================================================================

/// One selectable property on the `/hk` branch picker.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HkBranchOption {
    /// `hfhotel` | `hfville` — the exact `?branch=` spelling.
    pub id: String,
    pub label_th: String,
}

/// `GET /api/hk/me` — the verified identity plus the surface's configuration.
///
/// Serving `branches` here (instead of a `NEXT_PUBLIC_*` build-time constant)
/// keeps ONE source of truth: `HK_BRANCHES` changes take effect on a backend
/// restart, with no frontend rebuild and no chance of the two disagreeing about
/// which property a maid may file against.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub success: bool,
    pub badge: String,
    pub display_name: Option<String>,
    /// Branches this maid may pick, in `HK_BRANCHES` order. Never empty; a
    /// single entry means the client auto-selects and renders NO picker.
    pub branches: Vec<HkBranchOption>,
    /// `HK_MARK_DIRTY_ENABLED`. `false` ⇒ the client hides the
    /// "แจ้งห้องไม่สะอาด" button rather than offering a dead tap.
    pub mark_dirty_enabled: bool,
}

/// Today's latest cleaning event for a room (the room's current progress).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningProgress {
    /// `started` | `done` | `dirty`.
    pub status: String,
    pub badge: String,
    pub name: Option<String>,
    pub at: DateTime<Utc>,
}

/// One room on the maid's list.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HkRoom {
    pub room_id: i32,
    pub room_no: String,
    pub floor: Option<i32>,
    pub building: Option<String>,
    /// Front-desk cleanliness flag (informational — which rooms need work).
    pub room_clean: bool,
    /// Today's latest maid-reported progress; `None` = nothing reported yet.
    pub cleaning: Option<CleaningProgress>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomsResponse {
    pub success: bool,
    pub data: Vec<HkRoom>,
}

/// One cleaning event in the room-detail log.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningEvent {
    pub event_id: i64,
    pub status: String,
    pub badge: String,
    pub name: Option<String>,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailResponse {
    pub success: bool,
    pub room: HkRoom,
    /// Today's cleaning events, recent first.
    pub events: Vec<CleaningEvent>,
}

/// Body for `POST /api/hk/rooms/{id}/cleaning`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCleaningBody {
    /// `started` | `done` | `dirty`. `dirty` additionally requires
    /// `HK_MARK_DIRTY_ENABLED` (else 403).
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCleaningResponse {
    pub success: bool,
    pub room_id: i32,
    pub status: String,
    /// `true` when this call performed the cleanliness transition and so
    /// enqueued the matching writeback (`MarkRoomClean` for `done`,
    /// `MarkRoomDirty` for `dirty`). `false` for `started` — which is
    /// legacy-inert by design — and for a repeat that changed nothing
    /// (idempotent no-op).
    ///
    /// ENQUEUED, NOT DELIVERED. This says a `writeback_jobs` row was committed
    /// alongside the canonical flip — nothing about iHOTEL having been written.
    /// The worker drains asynchronously and may retry, park (`'skipped'`, if
    /// the HF Ville allowlist excludes the intent) or exhaust. Callers must not
    /// render this as "the front desk can see it now".
    pub writeback_enqueued: bool,
}

/// Body of a `410 Gone` answer from a retired endpoint. Same
/// `{success:false, error}` shape the frontend's `hkFetch` already expects.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoneResponse {
    pub success: bool,
    pub error: String,
}

// ============================================================================
// Pure validation helpers (unit-tested below)
// ============================================================================

/// Normalize + validate a reported cleaning status.
fn parse_cleaning_status(raw: &str) -> Result<CleaningProgressStatus, ApiError> {
    let wanted = raw.trim().to_lowercase();
    VALID_CLEANING_STATUSES
        .iter()
        .find(|s| **s == wanted)
        .and_then(|s| CleaningProgressStatus::from_literal(s))
        .ok_or_else(|| {
            ApiError::BadRequest(format!(
                "invalid status '{raw}' (expected one of {VALID_CLEANING_STATUSES:?})"
            ))
        })
}

// ============================================================================
// SQL helpers (pool-parameterized so the DB-backed tests drive them directly)
// ============================================================================

/// The Thai-day predicate: an event "counts today" when its timestamp falls on
/// today's date in Asia/Bangkok. Both hotels run on Thai wall-clock days.
///
/// Shared verbatim with `routes::housekeeping`'s reception feed
/// (`GET /api/housekeeping/cleaning`) — ONE definition, so the maid's view of
/// "today" and reception's can never drift apart at a day boundary.
pub(crate) const TODAY_BKK: &str =
    "(hkev_created_at AT TIME ZONE 'Asia/Bangkok')::date = (NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// Fetch the active-room list with today's latest cleaning progress.
async fn fetch_rooms(pool: &PgPool) -> Result<Vec<HkRoom>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        WHERE COALESCE(r.room_active, true) = true
        ORDER BY r.room_no
        "#
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(room_from_row).collect())
}

/// Fetch one active room (today's progress + open-report count included).
async fn fetch_room(pool: &PgPool, room_id: i32) -> Result<Option<HkRoom>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        WHERE r.room_id = $1 AND COALESCE(r.room_active, true) = true
        "#
    );
    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(room_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.as_ref().map(room_from_row))
}

fn room_from_row(row: &sqlx::postgres::PgRow) -> HkRoom {
    let cleaning = row
        .try_get::<String, _>("hkev_status")
        .ok()
        .map(|status| CleaningProgress {
            status,
            badge: row.try_get::<String, _>("hkev_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkev_name").ok(),
            at: row
                .try_get::<DateTime<Utc>, _>("hkev_created_at")
                .unwrap_or_else(|_| Utc::now()),
        });
    HkRoom {
        room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
        room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
        floor: row.try_get::<i32, _>("room_floor").ok(),
        building: row.try_get::<String, _>("room_building").ok(),
        room_clean: row.try_get::<bool, _>("room_clean").unwrap_or(true),
        cleaning,
    }
}

/// Today's cleaning events for one room, recent first.
async fn fetch_today_events(
    pool: &PgPool,
    room_id: i32,
) -> Result<Vec<CleaningEvent>, sqlx::Error> {
    let sql = format!(
        "SELECT hkev_id, hkev_status, hkev_badge, hkev_name, hkev_created_at \
           FROM ht_hk_cleaning_events \
          WHERE hkev_room_id = $1 AND {TODAY_BKK} \
          ORDER BY hkev_created_at DESC, hkev_id DESC \
          LIMIT 20"
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(room_id)
        .fetch_all(pool)
        .await?;
    Ok(rows
        .iter()
        .map(|row| CleaningEvent {
            event_id: row.try_get::<i64, _>("hkev_id").unwrap_or(0),
            status: row.try_get::<String, _>("hkev_status").unwrap_or_default(),
            badge: row.try_get::<String, _>("hkev_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkev_name").ok(),
            at: row
                .try_get::<DateTime<Utc>, _>("hkev_created_at")
                .unwrap_or_else(|_| Utc::now()),
        })
        .collect())
}

/// 404-checking room existence probe (active rooms only).
async fn require_room(pool: &PgPool, room_id: i32) -> ApiResult<()> {
    let exists = sqlx::query(
        "SELECT 1 FROM ht_rooms_new WHERE room_id = $1 AND COALESCE(room_active, true) = true",
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound(format!("room {room_id} not found")));
    }
    Ok(())
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/hk/me — the verified identity + surface config for the header bar
/// and the branch picker.
///
/// The ONE hk endpoint that takes NO `?branch=`: it is what tells the client
/// which branches exist, so requiring one would be circular.
pub async fn me(
    Extension(identity): Extension<HkIdentity>,
    Extension(policy): Extension<HkPolicy>,
) -> ApiResult<Json<MeResponse>> {
    Ok(Json(MeResponse {
        success: true,
        badge: identity.badge,
        display_name: identity.display_name,
        branches: policy
            .branches
            .iter()
            .copied()
            .map(|b| HkBranchOption {
                id: branch_id(b).to_string(),
                label_th: branch_label_th(b).to_string(),
            })
            .collect(),
        mark_dirty_enabled: policy.mark_dirty_enabled,
    }))
}

/// GET /api/hk/rooms — the maid's room list with today's progress.
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
) -> ApiResult<Json<RoomsResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    let pool = resolve_pool(&state, branch)?;
    let data = fetch_rooms(pool).await?;
    Ok(Json(RoomsResponse {
        success: true,
        data,
    }))
}

/// GET /api/hk/rooms/{id} — one room with today's cleaning events.
pub async fn room_detail(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    let pool = resolve_pool(&state, branch)?;
    let room = fetch_room(pool, room_id)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("room {room_id} not found")))?;
    let events = fetch_today_events(pool, room_id).await?;
    Ok(Json(RoomDetailResponse {
        success: true,
        room,
        events,
    }))
}

/// POST /api/hk/rooms/{id}/cleaning — report cleaning progress.
///
/// A thin gate over
/// [`HousekeepingService::report_cleaning_progress`](crate::service::housekeeping::HousekeepingService::report_cleaning_progress),
/// which does the whole tap — append-only event row, conditional cleanliness
/// flip, writeback enqueue, domain event — in ONE transaction (invariants #1,
/// #2, #4). This handler owns only what the service must not: the required
/// `?branch=` gate, status validation, the `HK_MARK_DIRTY_ENABLED` dark-ship
/// gate, and the 404 for an unknown/inactive room.
///
/// Rejection ORDER is deliberate and pinned by `tests/test_hk_branch_required`:
/// branch (400/403) BEFORE status (400) BEFORE the mark-dirty flag (403). A
/// request with no branch must never be answered on the strength of its body.
///
/// `writeback_enqueued` means ENQUEUED, NOT DELIVERED — see
/// [`ReportCleaningResponse::writeback_enqueued`].
pub async fn report_cleaning(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<ReportCleaningBody>,
) -> ApiResult<Json<ReportCleaningResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    let status = parse_cleaning_status(&body.status)?;

    // Invariant #6: the mark-dirty TRIGGER is new even though the legacy write
    // shape is proven. Off ⇒ 403, in Thai, with the maid's own envelope.
    if status == CleaningProgressStatus::Dirty && !policy.mark_dirty_enabled {
        return Err(ApiError::Forbidden(MARK_DIRTY_DISABLED_ERROR.to_string()));
    }

    let pool = resolve_pool(&state, branch)?;
    require_room(pool, room_id).await?;

    let svc = service_for(&state, branch)?;
    let report = svc
        .report_cleaning_progress(ReportCleaningCommand {
            room_id,
            status,
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
            by: maid_label(&identity),
            source: hk_source(),
        })
        .await?;

    Ok(Json(ReportCleaningResponse {
        success: true,
        room_id,
        status: status.as_str().to_string(),
        writeback_enqueued: report.writeback_enqueued,
    }))
}

/// POST /api/hk/rooms/{id}/broken-items — **RETIRED (410 Gone)**.
///
/// Breakage intake moved to the Housekeeping ops app
/// (`housekeeping.thehfhotel.org/staff/report`), which opens a real Work Order
/// with photos, status lifecycle and reception ownership — see the
/// housekeeping-ops plan, Module 2. `410` (not `404`) is deliberate: the
/// resource existed and is permanently gone, which tells a stale cached
/// client to stop retrying rather than treat it as a transient routing error.
///
/// `ht_hk_broken_reports` and its rows are KEPT as history, and
/// [`broken_item_photo`] still serves their photos.
pub async fn report_broken_item() -> (StatusCode, Json<GoneResponse>) {
    (
        StatusCode::GONE,
        Json(GoneResponse {
            success: false,
            error: "endpoint ย้ายไปที่แอปแม่บ้านแล้ว: แจ้งซ่อมที่ housekeeping.thehfhotel.org/staff/report"
                .to_string(),
        }),
    )
}

/// GET /api/hk/broken-items/{id}/photo — stream a report's photo bytes.
pub async fn broken_item_photo(
    State(state): State<AppState>,
    Path(report_id): Path<i64>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
) -> ApiResult<Response> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    let pool = resolve_pool(&state, branch)?;
    let row = sqlx::query(
        "SELECT hkbr_photo, hkbr_photo_mime FROM ht_hk_broken_reports WHERE hkbr_id = $1",
    )
    .bind(report_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("report {report_id} not found")))?;

    let photo: Option<Vec<u8>> = row.try_get("hkbr_photo").ok();
    let Some(bytes) = photo else {
        return Err(ApiError::NotFound(format!(
            "report {report_id} has no photo"
        )));
    };
    let mime: String = row
        .try_get::<String, _>("hkbr_photo_mime")
        .unwrap_or_else(|_| "image/jpeg".to_string());

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "private, max-age=3600")
        .body(Body::from(bytes))
        .map_err(|e| ApiError::Internal(format!("failed to build photo response: {e}")))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- pure validation ----------------------------------------------

    #[test]
    fn cleaning_status_accepts_started_done_and_dirty() {
        assert_eq!(
            parse_cleaning_status("started").unwrap(),
            CleaningProgressStatus::Started
        );
        assert_eq!(
            parse_cleaning_status(" DONE ").unwrap(),
            CleaningProgressStatus::Done
        );
        assert_eq!(
            parse_cleaning_status("dirty").unwrap(),
            CleaningProgressStatus::Dirty
        );
        // `clean` is the near-miss that matters: it is the word an author
        // reaches for instead of `done`, and the CHECK constraint would reject
        // it at the DB layer as a 500 rather than a 400.
        for bad in ["", "cleaning", "clean", "finished", "เสร็จแล้ว"] {
            assert!(
                parse_cleaning_status(bad).is_err(),
                "'{bad}' must be rejected"
            );
        }
    }

    /// The route's accepted set MUST equal the service's, or a status the
    /// route admits lands on a CHECK-constraint violation (500) instead of a
    /// 400. Migration 087 widened the DB CHECK to the same three.
    #[test]
    fn valid_statuses_match_the_service_enum() {
        for literal in VALID_CLEANING_STATUSES {
            let parsed = CleaningProgressStatus::from_literal(literal)
                .unwrap_or_else(|| panic!("service must accept '{literal}'"));
            assert_eq!(parsed.as_str(), literal, "round-trip must be stable");
        }
    }

    // ---- HK_BRANCHES parsing -------------------------------------------

    #[test]
    fn hk_branches_defaults_to_hfhotel() {
        assert_eq!(parse_hk_branches(None), vec![Branch::Hfhotel]);
        assert_eq!(parse_hk_branches(Some("")), vec![Branch::Hfhotel]);
        assert_eq!(parse_hk_branches(Some("  , ,")), vec![Branch::Hfhotel]);
    }

    #[test]
    fn hk_branches_parses_order_case_and_duplicates() {
        assert_eq!(
            parse_hk_branches(Some("hfhotel,hfville")),
            vec![Branch::Hfhotel, Branch::Hfville]
        );
        // Order is the picker's order — `HK_BRANCHES` decides it.
        assert_eq!(
            parse_hk_branches(Some("hfville, hfhotel")),
            vec![Branch::Hfville, Branch::Hfhotel]
        );
        // Env config is forgiving about case (the QUERY PARAM is not).
        assert_eq!(
            parse_hk_branches(Some("HFVILLE")),
            vec![Branch::Hfville]
        );
        assert_eq!(
            parse_hk_branches(Some("hfhotel,hfhotel")),
            vec![Branch::Hfhotel]
        );
    }

    /// A typo must not brick the surface at boot, and must never widen it:
    /// unknown tokens (including `all`) are dropped, and an all-unknown value
    /// falls back to the documented default — which cannot admit HF Ville.
    #[test]
    fn hk_branches_drops_unknown_tokens_and_never_widens() {
        assert_eq!(parse_hk_branches(Some("all")), vec![Branch::Hfhotel]);
        assert_eq!(parse_hk_branches(Some("nonsense")), vec![Branch::Hfhotel]);
        assert_eq!(
            parse_hk_branches(Some("hfville,nonsense")),
            vec![Branch::Hfville]
        );
    }

    // ---- the required-branch gate --------------------------------------

    fn policy(branches: Vec<Branch>) -> HkPolicy {
        HkPolicy {
            branches,
            mark_dirty_enabled: false,
        }
    }

    /// Absent / empty / unparseable / wrong-case / `all` ⇒ 400, all with the
    /// same stable message. `all` is the load-bearing one: `write_pool` maps it
    /// to the PRIMARY pool, so accepting it would re-open the wrong-hotel bug
    /// under a different query string.
    #[test]
    fn branch_param_is_required_and_never_defaults() {
        let p = policy(vec![Branch::Hfhotel, Branch::Hfville]);
        for raw in [None, Some(""), Some("   "), Some("all"), Some("HFHOTEL"), Some("hf-hotel"), Some("hfhotel2")] {
            let err = require_branch(&p, raw).expect_err("{raw:?} must be refused");
            match err {
                ApiError::BadRequest(msg) => assert_eq!(msg, BRANCH_REQUIRED_ERROR),
                other => panic!("{raw:?} must be a 400, got {other:?}"),
            }
        }
        assert_eq!(require_branch(&p, Some("hfhotel")).unwrap(), Branch::Hfhotel);
        assert_eq!(require_branch(&p, Some("hfville")).unwrap(), Branch::Hfville);
        // Surrounding whitespace is forgiven; case is not.
        assert_eq!(
            require_branch(&p, Some(" hfville ")).unwrap(),
            Branch::Hfville
        );
    }

    /// A well-formed branch outside `HK_BRANCHES` is 403 (not 400): the request
    /// is understood, the property is simply not offered. This is what keeps HF
    /// Ville off the picker until `repair_room_legacy_keys --apply` is verified.
    #[test]
    fn branch_outside_hk_branches_is_forbidden() {
        let p = policy(vec![Branch::Hfhotel]);
        let err = require_branch(&p, Some("hfville")).expect_err("hfville must be refused");
        match err {
            ApiError::Forbidden(msg) => assert_eq!(msg, BRANCH_NOT_ENABLED_ERROR),
            other => panic!("expected 403, got {other:?}"),
        }
        // …and admitted once configured.
        let widened = policy(vec![Branch::Hfhotel, Branch::Hfville]);
        assert_eq!(
            require_branch(&widened, Some("hfville")).unwrap(),
            Branch::Hfville
        );
    }

    /// The dark-ship default: no env ⇒ HF Hotel only, mark-dirty off.
    #[test]
    fn default_policy_ships_dark() {
        let p = HkPolicy::default();
        assert_eq!(p.branches, vec![Branch::Hfhotel]);
        assert!(!p.mark_dirty_enabled);
        assert_eq!(p.branch_ids(), vec!["hfhotel"]);
    }

    /// The maid label stamped into `HT_Housewife.h_name` prefers the verified
    /// display name and falls back to the badge — never blank, because
    /// `mark_clean_if_dirty` rejects an empty `by`.
    #[test]
    fn maid_label_prefers_display_name_then_badge() {
        let with_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: Some("นก".into()),
            email: None,
        };
        assert_eq!(maid_label(&with_name), "นก");

        // Production reality today: the CF IdP forwards only `apps` + `badge`.
        let no_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: None,
            email: None,
        };
        assert_eq!(maid_label(&no_name), "Q1001");

        // A whitespace-only name must not produce a blank `by`.
        let blank_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: Some("   ".into()),
            email: None,
        };
        assert_eq!(maid_label(&blank_name), "Q1001");
    }

    /// `h_name` is `varchar(150)`. An unbounded IdP display name would make
    /// the legacy INSERT fail with MSSQL 8152 and the writeback job retry to
    /// `exhausted` — a queue stuck on somebody's long name. The clamp must
    /// hold under BOTH byte budgets and never split a UTF-8 sequence.
    #[test]
    fn maid_label_is_clamped_for_ht_housewife_varchar_150() {
        // Thai is the realistic worst case: 3 UTF-8 bytes per char.
        let long_thai = "น".repeat(500);
        let clamped = maid_label(&HkIdentity {
            badge: "Q1001".into(),
            display_name: Some(long_thai),
            email: None,
        });
        assert_eq!(clamped.chars().count(), MAX_H_NAME_CHARS);
        assert!(
            clamped.len() <= MAX_H_NAME_BYTES,
            "{} bytes exceeds the byte budget",
            clamped.len()
        );
        assert!(clamped.len() < 150, "must fit varchar(150) as raw bytes too");
        // Truncation landed on a char boundary — the string is still valid
        // UTF-8 and contains only whole Thai characters.
        assert!(clamped.chars().all(|c| c == 'น'));

        // ASCII hits the char cap before the byte cap.
        let long_ascii = "a".repeat(500);
        let clamped_ascii = maid_label(&HkIdentity {
            badge: "Q1001".into(),
            display_name: Some(long_ascii),
            email: None,
        });
        assert_eq!(clamped_ascii.len(), MAX_H_NAME_CHARS);

        // Normal names are untouched.
        assert_eq!(
            maid_label(&HkIdentity {
                badge: "Q1001".into(),
                display_name: Some("นก สมใจ".into()),
                email: None,
            }),
            "นก สมใจ"
        );

        // 4-byte chars must not overflow the byte budget either.
        let emoji_ish = "𝔘".repeat(100); // 4 bytes each
        let clamped_wide = truncate_h_name(&emoji_ish);
        assert!(clamped_wide.len() <= MAX_H_NAME_BYTES);
        assert!(clamped_wide.chars().count() <= MAX_H_NAME_CHARS);
    }

    /// The retired broken-item intake answers 410 (permanently gone), not 404
    /// (transient/unknown) — so stale cached clients stop retrying.
    #[tokio::test]
    async fn retired_broken_item_endpoint_answers_410() {
        let (status, Json(body)) = report_broken_item().await;
        assert_eq!(status, StatusCode::GONE);
        assert!(!body.success);
        assert!(
            body.error.contains("housekeeping.thehfhotel.org/staff/report"),
            "the 410 must point maids at the new intake; got: {}",
            body.error
        );
    }

    // ---- DB-backed (skip gracefully when no local PG — same `try_pool`
    //      convention as `service::housekeeping::tests`) ------------------

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    /// Append a cleaning event directly, for READ-path fixtures. The write path
    /// itself lives in `service::housekeeping::report_cleaning_progress` (and is
    /// tested there); this raw INSERT exists so these tests can also drive the
    /// `hkev_status` CHECK constraint with values the service cannot express.
    async fn seed_cleaning_event(
        pool: &PgPool,
        room_id: i32,
        status: &str,
        badge: &str,
        name: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            "INSERT INTO ht_hk_cleaning_events (hkev_room_id, hkev_status, hkev_badge, hkev_name) \
             VALUES ($1, $2, $3, $4) RETURNING hkev_id",
        )
        .bind(room_id)
        .bind(status)
        .bind(badge)
        .bind(name)
        .fetch_one(pool)
        .await?;
        row.try_get("hkev_id")
    }

    /// Seed a marker room, exercise the full progress + broken-report flow,
    /// and verify the "today" room list reflects the LATEST event.
    #[tokio::test]
    async fn cleaning_and_broken_report_round_trip() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping cleaning_and_broken_report_round_trip — PG not reachable");
            return;
        };
        // Reset marker rows from prior (possibly aborted) runs. Events and
        // reports cascade with the room delete (ON DELETE CASCADE).
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HK1'")
            .execute(&pool)
            .await;
        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HK1', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = row.try_get("room_id").unwrap();

        // started → done: the list must surface the LATEST event.
        seed_cleaning_event(&pool, room_id, "started", "Q1001", Some("Nok"))
            .await
            .expect("started event must insert");
        seed_cleaning_event(&pool, room_id, "done", "Q1001", Some("Nok"))
            .await
            .expect("done event must insert");

        let rooms = fetch_rooms(&pool).await.expect("room list must fetch");
        let room = rooms
            .iter()
            .find(|r| r.room_id == room_id)
            .expect("seeded room must be listed");
        let progress = room.cleaning.as_ref().expect("today's progress present");
        assert_eq!(progress.status, "done");
        assert_eq!(progress.badge, "Q1001");

        // Historical broken-item rows must stay READABLE by the surviving
        // photo route even though the list read path and its `reports` /
        // `openReports` response fields are gone (intake retired, 410; the
        // Housekeeping ops app owns new reports). Seeded via raw SQL because
        // no intake helper remains.
        let report_row = sqlx::query(
            "INSERT INTO ht_hk_broken_reports \
                 (hkbr_room_id, hkbr_description, hkbr_badge, hkbr_name, hkbr_photo, hkbr_photo_mime) \
             VALUES ($1, 'ก๊อกน้ำรั่ว', 'Q1001', 'Nok', $2, 'image/jpeg') RETURNING hkbr_id",
        )
        .bind(room_id)
        .bind(b"fakejpeg".as_slice())
        .fetch_one(&pool)
        .await
        .expect("historical broken report must insert");
        let report_id: i64 = report_row.try_get("hkbr_id").unwrap();
        assert!(report_id > 0);

        let stored: Vec<u8> =
            sqlx::query_scalar("SELECT hkbr_photo FROM ht_hk_broken_reports WHERE hkbr_id = $1")
                .bind(report_id)
                .fetch_one(&pool)
                .await
                .expect("the photo route's row must still be readable");
        assert_eq!(stored, b"fakejpeg", "history rows survive the retirement");

        let events = fetch_today_events(&pool, room_id).await.unwrap();
        assert_eq!(events.len(), 2, "both events land in today's log");
        assert_eq!(events[0].status, "done", "recent-first ordering");

        // Migration 087 widened the CHECK to accept 'dirty'. If this fails the
        // migration has not been applied to this database — and the maid's
        // mark-dirty tap would 500 instead of writing.
        seed_cleaning_event(&pool, room_id, "dirty", "Q1001", Some("Nok"))
            .await
            .expect("migration 087 must allow hkev_status = 'dirty'");
        let rooms = fetch_rooms(&pool).await.expect("room list must fetch");
        let progress = rooms
            .iter()
            .find(|r| r.room_id == room_id)
            .and_then(|r| r.cleaning.as_ref())
            .expect("today's progress present");
        assert_eq!(progress.status, "dirty", "latest event wins");

        // CHECK constraint still rejects everything else at the DB layer.
        let bad = seed_cleaning_event(&pool, room_id, "finished", "Q1001", None).await;
        assert!(bad.is_err(), "CHECK constraint must reject bad status");

        // Cleanup (cascades the events + reports).
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// The room-existence probe 404s inactive/unknown rooms.
    #[tokio::test]
    async fn require_room_rejects_unknown_and_inactive() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping require_room_rejects_unknown_and_inactive — PG not reachable");
            return;
        };
        // Unknown id.
        assert!(require_room(&pool, -424242).await.is_err());

        // Inactive room.
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HK2'")
            .execute(&pool)
            .await;
        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_active) \
             VALUES ('ZT-HK2', false) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = row.try_get("room_id").unwrap();
        assert!(require_room(&pool, room_id).await.is_err());
        assert!(
            fetch_room(&pool, room_id).await.unwrap().is_none(),
            "inactive rooms are hidden from the maid list"
        );
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }
}
