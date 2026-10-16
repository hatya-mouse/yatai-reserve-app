use crate::{AppState, println_info, tui::println_error};
use axum::{
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

pub(crate) async fn signout(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    println_info("Signout requested");

    // Delete the session from the database
    if let Some(cookie) = cookies.get("session") {
        sqlx::query("DELETE FROM sessions WHERE token = ?")
            .bind(cookie.value())
            .execute(&state.pool)
            .await
            .map_err(|e| {
                println_error(format!("DB Error: {}", e));
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    // Remove the session token
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        "session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0"
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}
