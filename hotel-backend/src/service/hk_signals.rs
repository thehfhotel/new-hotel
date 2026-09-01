//! Room-signal service — the canned reception ⇄ maid notices of ADR 0008.
//!
//! Owns `ht_hk_room_signals` (migration 089): raise, ack, done, cancel, and the
//! ขอเช็คห้อง ANSWER, plus the sweep a maid's เสร็จแล้ว cleaning report runs.
//! Every rule it enforces is a pure function in [`crate::domain::hk_signal`];
//! this module is the transaction boundary and the SQL, nothing else.
//!
//! ## Coexistence stance (ADR 0002 / invariant #6)
//!
//! **PG-CANONICAL ONLY, with no path to legacy by design.** iHOTEL has no
//! counterpart to a room signal at all, so there is no sync mapper, no
//! writeback recipe, no [`crate::outbox::intent::WritebackIntent`] and no
//! ship-dark flag waiting to enable one — the same posture as
//! `ht_hk_linen_reports` (migration 088), and unlike `ht_hk_cleaning_events`,
//! whose `done` phase did grow a `MarkRoomClean` writeback in 2026-08-11.
//!
//! What it DOES publish is [`DomainEvent`]s — `RoomSignalRaised` /
//! `RoomSignalAcked` / `RoomSignalCompleted` / `RoomSignalCancelled` — through
//! the existing [`EventBus`], i.e. one `event_log` row plus one
//! `pg_notify('domain_events')` per change, buffered until commit. That is UI
//! event plumbing over the fan-out `routes::events` already runs; an event is
//! not a legacy write, and none of these four has a writeback twin.
//!
//! ## One transaction, always
//!
//! Two commands are genuinely multi-row and both commit atomically:
//!
//! * [`HkSignalService::answer_room_check`] — completes the check AND inserts
//!   one standing maid→desk child signal per reported problem
//!   (`sig_parent_id` → the check), publishing one event each. A guest is at
//!   the counter while this runs; "the check closed but the มีของหาย never
//!   appeared" is not a state the desk may ever observe.
//! * [`auto_complete_clean_report`] — called from
//!   [`crate::service::housekeeping::HousekeepingService::report_cleaning_progress`]
//!   INSIDE that method's existing transaction, so a maid's เสร็จแล้ว report
//!   and the closure of that room's ทำห้องนี้ก่อน / แขกเช็คเอาท์แล้ว signals
//!   are one commit with the cleaning event and the `room_clean` flip.
//!
//! ## Wrong-status transitions answer 400, not 409
//!
//! [`crate::domain::hk_signal::next_status`] refuses a transition from a
//! status it cannot leave; this module raises that as
//! [`ServiceError::Conflict`], which the repo's single
//! `From<ServiceError> for ApiError` maps to **400** (see `service::error`).
//! That mapping is shared with booking-cancel, payment-void and the
//! `writeback_jobs` idempotency race, and `ApiError::Conflict` (409) is
//! reserved here for a ship-dark FLAG refusing a request. Answering 409 for
//! room signals alone would have meant either a second mapping or changing five
//! shipped surfaces, so signals follow the house convention: **400 with a
//! message naming the status it is already in.** Role refusals are the one
//! thing that is NOT 400 — they are [`ServiceError::Forbidden`] ⇒ 403.
//!
//! ## Identity
//!
//! Badge + display name come exclusively from the verified identity the route
//! resolved ([`crate::middleware::hk_access::HkIdentity`] on `/hk`, the desk's
//! operator label on the reception surface). Nothing identity-like is ever
//! accepted from a request body — same rule as every other `ht_hk_*` writer,
//! and no FK to `ht_users` (maids are CF Access + HF ID identities).
//!
//! All SQL here is RUNTIME `sqlx::query` (never the `query!` macro), so this
//! module needs no `.sqlx/` cache regeneration — the same choice
//! `service::housekeeping` and `routes::housekeeping` already make.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::domain::hk_signal::{
    direction_for_role_type, next_status, parse_problems, RoomCheckOutcome, RoomSignal,
    SignalAction, SignalActor, SignalDirection, SignalDoneSource, SignalRole, SignalRuleError,
    SignalStatus, CLEAN_REPORT_AUTO_COMPLETE_TYPES, ROOM_CHECK,
};
use crate::outbox::event::{DomainEvent, EventSource};
use crate::outbox::EventBus;

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// The projection every read of this table uses, joined to `ht_rooms_new` for
/// the room number the boards render.
///
/// ONE constant, so the list endpoint, the post-mutation reload and the
/// auto-complete sweep cannot drift into serving different DTO shapes. The
/// join is INNER on purpose: `sig_room_id` is a `NOT NULL` FK with
/// `ON DELETE CASCADE`, so a signal without a room is not a state that exists.
const SIGNAL_SELECT: &str = "SELECT s.sig_id, s.sig_room_id, r.room_no, s.sig_direction, \
                             s.sig_type, s.sig_status, s.sig_outcome, s.sig_parent_id, \
                             s.sig_created_badge, s.sig_created_name, s.sig_created_at, \
                             s.sig_acked_badge, s.sig_acked_name, s.sig_acked_at, \
                             s.sig_done_badge, s.sig_done_name, s.sig_done_at, s.sig_done_source \
                             FROM ht_hk_room_signals s \
                             JOIN ht_rooms_new r ON r.room_id = s.sig_room_id";

