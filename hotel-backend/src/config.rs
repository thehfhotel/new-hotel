//! Application configuration from environment variables

use std::env;

/// Database configuration for legacy database
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub server: String,
    pub database: String,
    pub user: String,
    pub password: String,
    pub pool_max: u32,
}

impl DbConfig {
    pub fn from_env() -> Self {
        Self {
            server: env::var("DB_SERVER").unwrap_or_else(|_| "192.168.100.222".to_string()),
            database: env::var("DB_NAME").unwrap_or_else(|_| "db".to_string()),
            user: env::var("DB_USER").unwrap_or_else(|_| "sa".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string()),
            pool_max: env::var("DB_POOL_MAX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        }
    }
}

/// Database configuration for new HotelNew database
#[derive(Debug, Clone)]
pub struct NewDbConfig {
    pub server: String,
    pub database: String,
    pub user: String,
    pub password: String,
    pub pool_max: u32,
}

impl NewDbConfig {
    pub fn from_env() -> Self {
        Self {
            server: env::var("NEW_DB_SERVER").unwrap_or_else(|_| "192.168.100.222".to_string()),
            database: env::var("NEW_DB_NAME").unwrap_or_else(|_| "HotelNew".to_string()),
            user: env::var("NEW_DB_USER").unwrap_or_else(|_| "sa".to_string()),
            password: env::var("NEW_DB_PASSWORD").unwrap_or_else(|_| "***REMOVED***".to_string()),
            pool_max: env::var("NEW_DB_POOL_MAX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        }
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

/// Complete application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db: DbConfig,
    pub new_db: NewDbConfig,
    pub mode: SystemMode,
    pub slack: SlackConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            db: DbConfig::from_env(),
            new_db: NewDbConfig::from_env(),
            mode: SystemMode::from_env(),
            slack: SlackConfig::from_env(),
            server: ServerConfig::from_env(),
        }
    }
}
