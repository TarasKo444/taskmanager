use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Pool<Postgres> {
    let url = "postgres://postgres:root@localhost:5432/taskmanagerstore";
    PgPoolOptions::new().connect(url).await.unwrap()
}