/// The Bangkok civil-day boundary for a signal's COMPLETION timestamp.
///
/// Character-for-character `routes::hk::TODAY_BKK`, on `sig_done_at` instead of
/// `hkev_created_at` — exactly the construction `TODAY_BKK_LINEN` (migration
/// 088) made for `hklr_created_at`, and pinned to the canonical boundary by
/// `today_bkk_signal_done_is_the_cleaning_day_boundary` below. The column is
/// spelled bare, not `s.sig_done_at`, because `sig_done_at` exists on exactly
/// one of [`SIGNAL_SELECT`]'s two tables — the same way `TODAY_BKK` is dropped
/// unqualified into a join with `ht_rooms_new`.
///
/// `CURRENT_DATE` is BANNED here for `TODAY_BKK`'s reason: it is the SERVER's
/// date, and between 17:00 and 24:00 UTC it names YESTERDAY in Bangkok. Using
/// it would blank reception's answered-check row for the seven hours a night
/// that straddle the busiest late checkouts.
pub(crate) const TODAY_BKK_SIGNAL_DONE: &str =
    "(sig_done_at AT TIME ZONE 'Asia/Bangkok')::date = (NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// The newest ANSWERED ขอเช็คห้อง per room, for the Bangkok civil day.
///
/// A function rather than a `const` because it composes [`SIGNAL_SELECT`] (so
/// there is still ONE projection, and an answered check deserializes through
/// the same [`signal_from_row`] as every other read) with the
/// `DISTINCT ON (s.sig_room_id)` de-duplication — same "SQL you can read at the
/// call site, pinned by a unit test" idiom as `routes::hk`'s
/// `linen_shortage_today_sql`.
///
/// Why each clause is here:
///
/// * `sig_type = 'room_check'` + `sig_status = 'done'` +
///   `sig_done_source = 'room_check_answer'` — an answered check, specifically.
///   `cancelled` is excluded by the status predicate alone (a withdrawn check
///   was never answered and must never read as เคลียร์), and so is a check
///   still open on the board, which the live list already carries.
/// * `sig_outcome IS NOT NULL` — belt and braces with `room_check_answer`: the
///   whole point of this read is the ANSWER, and a done row without one would
///   serialize as `outcome: null` and tell the desk nothing.
/// * [`TODAY_BKK_SIGNAL_DONE`] — yesterday's เคลียร์ is not a fact about the
///   guest standing at the counter this morning.
///
/// The `DISTINCT ON` inner ORDER BY must lead with the distinct expression, so
/// the newest-first ordering the contract promises is applied by the wrapping
/// SELECT: one entry per room, `doneAt` descending, `sig_id` breaking a tie.
fn answered_room_checks_today_sql() -> String {
    // ONE `SELECT ` prefix, and it is the first six characters of the shared
    // projection — asserted by `the_answered_query_reuses_the_one_projection`.
    let newest_per_room = format!(
        "{distinct} \
          WHERE s.sig_type = '{ROOM_CHECK}' \
            AND s.sig_status = '{done}' \
            AND s.sig_done_source = '{answer}' \
            AND s.sig_outcome IS NOT NULL \
            AND {TODAY_BKK_SIGNAL_DONE} \
          ORDER BY s.sig_room_id, s.sig_done_at DESC, s.sig_id DESC",
        distinct = SIGNAL_SELECT.replacen("SELECT ", "SELECT DISTINCT ON (s.sig_room_id) ", 1),
        done = SignalStatus::Done.as_str(),
        answer = SignalDoneSource::RoomCheckAnswer.as_str(),
    );
    format!(
        "SELECT * FROM ({newest_per_room}) answered \
          ORDER BY answered.sig_done_at DESC, answered.sig_id DESC"
    )
}

/// Command for [`HkSignalService::raise`].
#[derive(Debug, Clone)]
pub struct RaiseSignalCommand {
    pub room_id: i32,
    /// The canned code as it arrived. NOT pre-validated: the service
    /// normalises and judges it, so any caller — the maid surface, the desk
    /// surface, a future one — gets the identical verdict.
    pub signal_type: String,
    /// Which side the caller is on. On `/hk` this is
    /// `SignalRole::from_can_report(identity.can_report)`; on the desk surface
    /// it is constantly [`SignalRole::Desk`].
    pub role: SignalRole,
    pub badge: String,
    pub name: Option<String>,
    pub source: EventSource,
}

/// Command for [`HkSignalService::act`] — ack / done / cancel.
///
/// `answer` is deliberately NOT expressible here: it carries an outcome and can
/// spawn child rows, so it has its own command and its own method rather than
/// an `Option<outcome>` that is meaningless for the other three.
#[derive(Debug, Clone)]
pub struct ActOnSignalCommand {
    pub signal_id: i64,
    pub action: SignalAction,
    pub role: SignalRole,
    pub badge: String,
    pub name: Option<String>,
    pub source: EventSource,
}

/// Command for [`HkSignalService::answer_room_check`].
#[derive(Debug, Clone)]
pub struct AnswerRoomCheckCommand {
    pub signal_id: i64,
    pub outcome: RoomCheckOutcome,
    /// Raw problem codes; ignored (and required empty) for `clear`.
    pub problems: Vec<String>,
    pub role: SignalRole,
    pub badge: String,
    pub name: Option<String>,
    pub source: EventSource,
}

/// Outcome of a single-signal command.
#[derive(Debug, Clone)]
pub struct SignalOutcome {
    pub signal: RoomSignal,
}

/// Outcome of a ขอเช็คห้อง answer: the completed check plus the standing
/// guest-accountability signals it spawned (empty for `clear`).
#[derive(Debug, Clone)]
pub struct AnswerOutcome {
    pub signal: RoomSignal,
    pub spawned: Vec<RoomSignal>,
}

/// Service handle for the room-signal aggregate.
///
/// Holds ONLY a pool — no `RoomRepository`, no `OutboxRepository`, no
/// `EventBus` handle. That is not an oversight and not a shortcut: signals
/// enqueue no writeback intent (there is no legacy counterpart), and
/// [`EventBus::publish`] is an associated function taking the caller's
/// transaction, so an `Arc<EventBus>` here would be a field that is never
/// read — and a misleading hint that one day a writeback lands. Compare
/// [`crate::service::housekeeping::HousekeepingService`], which holds all
/// four because it genuinely enqueues `MarkRoomClean`.
#[derive(Clone)]
pub struct HkSignalService {
    pg: PgPool,
}

