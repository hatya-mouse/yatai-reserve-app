mod config;
mod db;
mod handlers;

use crate::config::Config;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

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

    // Initialize the database
    db::init_db(&pool)
        .await
        .expect("could not initialize the database");

    // Start the server
    let app = Router::new()
        .route("/auth/request-link", post(handlers::auth::request_link))
        .route("/auth/verify", get(handlers::auth::verify))
        .with_state(pool);

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.ok();
}
