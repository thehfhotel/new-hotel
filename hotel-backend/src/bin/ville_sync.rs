//! HF Ville Sync Binary
//!
//! Syncs data from HF Ville SQL Server 2005 (192.168.11.51) to a local
//! PostgreSQL mirror database. Runs continuously with configurable interval
//! (default 90 seconds).
//!
//! Syncs 4 entity types in order:
//! 1. Customers (View_Customers -> ht_customers_legacy)
//! 2. Rooms (HT_Rooms -> ht_rooms_legacy)
//! 3. Bookings (View_Booking_Ds -> ht_bookings_legacy)
//! 4. Check-ins (View_CheckIn_Ds -> ht_checkins_legacy)
//!
//! Uses SHA256 hash-based change detection (identical to scheduler/sync.rs).

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Instant;

type DbPool = Pool<ConnectionManager>;
type PgPool = sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ville_sync=info".into()),
        )
        .init();

    tracing::info!("HF Ville Sync starting...");

    // Load config from env
    let mssql_server = env::var("MSSQL_SERVER").unwrap_or_else(|_| "192.168.11.51".to_string());
    let mssql_database = env::var("MSSQL_DATABASE").unwrap_or_else(|_| "hotel".to_string());
    let mssql_user = env::var("MSSQL_USER").unwrap_or_else(|_| "sa".to_string());
    let mssql_password = env::var("MSSQL_PASSWORD").unwrap_or_else(|_| "12345678".to_string());

    let pg_server = env::var("PG_SERVER").unwrap_or_else(|_| "localhost".to_string());
    let pg_port = env::var("PG_PORT").unwrap_or_else(|_| "5440".to_string());
    let pg_database = env::var("PG_DATABASE").unwrap_or_else(|_| "hfville".to_string());
    let pg_user = env::var("PG_USER").unwrap_or_else(|_| "postgres".to_string());
    let pg_password = env::var("PG_PASSWORD").unwrap_or_else(|_| "HfVille@2026!".to_string());

    let sync_interval: u64 = env::var("SYNC_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(90);

    tracing::info!("MSSQL: {}@{}/{}", mssql_user, mssql_server, mssql_database);
    tracing::info!("PG: {}@{}:{}/{}", pg_user, pg_server, pg_port, pg_database);
    tracing::info!("Sync interval: {}s", sync_interval);

    // Create SQL Server pool
    // SQL Server 2005 at HF Ville may not support TLS — disable encryption
    let mut tib_config = tiberius::Config::new();
    tib_config.host(&mssql_server);
    tib_config.port(1433);
    tib_config.database(&mssql_database);
    tib_config.authentication(tiberius::AuthMethod::sql_server(&mssql_user, &mssql_password));
    tib_config.trust_cert();
    tib_config.encryption(tiberius::EncryptionLevel::Off);

    let manager = ConnectionManager::new(tib_config);
    tracing::info!("Connecting to MSSQL...");
    let mssql_pool = Pool::builder()
        .max_size(3)
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(manager)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create MSSQL pool: {}", e);
            e
        })?;

    // Test MSSQL connection
    {
        let mut conn = mssql_pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
        tracing::info!("MSSQL connection established to {}", mssql_server);
    }

    // Create PostgreSQL pool
    let pg_conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_user, pg_password, pg_server, pg_port, pg_database
    );
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_conn_str)
        .await?;

    // Test PG connection
    sqlx::query("SELECT 1").execute(&pg_pool).await?;
    tracing::info!("PostgreSQL connection established to {}:{}", pg_server, pg_port);

    // Main sync loop
    loop {
        tracing::info!("[Sync] Starting sync cycle...");

        if let Err(e) = sync_customers(&mssql_pool, &pg_pool).await {
            tracing::error!("[Sync] Customer sync failed: {}", e);
            record_error(&pg_pool, "customers", &e.to_string()).await;
        }

        if let Err(e) = sync_rooms(&mssql_pool, &pg_pool).await {
            tracing::error!("[Sync] Room sync failed: {}", e);
            record_error(&pg_pool, "rooms", &e.to_string()).await;
        }

        if let Err(e) = sync_bookings(&mssql_pool, &pg_pool).await {
            tracing::error!("[Sync] Booking sync failed: {}", e);
            record_error(&pg_pool, "bookings", &e.to_string()).await;
        }

        if let Err(e) = sync_checkins(&mssql_pool, &pg_pool).await {
            tracing::error!("[Sync] Check-in sync failed: {}", e);
            record_error(&pg_pool, "checkins", &e.to_string()).await;
        }

        tracing::info!("[Sync] Sync cycle complete. Sleeping {}s...", sync_interval);
        tokio::time::sleep(tokio::time::Duration::from_secs(sync_interval)).await;
    }
}

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

