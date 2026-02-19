//! HF Ville Sync Binary
//!
//! Syncs data from HF Ville SQL Server (192.168.11.51) to a local
//! PostgreSQL mirror database. Runs continuously with configurable interval
//! (default 90 seconds).
//!
//! Uses FreeTDS `tsql` subprocess for MSSQL queries (TDS 7.0 required
//! by the legacy SQL Server — tiberius only supports TDS 7.2+).
//!
//! Syncs 4 entity types in order:
//! 1. Customers (View_Customers -> ht_customers_legacy)
//! 2. Rooms (HT_Rooms -> ht_rooms_legacy)
//! 3. Bookings (View_Booking_Ds -> ht_bookings_legacy)
//! 4. Check-ins (View_CheckIn_Ds -> ht_checkins_legacy)
//!
//! Uses SHA256 hash-based change detection (identical to scheduler/sync.rs).

use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::process::Command;
use std::time::Instant;

type PgPool = sqlx::PgPool;

/// MSSQL connection config for FreeTDS
struct MssqlConfig {
    server: String,
    port: u16,
    database: String,
    user: String,
    password: String,
}

/// Run a SQL query against MSSQL via FreeTDS tsql subprocess.
/// Returns rows as Vec<Vec<String>> (tab-separated columns).
fn run_mssql_query(
    config: &MssqlConfig,
    sql: &str,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error + Send + Sync>> {
    let input = format!("{}\nGO\nQUIT\n", sql);

    let output = Command::new("tsql")
        .args([
            "-H", &config.server,
            "-p", &config.port.to_string(),
            "-U", &config.user,
            "-P", &config.password,
            "-D", &config.database,
        ])
        .env("TDSVER", "7.0")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // tsql prints connection info to stderr; check for actual errors
        if stderr.contains("There was a problem connecting")
            || stderr.contains("Error ")
        {
            return Err(format!("tsql failed: {}", stderr.trim()).into());
        }
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut rows = Vec::new();
    let mut is_first_data_row = true;

    for line in stdout.lines() {
        let trimmed = line.trim();
        // Skip empty lines and "affected" summary lines
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.starts_with("return status") {
            continue;
        }
        // Skip column header row (first non-empty line from tsql)
        if is_first_data_row {
            is_first_data_row = false;
            continue;
        }
        let cols: Vec<String> = line.split('\t').map(|s| s.trim().to_string()).collect();
        rows.push(cols);
    }

    Ok(rows)
}

/// Parse a date string from MSSQL (format varies: "Jan 22 2026 11:59AM", "2026-01-22 11:59:00.000")
fn parse_datetime(s: &str) -> Option<NaiveDateTime> {
    let s = s.trim();
    if s.is_empty() || s == "NULL" {
        return None;
    }
    // Try ISO format first
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(dt);
    }
    // Try MSSQL US format: "Jan 22 2026 11:59AM"
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%b %d %Y %I:%M%p") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%b %e %Y %I:%M%p") {
        return Some(dt);
    }
    // Try "Jan 22 2026 11:59:00:000AM"
    if let Some(base) = s.split(':').take(2).collect::<Vec<_>>().join(":").chars().take(20).collect::<String>().strip_suffix("M").and_then(|_| None::<NaiveDateTime>) {
        let _ = base;
    }
    tracing::warn!("Failed to parse datetime: '{}'", s);
    None
}

fn get_col(row: &[String], idx: usize) -> String {
    row.get(idx)
        .map(|s| {
            let s = s.trim();
            if s == "NULL" { String::new() } else { s.to_string() }
        })
        .unwrap_or_default()
}

fn get_col_opt(row: &[String], idx: usize) -> Option<String> {
    row.get(idx).and_then(|s| {
        let s = s.trim();
        if s.is_empty() || s == "NULL" { None } else { Some(s.to_string()) }
    })
}

fn get_col_f64(row: &[String], idx: usize) -> Option<f64> {
    row.get(idx).and_then(|s| {
        let s = s.trim();
        if s.is_empty() || s == "NULL" { None } else { s.parse().ok() }
    })
}

fn get_col_i32(row: &[String], idx: usize) -> Option<i32> {
    row.get(idx).and_then(|s| {
        let s = s.trim();
        if s.is_empty() || s == "NULL" { None } else { s.parse().ok() }
    })
}