impl HkSignalService {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    /// Every signal still on the boards for this site — `open` + `acked`, in
    /// the order the room-signal partial index serves them.
    ///
    /// No branch predicate: site scoping is CONNECTION-level (each site's pool
    /// holds its own signals), the same model as `ht_hk_cleaning_events` and
    /// `ht_hk_linen_reports`. A `?branch=` therefore selects the POOL, never a
    /// `WHERE` clause — which is why the maid stream cannot leak the other
    /// branch either: the two sites' `domain_events` channels are separate
    /// databases.
    pub async fn list_live(&self) -> ServiceResult<Vec<RoomSignal>> {
        let sql = format!(
            "{SIGNAL_SELECT} WHERE s.sig_status IN ('open', 'acked') \
             ORDER BY s.sig_created_at ASC, s.sig_id ASC"
        );
        let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql)).fetch_all(&self.pg).await?;
        rows.iter().map(signal_from_row).collect()
    }

    /// Today's ANSWERED ขอเช็คห้อง — the newest per room, newest room first.
    ///
    /// ## The gap this closes
    ///
    /// [`list_live`](Self::list_live) is `open` + `acked` by contract, so the
    /// instant a maid answers a check it leaves that list. A `problems` answer
    /// leaves its standing child signals behind and stays fully visible; a
    /// เคลียร์ answer leaves NOTHING, so `components/v2/signals/RoomCheckPanel`
    /// could only infer it from a transition its own tab watched — and a desk
    /// tab reload (or a second receptionist's tab) lost that inference and
    /// showed "not requested" for a room the maid had already cleared. The
    /// panel's header documents the gap and names this exact fix: "let the desk
    /// read the room's last `room_check` including its `outcome`".
    ///
    /// ## Scope, and why it is not just "the last check"
    ///
    /// One row per room, `status='done'` with `sig_done_source =
    /// 'room_check_answer'` and an `outcome`, bounded to the Bangkok civil day
    /// ([`TODAY_BKK_SIGNAL_DONE`]) — the same TODAY the maid's cleaning and
    /// linen reads use. A cancelled check can never appear (it is not `done`),
    /// which is what keeps a withdrawn ขอเช็คห้อง from ever rendering green.
    ///
    /// Read-only, no transaction, and deliberately a SECOND query rather than a
    /// widening of the live list: the live board's predicate is a published
    /// contract (`open` + `acked`) that three surfaces render, and folding
    /// terminal rows into it would change what every one of them shows.
    pub async fn list_answered_room_checks_today(&self) -> ServiceResult<Vec<RoomSignal>> {
        let sql = answered_room_checks_today_sql();
        let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql)).fetch_all(&self.pg).await?;
        rows.iter().map(signal_from_row).collect()
    }

    /// Raise a new signal. One row, one `RoomSignalRaised`, one transaction.
    ///
    /// The room's existence is NOT re-probed here — the routes 404 an unknown
    /// or inactive room before calling, and the FK is the backstop, exactly as
    /// `report_linen_shortage` treats `hklr_room_id`.
    pub async fn raise(&self, cmd: RaiseSignalCommand) -> ServiceResult<SignalOutcome> {
        let badge = require_badge(&cmd.badge)?;
        let signal_type = crate::domain::hk_signal::normalize_type(&cmd.signal_type);
        let direction = direction_for_role_type(cmd.role, &signal_type).map_err(rule_error)?;

        let mut tx = self.pg.begin().await?;
        let signal = insert_signal(
            &mut tx,
            InsertSignal {
                room_id: cmd.room_id,
                direction,
                signal_type: &signal_type,
                parent_id: None,
                badge: &badge,
                name: cmd.name.as_deref(),
            },
        )
        .await?;
        publish(&mut tx, raised(&signal, cmd.source)).await?;
        tx.commit().await?;
        Ok(SignalOutcome { signal })
    }

    /// Ack, complete or cancel an existing signal.
    ///
    /// The row is locked (`SELECT … FOR UPDATE`) BEFORE the rules are judged,
    /// so two receptionists tapping ack at once cannot both succeed: the loser
    /// re-reads `acked` under the lock and is refused. Same serialization point
    /// idiom as `service::housekeeping`'s `lock_room_clean`.
    pub async fn act(&self, cmd: ActOnSignalCommand) -> ServiceResult<SignalOutcome> {
        if cmd.action == SignalAction::Answer {
            // Unreachable from the routes (answer has its own endpoint and its
            // own command). Refuse loudly rather than write a half-answer with
            // no outcome and no child signals.
            return Err(ServiceError::validation(
                "answering a room check goes through answer_room_check, not act",
            ));
        }
        let badge = require_badge(&cmd.badge)?;

        let mut tx = self.pg.begin().await?;
        let locked = lock_signal(&mut tx, cmd.signal_id).await?;
        let target = next_status(
            cmd.action,
            cmd.role,
            locked.direction,
            &locked.signal_type,
            locked.status,
        )
        .map_err(rule_error)?;

        let signal = match target {
            SignalStatus::Acked => {
                apply_ack(&mut tx, cmd.signal_id, &badge, cmd.name.as_deref()).await?
            }
            SignalStatus::Done => {
                apply_done(
                    &mut tx,
                    cmd.signal_id,
                    &badge,
                    cmd.name.as_deref(),
                    SignalDoneSource::Tap,
                    None,
                )
                .await?
            }
            SignalStatus::Cancelled => apply_cancel(&mut tx, cmd.signal_id).await?,
            // `next_status` never returns `Open` — it only ever moves a signal
            // forward. Refusing here keeps that a proved property rather than a
            // comment.
            SignalStatus::Open => {
                return Err(ServiceError::internal(
                    "transition table returned 'open' as a target status",
                ))
            }
        };

        publish(&mut tx, event_for_status(&signal, cmd.source)).await?;
        tx.commit().await?;
        Ok(SignalOutcome { signal })
    }

    /// Answer a ขอเช็คห้อง: complete the check and, for `problems`, insert one
    /// standing maid→desk child signal per problem — ALL in one transaction.
    ///
    /// The children are ordinary maid→desk signals with a `sig_parent_id`, not
    /// a special row shape: มีของหาย / มีของเสียหาย are real entries in the
    /// canned vocabulary, which is exactly what lets the desk ack and complete
    /// them through the same endpoints as any other maid→desk signal.
    pub async fn answer_room_check(
        &self,
        cmd: AnswerRoomCheckCommand,
    ) -> ServiceResult<AnswerOutcome> {
        let badge = require_badge(&cmd.badge)?;

        // Shape before state: a `problems` answer with nothing in it, or a
        // `clear` answer carrying problems, is malformed regardless of what the
        // signal's status turns out to be — and judging it first means the
        // caller is never told "already done" about a body that could not have
        // been accepted anyway.
        let problems: Vec<&'static str> = match cmd.outcome {
            RoomCheckOutcome::Clear => {
                if !cmd.problems.is_empty() {
                    return Err(ServiceError::validation(
                        "outcome 'clear' must not carry problems (เคลียร์ means nothing was found)",
                    ));
                }
                Vec::new()
            }
            RoomCheckOutcome::Problems => parse_problems(&cmd.problems).map_err(rule_error)?,
        };

        let mut tx = self.pg.begin().await?;
        let locked = lock_signal(&mut tx, cmd.signal_id).await?;
        next_status(
            SignalAction::Answer,
            cmd.role,
            locked.direction,
            &locked.signal_type,
            locked.status,
        )
        .map_err(rule_error)?;

        let signal = apply_done(
            &mut tx,
            cmd.signal_id,
            &badge,
            cmd.name.as_deref(),
            SignalDoneSource::RoomCheckAnswer,
            Some(cmd.outcome),
        )
        .await?;
        publish(&mut tx, completed(&signal, cmd.source.clone())).await?;

        let mut spawned = Vec::with_capacity(problems.len());
        for problem in problems {
            let child = insert_signal(
                &mut tx,
                InsertSignal {
                    room_id: locked.room_id,
                    // A problem is a MAID→DESK signal by definition — it is the
                    // maid telling the desk something the guest may owe for.
                    direction: SignalDirection::MaidToDesk,
                    signal_type: problem,
                    parent_id: Some(cmd.signal_id),
                    badge: &badge,
                    name: cmd.name.as_deref(),
                },
            )
            .await?;
            publish(&mut tx, raised(&child, cmd.source.clone())).await?;
            spawned.push(child);
        }

        tx.commit().await?;
        Ok(AnswerOutcome { signal, spawned })
    }
}

