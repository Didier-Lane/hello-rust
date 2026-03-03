//! HTTP request handlers for the hello-rust web server.
//!
//! This module contains all the HTTP handlers used by the Axum router.

#![forbid(unsafe_code)]

use axum::{
    extract::Json,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

/// Returns an HTML page with a greeting from Ferris, the Rust mascot.
///
/// # Example
///
/// ```bash
/// curl http://localhost:8080/
/// ```
///
/// # Errors
///
/// Returns an error page if the ASCII art generation fails.
pub async fn root() -> impl IntoResponse {
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut buffer = Vec::new();
    if ferris_says::say(&message, width, &mut buffer).is_err() {
        return Html(String::from(
            "<pre><code>Error generating ASCII art</code></pre>",
        ));
    }
    let output = String::from_utf8(buffer).unwrap_or_else(|_| "Error".to_string());

    Html(format!("<pre><code>{}</code></pre>", output))
}

/// Health check endpoint.
///
/// Returns `200 OK` when the server is running.
///
/// # Example
///
/// ```bash
/// curl http://localhost:8080/health
/// ```
pub async fn health() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "OK")
}

/// Returns a JSON greeting message.
///
/// # Example
///
/// ```bash
/// curl http://localhost:8080/api/hello
/// ```
///
/// # Response
///
/// ```json
/// {"message":"Hello from hello-rust server!"}
/// ```
pub async fn hello() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello from hello-rust server!".to_string(),
    })
}

/// Echoes back the JSON body sent in the request.
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8080/api/echo \
///   -H "Content-Type: application/json" \
///   -d '{"name":"test","value":42}'
/// ```
///
/// # Errors
///
/// - Returns `400 Bad Request` if the body is not valid JSON.
/// - Returns `413 Payload Too Large` if the body exceeds the size limit.
pub async fn echo(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    Json(payload)
}

/// Response struct for the `/api/hello` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    message: String,
}
