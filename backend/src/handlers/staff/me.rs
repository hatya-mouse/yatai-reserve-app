use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{Extension, Json, extract::State};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub(crate) struct MeResponse {
    name: String,
    email: String,
}

pub(crate) async fn me(
    Extension(user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> Result<Json<MeResponse>, AppError> {
    let me = sqlx::query_as(
        r#"
        SELECT staffs.name, staffs.email
        FROM staffs
        WHERE staffs.email = ?
        "#,
    )
    .bind(&user.email)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(me))
}
