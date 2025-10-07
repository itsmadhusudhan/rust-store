use crate::handlers::media_loader::ProductMediaLoader;
use crate::handlers::queries::QueryRoot;
use crate::handlers::variant_loader::VariantLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQL, GraphQLRequest, GraphQLResponse};
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::Extension,
    response::{Html, Json},
    routing::{get, post},
};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

/// GraphQL Schema type alias
pub type ApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Create and configure the GraphQL schema
pub fn create_schema(pool: Pool<Postgres>) -> ApiSchema {
    let variant_loader = VariantLoader { pool: pool.clone() };
    let media_loader = ProductMediaLoader { pool: pool.clone() };
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(DataLoader::new(variant_loader, tokio::spawn))
        .data(DataLoader::new(media_loader, tokio::spawn))
        .data(pool)
        .finish()
}

/// GraphQL handler for processing GraphQL requests
pub async fn graphql_handler(schema: Extension<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL Playground handler for development
pub async fn graphql_playground() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <link href="https://cdn.jsdelivr.net/npm/graphql-playground-react@1.7.26/build/static/css/index.css" rel="stylesheet" />
        </head>
        <body>
            <div id="root"></div>
            <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react@1.7.26/build/static/js/middleware.js"></script>
            <script>
                window.addEventListener('load', function (event) {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/graphql'
                    })
                })
            </script>
        </body>
        </html>
        "#,
    )
}

/// Health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "rust-store-graphql",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

/// Build the complete application router with all routes and middleware
pub fn create_router(pool: Pool<Postgres>) -> Router {
    let schema = create_schema(pool);

    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        // GraphQL endpoint
        .route("/graphql", post(graphql_handler))
        // GraphQL Playground for development
        .route("/playground", get(graphql_playground))
        // Add GraphQL schema as extension
        .layer(Extension(schema))
        // Add CORS layer for web clients
        .layer(CorsLayer::permissive())
}

/// Print server startup information
pub fn print_server_info() {
    println!("🚀 GraphQL Server running on http://0.0.0.0:3000");
    println!("📚 GraphQL Documentation:");
    println!("  POST   /graphql           - GraphQL endpoint");
    println!("  GET    /playground        - GraphQL Playground (development)");
    println!("  GET    /health            - Health check");
    println!("");
    println!("🎯 Example GraphQL Queries:");
    println!("  # Get all products");
    println!("  query {{ products {{ id name slug status }} }}");
    println!("");
    println!("  # Get product by ID");
    println!(
        "  query {{ product(id: \"1\") {{ id name description variants {{ sku price {{ amount currency }} }} }} }}"
    );
}
