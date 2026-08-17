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
//! ## Per-employee location enforcement (wave-4 C — ships DARK)
//!
//! `HK_BRANCHES` answers "which properties does this DEPLOYMENT serve". It
//! never answered "which property does THIS EMPLOYEE work at" — so `/api/hk/me`
//! handed every maid the same global allowlist, and an HF Ville maid offered
//! "ฮาร์เบอร์ฟร้อนท์" could file a cleaning report (and, for `done`, a real
//! `MarkRoomClean` writeback) against the WRONG PROPERTY.
//!
//! HF ID holds the authoritative answer in `Employee.location`, so with
//! `HK_LOCATION_ENFORCEMENT_ENABLED=true` this surface asks it
//! ([`crate::hfid_location`]) and applies TWO rules:
//!
//! - `GET /api/hk/me` serves `branches` = `HK_BRANCHES` ∩ {the employee's own
//!   location}. Empty ⇒ `[]` plus [`MeResponse::branches_unavailable_reason`],
//!   so the client renders an actionable message instead of an empty picker.
//! - every room endpoint — READS AND MUTATIONS ALIKE — additionally requires
//!   the requested `?branch=` to EQUAL the employee's location
//!   ([`require_location`]). Reads are included deliberately: the room list is
//!   what a maid works from, and showing her the other property's rooms is
//!   already the wrong-hotel bug, one tap earlier.
//!
//! **The hard property: there is no fallback.** A null location, a lookup
//! miss, an inactive/pending employee, an unreachable HF ID, an unset URL or
//! an unset secret all REFUSE (403 / 503). None of them may degrade to the
//! allowlist or to `hfhotel`, and there is no per-badge carve-out — a
//! carve-out is indistinguishable from the bug. See [`location_gate`], which
//! is pure and exhaustive precisely so that property is checkable by reading
//! one `match`.
//!
//! Flag OFF (the default) ⇒ no lookup is performed at all and every response
//! is byte-identical to the pre-enforcement build.
//!
//! ### The one employee who works both properties (`housekeeping_admin`)
//!
//! Some staff genuinely cover both hotels, and the rule above locks them to
//! one. The exception is an HF ID GRANT — [`crate::hfid_location::HK_ADMIN_GRANT`],
//! read from the same `/resolve-badge` answer's `apps` list — which resolves to
//! [`LocationOutcome::AnyLocation`] and means "not bound to one property".
//!
//! Its effect here is exactly one line in each of the two rules above:
//! [`intersect_location`] serves the WHOLE allowlist (so `/api/hk/me` offers
//! both properties and the existing multi-branch picker appears, with no
//! frontend change), and [`location_gate`] admits any branch that already
//! cleared [`require_branch`].
//!
//! That last clause is the safety property, and it is why this is a widening
//! and not a hole: `HK_BRANCHES` still binds, because every handler runs
//! [`require_branch`] BEFORE [`require_location`]. A grant-holder gets the
//! properties this deployment serves — never one it does not.
//!
//! The grant overrides `location` ENTIRELY, a NULL location included. That is
//! deliberate and is NOT the silent-default hole this stream closed: a null
//! location is an absence (nobody decided anything — still refused), while the
//! grant is a decision someone made against a named employee and can be
//! audited in HF ID. What it does not override is `found`/`active`/`pending` —
//! see [`crate::hfid_location`], where the check order enforces that.
//!
//! It is not a way IN. [`crate::middleware::hk_access::HK_GRANT`]
//! (`"housekeeping"`) is what opens this surface, at the Access policy and in
//! the middleware behind it; `housekeeping_admin` alone is 403 before any of
//! the above runs. The grant widens branches for people who already hold
//! `housekeeping` — it never substitutes for it.
//!
//! **It ships DARK-COMPATIBLE**: no flag, because the darkness is the grant
//! itself. While no badge holds it, every lookup answers exactly what it
//! answered before and every response on this surface is byte-unchanged. The
//! activation is the owner ticking the box in HF ID, and it takes effect
//! within [`crate::hfid_location::LOCATION_CACHE_TTL`].
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
//! ## iHOTEL-WINS room status (CR-1, owner decision locked 2026-08-15)
//!
//! What a maid sees in [`HkRoom::room_clean`] is **iHOTEL's**
//! `HT_Rooms.Room_Clean`, not our canonical mirror. Reception works the iHOTEL
//! board; a maid working a different answer for the same room is the whole
//! problem. Canonical `ht_rooms_new.room_clean` becomes the MIRROR — the value
//! shown only when iHOTEL cannot be reached.
//!
//! The read itself is [`crate::legacy_room_status`] (read-only, `Room_no`-keyed,
//! 3s budget); the POLICY is here, in three rules that are unit-tested as pure
//! functions ([`merge_legacy_room_clean`]):
//!
//! 1. **iHOTEL wins** for every room it has an answer for. A room absent from
//!    the legacy answer (unmatched `Room_no`, unrecognised literal) keeps its
//!    canonical value — never guessed.
//! 2. **Unreachable ⇒ fall back, and SAY SO.** Every room keeps its canonical
//!    value and the response carries `legacyStatusStale: true`, which the
//!    client renders as a visible Thai note. Stale-but-shown beats a dead
//!    screen on a stairwell — the fallback is a first-class state, not an
//!    error path. There is no 5xx on this surface for a legacy outage.
//! 3. **Divergence is LOGGED, never shown.** A PG↔iHOTEL disagreement is a free
//!    sync-drift signal for the operator (`warn`, with `room_no` and BOTH
//!    values). It is not shown to the maid: she has exactly one job per room
//!    and no action to take about which database is behind.
//!
//! Deliberately UNCACHED. Staleness is the failure mode this whole change
//! exists to remove, and the load is one 34-58 row `SELECT` per room-list load
//! from a surface with a handful of users. The write path (below) reads the
//! same way and for the same reason — caching the answer there would put the
//! defect it fixes back, one layer down.
//!
//! ## The WRITE judges the same truth (defect D1, wave-5)
//!
//! The display was iHOTEL-wins while the write guard judged canonical
//! `ht_rooms_new.room_clean` — the CT mirror. A lagging mirror therefore made a
//! maid see DIRTY (iHOTEL), tap เสร็จแล้ว, and get บันทึกแล้ว while the guard
//! called it "already clean", enqueued nothing, and left the room dirty on
//! reception's board. Read and write judged two different databases; the
//! 0a30079 read-sync fix made the window easier to hit by refreshing the
//! display promptly.
//!
//! `POST …/cleaning` now hands the service iHOTEL's answer for that one room
//! ([`legacy_hint_for_room`]) and the service decides on BOTH truths
//! ([`crate::service::housekeeping::decide_cleanliness`]): canonical dirty ⇒
//! transition (unchanged); canonical already clean but iHOTEL dirty ⇒ MIRROR
//! REPAIR — no flip, but the writeback the tap earned.
//!
//! Three properties this deliberately keeps:
//!
//! 1. **The legacy read stays OUTSIDE the PG transaction**, before the service
//!    call, on the adapter's own 3s budget. No transaction is ever held open
//!    across a WG-tunnelled round-trip.
//! 2. **Unreachable ⇒ today's behaviour, never worse.** `Unavailable` is
//!    `LegacyCleanliness::Unknown`, which is canonical-only judgement — the
//!    same failure surface as before D1. A maid's tap is never failed, and
//!    never blocked on legacy availability.
//! 3. **Only would-be-no-op taps pay for the read** ([`needs_legacy_opinion`]),
//!    so a normal dirty→clean tap is exactly as fast as before.
//!
//! Both poles are covered, because the display is iHOTEL-wins at both: the
//! `dirty` tap gets the mirror-image repair (canonical already dirty, iHOTEL
//! clean ⇒ `MarkRoomDirty`). It stays behind `HK_MARK_DIRTY_ENABLED` — the
//! guard change does not widen that gate, it only makes the tap honest once the
//! gate is open.
//!
//! No writeback RECIPE changes: the intents, their SQL and their byte-parity
//! literals are untouched. D1 is a decision-layer defect and the fix lives in
//! the decision layer.
//!
//! Branch-aware via `?branch=` resolved through the unified
//! [`AppState::write_pool`] chokepoint (Ship-B gate). The legacy reader is
//! resolved by the SAME branch through [`HkPolicy::legacy_room_clean`], so a
//! Ville maid's list can only ever be reconciled against Ville's legacy server.
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

