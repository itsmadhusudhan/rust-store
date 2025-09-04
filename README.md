# Rust Store

POC to create ecommerce apis using Rust and graphql.

- Dataloaders are used to optimize the queries and overcome the N+1 query problem.
- The project uses async-graphql and axum for the apis.


# Database Setup Instructions

## Prerequisites

1. Install PostgreSQL on your system
2. Create a database for the rust-store project

## Setup Commands

```bash
# 1. Copy environment file
cp .env.example .env

# 2. Edit .env with your database credentials
# DATABASE_URL=postgresql://username:password@localhost/rust_store

# 3. Create database (if it doesn't exist)
createdb rust_store

# 4. Run migrations
diesel migration run

# 5. Generate schema (if needed)
diesel print-schema > src/schema.rs
```

## Using Docker for Development

You can use the provided docker-compose file for the database:

```bash
# Start PostgreSQL container
docker-compose -f docker-compose-db.yaml up -d

# Use this DATABASE_URL in your .env
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_store
```

## Migration Commands

```bash
# Run all pending migrations
diesel migration run

# Rollback the last migration
diesel migration revert

# Check migration status
diesel migration pending

# Generate a new migration
diesel migration generate migration_name
```