// ============================================================================
// The เสร็จแล้ว hook
// ============================================================================

/// Auto-complete this room's live cleaning-urgency signals as part of a maid's
/// เสร็จแล้ว report — **inside the caller's transaction**.
///
/// Called from
/// [`crate::service::housekeeping::HousekeepingService::report_cleaning_progress`]
/// so the closure commits with the cleaning event and the `room_clean` flip, or
/// not at all. Takes `&mut Transaction` rather than a pool for exactly that
/// reason: a version of this that opened its own transaction would be a
/// silently-partial checkout every time the outer one rolled back.
///
/// Scope is [`CLEAN_REPORT_AUTO_COMPLETE_TYPES`] — ทำห้องนี้ก่อน and
/// แขกเช็คเอาท์แล้ว. ขอเช็คห้อง is deliberately excluded: its completion is a
/// judgement the maid must state, and closing it on a cleaning tap would answer
/// เคลียร์ on her behalf while a guest waits at the counter.
///
/// Both `open` and `acked` are swept — a maid who acked ทำห้องนี้ก่อน and then
/// finished the room has done exactly the thing the signal asked for, so
/// leaving it acked would strand it on the board forever.
pub async fn auto_complete_clean_report(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    badge: &str,
    name: Option<&str>,
    source: EventSource,
) -> ServiceResult<Vec<RoomSignal>> {
    let sql = format!(
        "UPDATE ht_hk_room_signals \
            SET sig_status = 'done', sig_done_badge = $1, sig_done_name = $2, \
                sig_done_at = NOW(), sig_done_source = '{}' \
          WHERE sig_room_id = $3 \
            AND sig_status IN ('open', 'acked') \
            AND sig_type = ANY($4) \
        RETURNING sig_id",
        SignalDoneSource::CleanReport.as_str()
    );
    let types: Vec<String> = CLEAN_REPORT_AUTO_COMPLETE_TYPES
        .iter()
        .map(|t| (*t).to_string())
        .collect();
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(badge)
        .bind(name)
        .bind(room_id)
        .bind(&types)
        .fetch_all(&mut **tx)
        .await?;

    let mut completed_signals = Vec::with_capacity(rows.len());
    for row in &rows {
        let sig_id: i64 = row.try_get("sig_id")?;
        let signal = load_signal(tx, sig_id).await?;
        publish(tx, completed(&signal, source.clone())).await?;
        completed_signals.push(signal);
    }
    Ok(completed_signals)
}

// ============================================================================
// Row plumbing
// ============================================================================

/// What the `FOR UPDATE` probe brings back — everything the transition table
/// needs, and nothing else.
struct LockedSignal {
    room_id: i32,
    direction: SignalDirection,
    signal_type: String,
    status: SignalStatus,
}

