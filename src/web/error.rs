//! Error types for the hello-rust web server.
//!
//! This module defines the application's error types and their behavior.

#![forbid(unsafe_code)]

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    Io(std::io::Error),
    Axum(axum::Error),
}

impl Clone for AppError {
    fn clone(&self) -> Self {
        match self {
            AppError::Io(e) => AppError::Io(std::io::Error::new(e.kind(), e.to_string())),
            AppError::Axum(e) => AppError::Axum(axum::Error::new(e.to_string())),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Axum(e) => write!(f, "Axum error: {}", e),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Axum(e) => Some(e),
        }
    }
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AppError::Io(e1), AppError::Io(e2)) => e1.kind() == e2.kind(),
            (AppError::Axum(e1), AppError::Axum(e2)) => e1.to_string() == e2.to_string(),
            _ => false,
        }
    }
}

impl Eq for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<axum::Error> for AppError {
    fn from(err: axum::Error) -> Self {
        AppError::Axum(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Io(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
            AppError::Axum(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
