//! `MappableRow` — testable abstraction over a single CT-projection row.
//!
//! `tiberius::Row` is the concrete row type the watcher hands to mappers,
//! but its constructors are private — a unit test can't synthesise one.
//! Wrapping the column-access surface in a small trait lets every mapper
//! be exercised against a `HashMap`-backed fixture in `#[cfg(test)]`
//! while production still binds to the real tiberius row.
//!
//! ## Why this trait, not `serde::Deserialize`?
//!
//! Rows in a CT JOIN have a denormalised, table-specific column set; we
//! reach for individual columns by name instead of deserialising into a
//! struct. Keeping the API column-oriented also matches how the existing
//! writeback / scheduler code reads from tiberius rows
//! (`row.get::<&str, _>("Cust_no")`).
//!
//! ## Lifetime contract
//!
//! `try_get_str` / `try_get_bytes` return borrows from the row to avoid
//! copying. Mappers translate those borrows into owned `String` / `Vec<u8>`
//! values when persisting to PG. This matches tiberius's own API and keeps
//! the test impl trivial.

use crate::sync::SyncError;

/// Column-access surface used by every CT mapper.
///
/// The production impl wraps `&tiberius::Row`; tests use `HashMapRow`.
/// The trait is small on purpose — every getter is `Result<Option<T>, _>`
/// because CT projections frequently surface NULLable columns, and a
/// missing column should fail loud rather than panic.
///
/// `Send + Sync` is required because mappers use `#[async_trait]` with
/// the default `Send` future bound — borrows of `&dyn MappableRow` cross
/// `.await` points inside `apply` so the trait object must be `Sync`.
pub trait MappableRow: Send + Sync {
    fn try_get_str(&self, col: &str) -> Result<Option<&str>, SyncError>;
    fn try_get_i32(&self, col: &str) -> Result<Option<i32>, SyncError>;
    fn try_get_i64(&self, col: &str) -> Result<Option<i64>, SyncError>;
    fn try_get_f64(&self, col: &str) -> Result<Option<f64>, SyncError>;
    fn try_get_datetime(
        &self,
        col: &str,
    ) -> Result<Option<chrono::NaiveDateTime>, SyncError>;
    fn try_get_bytes(&self, col: &str) -> Result<Option<&[u8]>, SyncError>;
}

/// Production impl — borrowed `tiberius::Row`. tiberius's `try_get` is
/// already `Result<Option<T>, _>`-shaped so this is mostly a one-liner per
/// type. The error variant maps onto `SyncError::Tiberius` to flow through
/// the watcher's existing error funnel.
impl MappableRow for tiberius::Row {
    fn try_get_str(&self, col: &str) -> Result<Option<&str>, SyncError> {
        Ok(tiberius::Row::try_get::<&str, _>(self, col)?)
    }

    fn try_get_i32(&self, col: &str) -> Result<Option<i32>, SyncError> {
        Ok(tiberius::Row::try_get::<i32, _>(self, col)?)
    }

    fn try_get_i64(&self, col: &str) -> Result<Option<i64>, SyncError> {
        Ok(tiberius::Row::try_get::<i64, _>(self, col)?)
    }

    fn try_get_f64(&self, col: &str) -> Result<Option<f64>, SyncError> {
        Ok(tiberius::Row::try_get::<f64, _>(self, col)?)
    }

    fn try_get_datetime(
        &self,
        col: &str,
    ) -> Result<Option<chrono::NaiveDateTime>, SyncError> {
        Ok(tiberius::Row::try_get::<chrono::NaiveDateTime, _>(self, col)?)
    }

    fn try_get_bytes(&self, col: &str) -> Result<Option<&[u8]>, SyncError> {
        Ok(tiberius::Row::try_get::<&[u8], _>(self, col)?)
    }
}

/// `HashMap<column_name, MockValue>`-backed row.
///
/// Originally introduced as a `#[cfg(test)]` fixture, this also serves
/// as the **production** boundary representation the watcher binary
/// (`bin/sync.rs`) materialises tiberius rows into. Keeping a single
/// concrete `MappableRow` impl means production and tests both flow
/// through the same code path; the cost is one extra `HashMap`
/// allocation per CT row, which is rounding error against the network
/// roundtrip the row arrived on.
///
/// The submodule is named `test_support` for historical consistency
/// with the early skeleton; it is *not* gated behind `#[cfg(test)]`.
pub mod test_support {
    use super::{MappableRow, SyncError};
    use std::collections::HashMap;

