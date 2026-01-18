use std::env;

use bottle_orm::{Database, Model};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;

#[derive(Model)]
struct User {
    #[orm(primary_key)]
    id: i32,
    #[orm(size = 50, unique)]
    username: String,
    age: i32,
}

#[derive(Model, sqlx::FromRow)]
struct Account {
    #[orm(primary_key, size = 21)]
    id: i32,
    // #[orm(foreign_key = "User::id", unique, index, size = 21)]
    user_id: i32,
    r#type: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined.");
    let db = Database::connect(&url).await?;
    db.migrator().register::<User>().register::<Account>().run().await?;
    // let acc = Account { id: 1, user_id: 1, r#type: "credential".to_string() };
    // db.model::<Account>().insert(&acc).await?;
    let other_acc: Account = db.model::<Account>().filter("id", "=", 1).first().await?;
    println!("{}", other_acc.r#type);
    Ok(())
}
