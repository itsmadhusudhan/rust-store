use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Database model for products table
#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct DbProduct {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Insert struct for products
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct NewDbProduct {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
}

/// Database model for categories table
#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = categories)]
pub struct DbCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Insert struct for categories
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewDbCategory {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
}

/// Database model for product_variants table
#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone,
)]
#[diesel(belongs_to(DbProduct, foreign_key = product_id))]
#[diesel(table_name = product_variants)]
pub struct DbProductVariant {
    pub id: i32,
    pub product_id: i32,
    pub sku: String,
    pub price_amount: bigdecimal::BigDecimal,
    pub price_currency: String,
    pub stock_quantity: i32,
    pub attributes: Option<JsonValue>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Insert struct for product_variants
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = product_variants)]
pub struct NewDbProductVariant {
    pub product_id: i32,
    pub sku: String,
    pub price_amount: bigdecimal::BigDecimal,
    pub price_currency: String,
    pub stock_quantity: i32,
    pub attributes: Option<JsonValue>,
    pub is_active: bool,
}

/// Database model for product_media table
#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone,
)]
#[diesel(belongs_to(DbProduct, foreign_key = product_id))]
#[diesel(table_name = product_media)]
pub struct DbProductMedia {
    pub id: i32,
    pub product_id: i32,
    pub url: String,
    pub media_type: String,
    pub sort_order: Option<i32>,
    pub alt_text: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub is_primary: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Insert struct for product_media
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = product_media)]
pub struct NewDbProductMedia {
    pub product_id: i32,
    pub url: String,
    pub media_type: String,
    pub sort_order: Option<i32>,
    pub alt_text: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub is_primary: bool,
}

/// Database model for product_attributes table
#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone,
)]
#[diesel(belongs_to(DbProduct, foreign_key = product_id))]
#[diesel(table_name = product_attributes)]
pub struct DbProductAttribute {
    pub id: i32,
    pub product_id: i32,
    pub namespace: String,
    pub attribute_key: String,
    pub attribute_value: String,
    pub value_type: Option<String>,
    pub is_searchable: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Insert struct for product_attributes
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = product_attributes)]
pub struct NewDbProductAttribute {
    pub product_id: i32,
    pub namespace: String,
    pub attribute_key: String,
    pub attribute_value: String,
    pub value_type: Option<String>,
    pub is_searchable: bool,
}

/// Database model for product_category_junction table
#[derive(
    Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone,
)]
#[diesel(belongs_to(DbProduct, foreign_key = product_id))]
#[diesel(belongs_to(DbCategory, foreign_key = category_id))]
#[diesel(table_name = product_category_junction)]
pub struct DbProductCategoryJunction {
    pub id: i32,
    pub product_id: i32,
    pub category_id: i32,
    pub is_primary: bool,
    pub created_at: NaiveDateTime,
}

/// Insert struct for product_category_junction
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = product_category_junction)]
pub struct NewDbProductCategoryJunction {
    pub product_id: i32,
    pub category_id: i32,
    pub is_primary: bool,
}
