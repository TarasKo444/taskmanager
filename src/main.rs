mod models;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

use crate::routes::auth::*;
use crate::routes::tasks::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    let pool = services::pool::get_pool().await;
    sqlx::migrate!("src/migrations").run(&pool).await.unwrap();

    let _rocket = rocket::build()
        .mount("/api", routes![get_tasks, create_task, login, test])
        .launch()
        .await?;

    Ok(())
}
