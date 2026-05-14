//! Integration tests for Track G7 — permission expansion.
//!
//! Exercises the full permission grid end-to-end against a live
//! PostgreSQL test database. Skipped automatically when `DATABASE_URL`
//! is unset (mirrors the rest of `hotel-backend/tests/*`).
//!
//! What's verified:
//!
//! 1. `ht_roles` / `ht_permissions` / `ht_role_permissions` / `ht_user_roles`
//!    tables exist with the expected primary keys + foreign keys after
//!    migration 046 lands.
//! 2. The seed grid matches the audit T4 grant table:
//!      * admin        → all 6 permissions
//!      * cashier      → payment.refund, checkin.round_bill, reports.rr4
//!      * housekeeper  → inventory.consume, checkin.room_change
//!      * receptionist → checkin.room_change, inventory.consume
//! 3. The three seed test accounts (`housekeeper_test`, `cashier_test`,
//!    `receptionist_test`) exist, are `active = TRUE`, hold the right
//!    primary role in the `ht_user_roles` junction, and their stored
//!    Argon2id hashes verify against the documented `temp_password_2026`
//!    plaintext (so QA-time login actually works).
//! 4. `middleware::permissions::permissions_for_user` returns the
//!    correct set for each seed account — this is the same code path
//!    the `/api/auth/me` route + the `require_permission` gate use, so
//!    a regression here would break both.

mod common;

use hotel_backend::middleware::permissions_for_user;
use hotel_backend::service::auth::AuthService;
use hotel_backend::repository::session::PgSessionRepository;
use hotel_backend::repository::user::PgUserRepository;
use sqlx::Row;

const SEED_PASSWORD: &str = "temp_password_2026";

/// Resolve a username to its `user_id`, panicking with a useful message
/// if the seed account is missing — that means migration 046 did not
/// run (or the test DB is stale).
async fn user_id_for(pool: &sqlx::PgPool, username: &str) -> i64 {
    let row = sqlx::query("SELECT user_id FROM ht_users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .expect("query ht_users")
        .unwrap_or_else(|| panic!(
            "seed user {username} not found — migration 046 likely missing"
        ));
    row.get::<i64, _>("user_id")
}

#[tokio::test]
async fn role_permission_grid_matches_audit_t4() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    // Helper: load the permission keys granted to a single role_key.
    async fn perms_for_role(pool: &sqlx::PgPool, role_key: &str) -> Vec<String> {
        let rows = sqlx::query(
            r#"
            SELECT p.permission_key
            FROM ht_roles r
            JOIN ht_role_permissions rp ON rp.role_id = r.role_id
            JOIN ht_permissions p        ON p.permission_id = rp.permission_id
            WHERE r.role_key = $1
            ORDER BY p.permission_key
            "#,
        )
        .bind(role_key)
        .fetch_all(pool)
        .await
        .expect("query role permissions");

        rows.into_iter()
            .map(|r| r.get::<String, _>("permission_key"))
            .collect()
    }

    let admin = perms_for_role(&pool, "admin").await;
    assert_eq!(
        admin,
        vec![
            "admin.users".to_string(),
            "checkin.room_change".to_string(),
            "checkin.round_bill".to_string(),
            "coupon.issue".to_string(),
            "coupon.redeem".to_string(),
            "inventory.consume".to_string(),
            "payment.refund".to_string(),
            "reports.rr4".to_string(),
        ],
        "admin role must hold all 8 permissions (got {admin:?})"
    );

    let cashier = perms_for_role(&pool, "cashier").await;
    assert_eq!(
        cashier,
        vec![
            "checkin.round_bill".to_string(),
            "coupon.redeem".to_string(),
            "payment.refund".to_string(),
            "reports.rr4".to_string(),
        ],
        "cashier role grid drifted (got {cashier:?})"
    );

    let housekeeper = perms_for_role(&pool, "housekeeper").await;
    assert_eq!(
        housekeeper,
        vec![
            "checkin.room_change".to_string(),
            "inventory.consume".to_string(),
        ],
        "housekeeper role grid drifted (got {housekeeper:?})"
    );

    let receptionist = perms_for_role(&pool, "receptionist").await;
    assert_eq!(
        receptionist,
        vec![
            "checkin.room_change".to_string(),
            "coupon.issue".to_string(),
            "coupon.redeem".to_string(),
            "inventory.consume".to_string(),
        ],
        "receptionist role grid drifted (got {receptionist:?})"
    );
}

