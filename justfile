# Generic Rust task runner
# Install `just` once: https://github.com/casey/just

_default:
    @just --list

# Lint with pedantic lints enabled and warnings as errors.
# Adjust the target flag or remove it if you are not building for WASM.
lint:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format the project
fmt:
    cargo fmt --all

# Run tests
test:
    cargo test

# Build the project in release mode
build:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# Run the full validation suite used in CI
validate: lint fmt-check test build
