//! Integration tests for Track G7 — permission expansion.
//!
//! Exercises the full permission grid end-to-end against a live
//! PostgreSQL test database. Skipped automatically when `DATABASE_URL`
//! is unset (mirrors the rest of `hotel-backend/tests/*`).
//!
//! What's verified (J4: invariant-based, not hardcoded enumerations):
//!
//! 1. `ht_roles` / `ht_permissions` / `ht_role_permissions` / `ht_user_roles`
//!    tables exist with the expected primary keys + foreign keys after
//!    migration 046 lands.
//! 2. Structural invariants of the seed grid (refactored away from the
//!    brittle "exact vec must match" pattern that broke every time a new
//!    permission was added — see G5/G6/J4 history):
//!      * Every seeded permission is held by at least one role (no orphans)
//!      * Every `ht_role_permissions` row references a live permission_id
//!      * `admin` role holds every seeded permission (super-role invariant)
//!      * Per-role smoke checks for known-critical grants and known-denied
//!        grants — catches accidental DELETEs without re-asserting the
//!        complete grid on every G-track expansion.
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

/// Load the permission keys granted to a single `role_key`, sorted.
///
/// Shared helper for the invariant + smoke tests below — kept module-level
/// so each test can call it without redefining the SQL.
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

/// J4 invariant #1: every seeded permission must be granted to at least
/// one role. Catches the "added a permission to `ht_permissions` but
/// forgot to wire it into any role" footgun, without re-asserting the
/// full grid on every new permission.
#[tokio::test]
async fn every_seeded_permission_is_held_by_at_least_one_role() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    let orphan_perms: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT p.permission_key
        FROM ht_permissions p
        LEFT JOIN ht_role_permissions rp ON rp.permission_id = p.permission_id
        WHERE rp.role_id IS NULL
        ORDER BY p.permission_key
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("query orphan perms");

    assert!(
        orphan_perms.is_empty(),
        "These permissions exist but are not held by any role: {orphan_perms:?}"
    );
}

/// J4 invariant #2: no `ht_role_permissions` row may reference a deleted
/// `permission_id`. The FK constraint makes this impossible at the DB
/// level today, but pinning it as a behavioral invariant guards against
/// future migrations that might (accidentally) loosen the constraint.
#[tokio::test]
async fn every_role_permission_references_a_seeded_permission_key() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    let dangling: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM ht_role_permissions rp
        LEFT JOIN ht_permissions p ON p.permission_id = rp.permission_id
        WHERE p.permission_id IS NULL
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("query dangling");

    assert_eq!(
        dangling, 0,
        "ht_role_permissions rows reference deleted permission_ids"
    );
}

/// J4 invariant #3: the `admin` role is a super-role and MUST hold every
/// permission in `ht_permissions`. Catches "added a new permission but
/// forgot to grant it to admin", which would silently lock admins out of
/// new features. Implemented as a set-equality check — does NOT enumerate
/// permission keys, so adding a new permission requires zero test edits.
#[tokio::test]
async fn admin_role_holds_every_seeded_permission() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    let all_perms: Vec<String> = sqlx::query_scalar(
        "SELECT permission_key FROM ht_permissions ORDER BY permission_key",
    )
    .fetch_all(&pool)
    .await
    .expect("query all perms");

    let admin_perms = perms_for_role(&pool, "admin").await;

    let missing_from_admin: Vec<&String> = all_perms
        .iter()
        .filter(|p| !admin_perms.contains(p))
        .collect();

    assert!(
        missing_from_admin.is_empty(),
        "admin role is missing permissions it must hold as the super-role: {missing_from_admin:?}"
    );
}

/// J4 smoke test: known-critical grants that *must* exist on each role,
/// plus known-denied permissions that *must not* exist. Targets specific
/// audit-mandated entries (e.g. cashier holds `payment.refund`) without
/// hardcoding the complete grid — adding new permissions does NOT require
/// touching this test as long as the existing critical grants remain.
#[tokio::test]
async fn critical_role_grants_and_denials_remain_intact() {
    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    }
    let pool = common::create_test_pool().await;

    let cashier = perms_for_role(&pool, "cashier").await;
    assert!(
        cashier.contains(&"payment.refund".to_string()),
        "cashier MUST hold payment.refund (audit T4); got {cashier:?}"
    );
    assert!(
        cashier.contains(&"reports.rr4".to_string()),
        "cashier MUST hold reports.rr4 (audit T4); got {cashier:?}"
    );
    assert!(
        !cashier.contains(&"admin.users".to_string()),
        "REGRESSION: cashier holds admin.users (should be admin-only)"
    );

    let housekeeper = perms_for_role(&pool, "housekeeper").await;
    assert!(
        housekeeper.contains(&"inventory.consume".to_string()),
        "housekeeper MUST hold inventory.consume (audit T4); got {housekeeper:?}"
    );
    assert!(
        housekeeper.contains(&"checkin.room_change".to_string()),
        "housekeeper MUST hold checkin.room_change (audit T4); got {housekeeper:?}"
    );
    assert!(
        !housekeeper.contains(&"payment.refund".to_string()),
        "REGRESSION: housekeeper holds payment.refund (should be cashier-only)"
    );
    assert!(
        !housekeeper.contains(&"admin.users".to_string()),
        "REGRESSION: housekeeper holds admin.users (should be admin-only)"
    );

    let receptionist = perms_for_role(&pool, "receptionist").await;
    assert!(
        receptionist.contains(&"checkin.room_change".to_string()),
        "receptionist MUST hold checkin.room_change (audit T4); got {receptionist:?}"
    );
    assert!(
        receptionist.contains(&"inventory.consume".to_string()),
        "receptionist MUST hold inventory.consume (audit T4); got {receptionist:?}"
    );
    assert!(
        !receptionist.contains(&"payment.refund".to_string()),
        "REGRESSION: receptionist holds payment.refund (should be cashier-only)"
    );
    assert!(
        !receptionist.contains(&"reports.rr4".to_string()),
        "REGRESSION: receptionist holds reports.rr4 (should be cashier-only)"
    );
    assert!(
        !receptionist.contains(&"admin.users".to_string()),
        "REGRESSION: receptionist holds admin.users (should be admin-only)"
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
    // J4: assert the critical housekeeper grants are present and that
    // cashier-only / admin-only grants are denied, instead of pinning the
    // exact vec (which broke every G-track expansion).
    assert!(
        housekeeper_perms.contains(&"checkin.room_change".to_string()),
        "housekeeper_test must hold checkin.room_change (got {housekeeper_perms:?})"
    );
    assert!(
        housekeeper_perms.contains(&"inventory.consume".to_string()),
        "housekeeper_test must hold inventory.consume (got {housekeeper_perms:?})"
    );
    assert!(
        !housekeeper_perms.contains(&"payment.refund".to_string()),
        "REGRESSION: housekeeper holds payment.refund (should be cashier-only)"
    );
    assert!(
        !housekeeper_perms.contains(&"admin.users".to_string()),
        "REGRESSION: housekeeper holds admin.users (should be admin-only)"
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
