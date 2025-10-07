use async_graphql::connection::PageInfo;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

pub struct ProductEdge {
    pub cursor: String,
    pub node: Product,
}

pub struct ProductConnection {
    pub edges: Vec<ProductEdge>,
    pub nodes: Vec<Product>,
    pub page_info: PageInfo,
}
