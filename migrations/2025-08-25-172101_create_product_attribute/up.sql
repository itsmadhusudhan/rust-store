-- Create product_attributes table for flexible metadata
CREATE TABLE product_attributes (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    namespace VARCHAR NOT NULL,
    attribute_key VARCHAR NOT NULL,
    attribute_value TEXT NOT NULL,
    value_type VARCHAR DEFAULT 'string',
    is_searchable BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, namespace, attribute_key)
);

-- Create indexes for efficient queries
CREATE INDEX idx_product_attributes_product_id ON product_attributes(product_id);
CREATE INDEX idx_product_attributes_namespace ON product_attributes(namespace);
CREATE INDEX idx_product_attributes_key ON product_attributes(attribute_key);
CREATE INDEX idx_product_attributes_searchable ON product_attributes(is_searchable);
CREATE INDEX idx_product_attributes_namespace_key ON product_attributes(namespace, attribute_key);

-- Set up automatic updated_at trigger
SELECT diesel_manage_updated_at('product_attributes');
