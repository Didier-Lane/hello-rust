# AGENTS.md - Guidelines for Agentic Coding in this Repository

This is a Rust project using Cargo with an Axum HTTP server.

## Code Style Guidelines

### Imports
- Use standard library imports first, then external crates
- Group imports by crate: std → external → crate
- Use absolute paths for external crates: `use axum::{Router, routing::get};`
- Prefer importing specific items: `use std::io::{BufWriter, Write};`

```rust
// Good
use std::io::{BufWriter, stdout, Write};
use axum::{Router, routing::get};

// Avoid
use std::io::*;
use axum::*;
```

### Formatting
- Follow `rustfmt` settings in `rustfmt.toml` (4 spaces, 100 char max)
- Always run `cargo fmt` before committing

### Types
- Prefer explicit type annotations for function signatures
- Use idiomatic Rust types: `&str` over `String` for references
- Use `Result<T, E>` for error handling

### Error Handling
- Use `anyhow::Result<T>` for application-level errors
- Define custom error types in `src/web/error.rs`
- Implement `IntoResponse` for custom errors

### Patterns to Follow
- **Module organization**: `src/web/` for HTTP-related code
- **Handlers**: Keep handlers thin, delegate logic to services
- **State**: Use `Arc<AppState>` for shared state
- **Configuration**: Use `clap` for CLI, env vars for runtime config

### Patterns to Avoid
- `unwrap()` / `expect()` in production code
- `panic!()` for control flow
- Global mutable state
- Putting business logic in handlers

### Testing
- Add unit tests for handlers
- Use `tower::ServiceExt` for testing routers
- Mock external dependencies

## Security Guidelines (ANSSI-inspired)

Based on [ANSSI Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/).

### Unsafe Code
- **Never use `unsafe` code** unless absolutely necessary for FFI
- All source files must have `#![forbid(unsafe_code)]` at the top
- If FFI is required, isolate `unsafe` blocks and document thoroughly

### Memory Safety
- Never use `std::mem::forget` (causes memory leaks)
- Use `Box`, `Vec`, and other smart pointers for memory management
- Avoid manual memory management

### Error Handling
- Prefer explicit error handling (`Result`) over `panic!`
- Never use `panic!` for control flow
- Implement proper error types with `thiserror` or `anyhow`

### Input Validation
- Always validate user input
- Use size limits on request bodies
- Validate JSON payloads before processing

### Dependencies
- Run `cargo audit` regularly to check for vulnerabilities
- Run `cargo outdated` to check for outdated dependencies
- Pin dependency versions when possible

## Rust API Guidelines

Based on [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html).

### Documentation (C-CRATE-DOC, C-EXAMPLE, C-FAILURE)
- All crates must have comprehensive documentation with examples
- All public functions must have rustdoc examples showing usage
- Function docs must include `# Errors`, `# Panics`, and `# Safety` sections when applicable
- Use `?` operator instead of `try!` or `unwrap` in examples

### Interoperability (C-COMMON-TRAITS, C-GOOD-ERR)
- Implement common traits: `Debug`, `Clone`, `PartialEq`, `Eq`, `Display`, `Error`
- Error types must implement `source()` for error chaining
- Error types should be `Send` and `Sync` where possible
- Use `thiserror` or `anyhow` for error handling

### Validation (C-VALIDATE)
- Validate all user input
- Set size limits on request bodies (e.g., 64KB for JSON)
- Use explicit error types with meaningful messages

### Cargo.toml Metadata (C-METADATA)
Required fields in `[package]`:
- `license` - e.g., "MIT"
- `repository` - URL to source repository
- `homepage` - URL to project homepage
- `documentation` - URL to docs.rs
- `keywords` - array of relevant keywords
- `categories` - array of crate categories

## Git Conventions
- Use conventional commit messages: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`
- Run `cargo fmt` and `cargo clippy` before committing
- Ensure code compiles and passes tests before submitting
- Create feature branches for new features (e.g., `feature/http-server`)
