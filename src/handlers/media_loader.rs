use std::{collections::HashMap, sync::Arc};

use async_graphql::{SimpleObject, dataloader::Loader};
use sqlx::{PgPool, Row, postgres::PgRow};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProductMediaLoadKey {
    pub product_id: i32,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, SimpleObject)]
pub struct Media {
    pub id: i32,
    pub product_id: i32,
    pub category_id: i32,
    pub is_primary: bool,
}

pub struct ProductMediaLoader {
    pub pool: PgPool,
}

impl Loader<ProductMediaLoadKey> for ProductMediaLoader {
    type Value = Vec<Media>;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[ProductMediaLoadKey],
    ) -> Result<HashMap<ProductMediaLoadKey, Self::Value>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        // Use columns from the first key since they're the same for all keys in a single query
        let columns = &keys[0].columns;
        let product_ids: Vec<i32> = keys.iter().map(|k| k.product_id).collect();

        let mut safe_columns = vec!["id", "product_id"];
        for col in columns {
            safe_columns.push(col)
        }
        safe_columns.dedup();

        let sql = format!(
            "SELECT {} FROM product_category_junction WHERE product_id = ANY($1)",
            safe_columns.join(", ")
        );

        let rows = sqlx::query(&sql)
            .bind(&product_ids)
            .map(|row: PgRow| {
                return Media {
                    id: row.get("id"),
                    product_id: row.try_get("product_id").unwrap_or_default(),
                    category_id: row.try_get("category_id").unwrap_or_default(),
                    is_primary: row.try_get("is_primary").unwrap_or_default(),
                };
            })
            .fetch_all(&self.pool)
            .await?;

        // Create result map for each key
        let mut result_map: HashMap<ProductMediaLoadKey, Vec<Media>> = HashMap::new();
        for key in keys {
            let variants: Vec<Media> = rows
                .iter()
                .filter(|v| v.product_id == key.product_id)
                .cloned()
                .collect();
            result_map.insert(key.clone(), variants);
        }

        Ok(result_map)
    }
}
