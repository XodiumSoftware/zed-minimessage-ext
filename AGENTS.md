# zed-minimessage-ext — Claude Code Context

## Project at a Glance

- **Name:** zed-minimessage-ext
- **Type:** Zed extension (Rust/WASM)
- **Language:** Rust (Edition 2024)
- **Target:** `wasm32-wasip1` (Zed extensions run as WASM components)

## Quick Commands

Build tasks run through [just](https://github.com/casey/just) (cross-platform; see the `justfile` for all recipes).

```bash
# Lint with pedantic lints (warnings as errors)
just lint

# Build the extension (release WASM)
just build

# Build for development (native, faster)
just build-dev

# Run tests
just test

# Full validation (lint + fmt + test + build)
just validate

# Install required WASM target
just install-target
```

## Architecture Overview

### Entry Point

**`MiniMessageExtension`** — implements `zed::Extension` from `zed_extension_api`:

1. **Registration**: Via `zed::register_extension!(MiniMessageExtension)` macro
2. **`new()`**: Initializes the extension (called when Zed loads the extension)
3. **`language_server_command()`**: Returns command to spawn LSP server (currently `None` — we handle logic in-extension)
4. **`language_server_initialization_options()`**: Returns options for LSP (currently `None`)

### How It Works

Unlike traditional language servers, this extension implements validation and hover **directly in the extension code** (no external LSP process). This is simpler and sufficient for MiniMessage's lightweight syntax.

**Key Components:**

| Component          | File                      | Purpose                                        |
| ------------------ | ------------------------- | ---------------------------------------------- |
| Extension Entry    | `src/lib.rs`              | Main extension struct and registration         |
| MiniMessage Parser | `src/parser.rs` (TODO)    | Parses MiniMessage strings into tokens         |
| Validator          | `src/validator.rs` (TODO) | Validates parsed tokens, generates diagnostics |
| Hover Provider     | `src/hover.rs` (TODO)     | Generates hover previews with formatting info  |
| Extension Manifest | `extension.toml`          | Zed extension configuration                    |

### Project Structure

```
src/
├── lib.rs            # Extension entry point, MiniMessageExtension struct
└── ...               # Add parser.rs, validator.rs, hover.rs as needed

extension.toml        # Zed extension manifest (required)
justfile              # Build tasks
Cargo.toml            # Rust project configuration
```

### Key Conventions

- **Register modules and re-exports in `src/lib.rs` explicitly.** Do not use `mod.rs` files, and do not nest `mod` declarations inside other module files. Use `#[path = "..."]` attributes when a module file lives in a subdirectory.
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors.
- `unsafe_code` is forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`).
- Add `rustdoc` comments when you introduce public APIs, change interfaces, or add complex logic.
- **Zed extensions use `zed_extension_api`** for all Zed interactions (events, diagnostics, hover, etc.).

### MiniMessage Syntax Notes

**Valid tags:**

- Colors: `<red>`, `<green>`, `<blue>`, `<yellow>`, `<aqua>`, `<white>`, `<black>`, `<gray>`, `<dark_red>`, `<dark_green>`, `<dark_blue>`, `<dark_yellow>`, `<dark_aqua>`, `<dark_gray>`, `<light_purple>`, `<dark_purple>`, `<gold>`, `<reset>`
- Styles: `<bold>`, `<italic>`, `<underline>`, `<strikethrough>`, `<obfuscated>`
- Events: `<click:action:value>`, `<hover:action:value>`, `<insertion:value>`, `<keybind:key>`, `<translate:key>`, `<font:font>`, `<lang:key>`, `<selector:selector>`, `<score:name:objective>`, `<nbt:...>`
- Gradients: `<gradient:#FF0000,#00FF00>text</gradient>` or `<gradient:color1,color2>text</gradient>`
- Transitions: `<transition:#FF0000>text</transition>`
- Rainbow: `<rainbow>text</rainbow>`

**Common errors to detect:**

- Unclosed tags: `<red>hello` (missing `</red>`)
- Invalid colors: `<notacolor>hello</notacolor>`
- Mismatched tags: `<red><bold>hello</red></bold>`
- Invalid gradient syntax: `<gradient:FF0000>hello</gradient>` (missing `#` or comma)

## Testing

- Unit tests in `src/lib.rs` under `#[cfg(test)]` and in separate `src/*_tests.rs` files
- Run with: `cargo test`
- Integration testing: Build WASM and load in Zed via extension manifest
- For WASM testing, use `wasm-pack test` or `wasm-bindgen-test` (TODO: add)

## CI/CD

GitHub Actions workflows in `.github/workflows/`:

- **rust.yml** — Lints, tests, and builds the project on every push and pull request
- **enforce_pr_title.yml** — Validates PR titles follow conventional commits

## Adding a New Feature

To add a new feature (e.g., hover preview, validation, completion):

1. Create new module in `src/` (e.g., `src/hover.rs`)
2. Add module-level docs `//!` with description and examples
3. Implement the feature using `zed_extension_api` types
4. Register the module in `src/lib.rs` with `pub mod hover;`
5. Update `extension.toml` if adding new language support or commands
6. Run `just validate` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
