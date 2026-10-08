//! `MiniMessage` hover provider for showing parsed structure previews.
//!
//! This module generates hover previews for `MiniMessage` strings, showing the
//! parsed structure, colors, and formatting information.

use crate::parser::{MiniMessageParser, Token};
use std::fmt::Write;

/// A hover provider for `MiniMessage` strings.
pub struct MiniMessageHover {
    /// The parser used to parse `MiniMessage` syntax
    parser: MiniMessageParser,
}

impl MiniMessageHover {
    /// Create a new `MiniMessage` hover provider.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: MiniMessageParser::new(),
        }
    }

    /// Generate a hover preview for a `MiniMessage` string.
    ///
    /// This function parses the input string and generates a formatted preview
    /// showing the structure and colors of the `MiniMessage`.
    #[must_use]
    pub fn hover(&self, input: &str) -> String {
        match self.parser.parse(input) {
            Ok(tokens) => Self::generate_preview(&tokens),
            Err(errors) => Self::generate_error_preview(errors),
        }
    }

    /// Generate a preview from parsed tokens.
    ///
    /// This is an associated function since it doesn't use `self`.
    fn generate_preview(tokens: &[Token]) -> String {
        let mut preview = String::from("**MiniMessage Preview**\n\n");

        // Show the parsed tokens
        preview.push_str("### Parsed Structure\n\n");
        preview.push_str("```\n");
        for token in tokens {
            match token {
                Token::Text(text) => {
                    writeln!(preview, "Text: \"{text}\"")
                        .expect("Writing to string should not fail");
                }
                Token::OpenTag(tag) => {
                    writeln!(preview, "Open Tag: <{tag}>")
                        .expect("Writing to string should not fail");
                }
                Token::CloseTag(tag) => {
                    writeln!(preview, "Close Tag: </{tag}>")
                        .expect("Writing to string should not fail");
                }
                Token::SelfClosingTag(tag) => {
                    writeln!(preview, "Self-closing Tag: <{tag} />")
                        .expect("Writing to string should not fail");
                }
                Token::HexColor(color) => {
                    writeln!(preview, "Hex Color: {color}")
                        .expect("Writing to string should not fail");
                }
                Token::Gradient(colors) => {
                    writeln!(preview, "Gradient: [{}]", colors.join(", "))
                        .expect("Writing to string should not fail");
                }
            }
        }
        preview.push_str("```\n\n");

        // Show color information
        preview.push_str("### Color Information\n\n");
        for token in tokens {
            match token {
                Token::OpenTag(tag) => {
                    if let Some(color) = MiniMessageParser::new().get_color_value(tag) {
                        writeln!(preview, "- `<{tag}>` = `{color}`")
                            .expect("Writing to string should not fail");
                    }
                }
                Token::HexColor(color) => {
                    writeln!(preview, "- `{color}` (custom hex color)")
                        .expect("Writing to string should not fail");
                }
                Token::Gradient(colors) => {
                    preview.push_str("- Gradient colors: ");
                    let mut first = true;
                    for color in colors {
                        if !first {
                            preview.push_str(", ");
                        }
                        if let Some(value) = MiniMessageParser::new().get_color_value(color) {
                            write!(preview, "`{color}` → `{value}`")
                                .expect("Writing to string should not fail");
                        } else {
                            write!(preview, "`{color}`")
                                .expect("Writing to string should not fail");
                        }
                        first = false;
                    }
                    preview.push('\n');
                }
                _ => {}
            }
        }

        // Show the reconstructed message
        preview.push_str("\n### Reconstructed Message\n\n");
        preview.push_str("```\n");
        preview.push_str(&Self::reconstruct_message(tokens));
        preview.push_str("\n```");

        preview
    }

    /// Generate an error preview from validation errors.
    ///
    /// This is an associated function since it doesn't use `self`.
    fn generate_error_preview(errors: Vec<crate::parser::ValidationError>) -> String {
        let mut preview = String::from("**Invalid MiniMessage**\n\nErrors found:\n");
        for error in errors {
            writeln!(
                preview,
                "- {} at position {}-{}",
                error.message,
                error.position,
                error.position + error.length
            )
            .expect("Writing to string should not fail");
        }
        preview
    }

    /// Reconstruct the message from tokens (without formatting).
    ///
    /// This is an associated function since it doesn't use `self`.
    fn reconstruct_message(tokens: &[Token]) -> String {
        let mut result = String::new();
        for token in tokens {
            match token {
                Token::Text(text) => result.push_str(text),
                Token::OpenTag(_)
                | Token::CloseTag(_)
                | Token::SelfClosingTag(_)
                | Token::HexColor(_)
                | Token::Gradient(_) => {
                    // Skip tags in reconstruction
                }
            }
        }
        result
    }

    /// Generate hover preview for a `MiniMessage` string at a specific position in a Rust file.
    ///
    /// This function finds the `MiniMessage` string at the given position and generates
    /// a hover preview for it.
    ///
    /// # Panics
    ///
    /// This function may panic if it encounters invalid UTF-8 sequences in the source text.
    /// It assumes the input is valid UTF-8.
    #[must_use]
    pub fn hover_rust_file(&self, source: &str, position: usize) -> Option<String> {
        // Find string literals that might contain MiniMessage
        let mut string_start = 0;
        while string_start < source.len() {
            // Look for string literals
            if let Some(start) = source[string_start..].find('"') {
                let string_start_abs = string_start + start;

                // Find the end of the string
                let mut end = string_start_abs + 1;
                let mut in_escape = false;

                while end < source.len() {
                    let c = source.chars().nth(end).unwrap();

                    if c == '\\' && !in_escape {
                        in_escape = true;
                    } else if c == '"' && !in_escape {
                        break;
                    } else {
                        in_escape = false;
                    }

                    end += 1;
                }

                if end >= source.len() {
                    break; // Unterminated string
                }

                // Check if position is within this string
                if position > string_start_abs && position < end {
                    // Extract the string content
                    let string_content = &source[string_start_abs + 1..end];

                    // Check if it looks like a MiniMessage string
                    if string_content.contains('<') && string_content.contains('>') {
                        return Some(self.hover(string_content));
                    }
                }

                string_start = end + 1;
            } else {
                break; // No more strings
            }
        }

        None
    }

    /// Generate hover preview for a `MiniMessage` string at a specific position in a TOML file.
    ///
    /// This function looks for `MiniMessage` strings in TOML values at the given position
    /// and generates a hover preview.
    ///
    /// # Panics
    ///
    /// This function may panic if it encounters invalid UTF-8 sequences in the source text.
    /// It assumes the input is valid UTF-8.
    #[must_use]
    pub fn hover_toml_file(&self, source: &str, position: usize) -> Option<String> {
        // Simple implementation for MVP - look for specific TOML keys
        let minimessage_keys = [
            "chat.format",
            "header",
            "footer",
            "join.message",
            "leave.message",
            "kick.message",
        ];

        for key in minimessage_keys {
            if let Some(key_pos) = source.find(key) {
                // Look for the value after the key
                if let Some(equal_pos) = source[key_pos..].find('=') {
                    let equal_pos = key_pos + equal_pos;

                    // Find the value (could be string, multiline string, etc.)
                    let value_start = equal_pos + 1;

                    // Skip whitespace
                    let mut value_pos = value_start;
                    while value_pos < source.len()
                        && (source.chars().nth(value_pos).unwrap() == ' '
                            || source.chars().nth(value_pos).unwrap() == '\t')
                    {
                        value_pos += 1;
                    }

                    // Check if value is a string
                    if value_pos < source.len() && source.chars().nth(value_pos).unwrap() == '"' {
                        // Find the end of the string
                        let mut end = value_pos + 1;
                        let mut in_escape = false;

                        while end < source.len() {
                            let c = source.chars().nth(end).unwrap();

                            if c == '\\' && !in_escape {
                                in_escape = true;
                            } else if c == '"' && !in_escape {
                                break;
                            } else {
                                in_escape = false;
                            }

                            end += 1;
                        }

                        if end < source.len() {
                            // Check if position is within this string
                            if position > value_pos && position < end {
                                // Extract the string content
                                let string_content = &source[value_pos + 1..end];

                                // Check if it looks like a MiniMessage string
                                if string_content.contains('<') && string_content.contains('>') {
                                    return Some(self.hover(string_content));
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl Default for MiniMessageHover {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_valid_minimessage() {
        let hover = MiniMessageHover::new();
        let input = "<red>Hello <bold>World</bold>!</red>";
        let preview = hover.hover(input);

        // Check that the preview contains the parsed structure
        assert!(preview.contains("Open Tag: <red>"));
        assert!(preview.contains("Text: \"Hello \""));
        assert!(preview.contains("Open Tag: <bold>"));
        assert!(preview.contains("Text: \"World\""));
        assert!(preview.contains("Close Tag: </bold>"));
        assert!(preview.contains("Text: \"!\""));
        assert!(preview.contains("Close Tag: </red>"));

        // Check that color information is included
        assert!(preview.contains("`<red>` = `#FF5555`"));
        // Note: <bold> is a style tag, not a color, so no color value is shown
    }

    #[test]
    fn test_hover_invalid_minimessage() {
        let hover = MiniMessageHover::new();
        let input = "<red>Hello <bold>World</red></bold>";
        let preview = hover.hover(input);

        // Check that errors are shown
        assert!(preview.contains("**Invalid MiniMessage**"));
        assert!(preview.contains("Mismatched tags"));
        assert!(preview.contains("<red>"));
        assert!(preview.contains("</bold>"));
    }

    #[test]
    fn test_hover_rust_file() {
        let hover = MiniMessageHover::new();
        let rust_code = r#"
            fn main() {
                let message = Component::text("<red>Hello <bold>World</bold>!</red>");
                println!("{}", message);
            }
        "#;

        // Position inside the MiniMessage string
        let position = rust_code.find("<red>").unwrap() + 5;
        let preview = hover.hover_rust_file(rust_code, position);

        assert!(preview.is_some());
        let preview = preview.unwrap();
        assert!(preview.contains("Open Tag: <red>"));
        assert!(preview.contains("Text: \"Hello \""));
    }

    #[test]
    fn test_hover_toml_file() {
        let hover = MiniMessageHover::new();
        let toml_code = r#"
            chat.format = "<red><bold>Player</bold></red>: <italic>Message</italic>"
            header = "<gold>Welcome to the server!</gold>"
        "#;

        // Position inside the chat.format MiniMessage string
        let position = toml_code.find("<red><bold>").unwrap() + 10;
        let preview = hover.hover_toml_file(toml_code, position);

        assert!(preview.is_some());
        let preview = preview.unwrap();
        assert!(preview.contains("Open Tag: <red>"));
        assert!(preview.contains("Open Tag: <bold>"));
    }
}
