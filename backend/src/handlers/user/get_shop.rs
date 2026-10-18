use crate::{AppState, data_types::ShopResponse, error::AppError};
use axum::{
    Json,
    extract::{Path, State},
};

pub(crate) async fn get_shop(
    State(state): State<AppState>,
    Path(shop_id): Path<String>,
) -> Result<Json<ShopResponse>, AppError> {
    let shop = sqlx::query_as::<_, ShopResponse>(
        r#"
        SELECT
            shops.id,
            shops.name,
            shops.notify_previous
        FROM shops
        INNER JOIN shop_members
            ON shop_members.shop_id = shops.id
        WHERE shops.id = ?
        "#,
    )
    .bind(&shop_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(shop))
}
