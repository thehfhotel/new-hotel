//! Maid-facing housekeeping surface (`/api/hk/*`) — employee-login plan
//! Phase 4 (HF-erp `docs/employee-login-plan.md`).
//!
//! Serves the mobile `/hk` pages maids open from their LINE Role Menu:
//!
//! - `GET  /api/hk/me`                          — the verified maid identity.
//! - `GET  /api/hk/rooms`                       — room list + today's progress.
//! - `GET  /api/hk/rooms/{id}`                  — one room + today's events.
//! - `POST /api/hk/rooms/{id}/cleaning`         — report progress (`started`/`done`/`dirty`).
//! - `POST /api/hk/rooms/{id}/linen-shortage`   — report a linen shortage (ขาดผ้า).
//! - `POST /api/hk/rooms/{id}/linen-shortage/resolve` — mark the room restocked (เติมผ้าแล้ว).
//! - `POST /api/hk/rooms/{id}/broken-items`     — RETIRED, answers `410 Gone`.
//! - `GET  /api/hk/broken-items/{id}/photo`     — stream a report's photo.
//!
//! The two `linen-shortage` routes (migrations 088 + 090) are the LEGACY-INERT
//! pair: they write `ht_hk_linen_reports` rows and return. No notification, no
//! domain event, no outbox row, no legacy writeback — iHOTEL has no linen
//! counterpart, so there is nothing to mirror and no dark flag waiting to
//! enable one. They share every gate with the cleaning route (required
//! `?branch=`, the location gate, the Ville-guard exemption) so the maid
//! mutations behave identically.
//!
//! ### Open vs today (migration 090)
//!
//! A linen-shortage report is **OPEN until a maid marks the room restocked**,
//! and completion is ROOM-LEVEL: one `…/resolve` closes every open row for the
//! room, whatever kind and whatever day. So the ขาดผ้า indication now means
//! "this room has OPEN reports" of ANY age — completion SUPERSEDES the old day
//! rollover, matching the visible-until-done convention ADR 0008's room signals
//! already follow. `linenShortageOpen` (list + detail) and `linenShortagesOpen`
//! (detail, summed per kind) are those fields.
//!
//! The day-scoped [`HkRoom::linen_shortage_today`] and
//! [`RoomDetailResponse::linen_shortages`] are **PRESERVED with their exact 088
//! meaning and DEPRECATED**, purely so a cached bundle keeps rendering while
//! the new one rolls out. New clients must badge from the OPEN fields; a room
//! restocked at 09:00 is not ขาดผ้า at 09:01, and a room short since yesterday
//! still is.
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
//! (its own Cloudflare Access application, HF ID silent login, grant keys
//! `housekeeping` / `reception`) which injects the verified [`HkIdentity`] —
//! handlers NEVER trust client-supplied reporter fields; the badge/name
//! stamped into rows come exclusively from the verified assertion. Fail
//! closed: no valid identity ⇒ the middleware already answered 401/403.
//!
//! ## Two roles: the maid and the reception viewer (2026-09)
//!
//! Reception wants the same board the maid works from — "which rooms are clean
//! right now" — and that is a READ of this exact data, so the `reception`
//! grant opens this surface READ-ONLY rather than getting a second copy of it.
//! The middleware resolves the capability once into
//! [`HkIdentity::can_report`](crate::middleware::hk_access::HkIdentity::can_report);
//! nothing here re-parses grants.
//!
//! | endpoint | `housekeeping` | `reception` only |
//! |---|---|---|
//! | `GET /api/hk/me` | `canReport: true` | `canReport: false`, `markDirtyEnabled: false` |
//! | `GET /api/hk/rooms`, `/rooms/{id}`, `/broken-items/{id}/photo` | ✅ | ✅ |
//! | `POST …/cleaning`, `POST …/linen-shortage`, `POST …/linen-shortage/resolve` | ✅ | **403** [`REPORT_NOT_PERMITTED_ERROR`] |
//!
//! [`require_report_capability`] is the enforcement and it runs FIRST in both
//! mutations. `canReport` in `/api/hk/me` is UX only: it lets the client hide
//! controls it cannot use, and a stale bundle that still shows them gets a
//! loud 403 instead of a silent write.
//!
//! ### Location enforcement and the reception viewer
//!
//! **Viewers are EXEMPT from the per-employee location gate below**, in BOTH
//! of its halves ([`require_location`] and [`me_branches`]), and no HF ID
//! lookup is issued for them at all. Three reasons, in order of weight:
//!
//! 1. The gate exists to stop a maid FILING against the wrong property. A
//!    viewer files nothing — [`require_report_capability`] already refused
//!    every mutation before the branch is even parsed — so the failure mode it
//!    guards cannot occur.
//! 2. A receptionist's HF ID `Employee.location` is routinely NULL; nobody
//!    ever needed it. Under enforcement that is [`LocationOutcome::NoLocation`]
//!    ⇒ `403` and a permanently empty board, i.e. the feature would not work
//!    at all for the people it is for.
//! 3. The desk is at the desk. "Which property's rooms may I LOOK at" is
//!    answered by `HK_BRANCHES`, which still binds: every handler runs
//!    [`require_branch`] first, so a viewer gets the properties this
//!    deployment serves and never one it does not — the same outer bound the
//!    `housekeeping_admin` widening cannot cross.
//!
//! Exempting both halves together is what keeps the picker and the per-request
//! gate in agreement, which is the invariant [`me_branches`] documents.
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
//! functions ([`merge_legacy_room_flags`]):
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
//! ## Occupancy rides the SAME read (display-only)
//!
//! [`HkRoom::occupancy`] is iHOTEL's `HT_Rooms.Room_Use` under the identical
//! three rules, merged PER FACT: the same row of the same single `SELECT`
//! carries both `Room_Clean` and `Room_Use`, so occupancy costs no extra
//! round-trip and cannot be stale in a way cleanliness is not. `legacy_status_stale`
//! therefore covers both facts, and a SECOND flag would be a second name for
//! the same outage.
//!
//! The canonical FALLBACK for occupancy is DERIVED per fetch from active
//! checkins ([`rooms_list_sql`]), never the stored `ht_rooms_new.room_status`
//! column — that column is bypassed by check-in/check-out and is months behind
//! reality (issue #200).
//!
//! Nothing writes, gates or decides on occupancy. It exists so a maid can see
//! whether the door she is about to open has a guest behind it.
//!
//! ## Arrivals / departures today (canonical-only)
//!
//! [`HkRoom::expected_arrival`] and [`HkRoom::expected_departure`] are the
//! maid's planning tags: a booking whose stay starts today and has not become
//! a check-in yet, and an active checkin due out today or earlier. Both are
//! derived per fetch from canonical PG with NO legacy counterpart in the CR-1
//! read — iHOTEL has no equivalent per-room flag — so they are deliberately
//! NOT covered by `legacy_status_stale` and stay live through a legacy outage.
//!
//! Both use the Bangkok civil day ([`TODAY_BKK_DATE`]), never `CURRENT_DATE`.
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
//! resolved by the SAME branch through [`HkPolicy::legacy_room_flags`], so a
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
    response::{
        sse::{Event, Sse},
        Response,
    },
    routing::{get, post},
    Extension, Json, Router,
};
use futures_util::Stream;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::config::HfidLocationConfig;
use crate::db::PgPool;
use crate::domain::hk_signal::{RoomCheckOutcome, RoomSignal, SignalAction, SignalRole};
use crate::error::{ApiError, ApiResult};
use crate::hfid_location::{EmployeeLocation, HfidLocationClient, LocationOutcome};
use crate::legacy_room_status::{RoomFlagsOutcome, RoomFlagsSource};
use crate::middleware::hk_access::HkIdentity;
use crate::outbox::event::EventSource;
use crate::service::hk_signals::{
    ActOnSignalCommand, AnswerRoomCheckCommand, HkSignalService, RaiseSignalCommand,
};
use crate::service::housekeeping::{
    CleaningProgressStatus, HousekeepingService, LegacyCleanliness, LinenShortageItem,
    ReportCleaningCommand, ReportLinenShortageCommand, ResolveLinenShortageCommand, MAX_LINEN_QTY,
    MIN_LINEN_QTY,
};

/// Cleaning-progress statuses a maid can report. `started` =
/// เริ่มทำความสะอาด, `done` = เสร็จแล้ว, `dirty` = ห้องยังไม่สะอาด. Matches the
/// CHECK constraint on `ht_hk_cleaning_events.hkev_status` (migration 077,
/// widened by migration 087) — keep the two in lock-step.
pub const VALID_CLEANING_STATUSES: [&str; 3] = ["started", "done", "dirty"];

/// Linen kinds a maid can report short (ขาดผ้า) — migration 088.
///
/// **This constant is the ONLY allowlist.** `ht_hk_linen_reports.hklr_kind` is
/// deliberately plain `TEXT` with no CHECK, so widening the vocabulary is a
/// one-line change here plus the frontend's label — no migration, no `ALTER` on
/// a live table at two sites, and no window where the deployed binary and the
/// deployed constraint disagree about the valid set. That is exactly the
/// coupling migration 087 had to unpick for `hkev_status`; this table does not
/// inherit it. `bed_sheet` (ผ้าปูที่นอน) was added exactly that way — code
/// only, migration 088 untouched.
///
/// ORDER IS THE DISPLAY ORDER, mirrored by the frontend and by the room
/// detail's `linenShortages` aggregation: bed linen largest-first, then the
/// towels. Reordering this array reorders both surfaces.
///
/// Thai labels belong to the client (which knows how to render them), so the
/// wire codes stay ASCII snake_case and are compared after normalisation.
pub const VALID_LINEN_KINDS: [&str; 6] = [
    "bed_sheet",   // ผ้าปูที่นอน
    "pillowcase",  // ปลอกหมอน
    "duvet_cover", // ปลอกผ้านวม
    "bath_towel",  // ผ้าเช็ดตัว
    "face_towel",  // ผ้าเช็ดหน้า
    "foot_towel",  // ผ้าเช็ดเท้า
];

/// Most kinds one linen-shortage submission may carry.
///
/// Equal to `VALID_LINEN_KINDS.len()` today, and that is not a coincidence: a
/// submission may name each kind at most once, so a body with more entries than
/// there are kinds is necessarily malformed regardless of the duplicate check.
/// Kept as its own constant so widening the vocabulary later does not silently
/// widen the accepted body size before anyone has thought about it.
pub const MAX_LINEN_ITEMS: usize = 6;

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

/// 403 body when a READ-ONLY viewer (the `reception` grant without
/// `housekeeping` — see [`crate::middleware::hk_access::HkRole`]) posts to one
/// of the two maid mutations.
///
/// Thai, and in [`MARK_DIRTY_DISABLED_ERROR`]'s class rather than
/// [`BRANCH_NOT_ENABLED_ERROR`]'s: a real person at the desk reads it, and they
/// reach it only from a stale bundle (the viewer UI renders no report controls)
/// or a hand-rolled request. So it says what the account CAN do and who to ask.
pub const REPORT_NOT_PERMITTED_ERROR: &str =
    "บัญชีนี้ดูสถานะห้องได้อย่างเดียว ไม่สามารถส่งรายงานได้ กรุณาติดต่อผู้ดูแลระบบ";

/// 403 body when the requested branch is not the employee's own location.
/// ACTIONABLE: it names what to do (pick your own branch) and who to ask.
pub const LOCATION_MISMATCH_ERROR: &str =
    "สาขาที่เลือกไม่ตรงกับสาขาที่คุณสังกัด กรุณาเลือกสาขาของคุณ หรือติดต่อผู้ดูแลระบบ";

/// 403 body when HF ID has no usable location for this employee — null
/// `location`, an unknown badge, or an inactive/pending employee. The fix is
/// an admin action, so that is what the message asks for.
pub const LOCATION_UNKNOWN_ERROR: &str = "ยังไม่ได้กำหนดสาขาของพนักงาน — ติดต่อผู้ดูแลระบบ";

/// 503 body when the requested branch has no live event fan-out, so the maid
/// signal stream (`GET /api/hk/events`) cannot be served for it.
///
/// A 503 with a retryable Thai message, NOT a silent fall back to the other
/// property's stream: [`crate::routes::events::hk_signal_receiver`] refuses
/// that fallback on purpose (see its docs). Reception's board may degrade to
/// HF Hotel; a Ville maid's phone may not.
pub const SIGNAL_STREAM_UNAVAILABLE_ERROR: &str =
    "ระบบแจ้งเตือนสดของสาขานี้ขัดข้องชั่วคราว กรุณาลองใหม่อีกครั้ง";

/// 400 body for an unrecognised `outcome` on the ขอเช็คห้อง answer.
pub const OUTCOME_INVALID_ERROR: &str = "invalid outcome (expected 'clear' or 'problems')";

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
    pub legacy_room_flags: BTreeMap<&'static str, Arc<dyn RoomFlagsSource>>,
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
            // Populated by `main.rs` via `with_legacy_room_flags` — it owns
            // the legacy pools (and is the ONE place that already has HF
            // Hotel's). Empty here means the fallback path, which is a
            // correct, shippable state.
            legacy_room_flags: BTreeMap::new(),
        }
    }

    /// Attach a branch's iHOTEL room-status reader. Called by `main.rs` once
    /// per branch that has one; branches without a reader use the fallback.
    pub fn with_legacy_room_flags(
        mut self,
        branch: Branch,
        source: Arc<dyn RoomFlagsSource>,
    ) -> Self {
        self.legacy_room_flags.insert(branch_id(branch), source);
        self
    }

    /// Branch ids that have a live iHOTEL reader — for the startup log, so an
    /// operator can see at a glance whether `/hk` is serving iHOTEL truth or
    /// the canonical mirror.
    pub fn legacy_room_flags_branches(&self) -> Vec<&'static str> {
        self.legacy_room_flags.keys().copied().collect()
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
            legacy_room_flags: BTreeMap::new(),
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

