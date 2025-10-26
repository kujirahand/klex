//! Parser module for klex.
//!
//! This module handles parsing of lexer specification files and provides
//! data structures to represent the parsed content.

use std::error::Error;
use std::fmt;

/// Represents a lexer rule with a pattern and token kind.
///
/// Each rule defines how to match a specific token type using a regular expression pattern.
/// Rules can optionally depend on a previous token context.
#[derive(Debug, Clone)]
pub struct LexerRule {
    pub pattern: String,
    pub kind: u32,
    pub name: String,
    pub context_token: Option<String>, // Optional context dependency
    pub action_code: Option<String>,   // Optional action code to execute when matched
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
            context_token: None,
            action_code: None,
        }
    }

    /// Creates a new context-dependent lexer rule.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern to match
    /// * `kind` - The numeric token kind identifier
    /// * `name` - The symbolic name for this token type
    /// * `context_token` - The name of the token that must precede this rule
    pub fn new_with_context(pattern: String, kind: u32, name: String, context_token: String) -> Self {
        LexerRule {
            pattern,
            kind,
            name,
            context_token: Some(context_token),
            action_code: None,
        }
    }

    /// Creates a new lexer rule with action code.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern to match
    /// * `action_code` - The Rust code to execute when this pattern matches
    pub fn new_with_action(pattern: String, action_code: String) -> Self {
        LexerRule {
            pattern,
            kind: 0, // Action rules don't need a kind
            name: String::new(), // Action rules don't have a name
            context_token: None,
            action_code: Some(action_code),
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

        // Parse different rule formats
        if line.starts_with('%') {
            // Context-dependent rule: %<CONTEXT_TOKEN> <pattern> -> <TOKEN_NAME>
            if let Some(arrow_pos) = line.find("->") {
                let left_part = line[1..arrow_pos].trim(); // Remove '%' and get left part
                let token_name = line[arrow_pos + 2..].trim().to_string();
                
                // Split left part to get context token and pattern
                let parts: Vec<&str> = left_part.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let context_token = parts[0].trim().to_string();
                    let pattern = parts[1].trim().to_string();
                    spec.rules.push(LexerRule::new_with_context(pattern, kind_counter, token_name, context_token));
                } else {
                    return Err(Box::new(ParseError::new(
                        format!("Invalid context rule format: {}", line)
                    )));
                }
            } else {
                return Err(Box::new(ParseError::new(
                    format!("Context rule must have -> operator: {}", line)
                )));
            }
        } else if let Some(arrow_pos) = line.find("->") {
            // Regular rule: pattern -> name or pattern -> { action_code }
            let pattern = line[..arrow_pos].trim().to_string();
            let right_part = line[arrow_pos + 2..].trim();
            
            if right_part.starts_with('{') && right_part.ends_with('}') {
                // Action rule: pattern -> { action_code }
                let action_code = right_part[1..right_part.len()-1].trim().to_string();
                let mut rule = LexerRule::new_with_action(pattern, action_code);
                rule.kind = kind_counter; // Set the kind for action rules too
                spec.rules.push(rule);
            } else {
                // Token rule: pattern -> TOKEN_NAME
                let name = right_part.to_string();
                spec.rules.push(LexerRule::new(pattern, kind_counter, name));
            }
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
