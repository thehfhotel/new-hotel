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
    /// Housekeeper identifier (legacy literal goes into `HT_Housewife.h_by`
    /// per spike §3j). Routes populate from auth context.
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
