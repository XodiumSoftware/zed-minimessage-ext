//! Zed extension for `MiniMessage` validation and hover preview.
//!
//! This extension provides:
//! - Syntax validation for `MiniMessage` strings (unclosed tags, invalid colors)
//! - Hover previews showing parsed `MiniMessage` structure
//! - Diagnostics with quick-fix suggestions

use zed_extension_api as zed;

pub mod hover;
pub mod language_server;
pub mod parser;
pub mod validator;

/// The `MiniMessage` extension for Zed.
struct MiniMessageExtension;

impl zed::Extension for MiniMessageExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Ok(zed::Command {
            command: "minimessage-lsp".to_string(),
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>, String> {
        Ok(Some(zed::serde_json::json!({
            "name": "MiniMessage Language Server",
            "version": "0.1.0",
            "capabilities": {
                "hoverProvider": true,
                "diagnosticProvider": true,
                "completionProvider": false,
            }
        })))
    }
}

// Register the extension with Zed
zed::register_extension!(MiniMessageExtension);
