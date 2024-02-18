use chrono::Local;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct Response<'r> {
    id: String,
    title: &'r str,
    description: &'r str,
    created_at: String,
}

#[get("/tasks")]
pub async fn get_tasks() -> Json<Response<'static>> {
    Json(Response {
        id: Uuid::new_v4().to_string(),
        title: "Test t",
        description: "Test d",
        created_at: Local::now().to_utc().to_string(),
    })
}

#[post("/tasks")]
pub async fn create_task() -> Json<Response<'static>> {
    
}
