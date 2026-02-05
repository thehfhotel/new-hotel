//! Database module for SQL Server connection pooling

mod dual_pool;
mod pool;

pub use dual_pool::{create_dual_pool, DualDbPool};
pub use pool::{create_pool, DbPool};
