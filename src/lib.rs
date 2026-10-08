//! Zed extension for `MiniMessage` validation and hover preview.
//!
//! This extension provides:
//! - Syntax validation for `MiniMessage` strings (unclosed tags, invalid colors)
//! - Hover previews showing parsed `MiniMessage` structure
//! - Diagnostics with quick-fix suggestions

use zed_extension_api as zed;

pub mod hover;
pub mod parser;
pub mod validator;

/// The `MiniMessage` extension for Zed.
struct MiniMessageExtension;

impl zed::Extension for MiniMessageExtension {
    fn new() -> Self {
        Self
    }

    /// Provide the language server command for `MiniMessage` files.
    ///
    /// For MVP, we handle diagnostics and hover directly in the extension
    /// rather than spawning an external LSP server.
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // TODO: For MVP, implement validation/hover directly in extension.
        // For advanced features, spawn an LSP server here.
        Err("No language server command configured (in-extension mode)".to_string())
    }

    /// Provide language server initialization options.
    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>, String> {
        Ok(None)
    }
}

// Register the extension with Zed
zed::register_extension!(MiniMessageExtension);
