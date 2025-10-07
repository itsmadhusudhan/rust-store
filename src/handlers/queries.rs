use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};
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

    async fn variants(&self, ctx: &Context<'_>, sku: Option<String>) -> Result<Vec<VariantGQL>> {
        let loader = ctx.data_unchecked::<DataLoader<VariantLoader>>();

        let mut cols = vec![];
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
            sku: sku,
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
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        // filter: Option<ProductFilter>,
    ) -> Result<Connection<String, ProductGQL, EmptyFields, EmptyFields>, String> {
        let db = ctx
            .data::<PgPool>()
            .expect("Db Connection is not available");
        let selection = ctx.look_ahead();

        let limit = first.unwrap_or(10);
        let offset = after
            .and_then(|cursor| cursor.parse::<i32>().ok())
            .unwrap_or(0);

        // Dynamically choose columns based on requested fields
        let mut cols = vec!["id".to_string()];
        let invalid_columns = ["created_by", "variants", "media", "node"];

        let node_lookahead = selection.field("edges").field("node");

        for field in node_lookahead.selection_fields() {
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

        // for field in selection.selection_fields() {
        //     println!("{:?}", field);
        //     let list = field.selection_set();
        //     for field in list {
        //         let field_name = field.name();
        //         println!("{:?}", field.arguments());

        //         if !invalid_columns.contains(&field_name) {
        //             let snake_name = to_snake_case(field_name);
        //             if !cols.contains(&snake_name) {
        //                 cols.push(snake_name);
        //             }
        //         }
        //     }
        // }

        let sql = format!(
            "SELECT {} FROM products LIMIT $1 OFFSET $2",
            cols.join(", ")
        );

        println!("{}", sql);

        let products = match sqlx::query(&sql)
            .bind(limit)
            .bind(offset)
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

        let mut conn = Connection::new(offset as usize > 0, products.len() > limit as usize);

        for (i, product) in products.into_iter().take(limit as usize).enumerate() {
            let cursor = ((offset as usize) + i).to_string(); // simple integer cursor
            conn.edges.push(Edge::new(cursor, product));
        }

        Ok(conn)
    }

    async fn product(
        &self,
        ctx: &Context<'_>,
        product_id: i32, // filter: Option<ProductFilter>,
    ) -> Result<Option<ProductGQL>, String> {
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

        let sql = format!(
            "SELECT {} FROM products WHERE id=$1 LIMIT $2",
            cols.join(", ")
        );

        let product = match sqlx::query(&sql)
            .bind(product_id)
            .bind(1)
            .map(|row: PgRow| ProductGQL {
                id: row.get("id"),
                name: row.try_get("name").ok(),
                slug: row.try_get("slug").ok(),
                description: row.try_get("description").ok(),
                status: row.try_get("status").ok(),
            })
            .fetch_optional(db)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        Ok(product)
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
