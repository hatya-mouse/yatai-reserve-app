use crate::{AppState, error::AppError, println_info};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
};

pub(crate) async fn signout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, AppError> {
    println_info("Signout requested");

    let session_token = headers
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or(AppError::Unauthorized)?;

    // Delete the session from the database
    sqlx::query("DELETE FROM sessions WHERE token = ?")
        .bind(session_token)
        .execute(&state.pool)
        .await?;

    Ok(StatusCode::OK)
}
