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

/// Command for [`HousekeepingService::report_cleaning_progress`] — one maid tap
/// on `POST /api/hk/rooms/{id}/cleaning`.
#[derive(Debug, Clone)]
pub struct ReportCleaningCommand {
    pub room_id: i32,
    pub status: CleaningProgressStatus,
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
    /// | `done` | `room_clean → true` when not already true | `MarkRoomClean` | `RoomMarkedClean` |
    /// | `dirty` | `room_clean → false` when not already false | `MarkRoomDirty` | `RoomMarkedDirty` |
    ///
    /// ## Idempotency (invariant #4)
    ///
    /// The conditional UPDATE (`… IS DISTINCT FROM …`) takes the row lock, so
    /// two concurrent taps serialize and the loser enqueues nothing — the same
    /// argument written out at [`mark_clean_if_dirty`](Self::mark_clean_if_dirty),
    /// applied symmetrically to `dirty`. The per-event v4 discriminator stays on
    /// the idempotency key: rooms legitimately flip every day and
    /// `writeback_jobs.idempotency_key` is permanently UNIQUE.
    ///
    /// The APPEND-ONLY event row is always written — `started`/`done` may
    /// legitimately repeat within a day and the "current progress" read takes
    /// only the LATEST row. What must not double is the legacy write, and that
    /// is what the conditional flip guards.
    ///
    /// The route resolves 404 for unknown/inactive rooms before calling; the
    /// `room_exists` probe here disambiguates "already in that state" from
    /// "no such room" if the row disappears between the two.
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
                });
            }
            CleaningProgressStatus::Done => {
                if !set_room_clean_flag_if_dirty(&mut tx, cmd.room_id).await? {
                    return finish_no_transition(tx, cmd.room_id, event_id).await;
                }
                (
                    WritebackIntent::MarkRoomClean {
                        room_id: aggregate_id,
                        by: cmd.by.clone(),
                    },
                    DomainEvent::RoomMarkedClean {
                        room_id: aggregate_id,
                        by: cmd.by,
                        source: cmd.source,
                    },
                )
            }
            CleaningProgressStatus::Dirty => {
                if !set_room_clean_flag_if_clean(&mut tx, cmd.room_id).await? {
                    return finish_no_transition(tx, cmd.room_id, event_id).await;
                }
                (
                    WritebackIntent::MarkRoomDirty {
                        room_id: aggregate_id,
                        by: cmd.by,
                    },
                    DomainEvent::RoomMarkedDirty {
                        room_id: aggregate_id,
                        source: cmd.source,
                    },
                )
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
        })
    }

    /// Mark a room dirty — flip `room_clean=false`, enqueue the
    /// [`WritebackIntent::MarkRoomDirty`] mirror (cheatsheet §3.13), and
    /// publish the event — all in one transaction.
    ///
    /// Audit 2026-06-11 P2 gap-close: this used to be PG-only ("the legacy
    /// app derives dirty from checkout §3e"), but iHOTEL's grid reads the
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

/// Close a cleaning report that performed NO cleanliness transition: keep the
/// appended event, enqueue nothing, publish nothing — but 404 first if the room
/// vanished (same disambiguation as `mark_clean_if_dirty`).
async fn finish_no_transition(
    mut tx: Transaction<'_, Postgres>,
    room_id: i32,
    event_id: i64,
) -> ServiceResult<CleaningReport> {
    if !room_exists(&mut tx, room_id).await? {
        return Err(ServiceError::not_found(format!(
            "room {room_id} does not exist"
        )));
    }
    tx.commit().await?;
    Ok(CleaningReport {
        room_id,
        event_id,
        writeback_enqueued: false,
    })
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

    fn cleaning_cmd(room_id: i32, status: CleaningProgressStatus) -> ReportCleaningCommand {
        ReportCleaningCommand {
            room_id,
            status,
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
