//! Shift service — owns `ht_shifts` open/close/lookup.
//!
//! Track F2 / T1 HIGH-5 from `docs/coexistence/audit-2026-05-13.md`.
//!
//! iHOTEL refuses to record a payment unless a `HT_Round_Bill` shift is
//! open. Our app previously accepted payments at any time, which made
//! cash-drawer reconciliation impossible. F2 lands the canonical PG
//! table plus this service so [`super::payment::PaymentService::record_payment`]
//! can gate on `current_open_shift(site).is_some()`.
//!
//! ## Scope (F2)
//!
//! - `open_shift` — insert a new `ht_shifts` row, allocating the next
//!   `shift_no` for the site. Fails with `ServiceError::Conflict` when a
//!   shift is already open (enforced both by an explicit pre-check and
//!   by the partial UNIQUE index `ht_shifts_one_open_per_site`).
//! - `close_shift` — stamp `shift_closed_at` + `shift_closed_by` on the
//!   currently-open row. Returns a [`ShiftSummary`] (payment counts +
//!   totals) so the route layer can render a "cash-drawer reconciliation"
//!   preview to the cashier.
//! - `current_open_shift(site)` — read the single open row, if any.
//! - `recent_shifts(site, limit)` — read the last N rows ordered by
//!   `shift_opened_at DESC`.
//!
//! ## Deferred to follow-ups
//!
//! - **Mirror writeback of `HT_Round_Bill`** (Track G / "round-bill
//!   shift discipline" feature). Today the legacy table keeps being
//!   written by iHOTEL; F2 is just the PG canonical + gate.
//! - **CT-side sync of `HT_Round_Bill` -> `ht_shifts`** — `HT_Round_Bill`
//!   likely needs CT enabled on the MSSQL side (separate legacy migration).
//!   Documented as TODO in `sync/mappers/` follow-on.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};

use super::error::{ServiceError, ServiceResult};
use crate::outbox::intent::{round_aggregate, CloseRoundPayload, OpenRoundPayload, WritebackIntent};
use crate::outbox::{generate_idempotency_key, OutboxRepository};

/// Command for [`ShiftService::open_shift`].
#[derive(Debug, Clone)]
pub struct OpenShiftCommand {
    /// Cashier opening the round. Mirrors `HT_Round_Bill.Round_By` per the
    /// legacy app. Routes populate from auth context.
    pub opened_by: String,
    /// Opening cash float in baht. NUMERIC(12,2) on the PG side.
    pub opening_float: f64,
    /// Free-form notes (optional). NULL when callers don't supply one.
    pub notes: Option<String>,
}

/// Command for [`ShiftService::close_shift`].
#[derive(Debug, Clone)]
pub struct CloseShiftCommand {
    /// Cashier closing the round. Mirrors `HT_Round_Bill.Round_Close_By`.
    pub closed_by: String,
    /// Optional closing-time notes (variance explanation, etc.).
    pub notes: Option<String>,
}

/// Row representation returned to callers.
///
/// `shift_id` is the PG SERIAL pk; `shift_no` is the per-site running
/// counter visible to the cashier.
#[derive(Debug, Clone)]
pub struct Shift {
    pub shift_id: i64,
    pub site_id: String,
    pub shift_no: i32,
    pub opening_float: f64,
    pub opened_by: String,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<String>,
    pub legacy_round_id: Option<i32>,
    pub notes: Option<String>,
}

/// Outcome of [`ShiftService::open_shift`].
#[derive(Debug, Clone)]
pub struct OpenShiftOutcome {
    pub shift_id: i64,
    pub shift_no: i32,
}

/// Summary returned by [`ShiftService::close_shift`]. Aggregates the
/// payments collected during the round so the route layer can render a
/// cash-drawer reconciliation preview.
#[derive(Debug, Clone)]
pub struct ShiftSummary {
    pub shift_id: i64,
    pub shift_no: i32,
    pub opening_float: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
    /// Number of non-voided `ht_payments` rows recorded during the round.
    pub payment_count: i64,
    /// Sum of `ht_payments.pay_amount` (non-voided) recorded between
    /// `opened_at` and `closed_at`.
    pub total_collected: f64,
}

