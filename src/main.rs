use dotenv::dotenv;
use rust_store::{create_router, print_server_info};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the application router with all routes and middleware
    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");
    let app = create_router(pool.clone());

    // Create TCP listener
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    // Print server startup information
    print_server_info();

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
