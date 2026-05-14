//! User aggregate — Phase 4 PR1 (authentication foundation).
//!
//! Pure types only — no I/O, no SQL. Mirrors the `ht_users` table created
//! by migration 027. The repository layer (`repository::user`) handles
//! persistence; the service layer (`service::auth`) handles password
//! hashing + session minting.
//!
//! `password_hash` carries the Argon2id PHC string — never the plaintext.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A local-credentials user.
///
/// `password_hash` is the Argon2id PHC string returned by
/// [`service::auth::AuthService::hash_password`]. We carry it on the
/// in-memory struct because the repository's `get_by_username` needs to
/// hand it to `verify_password` — but the HTTP serialization for `User`
/// (Phase 4 PR2) MUST NOT expose it. Use a dedicated wire DTO at the
/// route layer rather than leaning on `Serialize` here directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub last_login_at: Option<NaiveDateTime>,
}

/// Role assigned to a [`User`].
///
/// The schema stores the lowercase string (`'admin'` / `'receptionist'`)
/// enforced by a CHECK constraint in migration 027. We mirror that on
/// the wire via `serde(rename_all = "lowercase")` so JSON payloads round
/// trip cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Receptionist,
    Cashier,
    Housekeeper,
}

impl Role {
    /// Stable lowercase label used by the DB CHECK constraint and the
    /// JSON wire format.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Receptionist => "receptionist",
            Role::Cashier => "cashier",
            Role::Housekeeper => "housekeeper",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when a string cannot be parsed into a [`Role`].
///
/// Carries the offending input so callers (CLI, repository deserializer,
/// HTTP DTO layer) can surface it in their own error messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRoleError(pub String);

impl std::fmt::Display for ParseRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid role '{}' (expected 'admin', 'receptionist', 'cashier', or 'housekeeper')",
            self.0
        )
    }
}

impl std::error::Error for ParseRoleError {}

impl TryFrom<&str> for Role {
    type Error = ParseRoleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(Role::Admin),
            "receptionist" => Ok(Role::Receptionist),
            "cashier" => Ok(Role::Cashier),
            "housekeeper" => Ok(Role::Housekeeper),
            other => Err(ParseRoleError(other.to_string())),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = ParseRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Role::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_round_trips_via_as_str_and_try_from() {
        assert_eq!(Role::try_from("admin"), Ok(Role::Admin));
        assert_eq!(Role::try_from("receptionist"), Ok(Role::Receptionist));
        assert_eq!(Role::try_from("cashier"), Ok(Role::Cashier));
        assert_eq!(Role::try_from("housekeeper"), Ok(Role::Housekeeper));
        assert_eq!(Role::Admin.as_str(), "admin");
        assert_eq!(Role::Receptionist.as_str(), "receptionist");
        assert_eq!(Role::Cashier.as_str(), "cashier");
        assert_eq!(Role::Housekeeper.as_str(), "housekeeper");
    }

    #[test]
    fn role_try_from_rejects_unknown_value() {
        let err = Role::try_from("guest").unwrap_err();
        assert_eq!(err.0, "guest");
        assert!(err.to_string().contains("guest"));
    }

    #[test]
    fn role_serializes_as_lowercase_string() {
        let json = serde_json::to_string(&Role::Admin).unwrap();
        assert_eq!(json, "\"admin\"");
        let parsed: Role = serde_json::from_str("\"receptionist\"").unwrap();
        assert_eq!(parsed, Role::Receptionist);
    }
}