#[tokio::test]
async fn seed_test_accounts_have_expected_roles_and_passwords() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    for (username, expected_role_key) in [
        ("housekeeper_test", "housekeeper"),
        ("cashier_test", "cashier"),
        ("receptionist_test", "receptionist"),
    ] {
        let row = sqlx::query(
            r#"
            SELECT u.password_hash, u.role, u.active,
                   (SELECT array_agg(r.role_key)
                      FROM ht_user_roles ur
                      JOIN ht_roles r ON r.role_id = ur.role_id
                     WHERE ur.user_id = u.user_id) AS roles
            FROM ht_users u
            WHERE u.username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|err| panic!("seed user {username} missing: {err}"));

        let active: bool = row.get("active");
        assert!(active, "seed user {username} must be active");

        let role_col: String = row.get("role");
        assert_eq!(
            role_col, expected_role_key,
            "seed user {username} legacy role column drifted"
        );

        let roles: Vec<String> = row.get("roles");
        assert!(
            roles.contains(&expected_role_key.to_string()),
            "seed user {username} missing junction row for role {expected_role_key} (got {roles:?})"
        );

        let stored_hash: String = row.get("password_hash");
        let verified = AuthService::<PgUserRepository, PgSessionRepository>::verify_password(
            SEED_PASSWORD,
            &stored_hash,
        )
        .expect("verify_password should not error for a well-formed PHC string");
        assert!(
            verified,
            "seed account {username} password hash does not verify against {SEED_PASSWORD:?}"
        );
    }
}

#[tokio::test]
async fn permissions_for_user_returns_correct_grid_per_seed_account() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    // Clear the in-process cache so each lookup hits the DB and exercises
    // the SQL path; sibling tests may have populated it for these users.
    hotel_backend::middleware::permissions::invalidate_all();

    let housekeeper_id = user_id_for(&pool, "housekeeper_test").await;
    let cashier_id = user_id_for(&pool, "cashier_test").await;
    let receptionist_id = user_id_for(&pool, "receptionist_test").await;

    let housekeeper_perms = permissions_for_user(&pool, housekeeper_id)
        .await
        .expect("permissions_for_user(housekeeper)");
    assert_eq!(
        housekeeper_perms,
        vec![
            "checkin.room_change".to_string(),
            "inventory.consume".to_string(),
        ],
        "housekeeper_test should not see payment.refund"
    );
    assert!(
        !housekeeper_perms.contains(&"payment.refund".to_string()),
        "REGRESSION: housekeeper holds payment.refund (should be cashier-only)"
    );

    let cashier_perms = permissions_for_user(&pool, cashier_id)
        .await
        .expect("permissions_for_user(cashier)");
    assert!(
        cashier_perms.contains(&"payment.refund".to_string()),
        "cashier_test should hold payment.refund (got {cashier_perms:?})"
    );
    assert!(
        cashier_perms.contains(&"reports.rr4".to_string()),
        "cashier_test should hold reports.rr4 (got {cashier_perms:?})"
    );
    assert!(
        !cashier_perms.contains(&"admin.users".to_string()),
        "REGRESSION: cashier holds admin.users (should be admin-only)"
    );

    let receptionist_perms = permissions_for_user(&pool, receptionist_id)
        .await
        .expect("permissions_for_user(receptionist)");
    assert!(
        !receptionist_perms.contains(&"payment.refund".to_string()),
        "REGRESSION: receptionist holds payment.refund (should be cashier-only)"
    );
    assert!(
        !receptionist_perms.contains(&"reports.rr4".to_string()),
        "REGRESSION: receptionist holds reports.rr4 (should be cashier-only)"
    );
    assert!(
        receptionist_perms.contains(&"checkin.room_change".to_string()),
        "receptionist_test should hold checkin.room_change"
    );
}

#[tokio::test]
async fn permission_cache_returns_consistent_results_across_repeated_lookups() {
    // Cache invariant: two back-to-back lookups for the same user MUST
    // return identical sets. Catches a class of bug where the cache
    // returns a stale-but-different snapshot or fails to populate at all.
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;
    hotel_backend::middleware::permissions::invalidate_all();

    let cashier_id = user_id_for(&pool, "cashier_test").await;

    let first = permissions_for_user(&pool, cashier_id).await.unwrap();
    let second = permissions_for_user(&pool, cashier_id).await.unwrap();
    assert_eq!(
        first, second,
        "permission cache returned inconsistent results across repeated lookups"
    );
}
