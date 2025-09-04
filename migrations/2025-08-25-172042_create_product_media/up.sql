-- Create product_media table
CREATE TABLE product_media (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    url VARCHAR NOT NULL,
    media_type VARCHAR NOT NULL,
    sort_order INTEGER DEFAULT 0,
    alt_text VARCHAR,
    file_size INTEGER,
    mime_type VARCHAR,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for efficient queries
CREATE INDEX idx_product_media_product_id ON product_media(product_id);
CREATE INDEX idx_product_media_sort_order ON product_media(sort_order);
CREATE INDEX idx_product_media_type ON product_media(media_type);
CREATE INDEX idx_product_media_primary ON product_media(is_primary);

-- Set up automatic updated_at trigger
SELECT diesel_manage_updated_at('product_media');
