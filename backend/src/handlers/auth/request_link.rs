use crate::{
    crypto::gen_secure_token,
    tui::{println_error, println_info},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Duration;

#[derive(Deserialize)]
pub(crate) struct RequestLinkBody {
    pub email: String,
}

pub(crate) async fn request_link(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RequestLinkBody>,
) -> Result<impl IntoResponse, StatusCode> {
    println_info(format!("Login link requested for email {}", payload.email));

    let token = gen_secure_token();
    let expires_at = Utc::now() + Duration::from_mins(10);

    // Remove the existing query from the email address
    sqlx::query("DELETE FROM auth_tokens WHERE email = ?")
        .bind(&payload.email)
        .execute(&pool)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Insert the newly generated token and its expiration date
    sqlx::query("INSERT INTO auth_tokens (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&token)
        .bind(&payload.email)
        .bind(expires_at.timestamp())
        .execute(&pool)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}
