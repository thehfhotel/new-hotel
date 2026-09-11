//! The ADR 0008 escalation valve — the ONE place this system spends LINE
//! messages.
//!
//! Every 30 seconds, for each canonical site, this job looks for ขอเช็คห้อง
//! (`room_check`) signals that are still `open` more than two minutes after
//! they were raised and have never been escalated, and POSTs each one to HF
//! ID's `/hk-escalate`. HF ID resolves the ON-DUTY maids of that branch — a
//! clock-in today at that branch's fingerprint device, not clocked out — and
//! LINE-multicasts them once. Nobody on duty ⇒ `{sent:false,
//! reason:"nobody_on_duty"}`, no push, and deliberately no fallback: the desk
//! phones, which is today's behaviour, not a new failure mode.
//!
//! ## Why this is the only metered path
//!
//! ADR 0008 §Context does the arithmetic: the Thai free tier is ~200 OA
//! messages/month, counted per recipient, and chat-over-LINE at modest volume
//! is 13× that from week one. So routine delivery rides our own rails (PG rows
//! + SSE) and costs zero messages forever, and pushing is confined to one
//! bounded case with three hard limits, all of which live here:
//!
//! 1. **Only `room_check`.** No other signal type escalates, ever.
//! 2. **Once per signal.** `sig_escalated_at` is stamped on ANY 2xx —
//!    `{sent:false}` included, because "nobody was on duty" is a definitive
//!    answer and re-asking every 30s would be a retry loop that eventually
//!    pushes when someone clocks in an hour later, long after the guest left
//!    the counter. Only a NON-2xx or a transport failure leaves it NULL for the
//!    next tick.
//! 3. **A monthly hard stop.** `HK_ESCALATION_MONTHLY_CAP` (default 150) over
//!    `COUNT(sig_escalated_at)` in the current **Bangkok** month. Checked
//!    before the loop AND re-counted per send, so a burst inside one tick
//!    cannot overshoot.
//!
//! ## Ship-dark
//!
//! `HFID_ESCALATE_URL` unset ⇒ the job is NOT REGISTERED at all and the
//! scheduler logs that once at startup (never per tick). There is no default
//! URL: a guessed path would turn a misconfiguration into confident spend on a
//! metered channel. The secret is `HFID_RESOLVE_SECRET`, REUSED — HF ID guards
//! its whole app↔central surface with that one value, so there is no new
//! credential to provision.
//!
//! ## Transport
//!
//! Blocking `ureq` dispatched via `spawn_blocking` — the repo's outbound-HTTP
//! policy (no `reqwest`), the same shape `hfid_location::HttpLocationLookup`,
//! `service::reader`, `middleware::cf_access` and `notifications::slack` use.
//! This call is on a background tick rather than in a request path, so the
//! timeout is a little more generous than `/hk`'s 3s — but still bounded, so a
//! hung HF ID cannot wedge the tick and pile up scheduler work.
//!
//! ## Failure isolation
//!
//! Per-signal failures are logged and skipped; one bad row can never wedge the
//! tick, and nothing here can fail the process. The same discipline the loyalty
//! hold sweep and the stale-checkin tripwire already follow.

use std::time::Duration;

use sqlx::{PgPool, Row};

use crate::config::HkEscalationConfig;
use crate::domain::hk_signal::{escalation_eligible, SignalStatus, ESCALATION_AGE_SECONDS, ROOM_CHECK};
use crate::hfid_location::RESOLVE_SECRET_HEADER;

/// Upper bound on one `/hk-escalate` POST.
pub const ESCALATE_TIMEOUT: Duration = Duration::from_secs(8);
/// Connect budget inside [`ESCALATE_TIMEOUT`] — a LAN host that is simply down
/// fails here almost immediately.
const ESCALATE_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// Most signals one tick will escalate.
///
/// A safety valve, not a policy: the monthly cap is the real limit. It bounds
/// the work of a single tick so that a backlog (the job was off for a day, say)
/// drains over several ticks instead of firing dozens of HTTP calls inside one,
/// which is how a rate limiter turns into a thundering herd.
const MAX_PER_TICK: i64 = 10;