/// Service handle for the shift aggregate.
///
/// Holds the canonical PG pool + the site id this binary is configured
/// for (read from `SITE_ID` at startup). One service instance therefore
/// represents one site; the gate in `PaymentService::record_payment`
/// asks `current_open_shift()` without needing to pass the site through
/// every command.
#[derive(Clone)]
pub struct ShiftService {
    pg: PgPool,
    site_id: String,
    /// When `true`, [`open_shift`](Self::open_shift) /
    /// [`close_shift`](Self::close_shift) additionally enqueue an
    /// [`WritebackIntent::OpenRound`] / [`CloseRound`] so the round is
    /// mirrored into iHOTEL's `HT_Round_Bill` (Track J6, round coexistence
    /// step 2). Default `false`: the app only *reads* iHOTEL's rounds (via
    /// the `sync_round_bills` mirror) and **must not open/close them** — the
    /// open/close routes are not mounted in that mode, and this flag is the
    /// belt-and-suspenders guard if one is ever called directly.
    round_writeback: bool,
}

impl ShiftService {
    /// Construct a `ShiftService` bound to a specific site.
    pub fn new(pg: PgPool, site_id: impl Into<String>) -> Self {
        Self {
            pg,
            site_id: site_id.into(),
            round_writeback: false,
        }
    }

    /// Enable round-bill writeback (Track J6). Call this only for a service
    /// whose `pg` pool + `site_id` target the same logical DB the matching
    /// `writeback` worker drains — i.e. the hotel bundle on `new_pool`
    /// (`hfhotel`) or the ville bundle on `ville_pool` (`hfville`). A mismatch
    /// would enqueue an `OpenRound` into the wrong site's `writeback_jobs`.
    pub fn with_round_writeback(mut self, enabled: bool) -> Self {
        self.round_writeback = enabled;
        self
    }

    /// Whether this service emits round-bill writebacks.
    pub fn round_writeback_enabled(&self) -> bool {
        self.round_writeback
    }

    /// The site this service was constructed for.
    pub fn site_id(&self) -> &str {
        &self.site_id
    }