/// The report-capability gate: the ONE thing that separates a maid from a
/// read-only reception viewer on this surface. PURE — unit-tested below.
///
/// It reads the boolean the middleware already resolved
/// ([`HkIdentity::can_report`]); it deliberately does NOT re-derive the role
/// from grants, so there is exactly one place in the codebase where an HF ID
/// grant becomes a permission and no way for the two to drift.
///
/// Both mutations call it FIRST — ahead of [`require_branch`], and therefore
/// ahead of [`require_location`]:
///
/// * "you may not report anywhere" outranks "that branch is not offered" and
///   "that status is misspelled", the same reasoning that puts the location
///   gate ahead of status validation.
/// * a refused viewer never triggers the HF ID location lookup, so a read-only
///   badge cannot cost a network round-trip per rejected write.
///
/// A `housekeeping` identity has `can_report == true`, so for every maid this
/// function is a no-op and the pre-existing rejection order is unchanged.
fn require_report_capability(identity: &HkIdentity) -> ApiResult<()> {
    if identity.can_report {
        return Ok(());
    }
    tracing::warn!(
        badge = %identity.badge,
        "/hk refused a report from a read-only (reception) identity"
    );
    Err(ApiError::Forbidden(REPORT_NOT_PERMITTED_ERROR.to_string()))
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
///
/// **Read-only viewers are EXEMPT** — see the module doc's "Location
/// enforcement and the reception viewer". The gate exists to stop a maid
/// filing against the wrong property; a viewer files nothing, and a
/// receptionist's `Employee.location` is very often NULL (nobody ever needed
/// it), which under enforcement would answer `403 LOCATION_UNKNOWN_ERROR` and
/// leave the desk with a permanently empty board. The exemption is checked
/// BEFORE the flag so no lookup is issued for a viewer at all.
async fn require_location(
    policy: &HkPolicy,
    identity: &HkIdentity,
    branch: Branch,
) -> ApiResult<()> {
    if !identity.can_report {
        return Ok(());
    }
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

/// Build an [`HkSignalService`] bound to the branch's pool — same construction
/// shape as [`service_for`], through the SAME `write_pool` chokepoint, so a
/// `branch=hfville` signal can never land in the HF Hotel database.
fn signal_service_for(state: &AppState, branch: Branch) -> ApiResult<HkSignalService> {
    Ok(HkSignalService::new(state.write_pool(Some(branch))?.clone()))
}

/// Which side of the ADR 0008 conversation this `/hk` identity is on.
///
/// The SINGLE derivation on this surface, and it reads the SAME boolean
/// [`require_report_capability`] reads — `can_report`, resolved once in
/// `middleware::hk_access`. A `housekeeping` grant is the maid; a `reception`-
/// only viewer IS the desk here (that is the whole point of the viewer role —
/// reception works the same board), so it sends desk→maid signals and acts on
/// maid→desk ones. PURE.
fn hk_role(identity: &HkIdentity) -> SignalRole {
    SignalRole::from_can_report(identity.can_report)
}

/// Normalize + validate the ขอเช็คห้อง answer's `outcome`. Trim + lower-case,
/// the same forgiveness [`parse_cleaning_status`] grants. PURE.
fn parse_outcome(raw: &str) -> ApiResult<RoomCheckOutcome> {
    RoomCheckOutcome::parse(raw.trim().to_lowercase().as_str())
        .ok_or_else(|| ApiError::BadRequest(format!("{OUTCOME_INVALID_ERROR}, got '{raw}'")))
}

/// May this identity raise this signal type? The route-level pre-check.
///
/// The service asserts the SAME rule (it must hold for any caller), so this is
/// not the enforcement point — it is the ORDERING point. Without it the room
/// probe would run first, and a misspelled type or a wrong-direction type would
/// be answered with a database round-trip, or a `500` when the pool is
/// unreachable, instead of the 400/403 the client can act on. It puts body
/// validation ahead of room existence, exactly as [`report_linen_shortage`]
/// documents (`… → body validation (400) → room existence (404)`).
///
/// PURE; the verdicts themselves are `domain::hk_signal`'s.
fn require_signal_type(identity: &HkIdentity, raw: &str) -> ApiResult<()> {
    let normalized = crate::domain::hk_signal::normalize_type(raw);
    crate::domain::hk_signal::direction_for_role_type(hk_role(identity), &normalized)
        .map(|_| ())
        .map_err(signal_rule_error)
}

/// A domain rule refusal → the HTTP error, honouring the domain's own
/// 403-vs-400 split so a route can never re-decide it.
fn signal_rule_error(err: crate::domain::hk_signal::SignalRuleError) -> ApiError {
    let message = err.message();
    if err.is_forbidden() {
        ApiError::Forbidden(message)
    } else {
        ApiError::BadRequest(message)
    }
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
            "/api/hk/rooms/{room_id}/linen-shortage",
            post(report_linen_shortage),
        )
        // เติมผ้าแล้ว (migration 090). A SUB-path of the report route on
        // purpose: completing a shortage is the same object's other half, not a
        // new noun. It is the only SIX-segment write on this surface, which is
        // why `middleware::ville_guard`'s matcher had to gain one optional
        // trailing segment (matched against a closed pair list, never a prefix).
        .route(
            "/api/hk/rooms/{room_id}/linen-shortage/resolve",
            post(resolve_linen_shortage),
        )
        .route(
            "/api/hk/rooms/{room_id}/broken-items",
            post(report_broken_item),
        )
        .route(
            "/api/hk/broken-items/{report_id}/photo",
            get(broken_item_photo),
        )
        // Room signals (ADR 0008). The two shapes are deliberately different:
        // raising a signal is ABOUT a room, so it hangs off `/rooms/{id}`
        // alongside the other maid reports; acting on one addresses the signal
        // itself, which already knows its room — repeating the room id in the
        // path would just be a second thing that can disagree with the row.
        // `middleware::ville_guard` exempts BOTH shapes (they are PG-only).
        .route("/api/hk/signals", get(list_signals))
        .route("/api/hk/rooms/{room_id}/signals", post(raise_signal))
        .route("/api/hk/signals/{signal_id}/ack", post(ack_signal))
        .route("/api/hk/signals/{signal_id}/done", post(done_signal))
        .route("/api/hk/signals/{signal_id}/cancel", post(cancel_signal))
        .route("/api/hk/signals/{signal_id}/answer", post(answer_signal))
        // The maid's own live stream. Behind the SAME gates as every other
        // room endpoint (required `?branch=`, the location gate) — a stream is
        // a read of this branch's data, and showing a maid the other
        // property's signals is the wrong-hotel bug one tap earlier.
        .route("/api/hk/events", get(signal_events))
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
    /// `HK_MARK_DIRTY_ENABLED` **AND** [`Self::can_report`]. `false` ⇒ the
    /// client hides the "แจ้งห้องไม่สะอาด" button rather than offering a dead
    /// tap.
    ///
    /// The conjunction is not redundant: mark-dirty is a REPORT, so offering
    /// it to a read-only viewer would be a dead tap even with the flag on.
    /// A viewer therefore always reads `false` here, whatever the env says.
    pub mark_dirty_enabled: bool,
    /// May this identity file reports at all — `true` for the `housekeeping`
    /// grant, `false` for a `reception`-only viewer
    /// ([`crate::middleware::hk_access::HkIdentity::can_report`]).
    ///
    /// `false` ⇒ the client renders the room board WITHOUT the cleaning and
    /// linen-shortage controls. That is UX only: both `POST` handlers refuse a
    /// viewer server-side ([`REPORT_NOT_PERMITTED_ERROR`]), so a stale bundle
    /// fails loudly rather than writing.
    ///
    /// ADDITIVE and always serialized. An older backend never served the key
    /// because it only ever admitted maids, so a client MUST read a MISSING
    /// `canReport` as `true`.
    pub can_report: bool,
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

/// Whether a guest is in the room right now. Display-only — nothing on this
/// surface writes, gates or decides on it.
///
/// A two-variant enum rather than a bare `bool` because the wire value is read
/// by a human-facing card: `"occupied"` / `"vacant"` cannot be misread the way
/// `occupied: false` can be, and a future third state (iHOTEL has none today)
/// would be additive rather than a breaking type change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Occupancy {
    Occupied,
    Vacant,
}

impl Occupancy {
    /// `true` ⇒ [`Occupancy::Occupied`]. The one place the bool→enum mapping
    /// lives, so the SQL column, the legacy flag and the tests cannot drift
    /// into three spellings of it.
    fn from_occupied(occupied: bool) -> Self {
        if occupied {
            Self::Occupied
        } else {
            Self::Vacant
        }
    }
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
    /// this particular room. See [`merge_legacy_room_flags`].
    pub room_clean: bool,
    /// Whether a guest is in the room, as **iHOTEL** reports it (CR-1).
    ///
    /// Sourced from legacy `HT_Rooms.Room_Use` (NOT inverted: `'yes'` =
    /// occupied) whenever iHOTEL answers for this `room_no`. Falls back to the
    /// canonical DERIVED occupancy — `occupied_pms`, computed from ACTIVE
    /// CHECKINS in [`fetch_rooms`] / [`fetch_room`] — when the legacy read is
    /// unavailable (see the response's `legacy_status_stale`) or when iHOTEL
    /// has no usable `Room_Use` for this particular room.
    ///
    /// NEVER the stored `ht_rooms_new.room_status` column: that column is
    /// bypassed by check-in/check-out and is not kept in sync (issue #200,
    /// same defect `routes::new_rooms::LIVE_ROOM_FLAGS_SQL` was introduced to
    /// route around). Reading it here would put a room's occupancy months
    /// behind the guest standing in it.
    ///
    /// Display-only: no writeback, no guard, no decision reads this.
    pub occupancy: Occupancy,
    /// A canonical booking (CT mirror of `HT_Book_H`/`HT_Book_Date` via
    /// `ht_bookings`/`ht_booking_rooms`) assigned to this room, stay starting
    /// TODAY (Bangkok civil day), still `'confirmed'`/`'pending'`.
    ///
    /// Dies when the CT mirror flips the booking to `'checked_in'` /
    /// `'cancelled'`, or at Bangkok midnight — pure per-fetch derivation, no
    /// stored state and nothing to clean up.
    ///
    /// Canonical-side fact: it never consults iHOTEL, so it is NOT covered by
    /// `legacy_status_stale` and stays truthful during a legacy outage.
    pub expected_arrival: bool,
    /// Active checkin on this room due out TODAY **or earlier** — Bangkok
    /// civil day, no morning hour-gate.
    ///
    /// Dies with the active folio / room line: after a real checkout the card
    /// shows vacant + dirty, which is the existing "just left" reading.
    ///
    /// Canonical-side fact, exactly like [`Self::expected_arrival`]: NOT
    /// covered by `legacy_status_stale`.
    pub expected_departure: bool,
    /// Today's latest maid-reported progress; `None` = nothing reported yet.
    pub cleaning: Option<CleaningProgress>,
    /// **DEPRECATED (migration 090) — use [`Self::linen_shortage_open`].**
    ///
    /// At least one `ht_hk_linen_reports` row was filed for this room TODAY
    /// (Bangkok civil day, [`TODAY_BKK_LINEN`]) — migration 088, and this field
    /// keeps that EXACT meaning: it still ignores whether the shortage was
    /// resolved, and it still dies at Bangkok midnight.
    ///
    /// Preserved only so a cached bundle built before 090 keeps rendering
    /// something sensible during the rollout. It is the WRONG badge now: a room
    /// restocked at 09:00 still reads `true` all day, and a room short since
    /// yesterday reads `false`. Remove it once no deployed client reads it.
    ///
    /// Canonical-side fact exactly like [`Self::expected_arrival`]: it never
    /// consults iHOTEL (which has no linen counterpart at all), so it is NOT
    /// covered by `legacy_status_stale`.
    pub linen_shortage_today: bool,
    /// This room has at least one OPEN linen-shortage report — any age,
    /// `hklr_resolved_at IS NULL` (migration 090). **This is the ขาดผ้า badge.**
    ///
    /// A FLAG, not a count: the list only needs to badge the card, and the
    /// per-kind totals live on the detail card
    /// ([`RoomDetailResponse::linen_shortages_open`]).
    ///
    /// Cleared by a maid's เติมผ้าแล้ว ([`resolve_linen_shortage`]) and by
    /// nothing else — NOT by midnight, which is the whole correction 090 makes:
    /// a shortage is visible until it is done, whatever the day (the same
    /// convention ADR 0008's room signals follow). Still a pure per-fetch
    /// derivation with no stored flag to keep in step.
    ///
    /// Canonical-side fact, NOT covered by `legacy_status_stale`, for
    /// [`Self::linen_shortage_today`]'s reason.
    pub linen_shortage_open: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomsResponse {
    pub success: bool,
    pub data: Vec<HkRoom>,
    /// `true` when the iHOTEL read could not answer and every `roomClean` AND
    /// `occupancy` above is therefore the canonical fallback rather than
    /// iHOTEL truth (CR-1 rule 2). The client renders a visible Thai note; it
    /// must never render an error page — a stale list is usable, a blank one
    /// is not.
    ///
    /// ONE flag for BOTH facts, deliberately: they come from the same row of
    /// the same single `SELECT` ([`crate::legacy_room_status::ROOM_STATUS_SQL`]),
    /// so a second flag could never disagree with this one — it would only be
    /// a second name for the same outage.
    ///
    /// Note what it does NOT cover: `expectedArrival` / `expectedDeparture`
    /// are canonical-side derivations that never consult iHOTEL, so they stay
    /// live and truthful while this flag is `true`.
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
    /// **DEPRECATED (migration 090) — use [`Self::linen_shortages_open`].**
    ///
    /// Today's linen shortages for this room, summed per kind — migration 088,
    /// with its EXACT original meaning: the Bangkok civil day, resolved rows
    /// included. Preserved only for bundle skew during the rollout, exactly as
    /// [`HkRoom::linen_shortage_today`] is.
    ///
    /// One entry per kind actually reported (never a zero row), ordered by
    /// [`VALID_LINEN_KINDS`]; `[]` when nothing was reported today. The list's
    /// [`HkRoom::linen_shortage_today`] is exactly `!is_empty()` of this — same
    /// table, same Thai day, so the old badge and the old breakdown can never
    /// disagree with each other.
    ///
    /// ADDITIVE and always serialized, so a client branches on the VALUE, not
    /// on whether the key exists.
    pub linen_shortages: Vec<LinenShortageTotal>,
    /// This room's OPEN linen shortages, summed per kind — migration 090.
    /// **This is the breakdown behind the ขาดผ้า badge.**
    ///
    /// Same shape and same ordering rule as [`Self::linen_shortages`]
    /// (`[{"kind","qty"}]` in [`VALID_LINEN_KINDS`] order, `[]` when none, wire
    /// codes only), scoped by `hklr_resolved_at IS NULL` instead of by the
    /// Thai day — so it spans however long the shortage has been standing, and
    /// a maid's เติมผ้าแล้ว empties it in one tap.
    ///
    /// [`HkRoom::linen_shortage_open`] is exactly `!is_empty()` of this: same
    /// table, same predicate, so the badge and the breakdown cannot disagree.
    ///
    /// ADDITIVE and always serialized, same rule as every field above.
    pub linen_shortages_open: Vec<LinenShortageTotal>,
    /// Same meaning as [`RoomsResponse::legacy_status_stale`]. Carried here
    /// too so the two screens can never tell the maid different stories about
    /// the same room.
    pub legacy_status_stale: bool,
}

/// One linen kind reported short for a room today, summed across that day's
/// submissions — an entry of [`RoomDetailResponse::linen_shortages`].
///
/// Wire codes only (`{"kind":"bath_towel","qty":5}`): the Thai labels belong to
/// the client, same rule as [`VALID_LINEN_KINDS`] itself.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinenShortageTotal {
    /// A [`VALID_LINEN_KINDS`] code, as stored (already normalised on write).
    pub kind: String,
    /// Pieces missing, SUMmed across today's submissions for this room. Each
    /// row is 1..=20, but a maid may submit more than once in a day, so this
    /// total is deliberately not bounded by [`MAX_LINEN_QTY`].
    pub qty: i64,
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

/// Body for `POST /api/hk/rooms/{id}/linen-shortage` — migration 088.
///
/// `items` is an `Option` on purpose. A body of `{}` (or `{"items":null}`) must
/// answer with THIS module's `{success:false,error}` envelope, and a required
/// field would instead be rejected by serde inside axum's `Json` extractor,
/// which renders its own plain-text shape the maid's client cannot parse.
/// Making it optional moves the rejection into [`parse_linen_items`], where the
/// envelope is ours.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportLinenShortageBody {
    pub items: Option<Vec<LinenShortageEntry>>,
}

/// One `{kind, qty}` line as it arrives on the wire.
///
/// `qty` is a raw [`serde_json::Value`], NOT an `i32`, for the same reason
/// `items` is optional: `{"qty": 1.5}`, `{"qty": "3"}` and `{"qty": null}` are
/// all things a hand-rolled or half-broken client sends, and each of them
/// would otherwise be a serde rejection with a foreign body shape. Taking the
/// value untyped lets [`parse_linen_items`] answer every one of them with the
/// repo envelope and a message that names the offending kind.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinenShortageEntry {
    pub kind: String,
    pub qty: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportLinenShortageResponse {
    pub success: bool,
    pub room_id: i32,
    /// How many item ROWS were written — one per kind in the submission.
    ///
    /// Deliberately not a total of the quantities: it answers "did all my lines
    /// land", which is what a client that just POSTed a list needs to know.
    pub reported: usize,
}

/// Response for `POST /api/hk/rooms/{id}/linen-shortage/resolve` — migration
/// 090. There is no request body to document: the command is "this room is
/// restocked", which the path already says in full.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveLinenShortageResponse {
    pub success: bool,
    pub room_id: i32,
    /// How many previously-OPEN rows this tap closed, across every kind.
    ///
    /// **`0` is a SUCCESS, not a miss.** The room has no open shortage, which
    /// is the state the maid asked for — a double tap, or a second maid
    /// arriving behind the first, lands here. A client must render this as
    /// done, never as an error.
    pub resolved: usize,
}

