use crate::{
    AppState,
    data_types::{AuthUser, ShopDetailResponse, ShopMemberResponse, ShopResponse},
    error::AppError,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};

pub(crate) async fn get_shop(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(shop_id): Path<String>,
) -> Result<Json<ShopDetailResponse>, AppError> {
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
          AND shop_members.email = ?
        "#,
    )
    .bind(&shop_id)
    .bind(&user.email)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let members = sqlx::query_as::<_, ShopMemberResponse>(
        r#"
        SELECT email
        FROM shop_members
        WHERE shop_id = ?
        ORDER BY email
        "#,
    )
    .bind(&shop_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ShopDetailResponse { shop, members }))
}