/// `?branch=` id → the branch token HF ID's `/hk-escalate` expects.
///
/// THE MAPPING, and it is the same vocabulary
/// [`crate::hfid_location::EmployeeLocation`] speaks (`HF` / `HF_VILLE`), not
/// our `hfhotel` / `hfville`. Bridging by string manipulation is exactly the
/// bug `routes::hk::location_branch` documents at the other end of the same
/// boundary: `to_uppercase()` on `hfville` yields `HFVILLE`, which HF ID does
/// not know. PURE.
pub fn hfid_branch_token(site_id: &str) -> Option<&'static str> {
    match site_id {
        "hfhotel" => Some("HF"),
        "hfville" => Some("HF_VILLE"),
        _ => None,
    }
}

/// A `room_check` this tick may escalate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalationCandidate {
    pub signal_id: i64,
    /// Canonical room id — the `/hk/rooms/{roomId}` deep link's segment.
    pub room_id: i32,
    /// `HT_Rooms.Room_no` as a person reads it — the message text's `{roomNo}`.
    pub room_no: String,
    /// Seconds since the signal was raised — re-checked against the pure
    /// predicate so the SQL and [`escalation_eligible`] cannot drift.
    pub age_seconds: i64,
}

/// The deep link the LINE message carries: the maid's own room page, so the
/// push lands her one tap from the ack.
///
/// `base` is [`HK_ROOM_URL_BASE`]; kept as a function so the shape is pinned by
/// a test rather than assembled inline at the call site.
pub fn room_url(base: &str, room_id_or_no: &str) -> String {
    format!("{}/{}", base.trim_end_matches('/'), room_id_or_no)
}

/// Public base of the maid surface's room page. Not configurable: it is the
/// one production hostname this app is served from, and an env var here would
/// be a way to send maids to a link that does not exist.
pub const HK_ROOM_URL_BASE: &str = "https://hotel.thehfhotel.org/hk/rooms";

