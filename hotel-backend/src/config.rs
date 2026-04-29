//! Application configuration from environment variables

use std::env;

/// Default TCP port for SQL Server (1433). Overridden at HF Ville to 1436
/// via the `MSSQL_PORT` env var because the new SS2025 Express instance
/// listens on a non-default port. Kept as a named constant so the choice
/// is self-documenting at every call site.
const DEFAULT_MSSQL_PORT: u16 = 1433;

/// Database configuration for legacy database
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub server: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub pool_max: u32,
}

impl DbConfig {
    /// Build the legacy-MSSQL config from environment variables.
    ///
    /// `DB_PASSWORD` is REQUIRED — there is intentionally no fallback.
    /// A missing or empty value panics here so a deployment with a
    /// misconfigured GitHub secret never silently tries (and possibly
    /// succeeds with) hard-coded default credentials. See
    /// `docs/runbook-sync.md` for the `.env`-rewrite pitfall this guards
    /// against.
    pub fn from_env() -> Self {
        // Pool sizing: prefer the explicit `MSSQL_POOL_MAX_SIZE`
        // (introduced 2.49.2 to give operators a clearly-named knob for
        // the legacy MSSQL bb8 pool shared by writeback + sync +
        // ville-sync), falling back to the historical `DB_POOL_MAX`
        // for backward compatibility. Default bumped 10 → 20 to give
        // headroom to the Phase 5 sync worker; tighten via env if
        // legacy MSSQL ever pushes back on connection count.
        let pool_max = env::var("MSSQL_POOL_MAX_SIZE")
            .or_else(|_| env::var("DB_POOL_MAX"))
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(20);

        Self {
            server: env::var("DB_SERVER").unwrap_or_else(|_| "192.168.100.222".to_string()),
            port: parse_mssql_port("MSSQL_PORT", DEFAULT_MSSQL_PORT),
            database: env::var("DB_NAME").unwrap_or_else(|_| "db".to_string()),
            user: env::var("DB_USER").unwrap_or_else(|_| "sa".to_string()),
            password: require_secret("DB_PASSWORD"),
            pool_max,
        }
    }
}

/// Read a u16 port from `var_name`, defaulting to `default_port` when
/// the env var is unset. Panics with a clear message on a malformed
/// value (e.g. `MSSQL_PORT=garbage`) so a typo can never silently fall
/// back to the default and connect to the wrong instance.
fn parse_mssql_port(var_name: &str, default_port: u16) -> u16 {
    match env::var(var_name) {
        Ok(raw) => raw.parse::<u16>().unwrap_or_else(|err| {
            panic!(
                "{} must be a valid TCP port (1-65535), got `{}`: {}",
                var_name, raw, err
            )
        }),
        Err(_) => default_port,
    }
}

/// Read a required secret from `var_name`. Panics loudly if missing OR
/// empty — empty-string secrets are treated as missing because a
/// misconfigured CI substitution can produce `VAR=''` in the rendered
/// `.env`, which `env::var` happily returns as `Ok("")`.
fn require_secret(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(value) if !value.is_empty() => value,
        _ => panic!(
            "{} environment variable is required (must be set and non-empty). \
             Refusing to fall back to a default password — fail loud per the \
             docs/runbook-sync.md secret-rewrite pitfall.",
            var_name
        ),
    }
}

/// Database configuration for new HotelNew database (PostgreSQL)
#[derive(Debug, Clone)]
pub struct NewDbConfig {
    pub server: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub pool_max: u32,
}

impl NewDbConfig {
    pub fn from_env() -> Self {
        Self {
            server: env::var("NEW_DB_SERVER").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("NEW_DB_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5439),
            database: env::var("NEW_DB_NAME").unwrap_or_else(|_| "hotelnew".to_string()),
            user: env::var("NEW_DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("NEW_DB_PASSWORD").unwrap_or_else(|_| "12345678".to_string()),
            pool_max: env::var("NEW_DB_POOL_MAX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        }
    }

    /// Build PostgreSQL connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.server, self.port, self.database
        )
    }
}

/// System mode determining which database to use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemMode {
    /// Use legacy database (default)
    #[default]
    Legacy,
    /// Use new HotelNew database
    New,
}

impl SystemMode {
    pub fn from_env() -> Self {
        match env::var("SYSTEM_MODE").as_deref() {
            Ok("new") | Ok("New") | Ok("NEW") => SystemMode::New,
            _ => SystemMode::Legacy,
        }
    }

    pub fn is_legacy(&self) -> bool {
        matches!(self, SystemMode::Legacy)
    }

