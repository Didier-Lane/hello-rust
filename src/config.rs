#![forbid(unsafe_code)]

use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug, Clone)]
#[command(name = "hello-rust")]
#[command(about = "HTTP server")]
pub struct Config {
    #[arg(long, default_value = "8080")]
    pub http_port: u16,

    #[arg(long, default_value = "info")]
    pub log_level: String,
}

impl Config {
    pub fn http_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.http_port))
    }
}
