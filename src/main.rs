mod models;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = services::pool::get_pool().await;
    sqlx::migrate!("src/migrations").run(&pool).await.unwrap();

    use crate::routes::tasks::*;
    let _rocket = rocket::build()
        .mount("/api", routes![
            get_tasks,
            create_task
        ])
        .launch()
        .await?;

    Ok(())
}