async fn record_error(pg_pool: &PgPool, entity: &str, error: &str) {
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_error = $1,
            last_error_at = NOW(),
            consecutive_failures = consecutive_failures + 1
        WHERE entity_type = $2
        "#,
    )
    .bind(error)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

async fn record_success(
    pg_pool: &PgPool,
    entity: &str,
    added: i32,
    updated: i32,
    unchanged: i32,
    duration_ms: i32,
) {
    let total = added + updated + unchanged;
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_sync_at = NOW(),
            records_synced = $1,
            records_added = $2,
            records_updated = $3,
            records_unchanged = $4,
            sync_duration_ms = $5,
            consecutive_failures = 0
        WHERE entity_type = $6
        "#,
    )
    .bind(total)
    .bind(added)
    .bind(updated)
    .bind(unchanged)
    .bind(duration_ms)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

// =============================================================================
// Customer Sync
// =============================================================================

async fn sync_customers(
    mssql_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing customers...");

    let mut conn = mssql_pool.get().await?;
    let rows = conn
        .simple_query(
            "SELECT Cust_no, Cust_name, Cust_Type, Cust_Add_tel, Cust_IDcard, C_Address FROM View_Customers",
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cust_no = row.get::<&str, _>("Cust_no").unwrap_or_default().to_string();
        let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
        let cust_type = row.get::<&str, _>("Cust_Type").map(String::from);
        let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
        let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
        let cust_address = row.get::<&str, _>("C_Address").map(String::from);

        let hash_input = format!(
            "{}|{}|{}|{}|{}|{}",
            cust_no,
            cust_name.as_deref().unwrap_or(""),
            cust_type.as_deref().unwrap_or(""),
            cust_phone.as_deref().unwrap_or(""),
            cust_idcard.as_deref().unwrap_or(""),
            cust_address.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

        let existing: Option<Option<String>> = sqlx::query_scalar(
            "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1"
        )
        .bind(&cust_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(ref existing_hash)) if existing_hash == &hash => {
                unchanged += 1;
            }
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE ht_customers_legacy
                    SET cust_name = $1, cust_type = $2, cust_phone = $3,
                        cust_idcard = $4, cust_address = $5,
                        sync_hash = $6, synced_at = NOW()
                    WHERE cust_no = $7
                    "#,
                )
                .bind(&cust_name)
                .bind(&cust_type)
                .bind(&cust_phone)
                .bind(&cust_idcard)
                .bind(&cust_address)
                .bind(&hash)
                .bind(&cust_no)
                .execute(pg_pool)
                .await?;
                updated += 1;
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO ht_customers_legacy
                        (cust_no, cust_name, cust_type, cust_phone, cust_idcard, cust_address, sync_hash)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                )
                .bind(&cust_no)
                .bind(&cust_name)
                .bind(&cust_type)
                .bind(&cust_phone)
                .bind(&cust_idcard)
                .bind(&cust_address)
                .bind(&hash)
                .execute(pg_pool)
                .await?;
                added += 1;
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Customers: {} added, {} updated, {} unchanged in {}ms",
        added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "customers", added, updated, unchanged, duration_ms).await;
    Ok(())
}

// =============================================================================
// Room Sync
// =============================================================================

