//! `create_user` admin CLI — Phase 4 PR1.
//!
//! Mints a new local user (admin or receptionist) for the cookie-session
//! auth flow introduced in this PR. Used by operators on the production
//! box; not invoked by the running backend.
//!
//! ```text
//!   cargo run --bin create_user -- --username alice --role admin
//!   cargo run --bin create_user -- --username bob --role receptionist --password hunter2
//! ```
//!
//! When `--password` is omitted the CLI prompts on the TTY (twice, for
//! confirmation) so the secret never ends up in shell history.
//!
//! All work happens in a single PG transaction: hash the password,
//! INSERT into `ht_users`, COMMIT. On username collision the unique
//! index on `ht_users.username` makes the insert fail and the
//! transaction rolls back.

use std::process::ExitCode;
use std::sync::Arc;

use hotel_backend::config::NewDbConfig;
use hotel_backend::domain::user::Role;
use hotel_backend::repository::user::{PgUserRepository, UserRepository};
use hotel_backend::service::auth::AuthService;

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

    let args: Vec<String> = std::env::args().collect();
    let parsed = match parse_args(&args) {
        Ok(p) => p,
        Err(message) => {
            eprintln!("{message}");
            print_usage();
            return ExitCode::FAILURE;
        }
    };

    let role = match Role::try_from(parsed.role.as_str()) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("invalid --role: {err}");
            return ExitCode::FAILURE;
        }
    };

    let username = parsed.username.trim().to_string();
    if username.is_empty() {
        eprintln!("--username must not be empty");
        return ExitCode::FAILURE;
    }
    if username.len() > 64 {
        eprintln!("--username must be at most 64 characters");
        return ExitCode::FAILURE;
    }

    let password = match resolve_password(parsed.password) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let password_hash = match AuthService::<PgUserRepository, hotel_backend::repository::session::PgSessionRepository>::hash_password(&password) {
        Ok(h) => h,
        Err(err) => {
            eprintln!("failed to hash password: {err}");
            return ExitCode::FAILURE;
        }
    };

    let pool = match build_pool().await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("failed to connect to PostgreSQL: {err}");
            return ExitCode::FAILURE;
        }
    };

    let users = Arc::new(PgUserRepository::new());
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("failed to start transaction: {err}");
            return ExitCode::FAILURE;
        }
    };

    let user_id = match users.insert(&mut tx, &username, &password_hash, role).await {
        Ok(id) => id,
        Err(err) => {
            // Roll back implicitly when `tx` drops.
            if is_unique_violation(&err) {
                eprintln!("username '{username}' already exists");
            } else {
                eprintln!("failed to insert user: {err}");
            }
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = tx.commit().await {
        eprintln!("failed to commit transaction: {err}");
        return ExitCode::FAILURE;
    }

    println!(
        "Created user '{username}' (user_id={user_id}, role={role})",
        role = role.as_str()
    );
    ExitCode::SUCCESS
}

#[derive(Debug, Default)]
struct ParsedArgs {
    username: String,
    role: String,
    password: Option<String>,
}

/// Hand-rolled flag parser — keeps the CLI free of clap/structopt
/// dependencies for a single 3-flag tool.
fn parse_args(args: &[String]) -> Result<ParsedArgs, String> {
    let mut parsed = ParsedArgs::default();
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--username" => {
                parsed.username = iter
                    .next()
                    .ok_or_else(|| "--username requires a value".to_string())?
                    .clone();
            }
            "--role" => {
                parsed.role = iter
                    .next()
                    .ok_or_else(|| "--role requires a value".to_string())?
                    .clone();
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
    if parsed.username.is_empty() {
        return Err("missing required --username".to_string());
    }
    if parsed.role.is_empty() {
        return Err("missing required --role".to_string());
    }
    Ok(parsed)
}

fn print_usage() {
    eprintln!(
        "Usage: create_user --username NAME --role admin|receptionist [--password PASS]\n\
         \n\
         When --password is omitted the CLI prompts twice on the TTY for confirmation."
    );
}

/// Resolve the password either from the `--password` flag or by
/// prompting on the TTY. When prompting, asks twice and refuses to
/// proceed if the two values differ.
fn resolve_password(from_flag: Option<String>) -> Result<String, String> {
    if let Some(p) = from_flag {
        if p.is_empty() {
            return Err("--password must not be empty".to_string());
        }
        return Ok(p);
    }
    let first = rpassword::prompt_password("Password: ")
        .map_err(|err| format!("failed to read password from tty: {err}"))?;
    if first.is_empty() {
        return Err("password must not be empty".to_string());
    }
    let second = rpassword::prompt_password("Confirm password: ")
        .map_err(|err| format!("failed to read password confirmation: {err}"))?;
    if first != second {
        return Err("passwords did not match".to_string());
    }
    Ok(first)
}

async fn build_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let config = NewDbConfig::from_env();
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.connection_string())
        .await
}

/// Detect the PG unique-violation SQLSTATE so we can render a nicer
/// "username already exists" instead of dumping a raw constraint name.
fn is_unique_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        db_err.code().as_deref() == Some("23505")
    } else {
        false
    }
}
