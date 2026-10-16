use crate::{
    AppState,
    error::AppError,
    tui::println_info,
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
) -> Result<impl IntoResponse, AppError> {
    println_info(format!("Signin link requested for email {}", payload.email));

    let token = gen_secure_token();
    let expires_at = Utc::now() + Duration::from_mins(10);

    let mut tx = state.pool.begin().await?;

    // Remove the existing query from the email address
    sqlx::query("DELETE FROM auth_tokens WHERE email = ?")
        .bind(&payload.email)
        .execute(&mut *tx)
        .await?;

    // Insert the newly generated token and its expiration date
    sqlx::query("INSERT INTO auth_tokens (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&token)
        .bind(&payload.email)
        .bind(expires_at.timestamp())
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    // Send an email
    let subject = String::from("予約管理アプリメールアドレス認証");
    let verification_link = format!("{}{}", state.config.verification_url.clone(), token);
    let html_content = format!(
        r#"
        <p>予約管理アプリにサインインするには、以下のリンクを開いてください。</p>
        <p><a href={}>サインイン</a></p>
        <p>このリンクは発行から10分間有効です。</p>
        <p>このメールに心当たりがない場合は、破棄してください。</p>
        "#,
        verification_link
    );

    send_email(
        state.config.resend_key.clone(),
        state.config.email.clone(),
        payload.email.clone(),
        subject,
        html_content,
    )
    .await
    .ok();

    Ok(StatusCode::OK)
}
