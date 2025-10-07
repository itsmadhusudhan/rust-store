//! Rust Store - A GraphQL-based e-commerce API
//!
//! This crate provides a complete GraphQL API for managing products, categories,
//! and product variants in an e-commerce application.

pub mod domain;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;

// Re-export commonly used types for convenience
pub use domain::*;
pub use models::*;
pub use routes::{ApiSchema, create_router, create_schema, print_server_info};
