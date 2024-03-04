use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, Error, PgPool};
use std::env;

use crate::models::User;

use super::constants::{AUTH_URL, TOKEN_URL, USER_INFO_URL};

pub async fn login(redirect_url: &str) -> String {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not provided");

    let url = Url::parse_with_params(
        AUTH_URL,
        [
            ("client_id", client_id),
            ("redirect_uri", redirect_url.to_string()),
            ("response_type", "code".to_string()),
            ("scope", "email openid profile".to_string()),
            ("access_type", "offline".to_string()),
        ],
    )
    .unwrap();

    url.to_string()
}

pub async fn code(code: &String, redirect_uri: &str) -> GoogleTokenResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not provided");
    let client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not provided");

    let url = Url::parse_with_params(
        TOKEN_URL,
        [
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code.to_string()),
            ("grant_type", "authorization_code".to_string()),
            ("redirect_uri", redirect_uri.to_string()),
        ],
    )
    .unwrap();

    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .header("Content-Length", "0")
        .send()
        .await
        .unwrap();

    match res.status() {
        StatusCode::OK => {}
        _ => {
            panic!("Google oauth post request went wrong");
        }
    }
    
    match res.json::<GoogleTokenResponse>().await {
        Ok(parsed) => parsed,
        Err(_) => panic!("Google oauth could not create GoogleResponse"),
    }
}

pub async fn user_info(google_response: &GoogleTokenResponse) -> (GoogleResponse, String) {
    let client = reqwest::Client::new();

    let url = Url::parse_with_params(USER_INFO_URL, [("access_token", &google_response.access_token)]).unwrap();

    let res = client.get(url).send().await.unwrap();
    
    match res.status() {
        StatusCode::OK => {}
        _ => {
            panic!("Google oauth get request went wrong");
        }
    }
    
    match res.json::<GoogleResponse>().await {
        Ok(parsed) => (parsed, google_response.refresh_token.clone()),
        Err(e) => panic!("Google oauth could not create GoogleResponse -> {}", e),
    }
}

pub async fn create_user(user: &User, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let query = "insert into users (id, username, email, picture, joined_at, refresh_token) 
        values ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)";

    sqlx::query(query)
        .bind(&user.sub)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.picture)
        .bind(&user.refresh_token)
        .execute(pool)
        .await
}

pub async fn modify_user(user: &User, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let query = "
        UPDATE users
        SET username = $1,
            picture = $2,
            email = $3,
            refresh_token = $4
        WHERE id = $5
    ";

    sqlx::query(query)
        .bind(&user.name)
        .bind(&user.picture)
        .bind(&user.email)
        .bind(&user.refresh_token)
        .bind(&user.sub)
        .execute(pool)
        .await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleResponse {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}
