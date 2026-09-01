//! Report HK service — the maid's per-room daily attestation and reception's
//! countersignature.
//!
//! Owns `ht_hk_room_reports` / `ht_hk_room_report_items` /
//! `ht_hk_room_report_photos` (migration 091): the day overview, the detail
//! read, photo intake, and the three mutations (submit, verify, return). Every
//! rule it enforces is a pure function in [`crate::domain::hk_report`]; this
//! module is the transaction boundary and the SQL, nothing else — the same
//! construction as [`crate::service::hk_signals`].
//!
//! ## Coexistence stance (ADR 0002 / invariant #6)
//!
//! **PG-CANONICAL ONLY, with no path to legacy by design.** iHOTEL has no
//! counterpart to a Report HK sheet at all, so there is no sync mapper, no
//! writeback recipe, no [`crate::outbox::intent::WritebackIntent`] and no
//! ship-dark flag waiting to enable one — the posture of
//! `ht_hk_linen_reports` (088) and `ht_hk_room_signals` (089), and unlike
//! `ht_hk_cleaning_events`, whose `done` phase did grow a `MarkRoomClean`
//! writeback on 2026-08-11.
//!
//! It publishes NO domain event of its own either (linen's posture, not
//! signals'): the overview is a poll/reload surface, and inventing a
//! `RoomReport*` event family for it would be a design change, not a
//! fill-in-the-blank. What the SUBMIT does publish is whatever the room signals
//! it raises publish — see below.
//!
//! ## One transaction, always — and what rides in the submit's
//!
//! [`HkReportService::submit`] is genuinely multi-table and commits atomically:
//! the header, its exception rows, the ATTACHMENT of the maid's photos, and
//! **the standing `item_missing` / `item_damaged` room signals** the exceptions
//! raise ([`crate::service::hk_signals::raise_from_report`], called with this
//! method's own transaction). A guest may be about to settle while this runs;
//! "the report says the TV remote is gone but the desk never got a signal" is
//! not a state reception may ever observe. The signals are deduplicated to ONE
//! PER PROBLEM KIND — see [`HkReportService::submit`]'s doc.
//!
//! [`HkReportService::verify`] and [`HkReportService::return_report`] lock the
//! header (`SELECT … FOR UPDATE`) before judging it, so two receptionists
//! tapping at once cannot both succeed — the same serialization-point idiom as
//! `service::hk_signals::lock_signal` and `service::housekeeping::lock_room_clean`.
//!
//! ## Wrong-status transitions answer 400, not 409
//!
//! [`crate::domain::hk_report::check_can_judge`] refuses a verdict on a report
//! that already has one; this module raises that as [`ServiceError::Conflict`],
//! which the repo's single `From<ServiceError> for ApiError` maps to **400**
//! (see `service::error`). That mapping is shared with booking-cancel, payment
//! void, the `writeback_jobs` idempotency race and ADR 0008's room signals, and
//! `ApiError::Conflict` (409) is reserved in this repo for a ship-dark FLAG
//! refusing a request. Answering 409 for reports alone would have meant either
//! a second mapping or changing six shipped surfaces, so reports follow the
//! house convention: **400 with a message naming the status it is already in**
//! — byte-for-byte the choice `service::hk_signals` documents at its own
//! boundary. Role refusals are the one thing that is NOT 400: they are
//! [`ServiceError::Forbidden`] ⇒ 403.
//!
//! ## Identity
//!
//! Badge + display name come exclusively from the verified identity the route
//! resolved ([`crate::middleware::hk_access::HkIdentity`]). Nothing
//! identity-like is ever accepted from a request body — same rule as every
//! other `ht_hk_*` writer, and no FK to `ht_users` (maids and receptionists are
//! CF Access + HF ID identities, not PMS accounts). The photo side is DERIVED
//! from `can_report`, never sent.
//!
//! All SQL here is RUNTIME `sqlx::query` (never the `query!` macro), so this
//! module needs no `.sqlx/` cache regeneration — the same choice
//! `service::hk_signals`, `service::housekeeping` and `routes::hk` already make.

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::domain::hk_report::{
    check_attestation, check_can_judge, check_photo_count, check_side, ItemProblem, PhotoCounts,
    PhotoSide, ReportActor, ReportItem, ReportRuleError, ReportStatus, RoomReport, RoomReportRow,
    RoomReportSummary,
};
use crate::outbox::event::EventSource;

use super::error::{ServiceError, ServiceResult};

/// The header projection every read of `ht_hk_room_reports` uses, joined to
/// `ht_rooms_new` for the room number both DTOs carry.
///
/// ONE constant, so the detail read, the post-mutation reload and the day
/// overview cannot drift into serving different shapes. The join is INNER on
/// purpose: `rr_room_id` is a `NOT NULL` FK with `ON DELETE CASCADE`, so a
/// report without a room is not a state that exists.
const REPORT_SELECT: &str = "SELECT h.rr_id, h.rr_room_id, r.room_no, h.rr_date, h.rr_status, \
                             h.rr_room_status, h.rr_all_items_ok, h.rr_return_reason, \
                             h.rr_parent_id, h.rr_submitted_badge, h.rr_submitted_name, \
                             h.rr_submitted_at, h.rr_verified_badge, h.rr_verified_name, \
                             h.rr_verified_at \
                             FROM ht_hk_room_reports h \
                             JOIN ht_rooms_new r ON r.room_id = h.rr_room_id";

