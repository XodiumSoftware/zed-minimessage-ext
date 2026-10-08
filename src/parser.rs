//! `MiniMessage` parser for tokenizing and validating `MiniMessage` strings.
//!
//! This module provides a lightweight parser for `MiniMessage` syntax, which is used
//! in Minecraft plugins and mods for text formatting. The parser can identify
//! common errors like unclosed tags, invalid colors, and malformed gradients.

use std::collections::HashMap;

/// Represents a `MiniMessage` token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Plain text content
    Text(String),
    /// Opening tag (e.g., `<red>`, `<bold>`)
    OpenTag(String),
    /// Closing tag (e.g., `</red>`, `</bold>`)
    CloseTag(String),
    /// Self-closing tag (e.g., `<br>`, `<newline>`)
    SelfClosingTag(String),
    /// Color tag with hex value (e.g., `<#FF0000>`)
    HexColor(String),
    /// Gradient tag (e.g., `<gradient:#FF0000,#00FF00>`)
    Gradient(Vec<String>),
}

/// Represents a validation error in `MiniMessage` syntax.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The error message
    pub message: String,
    /// The position in the input string (byte offset)
    pub position: usize,
    /// The length of the problematic section
    pub length: usize,
    /// Severity of the error
    pub severity: ErrorSeverity,
}

/// Severity levels for validation errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// Critical error that breaks parsing
    Error,
    /// Warning that may cause issues
    Warning,
    /// Informational hint
    Info,
}

/// A lightweight `MiniMessage` parser for validation and tokenization.
pub struct MiniMessageParser {
    /// Color names
    colors: HashMap<&'static str, &'static str>,
    /// Style tags
    styles: Vec<&'static str>,
    /// Event tags
    events: Vec<&'static str>,
}

impl MiniMessageParser {
    /// Create a new `MiniMessage` parser with default settings.
    #[must_use]
    pub fn new() -> Self {
        let mut colors = HashMap::new();
        // Standard Minecraft colors
        colors.insert("black", "#000000");
        colors.insert("dark_blue", "#0000AA");
        colors.insert("dark_green", "#00AA00");
        colors.insert("dark_aqua", "#00AAAA");
        colors.insert("dark_red", "#AA0000");
        colors.insert("dark_purple", "#AA00AA");
        colors.insert("gold", "#FFAA00");
        colors.insert("gray", "#AAAAAA");
        colors.insert("dark_gray", "#555555");
        colors.insert("blue", "#5555FF");
        colors.insert("green", "#55FF55");
        colors.insert("aqua", "#55FFFF");
        colors.insert("red", "#FF5555");
        colors.insert("light_purple", "#FF55FF");
        colors.insert("yellow", "#FFFF55");
        colors.insert("white", "#FFFFFF");
        colors.insert("reset", "reset");

        let styles = vec![
            "bold",
            "b",
            "italic",
            "i",
            "underline",
            "u",
            "strikethrough",
            "s",
            "obfuscated",
            "o",
        ];

        let events = vec![
            "click",
            "hover",
            "insertion",
            "keybind",
            "translate",
            "font",
            "lang",
            "selector",
            "score",
            "nbt",
        ];

        Self {
            colors,
            styles,
            events,
        }
    }

    /// Parse a `MiniMessage` string into tokens.
    ///
    /// # Errors
    ///
    /// Returns a vector of validation errors if the input contains invalid `MiniMessage` syntax.
    /// If the input is valid, returns `Ok` with a vector of tokens.
    pub fn parse(&self, input: &str) -> Result<Vec<Token>, Vec<ValidationError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut position = 0;
        let input_len = input.len();

        while position < input_len {
            if input[position..].starts_with('<') {
                // Parse a tag
                match self.parse_tag(input, position) {
                    Ok((token, new_position)) => {
                        tokens.push(token);
                        position = new_position;
                    }
                    Err((error, new_position)) => {
                        errors.push(error);
                        position = new_position;
                    }
                }
            } else {
                // Parse text until next tag
                let text_end = input[position..]
                    .find('<')
                    .map_or(input_len, |pos| position + pos);

                let text = &input[position..text_end];
                if !text.is_empty() {
                    tokens.push(Token::Text(text.to_string()));
                }
                position = text_end;
            }
        }

