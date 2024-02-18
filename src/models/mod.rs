
use chrono::{Utc};
use sqlx::Decode;
use uuid::Uuid;

use sqlx::types::chrono::DateTime;

#[derive(Decode)]
pub struct Task<> {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
