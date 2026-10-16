use std::time::Duration;

use crate::{
    crypto::gen_secure_token,
    state::AppState,
    tui::{println_error, println_info},
};
use axum::{
    extract::{Query, State},
    http::{
        HeaderValue, StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub(crate) struct VerifyParams {
    pub token: String,
}

#[derive(FromRow)]
struct AuthToken {
    email: String,
}

pub(crate) async fn verify(
    State(state): State<AppState>,
    Query(params): Query<VerifyParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println_info("Verification requested with token");

    // Get the email associated with the token
    let record = sqlx::query_as::<_, AuthToken>(
        "SELECT email FROM auth_tokens WHERE token = ? AND expires_at > ?",
    )
    .bind(&params.token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        println_error(format!("DB Error: {}", e));
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::FORBIDDEN)?;

    // Consume the magic link token
    let result = sqlx::query("DELETE FROM auth_tokens WHERE token = ? AND expires_at > ?")
        .bind(&params.token)
        .bind(Utc::now().timestamp())
        .execute(&state.pool)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Prevent concurrent requests from consuming the same token
    if result.rows_affected() != 1 {
        return Err(StatusCode::FORBIDDEN);
    }

    // Generate a new session token and store it
    let session_token = gen_secure_token();
    let session_expires_at = Utc::now() + Duration::from_hours(24);
    sqlx::query("INSERT INTO sessions (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&session_token)
        .bind(&record.email)
        .bind(session_expires_at.timestamp())
        .execute(&state.pool)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Return the cookie with the new session token
    let cookie = format!(
        "session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
        session_token,
        24 * 60 * 60
    );
    let mut response = StatusCode::SEE_OTHER.into_response();

    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    response.headers_mut().insert(
        LOCATION,
        HeaderValue::from_str(&state.config.home_url)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}
