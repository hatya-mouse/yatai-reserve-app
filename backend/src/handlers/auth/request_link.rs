use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestLinkBody {
    pub email: String,
}

pub(crate) async fn request_link(Json(payload): Json<RequestLinkBody>) {
    sqlx::raw_sql("");

    println!("Login link requested for email {}", payload.email);
}
