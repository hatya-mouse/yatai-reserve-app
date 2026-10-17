use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{Extension, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct SetNameRequest {
    name: String,
}

pub(crate) async fn set_name(
    Extension(user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(body): Json<SetNameRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query(
        r#"
        UPDATE staffs
        SET name = ?
        WHERE email = ?
        "#,
    )
    .bind(body.name.trim())
    .bind(&user.email)
    .execute(&state.pool)
    .await?;

    Ok(StatusCode::OK)
}
