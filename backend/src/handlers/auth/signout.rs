use crate::{AppState, error::AppError, println_info};
use axum::{
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

pub(crate) async fn signout(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    println_info("Signout requested");

    // Delete the session from the database
    if let Some(cookie) = cookies.get("session") {
        sqlx::query("DELETE FROM sessions WHERE token = ?")
            .bind(cookie.value())
            .execute(&state.pool)
            .await?;
    }

    // Remove the session token
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        "session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0".parse()?,
    );

    Ok(response)
}
