//! Library crate for `hotel-backend`.
//!
//! Exists so integration tests under `tests/` can `use hotel_backend::...`
//! to reach the domain / outbox / repository modules. The binary
//! (`src/main.rs`) continues to declare these modules with `mod` directly —
//! both crates compile independently from the same source files.
//!
//! Keep this file small: no logic, just `pub mod` re-declarations of the
//! modules that integration tests need to reach. Anything that requires
//! `tokio::main`, server bootstrapping, or runtime config stays in `main.rs`.

pub mod config;
pub mod db;
pub mod domain;
pub mod error;
pub mod models;
pub mod notifications;
pub mod outbox;
pub mod repository;
pub mod routes;
pub mod scheduler;
pub mod utils;
