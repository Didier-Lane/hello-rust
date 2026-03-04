#![forbid(unsafe_code)]

pub mod error;
pub mod handlers;
pub mod router;
pub mod tracing;

#[cfg(test)]
mod handlers_tests;

pub use router::create_router;
