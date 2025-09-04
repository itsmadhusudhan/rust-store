// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        parent_id -> Nullable<Int4>,
        sort_order -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_attributes (id) {
        id -> Int4,
        product_id -> Int4,
        namespace -> Varchar,
        attribute_key -> Varchar,
        attribute_value -> Text,
        value_type -> Nullable<Varchar>,
        is_searchable -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_category_junction (id) {
        id -> Int4,
        product_id -> Int4,
        category_id -> Int4,
        is_primary -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    product_media (id) {
        id -> Int4,
        product_id -> Int4,
        url -> Varchar,
        media_type -> Varchar,
        sort_order -> Nullable<Int4>,
        alt_text -> Nullable<Varchar>,
        file_size -> Nullable<Int4>,
        mime_type -> Nullable<Varchar>,
        is_primary -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    product_variants (id) {
        id -> Int4,
        product_id -> Int4,
        sku -> Varchar,
        price_amount -> Numeric,
        #[max_length = 3]
        price_currency -> Varchar,
        stock_quantity -> Int4,
        attributes -> Nullable<Jsonb>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(product_attributes -> products (product_id));
diesel::joinable!(product_category_junction -> categories (category_id));
diesel::joinable!(product_category_junction -> products (product_id));
diesel::joinable!(product_media -> products (product_id));
diesel::joinable!(product_variants -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    product_attributes,
    product_category_junction,
    product_media,
    product_variants,
    products,
);
