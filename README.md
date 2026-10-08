<div id="readme-top"></div>

<h1 align="center">
  <br />
    <a href="https://github.com/XodiumSoftware/zed-minimessage-ext">
        <img src="https://raw.githubusercontent.com/XodiumSoftware/zed-minimessage-ext/main/logo.svg" alt="Zed MiniMessage Extension Logo" width="200">
    </a>
  <br /><br />
  Zed MiniMessage Extension
  <br />
  <br />
</h1>

<h4 align="center">MiniMessage validation and hover preview for Zed</h4><br />

<div align="center">

[![Contributors][contributors_shield_url]][contributors_url]
[![Issues][issues_shield_url]][issues_url]
[![Deps][deps_shield_url]][deps_url]

</div>

## About

**Zed MiniMessage Extension** provides real-time validation and preview for MiniMessage strings in Zed. Catch broken tags, unclosed gradients, and invalid colors before sending them to your Minecraft server.

### Features

- **Syntax Validation**: Detects unclosed tags, invalid color names, and malformed gradients
- **Hover Preview**: See parsed MiniMessage structure and formatting info on hover
- **Diagnostics**: Inline warnings and errors with quick-fix suggestions
- **Auto-completion**: Intelligent suggestions for tags and colors (planned)

### Supported File Types

- `.mm`, `.minimessage`, `.mcmsg` — Dedicated MiniMessage files
- `.rs` — Rust code containing MiniMessage strings (via string literal detection)
- `.toml`, `.json` — Config files with MiniMessage values (planned)

## Quick Start

### Installation

1. **Install the extension** in Zed:
    - Open Zed → `Extensions` → `Install Extension`
    - Search for "MiniMessage" or install from repository

2. **Enable for your project**:
    - The extension automatically activates for supported file types

### Usage

1. **Write MiniMessage** in any supported file:

    ```rust
    Component::text("<red>Hello <bold>World</bold>!</red>")
    ```

2. **See validation in real-time**:
    - Unclosed tags get red squiggles
    - Invalid colors show warnings
    - Hover to see parsed structure

3. **Preview formatted text**:
    - Hover over a MiniMessage string to see the rendered preview
    - Click diagnostics for quick-fix suggestions

## Built With

<div align="center">

[![Built With][built_with_shield_url]][built_with_url]

</div>

- **Rust** — Core extension logic (compiled to WASM)
- **Zed Extension API** — Integration with Zed editor
- **MiniMessage** — Inspired by the [Adventure](https://docs.adventure.kyori.net/minimessage/) library

## Roadmap

- [ ] **MVP**: Basic validation and hover preview
- [ ] **v0.2**: Auto-completion for tags and colors
- [ ] **v0.3**: Rich text hover preview (colors in tooltip)
- [ ] **v0.4**: Support for config files (TOML/JSON)
- [ ] **v0.5**: Quick-fix actions (auto-close tags, fix colors)
- [ ] **v1.0**: Full MiniMessage spec support, performance optimizations

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Install dependencies
just install-target

# Build the extension
just build

# Run tests
just test

# Lint and validate
just validate
```

## License

This project is licensed under the MIT OR Apache-2.0 license - see [LICENSE.md](LICENSE.md) for details.

## Security

For security concerns, please see [SECURITY.md](SECURITY.md).

<p align="right"><a href="#readme-top">▲</a></p>

[built_with_shield_url]: https://skillicons.dev/icons?i=rust,github,githubactions
[built_with_url]: https://skillicons.dev
[code_of_conduct_url]: https://github.com/XodiumSoftware/zed-minimessage-ext?tab=coc-ov-file
[contributing_url]: https://github.com/XodiumSoftware/zed-minimessage-ext?tab=contributing-ov-file
[contributors_shield_url]: https://img.shields.io/github/contributors/XodiumSoftware/zed-minimessage-ext?style=for-the-badge&color=blue
[contributors_url]: https://github.com/XodiumSoftware/zed-minimessage-ext/graphs/contributors
[issues_shield_url]: https://img.shields.io/github/issues/XodiumSoftware/zed-minimessage-ext?style=for-the-badge&color=yellow
[issues_url]: https://github.com/XodiumSoftware/zed-minimessage-ext/issues
[deps_shield_url]: https://deps.rs/repo/github/XodiumSoftware/zed-minimessage-ext/status.svg?style=for-the-badge
[deps_url]: https://deps.rs/repo/github/XodiumSoftware/zed-minimessage-ext
[license_url]: https://github.com/XodiumSoftware/zed-minimessage-ext?tab=AGPL-3.0-1-ov-file
[roadmap_shield_url]: https://img.shields.io/badge/Roadmap-Click%20Me!-purple.svg?style=for-the-badge
[roadmap_url]: https://github.com/orgs/XodiumSoftware/projects/zed-minimessage-ext
[security_url]: https://github.com/XodiumSoftware/zed-minimessage-ext?tab=security-ov-file