/// Arguments for one INSERT. A struct rather than seven positional parameters
/// so a future edit cannot silently swap `badge` and `name`.
struct InsertSignal<'a> {
    room_id: i32,
    direction: SignalDirection,
    signal_type: &'a str,
    parent_id: Option<i64>,
    badge: &'a str,
    name: Option<&'a str>,
}

async fn insert_signal(
    tx: &mut Transaction<'_, Postgres>,
    args: InsertSignal<'_>,
) -> ServiceResult<RoomSignal> {
    let row = sqlx::query(
        "INSERT INTO ht_hk_room_signals \
             (sig_room_id, sig_direction, sig_type, sig_status, sig_parent_id, \
              sig_created_badge, sig_created_name) \
         VALUES ($1, $2, $3, 'open', $4, $5, $6) \
         RETURNING sig_id",
    )
    .bind(args.room_id)
    .bind(args.direction.as_str())
    .bind(args.signal_type)
    .bind(args.parent_id)
    .bind(args.badge)
    .bind(args.name)
    .fetch_one(&mut **tx)
    .await
    .map_err(insert_error)?;
    let sig_id: i64 = row.try_get("sig_id")?;
    load_signal(tx, sig_id).await
}

/// A foreign-key violation on `sig_room_id` means the room vanished between the
/// route's 404 probe and this INSERT. That is a 404, not a 500 — the FK is the
/// backstop the route's probe already covers.
fn insert_error(err: sqlx::Error) -> ServiceError {
    if let sqlx::Error::Database(ref db) = err {
        if db.constraint() == Some("ht_hk_room_signals_sig_room_id_fkey") {
            return ServiceError::not_found("room not found");
        }
    }
    ServiceError::Repository(err)
}

async fn lock_signal(
    tx: &mut Transaction<'_, Postgres>,
    signal_id: i64,
) -> ServiceResult<LockedSignal> {
    let row = sqlx::query(
        "SELECT sig_room_id, sig_direction, sig_type, sig_status \
           FROM ht_hk_room_signals WHERE sig_id = $1 FOR UPDATE",
    )
    .bind(signal_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| ServiceError::not_found(format!("signal {signal_id} not found")))?;

    let direction_raw: String = row.try_get("sig_direction")?;
    let status_raw: String = row.try_get("sig_status")?;
    Ok(LockedSignal {
        room_id: row.try_get("sig_room_id")?,
        // Both columns carry a DB CHECK, so an unparseable value means the
        // constraint was hand-edited away — an internal error, never a 400 the
        // caller could act on.
        direction: SignalDirection::parse(&direction_raw).ok_or_else(|| {
            ServiceError::internal(format!("signal {signal_id} has unknown direction {direction_raw:?}"))
        })?,
        signal_type: row.try_get("sig_type")?,
        status: SignalStatus::parse(&status_raw).ok_or_else(|| {
            ServiceError::internal(format!("signal {signal_id} has unknown status {status_raw:?}"))
        })?,
    })
}

async fn apply_ack(
    tx: &mut Transaction<'_, Postgres>,
    signal_id: i64,
    badge: &str,
    name: Option<&str>,
) -> ServiceResult<RoomSignal> {
    sqlx::query(
        "UPDATE ht_hk_room_signals \
            SET sig_status = 'acked', sig_acked_badge = $1, sig_acked_name = $2, \
                sig_acked_at = NOW() \
          WHERE sig_id = $3",
    )
    .bind(badge)
    .bind(name)
    .bind(signal_id)
    .execute(&mut **tx)
    .await?;
    load_signal(tx, signal_id).await
}

async fn apply_done(
    tx: &mut Transaction<'_, Postgres>,
    signal_id: i64,
    badge: &str,
    name: Option<&str>,
    done_source: SignalDoneSource,
    outcome: Option<RoomCheckOutcome>,
) -> ServiceResult<RoomSignal> {
    sqlx::query(
        "UPDATE ht_hk_room_signals \
            SET sig_status = 'done', sig_done_badge = $1, sig_done_name = $2, \
                sig_done_at = NOW(), sig_done_source = $3, \
                sig_outcome = COALESCE($4, sig_outcome) \
          WHERE sig_id = $5",
    )
    .bind(badge)
    .bind(name)
    .bind(done_source.as_str())
    .bind(outcome.map(RoomCheckOutcome::as_str))
    .bind(signal_id)
    .execute(&mut **tx)
    .await?;
    load_signal(tx, signal_id).await
}

/// Cancel stamps NO actor columns.
///
/// `sig_acked_*` would be a lie (nobody took it) and `sig_done_*` doubly so —
/// a cancelled signal was never done, and the boards' "completed by" reads
/// straight off those columns. Who withdrew it is recoverable from the
/// creator's side, which is the only side allowed to cancel.
async fn apply_cancel(
    tx: &mut Transaction<'_, Postgres>,
    signal_id: i64,
) -> ServiceResult<RoomSignal> {
    sqlx::query("UPDATE ht_hk_room_signals SET sig_status = 'cancelled' WHERE sig_id = $1")
        .bind(signal_id)
        .execute(&mut **tx)
        .await?;
    load_signal(tx, signal_id).await
}

async fn load_signal(
    tx: &mut Transaction<'_, Postgres>,
    signal_id: i64,
) -> ServiceResult<RoomSignal> {
    let sql = format!("{SIGNAL_SELECT} WHERE s.sig_id = $1");
    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(signal_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| ServiceError::not_found(format!("signal {signal_id} not found")))?;
    signal_from_row(&row)
}

