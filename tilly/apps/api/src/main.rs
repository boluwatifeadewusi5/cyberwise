mod models;
mod routes;
mod services;
mod state;

use axum::{Json, Router, routing::get};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/users", routes::users::router())
        .nest("/hustles", routes::hustles::router())
        .nest("/transactions", routes::transactions::router())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind on 0.0.0.0:8080");

    println!("TILLY API listening on http://0.0.0.0:8080");
    axum::serve(listener, app)
        .await
        .expect("server exited unexpectedly");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({"status": "ok", "service": "tilly-api"}))
}