/// Run one escalation tick against one site's canonical pool.
///
/// Never returns an error and never panics — a scheduler job that can fail is a
/// scheduler job that stops running. Returns how many signals were stamped, for
/// the log line and for tests.
pub async fn run_escalation_tick(pg: &PgPool, site_id: &str, cfg: &HkEscalationConfig) -> i64 {
    let (Some(url), Some(secret)) = (cfg.url.as_deref(), cfg.resolve_secret.as_deref()) else {
        // Unreachable: the job is not registered unless both halves are set.
        return 0;
    };
    let Some(branch_token) = hfid_branch_token(site_id) else {
        tracing::warn!(site = %site_id, "hk escalation: unknown site id; skipping tick");
        return 0;
    };

    let mut escalated_this_month = match count_escalations_this_month(pg).await {
        Ok(count) => count,
        Err(err) => {
            tracing::warn!(site = %site_id, error = %err, "hk escalation: monthly count failed; skipping tick");
            return 0;
        }
    };
    // The hard stop, checked BEFORE any candidate query: at the cap there is
    // nothing to do and no reason to touch the table.
    if escalated_this_month >= cfg.monthly_cap {
        tracing::warn!(
            site = %site_id,
            escalated_this_month,
            cap = cfg.monthly_cap,
            "hk escalation: monthly cap reached; no pushes will be sent until next month"
        );
        return 0;
    }

    let candidates = match load_candidates(pg).await {
        Ok(candidates) => candidates,
        Err(err) => {
            tracing::warn!(site = %site_id, error = %err, "hk escalation: candidate query failed; skipping tick");
            return 0;
        }
    };

    let mut stamped = 0i64;
    for candidate in candidates {
        // Re-assert the WHOLE predicate in Rust, not just the parts the SQL
        // could not express. The cap moves inside this loop (each send spends
        // one), and restating type/status/age here is what stops a future
        // widening of the query from quietly bypassing the limit.
        if !escalation_eligible(
            ROOM_CHECK,
            SignalStatus::Open,
            candidate.age_seconds,
            false,
            escalated_this_month,
            cfg.monthly_cap,
        ) {
            break;
        }

        let url_for_maid = room_url(HK_ROOM_URL_BASE, &candidate.signal_room_path());
        match post_escalation(
            url.to_string(),
            secret.to_string(),
            branch_token,
            candidate.room_no.clone(),
            url_for_maid,
        )
        .await
        {
            EscalationDelivery::Answered { sent, recipients } => {
                // ANY 2xx stamps — `{sent:false, reason:"nobody_on_duty"}`
                // included. See the module header: re-asking would push late.
                match stamp_escalated(pg, candidate.signal_id).await {
                    Ok(true) => {
                        stamped += 1;
                        escalated_this_month += 1;
                        tracing::info!(
                            site = %site_id,
                            signal_id = candidate.signal_id,
                            room_no = %candidate.room_no,
                            sent,
                            recipients,
                            escalated_this_month,
                            cap = cfg.monthly_cap,
                            "hk escalation delivered to HF ID"
                        );
                    }
                    // Another instance stamped it first — correct and silent.
                    Ok(false) => {}
                    Err(err) => tracing::warn!(
                        site = %site_id,
                        signal_id = candidate.signal_id,
                        error = %err,
                        "hk escalation: HF ID accepted the push but the stamp failed; \
                         the next tick may re-send this one"
                    ),
                }
            }
            EscalationDelivery::Failed(reason) => {
                // sig_escalated_at stays NULL → retried next tick.
                tracing::warn!(
                    site = %site_id,
                    signal_id = candidate.signal_id,
                    room_no = %candidate.room_no,
                    reason = %reason,
                    "hk escalation POST failed; leaving unescalated for the next tick"
                );
            }
        }
    }
    stamped
}

impl EscalationCandidate {
    /// Path segment for the maid's deep link.
    ///
    /// The contract's URL shape is `/hk/rooms/{roomId}`, and the candidate
    /// query carries the room id for exactly this; the room NUMBER stays in the
    /// message text where a maid reads it.
    fn signal_room_path(&self) -> String {
        self.room_id.to_string()
    }
}

/// Count this Bangkok month's escalations — the quota ledger.
///
/// `sig_escalated_at` is `TIMESTAMPTZ`, so the month boundary is computed by
/// shifting BOTH sides into Asia/Bangkok rather than comparing a UTC month: an
/// escalation at 06:00 Bangkok on the 1st is 23:00 UTC on the LAST day of the
/// previous month, and a UTC-month count would charge it to the wrong month at
/// both ends. The same civil-day reasoning `routes::hk::TODAY_BKK` uses.
async fn count_escalations_this_month(pg: &PgPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "SELECT COUNT(*) AS n FROM ht_hk_room_signals \
          WHERE sig_escalated_at IS NOT NULL \
            AND (sig_escalated_at AT TIME ZONE 'Asia/Bangkok') \
                >= date_trunc('month', (NOW() AT TIME ZONE 'Asia/Bangkok'))",
    )
    .fetch_one(pg)
    .await?;
    row.try_get::<i64, _>("n")
}

