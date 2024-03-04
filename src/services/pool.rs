use std::env;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Pool<Postgres> {
    let url = env::var("DB_CONNECTION").expect("DB_CONNECTION not provided");
    PgPoolOptions::new().connect(url.as_str()).await.unwrap()
}