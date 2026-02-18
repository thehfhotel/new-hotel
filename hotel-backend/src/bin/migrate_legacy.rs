//! One-time legacy data migration CLI tool
//!
//! Imports all historical data from SQL Server (legacy) into PostgreSQL (HotelNew).
//!
//! Migration order (respects FK dependencies):
//! 1. Extract distinct Room_Type values -> ht_room_types
//! 2. Import HT_Rooms -> ht_rooms_new (link room_type_id)
//! 3. Import View_Customers -> ht_customers (split Cust_name into first/last)
//! 4. Import View_Booking_Ds -> ht_bookings + ht_booking_rooms (group by Book_No)
//! 5. Import View_CheckIn_Ds -> ht_checkins (link customer + room)
//! 6. Bump PostgreSQL sequences past max imported IDs
//! 7. All imported records tagged with source = 'legacy'
//!
//! Usage:
//!   cargo run --bin migrate_legacy
//!   cargo run --bin migrate_legacy -- --dry-run

use std::collections::HashMap;

use chrono::NaiveDateTime;

fn main() {
    // Use tokio runtime manually since this is a standalone binary
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async_main());
}

async fn async_main() {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "migrate_legacy=info".into()),
        )
        .init();

    let dry_run = std::env::args().any(|a| a == "--dry-run");

    if dry_run {
        tracing::info!("=== DRY RUN MODE - No data will be written ===");
    }

    tracing::info!("Starting legacy data migration...");

    // Connect to both databases
    let legacy_pool = create_legacy_pool().await.expect("Failed to connect to legacy SQL Server");
    let pg_pool = create_pg_pool().await.expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to both databases");

    // Run migration in a transaction
    let mut tx = pg_pool.begin().await.expect("Failed to start transaction");

    let result = run_migration(&legacy_pool, &mut tx, dry_run).await;

    match result {
        Ok(stats) => {
            if dry_run {
                tracing::info!("=== DRY RUN COMPLETE - Rolling back ===");
                tx.rollback().await.expect("Failed to rollback");
            } else {
                tx.commit().await.expect("Failed to commit transaction");
                tracing::info!("=== MIGRATION COMMITTED ===");
            }
            print_stats(&stats);
        }
        Err(e) => {
            tracing::error!("Migration failed: {}", e);
            tx.rollback().await.expect("Failed to rollback");
            tracing::info!("Transaction rolled back");
            std::process::exit(1);
        }
    }
}

#[derive(Default)]
struct MigrationStats {
    room_types: i32,
    rooms: i32,
    rooms_skipped: i32,
    customers: i32,
    customers_skipped: i32,
    bookings: i32,
    bookings_skipped: i32,
    booking_rooms: i32,
    checkins: i32,
    checkins_skipped: i32,
}

fn print_stats(stats: &MigrationStats) {
    tracing::info!("=== Migration Summary ===");
    tracing::info!("  Room types created:    {}", stats.room_types);
    tracing::info!("  Rooms imported:        {} (skipped: {})", stats.rooms, stats.rooms_skipped);
    tracing::info!("  Customers imported:    {} (skipped: {})", stats.customers, stats.customers_skipped);
    tracing::info!("  Bookings imported:     {} (skipped: {})", stats.bookings, stats.bookings_skipped);
    tracing::info!("  Booking-rooms created: {}", stats.booking_rooms);
    tracing::info!("  Check-ins imported:    {} (skipped: {})", stats.checkins, stats.checkins_skipped);
}

async fn run_migration(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    _dry_run: bool,
) -> Result<MigrationStats, Box<dyn std::error::Error + Send + Sync>> {
    let mut stats = MigrationStats::default();

    // Step 1: Extract room types from legacy rooms
    tracing::info!("[Step 1/6] Extracting room types...");
    let room_type_map = import_room_types(legacy_pool, &mut *tx, &mut stats).await?;

    // Step 2: Import rooms
    tracing::info!("[Step 2/6] Importing rooms...");
    let room_map = import_rooms(legacy_pool, &mut *tx, &room_type_map, &mut stats).await?;

    // Step 3: Import customers
    tracing::info!("[Step 3/6] Importing customers...");
    let customer_map = import_customers(legacy_pool, &mut *tx, &mut stats).await?;

    // Step 4: Import bookings
    tracing::info!("[Step 4/6] Importing bookings...");
    import_bookings(
        legacy_pool, &mut *tx, &customer_map, &room_type_map, &room_map, &mut stats,
    ).await?;

    // Step 5: Import check-ins
    tracing::info!("[Step 5/6] Importing check-ins...");
    import_checkins(
        legacy_pool, &mut *tx, &customer_map, &room_map, &mut stats,
    ).await?;

    // Step 6: Bump sequences
    tracing::info!("[Step 6/6] Bumping sequences...");
    bump_sequences(&mut *tx).await?;

    Ok(stats)
}

