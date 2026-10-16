use serde::Serialize;
use sqlx::prelude::FromRow;

/// Information of the reservation.
#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ReservationItem {
    pub id: i64,
    pub shop_id: String,
    pub email: String,
    pub status: String,
    pub created_at: i64,
}

/// Response of list reservations endpoint.
#[derive(Debug, Serialize)]
pub(crate) struct ReservationItems {
    pub reservations: Vec<ReservationItem>,
}

/// Information of the reservation for users.
#[derive(Debug, Serialize, FromRow)]
pub(crate) struct StatusItem {
    pub id: i64,
    pub status: String,
    pub created_at: i64,
    pub position: Option<i64>,
}

/// Reponse of the get status endpoint.
#[derive(Debug, Serialize)]
pub(crate) struct StatusResponse {
    pub reservations: Vec<StatusItem>,
}
