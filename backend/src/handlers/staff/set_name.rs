use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{Extension, Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize)]
pub(crate) struct SetNameRequest {
    name: String,
    email: String,
}

pub(crate) async fn set_name(
    Extension(user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(body): Json<SetNameRequest>,
) -> Result<StatusCode, AppError> {
    let me = sqlx::query_as(
        r#"
        UPDATE staffs
        SET
            name = COALESCE(?, name),
        WHERE email = ?
        "#,
    )
    .bind(body.name.trim())
    .bind(&body.email)
    .fetch_one(&state.pool)
    .await?;

    Ok(StatusCode::OK)
}