// =============================================================================
// Step 1: Room Types
// =============================================================================

/// Returns map of legacy room_type string -> new type_id
async fn import_room_types(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::PgConnection,
    stats: &mut MigrationStats,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query("SELECT DISTINCT Room_Type FROM HT_Rooms WHERE Room_Type IS NOT NULL AND Room_Type <> ''")
        .await?
        .into_first_result()
        .await?;

    let mut type_map: HashMap<String, i32> = HashMap::new();

    // First load existing types
    let existing: Vec<(i32, String)> = sqlx::query_as(
        "SELECT type_id, type_code FROM ht_room_types"
    )
    .fetch_all(&mut *tx)
    .await?;

    for (id, code) in &existing {
        type_map.insert(code.clone(), *id);
    }

    for row in &rows {
        let room_type = row.get::<&str, _>("Room_Type").unwrap_or_default().to_string();
        if room_type.is_empty() || type_map.contains_key(&room_type) {
            continue;
        }

        let type_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ht_room_types (type_code, type_name, type_name_en)
            VALUES ($1, $2, $3)
            ON CONFLICT (type_code) DO UPDATE SET type_code = EXCLUDED.type_code
            RETURNING type_id
            "#,
        )
        .bind(&room_type)
        .bind(&room_type)
        .bind(&room_type)
        .fetch_one(&mut *tx)
        .await?;

        type_map.insert(room_type.clone(), type_id);
        stats.room_types += 1;
        tracing::info!("  Created room type: {} -> id {}", room_type, type_id);
    }

    tracing::info!("  {} room types total ({} new)", type_map.len(), stats.room_types);
    Ok(type_map)
}

// =============================================================================
// Step 2: Rooms
// =============================================================================

/// Returns map of legacy room_no -> new room_id
async fn import_rooms(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::PgConnection,
    room_type_map: &HashMap<String, i32>,
    stats: &mut MigrationStats,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT Room_no, Room_Type, Room_Details, Room_PriceA, Room_PriceB,
                   Room_PriceC, Room_Group
            FROM HT_Rooms
            ORDER BY Room_no
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut room_map: HashMap<String, i32> = HashMap::new();

    // Load existing rooms
    let existing: Vec<(i32, String)> = sqlx::query_as(
        "SELECT room_id, room_no FROM ht_rooms_new"
    )
    .fetch_all(&mut *tx)
    .await?;

    for (id, no) in &existing {
        room_map.insert(no.clone(), *id);
    }

    for row in &rows {
        let room_no = row.get::<&str, _>("Room_no").unwrap_or_default().to_string();
        if room_no.is_empty() || room_map.contains_key(&room_no) {
            stats.rooms_skipped += 1;
            continue;
        }

        let room_type = row.get::<&str, _>("Room_Type").unwrap_or_default().to_string();
        let room_type_id = room_type_map.get(&room_type).copied();
        let room_details = row.get::<&str, _>("Room_Details").map(String::from);
        let price_a = row.get::<f64, _>("Room_PriceA");
        let price_b = row.get::<f64, _>("Room_PriceB");
        let price_c = row.get::<f64, _>("Room_PriceC");
        let room_group = row.get::<&str, _>("Room_Group").map(String::from);

        // Parse floor from room number (e.g., "201" -> floor 2)
        let floor: Option<i32> = room_no.chars().next()
            .and_then(|c| c.to_digit(10))
            .map(|d| d as i32);

        let room_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ht_rooms_new
                (room_no, room_type_id, room_floor, room_building, room_view,
                 room_price_weekday, room_price_weekend, room_price_special, room_notes)
            VALUES ($1, $2, $3, $4, NULL,
                    $5::float8, $6::float8, $7::float8, $8)
            ON CONFLICT (room_no) DO UPDATE SET room_no = EXCLUDED.room_no
            RETURNING room_id
            "#,
        )
        .bind(&room_no)
        .bind(&room_type_id)
        .bind(&floor)
        .bind(&room_group)
        .bind(&price_a)
        .bind(&price_b)
        .bind(&price_c)
        .bind(&room_details)
        .fetch_one(&mut *tx)
        .await?;

        room_map.insert(room_no.clone(), room_id);
        stats.rooms += 1;
    }

    tracing::info!("  {} rooms imported, {} skipped (already exist)", stats.rooms, stats.rooms_skipped);
    Ok(room_map)
}