/// Row → DTO. The ONE mapping; every read path goes through it.
fn signal_from_row(row: &sqlx::postgres::PgRow) -> ServiceResult<RoomSignal> {
    let direction_raw: String = row.try_get("sig_direction")?;
    let status_raw: String = row.try_get("sig_status")?;
    let outcome_raw: Option<String> = row.try_get("sig_outcome")?;
    let done_source_raw: Option<String> = row.try_get("sig_done_source")?;

    Ok(RoomSignal {
        signal_id: row.try_get("sig_id")?,
        room_id: row.try_get("sig_room_id")?,
        room_no: row.try_get("room_no")?,
        direction: SignalDirection::parse(&direction_raw).ok_or_else(|| {
            ServiceError::internal(format!("unknown sig_direction {direction_raw:?}"))
        })?,
        signal_type: row.try_get("sig_type")?,
        status: SignalStatus::parse(&status_raw)
            .ok_or_else(|| ServiceError::internal(format!("unknown sig_status {status_raw:?}")))?,
        // `sig_outcome` / `sig_done_source` carry NO CHECK (app-owned
        // vocabulary, migration 089), so an unrecognised value is possible in
        // principle. Serve it as absent rather than 500 the whole board: the
        // signal's status is still authoritative, and one odd row must not make
        // the list endpoint unusable.
        outcome: outcome_raw.as_deref().and_then(RoomCheckOutcome::parse),
        parent_id: row.try_get("sig_parent_id")?,
        created_by: SignalActor {
            badge: row.try_get("sig_created_badge")?,
            name: row.try_get("sig_created_name")?,
        },
        created_at: rfc3339(row.try_get("sig_created_at")?),
        acked_by: actor(
            row.try_get("sig_acked_badge")?,
            row.try_get("sig_acked_name")?,
        ),
        acked_at: row.try_get::<Option<DateTime<Utc>>, _>("sig_acked_at")?.map(rfc3339),
        done_by: actor(row.try_get("sig_done_badge")?, row.try_get("sig_done_name")?),
        done_at: row.try_get::<Option<DateTime<Utc>>, _>("sig_done_at")?.map(rfc3339),
        done_source: done_source_raw.as_deref().and_then(SignalDoneSource::parse),
    })
}

/// An actor exists exactly when its BADGE does — the name is a snapshot that
/// may legitimately be NULL, so keying on it would drop real acks.
fn actor(badge: Option<String>, name: Option<String>) -> Option<SignalActor> {
    badge.map(|badge| SignalActor { badge, name })
}

fn rfc3339(ts: DateTime<Utc>) -> String {
    ts.to_rfc3339()
}

/// A verified badge is ALWAYS present on both surfaces (the Access middleware
/// 401s without one; the desk falls back to its operator label). Blank means a
/// caller assembled a command by hand — refuse rather than write an
/// unattributable row into what is an audit record behind guest charges.
fn require_badge(raw: &str) -> ServiceResult<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ServiceError::validation(
            "a room signal requires a non-empty verified badge",
        ));
    }
    Ok(trimmed.to_string())
}

/// Domain rule → service error, honouring the domain's own 403-vs-400 split.
fn rule_error(err: SignalRuleError) -> ServiceError {
    let message = err.message();
    if err.is_forbidden() {
        ServiceError::Forbidden(message)
    } else if matches!(err, SignalRuleError::InvalidTransition { .. }) {
        // A precondition failure, surfaced as Conflict for observability — the
        // repo's shared mapping renders it 400 (see the module header).
        ServiceError::Conflict(message)
    } else {
        ServiceError::Validation(message)
    }
}

// ============================================================================
// Events
// ============================================================================

/// The aggregate a signal event belongs to is the ROOM, not the signal.
///
/// `event_log`'s `(aggregate_id, created_at DESC)` index is what reconstructs
/// an entity's history, and the entity a receptionist reasons about is "room
/// 104", not "signal 8871" — the same choice `RoomMarkedClean` and
/// `RoomCleaningStarted` already make, so one room's whole housekeeping story
/// stays a single index scan.
fn room_aggregate(signal: &RoomSignal) -> Uuid {
    aggregate_uuid(AggregateKind::Room, signal.room_id)
}

fn raised(signal: &RoomSignal, source: EventSource) -> DomainEvent {
    DomainEvent::RoomSignalRaised {
        room_id: room_aggregate(signal),
        signal: signal.clone(),
        source,
    }
}

fn completed(signal: &RoomSignal, source: EventSource) -> DomainEvent {
    DomainEvent::RoomSignalCompleted {
        room_id: room_aggregate(signal),
        signal: signal.clone(),
        source,
    }
}

/// Pick the event for a post-transition signal from the signal itself, so the
/// published name can never disagree with the row that was written.
fn event_for_status(signal: &RoomSignal, source: EventSource) -> DomainEvent {
    let room_id = room_aggregate(signal);
    match signal.status {
        SignalStatus::Acked => DomainEvent::RoomSignalAcked {
            room_id,
            signal: signal.clone(),
            source,
        },
        SignalStatus::Cancelled => DomainEvent::RoomSignalCancelled {
            room_id,
            signal: signal.clone(),
            source,
        },
        // `Done` is the only remaining transition target; `Open` is
        // unreachable (see `act`), and mapping it to Raised would be wrong, so
        // both fall to the completion event the row's own status implies.
        SignalStatus::Done | SignalStatus::Open => DomainEvent::RoomSignalCompleted {
            room_id,
            signal: signal.clone(),
            source,
        },
    }
}

