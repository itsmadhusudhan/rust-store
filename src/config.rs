use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub use_database: bool,
    pub database_url: Option<String>,
    pub database_max_connections: u32,
}

impl AppConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let use_database = env::var("USE_DATABASE")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let database_url = env::var("DATABASE_URL").ok();

        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        Self {
            use_database,
            database_url,
            database_max_connections,
        }
    }

    /// Create configuration for development (mock data)
    pub fn development() -> Self {
        Self {
            use_database: false,
            database_url: None,
            database_max_connections: 10,
        }
    }

    /// Create configuration for production (database)
    pub fn production(database_url: String) -> Self {
        Self {
            use_database: true,
            database_url: Some(database_url),
            database_max_connections: 10,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
