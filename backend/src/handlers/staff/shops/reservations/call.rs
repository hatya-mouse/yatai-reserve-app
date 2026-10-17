use crate::{AppState, error::AppError, utils::send_email};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};

pub(crate) async fn call(
    State(state): State<AppState>,
    Path((shop_id, reservation_id)): Path<(String, i64)>,
) -> Result<StatusCode, AppError> {
    let mut tx = state.pool.begin().await?;

    let (email, shop_id, shop_name) = sqlx::query_as::<_, (String, String, String)>(
        r#"
            SELECT r.email, r.shop_id, s.name
            FROM reservations AS r
            JOIN shops AS s ON s.id = r.shop_id
            WHERE r.id = ? AND r.shop_id = ?
            "#,
    )
    .bind(reservation_id)
    .bind(&shop_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NotFound)?;

    // Set the status to called
    sqlx::query(
        r#"
        UPDATE reservations
            SET status = 'called'
            WHERE id = ? AND shop_id = ?
        "#,
    )
    .bind(reservation_id)
    .bind(&shop_id)
    .execute(&mut *tx)
    .await?;

    let next_email: Option<String> = sqlx::query_scalar(
        r#"
        SELECT email
            FROM reservations
            WHERE shop_id = ?
                AND id > ?
                AND status = 'waiting'
            ORDER BY id ASC
            LIMIT 1
        "#,
    )
    .bind(&shop_id)
    .bind(reservation_id)
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;

    send_email(
        state.config.resend_key.clone(),
        state.config.email.clone(),
        email,
        format!("【{}】お呼び出しのお知らせ", shop_name),
        format!("<p>{}をご予約いただきありがとうございます。</p><p>お客様の順番になりましたので、お知らせいたします。<br/>お手数ですが、受付までお越しください。</p>", shop_name),
    )
    .await
    .map_err(|_| AppError::Internal)?;

    if let Some(email) = next_email {
        send_email(
            state.config.resend_key.clone(),

            state.config.email.clone(),
            email,
            String::from("【{}】まもなく順番です"),
            format!(
                "<p>{}をご予約いただきありがとうございます。</p>
                <p>お客様の順番が近づいてまいりました。<br/>まもなくお呼び出しいたしますので、受付付近でお待ちいただくか、すぐにお越しいただけるようご準備ください。</p>
                <p>順番になりましたら、改めてメールでお知らせいたします。</p>",
                shop_name
            ),
        )
        .await
        .map_err(|_| AppError::Internal)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