// =============================================================================
// Step 3: Customers
// =============================================================================

/// Returns map of legacy cust_no -> new cust_id
async fn import_customers(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::PgConnection,
    stats: &mut MigrationStats,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT Cust_no, Cust_name, Cust_Type, Cust_Add_tel, Cust_IDcard, C_Address
            FROM View_Customers
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut customer_map: HashMap<String, i32> = HashMap::new();

    // Load existing customers by code
    let existing: Vec<(i32, Option<String>)> = sqlx::query_as(
        "SELECT cust_id, cust_code FROM ht_customers WHERE cust_code IS NOT NULL"
    )
    .fetch_all(&mut *tx)
    .await?;

    for (id, code) in &existing {
        if let Some(code) = code {
            customer_map.insert(code.clone(), *id);
        }
    }

    for row in &rows {
        let cust_no = row.get::<&str, _>("Cust_no").unwrap_or_default().to_string();
        if cust_no.is_empty() || customer_map.contains_key(&cust_no) {
            stats.customers_skipped += 1;
            continue;
        }

        let full_name = row.get::<&str, _>("Cust_name").unwrap_or_default().to_string();
        let cust_type = row.get::<&str, _>("Cust_Type").map(String::from);
        let phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
        let idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
        let address = row.get::<&str, _>("C_Address").map(String::from);

        // Split name: first word is firstname, rest is lastname
        let (firstname, lastname) = split_name(&full_name);

        let cust_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ht_customers
                (cust_code, cust_firstname, cust_lastname, cust_phone,
                 cust_idcard, cust_address, cust_type, source)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'legacy')
            RETURNING cust_id
            "#,
        )
        .bind(&cust_no)
        .bind(&firstname)
        .bind(&lastname)
        .bind(&phone)
        .bind(&idcard)
        .bind(&address)
        .bind(&cust_type)
        .fetch_one(&mut *tx)
        .await?;

        customer_map.insert(cust_no.clone(), cust_id);
        stats.customers += 1;
    }

    tracing::info!("  {} customers imported, {} skipped", stats.customers, stats.customers_skipped);
    Ok(customer_map)
}

/// Split a Thai/English name into first and last name
fn split_name(full_name: &str) -> (String, Option<String>) {
    let trimmed = full_name.trim();
    if trimmed.is_empty() {
        return ("Unknown".to_string(), None);
    }

    match trimmed.split_once(' ') {
        Some((first, rest)) => (first.to_string(), Some(rest.trim().to_string())),
        None => (trimmed.to_string(), None),
    }
}

// =============================================================================
// Step 4: Bookings
// =============================================================================

/// Map legacy Book_Status to new status string
fn map_book_status(code: Option<i32>) -> &'static str {
    match code {
        Some(1) => "confirmed",
        Some(2) => "checkedin",
        Some(3) => "completed",
        Some(4) => "cancelled",
        _ => "pending",
    }
}

