use rocket::serde::json::serde_json::json;
use rocket::{http::Status, response::Redirect};
use sqlx::types::JsonValue;

use crate::{models::User, services::{
    auth::{self, modify_user},
    constants::DATABASE_ROW_EXISTS,
    pool,
}};

#[get("/auth/login")]
pub async fn login() -> Redirect {
    let url = auth::login("http://localhost:8000/api/auth/test").await;

    Redirect::to(url)
}

#[get("/auth/test?<code>")]
pub async fn test(code: String) -> Result<JsonValue, Status> {
    let google_response = auth::code(&code, "http://localhost:8000/api/auth/test").await;    
    
    let (user_info, refresh_token) = auth::user_info(&google_response).await;
    
    let user = User {
        sub: user_info.sub.clone(),
        name: user_info.name.clone(),
        picture: user_info.picture.clone(),
        email: user_info.email.clone(),
        refresh_token: refresh_token.clone()
    };
    
    let pool = pool::get_pool().await;
    let response = auth::create_user(&user, &pool).await;
    
    if let Err(e) = response {
        let err = e.as_database_error();

        if let Some(de) = err {
            let code = de.code();

            match code {
                Some(code) => {
                    if code == DATABASE_ROW_EXISTS && modify_user(&user, &pool).await.is_err() {
                        info!("User updated");
                        return Err(Status::InternalServerError);
                    }
                }
                None => return Err(Status::InternalServerError),
            }
        }
    }
    
    info!("User created");
    Ok(json!(user))
}
