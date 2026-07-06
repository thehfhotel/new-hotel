//! `set_user_card` admin CLI — link an NFC staff-card badge to a `ht_users` row.
//!
//! Lets an operator PRE-PROVISION a card→user mapping without a physical tap
//! (the tap path auto-provisions on first sight, but this is handy for seeding
//! known staff, or for attaching a badge to an EXISTING password/CF user so
//! the same person keeps one account).
//!
//! ```text
//!   # Attach a badge to an existing user (keeps their role/password):
//!   cargo run --bin set_user_card -- --badge B00123 --username winut
//!
//!   # Create a new card-only receptionist for a badge (no password login):
//!   cargo run --bin set_user_card -- --badge B00123 --display-name "Nok"
//!
//!   # Create a card user with a specific role + a real password too:
//!   cargo run --bin set_user_card -- --badge B00123 --username nok --role cashier --password s3cret
//! ```
//!
//! Idempotent-ish: re-linking the same badge to the same user is a no-op
//! UPDATE. A badge already bound to a DIFFERENT user is refused (the
//! `ux_ht_users_badge` unique index also enforces this at the DB level).

use std::process::ExitCode;

use hotel_backend::config::NewDbConfig;
use hotel_backend::domain::user::Role;
use hotel_backend::service::auth::AuthService;
use hotel_backend::service::reader::CARD_ONLY_PASSWORD_SENTINEL;
use sqlx::Row;

fn main() -> ExitCode {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("failed to start tokio runtime: {err}");
            return ExitCode::FAILURE;
        }
    };
    rt.block_on(async_main())
}

async fn async_main() -> ExitCode {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();

    let args: Vec<String> = std::env::args().collect();
    let parsed = match parse_args(&args) {
        Ok(p) => p,
        Err(message) => {
            eprintln!("{message}");
            print_usage();
            return ExitCode::FAILURE;
        }
    };

    let badge = parsed.badge.trim();
    if badge.is_empty() {
        eprintln!("--badge must not be empty");
        return ExitCode::FAILURE;
    }
    if badge.len() > 50 {
        eprintln!("--badge must be at most 50 characters");
        return ExitCode::FAILURE;
    }

    let role = match Role::try_from(parsed.role.as_str()) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("invalid --role: {err}");
            return ExitCode::FAILURE;
        }
    };

    // Username: explicit, or the auto-provision scheme `card-<badge>`.
    let username = parsed
        .username
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("card-{badge}"));
    if username.len() > 64 {
        eprintln!("--username must be at most 64 characters");
        return ExitCode::FAILURE;
    }

    let pool = match build_pool().await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("failed to connect to PostgreSQL: {err}");
            return ExitCode::FAILURE;
        }
    };

    // Refuse to steal a badge already bound to a different username.
    match existing_username_for_badge(&pool, badge).await {
        Ok(Some(owner)) if owner != username => {
            eprintln!("badge '{badge}' is already linked to user '{owner}'");
            return ExitCode::FAILURE;
        }
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to check existing badge: {err}");
            return ExitCode::FAILURE;
        }
    }

    let user_exists = match username_exists(&pool, &username).await {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to look up user: {err}");
            return ExitCode::FAILURE;
        }
    };

    let outcome = if user_exists {
        link_existing(&pool, &username, badge, parsed.display_name.as_deref()).await
    } else {
        let password_hash = match resolve_password_hash(parsed.password.as_deref()) {
            Ok(h) => h,
            Err(err) => {
                eprintln!("{err}");
                return ExitCode::FAILURE;
            }
        };
        create_card_user(
            &pool,
            &username,
            &password_hash,
            role,
            badge,
            parsed.display_name.as_deref(),
        )
        .await
    };

    let user_id = match outcome {
        Ok(id) => id,
        Err(err) => {
            if is_unique_violation(&err) {
                eprintln!("badge '{badge}' or username '{username}' collides with an existing row");
            } else {
                eprintln!("failed to link card: {err}");
            }
            return ExitCode::FAILURE;
        }
    };

    // Ensure the role junction row exists so permissions resolve (migration 046).
    if let Err(err) = ensure_role_junction(&pool, user_id, role).await {
        eprintln!("linked badge but failed to ensure role junction: {err}");
        return ExitCode::FAILURE;
    }

    println!(
        "Linked badge '{badge}' → user '{username}' (user_id={user_id}, role={role})",
        role = role.as_str()
    );
    ExitCode::SUCCESS
}