    pub fn is_new(&self) -> bool {
        matches!(self, SystemMode::New)
    }
}

/// Site identity configuration (Phase 5 / multi-site).
///
/// The Rust+Axum backend ships as a single binary that runs as one
/// process per site (HF Hotel + HF Ville from Phase 5 onward). Site
/// identity is purely a runtime config concern for observability —
/// PG topology is per-DB so the schema never carries a `site` column.
///
/// `id` is threaded into:
///   * Slack message text (operator triage when both instances POST to
///     the same webhook),
///   * tracing spans on the long-lived loops in `bin/sync.rs` and
///     `bin/writeback.rs` (so log lines carry `site=<id>`),
///   * the `/health` JSON body (so external monitors / smoke tests
///     can confirm they hit the right deployment).
///
/// The validator deliberately rejects unknown values rather than
/// silently accepting a typo — a typo in the env var would otherwise
/// produce alerts for a non-existent site nobody's on call for.
#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub id: String,
}

/// Sites this binary is allowed to identify as. Adding a new site
/// requires a code change here (and a per-site fingerprint baseline,
/// per task #70). Centralised so `SiteConfig::from_env` and the unit
/// tests reference the same authoritative list.
pub const SUPPORTED_SITE_IDS: &[&str] = &["hfhotel", "hfville"];

impl SiteConfig {
    /// Read `SITE_ID` from env. Defaults to `"hfhotel"` for back-compat
    /// with the pre-Phase-5 single-site deploy. Panics on an unknown
    /// value so a typo (`SITE_ID=hfvilel`) fails loud at startup
    /// instead of silently routing alerts to a non-existent site.
    pub fn from_env() -> Self {
        let raw = env::var("SITE_ID").unwrap_or_else(|_| "hfhotel".to_string());
        Self::parse(&raw)
    }

    /// Pure parsing helper extracted so the validation contract is
    /// unit-testable without env mutation.
    pub fn parse(raw: &str) -> Self {
        let trimmed = raw.trim();
        if !SUPPORTED_SITE_IDS.contains(&trimmed) {
            panic!(
                "Invalid SITE_ID={trimmed:?}; expected one of {SUPPORTED_SITE_IDS:?}. \
                 Set SITE_ID=hfhotel or SITE_ID=hfville at deploy time."
            );
        }
        Self { id: trimmed.to_string() }
    }

    /// Convenience: the upper-cased site id used to suffix per-site
    /// env-var overrides (e.g. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE`).
    pub fn id_upper(&self) -> String {
        self.id.to_uppercase()
    }
}

/// Slack configuration
#[derive(Debug, Clone)]
pub struct SlackConfig {
    pub webhook_url: Option<String>,
    pub enabled: bool,
}

impl SlackConfig {
    pub fn from_env() -> Self {
        let enabled = env::var("SLACK_NOTIFICATIONS_ENABLED")
            .map(|v| v != "false")
            .unwrap_or(true);

        Self {
            webhook_url: env::var("SLACK_WEBHOOK_URL").ok(),
            enabled,
        }
    }

    pub fn is_configured(&self) -> bool {
        self.enabled && self.webhook_url.is_some()
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3003),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Database configuration for HF Ville mirror (PostgreSQL via cloudflared tunnel)
#[derive(Debug, Clone)]
pub struct VilleDbConfig {
    pub server: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub pool_max: u32,
    pub enabled: bool,
}

impl VilleDbConfig {
    /// Build the HF Ville mirror config from environment variables.
    ///
    /// Mirrors `DbConfig::from_env`'s policy: `VILLE_DB_PASSWORD` is
    /// REQUIRED (no default) — but only when `VILLE_DB_ENABLED=true`.
    /// When the mirror is disabled (the default at HF Hotel), the
    /// password is allowed to stay empty so an operator without the
    /// secret provisioned can still boot the backend.
    pub fn from_env() -> Self {
        let enabled = env::var("VILLE_DB_ENABLED")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);

        // Only enforce the secret requirement when the mirror is active.
        // Skipping the panic for the disabled-default keeps backward
        // compatibility for HF Hotel deployments that have never set
        // VILLE_DB_PASSWORD.
        let password = if enabled {
            require_secret("VILLE_DB_PASSWORD")
        } else {
            env::var("VILLE_DB_PASSWORD").unwrap_or_default()
        };

