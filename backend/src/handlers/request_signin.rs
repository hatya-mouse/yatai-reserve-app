use crate::{
    AppState,
    tui::{println_error, println_info},
    utils::{gen_secure_token, send_email},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
pub(crate) struct RequestSigninBody {
    pub email: String,
}

pub(crate) async fn request_signin(
    State(state): State<AppState>,
    Json(payload): Json<RequestSigninBody>,
) -> Result<impl IntoResponse, StatusCode> {
    println_info(format!("Login link requested for email {}", payload.email));

    let token = gen_secure_token();
    let expires_at = Utc::now() + Duration::from_mins(10);

    let mut tx = state.pool.begin().await.map_err(|e| {
        println_error(format!("DB Error: {}", e));
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Remove the existing query from the email address
    sqlx::query("DELETE FROM auth_tokens WHERE email = ?")
        .bind(&payload.email)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Insert the newly generated token and its expiration date
    sqlx::query("INSERT INTO auth_tokens (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&token)
        .bind(&payload.email)
        .bind(expires_at.timestamp())
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            println_error(format!("DB Error: {}", e));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tx.commit().await.map_err(|e| {
        println_error(format!("DB Error: {}", e));
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Send an email
    tokio::spawn(async move {
        let subject = String::from("メールアドレス認証");
        let verification_link = format!("{}{}", state.config.verification_url.clone(), token);
        let html_content = format!(
            "10分以内に下のリンクをクリックしてサインインしてください。\n<a href={}>{}</a>",
            verification_link, verification_link
        );

        send_email(
            state.config.resend_key.clone(),
            state.config.email.clone(),
            payload.email.clone(),
            subject,
            html_content,
        )
        .await
    });

    Ok(StatusCode::OK)
}
