//! Crate root for the project.
//!
//! Add modules and re-exports here as the project grows.

/// Placeholder project name. Replace with the actual project name.
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

/// Returns a greeting message for the project.
///
/// This is a minimal example function. Remove or replace it with real logic.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to the project.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(greet("world"), "Hello, world! Welcome to the project.");
    }
}