    /// Open a new shift for this service's site.
    ///
    /// Fails with [`ServiceError::Conflict`] if a shift is already open
    /// (the partial UNIQUE index `ht_shifts_one_open_per_site` enforces
    /// the same invariant at the storage layer, so a race that sneaks
    /// past the explicit pre-check still surfaces as a clean conflict
    /// rather than a silent duplicate).
    pub async fn open_shift(
        &self,
        cmd: OpenShiftCommand,
    ) -> ServiceResult<OpenShiftOutcome> {
        validate_opened_by(&cmd.opened_by)?;
        if cmd.opening_float < 0.0 {
            return Err(ServiceError::validation(
                "opening_float must be >= 0",
            ));
        }
        // Belt-and-suspenders: opening a round our side without mirroring it
        // into `HT_Round_Bill` would let iHOTEL open a *second* round for the
        // same id space (it can't see our canonical-only row), so this path is
        // refused unless writeback is wired. In production the open/close
        // routes are only mounted when the flag is on, so this never trips.
        if !self.round_writeback {
            return Err(ServiceError::conflict(
                "round open/close is not enabled at this site \
                 (ROUND_WRITEBACK_ENABLED is off — iHOTEL owns the round)",
            ));
        }

        let mut tx = self.pg.begin().await?;

        // Defensive pre-check so callers get a clear "already open"
        // message instead of a raw UNIQUE-violation error. The partial
        // index still enforces correctness under concurrent inserts. Because
        // `sync_round_bills` mirrors iHOTEL's open round into `ht_shifts`,
        // this also blocks our app from opening when iHOTEL already has one
        // open (co-equal mutual exclusion via the shared canonical row).
        if let Some(open) = fetch_open_shift(&mut tx, &self.site_id).await? {
            return Err(ServiceError::conflict(format!(
                "site {} already has an open shift (shift_no={}, opened_by={})",
                self.site_id, open.shift_no, open.opened_by,
            )));
        }

        // `shift_no` doubles as the legacy `HT_Round_Bill.id` (the read-only
        // mirror keys `ht_shifts` on `shift_no = legacy id`, and the two id
        // spaces coincide today). Allocating `MAX(shift_no)+1` is exactly
        // iHOTEL's `get_id` (`MAX(id)+1`) computed from our sub-second-fresh
        // mirror; we stamp it into `shift_legacy_round_id` and pass it
        // explicitly to the writeback so canonical + legacy stay on one id.
        let next_no = next_shift_no(&mut tx, &self.site_id).await?;

        let row: (i64, i32, DateTime<Utc>) = sqlx::query_as(
            "INSERT INTO ht_shifts ( \
                 shift_site_id, shift_no, shift_legacy_round_id, shift_opening_float, \
                 shift_opened_by, shift_opened_at, shift_notes \
             ) VALUES ($1, $2, $2, $3::numeric, $4, NOW(), $5) \
             RETURNING shift_id, shift_no, shift_opened_at",
        )
        .bind(&self.site_id)
        .bind(next_no)
        .bind(cmd.opening_float)
        .bind(cmd.opened_by.trim())
        .bind(cmd.notes.as_deref())
        .fetch_one(&mut *tx)
        .await
        .map_err(map_unique_violation_to_conflict)?;

        // Mirror into iHOTEL's HT_Round_Bill (Track J6). Same tx as the
        // canonical INSERT → the job is queued iff the shift row commits.
        let intent = WritebackIntent::OpenRound {
            round_aggregate_id: round_aggregate(&self.site_id, next_no),
            payload: OpenRoundPayload {
                site_id: self.site_id.clone(),
                legacy_round_id: next_no,
                round_price: cmd.opening_float,
                round_by: cmd.opened_by.trim().to_string(),
                round_start: row.2,
            },
        };
        let key = generate_idempotency_key(&intent, intent.aggregate_id());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        tx.commit().await?;

        Ok(OpenShiftOutcome {
            shift_id: row.0,
            shift_no: row.1,
        })
    }

