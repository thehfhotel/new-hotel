//! Hotel Management System Backend
//!
//! A Rust/Axum backend server for the hotel management system.
//! Replaces the Next.js API routes with a high-performance Rust implementation.
//!
//! Supports dual-database architecture:
//! - Legacy database (shared with legacy application)
//! - New HotelNew database (owned by this application)

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
use crate::db::{create_pool, create_dual_pool};
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

    // Create legacy database connection pool (for backward compatibility)
    let legacy_pool = create_pool(&config.db).await?;
    tracing::info!("Legacy database pool created");

    // Try to create dual database pools
    let (app_state, dual_pool_available) = match create_dual_pool(&config).await {
        Ok(dual_pool) => {
            tracing::info!("Dual database pools created successfully");
            let mode = match config.mode {
                config::SystemMode::Legacy => SystemMode::Legacy,
                config::SystemMode::New => SystemMode::New,
            };
            (AppState::with_mode(dual_pool.legacy.clone(), dual_pool.new_hotel, mode), true)
        }
        Err(e) => {
            tracing::warn!("Failed to create new_hotel pool (will use legacy only): {}", e);
            // Create AppState with legacy pool for both (new features will be disabled)
            (AppState::with_mode(legacy_pool.clone(), legacy_pool.clone(), SystemMode::Legacy), false)
        }
    };

    // Initialize scheduler for background jobs
    if let Err(e) = init_scheduler(legacy_pool.clone(), config.slack.clone()).await {
        tracing::warn!("Failed to initialize scheduler: {}", e);
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build legacy routes (use legacy_pool as state)
    let legacy_routes = Router::new()
        // Rooms routes
        .route("/api/rooms", get(routes::rooms::list_rooms))
        .route("/api/rooms/status", get(routes::rooms::get_room_status))
        .route(
            "/api/rooms/checkouts-today",
            get(routes::rooms::get_checkouts_today),
        )
        .route("/api/rooms/:id", get(routes::rooms::get_room))
        // Bookings routes
        .route("/api/bookings", get(routes::bookings::list_bookings))
        .route("/api/bookings/:id", get(routes::bookings::get_booking))
        .route(
            "/api/bookings/:id/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
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
        .with_state(legacy_pool);

    // Build new routes (use AppState for dual-database access)
    let new_routes = Router::new()
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

    // Log dual pool status
    if dual_pool_available {
        tracing::info!("New database features enabled");
    } else {
        tracing::warn!("New database features disabled (legacy mode only)");
    }

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server.addr()).await?;
    tracing::info!("Server running on http://{}", config.server.addr());

    axum::serve(listener, app).await?;

    Ok(())
}
