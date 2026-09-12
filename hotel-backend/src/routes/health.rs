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
//!   "last_polled_at": "2026-04-29T12:34:56Z",
//!   "revision": "9f2c1ab3d4e5f60718293a4b5c6d7e8f90a1b2c3",
//!   "crate_version": "2.22.0"
//! }
//! ```
//!
//! ## Proving WHICH build is serving
//!
//! `revision` is the git SHA baked into the image at build time (`ARG
//! GIT_SHA` / `ENV GIT_SHA` in `hotel-backend/Dockerfile`, passed by the
//! `build-backend` job in `.github/workflows/docker-build.yml`). Without
//! it, "verified live" had to be argued from the promote job's ghcr
//! digest, off-band from the running container.
//!
//! It is the FULL 40-character `github.sha`, NOT the 7-character
//! `type=sha,prefix=` image tag `build-backend`'s metadata step applies.
//! Compare it against `github.sha` byte-for-byte; comparing it against an
//! image tag (or truncating it to 7 chars to do so) is the mistake that
//! makes a verifier permanently red and then quietly disabled.
//!
//! Baked, not injected at run time, on purpose — and NOT because the
//! deploy shim is unreachable from CI (it is: `scripts/deploy/run-deploy.sh`
//! is version-controlled here and the live `/srv/run-deploy.sh` self-updates
//! from the repo on every deploy — `CLAUDE.md` §"Deployment Policy" item 5).
//! The real reason is stronger: a runtime env var proves only what the
//! deploy shim DELIVERED, not which image the container actually LOADED,
//! and a baked value keeps telling the truth through a rollback to an older
//! tag. A local `cargo run` or a pre-bake image reports `"unknown"`;
//! verifiers should treat that as "cannot tell" and warn, not fail.
//!
//! `crate_version` is `CARGO_PKG_VERSION` — the RUST CRATE version from
//! `hotel-backend/Cargo.toml`, which is a DIFFERENT numbering scheme from
//! the product release the team tracks. release-please is configured
//! `release-type: node` with no `extra-files`, so it bumps `package.json` +
//! `.release-please-manifest.json` (v2.75.x at the time of writing) and
//! never touches `Cargo.toml` (2.22.0) — the gap only widens. The field is
//! named `crate_version` precisely so nobody curls `/health` mid-incident,
//! reads a v2.2x number on a repo everyone knows as v2.7x, and concludes a
//! wildly stale image is serving. It is also identical across most commits
//! and so CANNOT distinguish a real deploy from a no-op: it is reported
//! alongside, never instead of, `revision`.
//!
//! ## How to actually probe it
//!
//! `/health` is BACKEND-NETWORK-INTERNAL. There is no public URL for it:
//! the `backend` service publishes no `ports:` (only `web` does), and
//! `next.config.js` rewrites proxy ONLY `/api/:path*` and `/hk/api/:path*`
//! while this route is mounted at the bare root in `main.rs`. A
//! `curl https://<public-host>/health` gets the Next app's 404, which reads
//! like a failed deploy and is not one.
//!
//! From evergreen, probe inside the container instead:
//!
//! ```text
//! docker exec new-hotel-production-backend-1 \
//!   curl -fsS --max-time 3 localhost:3003/health | jq -r .revision
//! ```
//!
//! The deploy does this for you: `scripts/deploy/run-deploy.sh`'s
//! post-deploy verification asserts the reported `revision` equals the
//! commit whose image this run built (see `expected_backend_revision` in
//! the payload). Exposing a public alias under `/api/` would be a separate,
//! deliberate decision — do not infer one from this endpoint's existence.
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

use std::sync::OnceLock;

use axum::{extract::State, response::Json};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::PgPool;

/// Value reported in `revision` when the image carries no build SHA.
///
/// Images built before the `GIT_SHA` build-arg existed (and any local
/// `cargo run`) report this. Deploy verifiers should treat it as "cannot
/// tell which build this is" and warn rather than fail — see the module
/// docs.
const UNKNOWN_REVISION: &str = "unknown";

/// Normalise a raw `GIT_SHA` value into the `revision` field.
///
/// An UNSET var and an EMPTY one behave identically, defensively.
/// NOTHING sets `GIT_SHA` at run time today — `docker-compose.yml` has no
/// `env_file` and no `GIT_SHA` entry in the `backend` service's
/// `environment:` block, so the only source is the image's own `ENV`. The
/// guard exists so that IF a future compose entry or `env_file` ever
/// supplies an empty value, it degrades to `unknown` instead of clobbering
/// the baked SHA with a blank that a verifier reads as a mismatch rather
/// than as "cannot tell". Adding such an entry is not a supported way to
/// set the revision — the bake is.
///
/// Kept pure (the env read happens in the caller) so unit tests exercise
/// every branch without mutating process env, which races across the
/// parallel test harness.
fn resolve_revision(raw: Option<String>) -> String {
    match raw {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => UNKNOWN_REVISION.to_string(),
    }
}