    /// Close the currently-open shift and return a reconciliation summary.
    ///
    /// Fails with [`ServiceError::NotFound`] if no shift is open.
    pub async fn close_shift(
        &self,
        cmd: CloseShiftCommand,
    ) -> ServiceResult<ShiftSummary> {
        validate_closed_by(&cmd.closed_by)?;
        if !self.round_writeback {
            return Err(ServiceError::conflict(
                "round open/close is not enabled at this site \
                 (ROUND_WRITEBACK_ENABLED is off — iHOTEL owns the round)",
            ));
        }

        let mut tx = self.pg.begin().await?;

        let open = fetch_open_shift(&mut tx, &self.site_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "no open shift for site {}",
                    self.site_id
                ))
            })?;

        // The legacy id is set on open (by us) or by `sync_round_bills` (for
        // iHOTEL-opened rounds) — so our app can co-equally close a round
        // iHOTEL opened. A NULL here would mean an unsynced canonical row;
        // refuse rather than fire a blind `WHERE round_end IS NULL` UPDATE.
        let legacy_round_id = open.legacy_round_id.ok_or_else(|| {
            ServiceError::conflict(format!(
                "open shift {} for site {} has no legacy round id to write back",
                open.shift_no, self.site_id
            ))
        })?;

        let closed_row: (DateTime<Utc>,) = sqlx::query_as(
            "UPDATE ht_shifts \
                SET shift_closed_at = NOW(), \
                    shift_closed_by = $2, \
                    shift_notes = COALESCE($3, shift_notes) \
              WHERE shift_id = $1 \
              RETURNING shift_closed_at",
        )
        .bind(open.shift_id)
        .bind(cmd.closed_by.trim())
        .bind(cmd.notes.as_deref())
        .fetch_one(&mut *tx)
        .await?;

        // Mirror the close into HT_Round_Bill (Track J6), same tx.
        let intent = WritebackIntent::CloseRound {
            round_aggregate_id: round_aggregate(&self.site_id, legacy_round_id),
            payload: CloseRoundPayload {
                site_id: self.site_id.clone(),
                legacy_round_id,
                round_by: cmd.closed_by.trim().to_string(),
                round_end: closed_row.0,
            },
        };
        let key = generate_idempotency_key(&intent, intent.aggregate_id());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let summary_row: (i64, Option<f64>) = sqlx::query_as(
            "SELECT \
                 COUNT(*)::BIGINT AS count, \
                 COALESCE(SUM(pay_amount), 0)::float8 AS total \
               FROM ht_payments \
              WHERE pay_voided = false \
                AND created_at >= $1 \
                AND created_at <= $2",
        )
        .bind(open.opened_at)
        .bind(closed_row.0)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ShiftSummary {
            shift_id: open.shift_id,
            shift_no: open.shift_no,
            opening_float: open.opening_float,
            opened_at: open.opened_at,
            closed_at: closed_row.0,
            payment_count: summary_row.0,
            total_collected: summary_row.1.unwrap_or(0.0),
        })
    }

    /// Return the currently-open shift for this site, if any.
    ///
    /// This is the read the payment gate consults. It is intentionally a
    /// pool-borrowed read (no transaction) — the partial UNIQUE index
    /// guarantees at-most-one open row, so dirty-read interleaving with a
    /// concurrent `open_shift` either sees the row (and accepts the
    /// payment) or doesn't (and the payment is rejected) — both
    /// outcomes are safe.
    pub async fn current_open_shift(&self) -> ServiceResult<Option<Shift>> {
        let row: Option<ShiftRow> = sqlx::query_as::<_, ShiftRow>(
            "SELECT shift_id, shift_site_id, shift_no, \
                    shift_opening_float::float8 AS shift_opening_float, \
                    shift_opened_by, shift_opened_at, shift_closed_at, \
                    shift_closed_by, shift_legacy_round_id, shift_notes \
               FROM ht_shifts \
              WHERE shift_site_id = $1 AND shift_closed_at IS NULL \
              LIMIT 1",
        )
        .bind(&self.site_id)
        .fetch_optional(&self.pg)
        .await?;

        Ok(row.map(Shift::from))
    }

    /// Return up to `limit` most-recent shifts for this site, ordered
    /// newest-first by `shift_opened_at`.
    pub async fn recent_shifts(&self, limit: i64) -> ServiceResult<Vec<Shift>> {
        // Clamp to sane bounds so a misbehaving client cannot DoS the
        // dashboard.
        let bounded = limit.clamp(1, 200);

        let rows: Vec<ShiftRow> = sqlx::query_as::<_, ShiftRow>(
            "SELECT shift_id, shift_site_id, shift_no, \
                    shift_opening_float::float8 AS shift_opening_float, \
                    shift_opened_by, shift_opened_at, shift_closed_at, \
                    shift_closed_by, shift_legacy_round_id, shift_notes \
               FROM ht_shifts \
              WHERE shift_site_id = $1 \
              ORDER BY shift_opened_at DESC \
              LIMIT $2",
        )
        .bind(&self.site_id)
        .bind(bounded)
        .fetch_all(&self.pg)
        .await?;

        Ok(rows.into_iter().map(Shift::from).collect())
    }
}

// -----------------------------------------------------------------------------
// internal helpers
// -----------------------------------------------------------------------------

/// `sqlx::FromRow`-friendly intermediate so the public [`Shift`] type can
/// stay decoupled from `sqlx` column names.
#[derive(sqlx::FromRow)]
struct ShiftRow {
    shift_id: i64,
    shift_site_id: String,
    shift_no: i32,
    shift_opening_float: f64,
    shift_opened_by: String,
    shift_opened_at: DateTime<Utc>,
    shift_closed_at: Option<DateTime<Utc>>,
    shift_closed_by: Option<String>,
    shift_legacy_round_id: Option<i32>,
    shift_notes: Option<String>,
}

