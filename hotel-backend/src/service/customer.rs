//! Customer service — orchestrates `ht_customers` writes plus outbox + events.
//!
//! Per `docs/architecture.md` §1, §6. Each public method opens a single PG
//! transaction, performs the canonical write through
//! [`CustomerRepository`](crate::repository::customer::CustomerRepository),
//! optionally enqueues a writeback via
//! [`OutboxRepository`](crate::outbox::OutboxRepository), publishes a
//! [`DomainEvent`](crate::outbox::DomainEvent) via
//! [`EventBus`](crate::outbox::EventBus), and commits — making all three
//! effects atomic.
//!
//! Routes do **not** call this layer yet (Wave 4 thins them). The service
//! is wired into `AppState` so it is constructible and testable today.

use std::sync::Arc;

use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::domain::customer::CustomerType;
use crate::outbox::event::{CustomerSnapshot, DomainEvent, EventSource};
use crate::outbox::intent::{CustomerResave, WritebackIntent};
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::customer::{CustomerRepository, CustomerWrite};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// Inbound command for [`CustomerService::create`].
///
/// Owned by the service module so the route layer (Wave 4) can construct one
/// from its request DTO without exposing the repository's borrowed-string
/// `CustomerWrite` shape.
#[derive(Debug, Clone)]
pub struct CreateCustomerCommand {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
    /// Where this command originated. Routes populate from auth context;
    /// background jobs use `EventSource::System { reason: ... }`.
    pub source: EventSource,
}

/// Inbound command for [`CustomerService::update`].
#[derive(Debug, Clone)]
pub struct UpdateCustomerCommand {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
    /// Auth-residual (task #56) editing operator → `cust_updated_by`. The
    /// route resolves this from the authenticated `Extension<User>` (preferred)
    /// or the body `updatedBy` fallback. `None` (auth dark, no body) ⇒ the
    /// column is left untouched — behavior is identical to before this field.
    pub updated_by: Option<String>,
    pub source: EventSource,
}

/// Outcome of a successful `create` / `update`.
///
/// The numeric `customer_id` is the SERIAL value the repository minted (or
/// echoes back on update); the `aggregate_id` is the deterministic UUID used
/// in the published event so subscribers (SSE broadcaster, audit log) match
/// the same `aggregate_id` shape used by the rest of the bus.
#[derive(Debug, Clone)]
pub struct CustomerOutcome {
    pub customer_id: i32,
    pub aggregate_id: uuid::Uuid,
}