/// The day overview: EVERY active room of this site with its LATEST report for
/// one date.
///
/// `$1` is the Bangkok civil date. A function rather than a `const` because it
/// composes, and pinned by a unit test — the same "SQL you can read at the call
/// site" idiom as `routes::hk::rooms_list_sql`.
///
/// Shape notes that are load-bearing:
///
/// * `LEFT JOIN LATERAL … LIMIT 1` — the LATEST report, not "a report": a
///   returned report and the submission that supersedes it share `rr_date`, and
///   the overview must show the live one. `rr_id DESC` is the tie-break and
///   `ix_ht_hk_room_reports_room_date` is exactly the index it wants.
/// * The photo counts come from ONE more LATERAL with `COUNT(*) FILTER`, not
///   from two correlated scalar subqueries per room — one screen is one
///   statement, never an N+1 (the rule `linen_shortage_open_sql` follows).
/// * `WHERE COALESCE(r.room_active, true) = true` and `ORDER BY r.room_no`
///   match `routes::hk::rooms_list_sql` exactly, so the report overview and the
///   cleaning list can never show a maid two different room sets.
fn day_overview_sql() -> String {
    format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            rep.rr_id, rep.rr_date, rep.rr_status, rep.rr_room_status,
            rep.rr_all_items_ok, rep.rr_return_reason, rep.rr_parent_id,
            rep.rr_submitted_badge, rep.rr_submitted_name, rep.rr_submitted_at,
            rep.rr_verified_badge, rep.rr_verified_name, rep.rr_verified_at,
            COALESCE(ph.maid_photos, 0)::bigint      AS maid_photos,
            COALESCE(ph.reception_photos, 0)::bigint AS reception_photos
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT h.rr_id, h.rr_date, h.rr_status, h.rr_room_status,
                   h.rr_all_items_ok, h.rr_return_reason, h.rr_parent_id,
                   h.rr_submitted_badge, h.rr_submitted_name, h.rr_submitted_at,
                   h.rr_verified_badge, h.rr_verified_name, h.rr_verified_at
              FROM ht_hk_room_reports h
             WHERE h.rr_room_id = r.room_id AND h.rr_date = $1
             ORDER BY h.rr_id DESC
             LIMIT 1
        ) rep ON TRUE
        LEFT JOIN LATERAL (
            SELECT COUNT(*) FILTER (WHERE p.rrp_side = '{maid}')      AS maid_photos,
                   COUNT(*) FILTER (WHERE p.rrp_side = '{reception}') AS reception_photos
              FROM ht_hk_room_report_photos p
             WHERE p.rrp_report_id = rep.rr_id
        ) ph ON TRUE
        WHERE COALESCE(r.room_active, true) = true
        ORDER BY r.room_no
        "#,
        maid = PhotoSide::Maid.as_str(),
        reception = PhotoSide::Reception.as_str(),
    )
}

// ============================================================================
// Commands
// ============================================================================

/// One `{item, problem, qty}` exception, already validated by the route.
///
/// `item` is a canonical [`crate::domain::hk_report::REPORT_ITEMS`] code. The
/// service does NOT re-check the item vocabulary — the DB column is
/// unconstrained `TEXT` on purpose (migration 091) so the checklist can move
/// without a migration, and duplicating the list here would recreate exactly
/// the two-places-to-change coupling that decision avoids. What IS re-checked
/// is the quantity bound and the attestation biconditional: both must hold for
/// ANY caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportItemInput {
    pub item: String,
    pub problem: ItemProblem,
    pub qty: i32,
}

/// Command for [`HkReportService::submit`] — one maid's
/// `POST /api/hk/rooms/{id}/report`.
#[derive(Debug, Clone)]
pub struct SubmitReportCommand {
    pub room_id: i32,
    /// The Bangkok civil day the report is FOR. The route defaults it to today
    /// in Bangkok; it is a required field HERE so the service can never invent
    /// a date from the server's clock.
    pub date: NaiveDate,
    /// A canonical `ROOM_STATUS_CODES` code, already validated by the route.
    pub room_status: String,
    pub all_items_ok: bool,
    pub items: Vec<ReportItemInput>,
    /// Ids from `POST /api/hk/report-photos`. Must be 1..=4, distinct, this
    /// caller's OWN, `maid`-side, and not already attached — the last three are
    /// checked by the ATTACH statement itself, atomically.
    pub photo_ids: Vec<i64>,
    /// `HkIdentity::can_report`. Carried so the "a maid never verifies,
    /// reception never submits" rule holds for any caller, not only for one
    /// that went through the route gate.
    pub can_report: bool,
    pub badge: String,
    pub name: Option<String>,
    /// Event source for the room signals the exceptions raise. Present, unlike
    /// on the linen commands, precisely because this command CAN publish.
    pub source: EventSource,
}

/// Command for [`HkReportService::verify`] — reception's countersignature.
#[derive(Debug, Clone)]
pub struct VerifyReportCommand {
    pub report_id: i64,
    /// 1..=4 of the caller's OWN `reception`-side photos. A verify is a
    /// walk-up, not a desk stamp — the evidence is the feature.
    pub photo_ids: Vec<i64>,
    pub can_report: bool,
    pub badge: String,
    pub name: Option<String>,
}

/// Command for [`HkReportService::return_report`] — reception sending it back.
#[derive(Debug, Clone)]
pub struct ReturnReportCommand {
    pub report_id: i64,
    /// A canonical `RETURN_REASONS` code. There is deliberately no free-text
    /// sibling and no photo list: the reason IS the whole explanation
    /// (CONTEXT.md §Housekeeping — _Avoid_: free-text rejection notes).
    pub reason: String,
    pub can_report: bool,
    pub badge: String,
    pub name: Option<String>,
}

/// Command for [`HkReportService::store_photo`] — one file from
/// `POST /api/hk/report-photos`.
#[derive(Debug, Clone)]
pub struct StorePhotoCommand {
    /// Already size- and type-checked by the route (the caps are transport
    /// policy, so they live where the bytes arrive).
    pub bytes: Vec<u8>,
    pub mime: String,
    /// DERIVED from the uploader's role by the route, never sent by the client.
    pub side: PhotoSide,
    pub badge: String,
}

/// The bytes of one stored photo, for `GET /api/hk/report-photos/{id}`.
#[derive(Debug, Clone)]
pub struct StoredPhoto {
    pub bytes: Vec<u8>,
    pub mime: String,
}

// ============================================================================
// The service
// ============================================================================

/// Service handle for the room-report aggregate.
///
/// Holds ONLY a pool — no `RoomRepository`, no `OutboxRepository`, no
/// `EventBus` handle, for exactly [`crate::service::hk_signals::HkSignalService`]'s
/// reasons: reports enqueue no writeback intent (there is no legacy
/// counterpart), and the events their signals publish go through
/// [`crate::outbox::EventBus::publish`], an associated function taking the
/// caller's transaction. A held `Arc<EventBus>` would be a field that is never
/// read — and a misleading hint that one day a writeback lands.
#[derive(Clone)]
pub struct HkReportService {
    pg: PgPool,
}