        // Validate token consistency (e.g., matching open/close tags)
        Self::validate_tokens(&tokens, &mut errors);

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    /// Parse a single tag starting at the given position.
    ///
    /// This is a helper function for `parse` and handles the different tag types.
    #[allow(clippy::too_many_lines)]
    fn parse_tag(
        &self,
        input: &str,
        start: usize,
    ) -> Result<(Token, usize), (ValidationError, usize)> {
        let input_len = input.len();

        // Find the closing '>'
        let tag_end = input[start..]
            .find('>')
            .map(|pos| start + pos)
            .ok_or_else(|| {
                (
                    ValidationError {
                        message: "Unclosed tag (missing '>')".to_string(),
                        position: start,
                        length: 1,
                        severity: ErrorSeverity::Error,
                    },
                    start + 1,
                )
            })?;

        if tag_end >= input_len {
            return Err((
                ValidationError {
                    message: "Unclosed tag (missing '>')".to_string(),
                    position: start,
                    length: 1,
                    severity: ErrorSeverity::Error,
                },
                start + 1,
            ));
        }

        let tag_content = &input[start + 1..tag_end];

        // Check if it's a closing tag
        if let Some(tag_name) = tag_content.strip_prefix('/') {
            // Check if it's a hex color closing tag or valid tag name
            if Self::is_hex_color(tag_name) || self.is_valid_tag(tag_name) {
                Ok((Token::CloseTag(tag_name.to_string()), tag_end + 1))
            } else {
                Err((
                    ValidationError {
                        message: format!("Invalid closing tag '</{tag_name}>'"),
                        position: start,
                        length: tag_end - start + 1,
                        severity: ErrorSeverity::Error,
                    },
                    tag_end + 1,
                ))
            }
        }
        // Check if it's a self-closing tag
        else if let Some(tag_name) = tag_content.strip_suffix('/') {
            if self.is_valid_tag(tag_name) || Self::is_hex_color(tag_name) {
                Ok((Token::SelfClosingTag(tag_name.to_string()), tag_end + 1))
            } else {
                Err((
                    ValidationError {
                        message: format!("Invalid self-closing tag '<{tag_name}/>'"),
                        position: start,
                        length: tag_end - start + 1,
                        severity: ErrorSeverity::Error,
                    },
                    tag_end + 1,
                ))
            }
        }
        // Check if it's a gradient tag (opening or closing)
        else if let Some(gradient_part) = tag_content.strip_prefix("gradient:") {
            let colors: Vec<String> = gradient_part
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            // Validate gradient colors
            let mut invalid_colors = Vec::new();

            for color in &colors {
                if !self.is_valid_color(color) && !Self::is_hex_color(color) {
                    invalid_colors.push(color.clone());
                }
            }

            if invalid_colors.is_empty() {
                Ok((Token::Gradient(colors), tag_end + 1))
            } else {
                Err((
                    ValidationError {
                        message: format!("Invalid gradient colors: {}", invalid_colors.join(", ")),
                        position: start,
                        length: tag_end - start + 1,
                        severity: ErrorSeverity::Error,
                    },
                    tag_end + 1,
                ))
            }
        }
        // Check if it's a gradient closing tag
        else if tag_content == "/gradient" {
            Ok((Token::CloseTag("gradient".to_string()), tag_end + 1))
        }
        // Check if it's a hex color tag
        else if Self::is_hex_color(tag_content) {
            Ok((Token::HexColor(tag_content.to_string()), tag_end + 1))
        }
        // Otherwise, it's a regular tag
        else if self.is_valid_tag(tag_content) {
            Ok((Token::OpenTag(tag_content.to_string()), tag_end + 1))
        } else {
            Err((
                ValidationError {
                    message: format!("Invalid tag '<{tag_content}>'"),
                    position: start,
                    length: tag_end - start + 1,
                    severity: ErrorSeverity::Error,
                },
                tag_end + 1,
            ))
        }
    }

