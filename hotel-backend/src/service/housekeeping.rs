//! Housekeeping service — room cleanliness state transitions.
//!
//! Per `docs/architecture.md` §1, §6 and `docs/legacy-spike/findings.md` §3j
//! (mark-room-clean recipe). Each public method opens one PG transaction,
//! flips `ht_rooms_new.room_clean`, enqueues the relevant
//! [`WritebackIntent`] (`MarkRoomClean` per spike §3j; `MarkRoomDirty` per
//! cheatsheet §3.13 — added by the 2026-06-11 coexistence audit P2 track,
//! closing the gap where a standalone mark-dirty never reached iHOTEL's
//! grid), publishes a [`DomainEvent`], and commits.

use std::sync::Arc;

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::outbox::event::{DomainEvent, EventSource};
use crate::outbox::intent::WritebackIntent;
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::room::RoomRepository;

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// How long a MIRROR REPAIR (defect D1) is suppressed after a writeback for the
/// same room + intent was already enqueued.
///
/// Only the repair path consults it — a real canonical transition is never
/// suppressed. It exists because iHOTEL keeps answering "dirty" until our own
/// `MarkRoomClean` job actually drains, so a maid double-tapping เสร็จแล้ว
/// during that gap would otherwise force a SECOND job for the same cleaning.
///
/// 5 minutes matches `writeback::recipes::mark_clean`'s own
/// `WHERE NOT EXISTS … DATEADD(minute, -5, GETDATE())` guard on the
/// `HT_Housewife` audit insert: inside the window a duplicate would be swallowed
/// legacy-side anyway, and beyond it BOTH sides treat the write as a legitimate
/// re-clean. Deliberately finite — if a repair is emitted, drains, and iHOTEL
/// STILL disagrees minutes later, re-emitting is the correct healing behaviour.
const MIRROR_REPAIR_DEDUP_MINUTES: i32 = 5;

/// Command for [`HousekeepingService::mark_clean`].
#[derive(Debug, Clone)]
pub struct MarkCleanCommand {
    pub room_id: i32,
    /// Housekeeper identifier. Lands verbatim in `HT_Housewife.h_name` —
    /// `writeback::recipes::mark_clean::build_housewife_audit_insert` projects
    /// this value into the `[h_name]` column (spike §3j; the neighbouring
    /// `h_cin_name` is the *prior occupant*, looked up in the recipe, NOT this
    /// field). Routes populate from auth context; the `/hk` maid surface
    /// passes the verified HF ID display name so the audit row names the maid.
    pub by: String,
    pub source: EventSource,
}

/// Command for [`HousekeepingService::mark_dirty`].
#[derive(Debug, Clone)]
pub struct MarkDirtyCommand {
    pub room_id: i32,
    /// Operator identifier — lands in `HT_Housewife.h_name` of the
    /// "start cleaning" audit row (cheatsheet §3.13). Routes populate
    /// from auth context, mirroring [`MarkCleanCommand::by`].
    pub by: String,
    pub source: EventSource,
}

/// Command for [`HousekeepingService::set_maintenance`].
#[derive(Debug, Clone)]
pub struct MarkMaintenanceCommand {
    pub room_id: i32,
    /// `true` ⇒ take the room out of service (legacy `Room_Manternace='yes'`);
    /// `false` ⇒ return it to service.
    pub maintenance: bool,
    /// Carried for symmetry with the clean/dirty commands and forward-compat
    /// (a maintenance `DomainEvent` may be added later). Not currently
    /// published — `update_room` likewise enqueues the maintenance writeback
    /// without a domain event.
    pub source: EventSource,
}

/// Outcome of a housekeeping mutation.
#[derive(Debug, Clone)]
pub struct HousekeepingOutcome {
    pub room_id: i32,
    pub aggregate_id: Uuid,
}

/// A maid-reported cleaning-progress state on the `/hk` surface.
///
/// Mirrors the `ht_hk_cleaning_events.hkev_status` CHECK constraint
/// (migration 077, widened to include `dirty` by migration 087) — keep the two
/// in lock-step: a status accepted here that the CHECK rejects turns a maid's
/// tap into a 500.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleaningProgressStatus {
    /// เริ่มทำความสะอาด — PG-only, never mirrored (see
    /// [`DomainEvent::RoomCleaningStarted`]).
    Started,
    /// เสร็จแล้ว — flips `room_clean` to `true` and mirrors `MarkRoomClean`.
    Done,
    /// ห้องยังไม่สะอาด — flips `room_clean` to `false` and mirrors
    /// `MarkRoomDirty`. Reachable only while `HK_MARK_DIRTY_ENABLED` is on
    /// (invariant #6 — the route gates it; the service does not read env).
    Dirty,
}

impl CleaningProgressStatus {
    /// The literal stored in `ht_hk_cleaning_events.hkev_status`.
    pub fn as_str(self) -> &'static str {
        match self {
            CleaningProgressStatus::Started => "started",
            CleaningProgressStatus::Done => "done",
            CleaningProgressStatus::Dirty => "dirty",
        }
    }

    /// Parse a already-normalized (trimmed, lowercased) status literal.
    pub fn from_literal(raw: &str) -> Option<Self> {
        match raw {
            "started" => Some(CleaningProgressStatus::Started),
            "done" => Some(CleaningProgressStatus::Done),
            "dirty" => Some(CleaningProgressStatus::Dirty),
            _ => None,
        }
    }
}

/// What iHOTEL said about THIS room at the moment of the tap — the write-guard
/// half of the CR-1 "iHOTEL wins" decision (defect D1, wave-5).
///
/// ## Why the guard needs this at all
///
/// `/hk`'s DISPLAY is iHOTEL-wins (`routes::hk::merge_legacy_room_clean`), but
/// the write guard used to judge against canonical `ht_rooms_new.room_clean` —
/// the CT MIRROR. When the mirror lagged iHOTEL, a maid saw DIRTY (iHOTEL
/// truth), tapped เสร็จแล้ว, the guard read the mirror, said "already clean",
/// no-opped, and answered บันทึกแล้ว — while NOTHING reached iHOTEL and the
/// room stayed dirty on reception's board. Read and write judged two different
/// databases. This carries the read's answer into the write.
///
/// ## Deliberately a plain enum
///
/// No tiberius, no [`crate::legacy_room_status`] type: the service stays PG +
/// outbox only (invariant #1). `routes::hk` performs the read (outside the PG
/// transaction, on its own 3s budget) and hands the answer down as data.
///
/// [`Unknown`](Self::Unknown) is the FALLBACK and the default: no reader for
/// the branch, legacy unreachable, budget elapsed, room absent from the answer,
/// unrecognised `Room_Clean` literal, or the read deliberately skipped. It
/// reproduces today's canonical-only behaviour exactly — a maid's tap is never
/// blocked, delayed past its budget, or failed because a legacy server blinked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacyCleanliness {
    /// No usable answer ⇒ judge on canonical alone (today's behaviour).
    #[default]
    Unknown,
    /// iHOTEL says the room IS clean (`Room_Clean='no'`).
    Clean,
    /// iHOTEL says the room needs cleaning (`Room_Clean='yes'`).
    Dirty,
}

impl LegacyCleanliness {
    /// Canonical polarity (`true` = IS clean), or `None` when unknown.
    pub fn is_clean(self) -> Option<bool> {
        match self {
            LegacyCleanliness::Unknown => None,
            LegacyCleanliness::Clean => Some(true),
            LegacyCleanliness::Dirty => Some(false),
        }
    }
}

/// What one `done`/`dirty` tap should actually do, decided under the room lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanlinessDecision {
    /// Canonical is not yet at the target state: flip it and mirror it. The
    /// pre-D1 behaviour, unchanged.
    Transition,
    /// Canonical already matches the target, but iHOTEL still disagrees —
    /// the mirror-lag case. Nothing to flip; enqueue the writeback so the tap
    /// actually reaches the board reception works from.
    MirrorRepair,
    /// Both truths already agree with the target. Nothing to do.
    NoOp,
}

