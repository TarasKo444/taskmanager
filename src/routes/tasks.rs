use crate::models::Task;
use crate::*;
use chrono::DateTime;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use serde::Deserialize;
use sqlx::types::JsonValue;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Request {
    title: String,
    description: String,
}

#[get("/tasks")]
pub async fn get_tasks() -> JsonValue {
    let pool = services::pool::get_pool().await;
    let tasks = services::task::get_all(&pool).await.unwrap();

    json!(tasks)
}

#[post("/tasks", data = "<task>")]
pub async fn create_task(task: Json<Request>) -> Status {
    let new_task = Task {
        id: Uuid::nil(),
        created_at: DateTime::default(),
        title: task.title.to_string(),
        description: task.description.to_string(),
    };

    let pool = services::pool::get_pool().await;
    let response = services::task::create(new_task, &pool).await;

    match response {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
