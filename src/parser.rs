//! Parser module for klex.
//!
//! This module handles parsing of lexer specification files and provides
//! data structures to represent the parsed content.

use std::error::Error;
use std::fmt;

/// Represents a lexer rule with a pattern and token kind.
///
/// Each rule defines how to match a specific token type using a regular expression pattern.
#[derive(Debug, Clone)]
pub struct LexerRule {
    pub pattern: String,
    pub kind: u32,
    pub name: String,
}

impl LexerRule {
    /// Creates a new lexer rule.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern to match
    /// * `kind` - The numeric token kind identifier
    /// * `name` - The symbolic name for this token type
    pub fn new(pattern: String, kind: u32, name: String) -> Self {
        LexerRule {
            pattern,
            kind,
            name,
        }
    }
}

/// Represents the parsed lexer specification.
///
/// Contains all the information needed to generate a lexer:
/// - Prefix code (Rust code to include at the beginning)
/// - Lexer rules (pattern -> token mappings)
/// - Suffix code (Rust code to include at the end)
#[derive(Debug)]
pub struct LexerSpec {
    pub prefix_code: String,
    pub rules: Vec<LexerRule>,
    pub suffix_code: String,
}

impl LexerSpec {
    /// Creates a new empty lexer specification.
    pub fn new() -> Self {
        LexerSpec {
            prefix_code: String::new(),
            rules: Vec::new(),
            suffix_code: String::new(),
        }
    }
}

impl Default for LexerSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for parsing failures.
#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    /// Creates a new parse error with the given message.
    pub fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

/// Parses a lexer specification file.
///
/// The input should be in the format:
/// ```text
/// (Rust code)
/// %%
/// (Lexer rules)
/// %%
/// (Rust code)
/// ```
///
/// Rules should be in the format: `pattern -> TOKEN_NAME` or just `pattern`.
///
/// # Arguments
///
/// * `input` - The lexer specification file content
///
/// # Returns
///
/// A `Result` containing the parsed `LexerSpec` or an error.
///
/// # Examples
///
/// ```rust
/// use klex::parse_spec;
///
/// let input = r#"
/// use std::collections::HashMap;
/// %%
/// [0-9]+ -> NUMBER
/// [a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
/// %%
/// // Generated code will be here
/// "#;
///
/// let spec = parse_spec(input).unwrap();
/// assert_eq!(spec.rules.len(), 2);
/// ```
pub fn parse_spec(input: &str) -> Result<LexerSpec, Box<dyn Error>> {
    let mut spec = LexerSpec::new();

    // Split by %%
    let parts: Vec<&str> = input.split("%%").collect();

    if parts.len() != 3 {
        return Err(Box::new(ParseError::new(
            "Input must have exactly 3 sections separated by %%".to_string(),
        )));
    }

    spec.prefix_code = parts[0].trim().to_string();
    spec.suffix_code = parts[2].trim().to_string();

    // Parse rules section
    let rules_section = parts[1].trim();
    let mut kind_counter = 0u32;

    for line in rules_section.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Parse rule: pattern -> name or just pattern
        if let Some(arrow_pos) = line.find("->") {
            let pattern = line[..arrow_pos].trim().to_string();
            let name = line[arrow_pos + 2..].trim().to_string();
            spec.rules.push(LexerRule::new(pattern, kind_counter, name));
        } else {
            // Use the pattern as the name
            let pattern = line.to_string();
            let name = format!("TOKEN_{}", kind_counter);
            spec.rules.push(LexerRule::new(pattern, kind_counter, name));
        }

        kind_counter += 1;
    }

    Ok(spec)
}
