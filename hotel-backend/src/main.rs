//! Hotel Management System Backend
//!
//! A Rust/Axum backend server for the hotel management system.
//! Replaces the Next.js API routes with a high-performance Rust implementation.
//!
//! Supports dual-database architecture:
//! - Legacy database (shared with legacy application) - Optional
//! - New HotelNew database (owned by this application) - Primary
//!
//! When SYSTEM_MODE=new, the app can run without the legacy database.
//! Legacy routes will return 503 Service Unavailable when legacy DB is unavailable.

mod config;
mod db;
mod error;
mod models;
mod notifications;
mod routes;
mod scheduler;
mod utils;

use axum::{
    routing::{get, patch, post, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::{create_pool, create_new_pool};
use crate::routes::mode::{AppState, SystemMode};
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
    tracing::info!("Legacy Database: {} / {}", config.db.server, config.db.database);
    tracing::info!("New Database: {} / {}", config.new_db.server, config.new_db.database);
    tracing::info!("System Mode: {:?}", config.mode);
    tracing::info!("Server: {}", config.server.addr());

    // Determine system mode
    let system_mode = match config.mode {
        config::SystemMode::Legacy => SystemMode::Legacy,
        config::SystemMode::New => SystemMode::New,
    };

    // Try to create the new HotelNew pool first (required for New mode)
    let new_pool = match create_new_pool(&config.new_db).await {
        Ok(pool) => {
            tracing::info!("HotelNew database pool created successfully");
            Some(pool)
        }
        Err(e) => {
            if system_mode == SystemMode::New {
                // In New mode, HotelNew database is required
                return Err(format!("Failed to connect to HotelNew database (required in New mode): {}", e).into());
            }
            tracing::warn!("Failed to create HotelNew pool: {}", e);
            None
        }
    };

    // Try to create legacy database connection pool
    let legacy_pool = match create_pool(&config.db).await {
        Ok(pool) => {
            tracing::info!("Legacy database pool created successfully");
            Some(pool)
        }
        Err(e) => {
            if system_mode == SystemMode::Legacy {
                // In Legacy mode, legacy database is required
                return Err(format!("Failed to connect to legacy database (required in Legacy mode): {}", e).into());
            }
            tracing::warn!("Failed to create legacy pool (legacy routes will be unavailable): {}", e);
            None
        }
    };

    // Create AppState based on available pools
    let (app_state, legacy_available, new_available) = match (legacy_pool.clone(), new_pool) {
        (Some(legacy), Some(new_hotel)) => {
            tracing::info!("Dual database mode: Both databases available");
            (AppState::with_mode(legacy, new_hotel, system_mode), true, true)
        }
        (Some(legacy), None) => {
            tracing::warn!("Legacy-only mode: HotelNew database unavailable");
            (AppState::with_mode(legacy.clone(), legacy, SystemMode::Legacy), true, false)
        }
        (None, Some(new_hotel)) => {
            tracing::info!("New-only mode: Legacy database unavailable, running with HotelNew only");
            (AppState::with_mode(new_hotel.clone(), new_hotel, SystemMode::New), false, true)
        }
        (None, None) => {
            return Err("No database connections available".into());
        }
    };

    // Initialize scheduler for background jobs (only if legacy pool is available)
    if let Some(ref pool) = legacy_pool {
        if let Err(e) = init_scheduler(pool.clone(), config.slack.clone()).await {
            tracing::warn!("Failed to initialize scheduler: {}", e);
        }
    } else {
        tracing::info!("Scheduler disabled (legacy database unavailable)");
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build legacy routes (use legacy_pool as state) - READ-ONLY
    // Note: Booking notes and get_booking moved to new_routes (require AppState for HotelNew DB writes)
    // These routes are only available when legacy database is connected
    let legacy_routes = if let Some(pool) = legacy_pool {
        Router::new()
            // Rooms routes
            .route("/api/rooms", get(routes::rooms::list_rooms))
            .route("/api/rooms/status", get(routes::rooms::get_room_status))
            .route(
                "/api/rooms/checkouts-today",
                get(routes::rooms::get_checkouts_today),
            )
            .route("/api/rooms/:id", get(routes::rooms::get_room))
            // Bookings list route (read-only from legacy)
            .route("/api/bookings", get(routes::bookings::list_bookings))
            // Check-ins routes
            .route("/api/checkins", get(routes::checkins::list_checkins))
            // Customers routes
            .route("/api/customers", get(routes::customers::list_customers))
            .route(
                "/api/customers/:id/bookings",
                get(routes::customers::get_customer_bookings),
            )
            .route(
                "/api/customers/:id/stats",
                get(routes::customers::get_customer_stats),
            )
            // Stats and occupancy routes
            .route("/api/stats", get(routes::stats::get_stats))
            .route("/api/occupancy", get(routes::occupancy::get_occupancy))
            .with_state(pool)
    } else {
        // Legacy database not available - return empty router
        // Legacy API endpoints will return 404
        tracing::warn!("Legacy routes disabled (no legacy database connection)");
        Router::new()
    };

    // Build new routes (use AppState for dual-database access)
    let new_routes = Router::new()
        // Legacy booking routes that need AppState (for notes in HotelNew DB)
        // get_booking uses legacy DB for booking data + HotelNew DB for notes
        .route("/api/bookings/:id", get(routes::bookings::get_booking))
        // Notes routes use HotelNew DB exclusively (writable)
        .route(
            "/api/bookings/:id/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
        // Mode and calendar routes
        .route("/api/mode", get(routes::mode::get_mode))
        .route("/api/calendar", get(routes::calendar::get_calendar))
        // New stats
        .route("/api/new/stats", get(routes::new_stats::get_stats))
        // New customers CRUD
        .route("/api/new/customers", get(routes::new_customers::list_customers).post(routes::new_customers::create_customer))
        .route("/api/new/customers/:id", get(routes::new_customers::get_customer).put(routes::new_customers::update_customer).delete(routes::new_customers::delete_customer))
        // New rooms CRUD
        .route("/api/new/rooms", get(routes::new_rooms::list_rooms).post(routes::new_rooms::create_room))
        .route("/api/new/rooms/:id", get(routes::new_rooms::get_room).put(routes::new_rooms::update_room))
        .route("/api/new/rooms/:id/status", patch(routes::new_rooms::update_room_status))
        // New bookings CRUD
        .route("/api/new/bookings", get(routes::new_bookings::list_bookings).post(routes::new_bookings::create_booking))
        .route("/api/new/bookings/:id", get(routes::new_bookings::get_booking).put(routes::new_bookings::update_booking))
        .route("/api/new/bookings/:id/cancel", put(routes::new_bookings::cancel_booking))
        // New check-ins CRUD
        .route("/api/new/checkins", get(routes::new_checkins::list_checkins).post(routes::new_checkins::create_checkin))
        .route("/api/new/checkins/:id", get(routes::new_checkins::get_checkin))
        .route("/api/new/checkins/:id/checkout", put(routes::new_checkins::checkout))
        // Guest registry
        .route("/api/new/checkins/:id/guests", get(routes::new_checkins::list_guests).post(routes::new_checkins::create_guest))
        .route("/api/new/checkins/:id/guests/:guest_id", delete(routes::new_checkins::delete_guest))
        // Room types CRUD
        .route("/api/new/room-types", get(routes::new_room_types::list_room_types).post(routes::new_room_types::create_room_type))
        .route("/api/new/room-types/:id", get(routes::new_room_types::get_room_type).put(routes::new_room_types::update_room_type).delete(routes::new_room_types::delete_room_type))
        // Rates CRUD
        .route("/api/new/rates", get(routes::new_rates::list_rates).post(routes::new_rates::create_rate))
        .route("/api/new/rates/:id", get(routes::new_rates::get_rate).put(routes::new_rates::update_rate).delete(routes::new_rates::delete_rate))
        // Reports
        .route("/api/new/reports/revenue", get(routes::new_reports::get_revenue))
        .route("/api/new/reports/occupancy", get(routes::new_reports::get_occupancy))
        .route("/api/new/reports/revenue-by-room-type", get(routes::new_reports::get_revenue_by_room_type))
        // Invoice
        .route("/api/new/checkins/:id/invoice", get(routes::new_invoice::get_invoice))
        // Inventory Management
        .route("/api/new/inventory/categories", get(routes::new_inventory::list_categories).post(routes::new_inventory::create_category))
        .route("/api/new/inventory/items", get(routes::new_inventory::list_items).post(routes::new_inventory::create_item))
        .route("/api/new/inventory/items/:id", get(routes::new_inventory::get_item).put(routes::new_inventory::update_item).delete(routes::new_inventory::delete_item))
        .route("/api/new/inventory/rooms/:room_id", get(routes::new_inventory::get_room_inventory).put(routes::new_inventory::update_room_inventory))
        .route("/api/new/inventory/transactions", get(routes::new_inventory::list_transactions).post(routes::new_inventory::create_transaction))
        .route("/api/new/inventory/stats", get(routes::new_inventory::get_stats))
        .route("/api/new/inventory/low-stock", get(routes::new_inventory::get_low_stock))
        .with_state(app_state);

    // Merge all routes
    let app = Router::new()
        .merge(legacy_routes)
        .merge(new_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Log database availability status
    match (legacy_available, new_available) {
        (true, true) => tracing::info!("Full dual-database mode: All features available"),
        (true, false) => tracing::warn!("Legacy-only mode: New features disabled"),
        (false, true) => tracing::info!("New-only mode: Legacy routes disabled, using HotelNew database"),
        (false, false) => unreachable!(), // We return early if both are unavailable
    }

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server.addr()).await?;
    tracing::info!("Server running on http://{}", config.server.addr());

    axum::serve(listener, app).await?;

    Ok(())
}