/// Body for `POST /api/hk/rooms/{id}/signals` — ADR 0008.
///
/// `{type}` and nothing else. There is deliberately no `direction` field (the
/// server derives it from the role), no `note` field (canned-only, ADR 0008
/// §Alternatives), and no reporter field (identity comes from the verified
/// assertion, same rule as every other handler here).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiseSignalBody {
    /// One of the canned codes in `domain::hk_signal` — mirroring
    /// `app/hk/signal-vocab.ts`. Trimmed + lower-cased before matching.
    #[serde(rename = "type")]
    pub signal_type: String,
}

/// Body for `POST /api/hk/signals/{id}/answer`.
///
/// `problems` is optional so `{"outcome":"clear"}` is a complete body, and so a
/// `problems` answer that forgot the list is refused by [`parse_outcome`]'s
/// sibling in the service (with this module's envelope) rather than by serde
/// inside axum's `Json` extractor, which renders a plain-text shape the maid's
/// client cannot parse. Same reasoning as
/// [`ReportLinenShortageBody::items`](ReportLinenShortageBody).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerSignalBody {
    /// `clear` | `problems`.
    pub outcome: String,
    /// One or both of `item_missing` / `item_damaged`; required (non-empty)
    /// when `outcome` is `problems`, and must be absent/empty for `clear`.
    pub problems: Option<Vec<String>>,
}

/// `GET /api/hk/signals` — this branch's live board.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalListResponse {
    pub success: bool,
    /// `open` + `acked`, oldest first. The DTO is spelled in
    /// `domain::hk_signal::RoomSignal`, byte-for-byte the `RoomSignal`
    /// interface in `app/hk/signal-vocab.ts`.
    pub signals: Vec<RoomSignal>,
}

/// The single-signal envelope shared by create / ack / done / cancel.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalResponse {
    pub success: bool,
    pub signal: RoomSignal,
}

/// `POST /api/hk/signals/{id}/answer` — the completed check plus whatever it
/// spawned.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerSignalResponse {
    pub success: bool,
    /// The room_check, now `done` with `outcome` and
    /// `doneSource: "room_check_answer"`.
    pub signal: RoomSignal,
    /// The standing guest-accountability signals this answer raised — one per
    /// problem, each with `parentId` = the check. `[]` for a `clear` answer.
    pub spawned: Vec<RoomSignal>,
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

/// Normalize + validate a whole linen-shortage submission — migration 088.
/// PURE, and the single place the wire contract is enforced; unit-tested below.
///
/// Every rejection is an [`ApiError::BadRequest`], so every one of them reaches
/// the maid as the repo's `{success:false,error}` envelope with a 400.
///
/// Order, and why: **shape before content**. The list must exist and be a
/// plausible size before any single entry is judged, so an empty or oversized
/// body is never answered with a complaint about its first item. Within the
/// loop the kind is resolved first (an unknown kind makes the rest of the line
/// meaningless), then checked for a duplicate — the duplicate test necessarily
/// runs on the NORMALIZED code, otherwise `"Bath_Towel"` and `"bath_towel"`
/// would slip past as two lines and land as two rows for one kind — and only
/// then is the quantity read.
///
/// Kinds are trimmed and lower-cased before matching, the same forgiveness
/// [`parse_cleaning_status`] already grants a status. The normalized code is
/// what gets stored, so the table never accumulates casing variants of one kind.
fn parse_linen_items(
    items: Option<Vec<LinenShortageEntry>>,
) -> Result<Vec<LinenShortageItem>, ApiError> {
    let items = items.unwrap_or_default();
    if items.is_empty() {
        return Err(ApiError::BadRequest(
            "items is required and must contain at least one linen entry".to_string(),
        ));
    }
    if items.len() > MAX_LINEN_ITEMS {
        return Err(ApiError::BadRequest(format!(
            "too many linen entries ({}); at most {MAX_LINEN_ITEMS} are accepted",
            items.len()
        )));
    }

    let mut parsed: Vec<LinenShortageItem> = Vec::with_capacity(items.len());
    for entry in items {
        let wanted = entry.kind.trim().to_lowercase();
        let kind = VALID_LINEN_KINDS
            .iter()
            .find(|valid| ***valid == *wanted)
            .ok_or_else(|| {
                ApiError::BadRequest(format!(
                    "invalid linen kind '{}' (expected one of {VALID_LINEN_KINDS:?})",
                    entry.kind
                ))
            })?;

        if parsed.iter().any(|item| item.kind == *kind) {
            return Err(ApiError::BadRequest(format!(
                "duplicate linen kind '{kind}'; report each kind once with its total"
            )));
        }

        // `as_i64` is the whole integer test: it says None for `1.5`, for
        // `"3"`, for `null` and for `true`, and Some only for a JSON integer.
        // The i32 narrowing then rejects anything beyond i32 before the range
        // check, so a 2^40 quantity cannot wrap into an accepted value.
        let qty = entry
            .qty
            .as_i64()
            .and_then(|n| i32::try_from(n).ok())
            .filter(|n| (MIN_LINEN_QTY..=MAX_LINEN_QTY).contains(n))
            .ok_or_else(|| {
                ApiError::BadRequest(format!(
                    "invalid qty {} for linen kind '{kind}' (expected an integer \
                     {MIN_LINEN_QTY}..={MAX_LINEN_QTY})",
                    entry.qty
                ))
            })?;

        parsed.push(LinenShortageItem {
            kind: (*kind).to_string(),
            qty,
        });
    }

    Ok(parsed)
}

// ============================================================================
// iHOTEL-wins merge (CR-1) — pure, unit-tested below
// ============================================================================

