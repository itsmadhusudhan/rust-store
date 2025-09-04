use crate::data::DataStore;
use crate::domain::*;
use async_graphql::{Context, ID, Object, Result};
pub mod media_loader;
pub mod products;
pub mod queries;
pub mod variant_loader;

/// GraphQL Query resolver
pub struct Query;

#[Object]
impl Query {
    /// Get all products
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let store = ctx.data::<DataStore>()?;
        Ok(store.get_products())
    }

    /// Get a specific product by ID
    async fn product(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Product>> {
        let store = ctx.data::<DataStore>()?;
        Ok(store.get_product(&id.to_string()))
    }

    /// Get a product by its slug
    async fn product_by_slug(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Product>> {
        let store = ctx.data::<DataStore>()?;
        Ok(store.get_product_by_slug(&slug))
    }

    /// Get all categories
    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<Category>> {
        let store = ctx.data::<DataStore>()?;
        Ok(store.get_categories())
    }

    /// Get a specific category by ID
    async fn category(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Category>> {
        let store = ctx.data::<DataStore>()?;
        Ok(store.get_category(&id.to_string()))
    }

    /// Search products by name or description
    async fn search_products(
        &self,
        ctx: &Context<'_>,
        query: String,
        limit: Option<i32>,
    ) -> Result<Vec<Product>> {
        let store = ctx.data::<DataStore>()?;
        let limit = limit.unwrap_or(10) as usize;
        Ok(store.search_products(&query, limit))
    }

    /// Get products by category
    async fn products_by_category(
        &self,
        ctx: &Context<'_>,
        category_id: ID,
    ) -> Result<Vec<Product>> {
        let store = ctx.data::<DataStore>()?;
        let products = store
            .get_products()
            .into_iter()
            .filter(|p| p.categories.iter().any(|c| c.id == category_id))
            .collect();
        Ok(products)
    }

    /// Get products by status
    async fn products_by_status(&self, ctx: &Context<'_>, status: String) -> Result<Vec<Product>> {
        let store = ctx.data::<DataStore>()?;
        let products = store
            .get_products()
            .into_iter()
            .filter(|p| p.status == status)
            .collect();
        Ok(products)
    }
}

/// GraphQL Mutation resolver
pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a new product
    async fn create_product(
        &self,
        _ctx: &Context<'_>,
        input: CreateProductInput,
    ) -> Result<Product> {
        // In a real implementation, this would save to the database
        let new_id = format!("{}", chrono::Utc::now().timestamp());

        let product = Product {
            id: ID(new_id),
            name: input.name,
            slug: input.slug,
            description: input.description,
            status: input.status,
            variants: vec![],
            categories: vec![],
            // media: vec![],
        };

        Ok(product)
    }

    /// Update an existing product
    async fn update_product(
        &self,
        ctx: &Context<'_>,
        input: UpdateProductInput,
    ) -> Result<Option<Product>> {
        let store = ctx.data::<DataStore>()?;

        if let Some(mut product) = store.get_product(&input.id.to_string()) {
            // Apply updates
            if let Some(name) = input.name {
                product.name = name;
            }
            if let Some(slug) = input.slug {
                product.slug = slug;
            }
            if let Some(description) = input.description {
                product.description = Some(description);
            }
            if let Some(status) = input.status {
                product.status = status;
            }

            // In a real implementation, save the updated product to database
            Ok(Some(product))
        } else {
            Ok(None)
        }
    }

    /// Create a new category
    async fn create_category(
        &self,
        ctx: &Context<'_>,
        input: CreateCategoryInput,
    ) -> Result<Category> {
        let store = ctx.data::<DataStore>()?;
        let new_id = format!("{}", chrono::Utc::now().timestamp());

        let parent = if let Some(parent_id) = input.parent_id {
            store
                .get_category(&parent_id.to_string())
                .map(|p| Box::new(p))
        } else {
            None
        };

        let category = Category {
            id: ID(new_id),
            name: input.name,
            slug: input.slug,
            children: vec![],
            parent,
        };

        Ok(category)
    }

    /// Create a product variant
    async fn create_product_variant(
        &self,
        _ctx: &Context<'_>,
        input: CreateProductVariantInput,
    ) -> Result<ProductVariant> {
        let new_id = format!("{}", chrono::Utc::now().timestamp());

        let variant = ProductVariant {
            id: ID(new_id),
            sku: input.sku,
            price: Money {
                amount: input.price.amount,
                currency: input.price.currency,
            },
            stock_quantity: input.stock_quantity,
            attributes: input.attributes,
        };

        Ok(variant)
    }

    /// Delete a product
    async fn delete_product(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let store = ctx.data::<DataStore>()?;

        // Check if product exists
        if store.get_product(&id.to_string()).is_some() {
            // In a real implementation, delete from database
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Update product variant stock
    async fn update_variant_stock(
        &self,
        ctx: &Context<'_>,
        variant_id: ID,
        stock_quantity: i32,
    ) -> Result<Option<ProductVariant>> {
        let store = ctx.data::<DataStore>()?;

        // Find product containing the variant
        for product in store.get_products() {
            if let Some(mut variant) = product.variants.into_iter().find(|v| v.id == variant_id) {
                variant.stock_quantity = stock_quantity;
                // In a real implementation, save to database
                return Ok(Some(variant));
            }
        }

        Ok(None)
    }
}
