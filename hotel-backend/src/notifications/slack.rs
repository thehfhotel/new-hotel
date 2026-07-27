//! Slack notification utilities
//!
//! Sends messages to Slack via webhook with retry logic.
//! Includes Thai date formatting with Buddhist calendar.

use chrono::{Datelike, NaiveDateTime, Timelike, Utc};
use serde::Serialize;

use crate::config::SlackConfig;

/// Thai month names
const THAI_MONTHS: [&str; 12] = [
    "มกราคม",
    "กุมภาพันธ์",
    "มีนาคม",
    "เมษายน",
    "พฤษภาคม",
    "มิถุนายน",
    "กรกฎาคม",
    "สิงหาคม",
    "กันยายน",
    "ตุลาคม",
    "พฤศจิกายน",
    "ธันวาคม",
];

/// Slack message block
#[derive(Debug, Serialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackText>,
}

/// Slack text element
#[derive(Debug, Serialize)]
pub struct SlackText {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
}

/// Slack message
#[derive(Debug, Serialize)]
pub struct SlackMessage {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlock>>,
}

impl SlackMessage {
    /// Convenience constructor — text-only message, no blocks. Use for
    /// alerts and short notifications where Block Kit layout isn't needed.
    pub fn with_text(text: impl Into<String>) -> Self {
        Self { text: text.into(), blocks: None }
    }

    /// Convenience constructor — text-only message, prefixed with the
    /// site identifier so an on-call operator can tell which deployment
    /// fired the alert when both HF Hotel and HF Ville post to the same
    /// webhook (task #69). Format: `[site=<id>] <text>`. Kept short on
    /// purpose so the human-readable body still fits Slack's preview.
    pub fn with_site_text(site_id: &str, text: impl Into<String>) -> Self {
        Self {
            text: format_site_prefixed(site_id, &text.into()),
            blocks: None,
        }
    }
}

/// Prepend the site-id prefix to a Slack message body. Pulled out as a
/// pure helper so the prefix shape (`[site=<id>] ...`) is unit-testable
/// and consistent across every alert call site.
pub fn format_site_prefixed(site_id: &str, text: &str) -> String {
    format!("[site={site_id}] {text}")
}

