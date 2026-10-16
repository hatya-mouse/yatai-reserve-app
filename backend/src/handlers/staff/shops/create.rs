use crate::{
    AppState,
    data_types::{AuthUser, ShopResponse},
    error::AppError,
};
use axum::{Extension, Json, extract::State};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateShopRequest {
    name: String,
    #[serde(default)]
    notify_previous: bool,
}

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(body): Json<CreateShopRequest>,
) -> Result<Json<ShopResponse>, AppError> {
    let name = body.name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest);
    }

    let shop_id = Uuid::new_v4().to_string();

    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO shops (id, name, notify_previous)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&shop_id)
    .bind(name)
    .bind(body.notify_previous)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO shop_members (shop_id, email)
        VALUES (?, ?)
        "#,
    )
    .bind(&shop_id)
    .bind(&user.email)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ShopResponse {
        id: shop_id,
        name: name.to_owned(),
        notify_previous: body.notify_previous,
    }))
}
