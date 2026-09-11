//! Loyalty stay hook — POSTs completed stays of membership-linked guests to
//! the loyalty app on checkout. See `docs/loyalty-channel.md` (Piece 3).
//!
//! Contract (locked):
//!
//! ```text
//! POST {LOYALTY_APP_URL}/api/loyalty/stays
//! Authorization: Bearer {LOYALTY_SERVICE_TOKEN}
//! { "pms_stay_id": "hf-98765", "membership_id": "...", "property": "hf",
//!   "check_in": "2026-07-01", "check_out": "2026-07-03", "nights": 2 }
//! ```
//!
//! The loyalty side is idempotent on `pms_stay_id`, so retries are safe.
//! `pms_stay_id = "{property}-{cin_id}"` — `cin_id` is the stable per-site
//! stay id, and the property prefix disambiguates the two per-site databases
//! whose SERIAL sequences overlap.
//!
//! ## Failure posture (locked by the contract)
//!
//! Best-effort with bounded retries (3 attempts, 1s/2s/4s backoff — the
//! `SlackClient::send_message` idiom); on persistent failure log loudly AND
//! page Slack, but NEVER block or fail the checkout itself. The hook runs in
//! a detached `tokio::spawn` after the checkout transaction committed.
//!
//! Feature is OFF unless both `LOYALTY_APP_URL` and `LOYALTY_SERVICE_TOKEN`
//! are set ([`LoyaltyClient::from_env`] returns `None` — fail closed).
//!
//! Client shape mirrors `service::reader::HttpHfIdClient` / `SlackClient`:
//! blocking `ureq` dispatched via `tokio::task::spawn_blocking` (deliberate
//! repo choice over reqwest — see Cargo.toml).

use std::time::Duration;

use chrono::NaiveDate;
use serde::Serialize;

use crate::config::{LoyaltyConfig, SlackConfig};
use crate::db::PgPool;
use crate::notifications::slack::{SlackClient, SlackMessage};
use crate::repository::channel::{stay_snapshot_for_loyalty, StaySnapshot};

/// Wire payload for `POST /api/loyalty/stays`. Field names are the LOCKED
/// interface contract — do not rename.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LoyaltyStayPayload {
    pub pms_stay_id: String,
    pub membership_id: String,
    pub property: String,
    /// ISO `YYYY-MM-DD`.
    pub check_in: String,
    /// ISO `YYYY-MM-DD`.
    pub check_out: String,
    pub nights: i64,
}

