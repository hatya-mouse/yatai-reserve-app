use crate::{
    AppState,
    error::AppError,
    handlers::{email::send_status_email, user::renew_status_token},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReserveRequest {
    email: String,
}

pub(crate) async fn reserve(
    State(state): State<AppState>,
    Path(shop_id): Path<String>,
    Json(body): Json<ReserveRequest>,
) -> Result<StatusCode, AppError> {
    let email = body.email.trim().to_ascii_lowercase();

    if email.is_empty() || !email.contains('@') {
        return Err(AppError::BadRequest);
    }

    // Make sure that that shop exists
    let shop_exists =
        sqlx::query_scalar::<_, i64>("SELECT EXISTS(SELECT 1 FROM shops WHERE id = ?)")
            .bind(&shop_id)
            .fetch_one(&state.pool)
            .await?;

    if shop_exists == 0 {
        return Err(AppError::NotFound);
    }

    // Check if the waiting reservation with the same email address exists
    let reservation_exists = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM reservations
            WHERE shop_id = ?
                AND email = ?
                AND status = 'waiting'
        )
        "#,
    )
    .bind(&shop_id)
    .bind(&email)
    .fetch_one(&state.pool)
    .await?;

    // If so, return an error
    if reservation_exists != 0 {
        return Err(AppError::Conflict);
    }

    let now = Utc::now().timestamp();

    // Create a reservation
    sqlx::query(
        r#"
        INSERT INTO reservations (shop_id, email, status, created_at)
        VALUES (?, ?, 'waiting', ?)
        "#,
    )
    .bind(&shop_id)
    .bind(&email)
    .bind(now)
    .execute(&state.pool)
    .await?;

    // Update the status token for the email address
    let token = renew_status_token(&state, &email).await?;
    send_status_email(&state, &email, &token).await?;

    Ok(StatusCode::CREATED)
}
