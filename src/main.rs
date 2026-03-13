use axum::routing::get;
use dotenv;
use std::env;
use std::error::Error;

use axum::{Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};

struct AppState {
    pool: PgPool,
}

#[derive(Serialize, FromRow)]
struct Project {
    id: i32,
    title: String,
    description: String,
    is_completed: bool,
    is_created: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    let app = Router::new().route("/", get(|| async {"Hello Deepesh"}));
        // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
