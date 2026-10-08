# <Project Name> — Claude Code Context

## Project at a Glance

- **Name:** <project_name>
- **Type:** Rust project
- **Language:** Rust (Edition 2024)

> This file & project was generated from a template. Replace all placeholders and update every section to match the actual project.

## Quick Commands

```bash
# Run tests
cargo test

# Build for release
cargo build --release

# Generate Rust documentation
cargo doc --no-deps

# Clean build artifacts
cargo clean

# Lint with pedantic lints enabled; every warning is treated as an error.
cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings
```

## Architecture Overview

### Entry Points

| File      | Purpose                        |
|-----------|--------------------------------|
| `main.rs` | Application entry point        |
| `lib.rs`  | Crate root; declare modules here |

### Project Structure

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Crate root, module declarations
└── ...               # Add modules as the project grows
```

### Key Conventions

- **Register modules and re-exports in `src/lib.rs` explicitly.** Do not use `mod.rs` files, and do not nest `mod` declarations inside other module files. Every module in the crate must be declared directly in the crate root (`src/lib.rs`). Use `#[path = "..."]` attributes when a module file lives in a subdirectory.
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors.
- `unsafe_code` is not needed for most projects. Avoid it unless there is a clear, justified reason.
- Add `rustdoc` comments when you introduce public APIs, change interfaces, or add complex logic.

## Testing

- Run tests with `cargo test`.
- Add unit tests in the same file under `#[cfg(test)] mod tests` when they help isolate logic or prevent regressions.

## Claude Code Workflow

### After Making Edits

**Always update documentation when code changes:**

1. **rustdoc comments** — Add/update if you:
    - Add new public APIs
    - Change component interfaces
    - Add complex logic
    - **Run `cargo build`** or `cargo test` to verify compilation

**Rule of thumb:** If a code change would confuse someone reading the docs, update the docs.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:

- **rust.yml** — Lints, tests, and builds the project on every push and pull request.
- **enforce_pr_title.yml** — Validates PR titles follow conventional commits.

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
