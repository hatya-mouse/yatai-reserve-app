mod cancel;
mod get_status;
mod request_status;
mod reserve;

pub(crate) use cancel::cancel;
pub(crate) use get_status::get_status;
pub(crate) use request_status::request_status;
pub(crate) use reserve::reserve;

use crate::{AppState, error::AppError, utils::gen_secure_token};
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub(crate) struct StatusQuery {
    token: String,
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
