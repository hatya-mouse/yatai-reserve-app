use crate::{AppState, data_types::AuthUser, error::AppError};
use axum::{Extension, Json, extract::State};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize)]
pub(crate) struct ListResponse {
    shops: Vec<ShopResponse>,
}

#[derive(Serialize, FromRow)]
pub(crate) struct ShopResponse {
    id: String,
    name: String,
    notify_previous: bool,
    is_admin: bool,
}

pub(crate) async fn list(
    Extension(user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> Result<Json<ListResponse>, AppError> {
    let shops = sqlx::query_as(
        r#"
        SELECT
            shops.id,
            shops.name,
            shops.notify_previous AS "notify_previous: bool",
            shops.is_admin A "is_admin: bool"
        FROM shops
        INNER JOIN shop_members
            ON shops.id = shop_members.shop_id
        WHERE shop_members.email = ?
        ORDER BY shops.name
        "#,
    )
    .bind(&user.email)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ListResponse { shops }))
}