/// The D1 guard, as a pure function of the two truths and the target state.
///
/// `target_clean` is `true` for `done`, `false` for `dirty`. `canonical` is
/// `ht_rooms_new.room_clean` READ UNDER THE ROW LOCK (`None` = SQL NULL).
///
/// Three rules, and the order matters:
///
/// 1. Canonical not at the target ⇒ [`Transition`](CleanlinessDecision::Transition).
///    Identical to the old `room_clean IS DISTINCT FROM <target>` predicate,
///    NULL included: an unknown cleanliness must reach iHOTEL explicitly.
/// 2. Canonical at the target but iHOTEL says the OPPOSITE ⇒
///    [`MirrorRepair`](CleanlinessDecision::MirrorRepair). This is the whole
///    defect: the maid acted on what iHOTEL told her, so the write must go.
/// 3. Anything else — the two agree, or iHOTEL has no usable answer ⇒
///    [`NoOp`](CleanlinessDecision::NoOp). `Unknown` lands here on purpose:
///    an unreachable legacy degrades to EXACTLY today's behaviour, never to
///    "enqueue anyway" (that would spam `HT_Housewife` during an outage) and
///    never to an error (a maid's tap must not depend on a legacy server).
pub fn decide_cleanliness(
    target_clean: bool,
    canonical: Option<bool>,
    legacy: LegacyCleanliness,
) -> CleanlinessDecision {
    if canonical != Some(target_clean) {
        return CleanlinessDecision::Transition;
    }
    if legacy.is_clean() == Some(!target_clean) {
        return CleanlinessDecision::MirrorRepair;
    }
    CleanlinessDecision::NoOp
}

/// Command for [`HousekeepingService::report_cleaning_progress`] — one maid tap
/// on `POST /api/hk/rooms/{id}/cleaning`.
#[derive(Debug, Clone)]
pub struct ReportCleaningCommand {
    pub room_id: i32,
    pub status: CleaningProgressStatus,
    /// iHOTEL's answer for THIS room at tap time (defect D1). Default
    /// [`LegacyCleanliness::Unknown`] = judge on canonical alone, i.e. the
    /// pre-D1 behaviour. Ignored for `started` (legacy-inert).
    pub legacy_room_clean: LegacyCleanliness,
    /// Verified HF ID badge — ALWAYS present (the Access middleware 401s
    /// without one). Stamped into `hkev_badge`.
    pub badge: String,
    /// Display-name snapshot for `hkev_name` (usually `None` today — the CF IdP
    /// forwards only `apps` + `badge`).
    pub name: Option<String>,
    /// Housekeeper label carried by the writeback intent into
    /// `HT_Housewife.h_name` (display name, badge fallback — `routes::hk`
    /// clamps it to the legacy `varchar(150)`).
    pub by: String,
    pub source: EventSource,
}

/// Inclusive quantity bounds for ONE linen-shortage line.
///
/// Mirrored by the `CHECK (hklr_qty >= 1 AND hklr_qty <= 20)` in migration 088
/// and re-stated by `routes::hk` so a bad body is a 400, not a 500 from the
/// constraint. Kept here (not only in the route) because
/// [`HousekeepingService::report_linen_shortage`] must hold for ANY caller.
pub const MIN_LINEN_QTY: i32 = 1;
pub const MAX_LINEN_QTY: i32 = 20;

/// One line of a linen-shortage report: "I am short N of this kind".
///
/// `kind` is an already-validated code from `routes::hk::VALID_LINEN_KINDS`.
/// The service does NOT re-check the vocabulary — the DB column is
/// unconstrained `TEXT` on purpose (migration 088) so the allowlist can move
/// without a migration, and duplicating it here would recreate exactly the
/// two-places-to-change coupling that decision avoids.
#[derive(Debug, Clone)]
pub struct LinenShortageItem {
    pub kind: String,
    pub qty: i32,
}

/// Command for [`HousekeepingService::report_linen_shortage`] — one maid
/// submission on `POST /api/hk/rooms/{id}/linen-shortage`.
#[derive(Debug, Clone)]
pub struct ReportLinenShortageCommand {
    pub room_id: i32,
    /// One entry per kind, already deduplicated and validated by the route.
    pub items: Vec<LinenShortageItem>,
    /// Verified HF ID badge — ALWAYS present (the Access middleware 401s
    /// without one). Stamped into `hklr_badge`.
    pub badge: String,
    /// Display-name snapshot for `hklr_name`.
    pub name: Option<String>,
    // Deliberately NO `source: EventSource`: this command publishes no domain
    // event and enqueues no writeback, so an event source would be a field that
    // only ever gets discarded — and a misleading hint that one day it won't be.
}

/// Outcome of [`HousekeepingService::report_linen_shortage`].
#[derive(Debug, Clone)]
pub struct LinenShortageReport {
    pub room_id: i32,
    /// The server-minted `hklr_report_uuid` shared by every row this call wrote.
    pub report_uuid: Uuid,
    /// How many item rows were actually inserted (== `items.len()`).
    pub reported: usize,
}

/// Outcome of [`HousekeepingService::report_cleaning_progress`].
#[derive(Debug, Clone)]
pub struct CleaningReport {
    pub room_id: i32,
    /// `ht_hk_cleaning_events.hkev_id` of the row this call appended.
    pub event_id: i64,
    /// `true` iff this call performed the cleanliness transition and therefore
    /// enqueued a writeback job. ENQUEUED, NOT DELIVERED. Always `false` for
    /// `started` (legacy-inert) and for a repeat that changed nothing.
    pub writeback_enqueued: bool,
    /// Room signals this เสร็จแล้ว report auto-completed (ADR 0008) — the
    /// room's live ทำห้องนี้ก่อน / แขกเช็คเอาท์แล้ว, closed in THIS call's
    /// transaction with `done_source = 'clean_report'`. Empty for `started` /
    /// `dirty` and whenever the room had none.
    ///
    /// Not rendered in the `/hk` cleaning response: both boards already learn
    /// about the closures from the `RoomSignalCompleted` frames this call
    /// published, and adding them to that response would give the maid surface
    /// two sources of truth for the same fact. It is here so the closure is
    /// assertable from a test without reading the table back.
    pub auto_completed_signals: Vec<crate::domain::hk_signal::RoomSignal>,
}

/// Service handle for the housekeeping aggregate.
///
/// `events` Arc is held for Wave 4 — see [`super::customer`] note.
#[derive(Clone)]
pub struct HousekeepingService {
    pub(crate) repo: Arc<dyn RoomRepository>,
    pub(crate) outbox: Arc<OutboxRepository>,
    #[allow(dead_code)]
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl HousekeepingService {
    pub fn new(
        repo: Arc<dyn RoomRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg }
    }

