use crate::{
    AppState,
    data_types::{ShopMemberResponse, ShopMembersResponse},
    error::AppError,
};
use axum::{
    Json,
    extract::{Path, State},
};

pub(crate) async fn get_members(
    State(state): State<AppState>,
    Path(shop_id): Path<String>,
) -> Result<Json<ShopMembersResponse>, AppError> {
    let members = sqlx::query_as::<_, ShopMemberResponse>(
        r#"
        SELECT email
        FROM shop_members
        WHERE shop_id = ?
        ORDER BY email
        "#,
    )
    .bind(&shop_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ShopMembersResponse { members }))
}
