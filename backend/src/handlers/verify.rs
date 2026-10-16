use crate::{
    state::AppState,
    tui::{println_error, println_info},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
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
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::FORBIDDEN)?;

    // Remove the used token
    sqlx::query("DELETE FROM auth_tokens WHERE token = ?")
        .bind(&params.token)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Login
    let email = record.email;

    Ok(StatusCode::OK)
}
