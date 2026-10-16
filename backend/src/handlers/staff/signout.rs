use crate::{AppState, error::AppError, println_info};
use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;

pub(crate) async fn signout(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<StatusCode, AppError> {
    println_info("Signout requested");

    // Delete the session from the database
    if let Some(cookie) = cookies.get("session") {
        sqlx::query("DELETE FROM sessions WHERE token = ?")
            .bind(cookie.value())
            .execute(&state.pool)
            .await?;
    }

    Ok(StatusCode::OK)
}
