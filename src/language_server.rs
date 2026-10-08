//! `MiniMessage` language server integration for Zed.
//!
//! This module implements the integration between the `MiniMessage` extension
//! and Zed's extension API.

use std::fmt::Write;
use zed_extension_api as zed;

/// The `MiniMessage` language server implementation.
pub struct MiniMessageLanguageServer {
    /// The parser used to validate `MiniMessage` syntax
    parser: crate::parser::MiniMessageParser,
}

impl MiniMessageLanguageServer {
    /// Create a new `MiniMessage` language server.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: crate::parser::MiniMessageParser::new(),
        }
    }

    /// Validate `MiniMessage` content and return diagnostics.
    ///
    /// This function uses the parser to validate the content and returns
    /// a vector of diagnostic messages.
    #[must_use]
    pub fn validate_content(&self, content: &str) -> Vec<String> {
        match self.parser.parse(content) {
            Ok(_) => Vec::new(),
            Err(errors) => errors
                .into_iter()
                .map(|error| {
                    format!(
                        "{} at position {}-{}",
                        error.message,
                        error.position,
                        error.position + error.length
                    )
                })
                .collect(),
        }
    }

    /// Generate hover information for `MiniMessage` content.
    ///
    /// This function uses the parser to generate a hover preview.
    #[must_use]
    pub fn hover_content(&self, content: &str) -> Option<String> {
        match self.parser.parse(content) {
            Ok(tokens) => {
                let mut hover_info = String::from("MiniMessage Structure:\n");
                for token in tokens {
                    match token {
                        crate::parser::Token::Text(text) => {
                            let _ = writeln!(hover_info, "  Text: \"{text}\"");
                        }
                        crate::parser::Token::OpenTag(tag) => {
                            let _ = writeln!(hover_info, "  Open Tag: <{tag}>");
                        }
                        crate::parser::Token::CloseTag(tag) => {
                            let _ = writeln!(hover_info, "  Close Tag: </{tag}>");
                        }
                        crate::parser::Token::SelfClosingTag(tag) => {
                            let _ = writeln!(hover_info, "  Self-closing Tag: <{tag} />");
                        }
                        crate::parser::Token::HexColor(color) => {
                            let _ = writeln!(hover_info, "  Hex Color: {color}");
                        }
                        crate::parser::Token::Gradient(colors) => {
                            let _ = writeln!(hover_info, "  Gradient: {}", colors.join(", "));
                        }
                    }
                }
                Some(hover_info)
            }
            Err(_) => Some("Invalid MiniMessage syntax".to_string()),
        }
    }
}

/// Implementation of the Zed Extension for `MiniMessage`.
pub struct ZedExtension {
    /// The language server instance
    language_server: MiniMessageLanguageServer,
}

impl zed::Extension for ZedExtension {
    fn new() -> Self {
        Self {
            language_server: MiniMessageLanguageServer::new(),
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // For MVP, we're implementing everything in the extension directly
        // rather than spawning an external language server process.
        let _ = &self.language_server; // Prevent unused field warning
        Err("MiniMessage extension handles language server features internally".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>, String> {
        // Use the language server for configuration in a real implementation
        let _ = &self.language_server; // Prevent unused field warning
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>, String> {
        // Use the language server for workspace configuration in a real implementation
        let _ = &self.language_server; // Prevent unused field warning
        Ok(None)
    }
}

impl Default for MiniMessageLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_server_validation() {
        let server = MiniMessageLanguageServer::new();
        let content = "<red>Hello <bold>World</red></bold>";

        let diagnostics = server.validate_content(content);
        assert_ne!(diagnostics, [] as [String; 0]);

        // Check that we get the expected mismatched tags error
        assert!(
            diagnostics
                .iter()
                .any(|msg| msg.contains("Mismatched tags"))
        );
    }

    #[test]
    fn test_language_server_valid_content() {
        let server = MiniMessageLanguageServer::new();
        let content = "<red>Hello <bold>World</bold>!</red>";

        let diagnostics = server.validate_content(content);
        assert_eq!(diagnostics, [] as [String; 0]);
    }

    #[test]
    fn test_language_server_hover() {
        let server = MiniMessageLanguageServer::new();
        let content = "<red>Hello <bold>World</bold>!</red>";

        let hover_info = server.hover_content(content);
        assert!(hover_info.is_some());

        let hover_text = hover_info.unwrap();
        assert!(hover_text.contains("Open Tag: <red>"));
        assert!(hover_text.contains("Text: \"Hello \""));
        assert!(hover_text.contains("Open Tag: <bold>"));
        assert!(hover_text.contains("Text: \"World\""));
    }

    #[test]
    fn test_zed_extension() {
        // Test that the extension implements the Extension trait
        fn assert_extension_trait<T: zed::Extension>(_: &T) {}

        let extension = ZedExtension {
            language_server: MiniMessageLanguageServer::new(),
        };

        assert_extension_trait(&extension);
    }
}
