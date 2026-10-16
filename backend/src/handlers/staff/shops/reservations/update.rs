use crate::{AppState, error::AppError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateReservationRequest {
    pub status: String,
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Path((shop_id, reservation_id)): Path<(String, i64)>,
    Json(payload): Json<UpdateReservationRequest>,
) -> Result<StatusCode, AppError> {
    if !matches!(
        payload.status.as_str(),
        "waiting" | "called" | "cancelled" | "completed"
    ) {
        return Err(AppError::BadRequest);
    }

    let result = sqlx::query(
        r#"
        UPDATE reservations
        SET status = ?
        WHERE id = ?
          AND shop_id = ?
        "#,
    )
    .bind(&payload.status)
    .bind(reservation_id)
    .bind(&shop_id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
