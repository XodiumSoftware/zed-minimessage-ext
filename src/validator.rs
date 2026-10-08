//! `MiniMessage` validator for generating diagnostics from parser errors.
//!
//! This module converts parser errors into a generic format that can be
//! converted to Zed diagnostics or other editor representations.

use crate::parser::{ErrorSeverity, MiniMessageParser, ValidationError};

/// A generic diagnostic message for `MiniMessage` validation errors.
#[derive(Debug, Clone)]
pub struct MiniMessageDiagnostic {
    /// The range of the error in the source text
    pub start: usize,
    /// The end of the error range
    pub end: usize,
    /// The severity of the error
    pub severity: DiagnosticSeverity,
    /// The error message
    pub message: String,
}

/// Severity levels for diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticSeverity {
    /// Error that breaks parsing
    Error,
    /// Warning that may cause issues
    Warning,
    /// Informational hint
    Info,
}

/// A validator for `MiniMessage` strings.
pub struct MiniMessageValidator {
    /// The parser used to validate `MiniMessage` syntax
    parser: MiniMessageParser,
}

impl MiniMessageValidator {
    /// Create a new `MiniMessage` validator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: MiniMessageParser::new(),
        }
    }

    /// Validate a `MiniMessage` string and return diagnostics.
    ///
    /// This function parses the input string and converts any validation errors
    /// into generic diagnostics that can be used by the editor.
    #[must_use]
    pub fn validate(&self, input: &str) -> Vec<MiniMessageDiagnostic> {
        match self.parser.parse(input) {
            Ok(_) => Vec::new(), // No errors, no diagnostics
            Err(errors) => Self::errors_to_diagnostics(errors),
        }
    }

    /// Convert parser validation errors to generic diagnostics.
    ///
    /// This is an associated function since it doesn't use `self`.
    fn errors_to_diagnostics(errors: Vec<ValidationError>) -> Vec<MiniMessageDiagnostic> {
        errors
            .into_iter()
            .map(|error| {
                let severity = match error.severity {
                    ErrorSeverity::Error => DiagnosticSeverity::Error,
                    ErrorSeverity::Warning => DiagnosticSeverity::Warning,
                    ErrorSeverity::Info => DiagnosticSeverity::Info,
                };

                MiniMessageDiagnostic {
                    start: error.position,
                    end: error.position + error.length,
                    severity,
                    message: error.message,
                }
            })
            .collect()
    }

    /// Validate `MiniMessage` strings in a Rust source file.
    ///
    /// This function detects string literals containing `MiniMessage` in Rust code
    /// and validates them. It's a simple implementation for MVP that looks for
    /// common patterns like `Component::text("<red>Hello</red>")`.
    ///
    /// # Panics
    ///
    /// This function may panic if it encounters invalid UTF-8 sequences in the source text.
    /// It assumes the input is valid UTF-8.
    #[must_use]
    pub fn validate_rust_file(&self, source: &str) -> Vec<(usize, usize, MiniMessageDiagnostic)> {
        let mut diagnostics = Vec::new();
        let mut position = 0;

        // Simple pattern matching for common MiniMessage usage in Rust
        while position < source.len() {
            // Look for string literals that might contain MiniMessage
            if let Some(start) = source[position..].find('"') {
                let string_start = position + start;

                // Find the end of the string
                let mut end = string_start + 1;
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

                // Extract the string content
                let string_content = &source[string_start + 1..end];

                // Check if it looks like a MiniMessage string
                if string_content.contains('<') && string_content.contains('>') {
                    // Validate the MiniMessage string
                    let minimessage_diagnostics = self.validate(string_content);

                    // Adjust positions for the actual string location in the file
                    for diagnostic in minimessage_diagnostics {
                        let adjusted_start = string_start + 1 + diagnostic.start;
                        let adjusted_end = string_start + 1 + diagnostic.end;
                        diagnostics.push((
                            string_start,
                            end,
                            MiniMessageDiagnostic {
                                start: adjusted_start,
                                end: adjusted_end,
                                severity: diagnostic.severity,
                                message: diagnostic.message,
                            },
                        ));
                    }
                }

                position = end + 1;
            } else {
                break; // No more strings
            }
        }

        diagnostics
    }

    /// Validate `MiniMessage` strings in a TOML config file.
    ///
    /// This function looks for `MiniMessage` strings in TOML values, particularly
    /// in fields that commonly contain formatted text like `chat.format`, `header`, `footer`.
    ///
    /// # Panics
    ///
    /// This function may panic if it encounters invalid UTF-8 sequences in the source text.
    /// It assumes the input is valid UTF-8.
    #[must_use]
    pub fn validate_toml_file(&self, source: &str) -> Vec<(usize, usize, MiniMessageDiagnostic)> {
        let mut diagnostics = Vec::new();

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
                            // Extract the string content
                            let string_content = &source[value_pos + 1..end];

                            // Validate the MiniMessage string
                            let minimessage_diagnostics = self.validate(string_content);

                            // Adjust positions for the actual string location in the file
                            for diagnostic in minimessage_diagnostics {
                                let adjusted_start = value_pos + 1 + diagnostic.start;
                                let adjusted_end = value_pos + 1 + diagnostic.end;
                                diagnostics.push((
                                    value_pos,
                                    end,
                                    MiniMessageDiagnostic {
                                        start: adjusted_start,
                                        end: adjusted_end,
                                        severity: diagnostic.severity,
                                        message: diagnostic.message,
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }

        diagnostics
    }
}

impl Default for MiniMessageValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_minimessage() {
        let validator = MiniMessageValidator::new();
        let input = "<red>Hello <bold>World</bold>!</red>";
        let diagnostics = validator.validate(input);
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_validate_invalid_minimessage() {
        let validator = MiniMessageValidator::new();
        let input = "<red>Hello <bold>World</red></bold>";
        let diagnostics = validator.validate(input);
        assert_eq!(diagnostics.len(), 2); // Two mismatched tags errors
    }

    #[test]
    fn test_validate_rust_file() {
        let validator = MiniMessageValidator::new();
        let rust_code = r#"
            fn main() {
                let message = Component::text("<red>Hello <bold>World</red></bold>");
                println!("{}", message);
            }
        "#;

        let diagnostics = validator.validate_rust_file(rust_code);
        assert_eq!(diagnostics.len(), 2); // Two mismatched tags errors
    }

    #[test]
    fn test_validate_toml_file() {
        let validator = MiniMessageValidator::new();
        let toml_code = r#"
            chat.format = "<red><bold>Player</bold></red>: <italic>Message</italic>"
            header = "<gold>Welcome to the server!</gold>"
        "#;

        let diagnostics = validator.validate_toml_file(toml_code);

        // Print diagnostics for debugging
        for (_, _, diagnostic) in &diagnostics {
            println!(
                "Diagnostic: {} at range {}-{}",
                diagnostic.message, diagnostic.start, diagnostic.end
            );
        }

        assert_eq!(diagnostics.len(), 0); // Both strings are valid
    }
}