    /// Allowed cell values. Mirror the methods on [`MappableRow`].
    #[derive(Debug, Clone)]
    pub enum MockValue {
        Str(String),
        I32(i32),
        I64(i64),
        F64(f64),
        DateTime(chrono::NaiveDateTime),
        Bytes(Vec<u8>),
        /// Column is present but its value is SQL NULL.
        Null,
    }

    /// Test row backed by a `HashMap`. Lookups by missing column name
    /// return `Err(SyncError::Mapper)` — tests want to surface "you
    /// forgot to set the column" loudly rather than masking it as NULL.
    #[derive(Debug, Default, Clone)]
    pub struct HashMapRow {
        pub cells: HashMap<String, MockValue>,
        /// `static`-named so SyncError::Mapper { table } stays cheap.
        pub table: &'static str,
    }

    impl HashMapRow {
        pub fn new(table: &'static str) -> Self {
            Self {
                cells: HashMap::new(),
                table,
            }
        }

        /// Builder helper — chainable column inserter.
        pub fn with(mut self, col: &str, value: MockValue) -> Self {
            self.cells.insert(col.to_string(), value);
            self
        }
    }

    fn missing(table: &'static str, col: &str) -> SyncError {
        SyncError::Mapper {
            table,
            message: format!("column '{col}' not present in row"),
        }
    }

    fn type_mismatch(table: &'static str, col: &str, want: &str) -> SyncError {
        SyncError::Mapper {
            table,
            message: format!(
                "column '{col}' present but not of type {want}"
            ),
        }
    }

    impl MappableRow for HashMapRow {
        fn try_get_str(&self, col: &str) -> Result<Option<&str>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::Str(s)) => Ok(Some(s.as_str())),
                Some(_) => Err(type_mismatch(self.table, col, "str")),
            }
        }

        fn try_get_i32(&self, col: &str) -> Result<Option<i32>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::I32(n)) => Ok(Some(*n)),
                Some(_) => Err(type_mismatch(self.table, col, "i32")),
            }
        }

        fn try_get_i64(&self, col: &str) -> Result<Option<i64>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::I64(n)) => Ok(Some(*n)),
                Some(_) => Err(type_mismatch(self.table, col, "i64")),
            }
        }

        fn try_get_f64(&self, col: &str) -> Result<Option<f64>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::F64(n)) => Ok(Some(*n)),
                Some(_) => Err(type_mismatch(self.table, col, "f64")),
            }
        }

        fn try_get_datetime(
            &self,
            col: &str,
        ) -> Result<Option<chrono::NaiveDateTime>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::DateTime(d)) => Ok(Some(*d)),
                Some(_) => Err(type_mismatch(self.table, col, "datetime")),
            }
        }

        fn try_get_bytes(&self, col: &str) -> Result<Option<&[u8]>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::Bytes(b)) => Ok(Some(b.as_slice())),
                Some(_) => Err(type_mismatch(self.table, col, "bytes")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{HashMapRow, MockValue};
    use super::MappableRow;

    #[test]
    fn hashmap_row_returns_none_for_explicit_null() {
        let row = HashMapRow::new("HT_Test").with("c", MockValue::Null);
        assert!(row.try_get_str("c").unwrap().is_none());
    }

    #[test]
    fn hashmap_row_returns_value_for_str() {
        let row = HashMapRow::new("HT_Test").with("c", MockValue::Str("hi".into()));
        assert_eq!(row.try_get_str("c").unwrap(), Some("hi"));
    }

    #[test]
    fn hashmap_row_returns_value_for_i32() {
        let row = HashMapRow::new("HT_Test").with("c", MockValue::I32(42));
        assert_eq!(row.try_get_i32("c").unwrap(), Some(42));
    }

    #[test]
    fn hashmap_row_missing_column_is_loud() {
        let row = HashMapRow::new("HT_Test");
        let err = row.try_get_str("missing").expect_err("missing must error");
        assert!(err.to_string().contains("not present"));
    }

    #[test]
    fn hashmap_row_type_mismatch_is_loud() {
        let row = HashMapRow::new("HT_Test").with("c", MockValue::I32(1));
        let err = row.try_get_str("c").expect_err("wrong type must error");
        assert!(err.to_string().contains("not of type str"));
    }
}
