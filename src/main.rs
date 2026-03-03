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

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.parse().unwrap()),
        )
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
