//! `MiniMessage` language server implementation for Zed.
//!
//! This module implements the language server protocol for `MiniMessage` validation
//! and hover previews in Zed.

use std::fmt::Write;

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

impl Default for MiniMessageLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}
