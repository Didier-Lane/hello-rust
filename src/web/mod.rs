#![forbid(unsafe_code)]

pub mod error;
pub mod handlers;
pub mod router;

#[cfg(test)]
mod handlers_tests;

pub use router::create_router;
