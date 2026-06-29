//! API route handlers

use crate::domain::user::User;

/// Resolve the write actor for an audit / `*_by` field.
///
/// Prefers the authenticated session user's username — injected as
/// `Extension<User>` by [`crate::middleware::auth::require_auth`] when
/// `AUTH_ENABLED` is on — and falls back to the request-body value
/// (trimmed; blank treated as absent). Returns `None` when neither is
/// present so each call site can apply its own sentinel (empty string,
/// `"Front Desk"`, `"Admin"`, …).
///
/// Auth ships DARK today (`AUTH_ENABLED=false`): the middleware never
/// inserts a `User`, so `actor` is always `None` and the body value is
/// used unchanged — behavior is identical to before this helper existed.
/// Once auth is flipped on, the session username wins and we stop
/// trusting client-supplied actor strings for audit fields.
///
/// Handlers read the session user with `Option<Extension<User>>` (the
/// extractor resolves to `None` when auth is off, so the handler never
/// hard-requires a session) and pass `actor.as_deref()` here.
pub fn resolve_actor(actor: Option<&User>, body: Option<&str>) -> Option<String> {
    actor
        .map(|u| u.username.clone())
        .or_else(|| {
            body.map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
}

#[cfg(test)]
mod actor_tests {
    use super::resolve_actor;
    use crate::domain::user::{Role, User};
    use chrono::NaiveDate;

    fn sample_user(username: &str) -> User {
        User {
            user_id: 1,
            username: username.to_string(),
            password_hash: String::new(),
            role: Role::Receptionist,
            active: true,
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            last_login_at: None,
        }
    }

    #[test]
    fn prefers_authenticated_user_over_body() {
        let u = sample_user("alice");
        assert_eq!(
            resolve_actor(Some(&u), Some("body_value")).as_deref(),
            Some("alice")
        );
    }

    #[test]
    fn falls_back_to_trimmed_body_when_no_auth() {
        assert_eq!(resolve_actor(None, Some("  Nok  ")).as_deref(), Some("Nok"));
    }

    #[test]
    fn none_when_both_absent_or_blank() {
        assert_eq!(resolve_actor(None, None), None);
        assert_eq!(resolve_actor(None, Some("   ")), None);
    }
}

pub mod admin_users;
pub mod auth;
pub mod bookings;
pub mod calendar;
pub mod checkins;
pub mod customers;
pub mod events;
pub mod health;
pub mod housekeeping;
pub mod legacy_mirror;
pub mod mode;
pub mod new_bookings;
pub mod new_calendar;
pub mod new_cash;
pub mod new_checkins;
pub mod new_coupons;
pub mod new_customers;
pub mod new_inventory;
pub mod new_invoice;
pub mod new_maintenance;
pub mod new_notes;
pub mod new_payments;
pub mod new_pos;
pub mod new_products;
pub mod new_rates;
pub mod new_reports;
pub mod new_room_types;
pub mod new_rooms;
pub mod new_rosters;
pub mod new_feedback;
pub mod new_shifts;
pub mod new_sync;
pub mod new_verification;
pub mod rr4_export;
pub mod stats;