async fn import_bookings(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::PgConnection,
    customer_map: &HashMap<String, i32>,
    room_type_map: &HashMap<String, i32>,
    room_map: &HashMap<String, i32>,
    stats: &mut MigrationStats,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Book_No, Book_Date, Book_Date_in, Book_Date_out,
                Book_Cust_Name, Book_Cust_ID, Book_Status,
                Book_Room_Type, Book_Room_No
            FROM View_Booking_Ds
            ORDER BY Book_No, Book_Room_Type
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    // Check existing bookings by book_no
    let existing_bookings: Vec<String> = sqlx::query_scalar(
        "SELECT book_no FROM ht_bookings"
    )
    .fetch_all(&mut *tx)
    .await?;

    let existing_set: std::collections::HashSet<String> = existing_bookings.into_iter().collect();

    // Group rows by Book_No
    let mut grouped: HashMap<String, Vec<&tiberius::Row>> = HashMap::new();
    for row in &rows {
        let book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
        grouped.entry(book_no).or_default().push(row);
    }

    for (book_no, rows) in &grouped {
        if existing_set.contains(book_no) {
            stats.bookings_skipped += 1;
            continue;
        }

        let first = rows[0];
        let book_date = first.get::<NaiveDateTime, _>("Book_Date");
        let book_date_in = first.get::<NaiveDateTime, _>("Book_Date_in");
        let book_date_out = first.get::<NaiveDateTime, _>("Book_Date_out");
        let book_cust_id = first.get::<&str, _>("Book_Cust_ID").unwrap_or_default().to_string();
        let book_status = first.get::<i32, _>("Book_Status");

        // Find customer ID in PG
        let pg_cust_id = customer_map.get(&book_cust_id);
        if pg_cust_id.is_none() {
            tracing::warn!("  Skipping booking {} - customer {} not found", book_no, book_cust_id);
            stats.bookings_skipped += 1;
            continue;
        }
        let pg_cust_id = *pg_cust_id.unwrap();

        // Need NaiveDate for book_checkin/book_checkout
        let checkin_date = book_date_in.map(|dt| dt.date());
        let checkout_date = book_date_out.map(|dt| dt.date());

        if checkin_date.is_none() || checkout_date.is_none() {
            tracing::warn!("  Skipping booking {} - missing dates", book_no);
            stats.bookings_skipped += 1;
            continue;
        }

        let status = map_book_status(book_status);

        let book_id: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ht_bookings
                (book_no, book_date, book_cust_id, book_checkin, book_checkout,
                 book_status, source)
            VALUES ($1, COALESCE($2, NOW()::timestamp), $3, $4, $5, $6, 'legacy')
            RETURNING book_id
            "#,
        )
        .bind(book_no)
        .bind(&book_date)
        .bind(&pg_cust_id)
        .bind(&checkin_date.unwrap())
        .bind(&checkout_date.unwrap())
        .bind(status)
        .fetch_one(&mut *tx)
        .await?;

        stats.bookings += 1;

        // Create booking-room records for each row in the group
        for r in rows {
            let room_type = r.get::<&str, _>("Book_Room_Type").unwrap_or_default().to_string();
            let room_no = r.get::<&str, _>("Book_Room_No").map(String::from);

            let room_type_id = room_type_map.get(&room_type).copied();
            let room_id = room_no
                .as_ref()
                .and_then(|no| room_map.get(no))
                .copied();

            // Only create booking_room if we have a room
            if let Some(room_id) = room_id {
                let _ = sqlx::query(
                    r#"
                    INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_room_type_id)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (br_book_id, br_room_id) DO NOTHING
                    "#,
                )
                .bind(&book_id)
                .bind(&room_id)
                .bind(&room_type_id)
                .execute(&mut *tx)
                .await?;

                stats.booking_rooms += 1;
            }
        }
    }

    tracing::info!(
        "  {} bookings imported ({} rooms), {} skipped",
        stats.bookings, stats.booking_rooms, stats.bookings_skipped
    );
    Ok(())
}

// =============================================================================
// Step 5: Check-ins
// =============================================================================

async fn import_checkins(
    legacy_pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    tx: &mut sqlx::PgConnection,
    customer_map: &HashMap<String, i32>,
    room_map: &HashMap<String, i32>,
    stats: &mut MigrationStats,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Cin_no, Cin_Room_No, Cin_Room_In, Cin_Room_Out,
                Cin_cust_name, Cin_cust_no, Cin_status
            FROM View_CheckIn_Ds
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    // Check existing check-ins
    let existing_checkins: Vec<String> = sqlx::query_scalar(
        "SELECT cin_no FROM ht_checkins"
    )
    .fetch_all(&mut *tx)
    .await?;

    let existing_set: std::collections::HashSet<String> = existing_checkins.into_iter().collect();

    for row in &rows {
        let cin_no = row.get::<&str, _>("Cin_no").unwrap_or_default().to_string();
        if cin_no.is_empty() || existing_set.contains(&cin_no) {
            stats.checkins_skipped += 1;
            continue;
        }

        let room_no = row.get::<&str, _>("Cin_Room_No").unwrap_or_default().to_string();
        let cin_room_in = row.get::<NaiveDateTime, _>("Cin_Room_In");
        let cin_room_out = row.get::<NaiveDateTime, _>("Cin_Room_Out");
        let cust_no = row.get::<&str, _>("Cin_cust_no").unwrap_or_default().to_string();

        let pg_cust_id = customer_map.get(&cust_no);
        let pg_room_id = room_map.get(&room_no);

        if pg_cust_id.is_none() || pg_room_id.is_none() {
            tracing::debug!(
                "  Skipping check-in {} - customer {} or room {} not found",
                cin_no, cust_no, room_no
            );
            stats.checkins_skipped += 1;
            continue;
        }

        let pg_cust_id = *pg_cust_id.unwrap();
        let pg_room_id = *pg_room_id.unwrap();

        let checkin_time = cin_room_in.unwrap_or_else(|| chrono::Utc::now().naive_utc());
        let expected_checkout = cin_room_out
            .map(|dt| dt.date())
            .unwrap_or_else(|| (chrono::Utc::now() + chrono::Duration::days(1)).naive_utc().date());

        // Determine status
        let status = if cin_room_out.is_some() {
            "completed"
        } else {
            "active"
        };

        sqlx::query(
            r#"
            INSERT INTO ht_checkins
                (cin_no, cin_cust_id, cin_room_id, cin_checkin_time,
                 cin_checkout_time, cin_expected_checkout, cin_status, source)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'legacy')
            "#,
        )
        .bind(&cin_no)
        .bind(&pg_cust_id)
        .bind(&pg_room_id)
        .bind(&checkin_time)
        .bind(&cin_room_out)
        .bind(&expected_checkout)
        .bind(status)
        .execute(&mut *tx)
        .await?;

        stats.checkins += 1;
    }

    tracing::info!("  {} check-ins imported, {} skipped", stats.checkins, stats.checkins_skipped);
    Ok(())
}

