mod data_types;
mod db;
mod error;
mod handlers;
mod state;
mod tui;
mod utils;

use crate::{
    handlers::staff::{auth_middleware, shops::shop_member_middleware},
    state::{AppState, Config},
    tui::println_info,
};
use axum::{
    Router, middleware,
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
    // Clean up the database
    db::clean_db(&pool)
        .await
        .expect("could not clean up the database");

    println_info("Starting the server...");

    // Start the server
    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let state = AppState { pool, config };
    let shops_routes = Router::new()
        .route(
            "/",
            get(handlers::staff::shops::get).patch(handlers::staff::shops::update),
        )
        .route(
            "/members",
            get(handlers::staff::shops::get_members).post(handlers::staff::shops::add_member),
        )
        .route(
            "/reservations",
            get(handlers::staff::shops::reservations::list),
        )
        .route(
            "/reservations/{reservation_id}",
            post(handlers::staff::shops::reservations::call)
                .patch(handlers::staff::shops::reservations::update),
        )
        .route_layer(middleware::from_fn_with_state(
            state.pool.clone(),
            shop_member_middleware,
        ));

    let staff_routes = Router::new()
        .route("/signout", post(handlers::staff::signout))
        .route("/me", get(handlers::staff::me))
        .route("/set_name", post(handlers::staff::set_name))
        .route(
            "/shops",
            get(handlers::staff::shops::list).post(handlers::staff::shops::create),
        )
        .nest("/shops/{shop_id}", shops_routes)
        .route_layer(middleware::from_fn_with_state(
            state.pool.clone(),
            auth_middleware,
        ));

    let user_routes = Router::new()
        .route("/shops/{shop_id}/reserve", post(handlers::user::reserve))
        .route(
            "/reservations/{reservation_id}/cancel",
            post(handlers::user::cancel),
        )
        .route("/request-status", post(handlers::user::request_status))
        .route("/status", get(handlers::user::get_status));

    let app = Router::new()
        .route("/request-signin", post(handlers::staff::request_signin))
        .route("/verify", get(handlers::staff::verify))
        .nest("/staff", staff_routes)
        .nest("/user", user_routes)
        .with_state(state);

    println_info(format!("Listening on {}", listener.local_addr().unwrap()));
    axum::serve(listener, app).await.ok();
}
