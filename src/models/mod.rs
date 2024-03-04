
use chrono::Utc;
use serde::Serialize;
use sqlx::Decode;
use uuid::Uuid;

use sqlx::types::chrono::DateTime;

#[derive(Decode, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}


#[derive(Decode, Serialize)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub refresh_token: String,
}
