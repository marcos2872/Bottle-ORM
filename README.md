# Bottle ORM

[![Crates.io](https://img.shields.io/crates/v/bottle-orm.svg)](https://crates.io/crates/bottle-orm)
[![Docs.rs](https://docs.rs/bottle-orm/badge.svg)](https://docs.rs/bottle-orm)
[![License](https://img.shields.io/crates/l/bottle-orm.svg)](https://github.com/Murilinho145SG/bottle-orm/blob/main/LICENSE)

**Bottle ORM** is a lightweight, async ORM for Rust built on top of [sqlx](https://github.com/launchbadge/sqlx). It is designed to be simple, efficient, and easy to use, providing a fluent Query Builder and automatic schema migrations.

## Features

- **Async & Non-blocking**: Built on `tokio` and `sqlx`.
- **Multi-Driver Support**: Compatible with PostgreSQL, MySQL, and SQLite (via `sqlx::Any`).
- **Macro-based Models**: Define your schema using standard Rust structs with `#[derive(Model)]`.
- **Fluent Query Builder**: Chainable methods for filtering, selecting, pagination, and sorting.
- **Auto-Migration**: Automatically creates tables and foreign key constraints based on your structs.

## Project Structure

This repository is a workspace containing:

- **[`bottle-orm`](./bottle-orm)**: The main crate.
- **[`bottle-orm-macro`](./bottle-orm-macro)**: Procedural macros for the ORM.

## Installation

Add `bottle-orm` to your `Cargo.toml`. You will also need `sqlx`, `tokio`, and `serde`.

```toml
[dependencies]
bottle-orm = "0.1.0"
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-native-tls", "any", "postgres", "sqlite", "mysql", "chrono"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
```

## Quick Start

### 1. Define your Models

Use the `#[derive(Model)]` macro to define your database tables.

```rust
use bottle_orm::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Model, Debug, Clone, Serialize, Deserialize, FromRow)]
struct User {
    #[orm(primary_key)]
    id: i32,
    #[orm(size = 50, unique)]
    username: String,
    age: i32,
    #[orm(create_time)]
    created_at: DateTime<Utc>,
}

#[derive(Model, Debug, Clone, Serialize, Deserialize, FromRow)]
struct Post {
    #[orm(primary_key)]
    id: i32,
    #[orm(foreign_key = "User::id")]
    user_id: i32,
    title: String,
    content: String,
}
```

### 2. Connect and Migrate

Initialize the database connection and run migrations to create tables automatically.

```rust
use bottle_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite::memory:"; // Or your DB URL

    // 1. Connect to the database
    let db = Database::connect(&database_url).await?;

    // 2. Run Migrations
    db.migrator()
        .register::<User>()
        .register::<Post>()
        .run()
        .await?;
    
    Ok(())
}
```

### 3. Query Data

Use the fluent query builder to filter, sort and retrieve data.

```rust
// Fetch multiple records with conditions, order, and pagination
let adults: Vec<User> = db.model::<User>()
    .filter("age", ">=", 18)
    .order("age DESC")
    .limit(10)
    .scan()
    .await?;
```

## Supported Attributes (`#[orm(...)]`)

- `primary_key`: Marks the column as the Primary Key.
- `unique`: Adds a UNIQUE constraint.
- `index`: Creates an index for this column.
- `create_time`: Sets default value to current timestamp on creation.
- `foreign_key = "Table::Column"`: Creates a Foreign Key relationship.
- `size = N`: Sets the column size (e.g., `VARCHAR(N)`).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
