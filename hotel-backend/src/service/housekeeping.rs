//! Housekeeping service — room cleanliness state transitions.
//!
//! Per `docs/architecture.md` §1, §6 and `docs/legacy-spike/findings.md` §3j
//! (mark-room-clean recipe). Each public method opens one PG transaction,
//! flips `ht_rooms_new.room_clean`, enqueues the relevant
//! [`WritebackIntent`] (only `MarkRoomClean` exists today — `MarkDirty` is
//! handled implicitly by check-out per §3e and has no standalone recipe),
//! publishes a [`DomainEvent`], and commits.

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
    /// Housekeeper identifier (legacy literal goes into `HT_Housewife.h_by`
    /// per spike §3j). Routes populate from auth context.
    pub by: String,
    pub source: EventSource,
}

/// Command for [`HousekeepingService::mark_dirty`].
#[derive(Debug, Clone)]
pub struct MarkDirtyCommand {
    pub room_id: i32,
    pub source: EventSource,
}

/// Outcome of a housekeeping mutation.
#[derive(Debug, Clone)]
pub struct HousekeepingOutcome {
    pub room_id: i32,
    pub aggregate_id: Uuid,
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
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

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

    /// Mark a room dirty — flip `room_clean=false` + publish event.
    ///
    /// No outbox enqueue: the legacy app derives "dirty" from the absence
    /// of a `Room_Clean='no'` flag, set during checkout (§3e). Standalone
    /// dirty marking is local-only state.
    pub async fn mark_dirty(
        &self,
        cmd: MarkDirtyCommand,
    ) -> ServiceResult<HousekeepingOutcome> {
        let mut tx = self.pg.begin().await?;

        set_room_clean_flag(&mut tx, cmd.room_id, false).await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Room, cmd.room_id);
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
