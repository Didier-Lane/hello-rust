//! HTTP router configuration for the hello-rust web server.
//!
//! This module defines the routes and middleware for the application.

#![forbid(unsafe_code)]

use crate::web::handlers::{echo, health, hello, root};
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

const MAX_PAYLOAD_SIZE: usize = 64 * 1024; // 64KB

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/hello", get(hello))
        .route("/api/echo", post(echo))
        .layer(DefaultBodyLimit::max(MAX_PAYLOAD_SIZE))
        .layer(CorsLayer::permissive().max_age(std::time::Duration::from_secs(86400)))
}