    /// Mark a room clean — spike §3j.
    pub async fn mark_clean(
        &self,
        cmd: MarkCleanCommand,
    ) -> ServiceResult<HousekeepingOutcome> {
        if cmd.by.trim().is_empty() {
            return Err(ServiceError::validation(
                "mark_clean requires a non-empty `by` housekeeper identifier",
            ));
        }

        let mut tx = self.pg.begin().await?;

        set_room_clean_flag(&mut tx, cmd.room_id, true).await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);
        let intent = WritebackIntent::MarkRoomClean {
            room_id: aggregate_id,
            by: cmd.by.clone(),
        };
        // Repeatable-per-aggregate intent: the SECOND occurrence for the
        // same aggregate would collide on the permanently-retained
        // `writeback_jobs.idempotency_key` UNIQUE if we used the
        // deterministic (intent, aggregate) key — completed jobs stay as
        // status='done' rows. Per-event v4 discriminator instead (same
        // precedent as payment/customer-update; see outbox/idempotency.rs
        // "caller adds a discriminator"). 2026-06-12 audit follow-up.
        let key = generate_idempotency_key(&intent, uuid::Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let event = DomainEvent::RoomMarkedClean {
            room_id: aggregate_id,
            by: cmd.by,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        // Field touch — `repo` is held for symmetry with other services and
        // for Wave 4 when room-status reads happen behind a service method.
        let _ = &self.repo;

        tx.commit().await?;

        Ok(HousekeepingOutcome {
            room_id: cmd.room_id,
            aggregate_id,
        })
    }

    /// Mark a room clean ONLY if it is currently dirty — the idempotent
    /// variant of [`mark_clean`](Self::mark_clean), for callers that may
    /// legitimately fire the same "done" twice.
    ///
    /// Returns `Ok(None)` when the room was ALREADY clean: nothing was
    /// flipped, NO writeback was enqueued and NO event published. `Ok(Some(_))`
    /// means this call performed the transition.
    ///
    /// ## Why this exists (housekeeping-ops, 2026-08-11)
    ///
    /// `mark_clean` deliberately uses a per-event v4 idempotency-key
    /// discriminator (see its comment), so EVERY call enqueues its own job —
    /// correct for the front desk, where a re-clean is a real event. The `/hk`
    /// maid surface is different: a maid can tap เสร็จแล้ว twice, or retry on a
    /// flaky phone connection, and each extra tap must NOT produce another
    /// `HT_Housewife` audit row in iHOTEL.
    ///
    /// ## Why the guard is in SQL, not a read-then-write
    ///
    /// The `WHERE … room_clean IS DISTINCT FROM true` predicate makes the
    /// check and the flip ONE statement inside the transaction, so it takes
    /// the row lock atomically. Two concurrent "done" taps therefore serialize:
    /// the loser sees `rows_affected() == 0` and returns `Ok(None)` without
    /// enqueuing. A read-then-write would let both taps observe `false` and
    /// double-enqueue. (The recipe's 5-minute `WHERE NOT EXISTS` dedup window
    /// is a legacy-side backstop, not a substitute for this.)
    pub async fn mark_clean_if_dirty(
        &self,
        cmd: MarkCleanCommand,
    ) -> ServiceResult<Option<HousekeepingOutcome>> {
        if cmd.by.trim().is_empty() {
            return Err(ServiceError::validation(
                "mark_clean_if_dirty requires a non-empty `by` housekeeper identifier",
            ));
        }

        let mut tx = self.pg.begin().await?;

        // Conditional flip: 0 rows means either "already clean" or "no such
        // room" — `room_exists` disambiguates so an unknown room still 404s
        // instead of silently reporting a no-op success.
        let flipped = set_room_clean_flag_if_dirty(&mut tx, cmd.room_id).await?;
        if !flipped {
            if !room_exists(&mut tx, cmd.room_id).await? {
                return Err(ServiceError::not_found(format!(
                    "room {} does not exist",
                    cmd.room_id
                )));
            }
            // Already clean — commit the (empty) transaction and report the
            // no-op. No writeback, no event: invariant #4 (a repeat must not
            // double-write to legacy).
            tx.commit().await?;
            return Ok(None);
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);
        let intent = WritebackIntent::MarkRoomClean {
            room_id: aggregate_id,
            by: cmd.by.clone(),
        };
        // Same per-event v4 discriminator as `mark_clean`: a room is
        // legitimately cleaned again on later days, and
        // `writeback_jobs.idempotency_key` is permanently UNIQUE. The
        // dirty→clean guard above (not the key) is what makes the REPEAT
        // within one cleaning idempotent.
        let key = generate_idempotency_key(&intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let event = DomainEvent::RoomMarkedClean {
            room_id: aggregate_id,
            by: cmd.by,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let _ = (&self.repo, &self.outbox);

        tx.commit().await?;

        Ok(Some(HousekeepingOutcome {
            room_id: cmd.room_id,
            aggregate_id,
        }))
    }

    /// Record ONE maid cleaning-progress tap — the `/hk` surface's single
    /// write path (`POST /api/hk/rooms/{id}/cleaning`).
    ///
    /// ## Why this exists (wave-4, housekeeping stream B3)
    ///
    /// `routes::hk::report_cleaning` used to INSERT the append-only event with
    /// a bare pool query and only THEN call the service, which opened its own
    /// transaction. The module documented the consequence itself: "a client
    /// retry after a 5xx can append a SECOND row". This method closes that by
    /// doing the whole tap — event row, conditional flag flip, writeback
    /// enqueue, domain event — inside ONE transaction (invariants #1 + #2).
    ///
    /// ## Per-status behaviour
    ///
    /// | status | canonical flip | writeback | domain event |
    /// |---|---|---|---|
    /// | `started` | none | none | `RoomCleaningStarted` |
    /// | `done` | `room_clean → true` when not already true | `MarkRoomClean` when canonical OR iHOTEL said dirty | `RoomMarkedClean` |
    /// | `dirty` | `room_clean → false` when not already false | `MarkRoomDirty` when canonical OR iHOTEL said clean | `RoomMarkedDirty` |
    ///
    /// ## Which truth the guard judges against (defect D1, wave-5)
    ///
    /// `done`/`dirty` no longer judge canonical alone. The tap is decided by
    /// [`decide_cleanliness`] over BOTH truths — canonical under the row lock,
    /// and [`ReportCleaningCommand::legacy_room_clean`], iHOTEL's answer for
    /// this room as `routes::hk` read it (the same answer the maid's screen was
    /// rendered from). A canonical mirror that lags iHOTEL used to swallow the
    /// tap silently; now it produces a `MirrorRepair` — no flip (canonical is
    /// already at the target) but a real writeback, so the room actually clears
    /// on reception's board.
    ///
    /// `LegacyCleanliness::Unknown` (unreachable, no reader, unmapped room, or
    /// the read skipped) degrades to EXACTLY the pre-D1 behaviour.
    ///
    /// ## Idempotency (invariant #4)
    ///
    /// The guard opens with `SELECT … FOR UPDATE` on the room row — the SAME
    /// row lock the old conditional UPDATE took, acquired one statement earlier
    /// so the decision can consider two truths instead of one. Two concurrent
    /// taps still serialize on it: the loser blocks, then re-reads the winner's
    /// committed row (READ COMMITTED re-evaluates a locked row after the lock is
    /// granted) and decides against the NEW state. The read-then-write hazard the
    /// old comment warned about is a hazard only WITHOUT the lock.
    ///
    /// A repair additionally passes [`mirror_repair_suppressed`]: iHOTEL keeps
    /// saying "dirty" until our own job drains, so the second tap of a
    /// double-tap is deduplicated against the job the first one enqueued
    /// (whether that job came from a transition or a repair). Net effect: at
    /// most one writeback per real transition, exactly as before.
    ///
    /// The per-event v4 discriminator stays on the idempotency key: rooms
    /// legitimately flip every day and `writeback_jobs.idempotency_key` is
    /// permanently UNIQUE.
    ///
    /// The APPEND-ONLY event row is always written — `started`/`done` may
    /// legitimately repeat within a day and the "current progress" read takes
    /// only the LATEST row. What must not double is the legacy write, and that
    /// is what the guard above protects.
    ///
    /// The route resolves 404 for unknown/inactive rooms before calling; the
    /// locking read here re-checks existence, so a row that disappears between
    /// the two still 404s instead of reporting a silent success.
    pub async fn report_cleaning_progress(
        &self,
        cmd: ReportCleaningCommand,
    ) -> ServiceResult<CleaningReport> {
        if cmd.badge.trim().is_empty() {
            return Err(ServiceError::validation(
                "report_cleaning_progress requires a non-empty verified badge",
            ));
        }
        if cmd.by.trim().is_empty() {
            return Err(ServiceError::validation(
                "report_cleaning_progress requires a non-empty `by` housekeeper identifier",
            ));
        }

        let mut tx = self.pg.begin().await?;

        // 1. The maid's own record of work done — append-only, unconditional.
        let event_id = insert_cleaning_event(
            &mut tx,
            cmd.room_id,
            cmd.status.as_str(),
            &cmd.badge,
            cmd.name.as_deref(),
        )
        .await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);

        // 1b. ADR 0008: a maid's เสร็จแล้ว report auto-completes that room's
        // live cleaning-urgency signals (ทำห้องนี้ก่อน, แขกเช็คเอาท์แล้ว),
        // recorded as completed by her report. IN THIS TRANSACTION — the
        // closure must not be able to survive a rolled-back cleaning report,
        // nor be lost when one commits.
        //
        // Placed HERE, before the status branch, deliberately: every early
        // return below (`started`, an idempotent no-op, a suppressed mirror
        // repair) still commits this tx, and a เสร็จแล้ว tap on an ALREADY-clean
        // room is exactly the case where the signals are most likely to still
        // be open. Gating the sweep on the cleanliness flip would strand them.
        //
        // ขอเช็คห้อง is deliberately NOT swept — see
        // `domain::hk_signal::CLEAN_REPORT_AUTO_COMPLETE_TYPES`.
        let auto_completed = if cmd.status == CleaningProgressStatus::Done {
            crate::service::hk_signals::auto_complete_clean_report(
                &mut tx,
                cmd.room_id,
                &cmd.badge,
                cmd.name.as_deref(),
                cmd.source.clone(),
            )
            .await?
        } else {
            Vec::new()
        };

        // 2. Branch on status: only the two terminal states touch cleanliness.
        let (intent, event) = match cmd.status {
            CleaningProgressStatus::Started => {
                // Legacy-inert on purpose (iHOTEL's Room_Clean_Time drives its
                // room-power countdown). Publish so reception's board lights up
                // live over SSE, then commit.
                let event = DomainEvent::RoomCleaningStarted {
                    room_id: aggregate_id,
                    by: cmd.by,
                    source: cmd.source,
                };
                EventBus::publish(&mut tx, &event)
                    .await
                    .map_err(|err| ServiceError::outbox(err.to_string()))?;
                tx.commit().await?;
                return Ok(CleaningReport {
                    room_id: cmd.room_id,
                    event_id,
                    writeback_enqueued: false,
                    auto_completed_signals: auto_completed.clone(),
                });
            }
            // `done` and `dirty` are ONE path with opposite polarity — the D1
            // guard is symmetric, so writing it twice is how the two poles
            // drift apart.
            CleaningProgressStatus::Done | CleaningProgressStatus::Dirty => {
                let target_clean = matches!(cmd.status, CleaningProgressStatus::Done);

                // Serialization point. Everything below decides against the
                // value read HERE, under the lock.
                let Some(canonical) = lock_room_clean(&mut tx, cmd.room_id).await? else {
                    return Err(ServiceError::not_found(format!(
                        "room {} does not exist",
                        cmd.room_id
                    )));
                };

                let (intent, event) = if target_clean {
                    (
                        WritebackIntent::MarkRoomClean {
                            room_id: aggregate_id,
                            by: cmd.by.clone(),
                        },
                        DomainEvent::RoomMarkedClean {
                            room_id: aggregate_id,
                            by: cmd.by.clone(),
                            source: cmd.source.clone(),
                        },
                    )
                } else {
                    (
                        WritebackIntent::MarkRoomDirty {
                            room_id: aggregate_id,
                            by: cmd.by.clone(),
                        },
                        DomainEvent::RoomMarkedDirty {
                            room_id: aggregate_id,
                            source: cmd.source.clone(),
                        },
                    )
                };

                match decide_cleanliness(target_clean, canonical, cmd.legacy_room_clean) {
                    CleanlinessDecision::NoOp => {
                        // Both truths already agree with the target: keep the
                        // maid's event row, enqueue nothing, publish nothing.
                        tx.commit().await?;
                        return Ok(CleaningReport {
                            room_id: cmd.room_id,
                            event_id,
                            writeback_enqueued: false,
                            auto_completed_signals: auto_completed.clone(),
                        });
                    }
                    CleanlinessDecision::Transition => {
                        let flipped = if target_clean {
                            set_room_clean_flag_if_dirty(&mut tx, cmd.room_id).await?
                        } else {
                            set_room_clean_flag_if_clean(&mut tx, cmd.room_id).await?
                        };
                        if !flipped {
                            // Unreachable while we hold the row lock (nothing
                            // else can have moved the row since the read). If it
                            // ever happens, degrade to the pre-D1 no-op rather
                            // than enqueue a write we can no longer justify.
                            tx.commit().await?;
                            return Ok(CleaningReport {
                                room_id: cmd.room_id,
                                event_id,
                                writeback_enqueued: false,
                                auto_completed_signals: auto_completed.clone(),
                            });
                        }
                    }
                    CleanlinessDecision::MirrorRepair => {
                        if mirror_repair_suppressed(&mut tx, aggregate_id, intent.intent_name())
                            .await?
                        {
                            // Our own earlier writeback for this room has not
                            // reached iHOTEL yet — that, not a lagging mirror,
                            // is why legacy still disagrees. Do not double-write.
                            tx.commit().await?;
                            return Ok(CleaningReport {
                                room_id: cmd.room_id,
                                event_id,
                                writeback_enqueued: false,
                                auto_completed_signals: auto_completed.clone(),
                            });
                        }
                        // Operator signal, same family as `routes::hk`'s
                        // divergence warn: N of these = the CT mirror was behind
                        // iHOTEL at the moment a maid acted, and the tap would
                        // have been silently lost before wave-5 D1.
                        tracing::warn!(
                            room_id = cmd.room_id,
                            target_clean,
                            intent = intent.intent_name(),
                            "/hk mirror repair: canonical already matched the tap but iHOTEL \
                             disagreed — enqueuing the writeback the maid's tap earned (D1)"
                        );
                    }
                }

                (intent, event)
            }
        };

        let key = generate_idempotency_key(&intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let _ = (&self.repo, &self.outbox, &self.events);

        tx.commit().await?;

        Ok(CleaningReport {
            room_id: cmd.room_id,
            event_id,
            writeback_enqueued: true,
            auto_completed_signals: auto_completed,
        })
    }

    /// Mark a room dirty — flip `room_clean=false`, enqueue the
    /// [`WritebackIntent::MarkRoomDirty`] mirror (cheatsheet §3.13), and
    /// publish the event — all in one transaction.
    ///
    /// Audit 2026-06-11 P2 gap-close: this used to be PG-only ("the legacy
    /// app derives dirty from checkout" — spike §3e), but iHOTEL's grid reads the
    /// `HT_Rooms.Room_Clean` flag directly — a standalone mark-dirty in
    /// the new app left the room rendered clean in iHOTEL.
    pub async fn mark_dirty(
        &self,
        cmd: MarkDirtyCommand,
    ) -> ServiceResult<HousekeepingOutcome> {
        if cmd.by.trim().is_empty() {
            return Err(ServiceError::validation(
                "mark_dirty requires a non-empty `by` operator identifier",
            ));
        }

        let mut tx = self.pg.begin().await?;

        set_room_clean_flag(&mut tx, cmd.room_id, false).await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);
        let intent = WritebackIntent::MarkRoomDirty {
            room_id: aggregate_id,
            by: cmd.by.clone(),
        };
        // Per-event discriminator key: rooms flip dirty↔clean every day,
        // and `writeback_jobs.idempotency_key` is permanently UNIQUE — a
        // payload-independent `(intent, aggregate)` key would hard-fail
        // the second mark-dirty of the same room after the first job
        // completed. The enqueue commits atomically with the flag flip in
        // this TX, so there is no double-submit window for the key to
        // guard (see the discriminator note in `outbox::idempotency`).
        let key = generate_idempotency_key(&intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let event = DomainEvent::RoomMarkedDirty {
            room_id: aggregate_id,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let _ = (&self.repo, &self.outbox);

        tx.commit().await?;

        Ok(HousekeepingOutcome {
            room_id: cmd.room_id,
            aggregate_id,
        })
    }

    /// Toggle a room's maintenance (out-of-service) flag — flip
    /// `ht_rooms_new.room_maintenance` (and the canonical `room_status`
    /// projection so the two never disagree) and enqueue the
    /// [`WritebackIntent::SetRoomMaintenance`] mirror (cheatsheet §3.15/§3.16),
    /// all in one transaction.
    ///
    /// Mirrors the maintenance block in `routes::new_rooms::update_room`, but
    /// exposed as a standalone toggle the room grid can call without re-PUTting
    /// the whole room record (which would risk blanking the room type / prices).
    /// Before this method, the front-desk "out of service" toggle hit
    /// `PATCH /api/rooms/:id/status` (canonical `room_status` only, no
    /// writeback) so iHOTEL never saw the flag and kept renting the room.
    pub async fn set_maintenance(
        &self,
        cmd: MarkMaintenanceCommand,
    ) -> ServiceResult<HousekeepingOutcome> {
        let mut tx = self.pg.begin().await?;

        set_room_maintenance_flag(&mut tx, cmd.room_id, cmd.maintenance).await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);
        let intent = WritebackIntent::SetRoomMaintenance {
            room_id: aggregate_id,
            maintenance: cmd.maintenance,
        };
        // Per-event discriminator key: a room legitimately goes in and out of
        // maintenance over its lifetime and `writeback_jobs.idempotency_key` is
        // permanently UNIQUE — same rationale as `mark_dirty` above. The
        // enqueue commits atomically with the flag flip in this TX.
        let key = generate_idempotency_key(&intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        // No `DomainEvent` for maintenance (none exists; `update_room` likewise
        // emits the writeback without one). `source` is carried for forward
        // compat; touch the held handles for symmetry with the other methods.
        let _ = (&self.repo, &self.outbox, &self.events, &cmd.source);

        tx.commit().await?;

        Ok(HousekeepingOutcome {
            room_id: cmd.room_id,
            aggregate_id,
        })
    }

    /// Record ONE maid linen-shortage report (ขาดผ้า) — migration 088,
    /// `POST /api/hk/rooms/{id}/linen-shortage`.
    ///
    /// ## RECORD-ONLY, and deliberately unlike every other method here
    ///
    /// Every sibling on this service exists to move canonical state and mirror
    /// it to iHOTEL. This one does NOT, and the difference is the design:
    ///
    /// * no `ht_rooms_new` column is touched — a shortage of pillowcases is not
    ///   a cleanliness or occupancy fact;
    /// * NO [`WritebackIntent`] is enqueued and NO [`DomainEvent`] is published.
    ///   That is not a dark-ship pending verification (invariant #6) — it is
    ///   terminal. iHOTEL has no linen-inventory table, so there is nothing on
    ///   the legacy side to write and nothing for a later flag to enable;
    /// * no notification of any kind. The rows ARE the feature.
    ///
    /// Adding an outbox row here later would be a real design change, not a
    /// fill-in-the-blank — it would need a legacy counterpart to exist first.
    ///
    /// ## Shape
    ///
    /// One row per (submission, kind), all sharing `hklr_report_uuid` — minted
    /// HERE (uuid v4), never accepted from the client, so a replayed body cannot
    /// merge into or overwrite an earlier submission. Rows are inserted by a
    /// single `UNNEST` statement inside ONE transaction, so a partial submission
    /// can never be observed: either every kind the maid reported lands, or none
    /// does.
    ///
    /// ## Validation split
    ///
    /// The kind ALLOWLIST is the route's (`routes::hk::VALID_LINEN_KINDS`) —
    /// the DB column is deliberately unconstrained `TEXT` so a new kind needs no
    /// migration. What is re-checked here is only what must hold for ANY caller
    /// of this method: a verified badge, a non-empty item list, and the same
    /// 1..=20 quantity bound the DB `CHECK` enforces. This method is not a
    /// second gate on the product's kind vocabulary and must not grow into one.
    ///
    /// There is no idempotency key and no dedup: two identical reports are two
    /// real reports (a maid can be short of the same linen twice in a day), and
    /// with nothing crossing to legacy there is no double-write to prevent.
    pub async fn report_linen_shortage(
        &self,
        cmd: ReportLinenShortageCommand,
    ) -> ServiceResult<LinenShortageReport> {
        if cmd.badge.trim().is_empty() {
            return Err(ServiceError::validation(
                "report_linen_shortage requires a non-empty verified badge",
            ));
        }
        if cmd.items.is_empty() {
            return Err(ServiceError::validation(
                "report_linen_shortage requires at least one linen item",
            ));
        }
        if let Some(bad) = cmd
            .items
            .iter()
            .find(|item| !(MIN_LINEN_QTY..=MAX_LINEN_QTY).contains(&item.qty))
        {
            return Err(ServiceError::validation(format!(
                "linen quantity {} for '{}' is outside {MIN_LINEN_QTY}..={MAX_LINEN_QTY}",
                bad.qty, bad.kind
            )));
        }

        let report_uuid = Uuid::new_v4();

        let mut tx = self.pg.begin().await?;
        let reported = insert_linen_report_rows(
            &mut tx,
            report_uuid,
            cmd.room_id,
            &cmd.items,
            &cmd.badge,
            cmd.name.as_deref(),
        )
        .await?;

        // No enqueue, no publish — see the doc comment. The handles are held
        // for symmetry with the other methods on this service.
        let _ = (&self.repo, &self.outbox, &self.events);

        tx.commit().await?;

        Ok(LinenShortageReport {
            room_id: cmd.room_id,
            report_uuid,
            reported,
        })
    }
}

/// Set `ht_rooms_new.room_clean` directly via dynamic SQL.
///
/// We cannot route this through the existing `RoomRepository` because none of
/// its methods accept a clean-only update. Wave 4 will widen the trait to
/// add a dedicated method; today the inline UPDATE preserves the exact
/// semantic (no other column mutated).
async fn set_room_clean_flag(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    is_clean: bool,
) -> ServiceResult<()> {
    let result = sqlx::query("UPDATE ht_rooms_new SET room_clean = $1, updated_at = NOW() WHERE room_id = $2")
        .bind(is_clean)
        .bind(room_id)
        .execute(&mut **tx)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found(format!(
            "room {room_id} does not exist"
        )));
    }
    Ok(())
}

/// Flip `ht_rooms_new.room_clean` to `true` ONLY when it is not already
/// `true`. Returns whether this statement performed the transition.
///
/// `IS DISTINCT FROM true` (not `= false`) so the NULL case — `room_clean`
/// is a nullable column and legacy-mirrored rows can carry NULL — counts as
/// dirty and gets flipped, matching the read path's
/// `COALESCE(r.room_clean, true)`… inverted deliberately: an UNKNOWN
/// cleanliness must be treated as "needs the writeback", never silently
/// swallowed as "already clean".
async fn set_room_clean_flag_if_dirty(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
) -> ServiceResult<bool> {
    let result = sqlx::query(
        "UPDATE ht_rooms_new SET room_clean = true, updated_at = NOW() \
          WHERE room_id = $1 AND room_clean IS DISTINCT FROM true",
    )
    .bind(room_id)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Flip `ht_rooms_new.room_clean` to `false` ONLY when it is not already
/// `false`. Returns whether this statement performed the transition — the
/// mirror image of [`set_room_clean_flag_if_dirty`], and the mark-DIRTY half of
/// the `/hk` idempotency guard.
///
/// `IS DISTINCT FROM false` (not `= true`) so a NULL `room_clean` — legacy-
/// mirrored rows can carry one — counts as "not yet dirty" and IS flipped. An
/// UNKNOWN cleanliness must reach iHOTEL as an explicit `Room_Clean='yes'`,
/// never be swallowed as "already dirty".
async fn set_room_clean_flag_if_clean(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
) -> ServiceResult<bool> {
    let result = sqlx::query(
        "UPDATE ht_rooms_new SET room_clean = false, updated_at = NOW() \
          WHERE room_id = $1 AND room_clean IS DISTINCT FROM false",
    )
    .bind(room_id)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Append one maid cleaning-progress event inside the caller's transaction.
/// Returns the new `hkev_id`.
///
/// Moved here from `routes::hk` (wave-4 B3) so the event row commits atomically
/// with the flag flip + writeback enqueue it belongs to. Runtime `sqlx::query`
/// — no `.sqlx/` cache regeneration.
async fn insert_cleaning_event(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    status: &str,
    badge: &str,
    name: Option<&str>,
) -> ServiceResult<i64> {
    let row = sqlx::query(
        "INSERT INTO ht_hk_cleaning_events (hkev_room_id, hkev_status, hkev_badge, hkev_name) \
         VALUES ($1, $2, $3, $4) RETURNING hkev_id",
    )
    .bind(room_id)
    .bind(status)
    .bind(badge)
    .bind(name)
    .fetch_one(&mut **tx)
    .await?;
    Ok(sqlx::Row::try_get(&row, "hkev_id")?)
}

/// Insert every line of ONE linen-shortage submission — migration 088.
///
/// A single `UNNEST` statement rather than a loop of INSERTs: one round-trip,
/// and the all-or-nothing property is a property of the statement itself, not
/// of remembering to keep the loop inside the transaction. Returns the number
/// of rows written, which the handler reports as `reported`.
///
/// The arrays are bound as parameters (`$5::text[]`, `$6::int[]`) — nothing
/// about a maid's report is ever concatenated into SQL.
async fn insert_linen_report_rows(
    tx: &mut Transaction<'_, Postgres>,
    report_uuid: Uuid,
    room_id: i32,
    items: &[LinenShortageItem],
    badge: &str,
    name: Option<&str>,
) -> ServiceResult<usize> {
    let kinds: Vec<String> = items.iter().map(|item| item.kind.clone()).collect();
    let quantities: Vec<i32> = items.iter().map(|item| item.qty).collect();

    let result = sqlx::query(
        "INSERT INTO ht_hk_linen_reports \
             (hklr_report_uuid, hklr_room_id, hklr_kind, hklr_qty, hklr_badge, hklr_name) \
         SELECT $1, $2, line.kind, line.qty, $3, $4 \
           FROM UNNEST($5::text[], $6::int[]) AS line(kind, qty)",
    )
    .bind(report_uuid)
    .bind(room_id)
    .bind(badge)
    .bind(name)
    .bind(&kinds)
    .bind(&quantities)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() as usize)
}

/// Read `ht_rooms_new.room_clean` for one room AND take its row lock — the
/// serialization point of the D1 guard.
///
/// `Ok(None)` = no such room (the caller 404s, same disambiguation
/// `room_exists` gives `mark_clean_if_dirty`). `Ok(Some(None))` = the row
/// exists with a NULL `room_clean`, which
/// [`decide_cleanliness`] treats as "not at the target" in both directions.
///
/// `FOR UPDATE` is what keeps the guard idempotent now that the decision spans
/// two statements: a second concurrent tap blocks here until the first commits,
/// then (READ COMMITTED) re-reads the row it just wrote. Same lock the old
/// conditional UPDATE took — no new lock is introduced, only taken earlier.
async fn lock_room_clean(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
) -> ServiceResult<Option<Option<bool>>> {
    let row = sqlx::query("SELECT room_clean FROM ht_rooms_new WHERE room_id = $1 FOR UPDATE")
        .bind(room_id)
        .fetch_optional(&mut **tx)
        .await?;
    match row {
        Some(row) => Ok(Some(sqlx::Row::try_get::<Option<bool>, _>(
            &row,
            "room_clean",
        )?)),
        None => Ok(None),
    }
}

/// Whether a MIRROR REPAIR for this room + intent must be suppressed because we
/// already have a writeback in flight (or a very recent one) that explains why
/// iHOTEL still disagrees.
///
/// Two halves, both needed:
///
/// * `status IN ('pending','in_progress')` — the causal case. A queued job IS
///   the reason legacy still reads dirty; enqueuing another would put a second
///   `HT_Housewife` row in for one cleaning.
/// * `created_at > now() - 5 minutes` — the bounded case. If the queue is stuck
///   or a job parked/failed, repeated taps must not pile up jobs; one per
///   window per room is the ceiling.
///
/// Called ONLY on the repair path — a genuine canonical transition is never
/// suppressed, so a legitimate re-clean after the room went dirty again still
/// enqueues immediately. Runs after [`lock_room_clean`], so under READ COMMITTED
/// its snapshot already includes anything a serialized rival tap committed.
async fn mirror_repair_suppressed(
    tx: &mut Transaction<'_, Postgres>,
    aggregate_id: Uuid,
    intent_name: &str,
) -> ServiceResult<bool> {
    let row = sqlx::query(
        "SELECT 1 FROM writeback_jobs \
          WHERE aggregate_id = $1 \
            AND intent = $2 \
            AND (status IN ('pending', 'in_progress') \
                 OR created_at > now() - make_interval(mins => $3)) \
          LIMIT 1",
    )
    .bind(aggregate_id)
    .bind(intent_name)
    .bind(MIRROR_REPAIR_DEDUP_MINUTES)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(row.is_some())
}

/// Existence probe used to distinguish "already clean" from "no such room"
/// after a conditional flip affected 0 rows.
async fn room_exists(tx: &mut Transaction<'_, Postgres>, room_id: i32) -> ServiceResult<bool> {
    let row = sqlx::query("SELECT 1 FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(row.is_some())
}

/// Flip `ht_rooms_new.room_maintenance` and keep the canonical `room_status`
/// projection consistent with it via dynamic SQL.
///
/// Both columns are written so the live room-status derivation
/// (`routes::new_rooms::list_rooms_live`, which treats either
/// `room_maintenance=true` OR `room_status='maintenance'` as out-of-service)
/// can never read a stale `room_status='maintenance'` left behind by the older
/// `PATCH /status` toggle once the flag is cleared. When returning to service
/// the stored status is reset to `available`; the live derivation re-overlays
/// occupancy / booking / checkout, so this is a safe neutral value.
async fn set_room_maintenance_flag(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    maintenance: bool,
) -> ServiceResult<()> {
    let result = sqlx::query(
        "UPDATE ht_rooms_new \
            SET room_maintenance = $1, \
                room_status = CASE WHEN $1 THEN 'maintenance' ELSE 'available' END, \
                updated_at = NOW() \
          WHERE room_id = $2",
    )
    .bind(maintenance)
    .bind(room_id)
    .execute(&mut **tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found(format!(
            "room {room_id} does not exist"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Tests for the `MarkRoomDirty` writeback emission (audit 2026-06-11
    //! P2). DB-backed tests skip gracefully when no local PG is reachable
    //! — same `try_pool` convention as `service::checkin::tests`.

    use super::*;
    use crate::repository::room::PgRoomRepository;
    use crate::service::error::ServiceError;

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    fn build_service(pool: PgPool) -> HousekeepingService {
        HousekeepingService::new(
            Arc::new(PgRoomRepository::new()),
            Arc::new(OutboxRepository::new()),
            Arc::new(EventBus::new()),
            pool,
        )
    }

    /// Validation fires before any I/O, so a lazy (never-connected) pool
    /// is sufficient — this test runs even without a local PG.
    #[tokio::test]
    async fn mark_dirty_rejects_empty_by() {
        let pool = PgPool::connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
            .expect("lazy pool needs no live server");
        let svc = build_service(pool);
        let err = svc
            .mark_dirty(MarkDirtyCommand {
                room_id: 1,
                by: "   ".into(),
                source: EventSource::System { reason: "test".into() },
            })
            .await
            .expect_err("blank `by` must be rejected");
        assert!(matches!(err, ServiceError::Validation(_)), "got {err:?}");
    }

    /// housekeeping-ops invariant #4: the maid surface's `done` must enqueue
    /// EXACTLY ONE `mark_room_clean` job no matter how many times it fires on
    /// an already-clean room. The second call must report `None` (no-op) and
    /// leave the job count untouched — a double-tap on เสร็จแล้ว cannot
    /// produce a second `HT_Housewife` audit row in iHOTEL.
    #[tokio::test]
    async fn mark_clean_if_dirty_enqueues_once_then_no_ops() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping mark_clean_if_dirty_enqueues_once — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-MC1'")
            .execute(&pool)
            .await;

        // Seed DIRTY.
        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_clean) \
             VALUES ('ZT-MC1', false) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = sqlx::Row::try_get(&row, "room_id").unwrap();
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());
        let cmd = || MarkCleanCommand {
            room_id,
            by: "นก".into(),
            source: EventSource::System { reason: "test".into() },
        };

        // First `done` performs the transition.
        let first = svc
            .mark_clean_if_dirty(cmd())
            .await
            .expect("first mark_clean_if_dirty must succeed");
        assert!(first.is_some(), "first call must perform the transition");

        // Repeat taps are no-ops.
        for _ in 0..3 {
            let repeat = svc
                .mark_clean_if_dirty(cmd())
                .await
                .expect("repeat mark_clean_if_dirty must succeed");
            assert!(
                repeat.is_none(),
                "a repeat `done` on an already-clean room must be a no-op"
            );
        }

        let clean: bool =
            sqlx::query_scalar("SELECT room_clean FROM ht_rooms_new WHERE room_id = $1")
                .bind(room_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(clean, "canonical room_clean must be true");

        let jobs: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'mark_room_clean'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(jobs, 1, "exactly one writeback job across 4 `done` taps");

        // The maid's name is what reaches `HT_Housewife.h_name` — the recipe
        // projects the intent's `by` into that column. `WritebackIntent` is
        // adjacently tagged (`#[serde(tag = "intent", content = "payload")]`),
        // so the stored column is `{"intent":…,"payload":{…}}`. Asserting the
        // Thai name survives the JSONB round-trip guards the whole attribution
        // chain: maid identity → intent payload → h_name.
        let stored: serde_json::Value = sqlx::query_scalar(
            "SELECT payload FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'mark_room_clean'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            stored.get("intent").and_then(|v| v.as_str()),
            Some("mark_room_clean"),
            "adjacent tag must name the intent: {stored}"
        );
        assert_eq!(
            stored.pointer("/payload/by").and_then(|v| v.as_str()),
            Some("นก"),
            "the maid's display name must ride the intent payload as `by`: {stored}"
        );

        // A later legitimate re-clean (room went dirty again) DOES enqueue.
        let _ = sqlx::query("UPDATE ht_rooms_new SET room_clean = false WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
        assert!(
            svc.mark_clean_if_dirty(cmd()).await.unwrap().is_some(),
            "a re-clean after the room went dirty again must enqueue"
        );

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    // ---- report_cleaning_progress (the /hk single write path) -----------

    /// Helper: how many writeback jobs of `intent` exist for this room.
    async fn job_count(pool: &PgPool, agg: Uuid, intent: &str) -> i64 {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeback_jobs WHERE aggregate_id = $1 AND intent = $2",
        )
        .bind(agg)
        .bind(intent)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn event_count(pool: &PgPool, agg: Uuid, event_type: &str) -> i64 {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM event_log WHERE aggregate_id = $1 AND event_type = $2",
        )
        .bind(agg)
        .bind(event_type)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn room_clean(pool: &PgPool, room_id: i32) -> Option<bool> {
        sqlx::query_scalar("SELECT room_clean FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    /// A tap with NO iHOTEL opinion — the pre-D1 shape, so every legacy-free
    /// assertion in this suite keeps testing canonical-only judgement.
    fn cleaning_cmd(room_id: i32, status: CleaningProgressStatus) -> ReportCleaningCommand {
        cleaning_cmd_with_legacy(room_id, status, LegacyCleanliness::Unknown)
    }

    fn cleaning_cmd_with_legacy(
        room_id: i32,
        status: CleaningProgressStatus,
        legacy_room_clean: LegacyCleanliness,
    ) -> ReportCleaningCommand {
        ReportCleaningCommand {
            room_id,
            status,
            legacy_room_clean,
            badge: "Q1001".into(),
            name: Some("นก".into()),
            by: "นก".into(),
            source: EventSource::System {
                reason: "test".into(),
            },
        }
    }

    /// The whole `/hk` matrix in one place: `started` is legacy-inert,
    /// `done`/`dirty` each flip the flag once and enqueue exactly one job, and
    /// a repeat of either changes nothing (invariant #4) while STILL appending
    /// the maid's event row (the log is append-only by nature).
    #[tokio::test]
    async fn report_cleaning_progress_started_done_dirty_and_repeats() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping report_cleaning_progress_started_done_dirty — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-RCP1'")
            .execute(&pool)
            .await;
        // Seed DIRTY, the state a maid actually finds a room in.
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-RCP1', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());

        // --- started: PG-only. No flip, no writeback, but a live event. ---
        let started = svc
            .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Started))
            .await
            .expect("started must succeed");
        assert!(
            !started.writeback_enqueued,
            "`started` is legacy-inert — iHOTEL's Room_Clean_Time drives its \
             room-power countdown, so mirroring it is parity risk for no gain"
        );
        assert_eq!(
            room_clean(&pool, room_id).await,
            Some(false),
            "`started` must not touch the cleanliness flag"
        );
        assert_eq!(job_count(&pool, agg, "mark_room_clean").await, 0);
        assert_eq!(job_count(&pool, agg, "mark_room_dirty").await, 0);
        assert_eq!(
            event_count(&pool, agg, "RoomCleaningStarted").await,
            1,
            "reception's board is driven by this event — it MUST be published"
        );

        // --- done: flips to clean and enqueues exactly one MarkRoomClean. ---
        let done = svc
            .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Done))
            .await
            .expect("done must succeed");
        assert!(done.writeback_enqueued);
        assert_eq!(room_clean(&pool, room_id).await, Some(true));
        assert_eq!(job_count(&pool, agg, "mark_room_clean").await, 1);

        // A double-tap must not produce a second HT_Housewife audit row.
        for _ in 0..3 {
            let repeat = svc
                .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Done))
                .await
                .expect("repeat done must succeed");
            assert!(
                !repeat.writeback_enqueued,
                "a repeat `done` on an already-clean room must enqueue nothing"
            );
        }
        assert_eq!(
            job_count(&pool, agg, "mark_room_clean").await,
            1,
            "exactly one writeback across four `done` taps"
        );

        // --- dirty: the mirror image. ---
        let dirty = svc
            .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Dirty))
            .await
            .expect("dirty must succeed");
        assert!(dirty.writeback_enqueued);
        assert_eq!(room_clean(&pool, room_id).await, Some(false));
        assert_eq!(job_count(&pool, agg, "mark_room_dirty").await, 1);
        let repeat_dirty = svc
            .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Dirty))
            .await
            .expect("repeat dirty must succeed");
        assert!(!repeat_dirty.writeback_enqueued);
        assert_eq!(
            job_count(&pool, agg, "mark_room_dirty").await,
            1,
            "exactly one writeback across two `dirty` taps"
        );

