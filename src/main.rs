//! Hello Rust - An HTTP server built with Axum.
//!
//! A simple HTTP server that demonstrates Rust web development best practices.
//!
//! ## Features
//!
//! - RESTful API endpoints
//! - Graceful shutdown handling (SIGTERM/SIGINT)
//! - Configurable logging and port
//! - CORS support
//!
//! ## Usage
//!
//! ```bash
//! # Run with defaults (port 8080)
//! cargo run
//!
//! # Specify custom port
//! cargo run -- --http-port 3000
//!
//! # Set log level
//! cargo run -- --log-level debug
//! ```
//!
//! ## API Endpoints
//!
//! - `GET /` - Returns HTML with Ferris ASCII art
//! - `GET /health` - Health check endpoint
//! - `GET /api/hello` - JSON greeting
//! - `POST /api/echo` - Echoes JSON body back

#![forbid(unsafe_code)]

mod config;
mod web;

use clap::Parser;
use config::Config;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting hello-rust server");
    tracing::info!("HTTP port: {}", config.http_port);

    let app = web::create_router();

    let addr = config.http_addr();
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("HTTP server listening on http://{}", addr);

    let server = axum::serve(listener, app);

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM, shutting down gracefully...");
        }
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT, shutting down gracefully...");
        }
    }

    tracing::info!("Server shutdown complete");

    Ok(())
}
