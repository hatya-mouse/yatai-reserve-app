mod config;

use crate::config::Config;
use axum::{Router, response::Html};

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app = Router::new().route("/", get(handler));

    // Load the configuration
    let config = Config::load();

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await;
}

async fn handler() -> Html<&'static str> {
    Html("<h1>This is a backend server of yatai-reserve-app!</h1>")
}