use std::collections::BTreeMap;
use std::sync::Arc;

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
use crate::config::HfidLocationConfig;
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::hfid_location::{EmployeeLocation, HfidLocationClient, LocationOutcome};
use crate::legacy_room_status::{RoomCleanOutcome, RoomCleanSource};
use crate::middleware::hk_access::HkIdentity;
use crate::outbox::event::EventSource;
use crate::service::housekeeping::{
    CleaningProgressStatus, HousekeepingService, LegacyCleanliness, ReportCleaningCommand,
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

/// Env var gating per-employee location enforcement. Default OFF — with it
/// off, this surface behaves byte-identically to the pre-enforcement build.
pub const HK_LOCATION_ENFORCEMENT_ENV: &str = "HK_LOCATION_ENFORCEMENT_ENABLED";

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

/// 403 body when the requested branch is not the employee's own location.
/// ACTIONABLE: it names what to do (pick your own branch) and who to ask.
pub const LOCATION_MISMATCH_ERROR: &str =
    "สาขาที่เลือกไม่ตรงกับสาขาที่คุณสังกัด กรุณาเลือกสาขาของคุณ หรือติดต่อผู้ดูแลระบบ";

/// 403 body when HF ID has no usable location for this employee — null
/// `location`, an unknown badge, or an inactive/pending employee. The fix is
/// an admin action, so that is what the message asks for.
pub const LOCATION_UNKNOWN_ERROR: &str = "ยังไม่ได้กำหนดสาขาของพนักงาน — ติดต่อผู้ดูแลระบบ";

/// 503 body when the location lookup could not answer (HF ID unreachable,
/// non-2xx, undecodable, or `HFID_LOCATION_URL`/`HFID_RESOLVE_SECRET` unset).
/// Distinct from [`LOCATION_UNKNOWN_ERROR`] on purpose: this one IS worth
/// retrying, so the message says so instead of sending the maid to an admin
/// for a blip.
pub const LOCATION_LOOKUP_UNAVAILABLE_ERROR: &str =
    "ระบบตรวจสอบสาขาพนักงานขัดข้องชั่วคราว กรุณาลองใหม่อีกครั้ง หากยังไม่ได้ ให้ติดต่อผู้ดูแลระบบ";

/// [`MeResponse::branches_unavailable_reason`] — HF ID answered definitively
/// and there is no branch for this employee: no location on file, unknown
/// badge, inactive/pending, or a location this deployment does not serve.
/// Machine-readable and STABLE; the frontend maps it to its own Thai copy.
pub const REASON_NO_LOCATION: &str = "no_location";

/// [`MeResponse::branches_unavailable_reason`] — the lookup itself could not
/// answer. Retryable, unlike [`REASON_NO_LOCATION`].
pub const REASON_LOOKUP_UNAVAILABLE: &str = "lookup_unavailable";

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
    ///
    /// This is the DEPLOYMENT's allowlist. With location enforcement on it is
    /// only the first of two gates — it is intersected with the individual
    /// employee's own location, never used in place of it.
    pub branches: Vec<Branch>,
    /// `HK_MARK_DIRTY_ENABLED` — ships DARK (default `false`).
    pub mark_dirty_enabled: bool,
    /// `HK_LOCATION_ENFORCEMENT_ENABLED` — ships DARK (default `false`).
    /// `false` ⇒ [`require_location`] is a no-op and no lookup is issued, so
    /// the surface is byte-identical to the pre-enforcement build.
    pub location_enforcement_enabled: bool,
    /// The HF ID badge → location client.
    ///
    /// `None` means `HFID_LOCATION_URL` / `HFID_RESOLVE_SECRET` are not both
    /// set, i.e. there is nothing to ask. With enforcement ON that is
    /// [`REASON_LOOKUP_UNAVAILABLE`] / `503` — deliberately NOT a pass-through,
    /// because "unconfigured" and "allowed" must never be the same state.
    pub location: Option<Arc<HfidLocationClient>>,
    /// Per-branch iHOTEL room-status readers (CR-1), keyed by the stable
    /// branch id ([`branch_id`]) so a Ville maid's list can only ever be
    /// reconciled against Ville's legacy server.
    ///
    /// A MISSING entry is a first-class, supported state — it means exactly
    /// what an unreachable legacy means: fall back to the canonical PG value
    /// and show the Thai stale note. That is deliberate and is what lets this
    /// ship compatible: the HF Hotel entry reuses the legacy pool `main.rs`
    /// already builds, and the Ville entry only appears once `HK_BRANCHES`
    /// admits Ville (V13), so today's deploy opens NO new legacy connection.
    ///
    /// Contrast [`Self::location`], which fails CLOSED — answering wrongly
    /// there sends a maid to the wrong hotel; answering nothing here would
    /// only blank a screen she needs.
    pub legacy_room_clean: BTreeMap<&'static str, Arc<dyn RoomCleanSource>>,
}

impl HkPolicy {
    /// Read `HK_BRANCHES` + `HK_MARK_DIRTY_ENABLED` +
    /// `HK_LOCATION_ENFORCEMENT_ENABLED` (and, for the last, the HF ID lookup
    /// configuration). Call ONCE at startup.
    ///
    /// The client is built even when the flag is off: it holds no connection,
    /// only a `ureq::Agent`, and building it unconditionally means the startup
    /// line can report whether the flip would actually have somewhere to call.
    pub fn from_env() -> Self {
        let raw = std::env::var(HK_BRANCHES_ENV).ok();
        let branches = parse_hk_branches(raw.as_deref());
        let mark_dirty_enabled = std::env::var(HK_MARK_DIRTY_ENV)
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let location_enforcement_enabled = std::env::var(HK_LOCATION_ENFORCEMENT_ENV)
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let location =
            HfidLocationClient::from_config(HfidLocationConfig::from_env()).map(Arc::new);
        Self {
            branches,
            mark_dirty_enabled,
            location_enforcement_enabled,
            location,
            // Populated by `main.rs` via `with_legacy_room_clean` — it owns
            // the legacy pools (and is the ONE place that already has HF
            // Hotel's). Empty here means the fallback path, which is a
            // correct, shippable state.
            legacy_room_clean: BTreeMap::new(),
        }
    }

    /// Attach a branch's iHOTEL room-status reader. Called by `main.rs` once
    /// per branch that has one; branches without a reader use the fallback.
    pub fn with_legacy_room_clean(
        mut self,
        branch: Branch,
        source: Arc<dyn RoomCleanSource>,
    ) -> Self {
        self.legacy_room_clean.insert(branch_id(branch), source);
        self
    }

    /// Branch ids that have a live iHOTEL reader — for the startup log, so an
    /// operator can see at a glance whether `/hk` is serving iHOTEL truth or
    /// the canonical mirror.
    pub fn legacy_room_clean_branches(&self) -> Vec<&'static str> {
        self.legacy_room_clean.keys().copied().collect()
    }

    /// Stable ids of the configured branches, for logging + `GET /api/hk/me`.
    pub fn branch_ids(&self) -> Vec<&'static str> {
        self.branches.iter().copied().map(branch_id).collect()
    }

    /// Whether the HF ID lookup has both halves of its configuration — for the
    /// startup log. `false` while enforcement is ON means every `/hk` request
    /// 503s, which is exactly what the flip checklist must catch BEFORE the
    /// flag is flipped (PENDING-VERIFICATIONS.md V14).
    pub fn location_lookup_configured(&self) -> bool {
        self.location.is_some()
    }
}

