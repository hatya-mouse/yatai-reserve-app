use crate::{
    AppState,
    data_types::{ReservationItem, ReservationItems},
    error::AppError,
};
use axum::{
    Json,
    extract::{Path, State},
};

pub(crate) async fn list(
    State(state): State<AppState>,
    Path(shop_id): Path<String>,
) -> Result<Json<ReservationItems>, AppError> {
    let reservations = sqlx::query_as::<_, ReservationItem>(
        r#"
        SELECT id, shop_id, email, status, created_at
        FROM reservations
        WHERE shop_id = ?
        ORDER BY id ASC
        "#,
    )
    .bind(&shop_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ReservationItems { reservations }))
}
