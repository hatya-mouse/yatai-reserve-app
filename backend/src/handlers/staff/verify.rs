use crate::{error::AppError, state::AppState, tui::println_info, utils::gen_secure_token};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::time::Duration;

#[derive(Deserialize)]
pub(crate) struct VerifyParams {
    pub token: String,
}

#[derive(FromRow)]
struct AuthToken {
    email: String,
}

#[derive(Serialize)]
pub(crate) struct SessionData {
    token: String,
}

pub(crate) async fn verify(
    State(state): State<AppState>,
    Query(params): Query<VerifyParams>,
) -> Result<Json<SessionData>, AppError> {
    println_info("Verification requested with token");

    // Get the email associated with the token
    let record = sqlx::query_as::<_, AuthToken>(
        "SELECT email FROM auth_tokens WHERE token = ? AND expires_at > ?",
    )
    .bind(&params.token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Consume the magic link token
    let result = sqlx::query("DELETE FROM auth_tokens WHERE token = ? AND expires_at > ?")
        .bind(&params.token)
        .bind(Utc::now().timestamp())
        .execute(&state.pool)
        .await?;

    // Prevent concurrent requests from consuming the same token
    if result.rows_affected() != 1 {
        return Err(AppError::Unauthorized);
    }

    // Generate a new session token and store it
    let session_token = gen_secure_token();
    let session_expires_at = Utc::now() + Duration::from_hours(24);
    sqlx::query("INSERT INTO sessions (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&session_token)
        .bind(&record.email)
        .bind(session_expires_at.timestamp())
        .execute(&state.pool)
        .await?;

    // If the email is not registered to staffs, generate a new name and add it
    sqlx::query("INSERT INTO IF NOT EXISTS staffs (name, email) VALUES (?, ?)")
        .bind("ユーザ")
        .bind(&record.email)
        .execute(&state.pool)
        .await?;

    Ok(Json(SessionData {
        token: session_token,
    }))
}