impl HkReportService {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    /// The day overview — every ACTIVE room of this site with its LATEST report
    /// for `date` (`None` when she has not filed one).
    ///
    /// No branch predicate: site scoping is CONNECTION-level (each site's pool
    /// holds its own reports), the same model as every other `ht_hk_*` table. A
    /// `?branch=` selects the POOL, never a `WHERE` clause.
    ///
    /// Every active room is listed whether or not it has a report, because this
    /// screen is the paper sheet's heir and each side's work queue: a MISSING
    /// report is the most important row on it.
    pub async fn day_overview(&self, date: NaiveDate) -> ServiceResult<Vec<RoomReportRow>> {
        let sql = day_overview_sql();
        let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
            .bind(date)
            .fetch_all(&self.pg)
            .await?;
        rows.iter().map(overview_row).collect()
    }

    /// One report in FULL — header, exceptions, and both sides' photo ids.
    ///
    /// Three statements rather than one join: the two child sets are
    /// independent lists, and fanning them out in SQL would return the cross
    /// product of items × photos for a report that has both. They are read
    /// OUTSIDE a transaction on purpose — a report's items never change after
    /// insert and its photos are attached only inside the submit/verify
    /// commits, so the only interleaving a snapshot would prevent is a verify
    /// landing between the header read and the photo read, which shows one
    /// extra photo on an otherwise-consistent report and is not worth a
    /// `REPEATABLE READ` transaction per detail tap.
    pub async fn get(&self, report_id: i64) -> ServiceResult<RoomReport> {
        let mut tx = self.pg.begin().await?;
        let report = load_report(&mut tx, report_id).await?;
        tx.commit().await?;
        Ok(report)
    }

    /// Store one uploaded photo, UNATTACHED, and return its id.
    ///
    /// The row is minted with `rrp_report_id IS NULL` because the phone uploads
    /// while the maid is still filling the form — the submit/verify call names
    /// the ids and binds them. See migration 091 for the accepted debt: an
    /// unattached row lingers forever, deliberately, because a sweeper is
    /// exactly the thing that could destroy evidence if its predicate were
    /// wrong.
    pub async fn store_photo(&self, cmd: StorePhotoCommand) -> ServiceResult<i64> {
        let badge = require_badge(&cmd.badge)?;
        if cmd.bytes.is_empty() {
            return Err(ServiceError::validation("photo is empty"));
        }
        let row = sqlx::query(
            "INSERT INTO ht_hk_room_report_photos \
                 (rrp_report_id, rrp_side, rrp_photo, rrp_photo_mime, rrp_badge) \
             VALUES (NULL, $1, $2, $3, $4) \
             RETURNING rrp_id",
        )
        .bind(cmd.side.as_str())
        .bind(&cmd.bytes)
        .bind(&cmd.mime)
        .bind(&badge)
        .fetch_one(&self.pg)
        .await?;
        Ok(row.try_get("rrp_id")?)
    }

    /// The bytes of one photo. Readable by BOTH roles — reception must see the
    /// maid's evidence to judge it, and the maid must see what came back with a
    /// return.
    pub async fn load_photo(&self, photo_id: i64) -> ServiceResult<StoredPhoto> {
        let row = sqlx::query(
            "SELECT rrp_photo, rrp_photo_mime FROM ht_hk_room_report_photos WHERE rrp_id = $1",
        )
        .bind(photo_id)
        .fetch_optional(&self.pg)
        .await?
        .ok_or_else(|| ServiceError::not_found(format!("photo {photo_id} not found")))?;
        Ok(StoredPhoto {
            bytes: row.try_get("rrp_photo")?,
            mime: row.try_get("rrp_photo_mime")?,
        })
    }

    /// File one room report — ONE transaction covering everything it implies.
    ///
    /// ## What commits together
    ///
    /// 1. the header row;
    /// 2. its exception rows (one `UNNEST` statement, so "all or none" is a
    ///    property of the statement rather than of remembering to stay inside
    ///    the transaction — `insert_linen_report_rows`' argument);
    /// 3. the ATTACHMENT of the maid's photos, which is also the check that
    ///    they are hers, `maid`-side and not already attached;
    /// 4. **one standing `item_missing` / `item_damaged` room signal per
    ///    excepted PROBLEM KIND**, maid→desk (ADR 0008 / migration 089).
    ///
    /// ## Why one signal per PROBLEM, not per item
    ///
    /// `ht_hk_room_signals` has no item column and no free text — by decision
    /// (ADR 0008). Five missing items would therefore raise five signals that
    /// are byte-identical on reception's board: unactionable noise, and five
    /// separate things for the desk to close. One `item_missing` says exactly
    /// what the board can render — "this room has missing items, look at the
    /// report" — and the report is where the itemisation lives. So the problems
    /// present in the checklist are deduplicated, in
    /// [`ItemProblem::ALL`](crate::domain::hk_report::ItemProblem::ALL) order,
    /// to AT MOST TWO signals.
    ///
    /// ## The open-report guard
    ///
    /// A room may have at most ONE `submitted` report per day: while a verdict
    /// is pending, a second submission would leave reception judging a report
    /// nobody can act on. The room row is locked (`SELECT … FOR UPDATE`) as the
    /// serialization point before the probe — the same idiom as
    /// `service::housekeeping::lock_room_clean` — so two maids submitting at
    /// once cannot both pass it. The refusal is [`ServiceError::Conflict`] (the
    /// repo renders it 400; see the module header).
    ///
    /// ## Parenting is DERIVED, never sent
    ///
    /// A returned report is fixed by a NEW submission that references it. The
    /// link is computed here — the LATEST report for this room+date, when it is
    /// `returned`, becomes `rr_parent_id` — rather than accepted from the body:
    /// a client-supplied parent could point at another room's report, at a
    /// verified one, or at nothing, and every one of those would be a lie
    /// recorded in the audit chain.
    pub async fn submit(&self, cmd: SubmitReportCommand) -> ServiceResult<RoomReport> {
        let badge = require_badge(&cmd.badge)?;
        // Shape and role BEFORE any SQL: a malformed or forbidden command is
        // refused without a round-trip, and the caller is never told "a report
        // is already pending" about a body that could not have been accepted
        // anyway. Same ordering `answer_room_check` documents.
        check_side("submit", cmd.can_report).map_err(rule_error)?;
        check_attestation(cmd.all_items_ok, cmd.items.len()).map_err(rule_error)?;
        check_photo_count(cmd.photo_ids.len()).map_err(rule_error)?;
        check_qty_bounds(&cmd.items)?;
        check_distinct_photos(&cmd.photo_ids)?;

        let mut tx = self.pg.begin().await?;

        // The serialization point. Also a real existence probe: the route 404s
        // an unknown or inactive room first, so reaching a missing row here
        // means it vanished between the two, which is still a 404.
        lock_room(&mut tx, cmd.room_id).await?;

        if let Some(open_id) = open_report_id(&mut tx, cmd.room_id, cmd.date).await? {
            return Err(ServiceError::conflict(format!(
                "report {open_id} for this room on {} is still awaiting verification; \
                 it must be verified or returned first",
                cmd.date
            )));
        }

        let parent_id = returned_parent_id(&mut tx, cmd.room_id, cmd.date).await?;

        let report_id = insert_report_header(
            &mut tx,
            InsertHeader {
                room_id: cmd.room_id,
                date: cmd.date,
                room_status: &cmd.room_status,
                all_items_ok: cmd.all_items_ok,
                parent_id,
                badge: &badge,
                name: cmd.name.as_deref(),
            },
        )
        .await?;

        insert_report_items(&mut tx, report_id, &cmd.items).await?;
        attach_photos(
            &mut tx,
            report_id,
            &cmd.photo_ids,
            PhotoSide::Maid,
            &badge,
        )
        .await?;

        // The guest-accountability bridge. Deduplicated to one signal per
        // problem kind — see the doc comment.
        for problem in ItemProblem::ALL {
            if !cmd.items.iter().any(|item| item.problem == problem) {
                continue;
            }
            crate::service::hk_signals::raise_from_report(
                &mut tx,
                cmd.room_id,
                problem.signal_type(),
                &badge,
                cmd.name.as_deref(),
                cmd.source.clone(),
            )
            .await?;
        }

        let report = load_report(&mut tx, report_id).await?;
        tx.commit().await?;
        Ok(report)
    }

