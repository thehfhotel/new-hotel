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

// Modules live in `lib.rs` so they're reachable from integration tests under
// `tests/`. The binary brings them into scope via `use hotel_backend::*`.
// Phase 1b added `repository`; Phase 3b moved declaration to lib.rs — kept here
// so integration tests can reach all 11 top-level modules.
use hotel_backend::{config, db, middleware as app_middleware, routes, scheduler};

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    middleware as axum_middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{auth_enabled_from_env, AppConfig};
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

    // Create HF Ville pool. Phase 5 Ville cutover (#76, 2026-04-30) repointed
    // this from the legacy `?options=-csearch_path%3Dville` (ville schema in
    // hotelnew) to the new `hotelville` database (fed by sync-hfville CT
    // watcher). Now uses `VilleDbConfig` verbatim — VILLE_DB_SERVER / _PORT /
    // _NAME / _USER / _PASSWORD env vars drive the connection.
    let ville_pool = if config.ville_db.enabled {
        let ville_conn = config.ville_db.connection_string();
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.ville_db.pool_max)
            .connect(&ville_conn)
            .await
        {
            Ok(pool) => {
                tracing::info!(
                    "HF Ville pool created ({}:{}/{})",
                    config.ville_db.server, config.ville_db.port, config.ville_db.database
                );
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("Failed to create HF Ville pool: {}", e);
                None
            }
        }
    } else {
        tracing::info!("HF Ville database disabled (VILLE_DB_ENABLED not set)");
        None
    };

    // Phase 4 PR2: read AUTH_ENABLED once at startup. Defaults to false
    // so existing deployments stay unauthenticated until the operator
    // provisions an admin via `cargo run --bin create_user` and flips
    // the flag. The `/api/auth/*` endpoints are mounted regardless so
    // the frontend can probe for "auth is on" via `/api/auth/me`.
    let auth_enabled = auth_enabled_from_env();
    tracing::info!("Auth middleware: enabled={}", auth_enabled);

    // Create AppState based on available pools
    // Note: Pool types differ now (legacy=DbPool/tiberius, new=PgPool/sqlx)
    // so we can't clone one for the other.
    let (app_state, legacy_available, new_available) = match (&legacy_pool, new_pool) {
        (Some(legacy), Some(new_hotel)) => {
            tracing::info!("Dual database mode: Both databases available");
            let mut state = AppState::with_mode(legacy.clone(), new_hotel, system_mode)
                .with_auth_enabled(auth_enabled);
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
        if let Err(e) = init_scheduler(
            pool.clone(),
            pg_for_scheduler,
            config.slack.clone(),
            config.site.clone(),
        )
        .await
        {
            tracing::warn!("Failed to initialize scheduler: {}", e);
        }
    } else {
        tracing::info!("Scheduler disabled (legacy database unavailable)");
    }

    // Configure CORS — origins are locked to a curated allowlist read from
    // `BACKEND_ALLOWED_ORIGINS` (comma-separated, locked in v2.59.2).
    // Phase 7 audit M-3 (2026-05-10): tightened methods + headers from
    // wildcard `Any` to explicit lists so an origin-spoofed pre-flight
    // can't probe for arbitrary verbs/headers, and so credentialed
    // requests (cookie session) round-trip cleanly — `Any` is forbidden
    // by the CORS spec when `Access-Control-Allow-Credentials: true`.
    let cors = CorsLayer::new()
        .allow_origin(parse_allowed_origins())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    // Build all routes (use AppState for dual-database access).
    // User-facing legacy-read routes (rooms, bookings, customers, stats, etc.)
    // are PG-only as of Phase 8 — they read from `ht_*_legacy` mirror tables
    // populated by the CT mappers + drift-reconcile job. MSSQL is write-only
    // (writeback worker) for the legacy .NET app.
    let new_routes = if let Some(ref app_state) = final_app_state {
        build_new_routes(app_state.clone())
    } else if let Some(ref pg_pool) = new_pool_for_newonly {
        // New-only mode: create a placeholder AppState
        // Legacy pool won't be used since legacy routes are disabled
        // We need a DbPool though... Let's try creating one that will fail gracefully
        // Clone the pg_pool here so the original stays available for the
        // /health route's HealthState construction below (task #78).
        match create_pool(&config.db).await {
            Ok(legacy) => build_new_routes(
                AppState::with_mode(legacy, pg_pool.clone(), SystemMode::New)
                    .with_auth_enabled(auth_enabled),
            ),
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

    // Healthcheck (`/health`) — task #69 (site id) + task #78 (CT
    // watermark). Carries the site id so an operator / external monitor
    // can tell which deployment responded when HF Hotel + HF Ville
    // share a Slack webhook + log sink, AND the canonical PG pool so
    // the handler can include the current `legacy_ct_state` snapshot.
    // The PG pool is optional: legacy-only mode (no PG configured) is
    // still supported, and the handler renders `null` for the
    // watermark fields in that case.
    let health_pg_pool = match &final_app_state {
        Some(state) => Some(state.new_pool.clone()),
        None => new_pool_for_newonly.as_ref().cloned(),
    };
    let health_state = routes::health::HealthState {
        site_id: config.site.id.clone(),
        pg_pool: health_pg_pool,
    };
    let health_routes = Router::new()
        .route("/health", get(routes::health::health))
        .with_state(health_state);

    // Phase 4 PR2: mount the public `/api/auth/*` endpoints. These are
    // ALWAYS reachable — they're how unauthenticated callers acquire a
    // session, and `/api/auth/me` is how the frontend probes whether
    // auth is enabled at all (returns 401 + `{"error":"unauthenticated"}`
    // when no cookie is present, regardless of `AUTH_ENABLED`).
    //
    // Phase 7 audit M-2 (2026-05-10): the `/api/auth/login` route is
    // wrapped with the in-process per-IP rate limiter (10 attempts per
    // 15-minute sliding window). The limiter is mounted ONLY on login
    // — `/api/auth/me` and `/api/auth/logout` need to stay free of
    // throttling so a stuck client can always rotate its session
    // without first solving a 429. The limiter is wired here rather
    // than inside `routes::auth::router()` so the route module stays
    // free of cross-cutting concerns.
    let auth_routes = match &final_app_state {
        Some(state) => {
            let login_limiter = app_middleware::LoginRateLimitState::new();
            let login_layer = axum_middleware::from_fn_with_state(
                login_limiter,
                app_middleware::login_rate_limit,
            );
            Router::new()
                .route("/api/auth/login", post(routes::auth::login).layer(login_layer))
                .route("/api/auth/logout", post(routes::auth::logout))
                .route("/api/auth/me", get(routes::auth::me))
                .with_state(state.clone())
        }
        None => Router::new(),
    };

    // Phase 4 PR4: mount the protected `/api/admin/*` endpoints. The
    // subrouter is wrapped with the same `require_auth` middleware
    // PR2 added to `/api/new/*` so an authenticated `User` extension
    // is injected on every request. Each handler then performs an
    // explicit role check (admin vs receptionist) — see
    // `routes::admin_users::require_admin`.
    let admin_routes = match &final_app_state {
        Some(state) => {
            let auth_layer = axum_middleware::from_fn_with_state(
                state.clone(),
                app_middleware::require_auth,
            );
            routes::admin_users::router()
                .with_state(state.clone())
                .layer(auth_layer)
        }
        None => Router::new(),
    };

    // Merge all routes. Public routers (auth, health) MUST be merged
    // alongside the protected `new_routes`, never inside it — the
    // `require_auth` middleware below is applied only to `new_routes`,
    // so anything mounted there gets gated when `AUTH_ENABLED=true`.
    let app = Router::new()
        .merge(new_routes)
        .merge(auth_routes)
        .merge(admin_routes)
        .merge(health_routes)
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

/// Default CORS allowlist when `BACKEND_ALLOWED_ORIGINS` is unset.
/// Covers the Next.js dev server on the host (3003) + the in-container
/// `web` service. Production deployments should set the env var with the
/// public hostname(s) instead of relying on the default.
const DEFAULT_ALLOWED_ORIGINS: &str = "http://localhost:3003,http://web:3003";

/// Parse the CORS allowlist from `BACKEND_ALLOWED_ORIGINS` (comma-separated)
/// or fall back to `DEFAULT_ALLOWED_ORIGINS`. Empty entries are skipped
/// silently so trailing commas don't break startup. Malformed origins
/// panic loudly — config errors should fail at startup, not at request
/// time (same loud-failure stance as `config::require_secret`).
fn parse_allowed_origins() -> AllowOrigin {
    let raw = std::env::var("BACKEND_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| DEFAULT_ALLOWED_ORIGINS.to_string());

    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            HeaderValue::from_str(origin).unwrap_or_else(|err| {
                panic!(
                    "BACKEND_ALLOWED_ORIGINS contains a malformed origin {:?}: {}. \
                     Expected comma-separated absolute origins (e.g. \
                     'https://hotel.example.com,http://web:3003').",
                    origin, err
                )
            })
        })
        .collect();

    if origins.is_empty() {
        panic!(
            "BACKEND_ALLOWED_ORIGINS is set but resolved to zero origins after \
             trimming. Refusing to start with an empty CORS allowlist — set the \
             env var to a comma-separated list of absolute origins, or unset it \
             to use the default ({}).",
            DEFAULT_ALLOWED_ORIGINS
        );
    }

    tracing::info!(
        "CORS allowlist: {}",
        origins
            .iter()
            .filter_map(|value| value.to_str().ok())
            .collect::<Vec<_>>()
            .join(", ")
    );

    AllowOrigin::list(origins)
}

/// Build the new routes router with AppState.
///
/// Phase 4 PR2: the entire returned router is wrapped with the
/// `require_auth` middleware. The middleware itself short-circuits to
/// a no-op pass-through when `AppState::auth_enabled` is `false`
/// (the production default), so there is no per-request cost until an
/// operator opts in. The middleware is applied to THIS subrouter only
/// — `/api/auth/*` and `/health` are mounted separately in `main()`
/// and stay public.
fn build_new_routes(app_state: AppState) -> Router {
    let auth_layer = axum_middleware::from_fn_with_state(
        app_state.clone(),
        app_middleware::require_auth,
    );
    Router::new()
        // Rooms routes (PG-only, Phase 8 — reads `ht_rooms_legacy` mirror)
        .route("/api/rooms", get(routes::rooms::list_rooms))
        .route("/api/rooms/status", get(routes::rooms::get_room_status))
        .route("/api/rooms/checkouts-today", get(routes::rooms::get_checkouts_today))
        .route("/api/rooms/{id}", get(routes::rooms::get_room))
        // Legacy booking routes (PG-only, Phase 8 — reads `ht_bookings_legacy` mirror)
        .route("/api/bookings", get(routes::bookings::list_bookings))
        .route("/api/bookings/{id}", get(routes::bookings::get_booking))
        .route(
            "/api/bookings/{id}/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
        // Check-ins route (PG-only, Phase 8 — reads `ht_checkins_legacy` mirror)
        .route("/api/checkins", get(routes::checkins::list_checkins))
        // Legacy customers routes (PG-only, Phase 8 — reads `ht_customers_legacy` mirror)
        .route("/api/customers", get(routes::customers::list_customers))
        .route("/api/customers/{id}/bookings", get(routes::customers::get_customer_bookings))
        .route("/api/customers/{id}/stats", get(routes::customers::get_customer_stats))
        // Stats route (PG-only, Phase 8 — reads `ht_*_legacy` mirrors)
        .route("/api/stats", get(routes::stats::get_stats))
        // Occupancy route (PG-only, Phase 8 — reads `ht_checkins_legacy` mirror)
        .route("/api/occupancy", get(routes::occupancy::get_occupancy))
        // Mode and calendar routes
        .route("/api/mode", get(routes::mode::get_mode))
        .route("/api/calendar", get(routes::calendar::get_calendar))
        // Phase 5.5d — legacy_mirror.* read-only endpoints (coupons,
        // minibar, room moves, pricing reference). Surfaces legacy-
        // only features so receptionists don't switch to the .NET app.
        .route("/api/legacy-mirror/coupons", get(routes::legacy_mirror::list_coupons))
        .route("/api/legacy-mirror/products", get(routes::legacy_mirror::list_products))
        .route("/api/legacy-mirror/room-changes", get(routes::legacy_mirror::list_room_changes))
        .route("/api/legacy-mirror/pricing", get(routes::legacy_mirror::get_pricing_reference))
        // New stats
        .route("/api/new/stats", get(routes::new_stats::get_stats))
        // New customers CRUD
        .route("/api/new/customers", get(routes::new_customers::list_customers).post(routes::new_customers::create_customer))
        .route("/api/new/customers/{id}", get(routes::new_customers::get_customer).put(routes::new_customers::update_customer).delete(routes::new_customers::delete_customer))
        // New rooms CRUD
        .route("/api/new/rooms", get(routes::new_rooms::list_rooms).post(routes::new_rooms::create_room))
        .route("/api/new/rooms/{id}", get(routes::new_rooms::get_room).put(routes::new_rooms::update_room))
        .route("/api/new/rooms/{id}/status", patch(routes::new_rooms::update_room_status))
        // New bookings CRUD
        .route("/api/new/bookings", get(routes::new_bookings::list_bookings).post(routes::new_bookings::create_booking))
        .route("/api/new/bookings/{id}", get(routes::new_bookings::get_booking).put(routes::new_bookings::update_booking))
        .route("/api/new/bookings/{id}/cancel", put(routes::new_bookings::cancel_booking))
        // New check-ins CRUD
        .route("/api/new/checkins", get(routes::new_checkins::list_checkins).post(routes::new_checkins::create_checkin))
        .route("/api/new/checkins/{id}", get(routes::new_checkins::get_checkin))
        .route("/api/new/checkins/{id}/checkout", put(routes::new_checkins::checkout))
        // Guest registry
        .route("/api/new/checkins/{id}/guests", get(routes::new_checkins::list_guests).post(routes::new_checkins::create_guest))
        .route("/api/new/checkins/{id}/guests/{guest_id}", delete(routes::new_checkins::delete_guest))
        // Room types CRUD
        .route("/api/new/room-types", get(routes::new_room_types::list_room_types).post(routes::new_room_types::create_room_type))
        .route("/api/new/room-types/{id}", get(routes::new_room_types::get_room_type).put(routes::new_room_types::update_room_type).delete(routes::new_room_types::delete_room_type))
        // Rates CRUD (legacy ht_rates write path; reads post-F4 come
        // from ht_rate_tiers — see routes/new_rates.rs module docs)
        .route("/api/new/rates", get(routes::new_rates::list_rates).post(routes::new_rates::create_rate))
        .route("/api/new/rates/{id}", get(routes::new_rates::get_rate).put(routes::new_rates::update_rate).delete(routes::new_rates::delete_rate))
        // F4 canonical rate tiers (Room_Type × Cust_Type matrix) read path.
        .route("/api/new/rate-tiers", get(routes::new_rates::list_rate_tiers))
        // Reports
        .route("/api/new/reports/revenue", get(routes::new_reports::get_revenue))
        .route("/api/new/reports/occupancy", get(routes::new_reports::get_occupancy))
        .route("/api/new/reports/revenue-by-room-type", get(routes::new_reports::get_revenue_by_room_type))
        // Invoice
        .route("/api/new/checkins/{id}/invoice", get(routes::new_invoice::get_invoice))
        // Payments
        .route("/api/new/checkins/{id}/payments", get(routes::new_payments::list_payments).post(routes::new_payments::create_payment))
        .route("/api/new/payments/{id}", delete(routes::new_payments::void_payment))
        // Shifts (Track F2 / T1 HIGH-5 — cashier-shift gate for payments)
        .route("/api/new/shifts/open", post(routes::new_shifts::open_shift))
        .route("/api/new/shifts/close", post(routes::new_shifts::close_shift))
        .route("/api/new/shifts/current", get(routes::new_shifts::current_shift))
        .route("/api/new/shifts", get(routes::new_shifts::list_shifts))
        // Inventory Management
        .route("/api/new/inventory/categories", get(routes::new_inventory::list_categories).post(routes::new_inventory::create_category))
        .route("/api/new/inventory/items", get(routes::new_inventory::list_items).post(routes::new_inventory::create_item))
        .route("/api/new/inventory/items/{id}", get(routes::new_inventory::get_item).put(routes::new_inventory::update_item).delete(routes::new_inventory::delete_item))
        .route("/api/new/inventory/rooms", get(routes::new_inventory::list_inventory_rooms))
        .route("/api/new/inventory/rooms/{room_id}", get(routes::new_inventory::get_room_inventory).put(routes::new_inventory::update_room_inventory))
        .route("/api/new/inventory/rooms/{room_id}/check", axum::routing::post(routes::new_inventory::check_room_inventory))
        .route("/api/new/inventory/rooms/{room_id}/replenish", axum::routing::post(routes::new_inventory::replenish_room_inventory))
        .route("/api/new/inventory/adjustments", axum::routing::post(routes::new_inventory::create_stock_adjustment))
        .route("/api/new/inventory/transactions", get(routes::new_inventory::list_transactions).post(routes::new_inventory::create_transaction))
        .route("/api/new/inventory/stats", get(routes::new_inventory::get_stats))
        .route("/api/new/inventory/low-stock", get(routes::new_inventory::get_low_stock))
        // Products (Track F3 — `audit-2026-05-13.md` T1 CRIT-3)
        .route("/api/new/products", get(routes::new_products::list_products))
        .route("/api/new/products/{id}", get(routes::new_products::get_product))
        .route("/api/new/products/{id}/stock-adjust", axum::routing::post(routes::new_products::adjust_stock))
        // Maintenance Management
        .route("/api/new/maintenance/categories", get(routes::new_maintenance::list_categories))
        .route("/api/new/maintenance/requests", get(routes::new_maintenance::list_requests).post(routes::new_maintenance::create_request))
        .route("/api/new/maintenance/requests/{id}", get(routes::new_maintenance::get_request).put(routes::new_maintenance::update_request))
        .route("/api/new/maintenance/requests/{id}/status", put(routes::new_maintenance::update_request_status))
        // Sync status
        .route("/api/new/sync/status", get(routes::new_sync::get_sync_status))
        // Real-time domain-event stream (Phase 4a per architecture.md §3.6e).
        // Long-lived SSE connection; one PgListener per client.
        .route("/api/events", get(routes::events::stream))
        .with_state(app_state)
        // Phase 4 PR2: gate every route above behind the cookie-session
        // auth middleware. The middleware itself is a no-op when
        // `AUTH_ENABLED=false` (the production default), so this is
        // free until an operator opts in. Applied AFTER `with_state`
        // so the inner routes already have their state attached when
        // the layer wraps them.
        .layer(auth_layer)
}
