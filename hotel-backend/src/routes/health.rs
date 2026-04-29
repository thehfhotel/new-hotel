//! Healthcheck endpoint (`/health`).
//!
//! Returns `{ "site": "<id>", "ok": true, "service": "backend" }` so:
//!   * external monitors / smoke tests can confirm the right deployment
//!     responded (HF Hotel + HF Ville run two separate instances of
//!     this binary from Phase 5 onward — task #69),
//!   * the load-balancer health probe still sees the same `200 OK` it
//!     always did (the response shape is additive — adding a field
//!     never breaks a probe that only checks the status code).
//!
//! Deliberately stateless: never queries PG / MSSQL. The status-code
//! contract has historically been "process is alive" and we don't
//! change it here — deeper readiness checks live in `/api/new/sync/status`.

use axum::{extract::State, response::Json};
use serde_json::{json, Value};

/// State carried by the healthcheck handler. Just the site id — the
/// process that's serving has access to nothing else interesting from
/// a liveness perspective.
#[derive(Clone, Debug)]
pub struct HealthState {
    pub site_id: String,
}

/// `GET /health` — returns site id + ok flag.
pub async fn health(State(state): State<HealthState>) -> Json<Value> {
    Json(json!({
        "site": state.site_id,
        "ok": true,
        "service": "backend",
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
        let state = HealthState { site_id: "hfville".to_string() };
        let Json(body) = health(State(state)).await;
        assert_eq!(body.get("site").and_then(|v| v.as_str()), Some("hfville"));
        assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    }

    /// Default deploy (HF Hotel) keeps responding with the legacy site
    /// id. Locks the back-compat contract.
    #[tokio::test]
    async fn health_default_site_is_hfhotel() {
        let state = HealthState { site_id: "hfhotel".to_string() };
        let Json(body) = health(State(state)).await;
        assert_eq!(body.get("site").and_then(|v| v.as_str()), Some("hfhotel"));
    }
}
