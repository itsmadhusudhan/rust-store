use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
use std::env;

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: usize,
    pub min_connections: usize,
    pub connection_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/rust_store".to_string()),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: 30,
        }
    }
}

/// Create a new database connection pool
pub async fn create_pool(
    config: DatabaseConfig,
) -> Result<Pool<AsyncPgConnection>, Box<dyn std::error::Error>> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);

    let pool = Pool::builder(manager)
        .max_size(config.max_connections)
        .build()
        .expect("Failed to create pool");

    Ok(pool)
}

/// Database connection trait for dependency injection
#[async_trait::async_trait]
pub trait DatabaseConnection {
    async fn get_connection(&self) -> Result<AsyncPgConnection, Box<dyn std::error::Error>>;
}

/// Database pool wrapper implementing the connection trait
#[derive(Clone)]
pub struct DatabasePool {
    pool: Pool<AsyncPgConnection>,
}

impl DatabasePool {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }

    pub async fn from_config(config: DatabaseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = create_pool(config).await?;
        Ok(Self::new(pool))
    }
}
