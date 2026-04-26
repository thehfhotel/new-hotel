//! `ChangeOp` — typed wrapper around CT's `SYS_CHANGE_OPERATION`.
//!
//! `CHANGETABLE(CHANGES …)` exposes one of three single-character codes
//! per change row:
//!
//! | Code | Meaning |
//! |------|---------|
//! | `'I'` | INSERT — the row is new since the supplied watermark |
//! | `'U'` | UPDATE — the row existed before; one or more columns changed |
//! | `'D'` | DELETE — the row has been removed (no current data joinable) |
//!
//! Mappers branch on this enum to choose between UPSERT (I/U) and a
//! soft-delete or remove (D). Wrapping it in a typed enum keeps callers
//! away from raw `&str` matches that miss new variants silently.

use serde::{Deserialize, Serialize};

/// One CT change-operation code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeOp {
    Insert,
    Update,
    Delete,
}

impl ChangeOp {
    /// Stable single-character literal matching the CT column.
    pub fn as_char(&self) -> char {
        match self {
            ChangeOp::Insert => 'I',
            ChangeOp::Update => 'U',
            ChangeOp::Delete => 'D',
        }
    }
}

impl TryFrom<&str> for ChangeOp {
    type Error = UnknownChangeOp;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "I" => Ok(ChangeOp::Insert),
            "U" => Ok(ChangeOp::Update),
            "D" => Ok(ChangeOp::Delete),
            other => Err(UnknownChangeOp(other.to_string())),
        }
    }
}

impl TryFrom<char> for ChangeOp {
    type Error = UnknownChangeOp;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(ChangeOp::Insert),
            'U' => Ok(ChangeOp::Update),
            'D' => Ok(ChangeOp::Delete),
            other => Err(UnknownChangeOp(other.to_string())),
        }
    }
}

/// Returned when an unexpected `SYS_CHANGE_OPERATION` value is seen — a
/// signal that either Microsoft added a new code or the column was
/// corrupted. Either case warrants a loud failure rather than a silent
/// skip.
#[derive(Debug, thiserror::Error)]
#[error("unknown SYS_CHANGE_OPERATION value: {0:?} (expected 'I' | 'U' | 'D')")]
pub struct UnknownChangeOp(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_recognises_insert() {
        assert_eq!(ChangeOp::try_from("I").unwrap(), ChangeOp::Insert);
    }

    #[test]
    fn try_from_str_recognises_update() {
        assert_eq!(ChangeOp::try_from("U").unwrap(), ChangeOp::Update);
    }

    #[test]
    fn try_from_str_recognises_delete() {
        assert_eq!(ChangeOp::try_from("D").unwrap(), ChangeOp::Delete);
    }

    #[test]
    fn try_from_str_rejects_unknown() {
        let err = ChangeOp::try_from("X").expect_err("X is not a valid op");
        assert!(err.to_string().contains("X"));
    }

    #[test]
    fn try_from_char_round_trips() {
        for op in [ChangeOp::Insert, ChangeOp::Update, ChangeOp::Delete] {
            assert_eq!(ChangeOp::try_from(op.as_char()).unwrap(), op);
        }
    }

    #[test]
    fn as_char_matches_ct_codes() {
        assert_eq!(ChangeOp::Insert.as_char(), 'I');
        assert_eq!(ChangeOp::Update.as_char(), 'U');
        assert_eq!(ChangeOp::Delete.as_char(), 'D');
    }
}