/// Service handle for the customer aggregate.
///
/// Holds `Arc` references so it is `Clone`-cheap and shareable across the
/// request executor. Constructed once at startup in `AppState::new`.
/// Service handle for the customer aggregate.
///
/// `outbox` and `events` are held for Wave 4: today's `EventBus::publish` /
/// `OutboxRepository::enqueue` are stateless static methods, so the Arc
/// fields are not read on the hot path. They remain in the struct so the
/// constructor signature matches the architecture target (services own
/// their collaborators) and the Wave 4 refactor — which makes those
/// methods `&self` for mockability — needs no struct change.
#[derive(Clone)]
pub struct CustomerService {
    pub(crate) repo: Arc<dyn CustomerRepository>,
    #[allow(dead_code)]
    pub(crate) outbox: Arc<OutboxRepository>,
    #[allow(dead_code)]
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

/// Hydrate a full [`CustomerResave`] (the legacy-side 31-field UPDATE)
/// from the canonical `ht_customers` row, inside the caller's transaction.
///
/// Called AFTER the canonical UPDATE so the operator-edited columns
/// (`cust_firstname`, `cust_phone`, `cust_email`, `cust_idcard`,
/// `cust_address`, `cust_type`) already carry the new values, while the
/// columns the new-app edit form does NOT expose (`cust_add_moo`,
/// `cust_work_*`, `cust_title`, `cust_sex`, …) keep their CT-mirrored
/// legacy values — the re-save then never blanks iHOTEL-entered data
/// (the P1-9 class of bug from the 2026-06-11 audit).
///
/// Returns `Ok(None)` when the customer has no `legacy_cust_no` — i.e. it
/// has never been mirrored to MSSQL, so there is no legacy row to update.
/// (The eventual booking / check-in writeback that first INSERTs the
/// customer reads the then-current canonical values, so a skipped edit
/// here is not lost. Narrow known race: an edit committed while the very
/// first `CreateBooking` writeback for this customer is in flight uses
/// that booking's pre-edit snapshot; the next edit or booking self-heals.)
///
/// Column → legacy mapping notes (`sync/mappers/customer.rs` table,
/// inverted):
/// * `Cust_name` ← `cust_firstname` **verbatim, NOT joined with
///   `cust_lastname`** — the reconcile sweep hashes legacy `Cust_name`
///   against canonical `cust_firstname`
///   (`scheduler::sync::CUSTOMERS_RECONCILE_PROJECTION`) and the CT
///   mapper writes `Cust_name → cust_firstname`; joining would create
///   permanent hash divergence. `cust_lastname` is canonical-only.
/// * `Cust_Type` ← `cust_price_tier` (rate-tier label) and
///   `Cust_Type_main` ← `cust_type` (customer-category) — the legacy
///   pair is famously swapped-looking; see the mapper doc.
/// * `Cust_Add_no` ← `cust_address` — the new-app edit form writes
///   `cust_address`; the CT mapper keeps it as a copy of `Cust_Add_no`.
///
/// Uses dynamic `sqlx::query` (not the compile-time macro) to keep the
/// crate buildable without regenerating the `.sqlx/` cache — same
/// convention as `routes::new_rooms::load_legacy_room_no`.
async fn load_customer_resave(
    tx: &mut Transaction<'_, Postgres>,
    cust_id: i32,
) -> Result<Option<CustomerResave>, sqlx::Error> {
    let Some(row) = sqlx::query(
        "SELECT legacy_cust_no, cust_firstname, cust_name2, cust_price_tier, \
                cust_type, cust_email, cust_address, cust_add_moo, cust_add_soi, \
                cust_add_road, cust_add_tambon, cust_add_ampore, \
                cust_add_province, cust_add_code, cust_phone, cust_add_fax, \
                cust_work_name, cust_work_no, cust_work_moo, cust_work_soi, \
                cust_work_road, cust_work_tambon, cust_work_ampore, \
                cust_work_province, cust_work_code, cust_work_tel, \
                cust_work_fax, cust_work_tax, cust_title, cust_sex, \
                cust_idcard, cust_contry \
           FROM ht_customers WHERE cust_id = $1",
    )
    .bind(cust_id)
    .fetch_optional(&mut **tx)
    .await?
    else {
        return Ok(None);
    };

    let legacy_cust_no: String = row
        .try_get::<Option<String>, _>("legacy_cust_no")?
        .unwrap_or_default();
    if legacy_cust_no.is_empty() {
        return Ok(None);
    }

    // Legacy `''`-over-NULL convention (spike §3k): every optional column
    // collapses to the empty string for the re-save.
    let text = |col: &str| -> String {
        row.try_get::<Option<String>, _>(col)
            .ok()
            .flatten()
            .unwrap_or_default()
    };

    Ok(Some(CustomerResave {
        legacy_cust_no,
        cust_name: row.try_get::<String, _>("cust_firstname").unwrap_or_default(),
        cust_name2: text("cust_name2"),
        cust_type: text("cust_price_tier"),
        cust_type_main: text("cust_type"),
        cust_email: text("cust_email"),
        cust_add_no: text("cust_address"),
        cust_add_moo: text("cust_add_moo"),
        cust_add_soi: text("cust_add_soi"),
        cust_add_road: text("cust_add_road"),
        cust_add_tambon: text("cust_add_tambon"),
        cust_add_ampore: text("cust_add_ampore"),
        cust_add_province: text("cust_add_province"),
        cust_add_code: text("cust_add_code"),
        cust_add_tel: text("cust_phone"),
        cust_add_fax: text("cust_add_fax"),
        cust_work_name: text("cust_work_name"),
        cust_work_no: text("cust_work_no"),
        cust_work_moo: text("cust_work_moo"),
        cust_work_soi: text("cust_work_soi"),
        cust_work_road: text("cust_work_road"),
        cust_work_tambon: text("cust_work_tambon"),
        cust_work_ampore: text("cust_work_ampore"),
        cust_work_province: text("cust_work_province"),
        cust_work_code: text("cust_work_code"),
        cust_work_tel: text("cust_work_tel"),
        cust_work_fax: text("cust_work_fax"),
        cust_work_tax: text("cust_work_tax"),
        cust_perfix: text("cust_title"),
        cust_sex: text("cust_sex"),
        cust_idcard: text("cust_idcard"),
        cust_contry: text("cust_contry"),
    }))
}

impl CustomerService {
    /// Wire a customer service from its three collaborators + the PG pool.
    pub fn new(
        repo: Arc<dyn CustomerRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg }
    }

