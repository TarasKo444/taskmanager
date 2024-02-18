use chrono::Local;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Response<'r> {
    title: &'r str,
    description: &'r str,
    created_at: String
}

#[get("/")]
pub async fn hello() -> Json<Response<'static>> {
    Json(Response {
        title: "Test t",
        description: "Test d",
        created_at: Local::now().to_utc().to_string()
    })
}
