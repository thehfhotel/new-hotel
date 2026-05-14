//! One-shot debug binary: dumps MSSQL HT_Book_H + HT_Book_Ds + HT_Book_Date
//! for a given Book_no. Used to verify what writeback actually emitted vs
//! what the .NET app sees.
//!
//! Usage: docker compose --profile legacy run --rm writeback /app/inspect_booking R014835

use hotel_backend::config::DbConfig;
use hotel_backend::db::create_pool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Security audit 2026-05-14: hydrate Docker secret files into env vars
    // before any consumer reads them. See `hotel_backend::secrets`.
    hotel_backend::secrets::hydrate_env_from_secret_files();

    let book_no = env::args().nth(1).expect("Usage: inspect_booking <Book_No>");
    println!("Inspecting Book_No={book_no}");

    let cfg = DbConfig::from_env();
    let pool = create_pool(&cfg).await.map_err(|e| format!("pool: {e}"))?;
    let mut conn = pool.get().await?;

    println!("\n=== HT_Book_H ===");
    let stream = conn
        .simple_query(format!(
            "SELECT Book_ID, Book_Date_in, Book_Date_out, Book_Status, book_room_type FROM HT_Book_H WHERE Book_ID='{book_no}'"
        ))
        .await?;
    for row in stream.into_first_result().await? {
        println!(
            "  Book_ID={:?} Book_Date_in={:?} Book_Date_out={:?} Book_Status={:?} book_room_type={:?}",
            row.get::<&str, _>("Book_ID"),
            row.get::<chrono::NaiveDateTime, _>("Book_Date_in"),
            row.get::<chrono::NaiveDateTime, _>("Book_Date_out"),
            row.get::<i32, _>("Book_Status"),
            row.get::<i32, _>("book_room_type"),
        );
    }

    println!("\n=== HT_Book_Ds ===");
    let stream = conn
        .simple_query(format!(
            "SELECT Book_No, Book_Room_Type, Book_Room_Start, Book_Room_End, Book_status FROM HT_Book_Ds WHERE Book_No='{book_no}'"
        ))
        .await?;
    for row in stream.into_first_result().await? {
        println!(
            "  Book_No={:?} Book_Room_Type={:?} Book_Room_Start={:?} Book_Room_End={:?} Book_status={:?}",
            row.get::<&str, _>("Book_No"),
            row.get::<&str, _>("Book_Room_Type"),
            row.get::<chrono::NaiveDateTime, _>("Book_Room_Start"),
            row.get::<chrono::NaiveDateTime, _>("Book_Room_End"),
            row.get::<i32, _>("Book_status"),
        );
    }

    println!("\n=== HT_Book_Date ===");
    let stream = conn
        .simple_query(format!(
            "SELECT id, Book_no, Book_type, Book_date_ds, Book_USE, Book_ok FROM HT_Book_Date WHERE Book_no='{book_no}' ORDER BY id"
        ))
        .await?;
    for row in stream.into_first_result().await? {
        println!(
            "  id={:?} Book_no={:?} Book_type={:?} Book_date_ds={:?} Book_USE={:?} Book_ok={:?}",
            row.get::<i32, _>("id"),
            row.get::<&str, _>("Book_no"),
            row.get::<&str, _>("Book_type"),
            row.get::<&str, _>("Book_date_ds"),
            row.get::<i32, _>("Book_USE"),
            row.get::<i32, _>("Book_ok"),
        );
    }

    Ok(())
}
