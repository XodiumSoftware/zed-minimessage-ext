//! Example of MiniMessage usage in Rust code for Zed extension testing.
//!
//! This file demonstrates how MiniMessage strings appear in Rust code and
//! how the Zed extension would show hover previews for them.

/// Example function showing various MiniMessage patterns that would be
/// detected by the extension.
pub fn create_components() -> Vec<String> {
    let mut components = Vec::new();

    // Basic color usage
    components.push(format!("<red>Player {} joined the game!</red>", "JohnDoe"));

    // Nested formatting with bold and italic
    components.push(format!(
        "<green><bold>Welcome</bold> to the <italic>server</italic>!</green>"
    ));

    // Gradient example
    components.push(format!(
        "<gradient:#FF0000,#00FF00>Health: {}% ❤</gradient>",
        85
    ));

    // Click and hover events
    components.push(format!(
        "<blue><click:run_command:'help'>Click for help</click> <hover:show_text:'More info'>ℹ</hover></blue>"
    ));

    // Keybind translation
    components.push(format!(
        "Press <aqua><keybind:key.jump>SPACE</keybind> to jump!</aqua>"
    ));

    // Score display
    components.push(format!(
        "<yellow>Current score: <score:player:kills>0</score></yellow>"
    ));

    // Invalid example that would show errors
    // components.push("<red>Unclosed tag".to_string()); // Uncomment to see diagnostic

    components
}

/// Example of a TOML-like config that the extension would also validate.
/// This shows how MiniMessage strings appear in configuration contexts.
pub fn load_config() -> String {
    // This would be loaded from a file in a real application
    let config = r#"
# Chat configuration
chat.format = "<red>{player}</red>: {message}"

# Tab list configuration
header = "<gold>Welcome to the server!</gold>"
footer = "<gray>Online players: {online}</gray>"

# Join/leave messages
join.message = "<green>{player} joined the game!</green>"
leave.message = "<red>{player} left the game.</red>"

# MOTD (Message of the Day)
motd = "<gradient:#FF0000,#FFFF00>Welcome to our server! Enjoy your stay!</gradient>:"
"#;

    config.to_string()
}

/// Main function for testing the MiniMessage extension.
///
/// This function runs when the example is executed directly.
fn main() {
    println!("=== MiniMessage Extension Examples ===\n");

    // Create and display components
    println!("Creating components with MiniMessage:");
    let components = create_components();
    for (i, component) in components.iter().enumerate() {
        println!("  Component {}: {}", i + 1, component);
    }

    // Show configuration example
    println!("\nLoading configuration:");
    let config = load_config();
    println!("Config:\n{}", config);

    println!("\n=== Example complete ===");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components() {
        let components = create_components();
        assert!(!components.is_empty());

        // Check that components contain MiniMessage elements
        for component in components {
            if component.contains('<') && component.contains('>') {
                // This is a MiniMessage string
                println!("MiniMessage component: {}", component);
            }
        }
    }

    #[test]
    fn test_config_loading() {
        let config = load_config();
        assert!(config.contains("<red>"));
        assert!(config.contains("<gold>"));
        assert!(config.contains("<gradient:"));
    }
}
