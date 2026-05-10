//! Per-IP rate limiter for `POST /api/auth/login` (Phase 7 audit M-2).
//!
//! Mounted as a route-scoped layer on the login endpoint only — it does
//! NOT touch `/api/auth/me`, `/api/auth/logout`, or any other route. The
//! goal is to blunt password-spraying / credential-stuffing attacks
//! against the public login surface without slowing legitimate session
//! lookups (`/api/auth/me`) or logout.
//!
//! ## Algorithm
//!
//! Sliding-window counter, 10 attempts per IP per 15 minutes. State is
//! held in a process-local `Arc<Mutex<HashMap<IpKey, VecDeque<Instant>>>>`:
//! each entry records the timestamps of recent attempts for that IP.
//! Before each request we prune entries older than the window, then
//! reject when the surviving count is at or above the cap.
//!
//! ## IP resolution
//!
//! 1. `X-Forwarded-For` — leftmost comma-separated entry (the original
//!    client as seen by the outermost proxy).
//! 2. `X-Real-IP` — fallback for proxies that emit only this header.
//! 3. Neither present — a single shared `IpKey::Unknown` bucket. Doing
//!    this means a header-omitting attacker is throttled along with every
//!    other no-header caller, instead of getting a free per-attacker
//!    bucket. Local development without a reverse proxy lands in this
//!    bucket too, but the 10/15min cap is generous enough that it does
//!    not interfere with manual testing.
//!
//! ## Why not `tower-governor`?
//!
//! The crate is well-engineered but `cargo tree -p tower_governor`
//! resolves 261 lines of transitive deps including `tonic` (gRPC). That
//! is far over the 30-line budget the audit task set, so we ship this
//! ~150-line in-process limiter instead. Trade-offs accepted:
//! * Single-process state — fine while the backend runs as one container;
//!   would need Redis-backed state if we ever scale horizontally.
//! * No jitter / no token-bucket smoothing — a sliding window is
//!   sufficient for the threat model (slow brute-force, not bursty
//!   legitimate traffic).

use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Maximum login attempts allowed per IP per [`WINDOW`].
pub const MAX_ATTEMPTS: usize = 10;

/// Sliding window duration applied to [`MAX_ATTEMPTS`].
pub const WINDOW: Duration = Duration::from_secs(15 * 60);

/// Bucket key. Resolved IPs get their own bucket; everything else lands
/// in [`IpKey::Unknown`] so an attacker cannot bypass the limiter by
/// omitting `X-Forwarded-For`/`X-Real-IP`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum IpKey {
    Known(IpAddr),
    Unknown,
}

/// Mutable state shared across requests by the limiter. Wrapped in an
/// `Arc<Mutex<_>>` so the middleware closure owns a cheap clone.
#[derive(Default)]
struct LoginRateLimitInner {
    buckets: HashMap<IpKey, VecDeque<Instant>>,
}

/// Cloneable handle to the limiter — embeds the `Arc<Mutex<_>>` so the
/// router can pass a single instance to `from_fn_with_state` and every
/// concurrent request sees the same bucket map.
#[derive(Clone, Default)]
pub struct LoginRateLimitState {
    inner: Arc<Mutex<LoginRateLimitInner>>,
}

impl LoginRateLimitState {
    /// Construct an empty limiter ready to be cloned into the router.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an attempt at `now` and return whether the request should
    /// proceed. When the request is rejected, returns the number of
    /// seconds the caller must wait before the oldest tracked attempt
    /// ages out of the window.
    fn check(&self, key: IpKey, now: Instant) -> RateLimitDecision {
        let mut guard = self.inner.lock().expect("login rate-limit mutex poisoned");
        let entry = guard.buckets.entry(key).or_default();

        // Drop attempts that have aged out of the sliding window.
        prune_expired(entry, now);

        if entry.len() >= MAX_ATTEMPTS {
            // Caller must wait until the oldest tracked attempt ages out.
            let oldest = entry.front().copied().unwrap_or(now);
            let elapsed = now.saturating_duration_since(oldest);
            let remaining = WINDOW.saturating_sub(elapsed);
            // Round up so a sub-second remainder still surfaces as "1".
            let retry_after = remaining.as_secs().max(1);
            return RateLimitDecision::Reject { retry_after };
        }

        entry.push_back(now);
        RateLimitDecision::Allow
    }
}

