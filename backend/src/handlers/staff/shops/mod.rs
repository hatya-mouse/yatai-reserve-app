mod add_member;
mod create;
mod get;
mod get_members;
mod list;
pub(crate) mod reservations;
mod update;

pub(crate) use add_member::add_member;
pub(crate) use create::create;
pub(crate) use get::get;
pub(crate) use get_members::get_members;
pub(crate) use list::list;
pub(crate) use update::update;

use crate::{data_types::AuthUser, error::AppError};
use axum::{
    extract::{OriginalUri, Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;

pub(crate) async fn shop_member_middleware(
    State(pool): State<SqlitePool>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let shop_id = request
        .extensions()
        .get::<OriginalUri>()
        .and_then(|uri| {
            uri.0
                .path()
                .split('/')
                .collect::<Vec<_>>()
                .windows(2)
                .find_map(|parts| (parts[0] == "shops").then_some(parts[1]))
                .filter(|id| !id.is_empty())
        })
        .ok_or(AppError::BadRequest)?;

    let user = request
        .extensions()
        .get::<AuthUser>()
        .ok_or(AppError::Unauthorized)?;

    let is_member: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM shop_members
            WHERE shop_id = ?
              AND email = ?
        )
        "#,
    )
    .bind(shop_id)
    .bind(&user.email)
    .fetch_one(&pool)
    .await?;

    if !is_member {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}
