//! Docker-secret hydration (security audit 2026-05-14).
//!
//! ## Why
//!
//! Production secrets used to ship to containers via the `environment:`
//! block of `docker-compose.yml`. Anyone with `docker exec` rights could
//! then read them via `printenv` (and `deploy` is in the `docker` group
//! on evergreen). Concretely the audit found `DB_PASSWORD`,
//! `POSTGRES_PASSWORD`, `SLACK_WEBHOOK_URL` and the pre-baked
//! `DATABASE_URL` (which embeds the PG password) all visible in the
//! sync container's env.
//!
//! ## What this module does
//!
//! On startup, EACH binary calls [`hydrate_env_from_secret_files`] BEFORE
//! anything reads `std::env::var`. For every entry in [`SECRET_FILE_MAP`]
//! we check `/run/secrets/<file_name>`; if it exists and is non-empty,
//! we trim a single trailing newline and write the value into the
//! matching env var. Existing env-var values are preserved so:
//!
//! 1. Local dev keeps working without secret files (env-var fallback).
//! 2. A botched deploy that ships secret-file mount AND env var won't
//!    silently swap one for the other — the env-var wins, matching the
//!    "least surprise" of the pre-migration behavior.
//!
//! The hydrator also reconstructs `DATABASE_URL` from
//! `{POSTGRES_USER, POSTGRES_PASSWORD, NEW_DB_SERVER|newdb,
//! NEW_DB_PORT|PGPORT|5439, POSTGRES_DB|hotelnew}` if `DATABASE_URL`
//! isn't already set — that's the variable the `sync` / `writeback`
//! / `backfill_*` bins read. The reconstruction means the compose file
//! no longer has to embed `${POSTGRES_PASSWORD}` in a pre-baked DSN.
//!
//! ## Reversibility
//!
//! The env-var fallback is intentional. If the secret-file machinery
//! ever wedges in production, an operator can revert `docker-compose.yml`
//! (or set the corresponding env var back in `.env`) and the binary
//! still boots — no code change required. This is the "one git revert"
//! rollback the security migration committed to.
//!
//! ## What this module deliberately does NOT do
//!
//! - No retry / file watcher: secrets are read ONCE at process start.
//!   Rotating a secret means restart the container (matches Docker
//!   secret semantics — the file is bind-mounted, but processes cache
//!   env state).
//! - No panic on missing secret: that decision is delegated to
//!   `config.rs::require_secret`, which already panics with a helpful
//!   message when `DB_PASSWORD` is missing / empty. Centralising the
//!   "must exist" check there keeps test coverage in one place.

use std::env;
use std::fs;
use std::path::Path;

/// Directory Docker mounts secrets into by default. Override via the
/// `SECRETS_DIR` env var so tests + alternate orchestrators (Kubernetes
/// projected volumes, Nomad, plain bind-mounts) work without a code change.
const DEFAULT_SECRETS_DIR: &str = "/run/secrets";

/// Mapping of `<secret file name>` → `<env var to hydrate>`.
///
/// Each tuple is `(secret_file_name, env_var_name)`. Order doesn't matter —
/// the loop reads each file independently. Adding a new secret means
/// (a) add a tuple here, (b) declare the secret in `docker-compose.yml`,
/// (c) mount it on the service that needs it.
const SECRET_FILE_MAP: &[(&str, &str)] = &[
    // Legacy MSSQL `sa` password — shared across HF Hotel + HF Ville
    // until ADR 0001 Phase 8 splits them.
    ("db_password", "DB_PASSWORD"),
    // HotelNew PG password (also used as the canonical Ville PG password
    // by default — see `VILLE_DB_PASSWORD` below for the split path).
    ("postgres_password", "POSTGRES_PASSWORD"),
    // HF Ville PG password — same value as POSTGRES_PASSWORD until split.
    ("ville_db_password", "VILLE_DB_PASSWORD"),
    // Slack alerter webhook (carries a per-channel token in the URL).
    ("slack_webhook_url", "SLACK_WEBHOOK_URL"),
];