    /// Create a new customer — single transaction, atomic with event publish.
    ///
    /// **No outbox enqueue today.** The legacy MSSQL writeback for
    /// "create customer" is folded into the booking / check-in writeback
    /// recipes (per spike findings §3a, §3b — `INSERT HT_Customers` happens
    /// alongside the booking / check-in row insert, not as a standalone
    /// flow). When a standalone customer-create writeback recipe lands, this
    /// method gains an `outbox.enqueue(...)` call inside the same TX.
    pub async fn create(&self, cmd: CreateCustomerCommand) -> ServiceResult<CustomerOutcome> {
        validate_first_name(&cmd.first_name)?;

        let mut tx = self.pg.begin().await?;

        let customer_id = self
            .repo
            .insert(
                &mut tx,
                CustomerWrite {
                    first_name: &cmd.first_name,
                    last_name: cmd.last_name.as_deref(),
                    phone: cmd.phone.as_deref(),
                    email: cmd.email.as_deref(),
                    id_card: cmd.id_card.as_deref(),
                    address: cmd.address.as_deref(),
                    customer_type: cmd.customer_type.as_deref(),
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        let aggregate_id = aggregate_uuid(AggregateKind::Customer, customer_id);
        let snapshot = build_customer_snapshot(aggregate_id, &cmd);
        let event = DomainEvent::CustomerCreated {
            id: aggregate_id,
            source: cmd.source.clone(),
            snapshot,
        };

        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CustomerOutcome { customer_id, aggregate_id })
    }

    /// Update an existing customer — atomic with the
    /// [`WritebackIntent::UpdateCustomer`] enqueue and the modification
    /// event (one PG transaction; audit 2026-06-11 P2 gap-close).
    ///
    /// Returns `ServiceError::NotFound` when the row does not exist (the
    /// repository's `update` reports `0 rows_affected`).
    ///
    /// **Customer soft-deletes intentionally have no writeback** (and no
    /// service method — the route calls the repository directly): the
    /// only legacy delete shape is iHOTEL's destructive hard-DELETE +
    /// `'C0000'` cascade (cheatsheet §3.24), which would irreversibly
    /// destroy data our reversible `cust_active=false` soft-delete can
    /// restore. See `writeback/recipes/update_customer.rs` module doc.
    pub async fn update(&self, cmd: UpdateCustomerCommand) -> ServiceResult<CustomerOutcome> {
        validate_first_name(&cmd.first_name)?;

        let mut tx = self.pg.begin().await?;

        let rows_affected = self
            .repo
            .update(
                &mut tx,
                cmd.customer_id,
                CustomerWrite {
                    first_name: &cmd.first_name,
                    last_name: cmd.last_name.as_deref(),
                    phone: cmd.phone.as_deref(),
                    email: cmd.email.as_deref(),
                    id_card: cmd.id_card.as_deref(),
                    address: cmd.address.as_deref(),
                    customer_type: cmd.customer_type.as_deref(),
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        if rows_affected == 0 {
            return Err(ServiceError::not_found(format!(
                "customer {} does not exist",
                cmd.customer_id
            )));
        }

        // Auth residual (task #56): stamp the editing operator into
        // `cust_updated_by`. Additive + conditional — only runs when an actor
        // was resolved (authenticated user, or the body `updatedBy` fallback),
        // so with auth dark and no body value this is a zero-write no-op and the
        // prior behavior is unchanged. Runs inside this same TX so the audit
        // stamp commits atomically with the canonical UPDATE + writeback enqueue.
        if let Some(by) = cmd.updated_by.as_deref() {
            sqlx::query("UPDATE ht_customers SET cust_updated_by = $1 WHERE cust_id = $2")
                .bind(by)
                .bind(cmd.customer_id)
                .execute(&mut *tx)
                .await?;
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Customer, cmd.customer_id);

        // Coexistence audit 2026-06-11 P2 — mirror the edit onto legacy
        // `HT_Customers` (the iHOTEL 31-field re-save), atomic with the
        // canonical UPDATE. Skipped (with a debug line) when the customer
        // has never been mirrored to MSSQL — see `load_customer_resave`.
        match load_customer_resave(&mut tx, cmd.customer_id).await? {
            Some(resave) => {
                let intent = WritebackIntent::UpdateCustomer {
                    customer_id: aggregate_id,
                    resave,
                };
                // Per-edit discriminator key (NOT the deterministic
                // `(intent, aggregate)` derivation): `writeback_jobs.
                // idempotency_key` is permanently UNIQUE, so a
                // payload-independent key would hard-fail the SECOND edit
                // of the same customer once the first job completed.
                // Customers are edited repeatedly over their lifetime;
                // the enqueue commits atomically with the canonical
                // UPDATE in this TX, so there is no double-submit window
                // for the key to guard. Mirrors `service::payment`'s
                // per-event-aggregate discriminator pattern (see the
                // "caller is responsible for adding a discriminator"
                // note in `outbox::idempotency`).
                let key = generate_idempotency_key(&intent, uuid::Uuid::new_v4());
                OutboxRepository::enqueue(&mut tx, &intent, key)
                    .await
                    .map_err(|err| ServiceError::outbox(err.to_string()))?;
            }
            None => {
                tracing::debug!(
                    customer_id = cmd.customer_id,
                    "customer edit not mirrored to legacy: no legacy_cust_no \
                     yet (first booking/check-in writeback will carry the \
                     current canonical values)"
                );
            }
        }

        let event = DomainEvent::CustomerModified {
            id: aggregate_id,
            source: cmd.source.clone(),
            // Phase 2 keeps the changed-field list empty — Wave 4 routes will
            // diff their request DTOs against the prior repository row and
            // fill this in. Subscribers (SSE) only invalidate by id today.
            changed_fields: Vec::new(),
        };

        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CustomerOutcome {
            customer_id: cmd.customer_id,
            aggregate_id,
        })
    }
}

/// Reject an empty / whitespace-only first name. Mirrors the prior route-side
/// guard so request behavior is preserved when Wave 4 plugs services in.
fn validate_first_name(first_name: &str) -> ServiceResult<()> {
    if first_name.trim().is_empty() {
        Err(ServiceError::validation("first_name must not be empty"))
    } else {
        Ok(())
    }
}

/// Build a [`CustomerSnapshot`] from a create command + the freshly-minted
/// aggregate id. The snapshot intentionally carries only event-bus essentials
/// (per architecture.md §10 — keep payloads small).
fn build_customer_snapshot(aggregate_id: uuid::Uuid, cmd: &CreateCustomerCommand) -> CustomerSnapshot {
    let full_name = match cmd.last_name.as_deref() {
        Some(last) if !last.is_empty() => format!("{} {}", cmd.first_name, last),
        _ => cmd.first_name.clone(),
    };

    CustomerSnapshot {
        id: aggregate_id,
        legacy_cust_no: None,
        name: full_name,
        customer_type: parse_customer_type(cmd.customer_type.as_deref()),
        phone: cmd.phone.clone(),
    }
}

/// Best-effort string → [`CustomerType`] conversion. Unknown / missing values
/// fall back to `Individual` (the legacy app's default).
fn parse_customer_type(raw: Option<&str>) -> CustomerType {
    match raw.map(str::trim).map(str::to_lowercase).as_deref() {
        Some("company") => CustomerType::Company,
        Some("government") => CustomerType::Government,
        Some("other") => CustomerType::Other,
        _ => CustomerType::Individual,
    }
}

#[cfg(test)]
mod tests {
    //! DB-backed tests for the `UpdateCustomer` writeback emission (audit
    //! 2026-06-11 P2). Skip gracefully when no local PG is reachable —
    //! same `try_pool` convention as `service::checkin::tests`.

    use super::*;
    use crate::repository::customer::PgCustomerRepository;

    /// The three DB-backed tests below share the `TEST_UC_%` marker
    /// family and `cleanup()` sweeps by `LIKE 'TEST_UC_%'` — running them
    /// in parallel lets one test's cleanup delete a sibling's seeded
    /// customer mid-flight (observed 2026-06-12: `NotFound("customer N
    /// does not exist")`). One lock keeps them deterministic.
    static UC_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    fn build_service(pool: PgPool) -> CustomerService {
        CustomerService::new(
            Arc::new(PgCustomerRepository::new()),
            Arc::new(OutboxRepository::new()),
            Arc::new(EventBus::new()),
            pool,
        )
    }

    fn update_cmd(customer_id: i32, first_name: &str, phone: &str) -> UpdateCustomerCommand {
        UpdateCustomerCommand {
            customer_id,
            first_name: first_name.into(),
            last_name: Some("CanonicalOnlyLastName".into()),
            phone: Some(phone.into()),
            email: Some("uc@test.invalid".into()),
            id_card: Some("9999999999991".into()),
            address: Some("88/8".into()),
            customer_type: Some("บุคคลธรรมดา".into()),
            notes: None,
            updated_by: None,
            source: EventSource::System { reason: "test".into() },
        }
    }

    /// Remove rows from prior (possibly aborted) runs of these tests.
    async fn cleanup(pool: &PgPool, legacy_cust_no: &str) {
        if let Ok(rows) = sqlx::query("SELECT cust_id FROM ht_customers WHERE legacy_cust_no = $1")
            .bind(legacy_cust_no)
            .fetch_all(pool)
            .await
        {
            for row in rows {
                if let Ok(cust_id) = row.try_get::<i32, _>("cust_id") {
                    let agg = aggregate_uuid(AggregateKind::Customer, cust_id);
                    let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
                        .bind(agg)
                        .execute(pool)
                        .await;
                }
            }
        }
        let _ = sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1 OR cust_firstname LIKE 'TEST_UC_%'")
            .bind(legacy_cust_no)
            .execute(pool)
            .await;
    }

    /// An edit of a legacy-mirrored customer must enqueue exactly one
    /// `update_customer` job whose re-save payload (a) carries the edited
    /// values, (b) maps `Cust_name` from `cust_firstname` VERBATIM (no
    /// lastname join — reconcile-hash alignment), and (c) preserves the
    /// CT-mirrored columns the edit form doesn't expose.
    #[tokio::test]
    async fn update_enqueues_resave_hydrated_from_canonical_row() {
        let _guard = UC_TEST_LOCK.lock().await;
        let Some(pool) = try_pool().await else {
            eprintln!("skipping update_enqueues_resave — PG not reachable");
            return;
        };
        let marker_cust_no = "CTUC01";
        cleanup(&pool, marker_cust_no).await;

        let row = sqlx::query(
            "INSERT INTO ht_customers \
                 (cust_firstname, legacy_cust_no, cust_add_moo, cust_title, cust_sex) \
             VALUES ('TEST_UC_BEFORE', $1, '5', 'นาย', 'ชาย') \
             RETURNING cust_id",
        )
        .bind(marker_cust_no)
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let cust_id: i32 = row.try_get("cust_id").unwrap();

        let svc = build_service(pool.clone());
        svc.update(update_cmd(cust_id, "TEST_UC_AFTER", "0911111111"))
            .await
            .expect("service update must succeed");

        let agg = aggregate_uuid(AggregateKind::Customer, cust_id);
        let job = sqlx::query(
            "SELECT payload FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'update_customer'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .expect("exactly one update_customer job must be enqueued");
        let payload: serde_json::Value = job.try_get("payload").unwrap();
        let resave = &payload["payload"]["resave"];

        // (a) edited values land in the legacy column slots.
        assert_eq!(resave["cust_name"], "TEST_UC_AFTER");
        assert_eq!(resave["cust_add_tel"], "0911111111");
        assert_eq!(resave["cust_idcard"], "9999999999991");
        assert_eq!(resave["cust_add_no"], "88/8");
        assert_eq!(resave["cust_type_main"], "บุคคลธรรมดา");
        // (b) NO lastname join — `Cust_name` hashes against
        // `cust_firstname` in the reconcile sweep.
        assert!(
            !resave["cust_name"].as_str().unwrap().contains("CanonicalOnlyLastName"),
            "cust_name must be cust_firstname verbatim, got {:?}",
            resave["cust_name"]
        );
        // (c) CT-mirrored columns the edit form doesn't expose survive.
        assert_eq!(resave["cust_add_moo"], "5");
        assert_eq!(resave["cust_perfix"], "นาย");
        assert_eq!(resave["cust_sex"], "ชาย");
        assert_eq!(resave["legacy_cust_no"], marker_cust_no);

        cleanup(&pool, marker_cust_no).await;
    }

    /// A customer that has never been mirrored to MSSQL (no
    /// `legacy_cust_no`) must update canonically WITHOUT enqueuing a
    /// writeback — there is no legacy row to target, and the eventual
    /// first booking / check-in writeback carries current values.
    #[tokio::test]
    async fn update_skips_writeback_for_never_mirrored_customer() {
        let _guard = UC_TEST_LOCK.lock().await;
        let Some(pool) = try_pool().await else {
            eprintln!("skipping update_skips_writeback — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_firstname LIKE 'TEST_UCSKIP_%'")
            .execute(&pool)
            .await;

        let row = sqlx::query(
            "INSERT INTO ht_customers (cust_firstname) \
             VALUES ('TEST_UCSKIP_BEFORE') RETURNING cust_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let cust_id: i32 = row.try_get("cust_id").unwrap();
        let agg = aggregate_uuid(AggregateKind::Customer, cust_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(&pool)
            .await;

        let svc = build_service(pool.clone());
        svc.update(update_cmd(cust_id, "TEST_UCSKIP_AFTER", "0922222222"))
            .await
            .expect("service update must succeed");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'update_customer'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 0, "never-mirrored customer must not enqueue a writeback");

        let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
            .bind(cust_id)
            .execute(&pool)
            .await;
    }

    /// Customers are edited repeatedly over their lifetime — the per-edit
    /// discriminator key must let a SECOND edit enqueue a SECOND job
    /// instead of colliding with the permanently-UNIQUE
    /// `writeback_jobs.idempotency_key` of the first.
    #[tokio::test]
    async fn repeated_updates_enqueue_distinct_jobs() {
        let _guard = UC_TEST_LOCK.lock().await;
        let Some(pool) = try_pool().await else {
            eprintln!("skipping repeated_updates — PG not reachable");
            return;
        };
        let marker_cust_no = "CTUC02";
        cleanup(&pool, marker_cust_no).await;

        let row = sqlx::query(
            "INSERT INTO ht_customers (cust_firstname, legacy_cust_no) \
             VALUES ('TEST_UC_TWICE', $1) RETURNING cust_id",
        )
        .bind(marker_cust_no)
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let cust_id: i32 = row.try_get("cust_id").unwrap();

        let svc = build_service(pool.clone());
        svc.update(update_cmd(cust_id, "TEST_UC_TWICE_A", "0901"))
            .await
            .expect("first edit must succeed");
        svc.update(update_cmd(cust_id, "TEST_UC_TWICE_B", "0902"))
            .await
            .expect("second edit must not collide on the idempotency key");

        let agg = aggregate_uuid(AggregateKind::Customer, cust_id);
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeback_jobs \
              WHERE aggregate_id = $1 AND intent = 'update_customer'",
        )
        .bind(agg)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 2, "each edit must enqueue its own job");

        cleanup(&pool, marker_cust_no).await;
    }

    /// Auth residual (task #56): an edit carrying a resolved actor stamps
    /// `cust_updated_by`; an edit with `updated_by = None` (auth dark) leaves
    /// the column untouched.
    #[tokio::test]
    async fn update_stamps_cust_updated_by_when_actor_present() {
        let _guard = UC_TEST_LOCK.lock().await;
        let Some(pool) = try_pool().await else {
            eprintln!("skipping update_stamps_cust_updated_by — PG not reachable");
            return;
        };
        let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_firstname LIKE 'TEST_UCBY_%'")
            .execute(&pool)
            .await;

        let row = sqlx::query(
            "INSERT INTO ht_customers (cust_firstname) \
             VALUES ('TEST_UCBY_BEFORE') RETURNING cust_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let cust_id: i32 = row.try_get("cust_id").unwrap();

        let svc = build_service(pool.clone());

        // Auth dark: None ⇒ column stays NULL.
        svc.update(update_cmd(cust_id, "TEST_UCBY_NOAUTH", "0900000000"))
            .await
            .expect("update without actor must succeed");
        let by: Option<String> =
            sqlx::query_scalar("SELECT cust_updated_by FROM ht_customers WHERE cust_id = $1")
                .bind(cust_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(by, None, "no actor ⇒ cust_updated_by untouched (NULL)");

        // Auth on: the resolved actor lands in cust_updated_by.
        let mut cmd = update_cmd(cust_id, "TEST_UCBY_AUTH", "0900000001");
        cmd.updated_by = Some("alice".into());
        svc.update(cmd).await.expect("update with actor must succeed");
        let by: Option<String> =
            sqlx::query_scalar("SELECT cust_updated_by FROM ht_customers WHERE cust_id = $1")
                .bind(cust_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(by.as_deref(), Some("alice"));

        let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
            .bind(cust_id)
            .execute(&pool)
            .await;
    }
}
