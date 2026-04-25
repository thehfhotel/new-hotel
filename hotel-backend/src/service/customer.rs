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

use sqlx::PgPool;

use crate::domain::customer::CustomerType;
use crate::outbox::event::{CustomerSnapshot, DomainEvent, EventSource};
use crate::outbox::{EventBus, OutboxRepository};
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

    /// Update an existing customer — atomic with the modification event.
    ///
    /// Returns `ServiceError::NotFound` when the row does not exist (the
    /// repository's `update` reports `0 rows_affected`).
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

        let aggregate_id = aggregate_uuid(AggregateKind::Customer, cmd.customer_id);
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
