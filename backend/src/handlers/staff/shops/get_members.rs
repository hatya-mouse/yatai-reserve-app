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
        SELECT
            shop_members.email,
            shop_members.is_admin,
            staffs.name
        FROM shop_members
        LEFT JOIN staffs
            ON shop_members.email = staffs.email
        WHERE shop_members.shop_id = ?
        ORDER BY shop_members.email;
        "#,
    )
    .bind(&shop_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ShopMembersResponse { members }))
}
