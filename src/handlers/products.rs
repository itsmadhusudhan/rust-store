use crate::schema::products::dsl::*;
use async_graphql::{Context, Object, Result, SimpleObject};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::deadpool::Pool};
use serde::Serialize;

pub struct QueryRoot;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(SimpleObject, Debug)]
pub struct ProductGQL {
    id: String,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
}

impl From<Product> for ProductGQL {
    fn from(p: Product) -> Self {
        Self {
            id: p.id.to_string(),
            name: Some(p.name),
            description: p.description,
            status: Some(p.status),
        }
    }
}

#[Object]
impl QueryRoot {
    async fn products(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<ProductGQL>> {
        let pool = ctx.data_unchecked::<Pool<AsyncPgConnection>>();
        let mut conn = pool
            .get()
            .await
            .map_err(|e| async_graphql::Error::new(format!("Database connection error: {}", e)))?;

        // Dynamically build SELECT
        // For demo: just include/exclude columns manually
        // (diesel-dynamic-schema crates can help if you want fully dynamic projections)
        let mut query = products.into_boxed();

        // Always fetch id
        // Diesel requires selecting a known struct shape, so we fetch all & filter after
        // ⚠️ To *truly* select minimal columns, you’d need custom tuples or raw SQL.

        let results: Vec<Product> = query
            .limit(limit.unwrap_or(20) as i64)
            .select(Product::as_select())
            .load(&mut conn)
            .await?;

        // Map into GraphQL objects
        Ok(results
            .into_iter()
            .map(|p| ProductGQL {
                id: p.id.to_string(),
                name: Some(p.name),
                description: p.description,
                status: Some(p.status),
            })
            .collect())
    }
}