        Self {
            server: env::var("VILLE_DB_SERVER").unwrap_or_else(|_| "ville-tunnel".to_string()),
            port: env::var("VILLE_DB_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5440),
            database: env::var("VILLE_DB_NAME").unwrap_or_else(|_| "hfville".to_string()),
            user: env::var("VILLE_DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password,
            pool_max: env::var("VILLE_DB_POOL_MAX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            enabled,
        }
    }

    /// Build PostgreSQL connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.server, self.port, self.database
        )
    }
}

/// Complete application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db: DbConfig,
    pub new_db: NewDbConfig,
    pub ville_db: VilleDbConfig,
    pub mode: SystemMode,
    pub slack: SlackConfig,
    pub server: ServerConfig,
    pub site: SiteConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            db: DbConfig::from_env(),
            new_db: NewDbConfig::from_env(),
            ville_db: VilleDbConfig::from_env(),
            mode: SystemMode::from_env(),
            slack: SlackConfig::from_env(),
            server: ServerConfig::from_env(),
            site: SiteConfig::from_env(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Serialise env-mutating tests. `std::env::set_var` mutates
    /// process-wide state — running these in parallel under
    /// `cargo test` (which uses one process) would cause flakes when
    /// one test clears `DB_PASSWORD` while another asserts it parses.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    /// Snapshot the env vars we touch, restore them on drop. Lets each
    /// test mutate freely without leaking state to siblings.
    struct EnvGuard {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new(names: &[&'static str]) -> Self {
            let vars = names
                .iter()
                .map(|name| (*name, env::var(*name).ok()))
                .collect();
            // Start every test from a known-clean slate.
            for name in names {
                env::remove_var(name);
            }
            Self { vars }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (name, prior) in &self.vars {
                match prior {
                    Some(value) => env::set_var(name, value),
                    None => env::remove_var(name),
                }
            }
        }
    }

    fn with_password_set(test: impl FnOnce()) {
        env::set_var("DB_PASSWORD", "test-password");
        test();
    }

    #[test]
    fn from_env_defaults_port_to_1433_when_mssql_port_unset() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        with_password_set(|| {
            let config = DbConfig::from_env();
            assert_eq!(config.port, 1433);
        });
    }

    #[test]
    fn from_env_parses_explicit_mssql_port_1436() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        env::set_var("MSSQL_PORT", "1436");
        with_password_set(|| {
            let config = DbConfig::from_env();
            assert_eq!(config.port, 1436);
        });
    }

    #[test]
    #[should_panic(expected = "MSSQL_PORT must be a valid TCP port")]
    fn from_env_panics_on_garbage_mssql_port() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        env::set_var("MSSQL_PORT", "not-a-port");
        with_password_set(|| {
            let _ = DbConfig::from_env();
        });
    }

    #[test]
    #[should_panic(expected = "DB_PASSWORD environment variable is required")]
    fn from_env_panics_when_db_password_missing() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        // Intentionally do NOT set DB_PASSWORD — must fail loud.
        let _ = DbConfig::from_env();
    }

    #[test]
    #[should_panic(expected = "DB_PASSWORD environment variable is required")]
    fn from_env_panics_when_db_password_empty_string() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        // Empty string is the failure mode produced by a botched CI
        // `.env` rewrite (`DB_PASSWORD=''`). Must be rejected too.
        env::set_var("DB_PASSWORD", "");
        let _ = DbConfig::from_env();
    }

    #[test]
    fn from_env_propagates_db_password_value() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["MSSQL_PORT", "DB_PASSWORD"]);

        env::set_var("DB_PASSWORD", "s3cret-from-vault");
        let config = DbConfig::from_env();
        assert_eq!(config.password, "s3cret-from-vault");
    }

    // -------------------------------------------------------------------
    // SiteConfig (task #69) — SITE_ID env var validation
    // -------------------------------------------------------------------

    #[test]
    fn site_config_defaults_to_hfhotel_when_unset() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["SITE_ID"]);

        let site = SiteConfig::from_env();
        assert_eq!(site.id, "hfhotel");
    }

    #[test]
    fn site_config_accepts_hfville() {
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["SITE_ID"]);

        env::set_var("SITE_ID", "hfville");
        let site = SiteConfig::from_env();
        assert_eq!(site.id, "hfville");
    }

    #[test]
    fn site_config_trims_whitespace() {
        // A `.env` rewrite that leaves a trailing newline would otherwise
        // invalidate `hfhotel\n`.
        // `unwrap_or_else(|p| p.into_inner())`: a `#[should_panic]`
        // sibling will poison the mutex on success — recover the guard
        // so the next test still serialises correctly.
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let _guard = EnvGuard::new(&["SITE_ID"]);

        env::set_var("SITE_ID", "  hfville  ");
        let site = SiteConfig::from_env();
        assert_eq!(site.id, "hfville");
    }

    #[test]
    #[should_panic(expected = "Invalid SITE_ID")]
    fn site_config_rejects_unknown_value() {
        // Pure parse — no env mutation needed, so no lock either.
        let _ = SiteConfig::parse("garbage");
    }

    #[test]
    #[should_panic(expected = "Invalid SITE_ID")]
    fn site_config_rejects_typo_close_to_valid_value() {
        // Guards the documented motivation: a typo like "hfvilel" must
        // NOT silently accept — would otherwise route alerts to a
        // non-existent on-call rotation.
        let _ = SiteConfig::parse("hfvilel");
    }

    #[test]
    fn site_config_id_upper_for_env_suffix() {
        // Used by `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>` lookup.
        let site = SiteConfig::parse("hfville");
        assert_eq!(site.id_upper(), "HFVILLE");
    }
}
