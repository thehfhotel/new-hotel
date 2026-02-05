//! Hotel Management System Backend
//!
//! A Rust/Axum backend server for the hotel management system.
//! Replaces the Next.js API routes with a high-performance Rust implementation.

mod config;
mod db;
mod error;
mod models;
mod notifications;
mod routes;
mod scheduler;
mod utils;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::create_pool;
use crate::scheduler::init_scheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hotel_backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env();
    tracing::info!("Configuration loaded");
    tracing::info!("Database: {}", config.db.server);
    tracing::info!("Server: {}", config.server.addr());

    // Create database connection pool
    let pool = create_pool(&config.db).await?;
    tracing::info!("Database pool created");

    // Initialize scheduler for background jobs
    if let Err(e) = init_scheduler(pool.clone(), config.slack.clone()).await {
        tracing::warn!("Failed to initialize scheduler: {}", e);
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        // Rooms routes
        .route("/api/rooms", get(routes::rooms::list_rooms))
        .route("/api/rooms/status", get(routes::rooms::get_room_status))
        .route(
            "/api/rooms/checkouts-today",
            get(routes::rooms::get_checkouts_today),
        )
        .route("/api/rooms/{id}", get(routes::rooms::get_room))
        // Bookings routes
        .route("/api/bookings", get(routes::bookings::list_bookings))
        .route("/api/bookings/{id}", get(routes::bookings::get_booking))
        .route(
            "/api/bookings/{id}/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
        // Check-ins routes
        .route("/api/checkins", get(routes::checkins::list_checkins))
        // Customers routes
        .route("/api/customers", get(routes::customers::list_customers))
        .route(
            "/api/customers/{id}/bookings",
            get(routes::customers::get_customer_bookings),
        )
        .route(
            "/api/customers/{id}/stats",
            get(routes::customers::get_customer_stats),
        )
        // Stats and occupancy routes
        .route("/api/stats", get(routes::stats::get_stats))
        .route("/api/occupancy", get(routes::occupancy::get_occupancy))
        // Middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server.addr()).await?;
    tracing::info!("Server running on http://{}", config.server.addr());

    axum::serve(listener, app).await?;

    Ok(())
}