/// Hydrate env vars from Docker secret files, then reconstruct
/// `DATABASE_URL` from the resulting pieces if it isn't already set.
///
/// Call this BEFORE the first `std::env::var` lookup in any binary's
/// `main()`. Safe to call multiple times (idempotent). Reads are
/// best-effort: a missing file is silently skipped (env var stays
/// whatever it was — possibly unset, possibly the .env fallback).
///
/// Returns the count of env vars that were freshly populated from a
/// file. Useful for a startup log line — operators tailing the
/// container at boot can see "hydrated 3 secrets from /run/secrets/"
/// and confirm the mount worked without exposing the values.
pub fn hydrate_env_from_secret_files() -> usize {
    let secrets_dir = env::var("SECRETS_DIR").unwrap_or_else(|_| DEFAULT_SECRETS_DIR.to_string());
    let mut hydrated_count = 0;

    for (file_name, env_var_name) in SECRET_FILE_MAP {
        if env_already_set(env_var_name) {
            // Caller-provided env var wins. Either local dev (.env
            // populated `DB_PASSWORD=...`) or a deploy still on the
            // legacy env-var path during the migration window.
            continue;
        }
        let path = Path::new(&secrets_dir).join(file_name);
        if let Some(value) = read_secret_file(&path) {
            // SAFETY: this runs before any other thread spawns
            // (`tokio::main` macro expands AFTER `main()` body starts,
            // and the hydrate call is the FIRST line of every main).
            // env::set_var is undefined behaviour in multi-threaded
            // contexts on some platforms — keeping it single-threaded
            // is load-bearing.
            env::set_var(env_var_name, value);
            hydrated_count += 1;
        }
    }

    // Alias POSTGRES_PASSWORD → NEW_DB_PASSWORD when only the former is
    // populated. The compose file used to set both via env vars (with
    // the same value); now POSTGRES_PASSWORD comes from a secret file
    // and the backend's `NewDbConfig::from_env` still reads
    // `NEW_DB_PASSWORD`. Aliasing here avoids forcing every config
    // call-site to learn about either name.
    if !env_already_set("NEW_DB_PASSWORD") {
        if let Ok(value) = env::var("POSTGRES_PASSWORD") {
            if !value.is_empty() {
                env::set_var("NEW_DB_PASSWORD", value);
            }
        }
    }

    // DATABASE_URL — reconstruct from parts when not pre-baked. This is
    // the variable `bin/sync.rs`, `bin/writeback.rs`, and the backfill
    // bins read; keeping the reconstruction in this module means the
    // compose file can stop embedding `${POSTGRES_PASSWORD}` in a
    // string literal that ships in `docker exec ... printenv`.
    if !env_already_set("DATABASE_URL") {
        if let Some(url) = reconstruct_database_url() {
            env::set_var("DATABASE_URL", url);
        }
    }

    hydrated_count
}

/// Treat an unset OR empty env var as "not set". An empty value is the
/// failure mode of a botched CI `.env` rewrite (`POSTGRES_PASSWORD=''`)
/// and we want the secret-file path to overwrite it rather than the
/// caller getting a silently-empty password.
fn env_already_set(name: &str) -> bool {
    match env::var(name) {
        Ok(value) => !value.is_empty(),
        Err(_) => false,
    }
}

/// Read `path` if it exists, returning the trimmed contents. A single
/// trailing newline is stripped (common when a deploy writes the value
/// with `printf '%s\n' "$VALUE" > path` or `echo`) — internal
/// whitespace is preserved verbatim because some webhooks / passwords
/// legitimately contain spaces.
fn read_secret_file(path: &Path) -> Option<String> {
    let raw = fs::read_to_string(path).ok()?;
    let trimmed = strip_trailing_newline(&raw);
    if trimmed.is_empty() {
        // An empty file is treated the same as a missing file. Falls
        // through to the env-var fallback (or downstream
        // `require_secret` panic for required values).
        return None;
    }
    Some(trimmed.to_string())
}