        // The maid's log keeps EVERY tap — that is what makes it an audit
        // trail; only the legacy write is deduplicated.
        let events: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ht_hk_cleaning_events WHERE hkev_room_id = $1",
        )
        .bind(room_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(events, 7, "1 started + 4 done + 2 dirty");

        // The maid's name rides the intent into HT_Housewife.h_name.
        let stored: serde_json::Value = sqlx::query_scalar(
            "SELECT payload FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'mark_room_dirty'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            stored.pointer("/payload/by").and_then(|v| v.as_str()),
            Some("นก")
        );

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// A NULL `room_clean` (legacy-mirrored rows can carry one) counts as
    /// "not yet in that state" in BOTH directions — an unknown cleanliness must
    /// reach iHOTEL explicitly, never be swallowed as "already there".
    #[tokio::test]
    async fn null_room_clean_is_flipped_in_both_directions() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping null_room_clean_is_flipped — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-RCP2'")
            .execute(&pool)
            .await;
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-RCP2', NULL, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());
        assert!(
            svc.report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Dirty))
                .await
                .expect("dirty must succeed")
                .writeback_enqueued,
            "NULL cleanliness must be flipped to false and mirrored"
        );
        assert_eq!(room_clean(&pool, room_id).await, Some(false));

        let _ = sqlx::query("UPDATE ht_rooms_new SET room_clean = NULL WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
        assert!(
            svc.report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Done))
                .await
                .expect("done must succeed")
                .writeback_enqueued,
            "NULL cleanliness must be flipped to true and mirrored"
        );

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// D1 end to end, on the `done` pole: the mirror lags (canonical CLEAN,
    /// iHOTEL DIRTY), so the tap the old guard swallowed must now enqueue the
    /// writeback — and a double-tap must NOT enqueue a second one, because
    /// iHOTEL keeps answering "dirty" until our own job drains.
    #[tokio::test]
    async fn mirror_lag_repairs_a_done_tap_once_and_only_once() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping mirror_lag_repairs_a_done_tap_once — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-D1A'")
            .execute(&pool)
            .await;
        // Seed CLEAN: the CT mirror still carries yesterday's answer while
        // iHOTEL has already been told the guest checked out.
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-D1A', true, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());

        // Pre-D1 behaviour, pinned: with NO iHOTEL opinion this exact tap is
        // the silent no-op the owner reported.
        let blind = svc
            .report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Done))
            .await
            .expect("done must succeed");
        assert!(
            !blind.writeback_enqueued,
            "canonical-only judgement must still no-op — that is the fallback \
             a legacy outage degrades to"
        );
        assert_eq!(job_count(&pool, agg, "mark_room_clean").await, 0);

        // THE FIX: the maid saw iHOTEL's DIRTY, so her tap earns the writeback.
        let repaired = svc
            .report_cleaning_progress(cleaning_cmd_with_legacy(
                room_id,
                CleaningProgressStatus::Done,
                LegacyCleanliness::Dirty,
            ))
            .await
            .expect("done must succeed");
        assert!(
            repaired.writeback_enqueued,
            "iHOTEL said dirty — the tap MUST reach iHOTEL (defect D1)"
        );
        assert_eq!(job_count(&pool, agg, "mark_room_clean").await, 1);
        assert_eq!(
            room_clean(&pool, room_id).await,
            Some(true),
            "canonical already matched the target — a repair flips nothing"
        );
        assert_eq!(
            event_count(&pool, agg, "RoomMarkedClean").await,
            1,
            "reception's board must learn about the room over SSE too"
        );

        // Double-tap while the job is still queued: iHOTEL still reads dirty,
        // but that is OUR undelivered write, not a second dirty room.
        for _ in 0..3 {
            let repeat = svc
                .report_cleaning_progress(cleaning_cmd_with_legacy(
                    room_id,
                    CleaningProgressStatus::Done,
                    LegacyCleanliness::Dirty,
                ))
                .await
                .expect("repeat done must succeed");
            assert!(
                !repeat.writeback_enqueued,
                "a repeat tap must not force a second HT_Housewife audit row"
            );
        }
        assert_eq!(
            job_count(&pool, agg, "mark_room_clean").await,
            1,
            "exactly one writeback across four taps under a lagging mirror"
        );

        // The dedup gates ONLY repairs. A room that genuinely goes dirty again
        // inside the same window and is cleaned again must still enqueue.
        let _ = sqlx::query("UPDATE ht_rooms_new SET room_clean = false WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
        let real = svc
            .report_cleaning_progress(cleaning_cmd_with_legacy(
                room_id,
                CleaningProgressStatus::Done,
                LegacyCleanliness::Dirty,
            ))
            .await
            .expect("re-clean must succeed");
        assert!(
            real.writeback_enqueued,
            "a REAL canonical transition is never suppressed by the repair dedup"
        );
        assert_eq!(job_count(&pool, agg, "mark_room_clean").await, 2);

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// The symmetric pole, end to end: canonical already DIRTY while iHOTEL
    /// still shows the room CLEAN. The maid's ห้องยังไม่สะอาด tap must reach
    /// iHOTEL's grid, and repeat only once.
    #[tokio::test]
    async fn mirror_lag_repairs_a_dirty_tap_once_and_only_once() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping mirror_lag_repairs_a_dirty_tap_once — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-D1B'")
            .execute(&pool)
            .await;
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-D1B', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());

        assert!(
            !svc.report_cleaning_progress(cleaning_cmd(room_id, CleaningProgressStatus::Dirty))
                .await
                .expect("dirty must succeed")
                .writeback_enqueued,
            "canonical-only judgement still no-ops (the pre-D1 fallback)"
        );

        assert!(
            svc.report_cleaning_progress(cleaning_cmd_with_legacy(
                room_id,
                CleaningProgressStatus::Dirty,
                LegacyCleanliness::Clean,
            ))
            .await
            .expect("dirty must succeed")
            .writeback_enqueued,
            "iHOTEL showed the room clean — the tap MUST reach its grid"
        );
        assert_eq!(job_count(&pool, agg, "mark_room_dirty").await, 1);
        assert_eq!(room_clean(&pool, room_id).await, Some(false));

        assert!(
            !svc.report_cleaning_progress(cleaning_cmd_with_legacy(
                room_id,
                CleaningProgressStatus::Dirty,
                LegacyCleanliness::Clean,
            ))
            .await
            .expect("repeat dirty must succeed")
            .writeback_enqueued,
            "the second tap is deduplicated against our own queued job"
        );
        assert_eq!(
            job_count(&pool, agg, "mark_room_dirty").await,
            1,
            "exactly one writeback across the double-tap"
        );

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// Validation fires before any I/O. A blank `by` would land as a blank
    /// `HT_Housewife.h_name`; a blank badge would produce an unattributable
    /// audit row that no maid can be identified from.
    #[tokio::test]
    async fn report_cleaning_progress_rejects_blank_identity() {
        let pool = PgPool::connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
            .expect("lazy pool needs no live server");
        let svc = build_service(pool);
        for (badge, by) in [("   ", "นก"), ("Q1001", "  ")] {
            let err = svc
                .report_cleaning_progress(ReportCleaningCommand {
                    room_id: 1,
                    status: CleaningProgressStatus::Done,
                    legacy_room_clean: LegacyCleanliness::Unknown,
                    badge: badge.into(),
                    name: None,
                    by: by.into(),
                    source: EventSource::System {
                        reason: "test".into(),
                    },
                })
                .await
                .expect_err("blank identity must be rejected");
            assert!(matches!(err, ServiceError::Validation(_)), "got {err:?}");
        }
    }

    // ---- D1: which truth the guard judges (pure, no DB) -----------------

    /// THE DEFECT, as a unit test. The CT mirror lags: canonical still says
    /// CLEAN while iHOTEL — the value the maid's screen showed her — says
    /// DIRTY. Before D1 this was `IS DISTINCT FROM true` ⇒ 0 rows ⇒ silent
    /// no-op ⇒ nothing ever reached iHOTEL. It must now be a repair.
    #[test]
    fn mirror_lagging_dirty_makes_a_done_tap_a_repair_not_a_no_op() {
        assert_eq!(
            decide_cleanliness(true, Some(true), LegacyCleanliness::Dirty),
            CleanlinessDecision::MirrorRepair
        );
    }

    /// The symmetric pole: the display is iHOTEL-wins in BOTH directions, so a
    /// `dirty` tap on a room canonical already calls dirty, while iHOTEL still
    /// shows it clean, must reach iHOTEL too. Covering only `done` would mean
    /// "we trust the maid's screen, but only sometimes".
    #[test]
    fn mirror_lagging_clean_makes_a_dirty_tap_a_repair() {
        assert_eq!(
            decide_cleanliness(false, Some(false), LegacyCleanliness::Clean),
            CleanlinessDecision::MirrorRepair
        );
    }

    /// Legacy unreachable / unmapped / not consulted ⇒ EXACTLY the pre-D1
    /// behaviour: canonical alone decides. This is the fallback the whole
    /// design rests on — same failure surface as before, never worse, and a
    /// maid's tap is never gated on a legacy server being up.
    #[test]
    fn unknown_legacy_falls_back_to_canonical_only_judgement() {
        // Canonical already at the target ⇒ the old silent no-op, unchanged.
        assert_eq!(
            decide_cleanliness(true, Some(true), LegacyCleanliness::Unknown),
            CleanlinessDecision::NoOp
        );
        assert_eq!(
            decide_cleanliness(false, Some(false), LegacyCleanliness::Unknown),
            CleanlinessDecision::NoOp
        );
        // Canonical NOT at the target ⇒ the ordinary transition, unchanged.
        assert_eq!(
            decide_cleanliness(true, Some(false), LegacyCleanliness::Unknown),
            CleanlinessDecision::Transition
        );
        assert_eq!(
            decide_cleanliness(false, Some(true), LegacyCleanliness::Unknown),
            CleanlinessDecision::Transition
        );
    }

    /// A real canonical transition outranks whatever iHOTEL says — including
    /// iHOTEL AGREEING with the target. The tap still flips canonical and still
    /// mirrors: suppressing it would leave the two databases disagreeing with
    /// no event to reconcile them, and it is the path 55/55 production jobs
    /// have taken.
    #[test]
    fn a_real_transition_is_never_suppressed_by_legacy_agreement() {
        for legacy in [
            LegacyCleanliness::Clean,
            LegacyCleanliness::Dirty,
            LegacyCleanliness::Unknown,
        ] {
            assert_eq!(
                decide_cleanliness(true, Some(false), legacy),
                CleanlinessDecision::Transition,
                "canonical dirty + done must transition regardless of {legacy:?}"
            );
        }
    }

    /// Both truths already agree with the tap ⇒ nothing to do. The honest
    /// no-op, and the one D1 must NOT turn into a write (that is Candidate B,
    /// always-enqueue, which would spam `HT_Housewife` and break the
    /// double-tap story).
    #[test]
    fn agreement_is_still_a_no_op() {
        assert_eq!(
            decide_cleanliness(true, Some(true), LegacyCleanliness::Clean),
            CleanlinessDecision::NoOp
        );
        assert_eq!(
            decide_cleanliness(false, Some(false), LegacyCleanliness::Dirty),
            CleanlinessDecision::NoOp
        );
    }

    /// NULL canonical keeps the old `IS DISTINCT FROM` semantics in both
    /// directions: unknown cleanliness is "not at the target", so it flips and
    /// mirrors explicitly instead of being swallowed as "already there".
    #[test]
    fn null_canonical_is_still_a_transition_in_both_directions() {
        for legacy in [
            LegacyCleanliness::Clean,
            LegacyCleanliness::Dirty,
            LegacyCleanliness::Unknown,
        ] {
            assert_eq!(
                decide_cleanliness(true, None, legacy),
                CleanlinessDecision::Transition
            );
            assert_eq!(
                decide_cleanliness(false, None, legacy),
                CleanlinessDecision::Transition
            );
        }
    }

    /// The polarity of the hint itself — inverted relative to legacy's
    /// `Room_Clean` NEEDS-CLEANING flag, and the single most breakable fact in
    /// the chain. `Unknown` must be the Default so an un-set field can only
    /// ever mean "canonical-only", never "iHOTEL said clean".
    #[test]
    fn legacy_cleanliness_polarity_and_default() {
        assert_eq!(LegacyCleanliness::Clean.is_clean(), Some(true));
        assert_eq!(LegacyCleanliness::Dirty.is_clean(), Some(false));
        assert_eq!(LegacyCleanliness::Unknown.is_clean(), None);
        assert_eq!(LegacyCleanliness::default(), LegacyCleanliness::Unknown);
    }

    /// The status literals this service writes MUST be exactly what the
    /// `hkev_status` CHECK accepts (migration 077 + 087). A drift here turns a
    /// maid's tap into a 500.
    #[test]
    fn cleaning_status_literals_are_stable() {
        assert_eq!(CleaningProgressStatus::Started.as_str(), "started");
        assert_eq!(CleaningProgressStatus::Done.as_str(), "done");
        assert_eq!(CleaningProgressStatus::Dirty.as_str(), "dirty");
        assert_eq!(
            CleaningProgressStatus::from_literal("dirty"),
            Some(CleaningProgressStatus::Dirty)
        );
        assert_eq!(CleaningProgressStatus::from_literal("clean"), None);
        assert_eq!(CleaningProgressStatus::from_literal("DONE"), None);
    }

    /// Validation fires before any I/O — an empty `by` would land as a blank
    /// `HT_Housewife.h_name`.
    #[tokio::test]
    async fn mark_clean_if_dirty_rejects_empty_by() {
        let pool = PgPool::connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
            .expect("lazy pool needs no live server");
        let svc = build_service(pool);
        let err = svc
            .mark_clean_if_dirty(MarkCleanCommand {
                room_id: 1,
                by: "   ".into(),
                source: EventSource::System { reason: "test".into() },
            })
            .await
            .expect_err("blank `by` must be rejected");
        assert!(matches!(err, ServiceError::Validation(_)), "got {err:?}");
    }

    /// Mark-dirty must flip the canonical flag AND enqueue exactly one
    /// `mark_room_dirty` writeback job in the same transaction; repeated
    /// flips must keep enqueuing (per-event discriminator key — rooms go
    /// dirty every day, the permanently-UNIQUE idempotency key must not
    /// block the second event).
    #[tokio::test]
    async fn mark_dirty_flips_flag_and_enqueues_writeback() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping mark_dirty_flips_flag — PG not reachable");
            return;
        };
        // Reset marker rows from prior (possibly aborted) runs.
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-MD1'")
            .execute(&pool)
            .await;

        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_clean) \
             VALUES ('ZT-MD1', true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = sqlx::Row::try_get(&row, "room_id").unwrap();
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());
        for _ in 0..2 {
            svc.mark_dirty(MarkDirtyCommand {
                room_id,
                by: "Admin".into(),
                source: EventSource::System { reason: "test".into() },
            })
            .await
            .expect("mark_dirty must succeed (repeatedly)");
        }

        let clean: bool = sqlx::query_scalar(
            "SELECT room_clean FROM ht_rooms_new WHERE room_id = $1",
        )
        .bind(room_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(!clean, "canonical room_clean must be false after mark_dirty");

        let jobs: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'mark_room_dirty'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(jobs, 2, "each mark_dirty must enqueue its own job");

        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }
}