async fn publish(tx: &mut Transaction<'_, Postgres>, event: DomainEvent) -> ServiceResult<()> {
    EventBus::publish(tx, &event)
        .await
        .map_err(|err| ServiceError::outbox(err.to_string()))?;
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::hk_signal::{ITEM_MISSING, PRIORITY_CLEAN, ROOM_CHECK};

    fn dto(status: SignalStatus) -> RoomSignal {
        RoomSignal {
            signal_id: 1,
            room_id: 42,
            room_no: "104".to_string(),
            direction: SignalDirection::DeskToMaid,
            signal_type: ROOM_CHECK.to_string(),
            status,
            outcome: None,
            parent_id: None,
            created_by: SignalActor {
                badge: "Q1".to_string(),
                name: None,
            },
            created_at: "2026-09-01T03:00:00Z".to_string(),
            acked_by: None,
            acked_at: None,
            done_by: None,
            done_at: None,
            done_source: None,
        }
    }

    fn src() -> EventSource {
        EventSource::our_app(Uuid::nil(), Uuid::nil())
    }

    #[test]
    fn a_blank_badge_is_refused_before_any_sql() {
        for blank in ["", "   ", "\t"] {
            assert!(matches!(
                require_badge(blank),
                Err(ServiceError::Validation(_))
            ));
        }
        assert_eq!(require_badge("  Q1001 ").unwrap(), "Q1001");
    }

    /// The 403-vs-400 split is the domain's decision; this mapping must not
    /// re-decide it. Role refusals ⇒ `Forbidden`; a wrong-status transition ⇒
    /// `Conflict` (which the shared `From<ServiceError>` renders 400).
    #[test]
    fn rule_errors_keep_the_domains_own_status_class() {
        assert!(matches!(
            rule_error(SignalRuleError::NotYourDirection {
                role: SignalRole::Maid,
                direction: SignalDirection::MaidToDesk
            }),
            ServiceError::Forbidden(_)
        ));
        assert!(matches!(
            rule_error(SignalRuleError::WrongDirectionForRole {
                role: SignalRole::Desk,
                signal_type: ITEM_MISSING.to_string()
            }),
            ServiceError::Forbidden(_)
        ));
        assert!(matches!(
            rule_error(SignalRuleError::InvalidTransition {
                action: SignalAction::Ack,
                from: SignalStatus::Done
            }),
            ServiceError::Conflict(_)
        ));
        assert!(matches!(
            rule_error(SignalRuleError::UnknownType("nope".into())),
            ServiceError::Validation(_)
        ));
        assert!(matches!(
            rule_error(SignalRuleError::RoomCheckNeedsAnswer),
            ServiceError::Validation(_)
        ));
    }

    /// `ServiceError::Conflict` renders 400 in this repo (see `service::error`)
    /// — pinned here so the module header's claim cannot quietly go stale.
    #[test]
    fn a_wrong_status_transition_reaches_http_as_400_and_a_role_refusal_as_403() {
        use crate::error::ApiError;
        let conflict: ApiError = rule_error(SignalRuleError::InvalidTransition {
            action: SignalAction::Done,
            from: SignalStatus::Cancelled,
        })
        .into();
        assert!(matches!(conflict, ApiError::BadRequest(_)));

        let forbidden: ApiError = rule_error(SignalRuleError::NotYourDirection {
            role: SignalRole::Maid,
            direction: SignalDirection::MaidToDesk,
        })
        .into();
        assert!(matches!(forbidden, ApiError::Forbidden(_)));
    }

    /// The published event name is derived from the ROW's status, never from
    /// the requested action — so a name can never disagree with what was
    /// written.
    #[test]
    fn the_event_name_follows_the_written_status() {
        assert_eq!(
            event_for_status(&dto(SignalStatus::Acked), src()).type_name(),
            "RoomSignalAcked"
        );
        assert_eq!(
            event_for_status(&dto(SignalStatus::Cancelled), src()).type_name(),
            "RoomSignalCancelled"
        );
        assert_eq!(
            event_for_status(&dto(SignalStatus::Done), src()).type_name(),
            "RoomSignalCompleted"
        );
        assert_eq!(raised(&dto(SignalStatus::Open), src()).type_name(), "RoomSignalRaised");
        assert_eq!(
            completed(&dto(SignalStatus::Done), src()).type_name(),
            "RoomSignalCompleted"
        );
    }

    /// Every signal event is keyed on the ROOM aggregate, so one room's
    /// housekeeping history stays a single `event_log` index scan alongside
    /// `RoomMarkedClean` / `RoomCleaningStarted`.
    #[test]
    fn signal_events_are_keyed_on_the_room_aggregate() {
        let expected = aggregate_uuid(AggregateKind::Room, 42);
        for event in [
            raised(&dto(SignalStatus::Open), src()),
            event_for_status(&dto(SignalStatus::Acked), src()),
            completed(&dto(SignalStatus::Done), src()),
        ] {
            assert_eq!(event.aggregate_id(), expected);
        }
    }

    /// The auto-complete scope is exactly the two cleaning-urgency types.
    /// ขอเช็คห้อง must never be in it — closing it on a cleaning tap would
    /// answer เคลียร์ on the maid's behalf while a guest waits at the counter.
    #[test]
    fn the_clean_report_sweep_never_touches_a_room_check() {
        assert!(CLEAN_REPORT_AUTO_COMPLETE_TYPES.contains(&PRIORITY_CLEAN));
        assert!(!CLEAN_REPORT_AUTO_COMPLETE_TYPES.contains(&ROOM_CHECK));
    }

    /// An actor is keyed on the BADGE, not the name: a real ack whose
    /// display-name snapshot is NULL must still render "who's on it".
    #[test]
    fn an_actor_exists_when_the_badge_does() {
        assert_eq!(
            actor(Some("Q1".into()), None),
            Some(SignalActor {
                badge: "Q1".into(),
                name: None
            })
        );
        assert_eq!(actor(None, Some("ignored".into())), None);
        assert_eq!(actor(None, None), None);
    }

    /// The projection is one constant so every read serves the same shape —
    /// pinned because a hand-written variant that forgot a column would deliver
    /// a DTO with silently missing fields.
    #[test]
    fn the_projection_selects_every_dto_column() {
        for column in [
            "s.sig_id",
            "s.sig_room_id",
            "r.room_no",
            "s.sig_direction",
            "s.sig_type",
            "s.sig_status",
            "s.sig_outcome",
            "s.sig_parent_id",
            "s.sig_created_badge",
            "s.sig_created_name",
            "s.sig_created_at",
            "s.sig_acked_badge",
            "s.sig_acked_name",
            "s.sig_acked_at",
            "s.sig_done_badge",
            "s.sig_done_name",
            "s.sig_done_at",
            "s.sig_done_source",
        ] {
            assert!(
                SIGNAL_SELECT.contains(column),
                "{column} missing from SIGNAL_SELECT"
            );
        }
    }

    // ---- today's answered room checks ------------------------------------

    /// The day boundary is the CLEANING one with the column swapped — exactly
    /// how `TODAY_BKK_LINEN` was built from `TODAY_BKK`, and pinned for the
    /// same reason: reception's "answered today" and the maid's "cleaned today"
    /// must roll over at the same instant, or the desk's green เคลียร์ outlives
    /// (or predates) the housekeeping day that explains it.
    #[test]
    fn today_bkk_signal_done_is_the_cleaning_day_boundary() {
        use crate::routes::hk::{TODAY_BKK, TODAY_BKK_DATE};
        assert_eq!(
            TODAY_BKK_SIGNAL_DONE,
            TODAY_BKK.replace("hkev_created_at", "sig_done_at"),
            "the answered-check day boundary must be the cleaning one, column swapped"
        );
        assert!(TODAY_BKK_SIGNAL_DONE.contains(TODAY_BKK_DATE));
        assert!(
            !TODAY_BKK_SIGNAL_DONE.contains("CURRENT_DATE"),
            "CURRENT_DATE is the SERVER's date and names yesterday in Bangkok \
             between 17:00 and 24:00 UTC"
        );
        assert!(answered_room_checks_today_sql().contains(TODAY_BKK_SIGNAL_DONE));
    }

    /// The answered read serves the SAME DTO as every other read — it goes
    /// through `signal_from_row`, so a projection that dropped a column would
    /// hand reception a signal with silently missing fields.
    #[test]
    fn the_answered_query_reuses_the_one_projection() {
        let sql = answered_room_checks_today_sql();
        assert!(
            SIGNAL_SELECT.starts_with("SELECT "),
            "the DISTINCT ON is spliced after this exact prefix"
        );
        for column in [
            "s.sig_id",
            "s.sig_room_id",
            "r.room_no",
            "s.sig_direction",
            "s.sig_type",
            "s.sig_status",
            "s.sig_outcome",
            "s.sig_parent_id",
            "s.sig_created_badge",
            "s.sig_created_name",
            "s.sig_created_at",
            "s.sig_acked_badge",
            "s.sig_acked_name",
            "s.sig_acked_at",
            "s.sig_done_badge",
            "s.sig_done_name",
            "s.sig_done_at",
            "s.sig_done_source",
        ] {
            assert!(sql.contains(column), "{column} missing from the answered read");
        }
        assert_eq!(
            sql.matches("FROM ht_hk_room_signals s").count(),
            1,
            "one scan of the signal table, not a self-join: {sql}"
        );
    }

    /// Every clause of the contract, pinned as SQL.
    ///
    /// The literals are read off `domain::hk_signal` in the builder, so this
    /// test is what proves the SQL and the enums agree: a rename of the
    /// `room_check_answer` code would fail HERE rather than quietly returning
    /// an empty list forever.
    #[test]
    fn the_answered_query_is_one_answered_room_check_per_room_today() {
        let sql = answered_room_checks_today_sql();
        assert!(
            sql.contains("SELECT DISTINCT ON (s.sig_room_id)"),
            "one entry per room max: {sql}"
        );
        assert!(sql.contains("s.sig_type = 'room_check'"), "{sql}");
        assert!(sql.contains("s.sig_status = 'done'"), "{sql}");
        assert!(sql.contains("s.sig_done_source = 'room_check_answer'"), "{sql}");
        assert!(sql.contains("s.sig_outcome IS NOT NULL"), "{sql}");
        // The DISTINCT ON tie-break picks the NEWEST answer for a room…
        assert!(
            sql.contains("ORDER BY s.sig_room_id, s.sig_done_at DESC, s.sig_id DESC"),
            "{sql}"
        );
        // …and the wrapper orders the rooms themselves newest-answer first.
        assert!(
            sql.trim_end()
                .ends_with("ORDER BY answered.sig_done_at DESC, answered.sig_id DESC"),
            "{sql}"
        );
    }

    /// A cancelled check must be UNREACHABLE by this query — the desk reading
    /// a withdrawn ขอเช็คห้อง as เคลียร์ is the one failure this feature must
    /// never produce, and `status = 'done'` is what excludes it.
    #[test]
    fn a_cancelled_check_can_never_be_served_as_an_answer() {
        let sql = answered_room_checks_today_sql();
        assert!(
            !sql.contains(SignalStatus::Cancelled.as_str()),
            "'cancelled' must not appear anywhere in the answered read: {sql}"
        );
        assert!(
            !sql.contains("sig_status IN"),
            "the status predicate is EQUALITY on 'done', never a set that could \
             be widened by accident: {sql}"
        );
        // The other two done-sources are equally excluded: a `clean_report`
        // sweep never touches a room_check, and a `tap` on one is refused
        // outright by the domain — so a row with either source on a room_check
        // is corruption, not an answer.
        assert!(!sql.contains(SignalDoneSource::Tap.as_str()));
        assert!(!sql.contains(SignalDoneSource::CleanReport.as_str()));
    }
}
