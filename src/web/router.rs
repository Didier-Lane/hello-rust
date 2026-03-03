#![forbid(unsafe_code)]

use crate::web::handlers::{echo, health, hello, root};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/hello", get(hello))
        .route("/api/echo", post(echo))
        .layer(CorsLayer::permissive())
}
