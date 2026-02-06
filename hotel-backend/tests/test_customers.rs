//! Customer CRUD integration tests.
//!
//! Exercises create, read, update, and soft-delete operations on the
//! `ht_customers` table.  Every test row uses `TEST_` in `cust_notes` so that
//! the shared cleanup helper can remove it.

mod common;

use sqlx::Row;

#[tokio::test]
async fn customer_crud_lifecycle() {
    let pool = common::create_test_pool().await;

    // Ensure no stale test data
    common::cleanup(&pool).await;

    // ---- CREATE ----
    let row = sqlx::query(
        r#"INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_phone, cust_email, cust_notes)
           VALUES ('IntegrationFirst', 'IntegrationLast', '0999999999', 'test@example.com', 'TEST_customer_crud')
           RETURNING cust_id"#,
    )
    .fetch_one(&pool)
    .await
    .expect("INSERT customer should succeed");

    let cust_id: i32 = row.try_get("cust_id").expect("cust_id should be returned");
    assert!(cust_id > 0, "cust_id should be positive");

    // ---- READ ----
    let row = sqlx::query(
        "SELECT cust_id, cust_firstname, cust_lastname, cust_phone, cust_email, cust_active \
         FROM ht_customers WHERE cust_id = $1",
    )
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .expect("SELECT customer should succeed");

    let first_name: String = row.try_get("cust_firstname").unwrap();
    assert_eq!(first_name, "IntegrationFirst");

    let last_name: String = row.try_get("cust_lastname").unwrap();
    assert_eq!(last_name, "IntegrationLast");

    let phone: String = row.try_get("cust_phone").unwrap();
    assert_eq!(phone, "0999999999");

    let email: String = row.try_get("cust_email").unwrap();
    assert_eq!(email, "test@example.com");

    let active: bool = row.try_get("cust_active").unwrap();
    assert!(active, "Newly created customer should be active");

    // ---- UPDATE ----
    let result = sqlx::query(
        "UPDATE ht_customers SET cust_firstname = 'UpdatedFirst', cust_phone = '0888888888', \
         updated_at = NOW() WHERE cust_id = $1",
    )
    .bind(cust_id)
    .execute(&pool)
    .await
    .expect("UPDATE customer should succeed");

    assert_eq!(result.rows_affected(), 1);

    // Verify update
    let row = sqlx::query(
        "SELECT cust_firstname, cust_phone FROM ht_customers WHERE cust_id = $1",
    )
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let first_name: String = row.try_get("cust_firstname").unwrap();
    assert_eq!(first_name, "UpdatedFirst");

    let phone: String = row.try_get("cust_phone").unwrap();
    assert_eq!(phone, "0888888888");

    // ---- SOFT DELETE (set active=false) ----
    let result = sqlx::query(
        "UPDATE ht_customers SET cust_active = false, updated_at = NOW() WHERE cust_id = $1",
    )
    .bind(cust_id)
    .execute(&pool)
    .await
    .expect("Soft-delete customer should succeed");

    assert_eq!(result.rows_affected(), 1);

    // Verify soft delete
    let row = sqlx::query("SELECT cust_active FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let active: bool = row.try_get("cust_active").unwrap();
    assert!(!active, "Customer should be inactive after soft delete");

    // ---- CLEANUP ----
    common::cleanup(&pool).await;
}

#[tokio::test]
async fn customer_search_by_name() {
    let pool = common::create_test_pool().await;
    common::cleanup(&pool).await;

    // Insert two customers with distinct names
    sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ('AlphaSearch', 'Bravo', 'TEST_search')",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ('CharlieSearch', 'Delta', 'TEST_search')",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Search for 'Alpha' -- should find exactly one
    let rows = sqlx::query(
        "SELECT cust_id FROM ht_customers \
         WHERE cust_firstname LIKE '%AlphaSearch%' AND cust_notes LIKE 'TEST_%'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(rows.len(), 1, "Search for 'AlphaSearch' should return 1 row");

    // Search for 'Search' -- should find both
    let rows = sqlx::query(
        "SELECT cust_id FROM ht_customers \
         WHERE cust_firstname LIKE '%Search%' AND cust_notes LIKE 'TEST_%'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(rows.len(), 2, "Search for 'Search' should return 2 rows");

    common::cleanup(&pool).await;
}
