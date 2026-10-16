use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ShopResponse {
    pub id: String,
    pub name: String,
    pub notify_previous: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ShopMemberResponse {
    pub email: String,
}

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct ShopDetailResponse {
    pub shop: ShopResponse,
    pub members: Vec<ShopMemberResponse>,
}
