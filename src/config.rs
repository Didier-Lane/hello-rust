//! Configuration for the hello-rust HTTP server.
//!
//! This module provides the configuration structure used to configure
//! the HTTP server at runtime via command-line arguments.

#![forbid(unsafe_code)]

use clap::Parser;
use std::net::SocketAddr;

/// Command-line configuration for the HTTP server.
#[derive(Parser, Debug, Clone)]
#[command(name = "hello-rust")]
#[command(about = "HTTP server")]
pub struct Config {
    /// HTTP server port.
    ///
    /// Can also be set via the `HTTP_PORT` environment variable.
    #[arg(long, default_value = "8080", env = "HTTP_PORT")]
    pub http_port: u16,

    /// Logging level for the tracing subscriber.
    ///
    /// Common values: `error`, `warn`, `info`, `debug`, `trace`.
    /// Can also be set via the `RUST_LOG` environment variable.
    #[arg(long, default_value = "info", env = "RUST_LOG")]
    pub log_level: String,
}

impl Config {
    /// Returns the socket address for the HTTP server to bind to.
    pub fn http_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.http_port))
    }
}
