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

Add `bottle-orm` to your `Cargo.toml`. You will also need `sqlx`, `tokio`, `serde`, and optionally `uuid` for UUID support.

```toml
[dependencies]
bottle-orm = "0.1.0"
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-native-tls", "any", "postgres", "sqlite", "mysql", "chrono"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.11", features = ["v4", "v7", "serde"] }  # Optional: for UUID support
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
- `update_time`: Auto-updates timestamp on modification (future feature).
- `foreign_key = "Table::Column"`: Creates a Foreign Key relationship.
- `size = N`: Sets the column size (e.g., `VARCHAR(N)`).

## UUID Support (Versions 1-7)

Bottle ORM has full support for UUID types across all versions (1 through 7). UUIDs are ideal for distributed systems and provide better security than sequential IDs.

### UUID Version Overview

- **Version 1**: Time-based with MAC address
- **Version 3**: Name-based using MD5 hash
- **Version 4**: Random (most common)
- **Version 5**: Name-based using SHA-1 hash
- **Version 6**: Reordered time-based (better for database indexing)
- **Version 7**: Unix timestamp-based (sortable, recommended for new projects)

### Example with Different UUID Versions

```rust
use bottle_orm::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Model, Debug, Clone, Serialize, Deserialize, FromRow)]
struct User {
    #[orm(primary_key)]
    id: Uuid,  // Can use any UUID version
    #[orm(size = 50, unique)]
    username: String,
    #[orm(create_time)]
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://localhost/mydb").await?;
    
    db.migrator()
        .register::<User>()
        .run()
        .await?;

    // UUID v4 - Random (most common)
    let user_v4 = User {
        id: Uuid::new_v4(),
        username: "user_v4".to_string(),
        created_at: Utc::now(),
    };
    db.model::<User>().insert(&user_v4).await?;

    // UUID v7 - Timestamp-based (recommended for new projects)
    let user_v7 = User {
        id: Uuid::now_v7(),
        username: "user_v7".to_string(),
        created_at: Utc::now(),
    };
    db.model::<User>().insert(&user_v7).await?;

    // Query by UUID
    let found_user: User = db.model::<User>()
        .filter("id", "=", user_v4.id)
        .first()
        .await?;

    println!("Found user: {:?}", found_user);

    Ok(())
}
```

### UUID Foreign Keys

```rust
#[derive(Model, Debug, Clone, Serialize, Deserialize, FromRow)]
struct Post {
    #[orm(primary_key)]
    id: Uuid,
    #[orm(foreign_key = "User::id")]
    user_id: Uuid,  // Foreign key using UUID
    title: String,
    content: String,
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
