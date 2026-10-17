use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
};

pub(crate) async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(shop_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let mut tx = state.pool.begin().await?;

    // Check if the user is an administrator of the shop
    let result = sqlx::query(
        r#"
        DELETE FROM shops
        WHERE id = ?
          AND EXISTS (
              SELECT 1
              FROM shop_members
              WHERE shop_members.shop_id = shops.id
                AND shop_members.email = ?
                AND shop_members.is_admin = TRUE
          )
        "#,
    )
    .bind(&shop_id)
    .bind(&user.email)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
