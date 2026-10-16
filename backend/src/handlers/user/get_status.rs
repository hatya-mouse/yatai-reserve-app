use crate::{AppState, error::AppError};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct StatusQuery {
    token: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ReservationStatusItem {
    reservation_id: i64,
    shop_id: String,
    shop_name: String,
    is_called: bool,
    position: Option<i64>,
    created_at: i64,
}

#[derive(Debug, Serialize)]
pub(crate) struct StatusResponse {
    reservations: Vec<ReservationStatusItem>,
}

pub(crate) async fn get_status(
    State(state): State<AppState>,
    Query(query): Query<StatusQuery>,
) -> Result<Json<StatusResponse>, AppError> {
    let now = Utc::now().timestamp();

    // Verify the token and get the mail address associated with it
    // Do not remove the status token so that it can be reused within the expiration date
    let email = sqlx::query_scalar::<_, String>(
        r#"
        SELECT email
        FROM status_tokens
        WHERE token = ?
          AND expires_at > ?
        "#,
    )
    .bind(&query.token)
    .bind(now)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    // Get all reservations
    let rows = sqlx::query_as::<_, (i64, String, String, bool, i64)>(
        r#"
        SELECT
            reservations.id,
            reservations.shop_id,
            shops.name,
            reservations.is_called,
            reservations.created_at
        FROM reservations
        INNER JOIN shops ON shops.id = reservations.shop_id
        WHERE reservations.email = ?
        ORDER BY reservations.created_at ASC, reservations.id ASC
        "#,
    )
    .bind(&email)
    .fetch_all(&state.pool)
    .await?;

    let mut reservations = Vec::with_capacity(rows.len());

    for (reservation_id, shop_id, shop_name, is_called, created_at) in rows {
        let position = if is_called {
            None
        } else {
            let count = sqlx::query_scalar::<_, i64>(
                r#"
                            SELECT COUNT(*)
                            FROM reservations
                            WHERE shop_id = ?
                              AND is_called
                              AND (
                                  created_at < ?
                                  OR (created_at = ? AND id < ?)
                              )
                            "#,
            )
            .bind(&shop_id)
            .bind(created_at)
            .bind(created_at)
            .bind(reservation_id)
            .fetch_one(&state.pool)
            .await?;

            Some(count + 1)
        };

        reservations.push(ReservationStatusItem {
            reservation_id,
            shop_id,
            shop_name,
            is_called,
            position,
            created_at,
        });
    }

    Ok(Json(StatusResponse { reservations }))
}