fn get_col_datetime(row: &[String], idx: usize) -> Option<NaiveDateTime> {
    row.get(idx).and_then(|s| parse_datetime(s))
}

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
    let mssql_config = MssqlConfig {
        server: env::var("MSSQL_SERVER").unwrap_or_else(|_| "192.168.11.51".to_string()),
        port: env::var("MSSQL_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(1433),
        database: env::var("MSSQL_DATABASE").unwrap_or_else(|_| "hotel".to_string()),
        user: env::var("MSSQL_USER").unwrap_or_else(|_| "sa".to_string()),
        password: env::var("MSSQL_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string()),
    };

    let pg_server = env::var("PG_SERVER").unwrap_or_else(|_| "localhost".to_string());
    let pg_port = env::var("PG_PORT").unwrap_or_else(|_| "5440".to_string());
    let pg_database = env::var("PG_DATABASE").unwrap_or_else(|_| "hfville".to_string());
    let pg_user = env::var("PG_USER").unwrap_or_else(|_| "postgres".to_string());
    let pg_password = env::var("PG_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string());

    let sync_interval: u64 = env::var("SYNC_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(90);

    tracing::info!("MSSQL: {}@{}:{}/{} (via tsql TDS 7.0)", mssql_config.user, mssql_config.server, mssql_config.port, mssql_config.database);
    tracing::info!("PG: {}@{}:{}/{}", pg_user, pg_server, pg_port, pg_database);
    tracing::info!("Sync interval: {}s", sync_interval);

    // Test MSSQL connection via tsql
    tracing::info!("Testing MSSQL connection...");
    match run_mssql_query(&mssql_config, "SELECT 1") {
        Ok(_) => tracing::info!("MSSQL connection OK"),
        Err(e) => {
            tracing::error!("MSSQL connection failed: {}", e);
            return Err(format!("MSSQL connection failed: {}", e).into());
        }
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

        if let Err(e) = sync_customers(&mssql_config, &pg_pool).await {
            tracing::error!("[Sync] Customer sync failed: {}", e);
            record_error(&pg_pool, "customers", &e.to_string()).await;
        }

        if let Err(e) = sync_rooms(&mssql_config, &pg_pool).await {
            tracing::error!("[Sync] Room sync failed: {}", e);
            record_error(&pg_pool, "rooms", &e.to_string()).await;
        }

        if let Err(e) = sync_bookings(&mssql_config, &pg_pool).await {
            tracing::error!("[Sync] Booking sync failed: {}", e);
            record_error(&pg_pool, "bookings", &e.to_string()).await;
        }

        if let Err(e) = sync_checkins(&mssql_config, &pg_pool).await {
            tracing::error!("[Sync] Check-in sync failed: {}", e);
            record_error(&pg_pool, "checkins", &e.to_string()).await;
        }

        tracing::info!("[Sync] Sync cycle complete. Sleeping {}s...", sync_interval);
        tokio::time::sleep(tokio::time::Duration::from_secs(sync_interval)).await;
    }
}

fn sha256_hash(input: &str) -> String {
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
    mssql: &MssqlConfig,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing customers...");

    let rows = run_mssql_query(
        mssql,
        "SELECT Cust_no, Cust_name, Cust_Type, Cust_Add_tel, Cust_IDcard, C_Address FROM View_Customers",
    )?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cust_no = get_col(row, 0);
        if cust_no.is_empty() { continue; }
        let cust_name = get_col_opt(row, 1);
        let cust_type = get_col_opt(row, 2);
        let cust_phone = get_col_opt(row, 3);
        let cust_idcard = get_col_opt(row, 4);
        let cust_address = get_col_opt(row, 5);

        let hash_input = format!(
            "{}|{}|{}|{}|{}|{}",
            cust_no,
            cust_name.as_deref().unwrap_or(""),
            cust_type.as_deref().unwrap_or(""),
            cust_phone.as_deref().unwrap_or(""),
            cust_idcard.as_deref().unwrap_or(""),
            cust_address.as_deref().unwrap_or(""),
        );
        let hash = sha256_hash(&hash_input);

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
    mssql: &MssqlConfig,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing rooms...");

    let rows = run_mssql_query(
        mssql,
        "SELECT Room_no, Room_Type, Room_Details, Room_Clean, Room_Use, Room_Book, Room_Manternace, Room_PriceA, Room_PriceB, Room_PriceC, Room_Group, Room_Book_Name, Room_Book_Time FROM HT_Rooms ORDER BY Room_no",
    )?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let room_no = get_col(row, 0);
        if room_no.is_empty() { continue; }
        let room_type = get_col_opt(row, 1);
        let room_details = get_col_opt(row, 2);
        let room_clean = get_col_opt(row, 3);
        let room_use = get_col_opt(row, 4);
        let room_book = get_col_opt(row, 5);
        let room_manternace = get_col_opt(row, 6);
        let room_price_a = get_col_f64(row, 7);
        let room_price_b = get_col_f64(row, 8);
        let room_price_c = get_col_f64(row, 9);
        let room_group = get_col_opt(row, 10);
        let room_book_name = get_col_opt(row, 11);
        let room_book_time = get_col_datetime(row, 12);

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
        let hash = sha256_hash(&hash_input);

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
    mssql: &MssqlConfig,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing bookings...");

    let rows = run_mssql_query(
        mssql,
        "SELECT Book_No, Book_Date, Book_Date_in, Book_Date_out, Book_Cust_Name, Book_Cust_ID, Book_Status, Book_Room_Type FROM View_Booking_Ds",
    )?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let book_no = get_col(row, 0);
        if book_no.is_empty() { continue; }
        let book_date = get_col_datetime(row, 1);
        let book_date_in = get_col_datetime(row, 2);
        let book_date_out = get_col_datetime(row, 3);
        let book_cust_name = get_col_opt(row, 4);
        let book_cust_id = get_col_opt(row, 5);
        let book_status = get_col_i32(row, 6);
        let book_room_type = get_col_opt(row, 7);

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
        let hash = sha256_hash(&hash_input);

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
    mssql: &MssqlConfig,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing check-ins...");

    let rows = run_mssql_query(
        mssql,
        "SELECT Cin_no, Cin_Room_No, Cin_Room_In, Cin_Room_Out, Cin_cust_name, Cin_cust_no, Cin_status FROM View_CheckIn_Ds",
    )?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cin_no = get_col(row, 0);
        if cin_no.is_empty() { continue; }
        let cin_room_no = get_col_opt(row, 1);
        let cin_room_in = get_col_datetime(row, 2);
        let cin_room_out = get_col_datetime(row, 3);
        let cin_cust_name = get_col_opt(row, 4);
        let cin_cust_no = get_col_opt(row, 5);
        let cin_status = get_col_opt(row, 6);

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
        let hash = sha256_hash(&hash_input);

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