async fn sync_rooms(
    mssql_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing rooms...");

    let mut conn = mssql_pool.get().await?;
    let rows = conn
        .simple_query(
            r#"
            SELECT Room_no, Room_Type, Room_Details, Room_Clean, Room_Use, Room_Book,
                   Room_Manternace, Room_PriceA, Room_PriceB, Room_PriceC,
                   Room_Group, Room_Book_Name, Room_Book_Time
            FROM HT_Rooms ORDER BY Room_no
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let room_no = row.get::<&str, _>("Room_no").unwrap_or_default().to_string();
        let room_type = row.get::<&str, _>("Room_Type").map(String::from);
        let room_details = row.get::<&str, _>("Room_Details").map(String::from);
        let room_clean = row.get::<&str, _>("Room_Clean").map(String::from);
        let room_use = row.get::<&str, _>("Room_Use").map(String::from);
        let room_book = row.get::<&str, _>("Room_Book").map(String::from);
        let room_manternace = row.get::<&str, _>("Room_Manternace").map(String::from);
        let room_price_a = row.get::<f64, _>("Room_PriceA");
        let room_price_b = row.get::<f64, _>("Room_PriceB");
        let room_price_c = row.get::<f64, _>("Room_PriceC");
        let room_group = row.get::<&str, _>("Room_Group").map(String::from);
        let room_book_name = row.get::<&str, _>("Room_Book_Name").map(String::from);
        let room_book_time: Option<NaiveDateTime> = row.try_get("Room_Book_Time").unwrap_or(None);

        let hash_input = format!(
            "{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}|{}|{}|{:?}",
            room_no,
            room_type.as_deref().unwrap_or(""),
            room_details.as_deref().unwrap_or(""),
            room_clean.as_deref().unwrap_or(""),
            room_use.as_deref().unwrap_or(""),
            room_book.as_deref().unwrap_or(""),
            room_manternace.as_deref().unwrap_or(""),
            room_price_a,
            room_price_b,
            room_price_c,
            room_group.as_deref().unwrap_or(""),
            room_book_name.as_deref().unwrap_or(""),
            room_book_time,
        );
        let hash = sha256(&hash_input);

        let existing: Option<Option<String>> = sqlx::query_scalar(
            "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1"
        )
        .bind(&room_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(ref existing_hash)) if existing_hash == &hash => {
                unchanged += 1;
            }
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE ht_rooms_legacy
                    SET room_type = $1, room_details = $2, room_clean = $3,
                        room_use = $4, room_book = $5, room_manternace = $6,
                        room_price_a = $7::float8, room_price_b = $8::float8,
                        room_price_c = $9::float8, room_group = $10,
                        room_book_name = $11, room_book_time = $12,
                        sync_hash = $13, synced_at = NOW()
                    WHERE room_no = $14
                    "#,
                )
                .bind(&room_type)
                .bind(&room_details)
                .bind(&room_clean)
                .bind(&room_use)
                .bind(&room_book)
                .bind(&room_manternace)
                .bind(&room_price_a)
                .bind(&room_price_b)
                .bind(&room_price_c)
                .bind(&room_group)
                .bind(&room_book_name)
                .bind(&room_book_time)
                .bind(&hash)
                .bind(&room_no)
                .execute(pg_pool)
                .await?;
                updated += 1;
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO ht_rooms_legacy
                        (room_no, room_type, room_details, room_clean, room_use,
                         room_book, room_manternace, room_price_a, room_price_b,
                         room_price_c, room_group, room_book_name, room_book_time, sync_hash)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8::float8, $9::float8,
                            $10::float8, $11, $12, $13, $14)
                    "#,
                )
                .bind(&room_no)
                .bind(&room_type)
                .bind(&room_details)
                .bind(&room_clean)
                .bind(&room_use)
                .bind(&room_book)
                .bind(&room_manternace)
                .bind(&room_price_a)
                .bind(&room_price_b)
                .bind(&room_price_c)
                .bind(&room_group)
                .bind(&room_book_name)
                .bind(&room_book_time)
                .bind(&hash)
                .execute(pg_pool)
                .await?;
                added += 1;
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Rooms: {} added, {} updated, {} unchanged in {}ms",
        added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "rooms", added, updated, unchanged, duration_ms).await;
    Ok(())
}

// =============================================================================
// Booking Sync
// =============================================================================

