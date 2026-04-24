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
}

/// Maximum number of retry attempts for a Slack webhook POST.
const SLACK_MAX_RETRIES: u32 = 3;

impl SlackClient {
    /// Create a new Slack client
    pub fn new(config: SlackConfig) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(10))
            .build();
        Self { config, agent }
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

        tracing::error!("[Slack] All retry attempts failed");
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
