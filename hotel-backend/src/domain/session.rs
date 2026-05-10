//! Session aggregate — Phase 4 PR1 (authentication foundation).
//!
//! Pure types only — no I/O. Mirrors the `ht_sessions` table created by
//! migration 028. The session token (`id`) IS the HttpOnly cookie value
//! the browser sends back; we do NOT hash it before storage because the
//! cookie itself is the bearer credential (an attacker who already
//! possesses it has a valid session regardless).

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// One row in `ht_sessions`.
///
/// `id` is a 32-byte random token, hex-encoded (= 64 ASCII chars) by
/// [`service::auth::AuthService::login`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl Session {
    /// Returns true when this session's `expires_at` is at or before
    /// `now`. The repository's `get_active` filter applies the same
    /// predicate at query time so callers usually do not need this —
    /// it exists for in-memory checks (tests, mock repos, future
    /// sliding-window logic).
    pub fn is_expired(&self, now: NaiveDateTime) -> bool {
        self.expires_at <= now
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveDate};

    fn fixed_session(expires_at: NaiveDateTime) -> Session {
        Session {
            id: "deadbeef".repeat(8),
            user_id: 42,
            created_at: NaiveDate::from_ymd_opt(2026, 5, 10)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            expires_at,
            ip: None,
            user_agent: None,
        }
    }

    #[test]
    fn is_expired_true_when_now_at_or_after_expires_at() {
        let now = NaiveDate::from_ymd_opt(2026, 5, 11)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let session = fixed_session(now);
        assert!(session.is_expired(now));
        assert!(session.is_expired(now + Duration::seconds(1)));
    }

    #[test]
    fn is_expired_false_when_now_before_expires_at() {
        let expires = NaiveDate::from_ymd_opt(2026, 5, 11)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let session = fixed_session(expires);
        let now = expires - Duration::seconds(1);
        assert!(!session.is_expired(now));
    }
}