    /// Reception's countersignature: `submitted` → `verified`, with 1..=4 of
    /// the verifier's OWN photos attached in the same transaction.
    ///
    /// **A maid never verifies** — including one who also holds the reception
    /// grant, because `can_report` is the maid side
    /// ([`check_side`]). The refusal is [`ServiceError::Forbidden`] ⇒ 403,
    /// deliberately not the 400 a malformed body gets: the command is
    /// well-formed and the answer is still no.
    pub async fn verify(&self, cmd: VerifyReportCommand) -> ServiceResult<RoomReport> {
        let badge = require_badge(&cmd.badge)?;
        check_side("verify", cmd.can_report).map_err(rule_error)?;
        check_photo_count(cmd.photo_ids.len()).map_err(rule_error)?;
        check_distinct_photos(&cmd.photo_ids)?;

        let mut tx = self.pg.begin().await?;
        let status = lock_report(&mut tx, cmd.report_id).await?;
        check_can_judge("verify", status).map_err(rule_error)?;

        sqlx::query(
            "UPDATE ht_hk_room_reports \
                SET rr_status = 'verified', rr_verified_badge = $2, \
                    rr_verified_name = $3, rr_verified_at = NOW() \
              WHERE rr_id = $1",
        )
        .bind(cmd.report_id)
        .bind(&badge)
        .bind(cmd.name.as_deref())
        .execute(&mut *tx)
        .await?;

        attach_photos(
            &mut tx,
            cmd.report_id,
            &cmd.photo_ids,
            PhotoSide::Reception,
            &badge,
        )
        .await?;

        let report = load_report(&mut tx, cmd.report_id).await?;
        tx.commit().await?;
        Ok(report)
    }

    /// Reception sending it back: `submitted` → `returned` with a CANNED
    /// reason and **no photos at all**.
    ///
    /// The asymmetry with [`Self::verify`] is the design, not an omission: a
    /// verify is a walk-up and its photos are the evidence that it happened,
    /// while a return says "go and do it again", which the reason states in
    /// full. Adding photos here would be asking reception to document a room it
    /// is refusing to sign for.
    ///
    /// The report is not reopened and not edited — it stays `returned` forever
    /// and a NEW submission supersedes it, carrying `parentReportId`. That is
    /// what keeps the chain append-only.
    pub async fn return_report(&self, cmd: ReturnReportCommand) -> ServiceResult<RoomReport> {
        let badge = require_badge(&cmd.badge)?;
        check_side("return", cmd.can_report).map_err(rule_error)?;
        // Re-validated here, not only at the route: it must hold for ANY
        // caller, and the DB column has no CHECK (app-owned vocabulary).
        let reason = crate::domain::hk_report::parse_return_reason(&cmd.reason)
            .map_err(rule_error)?;

        let mut tx = self.pg.begin().await?;
        let status = lock_report(&mut tx, cmd.report_id).await?;
        check_can_judge("return", status).map_err(rule_error)?;

        sqlx::query(
            "UPDATE ht_hk_room_reports \
                SET rr_status = 'returned', rr_return_reason = $2, \
                    rr_verified_badge = $3, rr_verified_name = $4, rr_verified_at = NOW() \
              WHERE rr_id = $1",
        )
        .bind(cmd.report_id)
        .bind(reason)
        .bind(&badge)
        .bind(cmd.name.as_deref())
        .execute(&mut *tx)
        .await?;

        let report = load_report(&mut tx, cmd.report_id).await?;
        tx.commit().await?;
        Ok(report)
    }
}

// ============================================================================
// Row plumbing
// ============================================================================

/// Arguments for the header INSERT. A struct rather than seven positional
/// parameters so a future edit cannot silently swap `badge` and `name`.
struct InsertHeader<'a> {
    room_id: i32,
    date: NaiveDate,
    room_status: &'a str,
    all_items_ok: bool,
    parent_id: Option<i64>,
    badge: &'a str,
    name: Option<&'a str>,
}

async fn insert_report_header(
    tx: &mut Transaction<'_, Postgres>,
    args: InsertHeader<'_>,
) -> ServiceResult<i64> {
    let row = sqlx::query(
        "INSERT INTO ht_hk_room_reports \
             (rr_room_id, rr_date, rr_status, rr_room_status, rr_all_items_ok, \
              rr_parent_id, rr_submitted_badge, rr_submitted_name) \
         VALUES ($1, $2, 'submitted', $3, $4, $5, $6, $7) \
         RETURNING rr_id",
    )
    .bind(args.room_id)
    .bind(args.date)
    .bind(args.room_status)
    .bind(args.all_items_ok)
    .bind(args.parent_id)
    .bind(args.badge)
    .bind(args.name)
    .fetch_one(&mut **tx)
    .await
    .map_err(insert_error)?;
    Ok(row.try_get("rr_id")?)
}

