use crate::domain::*;
use async_graphql::ID;
use serde_json;
use std::collections::HashMap;

/// Mock data store for development and testing
/// In production, this would be replaced with database access layer
#[derive(Clone)]
pub struct DataStore {
    pub products: HashMap<String, Product>,
    pub categories: HashMap<String, Category>,
}

impl DataStore {
    /// Initialize the data store with sample data
    pub fn new() -> Self {
        let mut products = HashMap::new();
        let mut categories = HashMap::new();

        // Create sample categories
        let electronics = Category {
            id: ID("1".to_string()),
            name: "Electronics".to_string(),
            slug: "electronics".to_string(),
            children: vec![],
            parent: None,
        };

        let laptops = Category {
            id: ID("2".to_string()),
            name: "Laptops".to_string(),
            slug: "laptops".to_string(),
            children: vec![],
            parent: Some(Box::new(electronics.clone())),
        };

        let accessories = Category {
            id: ID("3".to_string()),
            name: "Accessories".to_string(),
            slug: "accessories".to_string(),
            children: vec![],
            parent: Some(Box::new(electronics.clone())),
        };

        categories.insert("1".to_string(), electronics);
        categories.insert("2".to_string(), laptops.clone());
        categories.insert("3".to_string(), accessories.clone());

        // Create sample products
        let macbook = Product {
            id: ID("1".to_string()),
            name: "MacBook Pro".to_string(),
            slug: "macbook-pro".to_string(),
            description: Some("High-performance laptop for professionals".to_string()),
            status: "ACTIVE".to_string(),
            variants: vec![
                ProductVariant {
                    id: ID("1".to_string()),
                    sku: "MBP-13-256".to_string(),
                    price: Money {
                        amount: 1299.99,
                        currency: "USD".to_string(),
                    },
                    stock_quantity: 50,
                    attributes: serde_json::json!({
                        "color": "Space Gray",
                        "storage": "256GB",
                        "screen_size": "13-inch"
                    }),
                },
                ProductVariant {
                    id: ID("2".to_string()),
                    sku: "MBP-13-512".to_string(),
                    price: Money {
                        amount: 1499.99,
                        currency: "USD".to_string(),
                    },
                    stock_quantity: 30,
                    attributes: serde_json::json!({
                        "color": "Space Gray",
                        "storage": "512GB",
                        "screen_size": "13-inch"
                    }),
                },
            ],
            categories: vec![laptops.clone()],
            media: vec![
                ProductMedia {
                    url: "https://example.com/macbook-pro-1.jpg".to_string(),
                    r#type: "image".to_string(),
                    sort_order: Some(1),
                },
                ProductMedia {
                    url: "https://example.com/macbook-pro-2.jpg".to_string(),
                    r#type: "image".to_string(),
                    sort_order: Some(2),
                },
            ],
        };

        let mouse = Product {
            id: ID("2".to_string()),
            name: "Magic Mouse".to_string(),
            slug: "magic-mouse".to_string(),
            description: Some("Wireless Bluetooth mouse with multi-touch surface".to_string()),
            status: "ACTIVE".to_string(),
            variants: vec![ProductVariant {
                id: ID("3".to_string()),
                sku: "MM-WHITE".to_string(),
                price: Money {
                    amount: 79.99,
                    currency: "USD".to_string(),
                },
                stock_quantity: 100,
                attributes: serde_json::json!({
                    "color": "White",
                    "connectivity": "Bluetooth",
                    "battery_type": "Built-in rechargeable"
                }),
            }],
            categories: vec![accessories],
            media: vec![ProductMedia {
                url: "https://example.com/magic-mouse.jpg".to_string(),
                r#type: "image".to_string(),
                sort_order: Some(1),
            }],
        };

        products.insert("1".to_string(), macbook);
        products.insert("2".to_string(), mouse);

        Self {
            products,
            categories,
        }
    }

    /// Get all products
    pub fn get_products(&self) -> Vec<Product> {
        self.products.values().cloned().collect()
    }

    /// Get product by ID
    pub fn get_product(&self, id: &str) -> Option<Product> {
        self.products.get(id).cloned()
    }

    /// Get product by slug
    pub fn get_product_by_slug(&self, slug: &str) -> Option<Product> {
        self.products.values().find(|p| p.slug == slug).cloned()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<Category> {
        self.categories.values().cloned().collect()
    }

    /// Get category by ID
    pub fn get_category(&self, id: &str) -> Option<Category> {
        self.categories.get(id).cloned()
    }

    /// Search products by name or description
    pub fn search_products(&self, query: &str, limit: usize) -> Vec<Product> {
        self.products
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query.to_lowercase())
                    || p.description
                        .as_ref()
                        .map_or(false, |d| d.to_lowercase().contains(&query.to_lowercase()))
            })
            .take(limit)
            .cloned()
            .collect()
    }
}

impl Default for DataStore {
    fn default() -> Self {
        Self::new()
    }
}