/// Slack client for sending messages
///
/// Uses `ureq` (blocking HTTP) under the hood — Slack webhooks are not
/// performance-critical and `ureq` pulls in ~100 fewer crates than `reqwest`.
/// The blocking call is dispatched via `tokio::task::spawn_blocking` so the
/// async runtime is never blocked.
#[derive(Clone)]
pub struct SlackClient {
    config: SlackConfig,
    agent: ureq::Agent,
    /// Consecutive fully-failed sends (all retries exhausted). Reset to 0
    /// on the next success. See [`EV_SLACK_DELIVERY_FAILED`].
    consecutive_failures: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

/// Maximum number of retry attempts for a Slack webhook POST.
const SLACK_MAX_RETRIES: u32 = 3;

/// A message was DROPPED after exhausting every retry — i.e. an alert
/// nobody will ever see. Greppable because Slack is itself the alerting
/// channel, so a dead webhook cannot be announced via Slack; the only
/// signal available is a distinctive, counted log line.
pub const EV_SLACK_DELIVERY_FAILED: &str = "slack.delivery_failed";

/// Deliveries resumed after ≥1 consecutive total failure.
pub const EV_SLACK_DELIVERY_RECOVERED: &str = "slack.delivery_recovered";

/// How much of the dropped message to echo into the failure log. Enough
/// to identify WHICH alert was lost without dumping a whole payload.
const DROPPED_MESSAGE_PREVIEW_CHARS: usize = 160;

impl SlackClient {
    /// Create a new Slack client
    pub fn new(config: SlackConfig) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(10))
            .build();
        Self {
            config,
            agent,
            consecutive_failures: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// First line of a message, truncated on a char boundary — the alert's
    /// identity for the drop log. Thai text is common in these payloads, so
    /// slicing by byte index would panic.
    fn preview(text: &str) -> String {
        let first_line = text.lines().next().unwrap_or("").trim();
        if first_line.chars().count() <= DROPPED_MESSAGE_PREVIEW_CHARS {
            return first_line.to_string();
        }
        let truncated: String = first_line.chars().take(DROPPED_MESSAGE_PREVIEW_CHARS).collect();
        format!("{truncated}…")
    }

    /// Send a message to Slack via webhook.
    ///
    /// Retries up to 3 times with exponential backoff (1s, 2s, 4s).
    /// Returns `true` on success, `false` on failure — Slack errors are
    /// logged but never propagated, since notifications are best-effort.
    pub async fn send_message(&self, message: &SlackMessage) -> bool {
        if !self.config.enabled {
            tracing::debug!("[Slack] Notifications disabled");
            return false;
        }

        let webhook_url = match &self.config.webhook_url {
            Some(url) => url.clone(),
            None => {
                tracing::warn!("[Slack] SLACK_WEBHOOK_URL not configured");
                return false;
            }
        };

        // Serialize the payload once, before moving into the blocking task.
        let payload = match serde_json::to_value(message) {
            Ok(value) => value,
            Err(e) => {
                tracing::error!("[Slack] Failed to serialize message: {}", e);
                return false;
            }
        };

        for attempt in 1..=SLACK_MAX_RETRIES {
            let agent = self.agent.clone();
            let url = webhook_url.clone();
            let body = payload.clone();

            // Dispatch the blocking ureq call onto a blocking-task thread so
            // we don't stall the async runtime.
            let result = tokio::task::spawn_blocking(move || {
                agent.post(&url).send_json(body)
            })
            .await;

            match result {
                Ok(Ok(response)) => {
                    let status = response.status();
                    if (200..300).contains(&status) {
                        tracing::info!("[Slack] Message sent successfully");
                        // Announce recovery so a resolved outage is visible
                        // in logs — the failures below are only interpretable
                        // if you can also see when they stopped.
                        let prior = self
                            .consecutive_failures
                            .swap(0, std::sync::atomic::Ordering::Relaxed);
                        if prior > 0 {
                            tracing::warn!(
                                event_name = EV_SLACK_DELIVERY_RECOVERED,
                                dropped_while_down = prior,
                                "[Slack] Delivery recovered — {prior} message(s) were dropped \
                                 while the webhook was failing and are NOT retried"
                            );
                        }
                        return true;
                    }

                    let error_text = response.into_string().unwrap_or_default();
                    tracing::error!(
                        "[Slack] Attempt {}/{} failed: {} - {}",
                        attempt,
                        SLACK_MAX_RETRIES,
                        status,
                        error_text
                    );
                }
                Ok(Err(ureq::Error::Status(status, response))) => {
                    let error_text = response.into_string().unwrap_or_default();
                    tracing::error!(
                        "[Slack] Attempt {}/{} failed: {} - {}",
                        attempt,
                        SLACK_MAX_RETRIES,
                        status,
                        error_text
                    );
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        "[Slack] Attempt {}/{} failed: {}",
                        attempt,
                        SLACK_MAX_RETRIES,
                        e
                    );
                }
                Err(join_err) => {
                    tracing::error!(
                        "[Slack] Attempt {}/{} blocking task panicked: {}",
                        attempt,
                        SLACK_MAX_RETRIES,
                        join_err
                    );
                }
            }

            if attempt < SLACK_MAX_RETRIES {
                let delay = std::time::Duration::from_secs(2u64.pow(attempt - 1));
                tokio::time::sleep(delay).await;
            }
        }

        // Every retry exhausted: this alert is GONE. Nothing re-queues it.
        // Callers all discard the returned bool (by design — notifications
        // are best-effort and must never break the caller), so this line is
        // the only record that a page was lost, and the only way to tell a
        // dead webhook from a genuinely quiet system.
        let consecutive = self
            .consecutive_failures
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;
        tracing::error!(
            event_name = EV_SLACK_DELIVERY_FAILED,
            consecutive_failures = consecutive,
            dropped_alert = %Self::preview(&message.text),
            "[Slack] All retry attempts failed — message DROPPED, not retried"
        );
        false
    }
}

/// Format Thai date string from database datetime
/// Uses the value as-is because database stores local Thai time
pub fn format_thai_date_from_db(dt: NaiveDateTime) -> String {
    let day = dt.day();
    let month = THAI_MONTHS[dt.month0() as usize];
    let year = dt.year() + 543; // Buddhist Era

    format!("{} {} {}", day, month, year)
}

