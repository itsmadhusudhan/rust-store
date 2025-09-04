use async_graphql::{Context, InputObject, Object, Result, dataloader::DataLoader};
use sqlx::{PgPool, Row, postgres::PgRow, prelude::FromRow};

use crate::handlers::{
    media_loader::{Media, ProductMediaLoadKey, ProductMediaLoader},
    variant_loader::{VariantGQL, VariantLoadKey, VariantLoader},
};

#[derive(InputObject)]
pub struct ProductFilter {
    pub category_slug: Option<String>,
    pub in_stock: Option<bool>,
}

#[derive(Debug, FromRow)]
pub struct ProductGQL {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[Object]
impl ProductGQL {
    async fn id(&self) -> &i32 {
        &self.id
    }

    async fn name(&self) -> Option<String> {
        self.name.clone()
    }

    async fn slug(&self) -> Option<String> {
        self.slug.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn status(&self) -> Option<String> {
        self.status.clone()
    }

    async fn variants(&self, ctx: &Context<'_>) -> Result<Vec<VariantGQL>> {
        let loader = ctx.data_unchecked::<DataLoader<VariantLoader>>();

        let mut cols = vec!["sku".to_string()];
        let selection = ctx.look_ahead();

        for field in selection.selection_fields() {
            let list = field.selection_set();
            for field in list {
                let field_name = field.name();
                let snake_name = to_snake_case(field_name);
                if !cols.contains(&snake_name) {
                    cols.push(snake_name);
                }
            }
        }

        let key = VariantLoadKey {
            product_id: self.id,
            columns: cols,
        };

        Ok(loader.load_one(key).await?.unwrap_or_default())
    }

    async fn media(&self, ctx: &Context<'_>) -> Result<Vec<Media>> {
        let loader = ctx.data_unchecked::<DataLoader<ProductMediaLoader>>();

        let mut cols = vec!["id".to_string()];
        let selection = ctx.look_ahead();

        for field in selection.selection_fields() {
            let list = field.selection_set();
            for field in list {
                let field_name = field.name();
                let snake_name = to_snake_case(field_name);
                if !cols.contains(&snake_name) {
                    cols.push(snake_name);
                }
            }
        }

        let key = ProductMediaLoadKey {
            product_id: self.id,
            columns: cols,
        };

        Ok(loader.load_one(key).await?.unwrap_or_default())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn products(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        // filter: Option<ProductFilter>,
    ) -> Result<Option<Vec<ProductGQL>>, String> {
        let db = ctx
            .data::<PgPool>()
            .expect("Db Connection is not available");
        let selection = ctx.look_ahead();

        // Dynamically choose columns based on requested fields
        let mut cols = vec!["id".to_string()];
        let invalid_columns = ["created_by", "variants", "media"];

        for field in selection.selection_fields() {
            let list = field.selection_set();
            for field in list {
                let field_name = field.name();
                if !invalid_columns.contains(&field_name) {
                    let snake_name = to_snake_case(field_name);
                    if !cols.contains(&snake_name) {
                        cols.push(snake_name);
                    }
                }
            }
        }

        let sql = format!("SELECT {} FROM products LIMIT $1", cols.join(", "));

        let products = match sqlx::query(&sql)
            .bind(limit.unwrap_or(20))
            .map(|row: PgRow| ProductGQL {
                id: row.get("id"),
                name: row.try_get("name").ok(),
                slug: row.try_get("slug").ok(),
                description: row.try_get("description").ok(),
                status: row.try_get("status").ok(),
            })
            .fetch_all(db)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Some(products))
    }
}

// Converts camelCase or PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            for lower in c.to_lowercase() {
                snake.push(lower);
            }
        } else {
            snake.push(c);
        }
    }

    snake
}