/// Overwrite each room's `room_clean` AND `occupancy` with iHOTEL's answer,
/// and report whether the maid is looking at a fallback.
///
/// Returns `true` when the caller must set `legacy_status_stale` — i.e. the
/// legacy read did not answer at all and every value left in `rooms` is the
/// canonical fallback (the PG `room_clean` mirror, and occupancy DERIVED from
/// active checkins).
///
/// The three CR-1 rules, in one place (see the module docs), applied **PER
/// FACT**:
///
/// * iHOTEL wins per room per fact it has a usable value for;
/// * a fact iHOTEL has no usable value for keeps its canonical value SILENTLY
///   — that is a mapping gap, not a staleness event, and flagging the whole
///   list for one unmatched room would train the maid to ignore the note. A
///   room with a readable `Room_Use` but a junk `Room_Clean` therefore takes
///   iHOTEL's occupancy and keeps canonical cleanliness;
/// * every disagreement is logged at `warn` with `room_no` and BOTH values,
///   and NONE of it reaches the response.
///
/// `expected_arrival` / `expected_departure` are NOT touched: they are
/// canonical-side derivations with no legacy counterpart in this read, and
/// they stay live even when everything else here falls back.
///
/// PURE apart from the log line, which is exactly why the divergence rule is
/// testable without a database or a legacy server.
pub(crate) fn merge_legacy_room_flags(
    rooms: &mut [HkRoom],
    outcome: &RoomFlagsOutcome,
    branch: Branch,
) -> bool {
    let legacy = match outcome {
        RoomFlagsOutcome::Available(map) => map,
        // Rule 2. Values stay as fetched from PG; the caller tells the client.
        RoomFlagsOutcome::Unavailable => return true,
    };

    // Counted SEPARATELY: the two facts drift for different reasons (a lagging
    // `Room_Clean` mirror vs. a check-in one side has not seen), and one
    // summary number would hide which engine is behind.
    let mut clean_divergences = 0usize;
    let mut occupancy_divergences = 0usize;

    for room in rooms.iter_mut() {
        let Some(flags) = legacy.get(room.room_no.trim()) else {
            continue;
        };

        if let Some(legacy_clean) = flags.is_clean {
            if legacy_clean != room.room_clean {
                clean_divergences += 1;
                // Rule 3 — operator-facing ONLY. `room_no` plus both values is
                // everything needed to chase it into the CT watcher or the
                // legacy-key repair (`bin/repair_room_legacy_keys`), which is
                // the known cause of this class at HF Ville.
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

        if let Some(legacy_occupied) = flags.occupied {
            let legacy_occupancy = Occupancy::from_occupied(legacy_occupied);
            if legacy_occupancy != room.occupancy {
                occupancy_divergences += 1;
                // Same operator signal, different engine: an occupancy
                // disagreement means one side has a check-in or check-out the
                // other has not applied yet.
                tracing::warn!(
                    branch = branch_id(branch),
                    room_no = %room.room_no,
                    ihotel_occupied = legacy_occupied,
                    pms_occupied = room.occupancy == Occupancy::Occupied,
                    "/hk occupancy divergence: iHOTEL Room_Use and the canonical \
                     derived occupancy disagree — showing iHOTEL (CR-1)"
                );
            }
            // Rule 1.
            room.occupancy = legacy_occupancy;
        }
    }

    if clean_divergences > 0 || occupancy_divergences > 0 {
        tracing::warn!(
            branch = branch_id(branch),
            clean_divergences,
            occupancy_divergences,
            rooms = rooms.len(),
            "/hk served iHOTEL room status over a diverging canonical fallback"
        );
    }
    false
}

/// Ask this branch's iHOTEL reader, or report [`RoomFlagsOutcome::Unavailable`]
/// when the branch has none configured.
///
/// "No reader" and "reader failed" are the SAME answer on purpose: both mean
/// the maid is about to see the canonical mirror, and she must be told so
/// either way. Collapsing them keeps the fallback path single — the one that
/// ships today, and the one an operator can reason about at 6am.
async fn resolve_legacy_room_flags(policy: &HkPolicy, branch: Branch) -> RoomFlagsOutcome {
    match policy.legacy_room_flags.get(branch_id(branch)) {
        Some(source) => source.room_flags().await,
        None => {
            tracing::debug!(
                branch = branch_id(branch),
                "/hk has no iHOTEL room-status reader for this branch — \
                 serving the canonical PG mirror with the stale note"
            );
            RoomFlagsOutcome::Unavailable
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
/// The same three-way answer [`merge_legacy_room_flags`] applies to the display,
/// reduced to the single room a tap is about: a room ABSENT from the legacy
/// answer (unmatched `Room_no`, unrecognised `Room_Clean` literal) is
/// [`LegacyCleanliness::Unknown`], never guessed — exactly as the display keeps
/// such a room's canonical value rather than inventing one. `Unavailable` is
/// Unknown too, which is what makes a legacy outage degrade the write path to
/// its pre-D1 behaviour instead of blocking or failing a maid's tap.
pub(crate) fn legacy_hint_for_room(outcome: &RoomFlagsOutcome, room_no: &str) -> LegacyCleanliness {
    match outcome {
        // Only the CLEANLINESS fact. Occupancy is display-only and has no say
        // in whether a maid's tap earns a writeback — an unrecognised
        // `Room_Clean` under a perfectly readable `Room_Use` is still Unknown
        // here, exactly as it was before the read widened.
        RoomFlagsOutcome::Available(map) => match map.get(room_no.trim()).and_then(|f| f.is_clean) {
            Some(true) => LegacyCleanliness::Clean,
            Some(false) => LegacyCleanliness::Dirty,
            None => LegacyCleanliness::Unknown,
        },
        RoomFlagsOutcome::Unavailable => LegacyCleanliness::Unknown,
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

/// Today's civil DATE in Bangkok — the right-hand side of [`TODAY_BKK`], reused
/// on its own by the arrival / departure predicates.
///
/// `CURRENT_DATE` is BANNED in these predicates: it is the SERVER's date, and
/// the server is not guaranteed to run on Thai wall-clock. Between 17:00 and
/// 24:00 UTC the two answers differ by a day, which would flip every arrival
/// and departure tag on the maid's list for seven hours a night.
/// `routes::new_rooms::LIVE_ROOM_FLAGS_SQL`'s `booked` predicate does use
/// `CURRENT_DATE` — that is the wrong idiom and must not be copied here.
pub(crate) const TODAY_BKK_DATE: &str = "(NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// [`TODAY_BKK`] for `ht_hk_linen_reports` (migration 088) — character-for-
/// character the same Bangkok civil-day boundary, on that table's own
/// `hklr_created_at` instead of `hkev_created_at`.
///
/// Spelled out rather than derived so it reads as SQL at the call site, and
/// pinned to [`TODAY_BKK`] by `today_bkk_linen_is_the_cleaning_day_boundary`:
/// a maid's "ขาดผ้า today" and her "cleaned today" MUST roll over at the same
/// instant, or the room-list badge outlives the events that explain it.
pub(crate) const TODAY_BKK_LINEN: &str =
    "(hklr_created_at AT TIME ZONE 'Asia/Bangkok')::date = (NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// The per-room resolution shared by the occupancy and departure predicates:
/// which ACTIVE folio, if any, holds this room right now.
///
/// `cr` rows are AUTHORITATIVE when the folio has any: `NOT IN` the checkout
/// spellings rather than `= 'เข้าพัก'`, because `cr_room_status` mixes
/// CT-mirrored legacy literals with our own `'active'` (production 2026-08-19:
/// `'Check-Out'` + `'เข้าพัก'` only, but the column is ours to write too, and
/// an allowlist would silently drop a spelling we add later). BOTH checkout
/// spellings are tolerated — `'Check-Out'` (ClickUSE.cs:1116) and `'Check Out'`
/// without the hyphen (FrmCheckOut.cs:6246, the known iHOTEL inconsistency in
/// COMPAT_CHEATSHEET) — plus the cancel literal `'ยกเลิก'`.
///
/// The bias is deliberate: junk under an active folio reads as OCCUPIED. A maid
/// walking in on a guest is the failure to avoid; a room shown occupied that is
/// actually empty costs her one door knock.
///
/// `cin_room_id` counts ONLY for folios with no `cr` rows at all — the pre-B5
/// single-room shape. Without that guard a multi-room folio would keep tagging
/// its `cin_room_id` room after that room's own line checked out.
///
/// This DELIBERATELY differs from `routes::new_rooms::LIVE_ROOM_FLAGS_SQL`,
/// which ORs `cin_room_id` in unconditionally and ignores `cr_room_status`
/// entirely. Do NOT unify them: that one feeds reception's grid, this one is
/// the maid's fallback and must not show a checked-out room as occupied.
const ACTIVE_FOLIO_HOLDS_ROOM: &str = r#"                   EXISTS (SELECT 1 FROM ht_checkin_rooms cr
                            WHERE cr.cr_cin_id = c.cin_id AND cr.cr_room_id = r.room_id
                              AND cr.cr_room_status NOT IN ('Check-Out', 'Check Out', 'ยกเลิก'))
                   OR (c.cin_room_id = r.room_id
                       AND NOT EXISTS (SELECT 1 FROM ht_checkin_rooms cr2 WHERE cr2.cr_cin_id = c.cin_id))"#;

/// The three derived per-room facts, as a SELECT-list fragment shared VERBATIM
/// by [`rooms_list_sql`] and [`room_detail_sql`] — one definition, so the list
/// and the detail card can never tell the maid different stories about the same
/// room.
fn derived_room_facts_sql() -> String {
    format!(
        r#"
            EXISTS (
              SELECT 1 FROM ht_checkins c
               WHERE c.cin_status = 'active' AND c.cin_checkout_time IS NULL
                 AND (
{ACTIVE_FOLIO_HOLDS_ROOM}
                 )
            ) AS occupied_pms,
            EXISTS (
              SELECT 1 FROM ht_booking_rooms br
                JOIN ht_bookings b ON b.book_id = br.br_book_id
               WHERE br.br_room_id = r.room_id
                 AND b.book_status IN ('confirmed', 'pending')
                 AND b.book_checkin = {TODAY_BKK_DATE}
            ) AS expected_arrival,
            EXISTS (
              SELECT 1 FROM ht_checkins c
               WHERE c.cin_status = 'active' AND c.cin_checkout_time IS NULL
                 AND c.cin_expected_checkout <= {TODAY_BKK_DATE}
                 AND (
{ACTIVE_FOLIO_HOLDS_ROOM}
                 )
            ) AS expected_departure"#
    )
}

/// Today's linen-shortage flag, as a SELECT-list fragment shared VERBATIM by
/// [`rooms_list_sql`] and [`room_detail_sql`] — same one-definition rule as
/// [`derived_room_facts_sql`], kept separate from it because this fact is not
/// part of the CR-1 legacy merge (iHOTEL has no linen counterpart).
///
/// An `EXISTS` correlated on `r.room_id`, NOT a per-room query: the list is one
/// statement and must stay one statement. `ix_ht_hk_linen_reports_room_created`
/// is exactly the index it wants.
fn linen_shortage_today_sql() -> String {
    format!(
        r#"
            EXISTS (
              SELECT 1 FROM ht_hk_linen_reports lr
               WHERE lr.hklr_room_id = r.room_id
                 AND {TODAY_BKK_LINEN}
            ) AS linen_shortage_today"#
    )
}

/// The OPEN linen-shortage flag — migration 090, and the one the ขาดผ้า badge
/// is actually served from. A SELECT-list fragment shared VERBATIM by
/// [`rooms_list_sql`] and [`room_detail_sql`], same one-definition rule as
/// [`linen_shortage_today_sql`] beside it.
///
/// Note what is NOT here: any date predicate at all. That absence is the
/// feature — a shortage is open until a maid restocks the room, whatever the
/// day, so the only condition is `hklr_resolved_at IS NULL`. The partial index
/// `ix_ht_hk_linen_reports_open` is built on exactly this predicate.
///
/// A correlated `EXISTS`, NOT a per-room query: the list is one statement and
/// must stay one statement.
fn linen_shortage_open_sql() -> String {
    r#"
            EXISTS (
              SELECT 1 FROM ht_hk_linen_reports lro
               WHERE lro.hklr_room_id = r.room_id
                 AND lro.hklr_resolved_at IS NULL
            ) AS linen_shortage_open"#
        .to_string()
}

/// `GET /api/hk/rooms` — the active-room list with today's latest cleaning
/// progress and the three derived facts.
///
/// A function rather than a `const` because [`derived_room_facts_sql`] is
/// interpolated; extracted from [`fetch_rooms`] so the SQL can be pinned by a
/// unit test without a database.
pub(crate) fn rooms_list_sql() -> String {
    format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,{facts},{linen},{linen_open},
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
        "#,
        facts = derived_room_facts_sql(),
        linen = linen_shortage_today_sql(),
        linen_open = linen_shortage_open_sql()
    )
}

/// `GET /api/hk/rooms/{id}` — the same row shape as [`rooms_list_sql`] for one
/// room. `$1` is `room_id`.
pub(crate) fn room_detail_sql() -> String {
    format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,{facts},{linen},{linen_open},
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
        "#,
        facts = derived_room_facts_sql(),
        linen = linen_shortage_today_sql(),
        linen_open = linen_shortage_open_sql()
    )
}

/// Fetch the active-room list with today's latest cleaning progress.
async fn fetch_rooms(pool: &PgPool) -> Result<Vec<HkRoom>, sqlx::Error> {
    let sql = rooms_list_sql();
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(room_from_row).collect())
}

/// Fetch one active room (today's progress + open-report count included).
async fn fetch_room(pool: &PgPool, room_id: i32) -> Result<Option<HkRoom>, sqlx::Error> {
    let sql = room_detail_sql();
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
        // `false` on a read failure for all three: the derived columns are
        // plain `EXISTS`, so a miss means the column is not there — and the
        // safe display for a fact we could not compute is "no tag", not an
        // invented one.
        occupancy: Occupancy::from_occupied(
            row.try_get::<bool, _>("occupied_pms").unwrap_or(false),
        ),
        expected_arrival: row.try_get::<bool, _>("expected_arrival").unwrap_or(false),
        expected_departure: row
            .try_get::<bool, _>("expected_departure")
            .unwrap_or(false),
        cleaning,
        // Same `false`-on-a-read-failure rule as the three above: a badge we
        // could not compute is better absent than invented.
        linen_shortage_today: row
            .try_get::<bool, _>("linen_shortage_today")
            .unwrap_or(false),
        linen_shortage_open: row
            .try_get::<bool, _>("linen_shortage_open")
            .unwrap_or(false),
    }
}

/// Where `kind` sorts in [`VALID_LINEN_KINDS`]; anything outside the vocabulary
/// ranks LAST (`len()`), never panics and is never dropped.
///
/// The DB column has no CHECK on purpose, so a code this binary does not know
/// is representable — an older row from before a kind was renamed, say. Showing
/// it at the end beats hiding a real shortage from housekeeping.
fn linen_kind_rank(kind: &str) -> usize {
    VALID_LINEN_KINDS
        .iter()
        .position(|known| *known == kind)
        .unwrap_or(VALID_LINEN_KINDS.len())
}

/// Put a room's per-kind totals in [`VALID_LINEN_KINDS`] order — the ONE
/// display order, shared with the frontend's label list.
///
/// Ordered here rather than in SQL: the vocabulary is a Rust constant, and a
/// `CASE`-ladder `ORDER BY` regenerated from it would be a second place for the
/// order to live. Unknown codes tie at the end and are broken alphabetically,
/// so the output is total and deterministic.
fn order_linen_totals(mut totals: Vec<LinenShortageTotal>) -> Vec<LinenShortageTotal> {
    totals.sort_by(|a, b| {
        linen_kind_rank(&a.kind)
            .cmp(&linen_kind_rank(&b.kind))
            .then_with(|| a.kind.cmp(&b.kind))
    });
    totals
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

/// Today's linen shortages for one room, summed per kind — migration 088.
///
/// GROUPed in the database (one row per kind, not one per submission) and
/// ORDERed in Rust by [`order_linen_totals`]. `SUM` widens to `numeric`, so the
/// cast to `bigint` is what makes the value readable as an `i64`.
async fn fetch_today_linen_shortages(
    pool: &PgPool,
    room_id: i32,
) -> Result<Vec<LinenShortageTotal>, sqlx::Error> {
    let sql = format!(
        "SELECT hklr_kind, SUM(hklr_qty)::bigint AS qty \
           FROM ht_hk_linen_reports \
          WHERE hklr_room_id = $1 AND {TODAY_BKK_LINEN} \
          GROUP BY hklr_kind"
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(room_id)
        .fetch_all(pool)
        .await?;
    Ok(order_linen_totals(
        rows.iter()
            .map(|row| LinenShortageTotal {
                kind: row.try_get::<String, _>("hklr_kind").unwrap_or_default(),
                qty: row.try_get::<i64, _>("qty").unwrap_or(0),
            })
            .collect(),
    ))
}

/// This room's OPEN linen shortages, summed per kind — migration 090, and the
/// breakdown behind the ขาดผ้า badge.
///
/// [`fetch_today_linen_shortages`] with ONE predicate swapped: no day window at
/// all, only `hklr_resolved_at IS NULL`. That is the whole 090 correction in
/// one line — a shortage is open until a maid restocks the room, whatever the
/// day. `ix_ht_hk_linen_reports_open` is built on exactly this predicate.
///
/// GROUPed in the database, ORDERed in Rust by [`order_linen_totals`] — the
/// same split, for the same reason (the vocabulary is a Rust constant).
async fn fetch_open_linen_shortages(
    pool: &PgPool,
    room_id: i32,
) -> Result<Vec<LinenShortageTotal>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT hklr_kind, SUM(hklr_qty)::bigint AS qty \
           FROM ht_hk_linen_reports \
          WHERE hklr_room_id = $1 AND hklr_resolved_at IS NULL \
          GROUP BY hklr_kind",
    )
    .bind(room_id)
    .fetch_all(pool)
    .await?;
    Ok(order_linen_totals(
        rows.iter()
            .map(|row| LinenShortageTotal {
                kind: row.try_get::<String, _>("hklr_kind").unwrap_or_default(),
                qty: row.try_get::<i64, _>("qty").unwrap_or(0),
            })
            .collect(),
    ))
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
    let can_report = identity.can_report;
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
        // A report the identity may not file is a dead tap however the env is
        // set, so the capability gates the flag rather than sitting beside it.
        mark_dirty_enabled: policy.mark_dirty_enabled && can_report,
        can_report,
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
    // A read-only viewer is exempt from location enforcement, so the picker
    // must offer exactly what [`require_location`] will then admit: the whole
    // allowlist. Checked FIRST, so no lookup is issued for a viewer — the same
    // symmetry the flag-off case relies on, and the reason a receptionist with
    // a NULL `Employee.location` gets a board instead of an empty picker.
    if !identity.can_report {
        return (policy.branches.clone(), None);
    }
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
    let outcome = resolve_legacy_room_flags(&policy, branch).await;
    let legacy_status_stale = merge_legacy_room_flags(&mut data, &outcome, branch);
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
    // Today's ขาดผ้า breakdown, from the SAME table and the SAME Thai day the
    // room's (deprecated) `linenShortageToday` badge was derived from.
    let linen_shortages = fetch_today_linen_shortages(pool, room_id).await?;
    // The OPEN breakdown (migration 090) — the same table under the SAME
    // predicate the room's `linenShortageOpen` badge came from, so the badge
    // and the card can never tell the maid different stories.
    let linen_shortages_open = fetch_open_linen_shortages(pool, room_id).await?;
    // CR-1: the SAME merge as the list, so a maid who taps into a room never
    // sees a different answer than the tile she tapped.
    let mut rooms = [room];
    let outcome = resolve_legacy_room_flags(&policy, branch).await;
    let legacy_status_stale = merge_legacy_room_flags(&mut rooms, &outcome, branch);
    let [room] = rooms;
    Ok(Json(RoomDetailResponse {
        success: true,
        room,
        events,
        linen_shortages,
        linen_shortages_open,
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
/// [`require_report_capability`] sits ahead of ALL of it: a `reception`-only
/// viewer may not report on any branch, so there is nothing for the branch gate
/// to decide and no reason to spend an HF ID lookup on the refusal. For a
/// `housekeeping` identity it is a no-op, so the order above is unchanged.
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
    require_report_capability(&identity)?;
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
        let outcome = resolve_legacy_room_flags(&policy, branch).await;
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

/// POST /api/hk/rooms/{id}/linen-shortage — report a linen shortage (ขาดผ้า).
///
/// A thin gate over
/// [`HousekeepingService::report_linen_shortage`](crate::service::housekeeping::HousekeepingService::report_linen_shortage),
/// which writes one `ht_hk_linen_reports` row per reported kind in ONE
/// transaction (migration 088).
///
/// **RECORD-ONLY.** No notification, no Slack, no domain event, no outbox row,
/// no legacy writeback — iHOTEL has no linen-inventory counterpart, so there is
/// nothing to mirror and no dark-shipped flag waiting to enable one. Reading
/// this handler top to bottom is the whole blast radius.
///
/// Gating is IDENTICAL to [`report_cleaning`] and in the same order, which is
/// what keeps the two maid mutations indistinguishable from the client's point
/// of view: report capability (403) → required `?branch=` (400/403) → location
/// gate (403/503) → body validation (400) → room existence (404). A request
/// with no branch is never answered on the strength of its body, and an unknown
/// room is resolved before any insert, so the FK on `hklr_room_id` is a
/// backstop rather than the gate.
///
/// There is no ship-dark flag between the location gate and validation — the
/// cleaning route's `HK_MARK_DIRTY_ENABLED` tier exists because `dirty` triggers
/// a LEGACY write. Nothing here reaches legacy, so invariant #6 has nothing to
/// gate.
///
/// The reporter's badge and display name come from the verified `HkIdentity`,
/// never from the body — same rule as every other handler on this surface.
pub async fn report_linen_shortage(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<ReportLinenShortageBody>,
) -> ApiResult<Json<ReportLinenShortageResponse>> {
    require_report_capability(&identity)?;
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let items = parse_linen_items(body.items)?;

    let pool = resolve_pool(&state, branch)?;
    require_room(pool, room_id).await?;

    let svc = service_for(&state, branch)?;
    let report = svc
        .report_linen_shortage(ReportLinenShortageCommand {
            room_id,
            items,
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
        })
        .await?;

    Ok(Json(ReportLinenShortageResponse {
        success: true,
        room_id,
        reported: report.reported,
    }))
}

/// POST /api/hk/rooms/{id}/linen-shortage/resolve — mark the room RESTOCKED
/// (เติมผ้าแล้ว), migration 090.
///
/// A thin gate over
/// [`HousekeepingService::resolve_linen_shortages`](crate::service::housekeeping::HousekeepingService::resolve_linen_shortages),
/// which closes EVERY open `ht_hk_linen_reports` row for the room in ONE
/// statement inside ONE transaction.
///
/// **Room-level, and there is no body.** One tap means "this room is
/// restocked": a maid carries the linen up once, so per-kind taps would be
/// busywork (owner request 2026-09-01). Nothing is deleted — history stays
/// append-only and a resolved row keeps everything it was filed with.
///
/// **Maid-only.** [`require_report_capability`] runs FIRST, so the read-only
/// reception viewer gets the same 403 [`REPORT_NOT_PERMITTED_ERROR`] it gets on
/// the other two maid mutations. That is deliberately NOT the room-signals rule
/// (where `can_report` picks a SIDE and both sides may act): completing a
/// shortage is a maid's physical act, not a message, so reception sees the
/// backlog and cannot close it.
///
/// Gating is otherwise IDENTICAL to [`report_linen_shortage`] and in the same
/// order: report capability (403) → required `?branch=` (400/403) → location
/// gate (403/503) → room existence (404). There is no body step, because there
/// is no body.
///
/// **`resolved: 0` is a 200.** A repeat tap, or a second maid arriving behind
/// the first, closes nothing and that is the state she asked for — the
/// predicate `hklr_resolved_at IS NULL` is both the selection and the
/// idempotency guard, so the FIRST resolver's identity and timestamp are never
/// overwritten.
///
/// **PG-only**, exactly like the report it completes: no notification, no
/// domain event, no outbox row, no legacy writeback (iHOTEL has no linen
/// counterpart). The boards refresh by poll/reload — this is record-domain, not
/// the live-board domain ADR 0008's signals publish events for. That is also
/// why `middleware::ville_guard` may exempt this path.
pub async fn resolve_linen_shortage(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Json<ResolveLinenShortageResponse>> {
    require_report_capability(&identity)?;
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;

    let pool = resolve_pool(&state, branch)?;
    require_room(pool, room_id).await?;

    let svc = service_for(&state, branch)?;
    let resolution = svc
        .resolve_linen_shortages(ResolveLinenShortageCommand {
            room_id,
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
        })
        .await?;

    Ok(Json(ResolveLinenShortageResponse {
        success: true,
        room_id,
        resolved: resolution.resolved,
    }))
}

// ============================================================================
// Room signals (ADR 0008)
// ============================================================================

/// `GET /api/hk/signals?branch=` — every signal still on this branch's board.
///
/// Gated exactly like the room reads: required `?branch=` (400/403) then the
/// per-employee location gate (403/503, no-op for a viewer and while the flag
/// is dark). NOT gated on [`require_report_capability`] — a reception viewer
/// is one of the two intended audiences of this list; what a viewer may not do
/// is RAISE a maid→desk signal, which the role gate below refuses.
///
/// `open` + `acked` only, per CONTEXT.md §Housekeeping: "a signal stays visible
/// until done, whatever the day". There is no date window and deliberately no
/// `?since=` — a signal raised before midnight is still the thing that needs
/// doing at 00:05.
pub async fn list_signals(
    State(state): State<AppState>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Json<SignalListResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let signals = signal_service_for(&state, branch)?.list_live().await?;
    Ok(Json(SignalListResponse {
        success: true,
        signals,
    }))
}

/// `POST /api/hk/rooms/{room_id}/signals` — raise one canned signal.
///
/// **The direction is DERIVED from the role, never sent by the client.** The
/// body carries only `{type}`; [`SignalRole::from_can_report`] turns the single
/// boolean the Access middleware resolved into "which side is this", and
/// `domain::hk_signal` decides whether that side may say this type. So a
/// hand-rolled request cannot post a desk→maid signal from a maid's badge by
/// adding a `direction` field, because there is no such field to add.
///
/// Note what is NOT here: [`require_report_capability`]. On the two cleaning
/// mutations it means "viewers may not write"; on THIS route it would mean
/// "reception may not raise desk→maid signals", which is the opposite of the
/// design — the `/hk` viewer IS the desk on this surface. The role gate inside
/// the service is what constrains each side, and it constrains BOTH.
pub async fn raise_signal(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<RaiseSignalBody>,
) -> ApiResult<Json<SignalResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    // Body BEFORE the room probe — same order as `report_linen_shortage`, so a
    // misspelled or wrong-direction type is a 400/403 rather than a database
    // round-trip (or a 500 when the pool is unreachable).
    require_signal_type(&identity, &body.signal_type)?;

    let pool = resolve_pool(&state, branch)?;
    require_room(pool, room_id).await?;

    let outcome = signal_service_for(&state, branch)?
        .raise(RaiseSignalCommand {
            room_id,
            signal_type: body.signal_type,
            role: hk_role(&identity),
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
            source: hk_source(),
        })
        .await?;
    Ok(Json(SignalResponse {
        success: true,
        signal: outcome.signal,
    }))
}

/// `POST /api/hk/signals/{id}/ack` — take a signal ("who's on it").
pub async fn ack_signal(
    state: State<AppState>,
    path: Path<i64>,
    query: Query<HkBranchQuery>,
    policy: Extension<HkPolicy>,
    identity: Extension<HkIdentity>,
) -> ApiResult<Json<SignalResponse>> {
    act_on_signal(state, path, query, policy, identity, SignalAction::Ack).await
}

/// `POST /api/hk/signals/{id}/done` — complete a signal.
///
/// A ขอเช็คห้อง is refused here with a 400 naming the answer endpoint: its
/// completion is a JUDGEMENT (เคลียร์ / มีของหาย / มีของเสียหาย), and a bare
/// tap would silently answer เคลียร์ while a guest waits at the counter. The
/// refusal is produced by `domain::hk_signal::next_status`, so the desk surface
/// answers it identically.
pub async fn done_signal(
    state: State<AppState>,
    path: Path<i64>,
    query: Query<HkBranchQuery>,
    policy: Extension<HkPolicy>,
    identity: Extension<HkIdentity>,
) -> ApiResult<Json<SignalResponse>> {
    act_on_signal(state, path, query, policy, identity, SignalAction::Done).await
}

/// `POST /api/hk/signals/{id}/cancel` — withdraw a still-open signal.
///
/// The ONE action performed on the caller's OWN direction, and only while
/// `open`: once somebody has acked it, it is their work, not the sender's to
/// take back.
pub async fn cancel_signal(
    state: State<AppState>,
    path: Path<i64>,
    query: Query<HkBranchQuery>,
    policy: Extension<HkPolicy>,
    identity: Extension<HkIdentity>,
) -> ApiResult<Json<SignalResponse>> {
    act_on_signal(state, path, query, policy, identity, SignalAction::Cancel).await
}

/// The shared body of ack / done / cancel — one gate order, one service call,
/// one response shape, so the three cannot drift into answering differently.
async fn act_on_signal(
    State(state): State<AppState>,
    Path(signal_id): Path<i64>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
    action: SignalAction,
) -> ApiResult<Json<SignalResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let outcome = signal_service_for(&state, branch)?
        .act(ActOnSignalCommand {
            signal_id,
            action,
            role: hk_role(&identity),
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
            source: hk_source(),
        })
        .await?;
    Ok(Json(SignalResponse {
        success: true,
        signal: outcome.signal,
    }))
}

/// `POST /api/hk/signals/{id}/answer` — the ขอเช็คห้อง answer.
///
/// `{"outcome":"clear"}` settles the room; `{"outcome":"problems","problems":
/// ["item_missing","item_damaged"]}` completes the check AND raises one
/// standing maid→desk signal per problem in the SAME transaction, each pointing
/// back at the check via `parentId`. Those children are the
/// guest-accountability signals the desk resolves before settling
/// (CONTEXT.md §Housekeeping), which is why they must not be able to go missing
/// while the check closes.
pub async fn answer_signal(
    State(state): State<AppState>,
    Path(signal_id): Path<i64>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<AnswerSignalBody>,
) -> ApiResult<Json<AnswerSignalResponse>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;
    let outcome_code = parse_outcome(&body.outcome)?;

    let answered = signal_service_for(&state, branch)?
        .answer_room_check(AnswerRoomCheckCommand {
            signal_id,
            outcome: outcome_code,
            problems: body.problems.unwrap_or_default(),
            role: hk_role(&identity),
            badge: identity.badge.clone(),
            name: identity.display_name.clone(),
            source: hk_source(),
        })
        .await?;
    Ok(Json(AnswerSignalResponse {
        success: true,
        signal: answered.signal,
        spawned: answered.spawned,
    }))
}

/// `GET /api/hk/events?branch=` — the maid page's live signal stream.
///
/// One SSE event name ([`crate::routes::events::HK_SIGNAL_EVENT`]) carrying the
/// [`RoomSignal`] DTO, plus the 30s keep-alive comments every stream in this
/// repo sends. It rides the SAME process-wide `domain_events` fan-out
/// reception's `/api/events` uses — no pool acquire, no second `PgListener`, so
/// a maid's open phone costs one `broadcast::Receiver` and nothing else (the
/// 2026-07-29 pool-exhaustion incident's rule).
///
/// **Branch isolation is the load-bearing property**, and it holds twice over:
/// the two sites are separate DATABASES with separate `domain_events` channels,
/// and [`crate::routes::events::hk_signal_receiver`] deliberately refuses the
/// hfhotel fallback that `EventFanout::receivers_for` performs for reception's
/// board. An unavailable Ville channel is a `503`, never HF Hotel's stream.
///
/// The gates run BEFORE the stream is built, so a refusal is an ordinary JSON
/// error response and not a stream that opens and then dies.
pub async fn signal_events(
    State(state): State<AppState>,
    Query(query): Query<HkBranchQuery>,
    Extension(policy): Extension<HkPolicy>,
    Extension(identity): Extension<HkIdentity>,
) -> ApiResult<Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>>> {
    let branch = require_branch(&policy, query.branch.as_deref())?;
    require_location(&policy, &identity, branch).await?;

    // `require_branch` already refused `all`, so this cannot be None; the
    // `ok_or_else` keeps that a checked fact rather than an `unwrap`.
    let site = crate::routes::events::EventSite::for_branch(branch)
        .ok_or_else(|| ApiError::BadRequest(BRANCH_REQUIRED_ERROR.to_string()))?;
    let receiver = crate::routes::events::hk_signal_receiver(&state.event_fanout, site)
        .ok_or_else(|| {
            tracing::warn!(
                branch = branch_id(branch),
                "/hk signal stream refused: that branch has no event fan-out"
            );
            ApiError::ServiceUnavailable(SIGNAL_STREAM_UNAVAILABLE_ERROR.to_string())
        })?;

    Ok(crate::routes::events::sse_from_frames(
        crate::routes::events::hk_signal_frames(receiver),
    ))
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
    use crate::legacy_room_status::LegacyRoomFlags;

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

    // ---- linen-shortage body validation (migration 088) ------------------

    /// Build the wire body from `(kind, qty)` pairs with integer quantities.
    fn linen_entries(pairs: &[(&str, i64)]) -> Option<Vec<LinenShortageEntry>> {
        Some(
            pairs
                .iter()
                .map(|(kind, qty)| LinenShortageEntry {
                    kind: (*kind).to_string(),
                    qty: serde_json::json!(qty),
                })
                .collect(),
        )
    }

    /// Assert a rejection is a 400 whose message mentions `needle`. Pinning the
    /// VARIANT matters as much as the fact of rejection: an `Internal` here
    /// would reach the maid as a 500 with no actionable text.
    fn assert_linen_400(
        result: Result<Vec<LinenShortageItem>, ApiError>,
        needle: &str,
        what: &str,
    ) {
        match result {
            Err(ApiError::BadRequest(msg)) => assert!(
                msg.contains(needle),
                "{what}: expected the 400 to mention '{needle}', got '{msg}'"
            ),
            Err(other) => panic!("{what}: expected 400 BadRequest, got {other:?}"),
            Ok(items) => panic!("{what}: expected a rejection, got {items:?}"),
        }
    }

    /// The happy path: every valid kind, at the quantity bounds, in one
    /// submission — and the parsed order matches the submitted order.
    #[test]
    fn linen_items_accept_every_kind_and_both_qty_bounds() {
        let parsed = parse_linen_items(linen_entries(&[
            ("bed_sheet", 1),
            ("pillowcase", 20),
            ("duvet_cover", 4),
            ("bath_towel", 7),
            ("face_towel", 2),
            ("foot_towel", 3),
        ]))
        .expect("a full, valid submission must parse");

        assert_eq!(parsed.len(), 6, "one item per reported kind");
        assert_eq!(
            parsed.iter().map(|i| i.kind.as_str()).collect::<Vec<_>>(),
            VALID_LINEN_KINDS.to_vec(),
            "submitted order is preserved"
        );
        assert_eq!(parsed[0].qty, MIN_LINEN_QTY, "1 is accepted");
        assert_eq!(parsed[1].qty, MAX_LINEN_QTY, "20 is accepted");
    }

    /// Kinds are trimmed + lower-cased, and the NORMALIZED code is what gets
    /// stored — otherwise the table accumulates casing variants of one kind.
    #[test]
    fn linen_kinds_are_normalized_before_storage() {
        let parsed = parse_linen_items(Some(vec![LinenShortageEntry {
            kind: "  Bath_Towel ".to_string(),
            qty: serde_json::json!(2),
        }]))
        .expect("a normalizable kind must parse");
        assert_eq!(parsed[0].kind, "bath_towel");
    }

    /// A missing `items` key and an explicitly empty list are the same answer:
    /// there is nothing to record.
    #[test]
    fn linen_items_missing_or_empty_is_400() {
        assert_linen_400(parse_linen_items(None), "items is required", "items absent");
        assert_linen_400(
            parse_linen_items(Some(vec![])),
            "items is required",
            "items empty",
        );
    }

    /// Six kinds is the whole vocabulary, so a seventh entry is malformed by
    /// construction — and the size check fires BEFORE any entry is judged, so
    /// an oversized body is never answered with a complaint about its content.
    #[test]
    fn linen_items_over_the_cap_is_400() {
        let seven = linen_entries(&[
            ("bed_sheet", 1),
            ("pillowcase", 1),
            ("duvet_cover", 1),
            ("bath_towel", 1),
            ("face_towel", 1),
            ("foot_towel", 1),
            ("pillowcase", 1),
        ]);
        assert_linen_400(
            parse_linen_items(seven),
            "too many linen entries",
            "7 entries",
        );

        // Shape is judged before content: an oversized list of INVALID kinds
        // still reports the size, not the first bad kind.
        let bogus: Vec<LinenShortageEntry> = (0..7)
            .map(|n| LinenShortageEntry {
                kind: format!("nonsense_{n}"),
                qty: serde_json::json!(1),
            })
            .collect();
        assert_linen_400(
            parse_linen_items(Some(bogus)),
            "too many linen entries",
            "7 invalid entries",
        );
    }

    /// Each kind may appear once, with its total. Two lines for one kind would
    /// land as two rows and double-count the shortage.
    #[test]
    fn linen_duplicate_kind_is_400() {
        assert_linen_400(
            parse_linen_items(linen_entries(&[("bath_towel", 2), ("bath_towel", 3)])),
            "duplicate linen kind",
            "exact duplicate",
        );
        // The duplicate test runs on the NORMALIZED code — otherwise these two
        // slip through as separate lines for the same kind.
        assert_linen_400(
            parse_linen_items(Some(vec![
                LinenShortageEntry {
                    kind: "bath_towel".to_string(),
                    qty: serde_json::json!(2),
                },
                LinenShortageEntry {
                    kind: "BATH_TOWEL".to_string(),
                    qty: serde_json::json!(3),
                },
            ])),
            "duplicate linen kind",
            "case-variant duplicate",
        );
    }

    /// Anything outside [`VALID_LINEN_KINDS`] is refused here, which is the ONLY
    /// place it can be refused: the DB column has no CHECK on purpose.
    #[test]
    fn linen_unknown_kind_is_400() {
        for bad in [
            "",
            "blanket",    // ผ้าห่ม — the plausible next kind, not shipped yet
            "bath-towel", // hyphen, not underscore
            "bathtowel",
            "ผ้าเช็ดตัว", // the Thai label; the wire codes are ASCII
            "pillowcase; DROP TABLE ht_hk_linen_reports",
        ] {
            assert_linen_400(
                parse_linen_items(linen_entries(&[(bad, 1)])),
                "invalid linen kind",
                bad,
            );
        }
    }

    /// The quantity must be a JSON INTEGER inside 1..=20. Every rejected shape
    /// here is one a hand-rolled client actually sends.
    #[test]
    fn linen_qty_must_be_an_integer_in_range() {
        for bad_qty in [0i64, -1, 21, 1000, i64::from(i32::MAX)] {
            assert_linen_400(
                parse_linen_items(linen_entries(&[("bath_towel", bad_qty)])),
                "invalid qty",
                &format!("qty {bad_qty}"),
            );
        }

        // A value beyond i32 must be refused, never wrapped into range.
        assert_linen_400(
            parse_linen_items(Some(vec![LinenShortageEntry {
                kind: "bath_towel".to_string(),
                qty: serde_json::json!(i64::MAX),
            }])),
            "invalid qty",
            "qty i64::MAX",
        );

        // Non-integer JSON types: each would otherwise be a serde rejection
        // with a body shape the maid's client cannot parse.
        for bad in [
            serde_json::json!(1.5),
            serde_json::json!("3"),
            serde_json::json!(null),
            serde_json::json!(true),
            serde_json::json!([2]),
            serde_json::json!({"qty": 2}),
        ] {
            let label = bad.to_string();
            assert_linen_400(
                parse_linen_items(Some(vec![LinenShortageEntry {
                    kind: "bath_towel".to_string(),
                    qty: bad,
                }])),
                "invalid qty",
                &label,
            );
        }
    }

    /// The cap and the vocabulary are stated separately but must stay
    /// consistent: a submission may name each kind at most once, so accepting
    /// more entries than there are kinds could only ever admit a duplicate.
    #[test]
    fn linen_cap_matches_the_vocabulary_size() {
        assert_eq!(
            MAX_LINEN_ITEMS,
            VALID_LINEN_KINDS.len(),
            "widening VALID_LINEN_KINDS without revisiting MAX_LINEN_ITEMS (or vice \
             versa) leaves the body cap and the dedup rule disagreeing"
        );
    }

    /// The route's bounds MUST equal the service's, which in turn mirror the
    /// `CHECK (hklr_qty >= 1 AND hklr_qty <= 20)` in migration 088 — otherwise a
    /// quantity the route admits dies on the constraint as a 500, not a 400.
    #[test]
    fn linen_qty_bounds_match_the_service_constants() {
        assert_eq!(MIN_LINEN_QTY, 1);
        assert_eq!(MAX_LINEN_QTY, 20);
    }

    // ---- linen READ surface (migration 088, room shapes) -----------------

    fn linen_total(kind: &str, qty: i64) -> LinenShortageTotal {
        LinenShortageTotal {
            kind: kind.to_string(),
            qty,
        }
    }

    /// The detail aggregation is displayed in VOCABULARY order, not in whatever
    /// order the `GROUP BY` happened to hand back — bed linen largest-first,
    /// the same sequence the frontend labels.
    #[test]
    fn linen_totals_are_ordered_by_the_vocabulary() {
        let ordered = order_linen_totals(vec![
            linen_total("foot_towel", 1),
            linen_total("bed_sheet", 2),
            linen_total("bath_towel", 3),
            linen_total("pillowcase", 4),
        ]);
        assert_eq!(
            ordered.iter().map(|t| t.kind.as_str()).collect::<Vec<_>>(),
            vec!["bed_sheet", "pillowcase", "bath_towel", "foot_towel"],
            "VALID_LINEN_KINDS is the display order"
        );
        // The quantities ride along with their kind, not with their position.
        assert_eq!(ordered[0].qty, 2);
        assert_eq!(ordered[3].qty, 1);
    }

    /// Every kind sorts, and each one sorts where the constant puts it — so
    /// reordering `VALID_LINEN_KINDS` reorders the response, which is the whole
    /// point of keeping the order in one place.
    #[test]
    fn linen_totals_ordering_covers_the_whole_vocabulary() {
        let ordered = order_linen_totals(
            VALID_LINEN_KINDS
                .iter()
                .rev()
                .map(|kind| linen_total(kind, 1))
                .collect(),
        );
        assert_eq!(
            ordered.iter().map(|t| t.kind.as_str()).collect::<Vec<_>>(),
            VALID_LINEN_KINDS.to_vec()
        );
    }

    /// A code outside the vocabulary is a row the DB accepted and this binary
    /// does not know (the column has no CHECK on purpose). It must be shown
    /// LAST, never dropped — a hidden shortage is worse than an odd label — and
    /// unknowns tie alphabetically so the output is deterministic.
    #[test]
    fn linen_totals_keep_unknown_kinds_at_the_end() {
        let ordered = order_linen_totals(vec![
            linen_total("zzz_retired", 1),
            linen_total("blanket", 2),
            linen_total("bath_towel", 3),
        ]);
        assert_eq!(
            ordered.iter().map(|t| t.kind.as_str()).collect::<Vec<_>>(),
            vec!["bath_towel", "blanket", "zzz_retired"]
        );
        assert_eq!(linen_kind_rank("bed_sheet"), 0, "first in the vocabulary");
        assert_eq!(
            linen_kind_rank("blanket"),
            VALID_LINEN_KINDS.len(),
            "an unknown kind ranks last, it does not panic"
        );
    }

    /// The LOCKED wire contract the `/hk` client is built against:
    /// `linenShortageToday` on every room, `linenShortages` on the detail —
    /// both ALWAYS present, the empty case an empty array rather than a missing
    /// key or `null`. Asserted on the SERIALIZED body, because camelCase and
    /// presence are the parts a Rust-side field check cannot see.
    #[test]
    fn the_linen_read_surface_serializes_under_its_locked_names() {
        let empty = serde_json::to_string(&RoomDetailResponse {
            success: true,
            room: pg_room("104", true),
            events: vec![],
            linen_shortages: vec![],
            linen_shortages_open: vec![],
            legacy_status_stale: false,
        })
        .expect("serializes");
        assert!(empty.contains("\"linenShortageToday\":false"), "{empty}");
        assert!(empty.contains("\"linenShortages\":[]"), "{empty}");
        // Migration 090's additions carry the same always-present rule.
        assert!(empty.contains("\"linenShortageOpen\":false"), "{empty}");
        assert!(empty.contains("\"linenShortagesOpen\":[]"), "{empty}");

        let reported = serde_json::to_string(&RoomDetailResponse {
            success: true,
            room: HkRoom {
                linen_shortage_today: true,
                linen_shortage_open: true,
                ..pg_room("104", true)
            },
            events: vec![],
            linen_shortages: order_linen_totals(vec![
                linen_total("bath_towel", 5),
                linen_total("bed_sheet", 2),
            ]),
            linen_shortages_open: order_linen_totals(vec![
                linen_total("bath_towel", 5),
                linen_total("bed_sheet", 2),
            ]),
            legacy_status_stale: false,
        })
        .expect("serializes");
        assert!(reported.contains("\"linenShortageToday\":true"), "{reported}");
        assert!(reported.contains("\"linenShortageOpen\":true"), "{reported}");
        assert!(
            reported.contains(
                "\"linenShortages\":[{\"kind\":\"bed_sheet\",\"qty\":2},\
                 {\"kind\":\"bath_towel\",\"qty\":5}]"
            ),
            "wire codes only, vocabulary order, {{kind,qty}} shape: {reported}"
        );
        assert!(
            reported.contains(
                "\"linenShortagesOpen\":[{\"kind\":\"bed_sheet\",\"qty\":2},\
                 {\"kind\":\"bath_towel\",\"qty\":5}]"
            ),
            "the OPEN totals share the report shape and the ONE display order: {reported}"
        );
    }

    /// The deprecated day-scoped fields and the migration-090 open fields are
    /// INDEPENDENT on the wire — the whole point of keeping both through the
    /// bundle-skew window. A room restocked this morning is `today: true`,
    /// `open: false`; a room short since yesterday is the mirror image, and
    /// THAT is the pair a pre-090 client gets wrong.
    #[test]
    fn the_open_and_today_linen_fields_do_not_track_each_other() {
        let restocked_today = serde_json::to_string(&RoomDetailResponse {
            success: true,
            room: HkRoom {
                linen_shortage_today: true,
                linen_shortage_open: false,
                ..pg_room("104", true)
            },
            events: vec![],
            linen_shortages: vec![linen_total("bath_towel", 5)],
            linen_shortages_open: vec![],
            legacy_status_stale: false,
        })
        .expect("serializes");
        assert!(
            restocked_today.contains("\"linenShortageToday\":true")
                && restocked_today.contains("\"linenShortageOpen\":false")
                && restocked_today.contains("\"linenShortagesOpen\":[]"),
            "a room reported AND restocked today is no longer ขาดผ้า: {restocked_today}"
        );

        let standing_since_yesterday = serde_json::to_string(&RoomDetailResponse {
            success: true,
            room: HkRoom {
                linen_shortage_today: false,
                linen_shortage_open: true,
                ..pg_room("104", true)
            },
            events: vec![],
            linen_shortages: vec![],
            linen_shortages_open: vec![linen_total("bath_towel", 5)],
            legacy_status_stale: false,
        })
        .expect("serializes");
        assert!(
            standing_since_yesterday.contains("\"linenShortageToday\":false")
                && standing_since_yesterday.contains("\"linenShortageOpen\":true"),
            "an unresolved shortage survives Bangkok midnight: {standing_since_yesterday}"
        );
    }

    /// The resolve response's LOCKED wire shape, including the `resolved: 0`
    /// success — a client must render that as done, so `success` must not
    /// depend on the count.
    #[test]
    fn the_resolve_response_serializes_under_its_locked_names() {
        let closed = serde_json::to_string(&ResolveLinenShortageResponse {
            success: true,
            room_id: 7,
            resolved: 3,
        })
        .expect("serializes");
        assert_eq!(closed, r#"{"success":true,"roomId":7,"resolved":3}"#);

        let idempotent = serde_json::to_string(&ResolveLinenShortageResponse {
            success: true,
            room_id: 7,
            resolved: 0,
        })
        .expect("serializes");
        assert_eq!(
            idempotent, r#"{"success":true,"roomId":7,"resolved":0}"#,
            "a repeat tap is a success with resolved=0, never an error"
        );
    }

    /// The list carries the flag too, and stays truthful during a legacy
    /// outage: linen is canonical-only, so `legacyStatusStale` never scopes it.
    #[test]
    fn the_linen_flag_survives_a_stale_legacy_read() {
        let mut rooms = vec![HkRoom {
            linen_shortage_today: true,
            ..pg_room("104", false)
        }];
        let stale =
            merge_legacy_room_flags(&mut rooms, &RoomFlagsOutcome::Unavailable, Branch::Hfhotel);
        let body = serde_json::to_string(&RoomsResponse {
            success: true,
            data: rooms,
            legacy_status_stale: stale,
        })
        .expect("serializes");
        assert!(body.contains("\"legacyStatusStale\":true"), "{body}");
        assert!(body.contains("\"linenShortageToday\":true"), "{body}");
    }

    /// The 090 flag is canonical-only for the same reason, so it survives a
    /// legacy outage too — the CR-1 merge must never reach it.
    #[test]
    fn the_open_linen_flag_survives_a_stale_legacy_read() {
        let mut rooms = vec![HkRoom {
            linen_shortage_open: true,
            ..pg_room("104", false)
        }];
        let stale =
            merge_legacy_room_flags(&mut rooms, &RoomFlagsOutcome::Unavailable, Branch::Hfhotel);
        let body = serde_json::to_string(&RoomsResponse {
            success: true,
            data: rooms,
            legacy_status_stale: stale,
        })
        .expect("serializes");
        assert!(body.contains("\"legacyStatusStale\":true"), "{body}");
        assert!(body.contains("\"linenShortageOpen\":true"), "{body}");
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

    /// A verified maid: the `housekeeping` grant, so `can_report` is `true`
    /// and every pre-reception-viewer assertion below is unchanged.
    fn maid(badge: &str) -> HkIdentity {
        HkIdentity {
            badge: badge.to_string(),
            display_name: None,
            email: None,
            can_report: true,
        }
    }

    /// A verified READ-ONLY viewer: the `reception` grant without
    /// `housekeeping`.
    fn viewer(badge: &str) -> HkIdentity {
        HkIdentity {
            can_report: false,
            ..maid(badge)
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

    // ---- the report capability (reception viewer) ------------------------

    /// The gate itself, both ways. A maid passes (so every pre-existing
    /// rejection order below is untouched); a viewer is refused with the
    /// repo's 403 and the stable Thai message.
    #[test]
    fn require_report_capability_admits_maids_and_refuses_viewers() {
        require_report_capability(&maid("Q1001")).expect("a housekeeping badge may report");

        match require_report_capability(&viewer("R2002"))
            .expect_err("a reception-only badge may not report")
        {
            ApiError::Forbidden(msg) => assert_eq!(msg, REPORT_NOT_PERMITTED_ERROR),
            other => panic!("expected a 403, got {other:?}"),
        }
    }

    /// The message a viewer actually reads. Thai (a person at the desk reads
    /// it, same class as [`MARK_DIRTY_DISABLED_ERROR`]) and it must not be
    /// silently reused for the branch/location errors, which mean something
    /// else entirely.
    #[test]
    fn report_not_permitted_error_is_its_own_thai_message() {
        assert!(
            REPORT_NOT_PERMITTED_ERROR.contains("ดูสถานะห้องได้อย่างเดียว"),
            "the copy must say what the account CAN do"
        );
        for other in [
            BRANCH_NOT_ENABLED_ERROR,
            MARK_DIRTY_DISABLED_ERROR,
            LOCATION_MISMATCH_ERROR,
            LOCATION_UNKNOWN_ERROR,
            LOCATION_LOOKUP_UNAVAILABLE_ERROR,
        ] {
            assert_ne!(
                REPORT_NOT_PERMITTED_ERROR, other,
                "each 403 on this surface names a DIFFERENT cause"
            );
        }
    }

    /// **The location-gate exemption.** A viewer clears the gate for every
    /// branch the allowlist offers, on every enforcement outcome — including
    /// the two that 403/503 a maid. The NoLocation row is the one that matters
    /// in production: reception badges routinely have a NULL
    /// `Employee.location`, and without this exemption the desk's board would
    /// be permanently empty.
    #[tokio::test]
    async fn viewers_are_exempt_from_the_location_gate() {
        for outcome in [
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            LocationOutcome::NoLocation,
            LocationOutcome::Unavailable,
            LocationOutcome::AnyLocation,
        ] {
            let p = enforcing(vec![Branch::Hfhotel, Branch::Hfville], outcome);
            for branch in [Branch::Hfhotel, Branch::Hfville] {
                assert!(
                    require_location(&p, &viewer("R2002"), branch)
                        .await
                        .is_ok(),
                    "{outcome:?} / {branch:?}: a viewer files nothing, so the \
                     wrong-property gate has nothing to guard"
                );
            }
        }
    }

    /// The exemption is the VIEWER's, not everybody's: the identical policy
    /// still refuses the same maid. This is the pairing that would catch an
    /// exemption written as an unconditional early return.
    #[tokio::test]
    async fn the_exemption_does_not_leak_to_maids() {
        let p = enforcing(vec![Branch::Hfhotel, Branch::Hfville], LocationOutcome::NoLocation);
        assert!(
            require_location(&p, &viewer("R2002"), Branch::Hfhotel)
                .await
                .is_ok()
        );
        match require_location(&p, &maid("421"), Branch::Hfhotel)
            .await
            .expect_err("a maid with no location is still refused")
        {
            ApiError::Forbidden(msg) => assert_eq!(msg, LOCATION_UNKNOWN_ERROR),
            other => panic!("expected 403, got {other:?}"),
        }
    }

    /// The picker half of the same exemption. `me_branches` and
    /// [`require_location`] must agree, or the board offers a branch every
    /// subsequent call then refuses — so a viewer gets the WHOLE allowlist and
    /// no `branchesUnavailableReason`, exactly what the gate above admits.
    #[tokio::test]
    async fn me_serves_a_viewer_the_whole_allowlist_under_enforcement() {
        for outcome in [
            LocationOutcome::NoLocation,
            LocationOutcome::Unavailable,
            LocationOutcome::Resolved(EmployeeLocation::Hf),
        ] {
            let p = enforcing(vec![Branch::Hfhotel, Branch::Hfville], outcome);
            let (branches, reason) = me_branches(&p, &viewer("R2002")).await;
            assert_eq!(
                branches,
                vec![Branch::Hfhotel, Branch::Hfville],
                "{outcome:?}: a viewer sees every property this deployment serves"
            );
            assert_eq!(reason, None, "{outcome:?}: nothing to explain — nothing is empty");
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
    /// merge is allowed to overwrite. Occupancy defaults to the canonical
    /// DERIVED `Vacant`; use [`pg_room_occupied`] for the other pole.
    fn pg_room(room_no: &str, clean: bool) -> HkRoom {
        HkRoom {
            room_id: 1,
            room_no: room_no.to_string(),
            floor: Some(1),
            building: None,
            room_clean: clean,
            occupancy: Occupancy::Vacant,
            expected_arrival: false,
            expected_departure: false,
            cleaning: None,
            linen_shortage_today: false,
            linen_shortage_open: false,
        }
    }

    /// A room PG derived as OCCUPIED, so an iHOTEL `Room_Use='no'` has
    /// something to overrule.
    fn pg_room_occupied(room_no: &str, clean: bool) -> HkRoom {
        HkRoom {
            occupancy: Occupancy::Occupied,
            ..pg_room(room_no, clean)
        }
    }

    /// iHOTEL's answer for the CLEANLINESS fact only, in CANONICAL polarity
    /// (the inversion already applied by
    /// `legacy_room_status::legacy_clean_to_is_clean`). `occupied` is UNKNOWN,
    /// which is what keeps the pre-existing merge tests honest about the
    /// cleanliness rules alone.
    fn ihotel(entries: &[(&str, bool)]) -> RoomFlagsOutcome {
        RoomFlagsOutcome::Available(
            entries
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

    /// iHOTEL's answer for BOTH facts.
    fn ihotel_flags(entries: &[(&str, Option<bool>, Option<bool>)]) -> RoomFlagsOutcome {
        RoomFlagsOutcome::Available(
            entries
                .iter()
                .map(|(no, is_clean, occupied)| {
                    (
                        (*no).to_string(),
                        LegacyRoomFlags {
                            is_clean: *is_clean,
                            occupied: *occupied,
                        },
                    )
                })
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
        let stale = merge_legacy_room_flags(
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
        let stale = merge_legacy_room_flags(
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
            merge_legacy_room_flags(&mut rooms, &RoomFlagsOutcome::Unavailable, Branch::Hfhotel);
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
        assert!(p.legacy_room_flags_branches().is_empty());
        assert_eq!(
            resolve_legacy_room_flags(&p, Branch::Hfville).await,
            RoomFlagsOutcome::Unavailable
        );
    }

    /// The reader is picked by BRANCH. A Ville maid must never be reconciled
    /// against HF Hotel's legacy server — that is the wrong-hotel bug this
    /// whole surface exists to close, one database deeper.
    #[tokio::test]
    async fn readers_are_resolved_per_branch() {
        #[derive(Debug)]
        struct Fixed(RoomFlagsOutcome);
        #[async_trait::async_trait]
        impl RoomFlagsSource for Fixed {
            async fn room_flags(&self) -> RoomFlagsOutcome {
                self.0.clone()
            }
        }

        let p = policy(vec![Branch::Hfhotel, Branch::Hfville]).with_legacy_room_flags(
            Branch::Hfhotel,
            Arc::new(Fixed(ihotel(&[("104", false)]))),
        );
        assert_eq!(p.legacy_room_flags_branches(), vec!["hfhotel"]);
        assert_eq!(
            resolve_legacy_room_flags(&p, Branch::Hfhotel).await,
            ihotel(&[("104", false)])
        );
        // Ville has no reader — fallback, NOT HF Hotel's answer.
        assert_eq!(
            resolve_legacy_room_flags(&p, Branch::Hfville).await,
            RoomFlagsOutcome::Unavailable
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
            merge_legacy_room_flags(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
        assert!(!stale, "one unmatched room is not a staleness event");
        assert!(!rooms[0].room_clean, "104 follows iHOTEL");
        assert!(!rooms[1].room_clean, "999 keeps its canonical value");
    }

    /// Legacy `Room_no` values are `varchar` and padded in places; the join
    /// must survive that or every room silently falls back.
    #[test]
    fn room_numbers_are_matched_trimmed() {
        let mut rooms = vec![pg_room(" 104 ", true)];
        merge_legacy_room_flags(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
        assert!(!rooms[0].room_clean);
    }

    /// An EMPTY answer from iHOTEL is still an answer — nothing is stale, and
    /// every room simply keeps its canonical value. (An empty `HT_Rooms` is
    /// not a thing; this pins that "no rows" can't be mistaken for "outage",
    /// because the two produce different notes on the maid's screen.)
    #[test]
    fn an_empty_ihotel_answer_is_not_stale() {
        let mut rooms = vec![pg_room("104", true)];
        let stale = merge_legacy_room_flags(&mut rooms, &ihotel(&[]), Branch::Hfhotel);
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
            merge_legacy_room_flags(&mut rooms, &ihotel(&[("104", false)]), Branch::Hfhotel);
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

    // ------------------------------------------------------------------
    // Occupancy — the SECOND fact, merged per-fact under the same rules
    // ------------------------------------------------------------------

    /// The wire spelling. A maid's card renders these strings; changing them
    /// is a frontend break, so pin them here rather than in a snapshot.
    #[test]
    fn occupancy_serializes_lowercase() {
        assert_eq!(
            serde_json::to_string(&Occupancy::Occupied).expect("serializes"),
            "\"occupied\""
        );
        assert_eq!(
            serde_json::to_string(&Occupancy::Vacant).expect("serializes"),
            "\"vacant\""
        );
    }

    /// Rule 1 for occupancy, both directions — iHOTEL's `Room_Use` replaces
    /// the canonical DERIVED value whichever way they disagree.
    ///
    /// The `Room_Use='no'` over a PG-derived OCCUPIED room is the live case:
    /// Ville room 106 on 2026-08-19 (PG derived occupied from an active folio,
    /// iHOTEL said vacant). The maid must see iHOTEL.
    #[test]
    fn ihotel_occupancy_wins_over_the_derived_value_in_both_directions() {
        let mut rooms = vec![pg_room_occupied("106", true), pg_room("203", true)];
        let stale = merge_legacy_room_flags(
            &mut rooms,
            &ihotel_flags(&[
                ("106", None, Some(false)),
                ("203", None, Some(true)),
            ]),
            Branch::Hfhotel,
        );
        assert!(!stale, "iHOTEL answered — nothing is stale");
        assert_eq!(rooms[0].occupancy, Occupancy::Vacant, "106 follows iHOTEL");
        assert_eq!(rooms[1].occupancy, Occupancy::Occupied, "203 follows iHOTEL");
    }

    /// PER-FACT, the whole point of [`LegacyRoomFlags`]: a room whose
    /// `Room_Clean` parsed but whose `Room_Use` did not takes iHOTEL's
    /// cleanliness and KEEPS its canonical occupancy — silently, and without
    /// flagging the list stale.
    #[test]
    fn an_unknown_fact_keeps_its_canonical_value_while_the_other_still_wins() {
        let mut rooms = vec![pg_room_occupied("104", true)];
        let stale = merge_legacy_room_flags(
            &mut rooms,
            &ihotel_flags(&[("104", Some(false), None)]),
            Branch::Hfhotel,
        );
        assert!(!stale, "one unknown FACT is not a staleness event");
        assert!(!rooms[0].room_clean, "cleanliness follows iHOTEL");
        assert_eq!(
            rooms[0].occupancy,
            Occupancy::Occupied,
            "occupancy keeps the canonical derived value"
        );
    }

    /// The mirror image — `Room_Use` parsed, `Room_Clean` did not.
    #[test]
    fn an_occupancy_only_answer_leaves_cleanliness_canonical() {
        let mut rooms = vec![pg_room("104", true)];
        let stale = merge_legacy_room_flags(
            &mut rooms,
            &ihotel_flags(&[("104", None, Some(true))]),
            Branch::Hfhotel,
        );
        assert!(!stale);
        assert!(rooms[0].room_clean, "cleanliness keeps its canonical value");
        assert_eq!(rooms[0].occupancy, Occupancy::Occupied);
    }

    /// Rule 2 covers BOTH facts with the ONE flag: an unreachable iHOTEL
    /// leaves cleanliness AND occupancy exactly as PG derived them.
    #[test]
    fn unavailable_legacy_leaves_both_facts_canonical_and_flags_stale() {
        let mut rooms = vec![pg_room_occupied("104", true), pg_room("203", false)];
        let stale =
            merge_legacy_room_flags(&mut rooms, &RoomFlagsOutcome::Unavailable, Branch::Hfhotel);
        assert!(stale, "the client MUST be told it is showing the fallback");
        assert_eq!(rooms[0].occupancy, Occupancy::Occupied);
        assert!(rooms[0].room_clean);
        assert_eq!(rooms[1].occupancy, Occupancy::Vacant);
        assert!(!rooms[1].room_clean);
    }

    /// The write guard reads the CLEANLINESS fact only. A room with a junk
    /// `Room_Clean` under a perfectly readable `Room_Use` must be Unknown to
    /// the D1 decision — occupancy has no say in whether a tap enqueues.
    #[test]
    fn the_write_hint_ignores_the_occupancy_fact() {
        let outcome = ihotel_flags(&[("104", None, Some(true))]);
        assert_eq!(
            legacy_hint_for_room(&outcome, "104"),
            LegacyCleanliness::Unknown
        );
    }

    // ------------------------------------------------------------------
    // Arrival / departure tags — canonical-only, untouched by the merge
    // ------------------------------------------------------------------

    /// The two planning tags are canonical derivations with no legacy
    /// counterpart. Whatever the merge does to cleanliness and occupancy, it
    /// must leave these EXACTLY as fetched — including on the `Unavailable`
    /// path, which is what keeps them truthful during a legacy outage.
    #[test]
    fn the_merge_never_touches_the_arrival_and_departure_tags() {
        for outcome in [
            ihotel_flags(&[("104", Some(false), Some(false))]),
            RoomFlagsOutcome::Unavailable,
        ] {
            let mut rooms = vec![HkRoom {
                expected_arrival: true,
                expected_departure: true,
                ..pg_room_occupied("104", true)
            }];
            merge_legacy_room_flags(&mut rooms, &outcome, Branch::Hfhotel);
            assert!(
                rooms[0].expected_arrival,
                "arrival is canonical-only and must survive the merge: {outcome:?}"
            );
            assert!(
                rooms[0].expected_departure,
                "departure is canonical-only and must survive the merge: {outcome:?}"
            );
        }
    }

    /// A stale response still carries LIVE tags. Serialized, because this is a
    /// wire-contract claim: `legacyStatusStale` scopes the two iHOTEL facts,
    /// never these two.
    #[test]
    fn a_stale_response_still_carries_live_arrival_and_departure_tags() {
        let mut rooms = vec![HkRoom {
            expected_arrival: true,
            expected_departure: true,
            ..pg_room("104", false)
        }];
        let stale =
            merge_legacy_room_flags(&mut rooms, &RoomFlagsOutcome::Unavailable, Branch::Hfhotel);
        let body = serde_json::to_string(&RoomsResponse {
            success: true,
            data: rooms,
            legacy_status_stale: stale,
        })
        .expect("serializes");
        assert!(body.contains("\"legacyStatusStale\":true"), "{body}");
        assert!(body.contains("\"expectedArrival\":true"), "{body}");
        assert!(body.contains("\"expectedDeparture\":true"), "{body}");
    }

    // ------------------------------------------------------------------
    // SQL pins — the derived columns, without a database
    // ------------------------------------------------------------------

    /// Both statements carry all three derived columns, from the ONE shared
    /// fragment. A list and a detail card that derived a room's facts
    /// differently would be the same "two screens, two answers" defect CR-1
    /// exists to close.
    #[test]
    fn both_fetch_statements_derive_the_same_three_facts() {
        let facts = derived_room_facts_sql();
        for sql in [rooms_list_sql(), room_detail_sql()] {
            assert!(sql.contains(&facts), "shared fragment missing: {sql}");
            for column in ["AS occupied_pms", "AS expected_arrival", "AS expected_departure"] {
                assert!(sql.contains(column), "{column} missing: {sql}");
            }
        }
    }

    /// The bypassed column must never come back. `ht_rooms_new.room_status` is
    /// not maintained by check-in/check-out (issue #200); reading it would put
    /// a room's occupancy months behind the guest standing in it.
    ///
    /// Checked as `r.room_status` — the folio-line column `cr.cr_room_status`
    /// is a DIFFERENT column and is read on purpose.
    #[test]
    fn the_derived_facts_never_read_the_stored_room_status_column() {
        for sql in [rooms_list_sql(), room_detail_sql()] {
            assert!(
                !sql.contains("r.room_status"),
                "occupancy must be DERIVED, never read from ht_rooms_new.room_status: {sql}"
            );
            assert!(
                sql.contains("cr.cr_room_status"),
                "…but the folio LINE status is exactly what resolves the room: {sql}"
            );
        }
    }

    /// Every date predicate is on the Bangkok civil day. `CURRENT_DATE` is the
    /// SERVER's date and disagrees with Bangkok for seven hours every night —
    /// long enough to blank every arrival and departure tag on a night shift.
    #[test]
    fn the_date_predicates_are_bangkok_never_current_date() {
        for sql in [rooms_list_sql(), room_detail_sql()] {
            assert!(
                !sql.contains("CURRENT_DATE"),
                "CURRENT_DATE is banned in these predicates: {sql}"
            );
            assert!(
                sql.contains("b.book_checkin = (NOW() AT TIME ZONE 'Asia/Bangkok')::date"),
                "arrival must key on the Bangkok civil day: {sql}"
            );
            assert!(
                sql.contains(
                    "c.cin_expected_checkout <= (NOW() AT TIME ZONE 'Asia/Bangkok')::date"
                ),
                "departure must key on the Bangkok civil day: {sql}"
            );
        }
    }

    /// The checkout literals both spellings must be tolerated in, plus the
    /// cancel literal — and the NOT-IN shape itself, which is what biases a
    /// junk `cr_room_status` under an active folio towards OCCUPIED.
    #[test]
    fn the_room_resolution_excludes_both_checkout_spellings() {
        assert!(ACTIVE_FOLIO_HOLDS_ROOM.contains("NOT IN ('Check-Out', 'Check Out', 'ยกเลิก')"));
        assert!(
            !ACTIVE_FOLIO_HOLDS_ROOM.contains("= 'เข้าพัก'"),
            "an allowlist would silently drop a spelling we add later"
        );
        // `cin_room_id` counts ONLY for folios with no `cr` rows at all.
        assert!(ACTIVE_FOLIO_HOLDS_ROOM
            .contains("NOT EXISTS (SELECT 1 FROM ht_checkin_rooms cr2 WHERE cr2.cr_cin_id = c.cin_id)"));
    }

    /// Occupancy and departure resolve "which folio holds this room" through
    /// the SAME text. Keeping them textually identical is what stops a room
    /// from being tagged "leaving today" while shown vacant.
    #[test]
    fn occupancy_and_departure_share_one_room_resolution() {
        let facts = derived_room_facts_sql();
        assert_eq!(
            facts.matches(ACTIVE_FOLIO_HOLDS_ROOM).count(),
            2,
            "occupancy and departure must use the same resolution verbatim: {facts}"
        );
    }

    /// Both statements carry today's linen flag, from the ONE shared fragment —
    /// same rule as the three derived facts: a list tile badged ขาดผ้า and a
    /// detail card that is not would be the identical "two screens, two
    /// answers" defect.
    #[test]
    fn both_fetch_statements_carry_todays_linen_flag() {
        let linen = linen_shortage_today_sql();
        for sql in [rooms_list_sql(), room_detail_sql()] {
            assert!(sql.contains(&linen), "shared fragment missing: {sql}");
            assert!(sql.contains("AS linen_shortage_today"), "{sql}");
            // ONE statement for the whole list: the flag is a correlated
            // EXISTS, never a per-room round trip.
            assert!(
                sql.contains("EXISTS (\n              SELECT 1 FROM ht_hk_linen_reports lr"),
                "the flag must stay an EXISTS subquery, not an N+1: {sql}"
            );
        }
    }

    /// "Linen today" and "cleaned today" MUST roll over at the same instant, or
    /// the room-list badge outlives the events that explain it. Pinned as the
    /// cleaning predicate with ONLY the column swapped — and `CURRENT_DATE`, the
    /// SERVER's date, stays banned here too.
    #[test]
    fn today_bkk_linen_is_the_cleaning_day_boundary() {
        assert_eq!(
            TODAY_BKK_LINEN,
            TODAY_BKK.replace("hkev_created_at", "hklr_created_at"),
            "the linen day boundary must be the cleaning one, column swapped"
        );
        assert!(TODAY_BKK_LINEN.contains(TODAY_BKK_DATE));
        assert!(!TODAY_BKK_LINEN.contains("CURRENT_DATE"));
        assert!(linen_shortage_today_sql().contains(TODAY_BKK_LINEN));
    }

    /// Both statements carry the OPEN flag too, from its OWN shared fragment —
    /// the badge the surface actually renders (migration 090). Same
    /// one-definition rule as the day-scoped flag beside it.
    #[test]
    fn both_fetch_statements_carry_the_open_linen_flag() {
        let open = linen_shortage_open_sql();
        for sql in [rooms_list_sql(), room_detail_sql()] {
            assert!(sql.contains(&open), "shared fragment missing: {sql}");
            assert!(sql.contains("AS linen_shortage_open"), "{sql}");
            // ONE statement for the whole list: a correlated EXISTS, never a
            // per-room round trip.
            assert!(
                sql.contains("EXISTS (\n              SELECT 1 FROM ht_hk_linen_reports lro"),
                "the open flag must stay an EXISTS subquery, not an N+1: {sql}"
            );
        }
    }

    /// **The 090 correction, pinned.** The open flag is scoped by RESOLUTION
    /// and by nothing else: no Bangkok day, no `CURRENT_DATE`, no
    /// `hklr_created_at` at all. A date predicate creeping back in here would
    /// silently restore the day-rollover bug — a room short since yesterday
    /// going un-badged at 00:01 — which is exactly what completion supersedes.
    #[test]
    fn the_open_linen_flag_is_scoped_by_resolution_never_by_a_day() {
        let open = linen_shortage_open_sql();
        assert!(
            open.contains("hklr_resolved_at IS NULL"),
            "resolution is the ONLY predicate: {open}"
        );
        for banned in [
            TODAY_BKK_LINEN,
            TODAY_BKK_DATE,
            "CURRENT_DATE",
            "hklr_created_at",
            "Asia/Bangkok",
        ] {
            assert!(
                !open.contains(banned),
                "the open flag must not be day-scoped, found '{banned}' in: {open}"
            );
        }
        // The two fragments are DIFFERENT facts and must not be confused: the
        // deprecated one stays day-scoped and resolution-blind.
        let today = linen_shortage_today_sql();
        assert!(
            !today.contains("hklr_resolved_at"),
            "linenShortageToday keeps its exact 088 meaning: {today}"
        );
        assert_ne!(open, today);
        // Distinct correlation aliases, so the two EXISTS clauses can sit in
        // one SELECT list without shadowing each other.
        assert!(open.contains(" lro\n") && today.contains(" lr\n"), "{open} / {today}");
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
            can_report: true,
        };
        assert_eq!(maid_label(&with_name), "นก");

        // Production reality today: the CF IdP forwards only `apps` + `badge`.
        let no_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: None,
            email: None,
            can_report: true,
        };
        assert_eq!(maid_label(&no_name), "Q1001");

        // A whitespace-only name must not produce a blank `by`.
        let blank_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: Some("   ".into()),
            email: None,
            can_report: true,
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
            can_report: true,
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
            can_report: true,
        });
        assert_eq!(clamped_ascii.len(), MAX_H_NAME_CHARS);

        // Normal names are untouched.
        assert_eq!(
            maid_label(&HkIdentity {
                badge: "Q1001".into(),
                display_name: Some("นก สมใจ".into()),
                email: None,
                can_report: true,
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
            legacy_hint_for_room(&RoomFlagsOutcome::Unavailable, "104"),
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
