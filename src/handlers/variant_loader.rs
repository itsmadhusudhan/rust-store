use async_graphql::{Object, dataloader::*};
use rust_decimal::Decimal;
use sqlx::{PgPool, Row, postgres::PgRow};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariantLoadKey {
    pub product_id: i32,
    pub columns: Vec<String>,
    pub sku: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VariantGQL {
    pub id: i32,
    pub product_id: i32,
    pub sku: String,
    pub price_amount: Decimal,
    pub price_currency: String,
    pub stock_quantity: i32,
    pub is_active: bool,
    pub attributes: serde_json::Value,
}

#[Object]
impl VariantGQL {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn product_id(&self) -> i32 {
        self.product_id
    }

    async fn sku(&self) -> &str {
        &self.sku
    }

    async fn price_amount(&self) -> String {
        self.price_amount.to_string()
    }

    async fn price_currency(&self) -> &str {
        &self.price_currency
    }

    async fn attributes(&self) -> &serde_json::Value {
        &self.attributes
    }

    async fn stock_quantity(&self) -> i32 {
        self.stock_quantity
    }

    async fn is_active(&self) -> bool {
        self.is_active
    }
}

pub struct VariantLoader {
    pub pool: PgPool,
}

impl Loader<VariantLoadKey> for VariantLoader {
    type Value = Vec<VariantGQL>;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[VariantLoadKey],
    ) -> Result<HashMap<VariantLoadKey, Self::Value>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        // Use columns from the first key since they're the same for all keys in a single query
        let columns = &keys[0].columns;
        let product_ids: Vec<i32> = keys.iter().map(|k| k.product_id).collect();

        // Ensure required columns are included
        let mut safe_columns = vec!["id", "sku", "product_id"];
        for col in columns {
            safe_columns.push(col)
        }
        safe_columns.dedup();

        let mut sql = format!(
            "SELECT {} FROM product_variants WHERE product_id = ANY($1)",
            safe_columns.join(", ")
        );

        let sku = &keys[0].sku;

        match sku {
            Some(sku) => {
                sql = format!("{} AND sku = '{}'", sql, sku);
            }
            None => {}
        };

        println!("{}", sql);

        let rows = sqlx::query(&sql)
            .bind(&product_ids)
            .map(|row: PgRow| VariantGQL {
                id: row.get("id"),
                product_id: row.get("product_id"),
                sku: row.try_get("sku").unwrap_or_default(),
                price_amount: row.try_get("price_amount").unwrap_or_default(),
                price_currency: row.try_get("price_currency").unwrap_or_default(),
                attributes: row.try_get("attributes").unwrap_or_default(),
                is_active: row.try_get("is_active").unwrap_or_default(),
                stock_quantity: row.try_get("stock_quantity").unwrap_or_default(),
            })
            .fetch_all(&self.pool)
            .await?;

        // Create result map for each key
        let mut result_map: HashMap<VariantLoadKey, Vec<VariantGQL>> = HashMap::new();
        for key in keys {
            let variants: Vec<VariantGQL> = rows
                .iter()
                .filter(|v| v.product_id == key.product_id)
                .cloned()
                .collect();
            result_map.insert(key.clone(), variants);
        }

        Ok(result_map)
    }
}
