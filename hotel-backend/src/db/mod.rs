//! Database module for connection pooling
//!
//! - Legacy DB: SQL Server via tiberius/bb8 (read-only)
//! - HotelNew DB: PostgreSQL via sqlx

mod dual_pool;
pub mod mssql_session;
pub mod mssql_timeout;
mod pg_pool;
mod pool;

pub use pg_pool::{create_pg_pool, PgPool};
pub use pool::{create_pool, DbPool};