    /// Validate token consistency (e.g., matching open/close tags).
    ///
    /// This is an associated function since it doesn't use `self`.
    fn validate_tokens(tokens: &[Token], errors: &mut Vec<ValidationError>) {
        let mut tag_stack: Vec<(String, usize)> = Vec::new(); // (tag_name, position)

        for (index, token) in tokens.iter().enumerate() {
            match token {
                Token::OpenTag(tag_name) => {
                    tag_stack.push((tag_name.clone(), index));
                }
                Token::Gradient(_) => {
                    // Push "gradient" to the stack for matching with closing tag
                    tag_stack.push(("gradient".to_string(), index));
                }
                Token::HexColor(color) => {
                    // Push the hex color to the stack for matching with closing tag
                    tag_stack.push((color.clone(), index));
                }
                Token::CloseTag(tag_name) => {
                    // Check if this closing tag matches the most recent opening tag
                    if let Some((open_tag, open_index)) = tag_stack.pop() {
                        // Special handling for gradients and hex colors
                        if open_tag == "gradient" && *tag_name == "gradient" {
                            // Gradients match by type
                        } else if Self::is_hex_color(&open_tag) && Self::is_hex_color(tag_name) {
                            // Hex colors match if they're the same
                            if open_tag != *tag_name {
                                errors.push(ValidationError {
                                    message: format!(
                                        "Mismatched hex colors: opened '<{open_tag}>' but closed '</{tag_name}>'"
                                    ),
                                    position: open_index,
                                    length: 1,
                                    severity: ErrorSeverity::Error,
                                });
                            }
                        } else if open_tag != *tag_name {
                            errors.push(ValidationError {
                                message: format!(
                                    "Mismatched tags: opened '<{open_tag}>' but closed '</{tag_name}>'"
                                ),
                                position: open_index,
                                length: 1,
                                severity: ErrorSeverity::Error,
                            });
                        }
                    } else {
                        // Closing tag without matching opening tag
                        errors.push(ValidationError {
                            message: format!(
                                "Closing tag '</{tag_name}>' without matching opening tag"
                            ),
                            position: index,
                            length: 1,
                            severity: ErrorSeverity::Error,
                        });
                    }
                }
                Token::Text(_) | Token::SelfClosingTag(_) => {
                    // These don't affect tag stack
                }
            }
        }

        // Any remaining tags in the stack are unclosed
        for (tag_name, position) in tag_stack {
            errors.push(ValidationError {
                message: format!("Unclosed tag '<{tag_name}>'"),
                position,
                length: 1,
                severity: ErrorSeverity::Error,
            });
        }
    }

    /// Check if a tag name is valid.
    fn is_valid_tag(&self, tag: &str) -> bool {
        // Check if it's a color
        if self.colors.contains_key(tag) {
            return true;
        }

        // Check if it's a style
        if self.styles.contains(&tag) {
            return true;
        }

        // Check if it's an event tag (may have parameters)
        if tag.contains(':') {
            let event_type = tag.split(':').next().unwrap_or("");
            return self.events.contains(&event_type);
        }

        // Check if it's a special tag
        matches!(
            tag,
            "br" | "newline"
                | "reset"
                | "bold"
                | "italic"
                | "underline"
                | "strikethrough"
                | "obfuscated"
                | "gradient"
        )
    }

    /// Check if a string is a valid color name.
    fn is_valid_color(&self, color: &str) -> bool {
        self.colors.contains_key(color)
    }

    /// Check if a string is a valid hex color.
    ///
    /// Hex colors start with `#` followed by 3 or 6 hexadecimal digits.
    /// This is an associated function since it doesn't use `self`.
    fn is_hex_color(s: &str) -> bool {
        if let Some(hex_part) = s.strip_prefix('#')
            && (hex_part.len() == 3 || hex_part.len() == 6)
        {
            return hex_part.chars().all(|c| c.is_ascii_hexdigit());
        }
        false
    }

    /// Get the color value for a color name.
    ///
    /// Returns the hex value for a named color, or `None` if the color name is invalid.
    #[must_use]
    pub fn get_color_value(&self, color_name: &str) -> Option<&'static str> {
        self.colors.get(color_name).copied()
    }
}

impl Default for MiniMessageParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_minimessage() {
        let parser = MiniMessageParser::new();
        let input = "<red>Hello <bold>World</bold>!</red>";
        let result = parser.parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unclosed_tag() {
        let parser = MiniMessageParser::new();
        let input = "<red>Hello World";
        let result = parser.parse(input);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Unclosed tag"));
    }

    #[test]
    fn test_invalid_color() {
        let parser = MiniMessageParser::new();
        let input = "<notacolor>Hello</notacolor>";
        let result = parser.parse(input);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        // Should have 2 errors: one for opening tag, one for closing tag
        assert_eq!(errors.len(), 2);
        assert!(errors[0].message.contains("Invalid tag"));
        assert!(errors[1].message.contains("Invalid closing tag"));
    }

    #[test]
    fn test_mismatched_tags() {
        let parser = MiniMessageParser::new();
        let input = "<red><bold>Hello</red></bold>";
        let result = parser.parse(input);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        // Should have 2 errors: both are mismatched tags
        assert_eq!(errors.len(), 2);
        assert!(errors[0].message.contains("Mismatched tags"));
        assert!(errors[1].message.contains("Mismatched tags"));
    }

    #[test]
    fn test_hex_color() {
        let parser = MiniMessageParser::new();
        let input = "<#FF0000>Red Text</#FF0000>";
        let result = parser.parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gradient() {
        let parser = MiniMessageParser::new();
        let input = "<gradient:#FF0000,#00FF00>Gradient Text</gradient>";
        let result = parser.parse(input);

        // Print errors for debugging
        if let Err(errors) = &result {
            for error in errors {
                println!("Error: {} at position {}", error.message, error.position);
            }
        }

        assert!(result.is_ok());
    }
}
