mod me;
mod request_signin;
mod set_name;
pub(crate) mod shops;
mod signout;
mod verify;

pub(crate) use me::me;
pub(crate) use request_signin::request_signin;
pub(crate) use set_name::set_name;
pub(crate) use signout::signout;
pub(crate) use verify::verify;

use crate::{data_types::AuthUser, error::AppError};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use sqlx::SqlitePool;

pub(crate) async fn auth_middleware(
    State(pool): State<SqlitePool>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let session_token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or(AppError::Unauthorized)?;

    let user = sqlx::query_as::<_, AuthUser>(
        "SELECT email FROM sessions WHERE token = ? AND expires_at > ?",
    )
    .bind(session_token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