impl From<ShiftRow> for Shift {
    fn from(r: ShiftRow) -> Self {
        Self {
            shift_id: r.shift_id,
            site_id: r.shift_site_id,
            shift_no: r.shift_no,
            opening_float: r.shift_opening_float,
            opened_by: r.shift_opened_by,
            opened_at: r.shift_opened_at,
            closed_at: r.shift_closed_at,
            closed_by: r.shift_closed_by,
            legacy_round_id: r.shift_legacy_round_id,
            notes: r.shift_notes,
        }
    }
}

async fn fetch_open_shift(
    tx: &mut Transaction<'_, Postgres>,
    site_id: &str,
) -> ServiceResult<Option<Shift>> {
    let row: Option<ShiftRow> = sqlx::query_as::<_, ShiftRow>(
        "SELECT shift_id, shift_site_id, shift_no, \
                shift_opening_float::float8 AS shift_opening_float, \
                shift_opened_by, shift_opened_at, shift_closed_at, \
                shift_closed_by, shift_legacy_round_id, shift_notes \
           FROM ht_shifts \
          WHERE shift_site_id = $1 AND shift_closed_at IS NULL \
          FOR UPDATE",
    )
    .bind(site_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(Shift::from))
}

/// Pick the next `shift_no` for the site. Atomicity is provided by the
/// surrounding transaction + the `UNIQUE (shift_site_id, shift_no)`
/// constraint — a concurrent opener will collide and be retried by the
/// caller. F2 doesn't ship a retry loop because the second invariant
/// (one open shift per site) already serializes openers.
async fn next_shift_no(
    tx: &mut Transaction<'_, Postgres>,
    site_id: &str,
) -> ServiceResult<i32> {
    let max: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(shift_no) FROM ht_shifts WHERE shift_site_id = $1",
    )
    .bind(site_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(max.unwrap_or(0) + 1)
}

fn validate_opened_by(opened_by: &str) -> ServiceResult<()> {
    if opened_by.trim().is_empty() {
        return Err(ServiceError::validation(
            "opened_by must not be empty",
        ));
    }
    Ok(())
}

fn validate_closed_by(closed_by: &str) -> ServiceResult<()> {
    if closed_by.trim().is_empty() {
        return Err(ServiceError::validation(
            "closed_by must not be empty",
        ));
    }
    Ok(())
}

/// Map a `sqlx::Error` produced by a UNIQUE-violation on
/// `ht_shifts_one_open_per_site` to a clean
/// [`ServiceError::Conflict`]. Any other sqlx error propagates unchanged.
fn map_unique_violation_to_conflict(err: sqlx::Error) -> ServiceError {
    if let sqlx::Error::Database(db_err) = &err {
        // PG error code 23505 = unique_violation. The pre-check above
        // already covers the common path; this branch handles a tight
        // race between two openers slipping past the SELECT.
        if db_err.code().as_deref() == Some("23505") {
            return ServiceError::conflict(
                "another shift is already open for this site",
            );
        }
    }
    ServiceError::from(err)
}

