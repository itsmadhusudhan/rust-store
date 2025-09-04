//! Rust Store - A GraphQL-based e-commerce API
//!
//! This crate provides a complete GraphQL API for managing products, categories,
//! and product variants in an e-commerce application.

pub mod data;
pub mod domain;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;

// Re-export commonly used types for convenience
pub use data::DataStore;
pub use domain::*;
pub use handlers::{Mutation, Query};
pub use models::*;
pub use routes::{ApiSchema, create_router, create_schema, print_server_info};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_store_creation() {
        let store = DataStore::new();
        assert!(!store.get_products().is_empty());
        assert!(!store.get_categories().is_empty());
    }

    #[test]
    fn test_product_search() {
        let store = DataStore::new();
        let results = store.search_products("macbook", 10);
        assert!(!results.is_empty());
        assert!(results[0].name.to_lowercase().contains("macbook"));
    }

    #[test]
    fn test_category_retrieval() {
        let store = DataStore::new();
        let categories = store.get_categories();
        assert!(!categories.is_empty());

        // Test getting specific category
        let electronics = store.get_category("1");
        assert!(electronics.is_some());
        assert_eq!(electronics.unwrap().name, "Electronics");
    }
}
