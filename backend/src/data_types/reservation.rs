use serde::Serialize;
use sqlx::prelude::FromRow;

/// Information of the reservation.
#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ReservationItem {
    pub id: i64,
    pub email: String,
    pub status: String,
    pub created_at: i64,
    pub position: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct StatusResponse {
    pub reservations: Vec<ReservationItem>,
}
