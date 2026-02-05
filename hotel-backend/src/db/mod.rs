//! Database module for SQL Server connection pooling

mod pool;

pub use pool::{create_pool, DbPool};
