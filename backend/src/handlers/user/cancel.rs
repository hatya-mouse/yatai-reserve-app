use crate::{AppState, error::AppError, handlers::user::StatusQuery};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;

pub(crate) async fn cancel(
    State(state): State<AppState>,
    Path(reservation_id): Path<String>,
    Query(query): Query<StatusQuery>,
) -> Result<StatusCode, AppError> {
    let now = Utc::now().timestamp();

    // Verify the token and get the mail address associated with it
    // Do not remove the status token so that it can be reused within the expiration date
    let email = sqlx::query_scalar::<_, String>(
        r#"
        SELECT email
        FROM status_tokens
        WHERE token = ?
            AND expires_at > ?;
        "#,
    )
    .bind(&query.token)
    .bind(now)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    // Cancel the reservation
    sqlx::query(
        r#"
        UPDATE reservations
        SET status = 'cancelled'
        WHERE id = ?
            AND email = ?
            AND status = 'waiting';
        "#,
    )
    .bind(&reservation_id)
    .bind(&email)
    .execute(&state.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