/// Strip exactly ONE trailing `\n` (or `\r\n`). Internal newlines are
/// preserved — webhooks shouldn't contain them, but if someone ever
/// puts a multi-line PEM in here we don't want to mangle it.
fn strip_trailing_newline(s: &str) -> &str {
    if let Some(stripped) = s.strip_suffix("\r\n") {
        return stripped;
    }
    if let Some(stripped) = s.strip_suffix('\n') {
        return stripped;
    }
    s
}

/// Build `postgres://user:pass@host:port/db` from the same env-var
/// pieces the backend already reads for `NewDbConfig`. Returns `None`
/// when any of the required parts is missing — the binary's existing
/// "DATABASE_URL or NEW_DATABASE_URL must be set" error message then
/// fires, surfacing the misconfiguration loudly.
///
/// Defaults match the production compose file:
///   * `NEW_DB_SERVER` / fallback `newdb` (the compose service name)
///   * `NEW_DB_PORT` / `PGPORT` / fallback `5439`
///   * `NEW_DB_NAME` / `POSTGRES_DB` / fallback `hotelnew`
///
/// `NEW_DB_NAME` takes precedence over `POSTGRES_DB` so a single
/// `POSTGRES_DB=hotelnew` env (shared across the cluster) can be
/// overridden per-service for HF Ville's `hotelville` worker pair —
/// the Ville `sync-hfville` / `writeback-hfville` services set
/// `NEW_DB_NAME=hotelville` to point at the sibling logical DB.
fn reconstruct_database_url() -> Option<String> {
    let user = env::var("POSTGRES_USER").ok()?;
    let password = env::var("POSTGRES_PASSWORD").ok()?;
    if user.is_empty() || password.is_empty() {
        return None;
    }
    let host = env::var("NEW_DB_SERVER").unwrap_or_else(|_| "newdb".to_string());
    let port = env::var("NEW_DB_PORT")
        .or_else(|_| env::var("PGPORT"))
        .unwrap_or_else(|_| "5439".to_string());
    let db = env::var("NEW_DB_NAME")
        .or_else(|_| env::var("POSTGRES_DB"))
        .unwrap_or_else(|_| "hotelnew".to_string());
    Some(format!("postgres://{user}:{password}@{host}:{port}/{db}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Serialise env-mutating tests — same rationale as `config::tests`.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    /// Snapshot env vars touched by a test, restore on drop.
    struct EnvGuard {
        snapshot: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new(names: &[&'static str]) -> Self {
            let snapshot = names
                .iter()
                .map(|name| (*name, env::var(*name).ok()))
                .collect();
            for name in names {
                env::remove_var(name);
            }
            Self { snapshot }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (name, prior) in &self.snapshot {
                match prior {
                    Some(value) => env::set_var(name, value),
                    None => env::remove_var(name),
                }
            }
        }
    }

    /// Per-test temp directory acting as a fake `/run/secrets`.
    struct SecretsDir {
        path: std::path::PathBuf,
    }

    impl SecretsDir {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "hotel-backend-secrets-test-{}-{}",
                std::process::id(),
                rand_suffix()
            ));
            fs::create_dir_all(&path).expect("create temp secrets dir");
            Self { path }
        }

        fn write(&self, name: &str, contents: &str) {
            fs::write(self.path.join(name), contents).expect("write secret file");
        }
    }

    impl Drop for SecretsDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    /// Lightweight unique suffix — we don't want to pull in `rand` just
    /// for this. Nanosecond timestamps collide rarely enough that a
    /// retry isn't worth coding.
    fn rand_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    }

    #[test]
    fn hydrates_db_password_from_file_when_env_unset() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
            "NEW_DB_SERVER",
            "NEW_DB_PORT",
            "PGPORT",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "from-file-secret");
        env::set_var("SECRETS_DIR", &dir.path);

        let hydrated = hydrate_env_from_secret_files();

        assert_eq!(hydrated, 1);
        assert_eq!(env::var("DB_PASSWORD").unwrap(), "from-file-secret");
    }

    #[test]
    fn existing_env_var_wins_over_secret_file() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "from-file");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("DB_PASSWORD", "from-env");

        hydrate_env_from_secret_files();

        // Caller-provided env var must NOT be overwritten by the secret
        // file. Keeps local-dev / pre-migration deploys working.
        assert_eq!(env::var("DB_PASSWORD").unwrap(), "from-env");
    }

    #[test]
    fn empty_env_var_is_overridden_by_secret_file() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "from-file");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("DB_PASSWORD", ""); // botched CI rewrite mode

        hydrate_env_from_secret_files();

        // Empty value should be replaced from the file — that's the
        // CI .env-rewrite pitfall guarded against here.
        assert_eq!(env::var("DB_PASSWORD").unwrap(), "from-file");
    }

    #[test]
    fn trailing_newline_is_stripped_from_secret_file() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "secret-with-newline\n");
        env::set_var("SECRETS_DIR", &dir.path);

        hydrate_env_from_secret_files();

        assert_eq!(env::var("DB_PASSWORD").unwrap(), "secret-with-newline");
    }

    #[test]
    fn crlf_trailing_newline_is_stripped() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "windows-style\r\n");
        env::set_var("SECRETS_DIR", &dir.path);

        hydrate_env_from_secret_files();

        assert_eq!(env::var("DB_PASSWORD").unwrap(), "windows-style");
    }

    #[test]
    fn empty_secret_file_falls_through_to_env_fallback() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "");
        env::set_var("SECRETS_DIR", &dir.path);

        let hydrated = hydrate_env_from_secret_files();

        // Empty file is skipped — caller's downstream `require_secret`
        // will then fire the loud panic.
        assert_eq!(hydrated, 0);
        assert!(env::var("DB_PASSWORD").is_err());
    }

    #[test]
    fn missing_secret_file_silently_skipped() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        // Intentionally write NO files.
        env::set_var("SECRETS_DIR", &dir.path);

        let hydrated = hydrate_env_from_secret_files();

        assert_eq!(hydrated, 0);
        // No env vars touched.
        assert!(env::var("DB_PASSWORD").is_err());
    }

    #[test]
    fn reconstructs_database_url_when_unset() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
            "NEW_DB_SERVER",
            "NEW_DB_PORT",
            "PGPORT",
        ]);

        let dir = SecretsDir::new();
        dir.write("postgres_password", "pg-pass");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("POSTGRES_USER", "postgres");
        env::set_var("POSTGRES_DB", "hotelnew");
        env::set_var("NEW_DB_SERVER", "newdb");
        env::set_var("PGPORT", "5439");

        hydrate_env_from_secret_files();

        assert_eq!(
            env::var("DATABASE_URL").unwrap(),
            "postgres://postgres:pg-pass@newdb:5439/hotelnew"
        );
    }

    #[test]
    fn new_db_name_overrides_postgres_db_for_hfville() {
        // The HF Ville worker pair (sync-hfville / writeback-hfville)
        // shares the `POSTGRES_*` credentials with HF Hotel but points
        // at the sibling `hotelville` logical DB. NEW_DB_NAME is the
        // per-service override that makes this possible without
        // dragging in a separate POSTGRES_PASSWORD secret.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
            "NEW_DB_NAME",
            "NEW_DB_SERVER",
            "NEW_DB_PORT",
            "PGPORT",
        ]);

        let dir = SecretsDir::new();
        dir.write("postgres_password", "pg-pass");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("POSTGRES_USER", "postgres");
        env::set_var("POSTGRES_DB", "hotelnew"); // cluster default
        env::set_var("NEW_DB_NAME", "hotelville"); // Ville override

        hydrate_env_from_secret_files();

        assert_eq!(
            env::var("DATABASE_URL").unwrap(),
            "postgres://postgres:pg-pass@newdb:5439/hotelville"
        );
    }

    #[test]
    fn preserves_existing_database_url() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
        ]);

        let dir = SecretsDir::new();
        dir.write("postgres_password", "pg-pass");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("POSTGRES_USER", "postgres");
        env::set_var("POSTGRES_DB", "hotelnew");
        env::set_var(
            "DATABASE_URL",
            "postgres://operator-override:pre-baked@example:9999/custom",
        );

        hydrate_env_from_secret_files();

        // Operator-provided URL wins — local-dev override path stays
        // intact, and the writeback worker's pre-baked URL (kept as a
        // compose fallback during the migration) also wins.
        assert_eq!(
            env::var("DATABASE_URL").unwrap(),
            "postgres://operator-override:pre-baked@example:9999/custom"
        );
    }

    #[test]
    fn skips_database_url_reconstruction_when_pieces_missing() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
        ]);

        let dir = SecretsDir::new();
        // No postgres_password file, no POSTGRES_USER env.
        env::set_var("SECRETS_DIR", &dir.path);

        hydrate_env_from_secret_files();

        // Reconstruction needs user + password; with neither available
        // the helper leaves DATABASE_URL unset so the binary's
        // existing "must be set" error fires instead.
        assert!(env::var("DATABASE_URL").is_err());
    }

    #[test]
    fn aliases_postgres_password_to_new_db_password() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
        ]);

        let dir = SecretsDir::new();
        dir.write("postgres_password", "shared-pg-pass");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("POSTGRES_USER", "postgres");
        env::set_var("POSTGRES_DB", "hotelnew");

        hydrate_env_from_secret_files();

        // `NewDbConfig::from_env` still calls `require_secret("NEW_DB_PASSWORD")`,
        // so the alias is load-bearing — without it the backend panics
        // on startup despite the secret file being present.
        assert_eq!(env::var("NEW_DB_PASSWORD").unwrap(), "shared-pg-pass");
    }

    #[test]
    fn preserves_explicit_new_db_password() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
        ]);

        let dir = SecretsDir::new();
        dir.write("postgres_password", "secret-pg-pass");
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("NEW_DB_PASSWORD", "explicit-override");

        hydrate_env_from_secret_files();

        // Explicit env var must NOT be overwritten by the alias. This
        // matters when an operator wants a different PG password for
        // application access vs. superuser, even though current prod
        // uses the same value for both.
        assert_eq!(env::var("NEW_DB_PASSWORD").unwrap(), "explicit-override");
    }

    #[test]
    fn hydrates_multiple_secrets_in_one_pass() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&[
            "SECRETS_DIR",
            "DB_PASSWORD",
            "POSTGRES_PASSWORD",
            "VILLE_DB_PASSWORD",
            "NEW_DB_PASSWORD",
            "SLACK_WEBHOOK_URL",
            "DATABASE_URL",
            "POSTGRES_USER",
            "POSTGRES_DB",
        ]);

        let dir = SecretsDir::new();
        dir.write("db_password", "mssql-pass");
        dir.write("postgres_password", "pg-pass");
        dir.write(
            "slack_webhook_url",
            "https://hooks.slack.com/services/T00/B00/XYZ",
        );
        env::set_var("SECRETS_DIR", &dir.path);
        env::set_var("POSTGRES_USER", "postgres");
        env::set_var("POSTGRES_DB", "hotelnew");

        let hydrated = hydrate_env_from_secret_files();

        assert_eq!(hydrated, 3);
        assert_eq!(env::var("DB_PASSWORD").unwrap(), "mssql-pass");
        assert_eq!(env::var("POSTGRES_PASSWORD").unwrap(), "pg-pass");
        assert_eq!(
            env::var("SLACK_WEBHOOK_URL").unwrap(),
            "https://hooks.slack.com/services/T00/B00/XYZ"
        );
        // DATABASE_URL reconstruction picks up the just-hydrated
        // POSTGRES_PASSWORD on the same pass.
        assert_eq!(
            env::var("DATABASE_URL").unwrap(),
            "postgres://postgres:pg-pass@newdb:5439/hotelnew"
        );
    }
}
