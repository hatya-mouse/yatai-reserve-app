use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AddShopMemberRequest {
    email: String,
}

pub(crate) async fn add_shop_member(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(shop_id): Path<String>,
    Json(body): Json<AddShopMemberRequest>,
) -> Result<StatusCode, AppError> {
    let email = body.email.trim().to_ascii_lowercase();

    if email.is_empty() || !email.contains('@') {
        return Err(AppError::BadRequest);
    }

    let mut tx = state.pool.begin().await?;

    let is_member = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM shop_members
            WHERE shop_id = ?
              AND email = ?
        )
        "#,
    )
    .bind(&shop_id)
    .bind(&user.email)
    .fetch_one(&mut *tx)
    .await?;

    if is_member == 0 {
        return Err(AppError::NotFound);
    }

    let result = sqlx::query(
        r#"
        INSERT INTO shop_members (shop_id, email)
        VALUES (?, ?)
        ON CONFLICT(shop_id, email) DO NOTHING
        "#,
    )
    .bind(&shop_id)
    .bind(&email)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::Conflict);
    }

    tx.commit().await?;

    Ok(StatusCode::CREATED)
}
