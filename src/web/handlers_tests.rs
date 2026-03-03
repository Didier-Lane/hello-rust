#![cfg(test)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::web::create_router;

#[tokio::test]
async fn test_root() {
    let app = create_router();
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("Hello fellow Rustaceans!"));
}

#[tokio::test]
async fn test_health() {
    let app = create_router();
    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_str, "OK");
}

#[tokio::test]
async fn test_hello() {
    let app = create_router();
    let response = app
        .oneshot(Request::get("/api/hello").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["message"], "Hello from hello-rust server!");
}

#[tokio::test]
async fn test_echo() {
    let app = create_router();
    let payload = json!({"name": "test", "value": 42});
    let body = Body::from(payload.to_string());

    let response = app
        .oneshot(
            Request::post("/api/echo")
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["name"], "test");
    assert_eq!(json["value"], 42);
}

#[tokio::test]
async fn test_not_found() {
    let app = create_router();
    let response = app
        .oneshot(Request::get("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
