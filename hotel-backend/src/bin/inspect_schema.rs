//! One-shot debug binary: dumps MSSQL column definitions for the legacy
//! tables we INSERT into. Used to verify our recipe column lists match the
//! legacy schema (audit batch 1, 2026-04-26).
//!
//! Reads from `INFORMATION_SCHEMA.COLUMNS` + `sys.columns` (for `is_identity`
//! and computed-column flags that INFORMATION_SCHEMA doesn't expose).
//!
//! Usage:
//!   docker compose --profile legacy run --rm \
//!     --entrypoint /app/inspect_schema writeback \
//!     HT_Customers HT_CheckIn_Ds HT_CheckIn_Pay HT_Receipt_H HT_Book_Ds
//!
//! Output is one block per table:
//!   === HT_Customers ===
//!     id             int          NOT NULL  IDENTITY  default=NULL
//!     Cust_no        varchar(20)  NOT NULL            default=NULL
//!     ...

use hotel_backend::config::DbConfig;
use hotel_backend::db::create_pool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Security audit 2026-05-14: hydrate Docker secret files into env vars
    // before any consumer reads them. See `hotel_backend::secrets`.
    hotel_backend::secrets::hydrate_env_from_secret_files();

    let tables: Vec<String> = env::args().skip(1).collect();
    if tables.is_empty() {
        eprintln!(
            "Usage: inspect_schema <table_name> [<table_name> ...]\n\
             Example: inspect_schema HT_Customers HT_CheckIn_Ds HT_CheckIn_Pay"
        );
        std::process::exit(2);
    }

    let cfg = DbConfig::from_env();
    let pool = create_pool(&cfg).await.map_err(|e| format!("pool: {e}"))?;
    let mut conn = pool.get().await?;

    for table in &tables {
        println!("\n=== {table} ===");
        // Single query joining INFORMATION_SCHEMA.COLUMNS with sys.columns to
        // get is_identity (which IS_C doesn't expose) + ordinal_position so
        // the dump matches the legacy app's INSERT column-order convention.
        let sql = format!(
            "SELECT \
                ic.COLUMN_NAME, \
                ic.DATA_TYPE, \
                ic.CHARACTER_MAXIMUM_LENGTH, \
                ic.IS_NULLABLE, \
                ic.COLUMN_DEFAULT, \
                CAST(sc.is_identity AS INT) AS is_identity, \
                CAST(sc.is_computed AS INT) AS is_computed, \
                ic.ORDINAL_POSITION \
             FROM INFORMATION_SCHEMA.COLUMNS ic \
             JOIN sys.columns sc \
               ON sc.object_id = OBJECT_ID(ic.TABLE_SCHEMA + '.' + ic.TABLE_NAME) \
              AND sc.name      = ic.COLUMN_NAME \
             WHERE ic.TABLE_NAME = '{table}' \
             ORDER BY ic.ORDINAL_POSITION"
        );
        let stream = conn.simple_query(sql).await?;
        let rows = stream.into_first_result().await?;
        if rows.is_empty() {
            println!("  (table not found or no columns)");
            continue;
        }
        for row in rows {
            let name = row.get::<&str, _>("COLUMN_NAME").unwrap_or("?");
            let dtype = row.get::<&str, _>("DATA_TYPE").unwrap_or("?");
            let len: Option<i32> = row.get("CHARACTER_MAXIMUM_LENGTH");
            let nullable = row.get::<&str, _>("IS_NULLABLE").unwrap_or("?");
            let default: Option<&str> = row.get("COLUMN_DEFAULT");
            let is_identity = row.get::<i32, _>("is_identity").unwrap_or(0) == 1;
            let is_computed = row.get::<i32, _>("is_computed").unwrap_or(0) == 1;
            let ordinal = row.get::<i32, _>("ORDINAL_POSITION").unwrap_or(0);

            let dtype_with_len = match len {
                Some(n) if n >= 0 => format!("{dtype}({n})"),
                Some(_) => format!("{dtype}(MAX)"),
                None => dtype.to_string(),
            };
            let null_marker = if nullable == "NO" { "NOT NULL" } else { "NULL    " };
            let identity_marker = if is_identity { "IDENTITY  " } else { "          " };
            let computed_marker = if is_computed { "COMPUTED  " } else { "          " };
            let default_marker = default.unwrap_or("NULL");

            println!(
                "  {ordinal:>3}  {name:<28}  {dtype_with_len:<18}  {null_marker}  {identity_marker}{computed_marker}default={default_marker}"
            );
        }
    }

    Ok(())
}