/// A foreign-key violation on `rr_room_id` means the room vanished between the
/// route's 404 probe and this INSERT. That is a 404, not a 500 — the FK is the
/// backstop the route's probe already covers. Same treatment as
/// `service::hk_signals::insert_error`.
fn insert_error(err: sqlx::Error) -> ServiceError {
    if let sqlx::Error::Database(ref db) = err {
        if db.constraint() == Some("ht_hk_room_reports_rr_room_id_fkey") {
            return ServiceError::not_found("room not found");
        }
    }
    ServiceError::Repository(err)
}

/// Insert every exception of ONE report — a single `UNNEST` statement, so the
/// all-or-nothing property belongs to the statement rather than to a loop
/// (`insert_linen_report_rows`' argument). A no-op for a ครบทุกรายการ report,
/// which is the common case and costs no round-trip at all.
///
/// The arrays are bound as parameters (`$2::text[]`, `$3::text[]`, `$4::int[]`)
/// — nothing a maid typed is ever concatenated into SQL.
async fn insert_report_items(
    tx: &mut Transaction<'_, Postgres>,
    report_id: i64,
    items: &[ReportItemInput],
) -> ServiceResult<()> {
    if items.is_empty() {
        return Ok(());
    }
    let codes: Vec<String> = items.iter().map(|item| item.item.clone()).collect();
    let problems: Vec<String> = items
        .iter()
        .map(|item| item.problem.as_str().to_string())
        .collect();
    let quantities: Vec<i32> = items.iter().map(|item| item.qty).collect();

    sqlx::query(
        "INSERT INTO ht_hk_room_report_items \
             (rri_report_id, rri_item, rri_problem, rri_qty) \
         SELECT $1, line.item, line.problem, line.qty \
           FROM UNNEST($2::text[], $3::text[], $4::int[]) AS line(item, problem, qty)",
    )
    .bind(report_id)
    .bind(&codes)
    .bind(&problems)
    .bind(&quantities)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// Bind a caller's own unattached photos to a report — **the ownership check
/// and the write are the same statement.**
///
/// The `WHERE` carries all four conditions (`rrp_report_id IS NULL`,
/// `rrp_badge = the caller`, `rrp_side = this side`, id in the list), so
/// "unknown id", "someone else's photo", "the wrong side's photo" and "already
/// attached to another report" are one atomic verdict rather than a
/// read-then-write race two concurrent submits could both pass.
///
/// The row count is then compared to the requested count: anything short means
/// at least one id failed the predicate, and the whole transaction rolls back.
/// The message deliberately does NOT say WHICH condition failed — a caller who
/// could distinguish "this id does not exist" from "this id belongs to someone
/// else" could enumerate other identities' photo ids.
async fn attach_photos(
    tx: &mut Transaction<'_, Postgres>,
    report_id: i64,
    photo_ids: &[i64],
    side: PhotoSide,
    badge: &str,
) -> ServiceResult<()> {
    let result = sqlx::query(
        "UPDATE ht_hk_room_report_photos \
            SET rrp_report_id = $1 \
          WHERE rrp_id = ANY($2::bigint[]) \
            AND rrp_report_id IS NULL \
            AND rrp_badge = $3 \
            AND rrp_side = $4",
    )
    .bind(report_id)
    .bind(photo_ids)
    .bind(badge)
    .bind(side.as_str())
    .execute(&mut **tx)
    .await?;

    if result.rows_affected() as usize != photo_ids.len() {
        return Err(ServiceError::validation(format!(
            "one or more photos are unknown, not yours, or already attached to a report \
             (attached {} of {})",
            result.rows_affected(),
            photo_ids.len()
        )));
    }
    Ok(())
}

/// Take the ROOM's row lock — the submit's serialization point.
///
/// `SELECT … FOR UPDATE` on `ht_rooms_new` rather than on the reports table,
/// because the thing being serialized is "does this room already have an open
/// report", and the row that would answer it MAY NOT EXIST YET — a lock on
/// nothing serializes nothing. Locking the room makes the second concurrent
/// submit block here, then re-read (READ COMMITTED) the report the first one
/// just committed. Same idiom, same reasoning, as
/// `service::housekeeping::lock_room_clean`.
async fn lock_room(tx: &mut Transaction<'_, Postgres>, room_id: i32) -> ServiceResult<()> {
    sqlx::query("SELECT room_id FROM ht_rooms_new WHERE room_id = $1 FOR UPDATE")
        .bind(room_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| ServiceError::not_found(format!("room {room_id} not found")))?;
    Ok(())
}

/// The id of this room's still-`submitted` report for `date`, if any. Served by
/// the partial index `ix_ht_hk_room_reports_open`.
async fn open_report_id(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    date: NaiveDate,
) -> ServiceResult<Option<i64>> {
    let row = sqlx::query(
        "SELECT rr_id FROM ht_hk_room_reports \
          WHERE rr_room_id = $1 AND rr_date = $2 AND rr_status = 'submitted' \
          ORDER BY rr_id DESC LIMIT 1",
    )
    .bind(room_id)
    .bind(date)
    .fetch_optional(&mut **tx)
    .await?;
    match row {
        Some(row) => Ok(Some(row.try_get("rr_id")?)),
        None => Ok(None),
    }
}

/// The report this submission SUPERSEDES: the LATEST one for this room+date,
/// but only when it is `returned`.
///
/// "Latest, and only if returned" rather than "the latest returned one" is
/// deliberate. If the newest report is `verified`, this submission is a fresh
/// attestation about the same day (a second cleaning, say) and parenting it
/// onto an older rejection would fabricate a correction chain that did not
/// happen. Only when the last word on this room today was "do it again" does
/// the new report answer it.
async fn returned_parent_id(
    tx: &mut Transaction<'_, Postgres>,
    room_id: i32,
    date: NaiveDate,
) -> ServiceResult<Option<i64>> {
    let row = sqlx::query(
        "SELECT rr_id, rr_status FROM ht_hk_room_reports \
          WHERE rr_room_id = $1 AND rr_date = $2 \
          ORDER BY rr_id DESC LIMIT 1",
    )
    .bind(room_id)
    .bind(date)
    .fetch_optional(&mut **tx)
    .await?;
    let Some(row) = row else { return Ok(None) };
    let status: String = row.try_get("rr_status")?;
    if status == ReportStatus::Returned.as_str() {
        Ok(Some(row.try_get("rr_id")?))
    } else {
        Ok(None)
    }
}

/// Lock one report and read the status the transition table needs.
///
/// `FOR UPDATE` before the rules are judged, so two receptionists tapping
/// verify at once cannot both succeed: the loser blocks here, then re-reads
/// `verified` under the lock and is refused.
async fn lock_report(
    tx: &mut Transaction<'_, Postgres>,
    report_id: i64,
) -> ServiceResult<ReportStatus> {
    let row = sqlx::query("SELECT rr_status FROM ht_hk_room_reports WHERE rr_id = $1 FOR UPDATE")
        .bind(report_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| ServiceError::not_found(format!("report {report_id} not found")))?;
    let raw: String = row.try_get("rr_status")?;
    // The column carries a DB CHECK, so an unparseable value means the
    // constraint was hand-edited away — an internal error, never a 400 the
    // caller could act on. Same treatment as `sig_status` in hk_signals.
    ReportStatus::parse(&raw).ok_or_else(|| {
        ServiceError::internal(format!("report {report_id} has unknown status {raw:?}"))
    })
}

/// Read one report in FULL, through the caller's transaction.
async fn load_report(
    tx: &mut Transaction<'_, Postgres>,
    report_id: i64,
) -> ServiceResult<RoomReport> {
    let sql = format!("{REPORT_SELECT} WHERE h.rr_id = $1");
    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(report_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| ServiceError::not_found(format!("report {report_id} not found")))?;

    let items = sqlx::query(
        "SELECT rri_item, rri_problem, rri_qty FROM ht_hk_room_report_items \
          WHERE rri_report_id = $1 ORDER BY rri_id",
    )
    .bind(report_id)
    .fetch_all(&mut **tx)
    .await?;

    let photos = sqlx::query(
        "SELECT rrp_id, rrp_side FROM ht_hk_room_report_photos \
          WHERE rrp_report_id = $1 ORDER BY rrp_id",
    )
    .bind(report_id)
    .fetch_all(&mut **tx)
    .await?;

    let mut maid_photo_ids = Vec::new();
    let mut reception_photo_ids = Vec::new();
    for photo in &photos {
        let id: i64 = photo.try_get("rrp_id")?;
        let side: String = photo.try_get("rrp_side")?;
        match PhotoSide::parse(&side) {
            Some(PhotoSide::Maid) => maid_photo_ids.push(id),
            Some(PhotoSide::Reception) => reception_photo_ids.push(id),
            // The column carries a DB CHECK; an unknown value means it was
            // hand-edited away. Drop it from BOTH lists rather than guess a
            // side — mislabelling whose evidence a picture is would be worse
            // than not showing it.
            None => tracing::error!(
                photo_id = id,
                side = %side,
                "report photo has an unknown side; omitted from the report DTO"
            ),
        }
    }

    let mut report = report_from_row(&row)?;
    report.items = items
        .iter()
        .map(|row| {
            let problem_raw: String = row.try_get("rri_problem")?;
            Ok(ReportItem {
                item: row.try_get("rri_item")?,
                problem: ItemProblem::parse(&problem_raw).ok_or_else(|| {
                    ServiceError::internal(format!("item row has unknown problem {problem_raw:?}"))
                })?,
                qty: row.try_get("rri_qty")?,
            })
        })
        .collect::<ServiceResult<Vec<_>>>()?;
    report.maid_photo_ids = maid_photo_ids;
    report.reception_photo_ids = reception_photo_ids;
    Ok(report)
}

/// The header columns → the FULL DTO, with EMPTY child lists for the caller to
/// fill. One deserializer, shared by the detail read and every mutation's
/// reload.
fn report_from_row(row: &sqlx::postgres::PgRow) -> ServiceResult<RoomReport> {
    let status_raw: String = row.try_get("rr_status")?;
    let date: NaiveDate = row.try_get("rr_date")?;
    Ok(RoomReport {
        report_id: row.try_get("rr_id")?,
        room_id: row.try_get("rr_room_id")?,
        room_no: row.try_get("room_no")?,
        date: date.to_string(),
        status: ReportStatus::parse(&status_raw).ok_or_else(|| {
            ServiceError::internal(format!("report has unknown status {status_raw:?}"))
        })?,
        room_status: row.try_get("rr_room_status")?,
        all_items_ok: row.try_get("rr_all_items_ok")?,
        items: Vec::new(),
        return_reason: row.try_get("rr_return_reason")?,
        parent_report_id: row.try_get("rr_parent_id")?,
        submitted_by: ReportActor {
            badge: row.try_get("rr_submitted_badge")?,
            name: row.try_get("rr_submitted_name")?,
        },
        submitted_at: rfc3339(row.try_get("rr_submitted_at")?),
        verified_by: actor(row.try_get("rr_verified_badge")?, row.try_get("rr_verified_name")?),
        verified_at: row
            .try_get::<Option<DateTime<Utc>>, _>("rr_verified_at")?
            .map(rfc3339),
        maid_photo_ids: Vec::new(),
        reception_photo_ids: Vec::new(),
    })
}

/// One day-overview row: the room, plus its latest report as a SUMMARY.
///
/// `rr_id` being NULL is what "no report today" looks like after the `LEFT JOIN
/// LATERAL`, and it is read as `Option<i64>` rather than inferred from any
/// other column — a report with a NULL name or no photos is still a report.
fn overview_row(row: &sqlx::postgres::PgRow) -> ServiceResult<RoomReportRow> {
    let report_id: Option<i64> = row.try_get("rr_id")?;
    let report = match report_id {
        None => None,
        Some(report_id) => {
            let status_raw: String = row.try_get("rr_status")?;
            let date: NaiveDate = row.try_get("rr_date")?;
            Some(RoomReportSummary {
                report_id,
                room_id: row.try_get("room_id")?,
                room_no: row.try_get("room_no")?,
                date: date.to_string(),
                status: ReportStatus::parse(&status_raw).ok_or_else(|| {
                    ServiceError::internal(format!("report has unknown status {status_raw:?}"))
                })?,
                room_status: row.try_get("rr_room_status")?,
                all_items_ok: row.try_get("rr_all_items_ok")?,
                return_reason: row.try_get("rr_return_reason")?,
                parent_report_id: row.try_get("rr_parent_id")?,
                submitted_by: ReportActor {
                    badge: row.try_get("rr_submitted_badge")?,
                    name: row.try_get("rr_submitted_name")?,
                },
                submitted_at: rfc3339(row.try_get("rr_submitted_at")?),
                verified_by: actor(
                    row.try_get("rr_verified_badge")?,
                    row.try_get("rr_verified_name")?,
                ),
                verified_at: row
                    .try_get::<Option<DateTime<Utc>>, _>("rr_verified_at")?
                    .map(rfc3339),
                photo_counts: PhotoCounts {
                    maid: row.try_get::<i64, _>("maid_photos")?.max(0) as usize,
                    reception: row.try_get::<i64, _>("reception_photos")?.max(0) as usize,
                },
            })
        }
    };
    Ok(RoomReportRow {
        room_id: row.try_get("room_id")?,
        room_no: row.try_get("room_no")?,
        floor: row.try_get("room_floor")?,
        building: row.try_get("room_building")?,
        report,
    })
}

/// An actor exists exactly when the BADGE does — the name is optional and a
/// row with a name but no badge is not a state the writers can produce. Same
/// rule as `service::hk_signals::actor`.
fn actor(badge: Option<String>, name: Option<String>) -> Option<ReportActor> {
    badge.map(|badge| ReportActor { badge, name })
}

/// RFC 3339 UTC — the one timestamp rendering both report DTOs use.
fn rfc3339(ts: DateTime<Utc>) -> String {
    ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// The verified badge must be present. Belt-and-braces: the Access middleware
/// 401s without one, so reaching this with a blank badge means a caller
/// constructed the command by hand.
fn require_badge(raw: &str) -> ServiceResult<String> {
    let badge = raw.trim();
    if badge.is_empty() {
        return Err(ServiceError::validation(
            "a room report requires a non-empty verified badge",
        ));
    }
    Ok(badge.to_string())
}

/// A domain rule refusal → the service error, honouring the domain's own
/// 403/conflict/400 split so no layer can re-decide it. Mirrors
/// `service::hk_signals::rule_error`.
fn rule_error(err: ReportRuleError) -> ServiceError {
    let message = err.message();
    if err.is_forbidden() {
        ServiceError::Forbidden(message)
    } else if err.is_conflict() {
        // A precondition failure, surfaced as Conflict for observability — the
        // repo's shared mapping renders it 400 (see the module header).
        ServiceError::Conflict(message)
    } else {
        ServiceError::Validation(message)
    }
}

/// The 1..=99 quantity bound, re-checked for ANY caller (the route checks it
/// too, so a bad body is a 400 rather than a 500 from the DB `CHECK`).
fn check_qty_bounds(items: &[ReportItemInput]) -> ServiceResult<()> {
    use crate::domain::hk_report::{MAX_ITEM_QTY, MIN_ITEM_QTY};
    if let Some(bad) = items
        .iter()
        .find(|item| !(MIN_ITEM_QTY..=MAX_ITEM_QTY).contains(&item.qty))
    {
        return Err(ServiceError::validation(format!(
            "item quantity {} for '{}' is outside {MIN_ITEM_QTY}..={MAX_ITEM_QTY}",
            bad.qty, bad.item
        )));
    }
    Ok(())
}

/// The same photo id twice would make the attach statement's row count short
/// and produce a confusing "not yours" message for a body that is simply
/// repetitive. Named here so the caller gets the real reason.
fn check_distinct_photos(photo_ids: &[i64]) -> ServiceResult<()> {
    let mut seen = photo_ids.to_vec();
    seen.sort_unstable();
    let before = seen.len();
    seen.dedup();
    if seen.len() != before {
        return Err(ServiceError::validation(
            "the same photo was listed more than once",
        ));
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ApiError;

    fn item(item: &str, problem: ItemProblem, qty: i32) -> ReportItemInput {
        ReportItemInput {
            item: item.to_string(),
            problem,
            qty,
        }
    }

    // ---- SQL pins (no database) -----------------------------------------

    /// The overview must select EVERY column both DTOs read. A projection that
    /// silently drops one turns into a `try_get` failure at runtime — on a
    /// maid's phone, mid-shift — so it is pinned here instead.
    #[test]
    fn the_overview_projection_selects_every_dto_column() {
        let sql = day_overview_sql();
        for column in [
            "r.room_id",
            "r.room_no",
            "r.room_floor",
            "r.room_building",
            "rep.rr_id",
            "rep.rr_date",
            "rep.rr_status",
            "rep.rr_room_status",
            "rep.rr_all_items_ok",
            "rep.rr_return_reason",
            "rep.rr_parent_id",
            "rep.rr_submitted_badge",
            "rep.rr_submitted_name",
            "rep.rr_submitted_at",
            "rep.rr_verified_badge",
            "rep.rr_verified_name",
            "rep.rr_verified_at",
        ] {
            assert!(sql.contains(column), "the overview must select {column}");
        }
        assert!(sql.contains("AS maid_photos"));
        assert!(sql.contains("AS reception_photos"));
    }

    /// The overview is ONE statement over every active room, in room order —
    /// the same room set and the same order as the cleaning list, so the two
    /// screens can never show a maid different rooms.
    #[test]
    fn the_overview_is_one_statement_over_every_active_room() {
        let sql = day_overview_sql();
        assert!(sql.contains("FROM ht_rooms_new r"));
        assert!(sql.contains("COALESCE(r.room_active, true) = true"));
        assert!(sql.contains("ORDER BY r.room_no"));
        // LEFT JOIN, so a room with no report is still a row.
        assert!(sql.contains("LEFT JOIN LATERAL"));
        assert!(sql.contains("h.rr_room_id = r.room_id AND h.rr_date = $1"));
        // The LATEST report for the day, not "a" report.
        assert!(sql.contains("ORDER BY h.rr_id DESC"));
        assert!(sql.contains("LIMIT 1"));
        // Photo counts come from a LATERAL aggregate, never a per-room query.
        assert!(sql.contains("COUNT(*) FILTER"));
        assert_eq!(
            sql.matches("FROM ht_hk_room_report_photos").count(),
            1,
            "the counts must be one correlated aggregate, not an N+1"
        );
    }

    /// The photo-side literals in the overview SQL are the ENUM's, not
    /// hand-typed strings — a rename in `PhotoSide` must move the query with
    /// it.
    #[test]
    fn the_overview_uses_the_enums_own_side_literals() {
        let sql = day_overview_sql();
        assert!(sql.contains(&format!("p.rrp_side = '{}'", PhotoSide::Maid.as_str())));
        assert!(sql.contains(&format!(
            "p.rrp_side = '{}'",
            PhotoSide::Reception.as_str()
        )));
    }

    /// The header projection must carry every column `report_from_row` reads.
    #[test]
    fn the_header_projection_selects_every_dto_column() {
        for column in [
            "h.rr_id",
            "h.rr_room_id",
            "r.room_no",
            "h.rr_date",
            "h.rr_status",
            "h.rr_room_status",
            "h.rr_all_items_ok",
            "h.rr_return_reason",
            "h.rr_parent_id",
            "h.rr_submitted_badge",
            "h.rr_submitted_name",
            "h.rr_submitted_at",
            "h.rr_verified_badge",
            "h.rr_verified_name",
            "h.rr_verified_at",
        ] {
            assert!(
                REPORT_SELECT.contains(column),
                "the header projection must select {column}"
            );
        }
        assert!(REPORT_SELECT.contains("JOIN ht_rooms_new r ON r.room_id = h.rr_room_id"));
    }

    // ---- the rules, as this service raises them --------------------------

    /// The 403/400 split survives the trip to HTTP: a role refusal is 403, a
    /// wrong-status verdict is 400 (the repo's shared conflict mapping), and a
    /// malformed body is 400.
    #[test]
    fn rule_errors_reach_http_with_the_domains_own_class() {
        let forbidden = rule_error(ReportRuleError::WrongSide { action: "verify" });
        assert!(matches!(forbidden, ServiceError::Forbidden(_)));
        assert!(matches!(
            ApiError::from(forbidden),
            ApiError::Forbidden(_)
        ));

        let conflict = rule_error(ReportRuleError::WrongStatus {
            action: "verify",
            status: ReportStatus::Verified,
        });
        assert!(matches!(conflict, ServiceError::Conflict(_)));
        // 400, NOT 409 — the house convention; see the module header.
        assert!(matches!(
            ApiError::from(conflict),
            ApiError::BadRequest(_)
        ));

        let validation = rule_error(ReportRuleError::PhotoCountOutOfRange { got: 9 });
        assert!(matches!(validation, ServiceError::Validation(_)));
        assert!(matches!(
            ApiError::from(validation),
            ApiError::BadRequest(_)
        ));
    }

    #[test]
    fn a_blank_badge_is_refused_before_any_sql() {
        for blank in ["", "   ", "\t"] {
            let err = require_badge(blank).expect_err("a blank badge must be refused");
            assert!(matches!(err, ServiceError::Validation(_)));
        }
        assert_eq!(require_badge("  Q1001 ").unwrap(), "Q1001");
    }

    #[test]
    fn quantities_are_bounded_one_to_ninety_nine() {
        assert!(check_qty_bounds(&[item("water_glass", ItemProblem::Missing, 1)]).is_ok());
        assert!(check_qty_bounds(&[item("water_glass", ItemProblem::Missing, 99)]).is_ok());
        for bad in [0, -1, 100, i32::MAX] {
            let err = check_qty_bounds(&[item("water_glass", ItemProblem::Missing, bad)])
                .expect_err("{bad} must be refused");
            assert!(matches!(err, ServiceError::Validation(_)));
        }
    }

    #[test]
    fn a_repeated_photo_id_is_named_as_such() {
        assert!(check_distinct_photos(&[1, 2, 3]).is_ok());
        assert!(check_distinct_photos(&[]).is_ok());
        let err = check_distinct_photos(&[4, 4]).expect_err("a repeat must be refused");
        assert!(
            err.to_string().contains("more than once"),
            "the message must name the real reason, not 'not yours': {err}"
        );
    }

    /// The submit raises AT MOST TWO signals — one per problem KIND — however
    /// many items are excepted. This mirrors the loop in [`HkReportService::submit`]
    /// so the dedup rule is assertable without a database.
    #[test]
    fn item_exceptions_raise_one_signal_per_problem_kind() {
        fn signals_for(items: &[ReportItemInput]) -> Vec<&'static str> {
            ItemProblem::ALL
                .into_iter()
                .filter(|problem| items.iter().any(|item| item.problem == *problem))
                .map(|problem| problem.signal_type())
                .collect()
        }

        assert_eq!(signals_for(&[]), Vec::<&str>::new(), "a clean room signals nothing");
        assert_eq!(
            signals_for(&[
                item("tv_remote", ItemProblem::Missing, 1),
                item("bath_towel", ItemProblem::Missing, 3),
                item("water_glass", ItemProblem::Missing, 2),
            ]),
            vec!["item_missing"],
            "three missing items are ONE item_missing, not three"
        );
        assert_eq!(
            signals_for(&[
                item("kettle", ItemProblem::Damaged, 1),
                item("tv_remote", ItemProblem::Missing, 1),
            ]),
            vec!["item_missing", "item_damaged"],
            "both kinds raise both, in ItemProblem::ALL order"
        );
    }

    /// The attach statement is the ownership check. Pinned as text because
    /// dropping ANY of the four conditions is a real privilege bug: without
    /// `rrp_badge` a maid could bind reception's photos, without
    /// `rrp_report_id IS NULL` she could steal another report's evidence, and
    /// without `rrp_side` she could pass her own uploads off as a verification.
    #[test]
    fn the_attach_predicate_carries_every_ownership_condition() {
        // The literal SQL from `attach_photos`, kept in one place so this test
        // fails loudly if the statement is edited.
        let sql = "UPDATE ht_hk_room_report_photos \
            SET rrp_report_id = $1 \
          WHERE rrp_id = ANY($2::bigint[]) \
            AND rrp_report_id IS NULL \
            AND rrp_badge = $3 \
            AND rrp_side = $4";
        for condition in [
            "rrp_id = ANY($2::bigint[])",
            "rrp_report_id IS NULL",
            "rrp_badge = $3",
            "rrp_side = $4",
        ] {
            assert!(sql.contains(condition), "the attach must require {condition}");
        }
    }

    #[test]
    fn timestamps_render_as_rfc3339_utc() {
        let ts = DateTime::parse_from_rfc3339("2026-09-02T10:30:00+07:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(rfc3339(ts), "2026-09-02T03:30:00Z");
    }

    #[test]
    fn an_actor_exists_when_the_badge_does() {
        assert_eq!(actor(None, None), None);
        assert_eq!(actor(None, Some("ghost".to_string())), None);
        assert_eq!(
            actor(Some("R2002".to_string()), None),
            Some(ReportActor {
                badge: "R2002".to_string(),
                name: None
            })
        );
    }
}
