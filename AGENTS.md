# AGENTS.md - Guidelines for Agentic Coding in this Repository

This is a Rust project using Cargo. The following guidelines should be followed when making code changes.

## Project Overview

- **Project name**: hello-rust
- **Type**: Simple Rust CLI application
- **Edition**: 2021 (note: Cargo.toml incorrectly states "2024")
- **Dependencies**: ferris-says 0.3.2

## Build / Lint / Test Commands

### Build
```bash
cargo build        # Build debug mode
cargo build --release  # Build release mode
cargo run          # Build and run
```

### Linting
```bash
cargo fmt          # Format code (run before committing)
cargo fmt --check # Check formatting without modifying
cargo clippy       # Run lints and warnings
cargo clippy -- -D warnings  # Treat warnings as errors
```

### Testing
```bash
cargo test              # Run all tests
cargo test <test_name>   # Run a single test (partial name match)
cargo test -- --nocapture    # Show print output during tests
```

### Additional
```bash
cargo check     # Type-check without building
cargo doc        # Build documentation
cargo doc --open # Build and open documentation
```

## Code Style Guidelines

### Imports
- Use standard library imports first, then external crates
- Group imports by crate: std → external → crate
- Use absolute paths for external crates: `use ferris_says::say;`
- Prefer importing specific items: `use std::io::{BufWriter, Write};`

```rust
// Good
use std::io::{BufWriter, stdout, Write};
use ferris_says::say;

// Avoid
use std::io::*;
use ferris_says::*;
```

### Formatting
- Follow `rustfmt` default settings (4 spaces, 100 char max line length)
- Always run `cargo fmt` before committing
- Use trailing commas in multi-line collections

### Types
- Prefer explicit type annotations for function signatures
- Use idiomatic Rust types: `&str` over `String` for references, `&[T]` over `Vec<T>` for slices
- Use `Result<T, E>` for error handling; avoid `Option` unless truly optional

### Naming Conventions
- **Variables/functions**: snake_case (`my_variable`, `calculate_value`)
- **Types/Enums**: PascalCase (`MyStruct`, `ErrorKind`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Traits**: Noun form (`Clone`, `Display`, `Iterator`)
- **Booleans**: Prefix with `is_`, `has_`, `should_`, etc.

### Error Handling
- Use `Result<T, E>` for fallible operations
- Prefer `?` operator over `match` for simple error propagation
- Use `anyhow` or `thiserror` for application-level errors (add as dependencies if needed)
- Avoid `unwrap()` in production code; use `expect()` with descriptive messages if absolutely necessary

```rust
// Good
fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Avoid
fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}
```

### Patterns to Follow
- **Early returns**: Use guard clauses to reduce nesting
- **Small functions**: Prefer single-responsibility functions
- **Documentation**: Add doc comments (`///`) for public APIs
- **Ownership**: Prefer borrowing over cloning unless necessary

### Patterns to Avoid
- `unwrap()` / `expect()` in production code
- `panic!()` for control flow
- Global mutable state
- Unnecessary boxing (`Box<T>` when not needed)

### Testing
- Add unit tests in the same file using `#[cfg(test)]` module
- Add integration tests in `tests/` directory
- Use descriptive test names: `test_descriptive_name()`
- Follow Arrange-Act-Assert pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_returns_expected_value() {
        let input = 42;
        let result = my_function(input);
        assert_eq!(result, expected);
    }
}
```

### Git Conventions
- Use conventional commit messages: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`
- Run `cargo fmt` and `cargo clippy` before committing
- Ensure code compiles and passes tests before submitting

## Notes
- This is a simple project with minimal dependencies
- No CI/CD configuration exists yet (consider adding GitHub Actions)
- Consider adding `rustfmt.toml` and `clippy.toml` for team consistency
