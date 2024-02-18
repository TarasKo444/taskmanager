use chrono::{DateTime};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::Task;
use crate::*;

#[derive(Serialize)]
pub struct Response {
    id: String,
    title: String,
    description: String,
    created_at: String,
}

#[derive(Deserialize)]
pub struct Request {
    title: String,
    description: String,
}

#[get("/tasks")]
pub async fn get_tasks() -> Json<Vec<Response>> {
    let pool = services::pool::get_pool().await;
    let tasks = services::task::get_all(&pool).await.unwrap();
    
    Json(tasks.iter().map(|t| {
        Response {
            id: t.id.to_string(),
            title: t.title.to_string(),
            description: t.description.to_string(),
            created_at: t.created_at.format("%d/%m/%Y %H:%M").to_string()
        }
    }).collect::<Vec<Response>>())
}

#[post("/tasks", data = "<task>")]
pub async fn create_task(task: Json<Request>) -> Status {
    let new_task = Task {
        id: Uuid::nil(),
        created_at: DateTime::default(),
        title: task.title.to_string(),
        description: task.description.to_string()
    };

    let pool = services::pool::get_pool().await;
    let response = services::task::create(new_task, &pool).await;
    
    match response {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}
