-- Create products table
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    slug VARCHAR UNIQUE NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for slug lookup
CREATE INDEX idx_products_slug ON products(slug);

-- Create index for status filtering
CREATE INDEX idx_products_status ON products(status);

-- Set up automatic updated_at trigger
SELECT diesel_manage_updated_at('products');
