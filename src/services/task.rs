use rocket::futures::TryStreamExt;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgQueryResult;
use crate::models::Task;

pub async fn create(task: Task, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    let query = "insert into tasks (title, description, created_at) values ($1, $2, CURRENT_TIMESTAMP)";

    sqlx::query(query)
        .bind(&task.title)
        .bind(&task.description)
        .execute(pool)
        .await
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let q = "select id, title, description, created_at from tasks";
    let query = sqlx::query(q);

    let mut rows = query.fetch(pool);
    let mut tasks = vec![];

    while let Some(row) = rows.try_next().await.unwrap() {
        tasks.push(Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        })
    }

    Ok(tasks)
}