// -----------------------------------------------------------------------------
// tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    //! Integration-style tests that exercise the real PG schema. Each
    //! test uses a unique `site_id` marker so the partial UNIQUE index on
    //! open shifts does not cause cross-test interference.
    use super::*;

    /// Connect to the same PG test database the integration tests use.
    /// Skipped at runtime when the database is unreachable so `cargo
    /// test` in a CI environment without a DB still succeeds for the
    /// rest of the suite.
    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    async fn cleanup_site(pool: &PgPool, site_id: &str) {
        let _ = sqlx::query("DELETE FROM ht_shifts WHERE shift_site_id = $1")
            .bind(site_id)
            .execute(pool)
            .await;
        // Round-bill writeback (Track J6) enqueues open_round/close_round jobs
        // whose idempotency_key is a pure function of (intent, site, legacy
        // id) — clear them so a re-run of the test doesn't collide on the
        // unique key.
        let _ = sqlx::query(
            "DELETE FROM writeback_jobs \
             WHERE intent IN ('open_round','close_round') \
               AND payload->'payload'->>'site_id' = $1",
        )
        .bind(site_id)
        .execute(pool)
        .await;
    }

    #[tokio::test]
    async fn open_close_roundtrip() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping open_close_roundtrip — PG not reachable");
            return;
        };
        let site = "TEST_open_close_RT";
        cleanup_site(&pool, site).await;

        let svc = ShiftService::new(pool.clone(), site).with_round_writeback(true);

        // Start: no open shift.
        assert!(
            svc.current_open_shift().await.unwrap().is_none(),
            "fresh site must not have an open shift"
        );

        // Open one.
        let opened = svc
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: 1000.0,
                notes: Some("morning shift".into()),
            })
            .await
            .expect("open_shift");
        assert!(opened.shift_id > 0);
        assert_eq!(opened.shift_no, 1, "first shift_no should be 1");

        // Now visible.
        let now_open = svc.current_open_shift().await.unwrap();
        assert!(now_open.is_some(), "open shift must be visible immediately");
        let open = now_open.unwrap();
        assert_eq!(open.shift_no, 1);
        assert_eq!(open.opened_by, "alice");
        assert!((open.opening_float - 1000.0).abs() < 0.01);
        assert!(open.closed_at.is_none());

        // Close it.
        let summary = svc
            .close_shift(CloseShiftCommand {
                closed_by: "alice".into(),
                notes: None,
            })
            .await
            .expect("close_shift");
        assert_eq!(summary.shift_id, opened.shift_id);
        assert!((summary.opening_float - 1000.0).abs() < 0.01);
        assert!(summary.closed_at >= summary.opened_at);

        // Closed -> no open shift again.
        assert!(svc.current_open_shift().await.unwrap().is_none());

        // Listing returns the just-closed row.
        let recent = svc.recent_shifts(5).await.unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].shift_no, 1);
        assert!(recent[0].closed_at.is_some());

        cleanup_site(&pool, site).await;
    }

    #[tokio::test]
    async fn cannot_open_when_one_is_already_open() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping cannot_open_when_one_is_already_open — PG not reachable");
            return;
        };
        let site = "TEST_one_open_only";
        cleanup_site(&pool, site).await;

        let svc = ShiftService::new(pool.clone(), site).with_round_writeback(true);

        svc.open_shift(OpenShiftCommand {
            opened_by: "alice".into(),
            opening_float: 500.0,
            notes: None,
        })
        .await
        .expect("first open");

        let err = svc
            .open_shift(OpenShiftCommand {
                opened_by: "bob".into(),
                opening_float: 500.0,
                notes: None,
            })
            .await
            .expect_err("second open must fail");

        assert!(
            matches!(err, ServiceError::Conflict(_)),
            "expected Conflict, got {err:?}"
        );

        cleanup_site(&pool, site).await;
    }

    #[tokio::test]
    async fn close_without_open_returns_not_found() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping close_without_open_returns_not_found — PG not reachable");
            return;
        };
        let site = "TEST_close_no_open";
        cleanup_site(&pool, site).await;

        let svc = ShiftService::new(pool.clone(), site).with_round_writeback(true);

        let err = svc
            .close_shift(CloseShiftCommand {
                closed_by: "alice".into(),
                notes: None,
            })
            .await
            .expect_err("close without open must fail");

        assert!(
            matches!(err, ServiceError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );

        cleanup_site(&pool, site).await;
    }

    #[tokio::test]
    async fn open_validates_inputs() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping open_validates_inputs — PG not reachable");
            return;
        };
        let site = "TEST_open_validates";
        cleanup_site(&pool, site).await;

        let svc = ShiftService::new(pool.clone(), site);

        let blank = svc
            .open_shift(OpenShiftCommand {
                opened_by: "   ".into(),
                opening_float: 0.0,
                notes: None,
            })
            .await
            .expect_err("blank opener must reject");
        assert!(matches!(blank, ServiceError::Validation(_)));

        let negative = svc
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: -10.0,
                notes: None,
            })
            .await
            .expect_err("negative float must reject");
        assert!(matches!(negative, ServiceError::Validation(_)));

        cleanup_site(&pool, site).await;
    }
}
