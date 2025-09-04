-- Create product_category_junction table for many-to-many relationships
CREATE TABLE product_category_junction (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, category_id)
);

-- Create indexes for efficient queries
CREATE INDEX idx_product_category_product_id ON product_category_junction(product_id);
CREATE INDEX idx_product_category_category_id ON product_category_junction(category_id);
CREATE INDEX idx_product_category_primary ON product_category_junction(is_primary);