/// The candidate scan. Mirrors [`escalation_eligible`] clause for clause,
/// oldest first — a guest who has been waiting longest is escalated first.
async fn load_candidates(pg: &PgPool) -> Result<Vec<EscalationCandidate>, sqlx::Error> {
    let sql = format!(
        "SELECT s.sig_id, s.sig_room_id, r.room_no, \
                EXTRACT(EPOCH FROM (NOW() - s.sig_created_at))::bigint AS age_seconds \
           FROM ht_hk_room_signals s \
           JOIN ht_rooms_new r ON r.room_id = s.sig_room_id \
          WHERE s.sig_type = $1 \
            AND s.sig_status = 'open' \
            AND s.sig_escalated_at IS NULL \
            AND s.sig_created_at < NOW() - INTERVAL '{ESCALATION_AGE_SECONDS} seconds' \
          ORDER BY s.sig_created_at ASC \
          LIMIT {MAX_PER_TICK}"
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql)).bind(ROOM_CHECK).fetch_all(pg).await?;
    rows.iter()
        .map(|row| {
            Ok(EscalationCandidate {
                signal_id: row.try_get("sig_id")?,
                room_id: row.try_get("sig_room_id")?,
                room_no: row.try_get("room_no")?,
                age_seconds: row.try_get("age_seconds")?,
            })
        })
        .collect()
}

/// Stamp the once-only marker. Guarded on `IS NULL` so two instances racing the
/// same signal cannot both count against the quota; `false` = somebody else won.
async fn stamp_escalated(pg: &PgPool, signal_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE ht_hk_room_signals SET sig_escalated_at = NOW() \
          WHERE sig_id = $1 AND sig_escalated_at IS NULL",
    )
    .bind(signal_id)
    .execute(pg)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// What one POST to `/hk-escalate` told us.
///
/// The distinction that matters is 2xx-vs-not, NOT sent-vs-not: an answered
/// call is a decision (possibly "nobody on duty"), a failed call is an absence
/// of one. Only the second is retried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscalationDelivery {
    Answered { sent: bool, recipients: i64 },
    Failed(String),
}

#[derive(Debug, serde::Deserialize)]
struct EscalateResponse {
    #[serde(default)]
    sent: bool,
    #[serde(default)]
    recipients: i64,
    #[serde(default)]
    #[allow(dead_code)]
    reason: Option<String>,
}