async fn sync_bookings(
    mssql_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing bookings...");

    let mut conn = mssql_pool.get().await?;
    let rows = conn
        .simple_query(
            "SELECT Book_No, Book_Date, Book_Date_in, Book_Date_out, Book_Cust_Name, Book_Cust_ID, Book_Status, Book_Room_Type FROM View_Booking_Ds",
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
        let book_date: Option<NaiveDateTime> = row.try_get("Book_Date").unwrap_or(None);
        let book_date_in: Option<NaiveDateTime> = row.try_get("Book_Date_in").unwrap_or(None);
        let book_date_out: Option<NaiveDateTime> = row.try_get("Book_Date_out").unwrap_or(None);
        let book_cust_name = row.get::<&str, _>("Book_Cust_Name").map(String::from);
        let book_cust_id = row.get::<&str, _>("Book_Cust_ID").map(String::from);
        let book_status = row.get::<i32, _>("Book_Status");
        let book_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);

        let hash_input = format!(
            "{}|{:?}|{:?}|{:?}|{}|{}|{:?}|{}",
            book_no,
            book_date,
            book_date_in,
            book_date_out,
            book_cust_name.as_deref().unwrap_or(""),
            book_cust_id.as_deref().unwrap_or(""),
            book_status,
            book_room_type.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

        let room_type_key = book_room_type.as_deref().unwrap_or("");

        let existing: Option<Option<String>> = sqlx::query_scalar(
            "SELECT sync_hash FROM ht_bookings_legacy WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2"
        )
        .bind(&book_no)
        .bind(room_type_key)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(ref existing_hash)) if existing_hash == &hash => {
                unchanged += 1;
            }
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE ht_bookings_legacy
                    SET book_date = $1, book_date_in = $2, book_date_out = $3,
                        book_cust_name = $4, book_cust_id = $5, book_status = $6,
                        sync_hash = $7, synced_at = NOW()
                    WHERE book_no = $8 AND COALESCE(book_room_type, '') = $9
                    "#,
                )
                .bind(&book_date)
                .bind(&book_date_in)
                .bind(&book_date_out)
                .bind(&book_cust_name)
                .bind(&book_cust_id)
                .bind(&book_status)
                .bind(&hash)
                .bind(&book_no)
                .bind(room_type_key)
                .execute(pg_pool)
                .await?;
                updated += 1;
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO ht_bookings_legacy
                        (book_no, book_date, book_date_in, book_date_out,
                         book_cust_name, book_cust_id, book_status,
                         book_room_type, sync_hash)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#,
                )
                .bind(&book_no)
                .bind(&book_date)
                .bind(&book_date_in)
                .bind(&book_date_out)
                .bind(&book_cust_name)
                .bind(&book_cust_id)
                .bind(&book_status)
                .bind(&book_room_type)
                .bind(&hash)
                .execute(pg_pool)
                .await?;
                added += 1;
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Bookings: {} added, {} updated, {} unchanged in {}ms",
        added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "bookings", added, updated, unchanged, duration_ms).await;
    Ok(())
}

// =============================================================================
// Check-in Sync
// =============================================================================

async fn sync_checkins(
    mssql_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing check-ins...");

    let mut conn = mssql_pool.get().await?;
    let rows = conn
        .simple_query(
            "SELECT Cin_no, Cin_Room_No, Cin_Room_In, Cin_Room_Out, Cin_cust_name, Cin_cust_no, Cin_status FROM View_CheckIn_Ds",
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cin_no = row.get::<&str, _>("Cin_no").unwrap_or_default().to_string();
        let cin_room_no = row.get::<&str, _>("Cin_Room_No").map(String::from);
        let cin_room_in: Option<NaiveDateTime> = row.try_get("Cin_Room_In").unwrap_or(None);
        let cin_room_out: Option<NaiveDateTime> = row.try_get("Cin_Room_Out").unwrap_or(None);
        let cin_cust_name = row.get::<&str, _>("Cin_cust_name").map(String::from);
        let cin_cust_no = row.get::<&str, _>("Cin_cust_no").map(String::from);
        let cin_status = row.get::<&str, _>("Cin_status").map(String::from);

        let hash_input = format!(
            "{}|{}|{:?}|{:?}|{}|{}|{}",
            cin_no,
            cin_room_no.as_deref().unwrap_or(""),
            cin_room_in,
            cin_room_out,
            cin_cust_name.as_deref().unwrap_or(""),
            cin_cust_no.as_deref().unwrap_or(""),
            cin_status.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

        let existing: Option<Option<String>> = sqlx::query_scalar(
            "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1"
        )
        .bind(&cin_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(ref existing_hash)) if existing_hash == &hash => {
                unchanged += 1;
            }
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE ht_checkins_legacy
                    SET cin_room_no = $1, cin_room_in = $2, cin_room_out = $3,
                        cin_cust_name = $4, cin_cust_no = $5, cin_status = $6,
                        sync_hash = $7, synced_at = NOW()
                    WHERE cin_no = $8
                    "#,
                )
                .bind(&cin_room_no)
                .bind(&cin_room_in)
                .bind(&cin_room_out)
                .bind(&cin_cust_name)
                .bind(&cin_cust_no)
                .bind(&cin_status)
                .bind(&hash)
                .bind(&cin_no)
                .execute(pg_pool)
                .await?;
                updated += 1;
            }
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO ht_checkins_legacy
                        (cin_no, cin_room_no, cin_room_in, cin_room_out,
                         cin_cust_name, cin_cust_no, cin_status, sync_hash)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#,
                )
                .bind(&cin_no)
                .bind(&cin_room_no)
                .bind(&cin_room_in)
                .bind(&cin_room_out)
                .bind(&cin_cust_name)
                .bind(&cin_cust_no)
                .bind(&cin_status)
                .bind(&hash)
                .execute(pg_pool)
                .await?;
                added += 1;
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Check-ins: {} added, {} updated, {} unchanged in {}ms",
        added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "checkins", added, updated, unchanged, duration_ms).await;
    Ok(())
}