/// Build the stay payload from checkout facts. Pure — unit-tested below.
/// `nights` is whole days between the dates, floored at 1 (a same-day
/// checkout still counts one night — matches the folio's `.max(1)` basis).
pub fn build_stay_payload(
    property: &str,
    cin_id: i32,
    membership_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> LoyaltyStayPayload {
    LoyaltyStayPayload {
        pms_stay_id: format!("{property}-{cin_id}"),
        membership_id: membership_id.to_string(),
        property: property.to_string(),
        check_in: check_in.format("%Y-%m-%d").to_string(),
        check_out: check_out.format("%Y-%m-%d").to_string(),
        nights: (check_out - check_in).num_days().max(1),
    }
}

/// Decide whether a just-checked-out stay should be posted, and with what
/// payload. Pure — the full gate matrix is unit-tested without a database.
/// `None` ⇒ nothing to send (not fully checked out, or no membership link).
pub fn stay_payload_from_snapshot(
    property: &str,
    cin_id: i32,
    snap: &StaySnapshot,
) -> Option<LoyaltyStayPayload> {
    // Per-room partial checkout keeps the header 'active' until the last
    // room leaves; only a fully-completed stay earns loyalty credit (and the
    // idempotent pms_stay_id means the completing checkout sends exactly one).
    if snap.cin_status != "checkedout" {
        return None;
    }
    let membership = snap.membership_id.as_deref()?.trim();
    if membership.is_empty() {
        return None;
    }
    let check_in = snap.check_in_time.date();
    let check_out = snap
        .check_out_time
        .map(|t| t.date())
        .unwrap_or(snap.expected_checkout);
    Some(build_stay_payload(property, cin_id, membership, check_in, check_out))
}

const MAX_ATTEMPTS: u32 = 3;

/// Outbound HTTP client for the loyalty app. Construct via [`from_env`]
/// (`None` = feature off) or [`from_config`].
#[derive(Clone)]
pub struct LoyaltyClient {
    agent: ureq::Agent,
    base_url: String,
    token: String,
}

impl LoyaltyClient {
    pub fn from_config(config: &LoyaltyConfig) -> Option<Self> {
        let base_url = config.app_url.clone()?;
        let token = config.service_token.clone()?;
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build();
        Some(Self {
            agent,
            base_url,
            token,
        })
    }

    /// Read `LOYALTY_APP_URL` + `LOYALTY_SERVICE_TOKEN`; `None` when either
    /// is unset/empty (hook off, fail closed).
    pub fn from_env() -> Option<Self> {
        Self::from_config(&LoyaltyConfig::from_env())
    }

    fn stays_url(&self) -> String {
        format!("{}/api/loyalty/stays", self.base_url.trim_end_matches('/'))
    }

    /// POST the stay with bounded retries. `true` = the loyalty app accepted
    /// it (2xx). Non-retryable client errors (4xx except 408/429) abort the
    /// loop early — a malformed payload won't heal by retrying.
    pub async fn post_stay(&self, payload: &LoyaltyStayPayload) -> bool {
        let url = self.stays_url();
        let body = match serde_json::to_value(payload) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(error = %e, "loyalty stay hook: payload serialization failed");
                return false;
            }
        };

        for attempt in 1..=MAX_ATTEMPTS {
            let agent = self.agent.clone();
            let url_c = url.clone();
            let token = self.token.clone();
            let body_c = body.clone();

            let result = tokio::task::spawn_blocking(move || {
                agent
                    .post(&url_c)
                    .set("Authorization", &format!("Bearer {token}"))
                    .send_json(body_c)
            })
            .await;

            match result {
                Ok(Ok(response)) => {
                    // ureq returns Ok for any 2xx (200 and 201 both land here).
                    tracing::info!(
                        pms_stay_id = %payload.pms_stay_id,
                        status = response.status(),
                        "loyalty stay hook: accepted"
                    );
                    return true;
                }
                Ok(Err(ureq::Error::Status(status, response))) => {
                    let detail = response.into_string().unwrap_or_default();
                    tracing::error!(
                        pms_stay_id = %payload.pms_stay_id,
                        attempt,
                        max = MAX_ATTEMPTS,
                        status,
                        detail = %detail,
                        "loyalty stay hook: rejected"
                    );
                    // Retrying a definitive client rejection is pointless
                    // (408 request-timeout / 429 rate-limit are the retryable
                    // exceptions).
                    if (400..500).contains(&status) && status != 408 && status != 429 {
                        return false;
                    }
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        pms_stay_id = %payload.pms_stay_id,
                        attempt,
                        max = MAX_ATTEMPTS,
                        error = %e,
                        "loyalty stay hook: transport error"
                    );
                }
                Err(join_err) => {
                    tracing::error!(
                        pms_stay_id = %payload.pms_stay_id,
                        attempt,
                        max = MAX_ATTEMPTS,
                        error = %join_err,
                        "loyalty stay hook: blocking task panicked"
                    );
                }
            }

            if attempt < MAX_ATTEMPTS {
                tokio::time::sleep(Duration::from_secs(2u64.pow(attempt - 1))).await;
            }
        }
        false
    }
}