/// Format Thai date string for current time (uses Asia/Bangkok timezone)
pub fn format_thai_date_now() -> String {
    let now = Utc::now();
    // Add 7 hours for Bangkok timezone
    let bangkok = now + chrono::Duration::hours(7);

    let day = bangkok.day();
    let month = THAI_MONTHS[bangkok.month0() as usize];
    let year = bangkok.year() + 543; // Buddhist Era

    format!("{} {} {}", day, month, year)
}

/// Format time string (HH:MM) from database datetime
/// Uses the value as-is because database stores local Thai time
pub fn format_time_from_db(dt: NaiveDateTime) -> String {
    format!("{:02}:{:02}", dt.hour(), dt.minute())
}

/// Format time string (HH:MM) for current time (Bangkok timezone)
pub fn format_time_now() -> String {
    let now = Utc::now();
    // Add 7 hours for Bangkok timezone
    let bangkok = now + chrono::Duration::hours(7);

    format!("{:02}:{:02}", bangkok.hour(), bangkok.minute())
}

/// Build hourly report message for Slack
pub fn build_hourly_report_message(
    occupied_rooms: i32,
    total_rooms: i32,
    today_bookings: i32,
) -> SlackMessage {
    let occupancy_percent = if total_rooms > 0 {
        (occupied_rooms as f64 / total_rooms as f64 * 100.0).round() as i32
    } else {
        0
    };

    let text = format!(
        "รายงานประจำชั่วโมง - ห้องที่มีผู้เข้าพัก: {}/{} ({}%), การจองวันนี้: {} รายการ",
        occupied_rooms, total_rooms, occupancy_percent, today_bookings
    );

    let body_text = vec![
        "─────────────────────────".to_string(),
        format!("*วันที่:* {}", format_thai_date_now()),
        format!("*เวลา:* {}", format_time_now()),
        String::new(),
        format!(
            ":bed: *ห้องที่มีผู้เข้าพัก:* {}/{} ({}%)",
            occupied_rooms, total_rooms, occupancy_percent
        ),
        format!(":calendar: *การจองวันนี้:* {} รายการ", today_bookings),
    ]
    .join("\n");

    SlackMessage {
        text,
        blocks: Some(vec![
            SlackBlock {
                block_type: "header".to_string(),
                text: Some(SlackText {
                    text_type: "plain_text".to_string(),
                    text: ":hotel: รายงานประจำชั่วโมง".to_string(),
                    emoji: Some(true),
                }),
            },
            SlackBlock {
                block_type: "section".to_string(),
                text: Some(SlackText {
                    text_type: "mrkdwn".to_string(),
                    text: body_text,
                    emoji: None,
                }),
            },
        ]),
    }
}

/// Build check-in alert message for Slack
pub fn build_check_in_alert_message(
    guest_name: &str,
    room_number: &str,
    check_in_time: NaiveDateTime,
) -> SlackMessage {
    let text = format!("เช็คอินใหม่ - {} ห้อง {}", guest_name, room_number);

    let body_text = vec![
        String::new(),
        format!(":bust_in_silhouette: *ชื่อผู้เข้าพัก:* {}", guest_name),
        format!(":door: *ห้อง:* {}", room_number),
        format!(":clock3: *เวลา:* {}", format_time_from_db(check_in_time)),
    ]
    .join("\n");

    SlackMessage {
        text,
        blocks: Some(vec![
            SlackBlock {
                block_type: "header".to_string(),
                text: Some(SlackText {
                    text_type: "plain_text".to_string(),
                    text: ":key: เช็คอินใหม่!".to_string(),
                    emoji: Some(true),
                }),
            },
            SlackBlock {
                block_type: "section".to_string(),
                text: Some(SlackText {
                    text_type: "mrkdwn".to_string(),
                    text: body_text,
                    emoji: None,
                }),
            },
        ]),
    }
}

