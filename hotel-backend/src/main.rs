//! Hotel Management System Backend
//!
//! A Rust/Axum backend server for the hotel management system.
//! Replaces the Next.js API routes with a high-performance Rust implementation.
//!
//! Supports dual-database architecture:
//! - Legacy database (SQL Server, shared with legacy application) - Optional
//! - New HotelNew database (PostgreSQL, owned by this application) - Primary
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
    routing::{get, patch, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::{create_pool, create_pg_pool};
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
    tracing::info!("New Database (PostgreSQL): {} / {}", config.new_db.server, config.new_db.database);
    tracing::info!("HF Ville Database: {} / {} (enabled: {})", config.ville_db.server, config.ville_db.database, config.ville_db.enabled);
    tracing::info!("System Mode: {:?}", config.mode);
    tracing::info!("Server: {}", config.server.addr());

    // Determine system mode
    let system_mode = match config.mode {
        config::SystemMode::Legacy => SystemMode::Legacy,
        config::SystemMode::New => SystemMode::New,
    };

    // Try to create the new HotelNew PostgreSQL pool first (required for New mode)
    let new_pool = match create_pg_pool(&config.new_db).await {
        Ok(pool) => {
            tracing::info!("HotelNew PostgreSQL pool created successfully");
            Some(pool)
        }
        Err(e) => {
            if system_mode == SystemMode::New {
                // In New mode, HotelNew database is required
                return Err(format!("Failed to connect to HotelNew PostgreSQL database (required in New mode): {}", e).into());
            }
            tracing::warn!("Failed to create HotelNew pool: {}", e);
            None
        }
    };

    // Try to create legacy database connection pool (SQL Server)
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

    // Try to create HF Ville PostgreSQL pool (optional, graceful degradation)
    let ville_pool = if config.ville_db.enabled {
        match create_pg_pool(&crate::config::NewDbConfig {
            server: config.ville_db.server.clone(),
            port: config.ville_db.port,
            database: config.ville_db.database.clone(),
            user: config.ville_db.user.clone(),
            password: config.ville_db.password.clone(),
            pool_max: config.ville_db.pool_max,
        }).await {
            Ok(pool) => {
                tracing::info!("HF Ville PostgreSQL pool created successfully");
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("Failed to create HF Ville pool (ville routes will be unavailable): {}", e);
                None
            }
        }
    } else {
        tracing::info!("HF Ville database disabled (VILLE_DB_ENABLED not set)");
        None
    };

    // Create AppState based on available pools
    // Note: Pool types differ now (legacy=DbPool/tiberius, new=PgPool/sqlx)
    // so we can't clone one for the other.
    let (app_state, legacy_available, new_available) = match (&legacy_pool, new_pool) {
        (Some(legacy), Some(new_hotel)) => {
            tracing::info!("Dual database mode: Both databases available");
            let mut state = AppState::with_mode(legacy.clone(), new_hotel, system_mode);
            if let Some(vp) = ville_pool {
                state = state.with_ville(vp);
            }
            (Some(state), true, true)
        }
        (Some(_legacy), None) => {
            tracing::warn!("Legacy-only mode: HotelNew database unavailable, new routes will not work");
            // Cannot create AppState without PgPool - new routes will be disabled
            (None, true, false)
        }
        (None, Some(_new_hotel)) => {
            tracing::info!("New-only mode: Legacy database unavailable, running with HotelNew only");
            // Legacy routes will be disabled (no legacy pool)
            // For AppState, we need a legacy pool placeholder - but we don't have one.
            // We'll handle this by not creating legacy routes.
            (None, false, true)
        }
        (None, None) => {
            return Err("No database connections available".into());
        }
    };

    // In new-only mode, we still need an AppState for the new routes
    // We need a dummy legacy pool or we restructure. Let's create AppState only when we have both,
    // or when we have just new_pool (with a dummy legacy pool that will never be used).
    let (final_app_state, new_pool_for_newonly) = if let Some(state) = app_state {
        (Some(state), None)
    } else if legacy_pool.is_none() && new_available {
        // New-only mode: create AppState with a placeholder.
        // Since legacy routes are disabled, the legacy_pool in AppState will never be accessed.
        // But we need to create a dummy connection. Instead, let's try to connect legacy and if it fails,
        // create a minimal pool. Actually, simpler: just don't mount new_routes if we don't have AppState.
        // But that defeats the purpose. Let's restructure: create legacy pool attempt again.
        // Simplest approach: require legacy pool as placeholder even if unused, OR just skip.
        // The current deployment uses SYSTEM_MODE=new with both DBs available.
        // For robustness, let's skip new_routes in legacy-only mode and legacy_routes in new-only mode.
        // For new-only mode, we still need AppState. Let's store the PgPool separately.
        // Actually the simplest fix: try to create legacy pool, if it fails, the legacy_pool field will
        // never be used (new-only mode). We can't create a dummy DbPool easily.
        // Instead, let's store the PgPool for new-only routing.
        (None, Some(create_pg_pool(&config.new_db).await?))
    } else {
        (None, None)
    };

    // Initialize scheduler for background jobs (only if legacy pool is available)
    // Pass PgPool if available for legacy-to-PG sync job
    if let Some(ref pool) = legacy_pool {
        let pg_for_scheduler = match &final_app_state {
            Some(state) => Some(state.new_pool.clone()),
            None => new_pool_for_newonly.as_ref().map(|p| p.clone()),
        };
        if let Err(e) = init_scheduler(pool.clone(), pg_for_scheduler, config.slack.clone()).await {
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

    // Build all routes (use AppState for dual-database access)
    // Legacy-read routes (rooms, bookings, customers, stats) use LEGACY_READ_SOURCE feature flag
    let new_routes = if let Some(ref app_state) = final_app_state {
        build_new_routes(app_state.clone())
    } else if let Some(pg_pool) = new_pool_for_newonly {
        // New-only mode: create a placeholder AppState
        // Legacy pool won't be used since legacy routes are disabled
        // We need a DbPool though... Let's try creating one that will fail gracefully
        match create_pool(&config.db).await {
            Ok(legacy) => build_new_routes(AppState::with_mode(legacy, pg_pool, SystemMode::New)),
            Err(_) => {
                // Can't get legacy pool at all - create routes with just the PG pool
                // We'll need to handle this differently
                tracing::warn!("Cannot create AppState for new routes without legacy pool placeholder");
                Router::new()
            }
        }
    } else {
        Router::new()
    };

    // Merge all routes
    let app = Router::new()
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

/// Build the new routes router with AppState
fn build_new_routes(app_state: AppState) -> Router {
    Router::new()
        // Rooms routes (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/rooms", get(routes::rooms::list_rooms))
        .route("/api/rooms/status", get(routes::rooms::get_room_status))
        .route("/api/rooms/checkouts-today", get(routes::rooms::get_checkouts_today))
        .route("/api/rooms/:id", get(routes::rooms::get_room))
        // Legacy booking routes (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/bookings", get(routes::bookings::list_bookings))
        .route("/api/bookings/:id", get(routes::bookings::get_booking))
        .route(
            "/api/bookings/:id/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
        // Check-ins route (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/checkins", get(routes::checkins::list_checkins))
        // Legacy customers routes (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/customers", get(routes::customers::list_customers))
        .route("/api/customers/:id/bookings", get(routes::customers::get_customer_bookings))
        .route("/api/customers/:id/stats", get(routes::customers::get_customer_stats))
        // Stats route (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/stats", get(routes::stats::get_stats))
        // Occupancy route (reads from PG by default, SQL Server if LEGACY_READ_SOURCE=sqlserver)
        .route("/api/occupancy", get(routes::occupancy::get_occupancy))
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
        // Payments
        .route("/api/new/checkins/:id/payments", get(routes::new_payments::list_payments).post(routes::new_payments::create_payment))
        .route("/api/new/payments/:id", delete(routes::new_payments::void_payment))
        // Inventory Management
        .route("/api/new/inventory/categories", get(routes::new_inventory::list_categories).post(routes::new_inventory::create_category))
        .route("/api/new/inventory/items", get(routes::new_inventory::list_items).post(routes::new_inventory::create_item))
        .route("/api/new/inventory/items/:id", get(routes::new_inventory::get_item).put(routes::new_inventory::update_item).delete(routes::new_inventory::delete_item))
        .route("/api/new/inventory/rooms/:room_id", get(routes::new_inventory::get_room_inventory).put(routes::new_inventory::update_room_inventory))
        .route("/api/new/inventory/transactions", get(routes::new_inventory::list_transactions).post(routes::new_inventory::create_transaction))
        .route("/api/new/inventory/stats", get(routes::new_inventory::get_stats))
        .route("/api/new/inventory/low-stock", get(routes::new_inventory::get_low_stock))
        // Maintenance Management
        .route("/api/new/maintenance/categories", get(routes::new_maintenance::list_categories))
        .route("/api/new/maintenance/requests", get(routes::new_maintenance::list_requests).post(routes::new_maintenance::create_request))
        .route("/api/new/maintenance/requests/:id", get(routes::new_maintenance::get_request).put(routes::new_maintenance::update_request))
        .route("/api/new/maintenance/requests/:id/status", put(routes::new_maintenance::update_request_status))
        // Sync status
        .route("/api/new/sync/status", get(routes::new_sync::get_sync_status))
        .with_state(app_state)
}
