use async_graphql::{ID, SimpleObject};
use serde_json::Value;

/// Main product entity with all associated data
#[derive(SimpleObject, Clone, Debug)]
pub struct Product {
    pub id: ID,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub variants: Vec<ProductVariant>,
    pub categories: Vec<Category>,
}

/// Product variant with pricing and stock information
#[derive(SimpleObject, Clone, Debug)]
pub struct ProductVariant {
    pub id: ID,
    pub sku: String,
    pub price: Money,
    pub stock_quantity: i32,
    /// Flexible JSON attributes for size, color, etc.
    pub attributes: Value,
}

/// Hierarchical category structure
#[derive(SimpleObject, Clone, Debug)]
pub struct Category {
    pub id: ID,
    pub name: String,
    pub slug: String,
    pub children: Vec<Category>,
    pub parent: Option<Box<Category>>,
}

/// Namespaced attribute for extensible product properties
#[derive(SimpleObject, Clone, Debug)]
pub struct Attribute {
    pub namespace: String,
    pub key: String,
    pub value: String,
}

/// Money type with amount and currency
#[derive(SimpleObject, Clone, Debug)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

// Input types for mutations
#[derive(async_graphql::InputObject)]
pub struct CreateProductInput {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(async_graphql::InputObject)]
pub struct UpdateProductInput {
    pub id: ID,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(async_graphql::InputObject)]
pub struct CreateCategoryInput {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<ID>,
}

#[derive(async_graphql::InputObject)]
pub struct CreateProductVariantInput {
    pub product_id: ID,
    pub sku: String,
    pub price: MoneyInput,
    pub stock_quantity: i32,
    pub attributes: Value,
}

#[derive(async_graphql::InputObject)]
pub struct MoneyInput {
    pub amount: f64,
    pub currency: String,
}