/// Build check-out alert message for Slack
pub fn build_check_out_alert_message(
    guest_name: &str,
    room_number: &str,
    check_out_time: NaiveDateTime,
) -> SlackMessage {
    let text = format!("เช็คเอาท์ - {} ห้อง {}", guest_name, room_number);

    let body_text = vec![
        "─────────────────────────".to_string(),
        format!(":bust_in_silhouette: *ชื่อผู้เข้าพัก:* {}", guest_name),
        format!(":door: *ห้อง:* {}", room_number),
        format!(":clock3: *เวลา:* {}", format_time_from_db(check_out_time)),
    ]
    .join("\n");

    SlackMessage {
        text,
        blocks: Some(vec![
            SlackBlock {
                block_type: "header".to_string(),
                text: Some(SlackText {
                    text_type: "plain_text".to_string(),
                    text: ":wave: เช็คเอาท์!".to_string(),
                    emoji: Some(true),
                }),
            },
            SlackBlock {
                block_type: "section".to_string(),
                text: Some(SlackText {
                    text_type: "mrkdwn".to_string(),
                    text: body_text,
                    emoji: None,
                }),
            },
        ]),
    }
}

/// Build new booking alert message for Slack
pub fn build_new_booking_alert_message(
    guest_name: &str,
    room_type: &str,
    check_in_date: NaiveDateTime,
    check_out_date: NaiveDateTime,
) -> SlackMessage {
    let text = format!("การจองใหม่ - {} ประเภทห้อง {}", guest_name, room_type);

    let body_text = vec![
        "─────────────────────────".to_string(),
        format!(":bust_in_silhouette: *ชื่อผู้จอง:* {}", guest_name),
        format!(":bed: *ประเภทห้อง:* {}", room_type),
        format!(
            ":airplane_arriving: *วันเข้าพัก:* {}",
            format_thai_date_from_db(check_in_date)
        ),
        format!(
            ":airplane_departure: *วันออก:* {}",
            format_thai_date_from_db(check_out_date)
        ),
    ]
    .join("\n");

    SlackMessage {
        text,
        blocks: Some(vec![
            SlackBlock {
                block_type: "header".to_string(),
                text: Some(SlackText {
                    text_type: "plain_text".to_string(),
                    text: ":calendar: การจองใหม่!".to_string(),
                    emoji: Some(true),
                }),
            },
            SlackBlock {
                block_type: "section".to_string(),
                text: Some(SlackText {
                    text_type: "mrkdwn".to_string(),
                    text: body_text,
                    emoji: None,
                }),
            },
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The drop log must name WHICH alert was lost — "all retries failed"
    /// alone tells an operator nothing about what they missed.
    #[test]
    fn preview_takes_the_first_line_only() {
        let msg = ":rotating_light: *Writeback EXHAUSTED retries*\nbody line\nmore body";
        assert_eq!(
            SlackClient::preview(msg),
            ":rotating_light: *Writeback EXHAUSTED retries*"
        );
    }

    /// These payloads routinely carry Thai guest names, so truncation MUST
    /// be char-wise — a byte-index slice would panic mid-codepoint.
    #[test]
    fn preview_truncates_multibyte_text_without_panicking() {
        let thai = "สุภาวดี เมียนเมือง ".repeat(40);
        let out = SlackClient::preview(&thai);
        assert!(out.chars().count() <= DROPPED_MESSAGE_PREVIEW_CHARS + 1);
        assert!(out.ends_with('…'));
    }

    #[test]
    fn preview_leaves_short_lines_untouched() {
        assert_eq!(SlackClient::preview("short"), "short");
        assert_eq!(SlackClient::preview("  padded  \nrest"), "padded");
    }

    /// Task #69 contract: the site prefix must show up at the START of
    /// the Slack message text so an operator sees `[site=hfvilel]` in
    /// the notification preview before any context.
    #[test]
    fn format_site_prefixed_prepends_marker() {
        let out = format_site_prefixed("hfville", "Reconcile drift exceeded");
        assert_eq!(out, "[site=hfville] Reconcile drift exceeded");
    }

    #[test]
    fn with_site_text_carries_prefix_into_message_text() {
        let msg = SlackMessage::with_site_text("hfhotel", "schema fingerprint mismatch");
        assert!(
            msg.text.starts_with("[site=hfhotel] "),
            "site prefix must lead the message text; got {:?}",
            msg.text
        );
        assert!(msg.text.contains("schema fingerprint mismatch"));
        assert!(msg.blocks.is_none());
    }
}
