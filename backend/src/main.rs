mod config;

use std::time::Duration;

use crate::config::Config;
use axum::{Router, response::Html, routing::get};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();
    // Load the configuration
    let config = Config::load();

    // Load the database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("could not connect to the database");

    // Start the server
    let app = Router::new().route("/", get(handler)).with_state(pool);

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.ok();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>This is a backend server of yatai-reserve-app!</h1>")
}
