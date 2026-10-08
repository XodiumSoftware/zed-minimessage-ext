# Zed MiniMessage Extension task runner
# Install `just` once: https://github.com/casey/just

_default:
    @just --list

# Lint with pedantic lints enabled and warnings as errors.
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

# Build the extension for Zed (WASM target)
build:
    cargo build --release --target wasm32-wasip1

# Build for debugging (native target for easier testing)
build-dev:
    cargo build

# Clean build artifacts
clean:
    cargo clean

# Install the WASM target if not already installed
install-target:
    rustup target add wasm32-wasip1

# Package the extension for distribution (creates .tar.gz)
package: build
    # TODO: Add packaging logic to create a distributable extension
    @echo "Packaging not yet implemented"

# Run the full validation suite used in CI
validate: lint fmt-check test build

# Development: watch for changes and rebuild
watch:
    cargo watch -x "build" -x "test"
