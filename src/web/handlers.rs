#![forbid(unsafe_code)]

use axum::{
    extract::Json,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

pub async fn root() -> Html<String> {
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

pub async fn health() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "OK")
}

pub async fn hello() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello from hello-rust server!".to_string(),
    })
}

pub async fn echo(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    Json(payload)
}

#[derive(Serialize, Deserialize)]
pub struct HelloResponse {
    message: String,
}