// =============================================================================
// Step 6: Bump Sequences
// =============================================================================

async fn bump_sequences(
    tx: &mut sqlx::PgConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Bump booking sequence
    let max_book: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(book_id) FROM ht_bookings"
    )
    .fetch_one(&mut *tx)
    .await?;

    if let Some(max_id) = max_book {
        sqlx::query(&format!("SELECT setval('ht_bookings_book_id_seq', {})", max_id + 1))
            .execute(&mut *tx)
            .await?;
        tracing::info!("  Booking sequence bumped to {}", max_id + 1);
    }

    // Bump checkin sequence
    let max_cin: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(cin_id) FROM ht_checkins"
    )
    .fetch_one(&mut *tx)
    .await?;

    if let Some(max_id) = max_cin {
        sqlx::query(&format!("SELECT setval('ht_checkins_cin_id_seq', {})", max_id + 1))
            .execute(&mut *tx)
            .await?;
        tracing::info!("  Check-in sequence bumped to {}", max_id + 1);
    }

    // Bump customer sequence
    let max_cust: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(cust_id) FROM ht_customers"
    )
    .fetch_one(&mut *tx)
    .await?;

    if let Some(max_id) = max_cust {
        sqlx::query(&format!("SELECT setval('ht_customers_cust_id_seq', {})", max_id + 1))
            .execute(&mut *tx)
            .await?;
        tracing::info!("  Customer sequence bumped to {}", max_id + 1);
    }

    // Bump room sequence
    let max_room: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(room_id) FROM ht_rooms_new"
    )
    .fetch_one(&mut *tx)
    .await?;

    if let Some(max_id) = max_room {
        sqlx::query(&format!("SELECT setval('ht_rooms_new_room_id_seq', {})", max_id + 1))
            .execute(&mut *tx)
            .await?;
        tracing::info!("  Room sequence bumped to {}", max_id + 1);
    }

    Ok(())
}

// =============================================================================
// Database Connection Helpers
// =============================================================================

async fn create_legacy_pool() -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, Box<dyn std::error::Error>> {
    let server = std::env::var("DB_SERVER").unwrap_or_else(|_| "192.168.100.222".to_string());
    let database = std::env::var("DB_NAME").unwrap_or_else(|_| "db".to_string());
    let user = std::env::var("DB_USER").unwrap_or_else(|_| "sa".to_string());
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string());

    let mut config = tiberius::Config::new();
    config.host(&server);
    config.port(1433);
    config.database(&database);
    config.authentication(tiberius::AuthMethod::sql_server(&user, &password));
    config.trust_cert();

    let manager = bb8_tiberius::ConnectionManager::new(config);
    let pool = bb8::Pool::builder()
        .max_size(5)
        .build(manager)
        .await?;

    // Test connection
    {
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
    }
    tracing::info!("Connected to legacy SQL Server at {}", server);

    Ok(pool)
}

async fn create_pg_pool() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let server = std::env::var("NEW_DB_SERVER").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("NEW_DB_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5439);
    let database = std::env::var("NEW_DB_NAME").unwrap_or_else(|_| "hotelnew".to_string());
    let user = std::env::var("NEW_DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("NEW_DB_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string());

    let conn_str = format!("postgres://{}:{}@{}:{}/{}", user, password, server, port, database);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await?;

    tracing::info!("Connected to PostgreSQL at {}:{}", server, port);
    Ok(pool)
}
