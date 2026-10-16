use crate::{AppState, data_types::AuthUser, error::AppError, handlers::staff::me::ShopResponse};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateShopRequest {
    name: Option<String>,
    notify_previous: Option<bool>,
}

pub(crate) async fn update_shop(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(shop_id): Path<String>,
    Json(body): Json<UpdateShopRequest>,
) -> Result<Json<ShopResponse>, AppError> {
    // If the both name and notify_previous is none, reject as bad request
    if body.name.is_none() && body.notify_previous.is_none() {
        return Err(AppError::BadRequest);
    }

    if let Some(name) = &body.name
        && name.trim().is_empty()
    {
        return Err(AppError::BadRequest);
    }

    let result = sqlx::query(
        r#"
        UPDATE shops
        SET
            name = COALESCE(?, name),
            notify_previous = COALESCE(?, notify_previous)
        WHERE id = ?
          AND EXISTS (
              SELECT 1
              FROM shop_members
              WHERE shop_members.shop_id = shops.id
                AND shop_members.email = ?
          )
        "#,
    )
    .bind(body.name.as_ref().map(|name| name.trim()))
    .bind(body.notify_previous)
    .bind(&shop_id)
    .bind(&user.email)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    let shop = sqlx::query_as::<_, ShopResponse>(
        r#"
        SELECT id, name, notify_previous
        FROM shops
        WHERE id = ?
        "#,
    )
    .bind(&shop_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(shop))
}
