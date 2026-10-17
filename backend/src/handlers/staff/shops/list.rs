use crate::{
    AppState,
    data_types::{AuthUser, ShopResponse},
    error::AppError,
};
use axum::{Extension, Json, extract::State};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ListResponse {
    shops: Vec<ShopResponse>,
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
            shops.notify_previous,
            shop_members.is_admin
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