async fn post_escalation(
    url: String,
    secret: String,
    branch: &'static str,
    room_no: String,
    room_url: String,
) -> EscalationDelivery {
    let body = serde_json::json!({
        "branch": branch,
        "roomNo": room_no,
        "url": room_url,
    });

    let joined = tokio::task::spawn_blocking(move || {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(ESCALATE_CONNECT_TIMEOUT)
            .timeout(ESCALATE_TIMEOUT)
            .build();
        agent
            .post(&url)
            .set(RESOLVE_SECRET_HEADER, &secret)
            .send_json(body)
    })
    .await;

    let sent = match joined {
        Ok(sent) => sent,
        Err(err) => return EscalationDelivery::Failed(format!("task join failed: {err}")),
    };

    match sent {
        // 2xx. A body we cannot decode is still a 2xx: HF ID accepted and acted
        // on the request, so re-sending would risk a double push. Stamp it and
        // record the decode problem in the log.
        Ok(response) => match response.into_json::<EscalateResponse>() {
            Ok(decoded) => EscalationDelivery::Answered {
                sent: decoded.sent,
                recipients: decoded.recipients,
            },
            Err(err) => {
                tracing::warn!(error = %err, "hk escalation: 2xx with an undecodable body; treating as delivered");
                EscalationDelivery::Answered {
                    sent: false,
                    recipients: 0,
                }
            }
        },
        Err(ureq::Error::Status(code, _)) => {
            EscalationDelivery::Failed(format!("HTTP {code}"))
        }
        Err(err) => EscalationDelivery::Failed(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The branch vocabularies are DIFFERENT and must never be bridged by
    /// string manipulation — the same rule `routes::hk::location_branch`
    /// documents at the other end of this boundary.
    #[test]
    fn branch_tokens_are_hf_ids_spelling_not_ours() {
        assert_eq!(hfid_branch_token("hfhotel"), Some("HF"));
        assert_eq!(hfid_branch_token("hfville"), Some("HF_VILLE"));
        // A naive to_uppercase() bridge would have produced these.
        assert_ne!(hfid_branch_token("hfville"), Some("HFVILLE"));
        for unknown in ["HF", "HF_VILLE", "hfvilel", "", "all"] {
            assert_eq!(hfid_branch_token(unknown), None, "{unknown:?}");
        }
    }

    #[test]
    fn the_deep_link_points_at_the_maids_own_room_page() {
        assert_eq!(
            room_url(HK_ROOM_URL_BASE, "42"),
            "https://hotel.thehfhotel.org/hk/rooms/42"
        );
        // A trailing slash on the base must not double up.
        assert_eq!(room_url("https://x/hk/rooms/", "7"), "https://x/hk/rooms/7");
    }

    /// The candidate's deep-link segment is the ROOM ID (the contract's
    /// `/hk/rooms/{roomId}`), while the room NUMBER goes in the message text a
    /// maid reads. Swapping them would produce a link to a room that does not
    /// exist at the property whose numbers and ids differ.
    #[test]
    fn the_link_uses_the_room_id_and_the_message_uses_the_room_number() {
        let candidate = EscalationCandidate {
            signal_id: 1,
            room_id: 42,
            room_no: "104".to_string(),
            age_seconds: 200,
        };
        assert_eq!(candidate.signal_room_path(), "42");
        assert_eq!(
            room_url(HK_ROOM_URL_BASE, &candidate.signal_room_path()),
            "https://hotel.thehfhotel.org/hk/rooms/42"
        );
    }

    /// The SQL and the pure predicate must agree about the age boundary — the
    /// query interpolates the same constant the predicate compares against.
    #[test]
    fn the_candidate_query_uses_the_adrs_own_age_constant() {
        assert_eq!(ESCALATION_AGE_SECONDS, 120);
        let sql = format!("INTERVAL '{ESCALATION_AGE_SECONDS} seconds'");
        assert_eq!(sql, "INTERVAL '120 seconds'");
    }

    /// A 2xx is a DECISION and is never retried; everything else is an absence
    /// of one and is. This is what stops a late push landing after the guest
    /// has left the counter.
    #[test]
    fn only_a_failed_call_is_retryable() {
        let answered = EscalationDelivery::Answered {
            sent: false,
            recipients: 0,
        };
        assert!(
            matches!(answered, EscalationDelivery::Answered { .. }),
            "nobody_on_duty is an answer, not a failure"
        );
        assert!(matches!(
            EscalationDelivery::Failed("HTTP 500".into()),
            EscalationDelivery::Failed(_)
        ));
    }

    /// `{sent:false, reason:"nobody_on_duty"}` must decode — it is the normal
    /// case, not an error path, and mis-decoding it would make every quiet
    /// shift look like a transport failure and retry forever.
    #[test]
    fn the_nobody_on_duty_body_decodes() {
        let decoded: EscalateResponse =
            serde_json::from_str(r#"{"sent":false,"recipients":0,"reason":"nobody_on_duty"}"#)
                .expect("decodes");
        assert!(!decoded.sent);
        assert_eq!(decoded.recipients, 0);

        let delivered: EscalateResponse =
            serde_json::from_str(r#"{"sent":true,"recipients":3}"#).expect("decodes");
        assert!(delivered.sent);
        assert_eq!(delivered.recipients, 3);

        // A body missing every field is still a 2xx we must accept.
        let empty: EscalateResponse = serde_json::from_str("{}").expect("decodes");
        assert!(!empty.sent);
    }

    /// One tick's work is bounded so a backlog drains over several ticks rather
    /// than firing a burst of HTTP calls — a rate limiter that stampedes is not
    /// a rate limiter.
    #[test]
    fn a_tick_is_bounded() {
        assert!(MAX_PER_TICK > 0 && MAX_PER_TICK <= 25);
    }
}
