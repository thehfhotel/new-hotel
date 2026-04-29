//! Healthcheck endpoint (`/health`).
//!
//! Returns site id + ok flag + (task #78) the canonical PG sync watermark
//! so external monitors / smoke tests can confirm:
//!   * the right deployment responded (HF Hotel + HF Ville run two
//!     separate instances of this binary from Phase 5 onward — task #69),
//!   * the CT watcher's last-observed `SYS_CHANGE_VERSION` and last poll
//!     timestamp (task #78), giving observability dashboards a single
//!     endpoint to scrape per backend instance.
//!
//! Response shape:
//! ```json
//! {
//!   "ok": true,
//!   "service": "backend",
//!   "site": "hfhotel",
//!   "ct_watermark": 1234,
//!   "last_polled_at": "2026-04-29T12:34:56Z"
//! }
//! ```
//!
//! ## Liveness vs readiness
//!
//! `/health` is the LIVENESS contract — `ok: true` + HTTP 200 means "the
//! backend process is alive and serving HTTP". The CT watermark fields
//! are observability metadata, NOT a readiness signal:
//!
//! * If `legacy_ct_state` is empty (pre-bootstrap), the fields are
//!   `null`. The healthcheck still returns `ok: true`. The backend is
//!   alive even if sync hasn't started.
//! * If the PG query errors (DB down, network blip, etc.), the fields
//!   are `null` and a `tracing::warn!` is emitted. Still `ok: true` —
//!   the backend is alive even if PG is unreachable.
//! * If no PG pool was configured at startup (rare; legacy-only mode),
//!   the fields are `null`. Still `ok: true`.
//!
//! Deeper readiness checks (sync freshness gating, drift counts, etc.)
//! live in `/api/new/sync/status` and `scripts/sync-status.sh
//! --readiness`. Don't expand THIS endpoint into a sync gate — that
//! would silently break the load-balancer probe whenever the watcher
//! hiccups.

use axum::{extract::State, response::Json};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::PgPool;

/// State carried by the healthcheck handler.
///
/// `pg_pool` is optional because legacy-only mode (no PG configured) is
/// still a supported topology. When `None`, the watermark fields render
/// as `null`.
#[derive(Clone, Debug)]
pub struct HealthState {
    pub site_id: String,
    pub pg_pool: Option<PgPool>,
}

/// Snapshot of the canonical PG `legacy_ct_state` row.
///
/// Held as a struct (rather than ad-hoc JSON building) so the SQL query
/// shape and the JSON shape stay decoupled — adding a column to
/// `legacy_ct_state` won't accidentally widen the public response.
struct CtState {
    last_seen_version: i64,
    last_polled_at: DateTime<Utc>,
}

/// Read the single-row CT watermark. Returns:
/// * `Ok(Some(state))` when the row exists.
/// * `Ok(None)` when the table is empty (pre-bootstrap install).
/// * `Err(_)` only on a SQL/connection error — the caller swallows this
///   and renders `null` for the watermark fields. We deliberately do
///   NOT propagate errors out of `/health`; this endpoint's contract is
///   backend liveness, not sync readiness.
async fn fetch_ct_state(pool: &PgPool) -> Result<Option<CtState>, sqlx::Error> {
    let row: Option<(i64, DateTime<Utc>)> = sqlx::query_as(
        "SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id = 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(v, ts)| CtState {
        last_seen_version: v,
        last_polled_at: ts,
    }))
}

/// `GET /health` — returns site id + ok flag + CT watermark snapshot.
pub async fn health(State(state): State<HealthState>) -> Json<Value> {
    // Resolve the watermark snapshot. Errors (no pool, no row, SQL
    // failure) all collapse to `(null, null)` — see module docs for the
    // liveness-vs-readiness reasoning.
    let (ct_watermark, last_polled_at): (Value, Value) = match &state.pg_pool {
        Some(pool) => match fetch_ct_state(pool).await {
            Ok(Some(s)) => (
                json!(s.last_seen_version),
                json!(s.last_polled_at.to_rfc3339()),
            ),
            Ok(None) => (Value::Null, Value::Null),
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "/health failed to read legacy_ct_state; responding with null watermark fields"
                );
                (Value::Null, Value::Null)
            }
        },
        None => (Value::Null, Value::Null),
    };

    Json(json!({
        "site": state.site_id,
        "ok": true,
        "service": "backend",
        "ct_watermark": ct_watermark,
        "last_polled_at": last_polled_at,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The response body MUST carry the site id verbatim — that's the
    /// whole point of task #69's healthcheck change. A regression here
    /// would silently break smoke tests against the HF Ville deploy
    /// (they assert site=hfville).
    #[tokio::test]
    async fn health_includes_configured_site_id() {
        let state = HealthState {
            site_id: "hfville".to_string(),
            pg_pool: None,
        };
        let Json(body) = health(State(state)).await;
        assert_eq!(body.get("site").and_then(|v| v.as_str()), Some("hfville"));
        assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    }

    /// Default deploy (HF Hotel) keeps responding with the legacy site
    /// id. Locks the back-compat contract.
    #[tokio::test]
    async fn health_default_site_is_hfhotel() {
        let state = HealthState {
            site_id: "hfhotel".to_string(),
            pg_pool: None,
        };
        let Json(body) = health(State(state)).await;
        assert_eq!(body.get("site").and_then(|v| v.as_str()), Some("hfhotel"));
    }

    /// Pre-bootstrap topology (no PG pool configured at all): the
    /// watermark fields are present but explicitly `null`. They MUST
    /// be present-and-null, not omitted — Slack/Datadog scrapers key on
    /// the existence of the field to detect "this site never bootstrapped"
    /// vs "this site is using an old backend that doesn't expose it".
    #[tokio::test]
    async fn health_pre_bootstrap_serializes_null_watermark_fields() {
        let state = HealthState {
            site_id: "hfhotel".to_string(),
            pg_pool: None,
        };
        let Json(body) = health(State(state)).await;
        // The keys must exist…
        assert!(
            body.as_object().unwrap().contains_key("ct_watermark"),
            "ct_watermark key must be present even when null"
        );
        assert!(
            body.as_object().unwrap().contains_key("last_polled_at"),
            "last_polled_at key must be present even when null"
        );
        // …and resolve to JSON null (not omitted, not 0, not "").
        assert!(body.get("ct_watermark").unwrap().is_null());
        assert!(body.get("last_polled_at").unwrap().is_null());
    }

    /// The populated case: a `CtState` snapshot serializes into an
    /// integer watermark + RFC3339 timestamp string. We test the JSON
    /// shape directly (without an sqlx pool) by short-circuiting the
    /// renderer — the SQL fetch path is exercised by integration tests
    /// in `tests/test_sync_watermark.rs`.
    #[tokio::test]
    async fn health_populated_watermark_serializes_int_and_rfc3339() {
        let snapshot = CtState {
            last_seen_version: 1234,
            last_polled_at: chrono::DateTime::parse_from_rfc3339("2026-04-29T12:34:56Z")
                .unwrap()
                .with_timezone(&Utc),
        };
        let body = json!({
            "site": "hfhotel",
            "ok": true,
            "service": "backend",
            "ct_watermark": snapshot.last_seen_version,
            "last_polled_at": snapshot.last_polled_at.to_rfc3339(),
        });
        assert_eq!(body.get("ct_watermark").and_then(|v| v.as_i64()), Some(1234));
        assert_eq!(
            body.get("last_polled_at").and_then(|v| v.as_str()),
            Some("2026-04-29T12:34:56+00:00")
        );
    }
}
