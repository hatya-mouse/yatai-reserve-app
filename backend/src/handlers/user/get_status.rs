use crate::{
    AppState,
    data_types::{StatusItem, StatusResponse},
    error::AppError,
    handlers::user::StatusQuery,
};
use axum::{
    Json,
    extract::{Query, State},
};

pub(crate) async fn get_status(
    State(state): State<AppState>,
    Query(query): Query<StatusQuery>,
) -> Result<Json<StatusResponse>, AppError> {
    use chrono::Utc;

    let email: String = sqlx::query_scalar(
        r#"
        SELECT email
            FROM status_tokens
            WHERE token = ?
            AND expires_at > ?
        "#,
    )
    .bind(&query.token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let reservations = sqlx::query_as::<_, StatusItem>(
        r#"
        SELECT
            r.id,
            r.shop_id,
            s.name AS shop_name,
            r.status,
            r.created_at,
            CASE
                WHEN s.notify_previous = 1 AND r.status = 'waiting'
                THEN (
                    SELECT COUNT(*)
                    FROM reservations AS prev
                    WHERE prev.shop_id = r.shop_id
                      AND prev.status = 'waiting'
                      AND prev.id < r.id
                )
                ELSE NULL
            END AS position
        FROM reservations AS r
        JOIN shops AS s ON s.id = r.shop_id
        WHERE r.email = ?
        ORDER BY r.created_at ASC, r.id ASC;
        "#,
    )
    .bind(&email)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(StatusResponse { reservations }))
}
