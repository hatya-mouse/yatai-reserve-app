mod add_shop_member;
mod get_shop;
mod update_shop;

pub(crate) use add_shop_member::add_shop_member;
pub(crate) use get_shop::get_shop;
pub(crate) use update_shop::update_shop;

use axum::{Extension, Json, extract::State};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    data_types::{AuthUser, ShopResponse},
    error::AppError,
};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateShopRequest {
    name: String,
    #[serde(default)]
    notify_previous: bool,
}

pub(crate) async fn create_shop(
    State(pool): State<SqlitePool>,
    Extension(user): Extension<AuthUser>,
    Json(body): Json<CreateShopRequest>,
) -> Result<Json<ShopResponse>, AppError> {
    let name = body.name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest);
    }

    let shop_id = Uuid::new_v4().to_string();

    let mut tx = pool.begin().await?;

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
