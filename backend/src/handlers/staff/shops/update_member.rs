use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateShopMemberRequest {
    email: String,
    is_admin: bool,
}

pub(crate) async fn update_member(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(shop_id): Path<String>,
    Json(body): Json<UpdateShopMemberRequest>,
) -> Result<StatusCode, AppError> {
    let email = body.email.trim().to_ascii_lowercase();

    if email.is_empty() || !email.contains('@') {
        return Err(AppError::BadRequest);
    }

    let mut tx = state.pool.begin().await?;

    // Check if the user is an administrator of the shop
    let is_admin = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM shop_members
            WHERE shop_id = ?
              AND email = ?
              AND is_admin = TRUE
        )
        "#,
    )
    .bind(&shop_id)
    .bind(&user.email)
    .fetch_one(&mut *tx)
    .await?;

    if is_admin == 0 {
        return Err(AppError::NotFound);
    }

    let result = sqlx::query(
        r#"
        UPDATE shop_members
        SET is_admin = ?
        WHERE shop_id = ?
          AND email = ?
        "#,
    )
    .bind(body.is_admin)
    .bind(&shop_id)
    .bind(&email)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