/// Fire-and-forget checkout hook body. Called from a detached task spawned
/// by `routes::new_checkins::checkout` AFTER the checkout committed — every
/// failure path here logs/pages and returns; nothing can affect the checkout.
///
/// Reads the post-commit stay snapshot (so a per-room partial checkout that
/// did NOT complete the stay is naturally skipped), gates on the membership
/// link, then posts with retries. Persistent failure ⇒ `tracing::error` +
/// Slack page (the repo's failure-surfacing convention) with enough context
/// to replay by hand — the loyalty side is idempotent on `pms_stay_id`.
pub async fn run_checkout_stay_hook(client: LoyaltyClient, pool: PgPool, property: String, cin_id: i32) {
    let snapshot = match stay_snapshot_for_loyalty(&pool, cin_id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            tracing::warn!(cin_id, "loyalty stay hook: check-in vanished after checkout");
            return;
        }
        Err(e) => {
            tracing::error!(cin_id, error = %e, "loyalty stay hook: snapshot query failed");
            return;
        }
    };

    let Some(payload) = stay_payload_from_snapshot(&property, cin_id, &snapshot) else {
        tracing::debug!(
            cin_id,
            status = %snapshot.cin_status,
            has_membership = snapshot.membership_id.is_some(),
            "loyalty stay hook: nothing to send"
        );
        return;
    };

    if client.post_stay(&payload).await {
        return;
    }

    // Persistent failure: the stay was NOT credited. Loud log + Slack page so
    // an operator can replay (idempotent on pms_stay_id).
    tracing::error!(
        pms_stay_id = %payload.pms_stay_id,
        membership_id = %payload.membership_id,
        "loyalty stay hook: FAILED after {MAX_ATTEMPTS} attempts — stay not credited; replay manually"
    );
    let slack = SlackClient::new(SlackConfig::from_env());
    let message = SlackMessage::with_site_text(
        &property,
        format!(
            ":rotating_light: loyalty stay hook failed after {MAX_ATTEMPTS} attempts — \
             stay {} (membership {}) was NOT credited. Check LOYALTY_APP_URL / the \
             loyalty app, then replay: POST /api/loyalty/stays is idempotent on pms_stay_id.",
            payload.pms_stay_id, payload.membership_id
        ),
    );
    slack.send_message(&message).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    fn dt(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    // ----- payload construction (the LOCKED wire contract) -----

    #[test]
    fn payload_shape_matches_locked_contract() {
        let p = build_stay_payload("hf", 98765, "M-000123", d("2026-07-01"), d("2026-07-03"));
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "pms_stay_id": "hf-98765",
                "membership_id": "M-000123",
                "property": "hf",
                "check_in": "2026-07-01",
                "check_out": "2026-07-03",
                "nights": 2
            })
        );
    }

    #[test]
    fn pms_stay_id_is_property_scoped_and_deterministic() {
        // The two per-site DBs have overlapping SERIAL cin_id sequences —
        // the property prefix is what keeps loyalty-side idempotency keys
        // distinct across sites.
        let hf = build_stay_payload("hf", 42, "M-1", d("2026-07-01"), d("2026-07-02"));
        let ville = build_stay_payload("hfville", 42, "M-1", d("2026-07-01"), d("2026-07-02"));
        assert_eq!(hf.pms_stay_id, "hf-42");
        assert_eq!(ville.pms_stay_id, "hfville-42");
        assert_ne!(hf.pms_stay_id, ville.pms_stay_id);
        // Determinism: same inputs → same id (safe to retry).
        assert_eq!(
            build_stay_payload("hf", 42, "M-1", d("2026-07-01"), d("2026-07-02")).pms_stay_id,
            hf.pms_stay_id
        );
    }

    #[test]
    fn nights_floor_at_one_for_same_day_checkout() {
        let p = build_stay_payload("hf", 1, "M-1", d("2026-07-01"), d("2026-07-01"));
        assert_eq!(p.nights, 1);
    }

    // ----- snapshot gate matrix -----

    fn snapshot(status: &str, membership: Option<&str>) -> StaySnapshot {
        StaySnapshot {
            cin_status: status.to_string(),
            membership_id: membership.map(str::to_string),
            check_in_time: dt("2026-07-01 14:00:00"),
            check_out_time: Some(dt("2026-07-03 11:30:00")),
            expected_checkout: d("2026-07-03"),
        }
    }

    #[test]
    fn completed_stay_with_membership_sends() {
        let p = stay_payload_from_snapshot("hf", 7, &snapshot("checkedout", Some("M-9"))).unwrap();
        assert_eq!(p.pms_stay_id, "hf-7");
        assert_eq!(p.check_in, "2026-07-01");
        assert_eq!(p.check_out, "2026-07-03");
        assert_eq!(p.nights, 2);
    }

    #[test]
    fn partial_checkout_header_still_active_sends_nothing() {
        // Per-room checkout of a multi-room stay: header stays 'active'
        // until the last room leaves — no premature loyalty credit.
        assert!(stay_payload_from_snapshot("hf", 7, &snapshot("active", Some("M-9"))).is_none());
    }

    #[test]
    fn unlinked_guest_sends_nothing() {
        assert!(stay_payload_from_snapshot("hf", 7, &snapshot("checkedout", None)).is_none());
        assert!(stay_payload_from_snapshot("hf", 7, &snapshot("checkedout", Some("  "))).is_none());
    }

    #[test]
    fn missing_checkout_time_falls_back_to_expected_date() {
        let mut s = snapshot("checkedout", Some("M-9"));
        s.check_out_time = None;
        let p = stay_payload_from_snapshot("hf", 7, &s).unwrap();
        assert_eq!(p.check_out, "2026-07-03");
    }

    // ----- client construction is fail-closed -----

    #[test]
    fn client_requires_both_url_and_token() {
        let mut cfg = LoyaltyConfig {
            channel_enabled: false,
            channel_token: None,
            app_url: Some("http://loyalty.local:4000".to_string()),
            service_token: None,
        };
        assert!(LoyaltyClient::from_config(&cfg).is_none(), "token missing → off");
        cfg.service_token = Some("tok".to_string());
        let client = LoyaltyClient::from_config(&cfg).expect("both set → on");
        assert_eq!(client.stays_url(), "http://loyalty.local:4000/api/loyalty/stays");
        cfg.app_url = None;
        assert!(LoyaltyClient::from_config(&cfg).is_none(), "url missing → off");
    }

    #[test]
    fn stays_url_tolerates_trailing_slash() {
        let cfg = LoyaltyConfig {
            channel_enabled: false,
            channel_token: None,
            app_url: Some("http://loyalty.local:4000/".to_string()),
            service_token: Some("tok".to_string()),
        };
        let client = LoyaltyClient::from_config(&cfg).unwrap();
        assert_eq!(client.stays_url(), "http://loyalty.local:4000/api/loyalty/stays");
    }
}
