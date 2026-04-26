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
    /// Read a SQL Server NUMERIC / DECIMAL / FLOAT column.
    ///
    /// Returns `f64` rather than a true big-decimal type because:
    ///
    /// 1. Every monetary column we mirror from MSSQL (`HT_Book_H.Book_Price_Total`,
    ///    `Book_Price_Pay`, `Book_Room_Price`, …) is declared `float` in the
    ///    legacy schema (verified via `legacy-reference/analysis/_SCHEMA.sql`),
    ///    not `numeric` / `decimal`. tiberius surfaces `float` as `f64` already.
    /// 2. Our canonical PG `DECIMAL(12,2)` columns are bound via `$N::float8`
    ///    casts (see `repository/booking.rs::insert_booking`), so a round-trip
    ///    through `f64` is the existing, validated boundary representation.
    /// 3. The writeback adapter (`writeback/format.rs::f64_sql`,
    ///    `money_2dp`) renders all baht amounts from `f64`, so adding a true
    ///    `rust_decimal::Decimal` arm here would mean translating back to
    ///    `f64` at every consumer.
    ///
    /// If a future legacy column lands as a true `decimal`/`numeric` type,
    /// extend the trait with a `try_get_big_decimal` arm rather than
    /// silently truncating through `f64`.
    fn try_get_decimal(&self, col: &str) -> Result<Option<f64>, SyncError>;
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

    fn try_get_decimal(&self, col: &str) -> Result<Option<f64>, SyncError> {
        // SQL Server `float` and `real` arrive as `f64`/`f32` respectively.
        // Try `f64` first (the common case) and fall back to `f32` so a
        // schema where someone declared `Book_Price_*` as `real` still works.
        if let Ok(v) = tiberius::Row::try_get::<f64, _>(self, col) {
            return Ok(v);
        }
        match tiberius::Row::try_get::<f32, _>(self, col)? {
            Some(v) => Ok(Some(v as f64)),
            None => Ok(None),
        }
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
        /// SQL Server `numeric` / `decimal` / `float` column. Stored as
        /// `f64` for the same reasons documented on
        /// [`MappableRow::try_get_decimal`].
        Decimal(f64),
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
                // Decimals are also valid f64 sources — keeps tests that
                // mix `Decimal` and `F64` ergonomic.
                Some(MockValue::Decimal(n)) => Ok(Some(*n)),
                Some(_) => Err(type_mismatch(self.table, col, "f64")),
            }
        }

        fn try_get_decimal(&self, col: &str) -> Result<Option<f64>, SyncError> {
            match self.cells.get(col) {
                None => Err(missing(self.table, col)),
                Some(MockValue::Null) => Ok(None),
                Some(MockValue::Decimal(n)) => Ok(Some(*n)),
                // `F64` cells are interchangeable with `Decimal` for the
                // purposes of probe; we accept both so test fixtures can
                // pick whichever variant reads more naturally.
                Some(MockValue::F64(n)) => Ok(Some(*n)),
                Some(_) => Err(type_mismatch(self.table, col, "decimal")),
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

    /// Phase 5.3 — `try_get_decimal` exists, returns the cell value for
    /// both `Decimal` and `F64` mock arms, and propagates SQL NULL.
    #[test]
    fn hashmap_row_decimal_arm_returns_value() {
        let row = HashMapRow::new("HT_Book_H").with("Book_Price_Total", MockValue::Decimal(890.0));
        assert_eq!(row.try_get_decimal("Book_Price_Total").unwrap(), Some(890.0));
    }

    #[test]
    fn hashmap_row_decimal_arm_accepts_f64_cells_too() {
        // Decimal/F64 are interchangeable so test fixtures can pick whichever
        // reads more naturally without forcing a type-tag rewrite.
        let row = HashMapRow::new("HT_Book_H").with("Book_Price_Pay", MockValue::F64(0.0));
        assert_eq!(row.try_get_decimal("Book_Price_Pay").unwrap(), Some(0.0));
    }

    #[test]
    fn hashmap_row_decimal_arm_returns_none_for_null() {
        let row = HashMapRow::new("HT_Book_H").with("Book_Price_Vat", MockValue::Null);
        assert!(row.try_get_decimal("Book_Price_Vat").unwrap().is_none());
    }

    #[test]
    fn hashmap_row_decimal_arm_rejects_str_cells() {
        let row = HashMapRow::new("HT_Book_H")
            .with("Book_Price_Total", MockValue::Str("not-a-number".into()));
        let err = row
            .try_get_decimal("Book_Price_Total")
            .expect_err("Str cell must not satisfy decimal probe");
        assert!(err.to_string().contains("not of type decimal"));
    }
}