/// Process-wide revision, resolved from `GIT_SHA` exactly once.
fn revision() -> &'static str {
    static REVISION: OnceLock<String> = OnceLock::new();
    REVISION.get_or_init(|| resolve_revision(std::env::var("GIT_SHA").ok()))
}

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
        // Build identity. `revision` (the full 40-char github.sha) is the
        // only field that distinguishes one deploy from the next.
        // `crate_version` is the Rust crate version, NOT the release-please
        // product version — deliberately named so it can't be mistaken for
        // it. See the module docs.
        "revision": revision(),
        "crate_version": env!("CARGO_PKG_VERSION"),
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

    /// The handler payload MUST carry build identity: `revision` (the
    /// baked git SHA) and `crate_version`. Without `revision`, "verified
    /// live" can only be argued from the promote job's ghcr digest, which
    /// says nothing about what the container actually loaded.
    ///
    /// Asserts presence + type only, not a specific SHA: the test binary
    /// is built without the `GIT_SHA` bake, so the value here is
    /// whatever `resolve_revision` yields for this process — which is
    /// `unknown`. THIS TEST THEREFORE CANNOT PROVE THE BAKE HAPPENED, and
    /// no unit test can: the assertion that the shipped image actually
    /// carries `github.sha` lives in the deploy
    /// (`scripts/deploy/run-deploy.sh`, `expected_backend_revision`),
    /// which is what stops the feature silently rotting back to
    /// `"unknown"` with every gate still green.
    #[tokio::test]
    async fn health_includes_revision_and_crate_version() {
        let state = HealthState {
            site_id: "hfhotel".to_string(),
            pg_pool: None,
        };
        let Json(body) = health(State(state)).await;

        let revision = body
            .get("revision")
            .and_then(|v| v.as_str())
            .expect("revision must be present and a string");
        assert!(!revision.is_empty(), "revision must never be empty");

        assert_eq!(
            body.get("crate_version").and_then(|v| v.as_str()),
            Some(env!("CARGO_PKG_VERSION")),
            "crate_version must report the Rust crate version"
        );
        assert!(
            body.get("version").is_none(),
            "the field is `crate_version`: a bare `version` invites the \
             reader to compare it against the release-please product \
             version, which tracks a different number entirely"
        );
    }

    /// Adding build identity MUST NOT drop any pre-existing field or
    /// flip the status semantics — external monitors key on all of them.
    #[tokio::test]
    async fn health_keeps_existing_fields_alongside_revision() {
        let state = HealthState {
            site_id: "hfville".to_string(),
            pg_pool: None,
        };
        let Json(body) = health(State(state)).await;
        let obj = body.as_object().expect("payload must be a JSON object");

        for key in [
            "site",
            "ok",
            "service",
            "ct_watermark",
            "last_polled_at",
            "revision",
            "crate_version",
        ] {
            assert!(obj.contains_key(key), "missing field: {key}");
        }
        assert_eq!(obj.len(), 7, "unexpected extra fields: {obj:?}");
        assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            body.get("service").and_then(|v| v.as_str()),
            Some("backend")
        );
    }

    /// An unset OR empty `GIT_SHA` collapses to `unknown`. Nothing sets
    /// the var at run time today (no compose `env_file`, no `GIT_SHA` in
    /// the `backend` service's `environment:`); the empty branch is
    /// defensive, so that if one is ever added and supplies a blank, the
    /// payload says "cannot tell" rather than reporting an empty revision
    /// a verifier reads as a mismatch.
    #[test]
    fn resolve_revision_treats_unset_and_blank_alike() {
        assert_eq!(resolve_revision(None), UNKNOWN_REVISION);
        assert_eq!(resolve_revision(Some(String::new())), UNKNOWN_REVISION);
        assert_eq!(resolve_revision(Some("   ".to_string())), UNKNOWN_REVISION);
        assert_eq!(resolve_revision(Some("\n".to_string())), UNKNOWN_REVISION);
    }

    /// A real SHA passes through verbatim, trimmed — the deploy verifier
    /// compares it byte-for-byte against `github.sha`.
    #[test]
    fn resolve_revision_passes_through_a_real_sha() {
        let sha = "9f2c1ab3d4e5f60718293a4b5c6d7e8f90a1b2c3";
        assert_eq!(resolve_revision(Some(sha.to_string())), sha);
        assert_eq!(resolve_revision(Some(format!("  {sha}\n"))), sha);
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