/// Outcome of a rate-limit check.
#[derive(Debug, PartialEq, Eq)]
enum RateLimitDecision {
    Allow,
    Reject { retry_after: u64 },
}

/// JSON body returned with the 429 response. Matches the audit spec
/// (`{"error":"too_many_attempts"}`).
#[derive(Debug, Serialize)]
struct TooManyAttemptsBody {
    error: &'static str,
}

/// Drop timestamps older than [`WINDOW`] from the front of `entry`.
fn prune_expired(entry: &mut VecDeque<Instant>, now: Instant) {
    while let Some(front) = entry.front() {
        if now.saturating_duration_since(*front) >= WINDOW {
            entry.pop_front();
        } else {
            break;
        }
    }
}

/// Resolve the bucket key for the request.
///
/// Mirrors [`crate::routes::auth::client_ip`] in spirit — we deliberately
/// re-implement it here (as a private helper) so the rate limiter does
/// not import handler internals AND so the missing-header case maps to
/// `IpKey::Unknown` instead of a `None` that would silently disable
/// throttling.
fn resolve_key(headers: &HeaderMap) -> IpKey {
    if let Some(ip) = parse_first_xff(headers) {
        return IpKey::Known(ip);
    }
    if let Some(ip) = parse_x_real_ip(headers) {
        return IpKey::Known(ip);
    }
    IpKey::Unknown
}

fn parse_first_xff(headers: &HeaderMap) -> Option<IpAddr> {
    let value = headers.get("x-forwarded-for")?.to_str().ok()?;
    let first = value.split(',').next()?.trim();
    first.parse::<IpAddr>().ok()
}

fn parse_x_real_ip(headers: &HeaderMap) -> Option<IpAddr> {
    let value = headers.get("x-real-ip")?.to_str().ok()?;
    value.trim().parse::<IpAddr>().ok()
}

/// Axum middleware function — mount via
/// `axum::middleware::from_fn_with_state(limiter, login_rate_limit)`.
pub async fn login_rate_limit(
    State(limiter): State<LoginRateLimitState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let key = resolve_key(request.headers());
    match limiter.check(key, Instant::now()) {
        RateLimitDecision::Allow => next.run(request).await,
        RateLimitDecision::Reject { retry_after } => too_many_attempts(retry_after),
    }
}