#[derive(Debug, Default)]
struct ParsedArgs {
    badge: String,
    username: Option<String>,
    role: String,
    display_name: Option<String>,
    password: Option<String>,
}

fn parse_args(args: &[String]) -> Result<ParsedArgs, String> {
    let mut parsed = ParsedArgs {
        role: "receptionist".to_string(),
        ..Default::default()
    };
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--badge" => {
                parsed.badge = iter
                    .next()
                    .ok_or_else(|| "--badge requires a value".to_string())?
                    .clone();
            }
            "--username" => {
                parsed.username = Some(
                    iter.next()
                        .ok_or_else(|| "--username requires a value".to_string())?
                        .clone(),
                );
            }
            "--role" => {
                parsed.role = iter
                    .next()
                    .ok_or_else(|| "--role requires a value".to_string())?
                    .clone();
            }
            "--display-name" => {
                parsed.display_name = Some(
                    iter.next()
                        .ok_or_else(|| "--display-name requires a value".to_string())?
                        .clone(),
                );
            }
            "--password" => {
                parsed.password = Some(
                    iter.next()
                        .ok_or_else(|| "--password requires a value".to_string())?
                        .clone(),
                );
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    if parsed.badge.is_empty() {
        return Err("missing required --badge".to_string());
    }
    Ok(parsed)
}

fn print_usage() {
    eprintln!(
        "Usage: set_user_card --badge BADGE [--username NAME] [--role admin|receptionist|cashier|housekeeper] \
         [--display-name NAME] [--password PASS]\n\
         \n\
         Links an NFC staff-card badge to a ht_users row. When --username is omitted the\n\
         auto-provision scheme `card-<badge>` is used. When creating a NEW user without\n\
         --password the account is card-only (a password sentinel that never verifies)."
    );
}

/// Hash `--password` if given, else the card-only sentinel (never verifies).
fn resolve_password_hash(password: Option<&str>) -> Result<String, String> {
    match password {
        Some(plain) if !plain.is_empty() => {
            AuthService::<
                hotel_backend::repository::user::PgUserRepository,
                hotel_backend::repository::session::PgSessionRepository,
            >::hash_password(plain)
            .map_err(|err| format!("failed to hash password: {err}"))
        }
        Some(_) => Err("--password must not be empty".to_string()),
        None => Ok(CARD_ONLY_PASSWORD_SENTINEL.to_string()),
    }
}

async fn build_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let config = NewDbConfig::from_env();
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.connection_string())
        .await
}

async fn existing_username_for_badge(
    pool: &sqlx::PgPool,
    badge: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query("SELECT username FROM ht_users WHERE badge = $1")
        .bind(badge)
        .fetch_optional(pool)
        .await?;
    row.map(|r| r.try_get::<String, _>("username")).transpose()
}

async fn username_exists(pool: &sqlx::PgPool, username: &str) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 FROM ht_users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

/// Attach the badge (and optional display_name) to an EXISTING user. Keeps the
/// user's current role + password untouched. Returns the user_id.
async fn link_existing(
    pool: &sqlx::PgPool,
    username: &str,
    badge: &str,
    display_name: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE ht_users
        SET badge = $2,
            display_name = COALESCE($3, display_name)
        WHERE username = $1
        RETURNING user_id
        "#,
    )
    .bind(username)
    .bind(badge)
    .bind(display_name)
    .fetch_one(pool)
    .await?;
    row.try_get::<i64, _>("user_id")
}

/// Create a fresh card user carrying the badge. Returns the new user_id.
async fn create_card_user(
    pool: &sqlx::PgPool,
    username: &str,
    password_hash: &str,
    role: Role,
    badge: &str,
    display_name: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO ht_users (username, password_hash, role, active, display_name, badge)
        VALUES ($1, $2, $3, TRUE, $4, $5)
        RETURNING user_id
        "#,
    )
    .bind(username)
    .bind(password_hash)
    .bind(role.as_str())
    .bind(display_name)
    .bind(badge)
    .fetch_one(pool)
    .await?;
    row.try_get::<i64, _>("user_id")
}

async fn ensure_role_junction(
    pool: &sqlx::PgPool,
    user_id: i64,
    role: Role,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO ht_user_roles (user_id, role_id)
        SELECT $1, role_id FROM ht_roles WHERE role_key = $2
        ON CONFLICT (user_id, role_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(role.as_str())
    .execute(pool)
    .await?;
    Ok(())
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("23505"))
}
