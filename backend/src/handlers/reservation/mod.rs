mod get_status;
mod request_status;
mod reserve;

pub(crate) use get_status::get_status;
pub(crate) use request_status::request_status;
pub(crate) use reserve::reserve;

use crate::{
    AppState,
    error::AppError,
    tui::println_error,
    utils::{gen_secure_token, send_email},
};
use chrono::Utc;
use std::time::Duration;

/// Sends the reservation status email to the given email address.
async fn send_status_email(state: &AppState, email: &str, token: &str) -> Result<(), AppError> {
    let config = &state.config;
    let link = format!("{}{}", config.status_url, token);

    let subject = "予約状況の確認リンク";
    let html_content = format!(
        r#"
        <p>予約状況を確認するには、以下のリンクを開いてください。</p>
        <p><a href="{link}">予約状況を確認する</a></p>
        <p>このリンクは発行から24時間有効です。</p>
        <p>このメールに心当たりがない場合は、破棄してください。</p>
        "#
    );

    send_email(
        config.resend_key.clone(),
        config.email.clone(),
        email.to_owned(),
        subject.to_owned(),
        html_content,
    )
    .await
    .map_err(|e| {
        println_error(format!("Failed to send status email: {e}"));
        AppError::Internal
    })?;

    Ok(())
}

/// Renews the status token for the given email address and returns the newly generated token.
async fn renew_status_token(state: &AppState, email: &str) -> Result<String, AppError> {
    // Generate a new token for the email address
    let new_token = gen_secure_token();
    let expires_at = Utc::now() + Duration::from_hours(24);

    // Update the token for the email address
    let mut tx = state.pool.begin().await?;

    sqlx::query("DELETE FROM status_tokens WHERE email = ?")
        .bind(email)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO status_tokens (token, email, expires_at) VALUES (?, ?, ?)")
        .bind(&new_token)
        .bind(email)
        .bind(expires_at.timestamp())
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(new_token)
}
