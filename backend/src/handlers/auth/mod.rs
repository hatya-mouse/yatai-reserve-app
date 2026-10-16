mod me;
mod shops;
mod signout;

pub(crate) use me::me;
pub(crate) use signout::signout;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use sqlx::SqlitePool;

use crate::{data_types::AuthUser, tui::println_error};

pub(crate) async fn auth_middleware(
    State(pool): State<SqlitePool>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let session_token = request
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|header| header.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let (name, value) = cookie.trim().split_once('=')?;
                (name == "session").then_some(value)
            })
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = sqlx::query_as::<_, AuthUser>(
        "SELECT email FROM sessions WHERE token = ? AND expires_at > ?",
    )
    .bind(session_token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        println_error(format!("DB Error: {}", e));
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