impl Default for HkPolicy {
    fn default() -> Self {
        Self {
            branches: vec![DEFAULT_HK_BRANCH],
            mark_dirty_enabled: false,
            location_enforcement_enabled: false,
            location: None,
            legacy_room_clean: BTreeMap::new(),
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

/// THE MAPPING. HF ID's `Employee.location` → our `?branch=` branch. PURE, and
/// pinned by an explicit table test below.
///
/// The two vocabularies are NOT the same strings and must never be bridged by
/// string manipulation. Lower-casing HF ID's values yields `"hf"` /
/// `"hf_ville"`, neither of which [`parse_branch_param`] accepts — so a naive
/// `to_lowercase()` bridge would turn every enforced request into a `400`
/// that reads like a client bug. Forwarding the raw token is worse: `"HF"` is
/// also refused, and `"HF_VILLE"` looks tantalisingly close to `hfville`.
/// Keeping the mapping a total function over a two-variant enum means the
/// compiler, not a reviewer, guarantees every location has exactly one branch.
fn location_branch(location: EmployeeLocation) -> Branch {
    match location {
        EmployeeLocation::Hf => Branch::Hfhotel,
        EmployeeLocation::HfVille => Branch::Hfville,
    }
}

/// The per-employee location gate, given the lookup's answer and the requested
/// branch. PURE and exhaustive — unit-tested below.
///
/// Every non-`Resolved` arm REFUSES. There is no arm that returns `Ok(())`
/// without an equality check against the employee's own location, and none
/// that falls back to [`DEFAULT_HK_BRANCH`] or to `policy.branches`. That is
/// the hard property of this whole stream, and it is checkable by reading this
/// one `match`.
fn location_gate(outcome: LocationOutcome, requested: Branch) -> ApiResult<()> {
    match outcome {
        LocationOutcome::Resolved(location) => {
            if location_branch(location) == requested {
                Ok(())
            } else {
                Err(ApiError::Forbidden(LOCATION_MISMATCH_ERROR.to_string()))
            }
        }
        // The `housekeeping_admin` grant: this employee is not bound to one
        // property, so any branch that got this far is theirs to act on.
        //
        // "That got this far" is doing real work and is not an accident of
        // ordering: every handler calls `require_branch` FIRST, so `requested`
        // is already known to be in `HK_BRANCHES`. The grant therefore widens
        // an employee to the DEPLOYMENT's allowlist and no further — an admin
        // asking for a branch this deployment does not serve is refused by the
        // allowlist before this function is ever called. That is the outer
        // bound, and it is the one thing the grant cannot cross.
        LocationOutcome::AnyLocation => Ok(()),
        // Definite answer, no usable branch. 403 (not 503): retrying changes
        // nothing, an admin must act.
        LocationOutcome::NoLocation => Err(ApiError::Forbidden(LOCATION_UNKNOWN_ERROR.to_string())),
        // No answer. 503 (not 403): the request may well be legitimate, we
        // simply cannot tell — and a maid must not be told "you are not
        // allowed" because a LAN cable is loose.
        LocationOutcome::Unavailable => Err(ApiError::ServiceUnavailable(
            LOCATION_LOOKUP_UNAVAILABLE_ERROR.to_string(),
        )),
    }
}

/// Ask HF ID for this badge's location, or report [`LocationOutcome::Unavailable`]
/// when no lookup is configured. NEVER returns a location it did not receive.
async fn resolve_location(policy: &HkPolicy, badge: &str) -> LocationOutcome {
    match policy.location.as_ref() {
        Some(client) => client.resolve(badge).await,
        // Enforcement is on but `HFID_LOCATION_URL` / `HFID_RESOLVE_SECRET`
        // are not both set. Fail closed — an unconfigured lookup must not be
        // indistinguishable from a permissive one.
        None => {
            tracing::warn!(
                "{HK_LOCATION_ENFORCEMENT_ENV} is on but the HF ID location lookup is \
                 unconfigured (need HFID_LOCATION_URL + HFID_RESOLVE_SECRET) — \
                 every /hk request will 503"
            );
            LocationOutcome::Unavailable
        }
    }
}

/// The location gate as the room handlers call it: a no-op while the flag is
/// off (NO lookup is issued — the dark path costs nothing and touches no
/// network), the full [`location_gate`] when it is on.
///
/// Applied to READS as well as mutations. A maid who can list the other
/// property's rooms is already looking at the wrong hotel; refusing only the
/// write would leave the bug visible and one tap away.
async fn require_location(
    policy: &HkPolicy,
    identity: &HkIdentity,
    branch: Branch,
) -> ApiResult<()> {
    if !policy.location_enforcement_enabled {
        return Ok(());
    }
    let outcome = resolve_location(policy, &identity.badge).await;
    let result = location_gate(outcome, branch);
    if result.is_err() {
        tracing::warn!(
            badge = %identity.badge,
            requested_branch = branch_id(branch),
            ?outcome,
            "/hk location gate refused a request"
        );
    }
    result
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
///
/// EVERY handler here now extracts `Extension<HkIdentity>` (the location gate
/// needs the badge on reads too, not just on the mutation), so a mount without
/// an identity layer is a 500 rather than a silently unauthenticated surface.
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
    /// Branches this maid may pick, in `HK_BRANCHES` order. A single entry
    /// means the client auto-selects and renders NO picker.
    ///
    /// With location enforcement ON this is `HK_BRANCHES` ∩ {the employee's
    /// own location}, so it is normally length 1 — and CAN be EMPTY, which is
    /// what [`Self::branches_unavailable_reason`] explains. (Before
    /// enforcement it was documented "never empty"; that invariant is now the
    /// flag-off case only.)
    pub branches: Vec<HkBranchOption>,
    /// `HK_MARK_DIRTY_ENABLED`. `false` ⇒ the client hides the
    /// "แจ้งห้องไม่สะอาด" button rather than offering a dead tap.
    pub mark_dirty_enabled: bool,
    /// Why [`Self::branches`] is empty, when it is: [`REASON_NO_LOCATION`] or
    /// [`REASON_LOOKUP_UNAVAILABLE`]. `None` whenever `branches` is non-empty
    /// (and always, while enforcement is off).
    ///
    /// ADDITIVE and always serialized (as `null` in the normal case) so the
    /// field's presence is stable and the client can branch on the VALUE
    /// rather than on whether the key exists. A machine-readable code rather
    /// than a message: the Thai copy belongs to the client, which knows
    /// whether it is rendering a picker or a room list.
    pub branches_unavailable_reason: Option<&'static str>,
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
    /// Cleanliness as **iHOTEL** reports it (CR-1) — `true` = clean.
    ///
    /// Sourced from legacy `HT_Rooms.Room_Clean` (INVERTED: `'no'` = clean)
    /// whenever iHOTEL answers for this `room_no`. Falls back to canonical
    /// `ht_rooms_new.room_clean` when the legacy read is unavailable (see the
    /// response's `legacy_status_stale`) or when iHOTEL has no usable value for
    /// this particular room. See [`merge_legacy_room_clean`].
    pub room_clean: bool,
    /// Today's latest maid-reported progress; `None` = nothing reported yet.
    pub cleaning: Option<CleaningProgress>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomsResponse {
    pub success: bool,
    pub data: Vec<HkRoom>,
    /// `true` when the iHOTEL read could not answer and every `roomClean`
    /// above is therefore the canonical PG MIRROR rather than iHOTEL truth
    /// (CR-1 rule 2). The client renders a visible Thai note; it must never
    /// render an error page — a stale list is usable, a blank one is not.
    ///
    /// ADDITIVE and always serialized, so a client can branch on the VALUE
    /// rather than on whether the key exists. A machine-readable flag rather
    /// than a message: the Thai copy belongs to the client, same precedent as
    /// [`MeResponse::branches_unavailable_reason`].
    pub legacy_status_stale: bool,
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
    /// Same meaning as [`RoomsResponse::legacy_status_stale`]. Carried here
    /// too so the two screens can never tell the maid different stories about
    /// the same room.
    pub legacy_status_stale: bool,
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
// iHOTEL-wins merge (CR-1) — pure, unit-tested below
// ============================================================================

/// Overwrite each room's `room_clean` with iHOTEL's answer, and report whether
/// the maid is looking at a fallback.
///
/// Returns `true` when the caller must set `legacy_status_stale` — i.e. the
/// legacy read did not answer at all and every value left in `rooms` is the
/// canonical PG mirror.
///
/// The three CR-1 rules, in one place (see the module docs):
///
/// * iHOTEL wins per room it has a usable value for;
/// * a room iHOTEL has no usable value for keeps its canonical value SILENTLY
///   — that is a mapping gap, not a staleness event, and flagging the whole
///   list for one unmatched room would train the maid to ignore the note;
/// * every disagreement is logged at `warn` with `room_no` and BOTH values,
///   and NONE of it reaches the response.
///
/// PURE apart from the log line, which is exactly why the divergence rule is
/// testable without a database or a legacy server.
pub(crate) fn merge_legacy_room_clean(
    rooms: &mut [HkRoom],
    outcome: &RoomCleanOutcome,
    branch: Branch,
) -> bool {
    let legacy = match outcome {
        RoomCleanOutcome::Available(map) => map,
        // Rule 2. Values stay as fetched from PG; the caller tells the client.
        RoomCleanOutcome::Unavailable => return true,
    };

    let mut divergences = 0usize;
    for room in rooms.iter_mut() {
        let Some(&legacy_clean) = legacy.get(room.room_no.trim()) else {
            continue;
        };
        if legacy_clean != room.room_clean {
            divergences += 1;
            // Rule 3 — operator-facing ONLY. `room_no` plus both values is
            // everything needed to chase it into the CT watcher or the
            // legacy-key repair (`bin/repair_room_legacy_keys`), which is the
            // known cause of this class at HF Ville.
            tracing::warn!(
                branch = branch_id(branch),
                room_no = %room.room_no,
                ihotel_clean = legacy_clean,
                pms_clean = room.room_clean,
                "/hk room-clean divergence: iHOTEL and canonical PG disagree — \
                 showing iHOTEL (CR-1); canonical is the mirror"
            );
        }
        // Rule 1.
        room.room_clean = legacy_clean;
    }

    if divergences > 0 {
        tracing::warn!(
            branch = branch_id(branch),
            divergences,
            rooms = rooms.len(),
            "/hk served iHOTEL room-clean status over a diverging canonical mirror"
        );
    }
    false
}

/// Ask this branch's iHOTEL reader, or report [`RoomCleanOutcome::Unavailable`]
/// when the branch has none configured.
///
/// "No reader" and "reader failed" are the SAME answer on purpose: both mean
/// the maid is about to see the canonical mirror, and she must be told so
/// either way. Collapsing them keeps the fallback path single — the one that
/// ships today, and the one an operator can reason about at 6am.
async fn resolve_legacy_room_clean(policy: &HkPolicy, branch: Branch) -> RoomCleanOutcome {
    match policy.legacy_room_clean.get(branch_id(branch)) {
        Some(source) => source.room_clean().await,
        None => {
            tracing::debug!(
                branch = branch_id(branch),
                "/hk has no iHOTEL room-status reader for this branch — \
                 serving the canonical PG mirror with the stale note"
            );
            RoomCleanOutcome::Unavailable
        }
    }
}

// ============================================================================
// D1 write guard — the WRITE half of "iHOTEL wins" (wave-5). Pure, unit-tested.
// ============================================================================

/// The canonical cleanliness a reported status is asking for. `None` for
/// `started`, which never touches cleanliness and never mirrors.
pub(crate) fn target_clean_for(status: CleaningProgressStatus) -> Option<bool> {
    match status {
        CleaningProgressStatus::Started => None,
        CleaningProgressStatus::Done => Some(true),
        CleaningProgressStatus::Dirty => Some(false),
    }
}

/// Pick ONE room's answer out of a legacy read outcome.
///
/// The same three-way answer [`merge_legacy_room_clean`] applies to the display,
/// reduced to the single room a tap is about: a room ABSENT from the legacy
/// answer (unmatched `Room_no`, unrecognised `Room_Clean` literal) is
/// [`LegacyCleanliness::Unknown`], never guessed — exactly as the display keeps
/// such a room's canonical value rather than inventing one. `Unavailable` is
/// Unknown too, which is what makes a legacy outage degrade the write path to
/// its pre-D1 behaviour instead of blocking or failing a maid's tap.
pub(crate) fn legacy_hint_for_room(outcome: &RoomCleanOutcome, room_no: &str) -> LegacyCleanliness {
    match outcome {
        RoomCleanOutcome::Available(map) => match map.get(room_no.trim()) {
            Some(true) => LegacyCleanliness::Clean,
            Some(false) => LegacyCleanliness::Dirty,
            None => LegacyCleanliness::Unknown,
        },
        RoomCleanOutcome::Unavailable => LegacyCleanliness::Unknown,
    }
}

/// Whether this tap needs iHOTEL's opinion before the service decides.
///
/// ONLY when canonical alone would answer "already in that state" — i.e. only
/// on the taps that would otherwise become the silent no-op D1 is about. A tap
/// that carries a real canonical transition is already going to enqueue, so
/// asking iHOTEL could not change the outcome and would only spend the reader's
/// 3s budget on the maid's request path.
///
/// Consequence worth stating plainly: during a legacy outage the ONLY taps that
/// wait for the budget are the ones that will answer "nothing to do" anyway, and
/// a normal dirty→clean tap is as fast as it was before D1.
///
/// The canonical value here is read outside the transaction, so it can be stale
/// by the time the service takes the row lock. Both directions of that race are
/// benign: it can only have moved to the target state via our own concurrent
/// duplicate tap, a front-desk mark-clean (which enqueues its own writeback), or
/// the CT watcher applying legacy's own value — and in every one of those cases
/// the no-op the service then decides on is the CORRECT answer.
pub(crate) fn needs_legacy_opinion(target_clean: Option<bool>, canonical: Option<bool>) -> bool {
    match target_clean {
        Some(target) => canonical == Some(target),
        None => false,
    }
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

/// What the 404 probe now brings back with it: the room's legacy join key and
/// its canonical cleanliness, both needed by the D1 write guard.
///
/// Folded into the EXISTING existence probe rather than added as a second
/// query — the handler already had to make this round-trip.
struct RoomGuardRow {
    /// `HT_Rooms.Room_no` — the key [`crate::legacy_room_status`] answers on
    /// (the proven-truthful pointer; see that module's header).
    room_no: String,
    /// Canonical `ht_rooms_new.room_clean`; `None` = SQL NULL.
    ///
    /// Read OUTSIDE the service's transaction, so it is a hint only: it decides
    /// whether the legacy read is worth issuing, never what gets written. The
    /// service re-reads under the row lock and decides there.
    room_clean: Option<bool>,
}

/// 404-checking room probe (active rooms only), returning what the write guard
/// needs.
async fn require_room(pool: &PgPool, room_id: i32) -> ApiResult<RoomGuardRow> {
    let row = sqlx::query(
        "SELECT room_no, room_clean FROM ht_rooms_new \
          WHERE room_id = $1 AND COALESCE(room_active, true) = true",
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("room {room_id} not found")))?;
    Ok(RoomGuardRow {
        room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
        room_clean: row.try_get::<Option<bool>, _>("room_clean").unwrap_or(None),
    })
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
    let (branches, branches_unavailable_reason) = me_branches(&policy, &identity).await;
    Ok(Json(MeResponse {
        success: true,
        badge: identity.badge,
        display_name: identity.display_name,
        branches: branches
            .into_iter()
            .map(|b| HkBranchOption {
                id: branch_id(b).to_string(),
                label_th: branch_label_th(b).to_string(),
            })
            .collect(),
        mark_dirty_enabled: policy.mark_dirty_enabled,
        branches_unavailable_reason,
    }))
}

/// The branch list `GET /api/hk/me` serves, plus the reason it is empty.
///
/// Flag off ⇒ the whole `HK_BRANCHES` allowlist, unchanged and with no lookup.
/// Flag on ⇒ the INTERSECTION of the allowlist with the employee's own
/// location, which is the picker-shaped half of the same rule
/// [`location_gate`] enforces per request. The two must agree, or the picker
/// would offer a branch every subsequent call then 403s.
async fn me_branches(
    policy: &HkPolicy,
    identity: &HkIdentity,
) -> (Vec<Branch>, Option<&'static str>) {
    if !policy.location_enforcement_enabled {
        return (policy.branches.clone(), None);
    }
    let outcome = resolve_location(policy, &identity.badge).await;
    (
        intersect_location(&policy.branches, outcome),
        me_reason(&policy.branches, outcome),
    )
}

/// `HK_BRANCHES` ∩ {employee's location}. PURE — unit-tested below.
///
/// At most one element, and never a branch outside `allowed`: the deployment
/// allowlist still binds. An employee whose property this deployment does not
/// serve yet (an `HF_VILLE` maid while `HK_BRANCHES=hfhotel`) therefore gets
/// `[]`, NOT `hfhotel` — the intersection is what makes that fall out
/// automatically instead of needing its own carve-out.
fn intersect_location(allowed: &[Branch], outcome: LocationOutcome) -> Vec<Branch> {
    match outcome {
        LocationOutcome::Resolved(location) => {
            let branch = location_branch(location);
            if allowed.contains(&branch) {
                vec![branch]
            } else {
                Vec::new()
            }
        }
        // `housekeeping_admin`: "any location" ∩ the allowlist IS the
        // allowlist — every branch this deployment serves, in `HK_BRANCHES`
        // order, and never one it does not. Today that is both properties, so
        // the existing multi-branch picker appears for exactly these
        // employees with no frontend change: `resolveInitialBranch` returns
        // `null` for a two-entry list with nothing stored (app/hk/hk-lib.ts),
        // which is precisely the "render the picker and block" signal.
        LocationOutcome::AnyLocation => allowed.to_vec(),
        LocationOutcome::NoLocation | LocationOutcome::Unavailable => Vec::new(),
    }
}

/// The machine-readable reason accompanying an empty [`intersect_location`].
/// PURE — unit-tested below.
///
/// Note the deliberate collapse: an employee at a real property this
/// deployment does not serve reports [`REASON_NO_LOCATION`], not
/// [`REASON_LOOKUP_UNAVAILABLE`]. The lookup WORKED; nothing is going to
/// change by retrying, and `lookup_unavailable` would invite exactly that. The
/// client's copy for this reason therefore covers both "no location on file"
/// and "your branch is not enabled here" — both end at "contact an admin".
fn me_reason(allowed: &[Branch], outcome: LocationOutcome) -> Option<&'static str> {
    if !intersect_location(allowed, outcome).is_empty() {
        return None;
    }
    match outcome {
        LocationOutcome::Unavailable => Some(REASON_LOOKUP_UNAVAILABLE),
        // `AnyLocation` reaches here only when the ALLOWLIST is empty, which no
        // env can produce (`parse_hk_branches` always yields at least one
        // branch). It is grouped with the definite answers rather than with the
        // outage because that is what it would be: the lookup worked, this
        // deployment simply serves nothing — not a thing a retry fixes.
        LocationOutcome::NoLocation
        | LocationOutcome::Resolved(_)
        | LocationOutcome::AnyLocation => Some(REASON_NO_LOCATION),
    }
}

/// GET /api/hk/rooms — the maid's room list with today's progress.
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Json<RoomsResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let pool = resolve_pool(&state, branch)?;
    let mut data = fetch_rooms(pool).await?;
    // CR-1: iHOTEL wins. A legacy failure NEVER fails the request — it
    // degrades to the canonical mirror plus the client-rendered Thai note.
    let outcome = resolve_legacy_room_clean(&policy, branch).await;
    let legacy_status_stale = merge_legacy_room_clean(&mut data, &outcome, branch);
    Ok(Json(RoomsResponse {
        success: true,
        data,
        legacy_status_stale,
    }))
}

/// GET /api/hk/rooms/{id} — one room with today's cleaning events.
pub async fn room_detail(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let pool = resolve_pool(&state, branch)?;
    let room = fetch_room(pool, room_id)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("room {room_id} not found")))?;
    let events = fetch_today_events(pool, room_id).await?;
    // CR-1: the SAME merge as the list, so a maid who taps into a room never
    // sees a different answer than the tile she tapped.
    let mut rooms = [room];
    let outcome = resolve_legacy_room_clean(&policy, branch).await;
    let legacy_status_stale = merge_legacy_room_clean(&mut rooms, &outcome, branch);
    let [room] = rooms;
    Ok(Json(RoomDetailResponse {
        success: true,
        room,
        events,
        legacy_status_stale,
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
/// The location gate joins the FIRST tier, immediately after the allowlist
/// check and still ahead of status validation: "you may not act on this
/// property at all" outranks "that status is misspelled". While the flag is
/// off it is a no-op, so the pinned order is unchanged for the dark build.
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
    require_location(&policy, &identity, branch).await?;
    let status = parse_cleaning_status(&body.status)?;

    // Invariant #6: the mark-dirty TRIGGER is new even though the legacy write
    // shape is proven. Off ⇒ 403, in Thai, with the maid's own envelope.
    if status == CleaningProgressStatus::Dirty && !policy.mark_dirty_enabled {
        return Err(ApiError::Forbidden(MARK_DIRTY_DISABLED_ERROR.to_string()));
    }

    let pool = resolve_pool(&state, branch)?;
    let room = require_room(pool, room_id).await?;

    // D1: judge the write against the SAME truth the maid's screen was rendered
    // from. Only asked when canonical alone would have said "already done" —
    // see `needs_legacy_opinion`. Unreachable / unmapped ⇒ `Unknown` ⇒ the
    // service falls back to its pre-D1, canonical-only judgement. A maid's tap
    // is NEVER failed or gated on legacy availability.
    let target_clean = target_clean_for(status);
    let legacy_room_clean = if needs_legacy_opinion(target_clean, room.room_clean) {
        let outcome = resolve_legacy_room_clean(&policy, branch).await;
        legacy_hint_for_room(&outcome, &room.room_no)
    } else {
        LegacyCleanliness::Unknown
    };

    let svc = service_for(&state, branch)?;
    let report = svc
        .report_cleaning_progress(ReportCleaningCommand {
            room_id,
            status,
            legacy_room_clean,
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
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Response> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
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
            ..HkPolicy::default()
        }
    }

    /// A policy with location enforcement ON and a scripted lookup behind it.
    fn enforcing(branches: Vec<Branch>, outcome: LocationOutcome) -> HkPolicy {
        HkPolicy {
            branches,
            location_enforcement_enabled: true,
            location: Some(Arc::new(HfidLocationClient::with_lookup(Arc::new(
                FixedLookup(outcome),
            )))),
            ..HkPolicy::default()
        }
    }

    struct FixedLookup(LocationOutcome);

    #[async_trait::async_trait]
    impl crate::hfid_location::LocationLookup for FixedLookup {
        async fn lookup(&self, _badge: &str) -> LocationOutcome {
            self.0
        }
    }

    fn maid(badge: &str) -> HkIdentity {
        HkIdentity {
            badge: badge.to_string(),
            display_name: None,
            email: None,
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

    /// The dark-ship default: no env ⇒ HF Hotel only, mark-dirty off,
    /// location enforcement off and no lookup wired.
    #[test]
    fn default_policy_ships_dark() {
        let p = HkPolicy::default();
        assert_eq!(p.branches, vec![Branch::Hfhotel]);
        assert!(!p.mark_dirty_enabled);
        assert_eq!(p.branch_ids(), vec!["hfhotel"]);
        assert!(
            !p.location_enforcement_enabled,
            "location enforcement must ship DARK"
        );
        assert!(!p.location_lookup_configured());
    }

    // ---- the employee-location gate -------------------------------------

    /// THE MAPPING TABLE, pinned explicitly. HF→hfhotel, HF_VILLE→hfville, and
    /// nothing else — this is the trap the whole stream turns on.
    #[test]
    fn employee_location_maps_onto_exactly_one_branch_each() {
        assert_eq!(location_branch(EmployeeLocation::Hf), Branch::Hfhotel);
        assert_eq!(location_branch(EmployeeLocation::HfVille), Branch::Hfville);
        // The mapping is NOT string manipulation, and must never become it:
        // HF ID's own spellings are refused outright by the `?branch=` parser,
        // and so are their lower-cased forms. Both of the tempting shortcuts
        // (forward the raw token / naive to_lowercase) would produce a 400.
        for raw in ["HF", "HF_VILLE", "hf", "hf_ville"] {
            assert_eq!(
                parse_branch_param(Some(raw)),
                None,
                "{raw:?} must never be usable as a ?branch= value"
            );
        }
        // …whereas the mapped branches round-trip through the wire spelling.
        assert_eq!(
            parse_branch_param(Some(branch_id(location_branch(EmployeeLocation::Hf)))),
            Some(Branch::Hfhotel)
        );
        assert_eq!(
            parse_branch_param(Some(branch_id(location_branch(EmployeeLocation::HfVille)))),
            Some(Branch::Hfville)
        );
    }

    /// The gate's full matrix. Every non-`Resolved` arm refuses, and the two
    /// refusal KINDS are distinct: 403 for a definite "no", 503 for "cannot
    /// tell". Nothing here can return `Ok` for a branch the employee does not
    /// belong to.
    #[test]
    fn location_gate_admits_only_an_exact_match() {
        // Match ⇒ admitted, both ways round.
        assert!(location_gate(
            LocationOutcome::Resolved(EmployeeLocation::Hf),
            Branch::Hfhotel
        )
        .is_ok());
        assert!(location_gate(
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            Branch::Hfville
        )
        .is_ok());

        // Mismatch ⇒ 403 with the actionable message, both ways round.
        for (outcome, requested) in [
            (
                LocationOutcome::Resolved(EmployeeLocation::Hf),
                Branch::Hfville,
            ),
            (
                LocationOutcome::Resolved(EmployeeLocation::HfVille),
                Branch::Hfhotel,
            ),
        ] {
            match location_gate(outcome, requested).expect_err("mismatch must refuse") {
                ApiError::Forbidden(msg) => assert_eq!(msg, LOCATION_MISMATCH_ERROR),
                other => panic!("expected 403, got {other:?}"),
            }
        }

        // Null location / miss / inactive / pending ⇒ 403, actionable.
        match location_gate(LocationOutcome::NoLocation, Branch::Hfhotel)
            .expect_err("no location must refuse")
        {
            ApiError::Forbidden(msg) => assert_eq!(msg, LOCATION_UNKNOWN_ERROR),
            other => panic!("expected 403, got {other:?}"),
        }

        // Unreachable / unconfigured ⇒ 503, NOT 403: the request may be
        // legitimate, we simply cannot tell.
        match location_gate(LocationOutcome::Unavailable, Branch::Hfhotel)
            .expect_err("an unavailable lookup must refuse")
        {
            ApiError::ServiceUnavailable(msg) => {
                assert_eq!(msg, LOCATION_LOOKUP_UNAVAILABLE_ERROR)
            }
            other => panic!("expected 503, got {other:?}"),
        }
    }

    /// …with ONE exception, and it is an explicit grant rather than a fallback:
    /// `housekeeping_admin` (⇒ `AnyLocation`) clears the gate for either
    /// branch. Note what this test canNOT show, because the gate never sees it:
    /// a branch outside `HK_BRANCHES` — `require_branch` has already 403'd it.
    /// That composition is asserted by
    /// `admin_grant_cannot_cross_the_hk_branches_allowlist` below.
    #[test]
    fn location_gate_admits_an_admin_grant_on_either_branch() {
        for requested in [Branch::Hfhotel, Branch::Hfville] {
            assert!(
                location_gate(LocationOutcome::AnyLocation, requested).is_ok(),
                "a grant-holder must be admitted on {requested:?}"
            );
        }
    }

    /// The outer bound, stated as a test: the grant widens an employee to the
    /// DEPLOYMENT's allowlist and not one branch further. `require_branch`
    /// runs first in every handler, so a grant-holder asking for a branch this
    /// deployment does not serve gets the ALLOWLIST 403 — the operator reads
    /// "widen HK_BRANCHES", which is the actual fix, rather than a location
    /// message about an employee who is not the problem.
    #[test]
    fn admin_grant_cannot_cross_the_hk_branches_allowlist() {
        let hotel_only = policy(vec![Branch::Hfhotel]);
        match require_branch(&hotel_only, Some("hfville"))
            .expect_err("an unserved branch must be refused before the location gate")
        {
            ApiError::Forbidden(msg) => assert_eq!(msg, BRANCH_NOT_ENABLED_ERROR),
            other => panic!("expected 403, got {other:?}"),
        }
        // The grant is irrelevant to that decision — it is not even consulted,
        // which is exactly why the allowlist is an outer bound and not a
        // second opinion.
        assert_eq!(
            intersect_location(&[Branch::Hfhotel], LocationOutcome::AnyLocation),
            vec![Branch::Hfhotel],
            "an hfhotel-only deployment offers a grant-holder hfhotel ONLY"
        );
    }

    /// The hard property, stated as a test: no outcome and no requested branch
    /// combination ever admits a branch the employee does not belong to — in
    /// particular nothing falls back to `DEFAULT_HK_BRANCH`.
    ///
    /// `should_admit` is an exhaustive `match`, not a `matches!`: a future
    /// outcome variant must be classified here DELIBERATELY (the compiler will
    /// stop on this arm) rather than sliding silently into "not admitted" —
    /// or, far worse, into an admitting default.
    #[test]
    fn location_gate_never_falls_back_to_a_default_branch() {
        let every_outcome = [
            LocationOutcome::Resolved(EmployeeLocation::Hf),
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            LocationOutcome::AnyLocation,
            LocationOutcome::NoLocation,
            LocationOutcome::Unavailable,
        ];
        for outcome in every_outcome {
            for requested in [Branch::Hfhotel, Branch::Hfville] {
                let admitted = location_gate(outcome, requested).is_ok();
                let should_admit = match outcome {
                    // Bound to one property: that property only.
                    LocationOutcome::Resolved(loc) => location_branch(loc) == requested,
                    // Explicitly granted every property this deployment
                    // serves. `requested` reached the gate, so `require_branch`
                    // already proved it is one of them.
                    LocationOutcome::AnyLocation => true,
                    // Nothing else admits anything, ever.
                    LocationOutcome::NoLocation | LocationOutcome::Unavailable => false,
                };
                assert_eq!(
                    admitted, should_admit,
                    "{outcome:?} + {requested:?} must {} be admitted",
                    if should_admit { "" } else { "NOT" }
                );
            }
        }
    }

    /// Flag OFF ⇒ the gate is a no-op AND no lookup is issued. The `location`
    /// client here would answer `Unavailable` (⇒ 503) if it were ever asked,
    /// so an `Ok` proves the dark path never reaches it.
    #[tokio::test]
    async fn location_gate_is_inert_while_the_flag_is_off() {
        let dark = HkPolicy {
            branches: vec![Branch::Hfhotel, Branch::Hfville],
            location_enforcement_enabled: false,
            location: Some(Arc::new(HfidLocationClient::with_lookup(Arc::new(
                FixedLookup(LocationOutcome::Unavailable),
            )))),
            ..HkPolicy::default()
        };
        for branch in [Branch::Hfhotel, Branch::Hfville] {
            assert!(
                require_location(&dark, &maid("421"), branch).await.is_ok(),
                "the dark build must not consult HF ID at all"
            );
        }
    }

    /// Enforcement ON with NO lookup configured (`HFID_LOCATION_URL` /
    /// `HFID_RESOLVE_SECRET` unset) must fail closed with 503 — never pass
    /// through. "Unconfigured" and "allowed" must not be the same state.
    #[tokio::test]
    async fn unconfigured_lookup_fails_closed_when_enforcement_is_on() {
        let p = HkPolicy {
            branches: vec![Branch::Hfhotel],
            location_enforcement_enabled: true,
            location: None,
            ..HkPolicy::default()
        };
        match require_location(&p, &maid("421"), Branch::Hfhotel)
            .await
            .expect_err("an unconfigured lookup must refuse")
        {
            ApiError::ServiceUnavailable(msg) => {
                assert_eq!(msg, LOCATION_LOOKUP_UNAVAILABLE_ERROR)
            }
            other => panic!("expected 503, got {other:?}"),
        }
    }

    /// End-to-end through the real client + cache: a HF_VILLE employee is
    /// admitted for `hfville` and refused for `hfhotel`.
    #[tokio::test]
    async fn require_location_enforces_the_employees_own_branch() {
        let p = enforcing(
            vec![Branch::Hfhotel, Branch::Hfville],
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
        );
        assert!(require_location(&p, &maid("421"), Branch::Hfville)
            .await
            .is_ok());
        match require_location(&p, &maid("421"), Branch::Hfhotel)
            .await
            .expect_err("a Ville maid must not act on HF Hotel")
        {
            ApiError::Forbidden(msg) => assert_eq!(msg, LOCATION_MISMATCH_ERROR),
            other => panic!("expected 403, got {other:?}"),
        }
    }

    // ---- `GET /api/hk/me` branch filtering ------------------------------

    /// Flag off ⇒ the allowlist verbatim, in order, with no reason field.
    #[tokio::test]
    async fn me_serves_the_whole_allowlist_while_enforcement_is_off() {
        let p = policy(vec![Branch::Hfhotel, Branch::Hfville]);
        let (branches, reason) = me_branches(&p, &maid("421")).await;
        assert_eq!(branches, vec![Branch::Hfhotel, Branch::Hfville]);
        assert_eq!(reason, None);
    }

    /// Flag on ⇒ the intersection. One entry means the client auto-selects and
    /// renders no picker — the maid sees only her own property.
    #[tokio::test]
    async fn me_filters_the_allowlist_to_the_employees_location() {
        let p = enforcing(
            vec![Branch::Hfhotel, Branch::Hfville],
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
        );
        let (branches, reason) = me_branches(&p, &maid("421")).await;
        assert_eq!(branches, vec![Branch::Hfville]);
        assert_eq!(reason, None);
    }

    /// The intersection, stated directly — including the case that has no
    /// carve-out: an `HF_VILLE` employee while `HK_BRANCHES` is hfhotel-only
    /// gets `[]`, never `hfhotel`. (This is today's production shape, and the
    /// reason V13 must widen `HK_BRANCHES` before Ville staff can work.)
    #[test]
    fn intersection_never_widens_past_the_allowlist() {
        let hotel_only = [Branch::Hfhotel];
        assert_eq!(
            intersect_location(
                &hotel_only,
                LocationOutcome::Resolved(EmployeeLocation::HfVille)
            ),
            Vec::<Branch>::new(),
            "an unserved property yields NO branch, not the default one"
        );
        assert_eq!(
            intersect_location(&hotel_only, LocationOutcome::Resolved(EmployeeLocation::Hf)),
            vec![Branch::Hfhotel]
        );
        for empty in [LocationOutcome::NoLocation, LocationOutcome::Unavailable] {
            assert!(
                intersect_location(&hotel_only, empty).is_empty(),
                "{empty:?} must yield no branch"
            );
        }
        // The grant is the one widening — to the allowlist, IN ITS ORDER, and
        // no further. Both rows matter: the first is what a grant-holder is
        // offered today (both properties ⇒ the picker), the second is that the
        // widening is still bounded by what this deployment serves.
        assert_eq!(
            intersect_location(
                &[Branch::Hfhotel, Branch::Hfville],
                LocationOutcome::AnyLocation
            ),
            vec![Branch::Hfhotel, Branch::Hfville],
            "a grant-holder is offered every served property, in HK_BRANCHES order"
        );
        assert_eq!(
            intersect_location(&hotel_only, LocationOutcome::AnyLocation),
            vec![Branch::Hfhotel],
            "…and never one the deployment does not serve"
        );
    }

    /// `/api/hk/me` for a grant-holder: BOTH properties, no reason — which is
    /// the shape the existing frontend already turns into a picker
    /// (`resolveInitialBranch` returns `null` for a 2-entry list with nothing
    /// stored, and `HkBranchChip` renders its switcher whenever
    /// `branches.length > 1`). That is why this capability needs no frontend
    /// change: it produces a response shape the client has always handled.
    #[tokio::test]
    async fn me_offers_a_grant_holder_every_served_property() {
        let p = enforcing(
            vec![Branch::Hfhotel, Branch::Hfville],
            LocationOutcome::AnyLocation,
        );
        let (branches, reason) = me_branches(&p, &maid("ADMIN")).await;
        assert_eq!(branches, vec![Branch::Hfhotel, Branch::Hfville]);
        assert_eq!(reason, None, "nothing is missing, so nothing to explain");
    }

    /// The reason codes are stable strings and map as documented — including
    /// the deliberate collapse of "real property, not served here" onto
    /// `no_location` (the lookup worked; retrying is pointless).
    #[test]
    fn me_reason_is_set_only_when_the_intersection_is_empty() {
        let both = [Branch::Hfhotel, Branch::Hfville];
        let hotel_only = [Branch::Hfhotel];

        assert_eq!(REASON_NO_LOCATION, "no_location");
        assert_eq!(REASON_LOOKUP_UNAVAILABLE, "lookup_unavailable");

        // Non-empty intersection ⇒ no reason.
        assert_eq!(
            me_reason(&both, LocationOutcome::Resolved(EmployeeLocation::HfVille)),
            None
        );
        // Definite answers with no usable branch ⇒ no_location.
        assert_eq!(
            me_reason(&both, LocationOutcome::NoLocation),
            Some(REASON_NO_LOCATION)
        );
        assert_eq!(
            me_reason(
                &hotel_only,
                LocationOutcome::Resolved(EmployeeLocation::HfVille)
            ),
            Some(REASON_NO_LOCATION),
            "a real but unserved property is not a retryable outage"
        );
        // No answer at all ⇒ lookup_unavailable.
        assert_eq!(
            me_reason(&both, LocationOutcome::Unavailable),
            Some(REASON_LOOKUP_UNAVAILABLE)
        );
        // A grant-holder is never short of a branch, so never carries a reason.
        assert_eq!(me_reason(&both, LocationOutcome::AnyLocation), None);
        assert_eq!(me_reason(&hotel_only, LocationOutcome::AnyLocation), None);
    }

    /// `/me` and the per-request gate must agree: every branch `/me` offers
    /// must be one `require_location` then admits, for the same employee.
    /// A disagreement is the worst shape — a picker that offers a branch whose
    /// every subsequent call 403s.
    #[tokio::test]
    async fn me_offers_only_branches_the_gate_will_admit() {
        for outcome in [
            LocationOutcome::Resolved(EmployeeLocation::Hf),
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            // The grant-holder row is the one that would break loudest: `/me`
            // offers TWO branches here, and BOTH must clear the gate.
            LocationOutcome::AnyLocation,
            LocationOutcome::NoLocation,
            LocationOutcome::Unavailable,
        ] {
            let p = enforcing(vec![Branch::Hfhotel, Branch::Hfville], outcome);
            let (offered, _) = me_branches(&p, &maid("421")).await;
            for branch in offered {
                assert!(
                    require_location(&p, &maid("421"), branch).await.is_ok(),
                    "{outcome:?}: /me offered {branch:?} but the gate refuses it"
                );
            }
        }
    }

    // ---- CR-1: iHOTEL-wins room status ---------------------------------

    /// A room as PG would hand it to us — `clean` is the CANONICAL value the
    /// merge is allowed to overwrite.
    fn pg_room(room_no: &str, clean: bool) -> HkRoom {
        HkRoom {
            room_id: 1,
            room_no: room_no.to_string(),
            floor: Some(1),
            building: None,
            room_clean: clean,
            cleaning: None,
        }
    }

    /// iHOTEL's answer, in CANONICAL polarity (the inversion already applied
    /// by `legacy_room_status::legacy_clean_to_is_clean`).
    fn ihotel(entries: &[(&str, bool)]) -> RoomCleanOutcome {
        RoomCleanOutcome::Available(
            entries
                .iter()
                .map(|(no, clean)| ((*no).to_string(), *clean))
                .collect(),
        )
    }

    /// Rule 1, both directions. This is the whole CR: iHOTEL's value replaces
    /// canonical, whichever way they disagree.
    #[test]
    fn ihotel_wins_over_the_canonical_mirror_in_both_directions() {
        let mut rooms = vec![
            // PG says clean, iHOTEL says dirty (the checked-out-but-unsynced
            // case reception is already looking at).
            pg_room("104", true),
            // PG says dirty, iHOTEL says clean (a stale mirror after someone
            // cleaned it in iHOTEL).
            pg_room("203", false),
        ];
        let stale = merge_legacy_room_clean(
            &mut rooms,
            &ihotel(&[("104", false), ("203", true)]),
            Branch::Hfhotel,
        );
        assert!(!stale, "iHOTEL answered — nothing is stale");
        assert!(!rooms[0].room_clean, "104 must follow iHOTEL (dirty)");
        assert!(rooms[1].room_clean, "203 must follow iHOTEL (clean)");
    }

    /// Agreement is the common case and must be a no-op, not an accidental
    /// flip. Pinned so a future refactor of the polarity can't pass rule 1's
    /// disagreement test by inverting everything.
    #[test]
    fn agreement_leaves_every_room_untouched() {
        let mut rooms = vec![pg_room("104", true), pg_room("203", false)];
        let stale = merge_legacy_room_clean(
            &mut rooms,
            &ihotel(&[("104", true), ("203", false)]),
            Branch::Hfhotel,
        );
        assert!(!stale);
        assert!(rooms[0].room_clean);
        assert!(!rooms[1].room_clean);
    }

    /// Rule 2 — the fallback the owner locked: legacy unreachable ⇒ the maid
    /// still gets her list, with the canonical values UNCHANGED, and the
    /// response is flagged so the client can say so in Thai. There is no
    /// error path here; a dead legacy must never blank the screen.
    #[test]
    fn unavailable_legacy_falls_back_to_pms_and_flags_stale() {
        let mut rooms = vec![pg_room("104", true), pg_room("203", false)];
        let stale =
            merge_legacy_room_clean(&mut rooms, &RoomCleanOutcome::Unavailable, Branch::Hfhotel);
        assert!(stale, "the client MUST be told it is showing the mirror");
        assert!(rooms[0].room_clean, "canonical value is preserved verbatim");
        assert!(!rooms[1].room_clean, "canonical value is preserved verbatim");
    }

    /// A branch with NO reader configured is the same displayable state as an
    /// unreachable one — that is what makes this ship compatible before the
    /// Ville reader exists (and before `HK_BRANCHES` admits Ville).
    #[tokio::test]
    async fn a_branch_without_a_reader_reports_unavailable() {
        let p = policy(vec![Branch::Hfhotel, Branch::Hfville]);
        assert!(p.legacy_room_clean_branches().is_empty());
        assert_eq!(
            resolve_legacy_room_clean(&p, Branch::Hfville).await,
            RoomCleanOutcome::Unavailable
        );
    }

    /// The reader is picked by BRANCH. A Ville maid must never be reconciled
    /// against HF Hotel's legacy server — that is the wrong-hotel bug this
    /// whole surface exists to close, one database deeper.
    #[tokio::test]
    async fn readers_are_resolved_per_branch() {
        #[derive(Debug)]
        struct Fixed(RoomCleanOutcome);
        #[async_trait::async_trait]
        impl RoomCleanSource for Fixed {
            async fn room_clean(&self) -> RoomCleanOutcome {
                self.0.clone()
            }
        }

        let p = policy(vec![Branch::Hfhotel, Branch::Hfville]).with_legacy_room_clean(
            Branch::Hfhotel,
            Arc::new(Fixed(ihotel(&[("104", false)]))),
        );
        assert_eq!(p.legacy_room_clean_branches(), vec!["hfhotel"]);
        assert_eq!(
            resolve_legacy_room_clean(&p, Branch::Hfhotel).await,
            ihotel(&[("104", false)])
        );
        // Ville has no reader — fallback, NOT HF Hotel's answer.
        assert_eq!(
            resolve_legacy_room_clean(&p, Branch::Hfville).await,
            RoomCleanOutcome::Unavailable
        );
    }

    /// A room iHOTEL has no usable value for (unmatched `Room_no`, or a
    /// `Room_Clean` literal the reader refused to interpret) keeps its
    /// canonical value SILENTLY. It must not flag the whole list stale —
    /// a note that fires for one unmatched room trains the maid to ignore it.
    #[test]
    fn a_room_missing_from_ihotel_keeps_its_canonical_value_without_flagging() {
        let mut rooms = vec![pg_room("104", true), pg_room("999", false)];
        let stale =
            merge_legacy_room_clean(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
        assert!(!stale, "one unmatched room is not a staleness event");
        assert!(!rooms[0].room_clean, "104 follows iHOTEL");
        assert!(!rooms[1].room_clean, "999 keeps its canonical value");
    }

    /// Legacy `Room_no` values are `varchar` and padded in places; the join
    /// must survive that or every room silently falls back.
    #[test]
    fn room_numbers_are_matched_trimmed() {
        let mut rooms = vec![pg_room(" 104 ", true)];
        merge_legacy_room_clean(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
        assert!(!rooms[0].room_clean);
    }

    /// An EMPTY answer from iHOTEL is still an answer — nothing is stale, and
    /// every room simply keeps its canonical value. (An empty `HT_Rooms` is
    /// not a thing; this pins that "no rows" can't be mistaken for "outage",
    /// because the two produce different notes on the maid's screen.)
    #[test]
    fn an_empty_ihotel_answer_is_not_stale() {
        let mut rooms = vec![pg_room("104", true)];
        let stale = merge_legacy_room_clean(&mut rooms, &ihotel(&[]), Branch::Hfhotel);
        assert!(!stale);
        assert!(rooms[0].room_clean);
    }

    /// Divergence is an OPERATOR signal. Nothing about it may reach the wire —
    /// the maid has one job per room and no action to take about which
    /// database is behind. Pinned by serializing the response and asserting
    /// the payload carries no divergence-shaped field.
    #[test]
    fn divergence_is_never_serialized_to_the_maid() {
        let mut rooms = vec![pg_room("104", true)];
        let stale =
            merge_legacy_room_clean(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
        let body = serde_json::to_string(&RoomsResponse {
            success: true,
            data: rooms,
            legacy_status_stale: stale,
        })
        .expect("serializes");
        assert!(body.contains("\"roomClean\":false"), "{body}");
        assert!(body.contains("\"legacyStatusStale\":false"), "{body}");
        for leaked in ["diverg", "pmsClean", "ihotel", "mirror"] {
            assert!(
                !body.to_lowercase().contains(leaked),
                "response must not surface divergence ({leaked}): {body}"
            );
        }
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

    // ---- D1 write guard: the WRITE half of iHOTEL-wins ------------------

    /// The hint handed to the service is the SAME per-room answer the display
    /// merge uses — including its polarity. Getting this backwards would make
    /// every mirror-lag repair fire on the wrong pole.
    #[test]
    fn the_write_hint_carries_ihotels_answer_for_that_one_room() {
        let outcome = ihotel(&[("104", false), ("203", true)]);
        assert_eq!(
            legacy_hint_for_room(&outcome, "104"),
            LegacyCleanliness::Dirty
        );
        assert_eq!(
            legacy_hint_for_room(&outcome, "203"),
            LegacyCleanliness::Clean
        );
        // Padded legacy `varchar` room numbers must still join — same trim the
        // display merge does, or the guard silently falls back for every room.
        assert_eq!(
            legacy_hint_for_room(&outcome, " 104 "),
            LegacyCleanliness::Dirty
        );
    }

    /// Everything the display treats as "no usable value" is `Unknown` on the
    /// write path too: an unreachable legacy AND a room iHOTEL had no usable
    /// literal for. Both must degrade to canonical-only judgement — never to a
    /// guess, and never to an error on a maid's tap.
    #[test]
    fn no_usable_legacy_answer_is_unknown_not_a_guess() {
        assert_eq!(
            legacy_hint_for_room(&RoomCleanOutcome::Unavailable, "104"),
            LegacyCleanliness::Unknown
        );
        assert_eq!(
            legacy_hint_for_room(&ihotel(&[("104", false)]), "999"),
            LegacyCleanliness::Unknown
        );
    }

    /// The read is issued ONLY for taps canonical alone would have called a
    /// no-op — the ones D1 is about. A tap that already carries a real
    /// transition must not spend the reader's 3s budget on the maid's request
    /// path, and `started` must never consult legacy at all (it is
    /// legacy-inert by design).
    #[test]
    fn only_would_be_no_op_taps_consult_ihotel() {
        // `done` on a room canonical already calls clean — the D1 case.
        assert!(needs_legacy_opinion(Some(true), Some(true)));
        // `dirty` on a room canonical already calls dirty — the mirror image.
        assert!(needs_legacy_opinion(Some(false), Some(false)));
        // Real transitions: the outcome cannot change, so do not ask.
        assert!(!needs_legacy_opinion(Some(true), Some(false)));
        assert!(!needs_legacy_opinion(Some(false), Some(true)));
        // NULL canonical is a transition in both directions — do not ask.
        assert!(!needs_legacy_opinion(Some(true), None));
        assert!(!needs_legacy_opinion(Some(false), None));
        // `started` never mirrors.
        assert!(!needs_legacy_opinion(None, Some(true)));
        assert!(!needs_legacy_opinion(None, Some(false)));
    }

    /// The status → target-state mapping the guard is built on. `started` must
    /// stay `None`: giving it a target would make a maid's "I have begun" flip
    /// a room's cleanliness and write to iHOTEL.
    #[test]
    fn target_state_per_status_is_pinned() {
        assert_eq!(target_clean_for(CleaningProgressStatus::Done), Some(true));
        assert_eq!(target_clean_for(CleaningProgressStatus::Dirty), Some(false));
        assert_eq!(target_clean_for(CleaningProgressStatus::Started), None);
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