fn too_many_attempts(retry_after_seconds: u64) -> Response {
    let body = Json(TooManyAttemptsBody {
        error: "too_many_attempts",
    });
    let mut response = (StatusCode::TOO_MANY_REQUESTS, body).into_response();
    if let Ok(value) = HeaderValue::from_str(&retry_after_seconds.to_string()) {
        response.headers_mut().insert("retry-after", value);
    }
    response
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn ipv4(octets: [u8; 4]) -> IpAddr {
        IpAddr::from(octets)
    }

    #[test]
    fn allows_first_attempts_within_cap() {
        let limiter = LoginRateLimitState::new();
        let key = IpKey::Known(ipv4([203, 0, 113, 4]));
        let now = Instant::now();
        for _ in 0..MAX_ATTEMPTS {
            assert_eq!(limiter.check(key.clone(), now), RateLimitDecision::Allow);
        }
    }

    #[test]
    fn rejects_attempt_at_cap_plus_one() {
        let limiter = LoginRateLimitState::new();
        let key = IpKey::Known(ipv4([203, 0, 113, 4]));
        let now = Instant::now();
        for _ in 0..MAX_ATTEMPTS {
            limiter.check(key.clone(), now);
        }
        match limiter.check(key, now) {
            RateLimitDecision::Reject { retry_after } => {
                // Window is 15 * 60 = 900s; first attempt was at `now`
                // so the bucket clears in 900s. saturating math caps to
                // WINDOW.as_secs(); minimum surfaced value is 1.
                assert!((1..=WINDOW.as_secs()).contains(&retry_after));
            }
            other => panic!("expected Reject, got {:?}", other),
        }
    }

    #[test]
    fn separate_ips_have_independent_buckets() {
        let limiter = LoginRateLimitState::new();
        let attacker = IpKey::Known(ipv4([198, 51, 100, 7]));
        let victim = IpKey::Known(ipv4([203, 0, 113, 4]));
        let now = Instant::now();
        for _ in 0..MAX_ATTEMPTS {
            limiter.check(attacker.clone(), now);
        }
        // Attacker bucket is full — but the victim's IP must still pass.
        assert_eq!(limiter.check(victim, now), RateLimitDecision::Allow);
    }

    #[test]
    fn unknown_bucket_is_shared_across_headerless_callers() {
        // Two distinct headerless requests both land in IpKey::Unknown,
        // so they share a bucket. This is the anti-bypass property.
        let limiter = LoginRateLimitState::new();
        let now = Instant::now();
        for _ in 0..MAX_ATTEMPTS {
            limiter.check(IpKey::Unknown, now);
        }
        assert!(matches!(
            limiter.check(IpKey::Unknown, now),
            RateLimitDecision::Reject { .. }
        ));
    }

    #[test]
    fn attempts_age_out_after_window() {
        let limiter = LoginRateLimitState::new();
        let key = IpKey::Known(ipv4([203, 0, 113, 4]));
        let t0 = Instant::now();
        for _ in 0..MAX_ATTEMPTS {
            limiter.check(key.clone(), t0);
        }
        // Right at the window boundary the entries age out and a fresh
        // attempt is allowed again.
        let t1 = t0 + WINDOW + Duration::from_secs(1);
        assert_eq!(limiter.check(key, t1), RateLimitDecision::Allow);
    }

    #[test]
    fn resolve_key_prefers_first_xff_entry() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.4, 10.0.0.1".parse().unwrap());
        assert_eq!(resolve_key(&headers), IpKey::Known(ipv4([203, 0, 113, 4])));
    }

    #[test]
    fn resolve_key_falls_back_to_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "198.51.100.7".parse().unwrap());
        assert_eq!(resolve_key(&headers), IpKey::Known(ipv4([198, 51, 100, 7])));
    }

    #[test]
    fn resolve_key_returns_unknown_without_proxy_headers() {
        let headers = HeaderMap::new();
        assert_eq!(resolve_key(&headers), IpKey::Unknown);
    }

    #[test]
    fn resolve_key_returns_unknown_for_unparseable_header() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "not-an-ip".parse().unwrap());
        // Falls past XFF parse failure → no x-real-ip → Unknown bucket.
        assert_eq!(resolve_key(&headers), IpKey::Unknown);
    }

    #[test]
    fn prune_expired_drops_only_old_entries() {
        let mut entries: VecDeque<Instant> = VecDeque::new();
        let t0 = Instant::now();
        entries.push_back(t0);
        entries.push_back(t0 + Duration::from_secs(60));
        entries.push_back(t0 + Duration::from_secs(120));
        // "now" is t0 + WINDOW + 65s — so the first two entries are
        // outside the window and the third is still inside.
        prune_expired(&mut entries, t0 + WINDOW + Duration::from_secs(65));
        assert_eq!(entries.len(), 1);
        assert_eq!(entries.front(), Some(&(t0 + Duration::from_secs(120))));
    }

    #[test]
    fn too_many_attempts_response_carries_retry_after() {
        let response = too_many_attempts(42);
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            response.headers().get("retry-after").and_then(|v| v.to_str().ok()),
            Some("42")
        );
    }
}
