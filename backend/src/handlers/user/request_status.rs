use crate::{
    AppState,
    error::AppError,
    handlers::user::{renew_status_token, send_status_email},
};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RequestStatusRequest {
    email: String,
}

pub(crate) async fn request_status(
    State(state): State<AppState>,
    Json(body): Json<RequestStatusRequest>,
) -> Result<StatusCode, AppError> {
    let email = body.email.trim().to_ascii_lowercase();

    if email.is_empty() || !email.contains('@') {
        return Err(AppError::BadRequest);
    }

    let has_reservations =
        sqlx::query_scalar::<_, i64>("SELECT EXISTS(SELECT 1 FROM reservations WHERE email = ?)")
            .bind(&email)
            .fetch_one(&state.pool)
            .await?;

    if has_reservations != 0 {
        let token = renew_status_token(&state, &email).await?;
        send_status_email(&state, &email, &token).await?;
    }

    Ok(StatusCode::ACCEPTED)
}
