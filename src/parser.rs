//! Parser module for klex.
//!
//! This module handles parsing of lexer specification files and provides
//! data structures to represent the parsed content.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Represents different types of rule patterns.
#[derive(Debug, Clone)]
pub enum RulePattern {
    /// Single character literal: 'c'
    CharLiteral(char),
    /// String literal: "string"
    StringLiteral(String),
    /// Regular expression pattern: /pattern/
    Regex(String),
    /// Character set with quantifier: [abc]+, [xyz]* etc.
    CharSet(String),
    /// Character range with one or more matches: [0-9]+, [a-z]+
    CharRangeMatch1(char, char),
    /// Character range with zero or more matches: [0-9]*, [a-z]*
    CharRangeMatch0(char, char),
    /// Choice between patterns: (pattern1 | pattern2)
    Choice(Vec<RulePattern>),
    /// Escaped special character: \+, \*, \n, etc.
    EscapedChar(char),
    /// Any single character: ?
    AnyChar,
    /// One or more any characters: ?+
    AnyCharPlus,
}

/// Represents a lexer rule with a pattern and token kind.
///
/// Each rule defines how to match a specific token type using a pattern.
/// Rules can optionally depend on a previous token context.
#[derive(Debug, Clone)]
pub struct LexerRule {
    pub pattern: RulePattern,
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
    /// * `pattern` - The pattern to match
    /// * `kind` - The numeric token kind identifier
    /// * `name` - The symbolic name for this token type
    pub fn new(pattern: RulePattern, kind: u32, name: String) -> Self {
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
    /// * `pattern` - The pattern to match
    /// * `kind` - The numeric token kind identifier
    /// * `name` - The symbolic name for this token type
    /// * `context_token` - The name of the token that must precede this rule
    pub fn new_with_context(
        pattern: RulePattern,
        kind: u32,
        name: String,
        context_token: String,
    ) -> Self {
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
    /// * `pattern` - The pattern to match
    /// * `action_code` - The Rust code to execute when this pattern matches
    pub fn new_with_action(pattern: RulePattern, action_code: String) -> Self {
        LexerRule {
            pattern,
            kind: 0,             // Action rules don't need a kind
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
/// - Custom tokens (explicitly declared with %token directive)
#[derive(Debug)]
pub struct LexerSpec {
    pub prefix_code: String,
    pub rules: Vec<LexerRule>,
    pub suffix_code: String,
    pub custom_tokens: Vec<String>,
}

impl LexerSpec {
    /// Creates a new empty lexer specification.
    pub fn new() -> Self {
        LexerSpec {
            prefix_code: String::new(),
            rules: Vec::new(),
            suffix_code: String::new(),
            custom_tokens: Vec::new(),
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

/// Parses a rule pattern from a string.
///
/// Supports various pattern formats:
/// - 'c' for character literals
/// - "string" for string literals  
/// - /regex/ for regular expressions
/// - [0-9]+, [abc]*, [a-z] for character sets with quantifiers
/// - (pattern1 | pattern2) for choices between patterns
/// - ? for any single character
/// - ?+ for one or more any characters
/// - \+, \n, \t, etc. for escaped characters
/// - Any other pattern is treated as a regex for backward compatibility
fn parse_pattern(input: &str) -> Result<RulePattern, ParseError> {
    let trimmed = input.trim();

    // Any character plus: ?+
    if trimmed == "?+" {
        return Ok(RulePattern::AnyCharPlus);
    }

    // Any single character: ?
    if trimmed == "?" {
        return Ok(RulePattern::AnyChar);
    }

    // Escaped character: \+, \n, etc.
    if trimmed.starts_with('\\') && trimmed.len() == 2 {
        let escape_char = trimmed.chars().nth(1).unwrap();
        let actual_char = match escape_char {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '\\' => '\\',
            '+' => '+',
            '*' => '*',
            '?' => '?',
            '(' => '(',
            ')' => ')',
            '[' => '[',
            ']' => ']',
            '{' => '{',
            '}' => '}',
            '|' => '|',
            '^' => '^',
            '$' => '$',
            '.' => '.',
            c => c, // Pass through other characters as-is
        };
        return Ok(RulePattern::EscapedChar(actual_char));
    }

    // Character literal: 'c'
    if trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() == 3 {
        let ch = trimmed.chars().nth(1).unwrap();
        return Ok(RulePattern::CharLiteral(ch));
    }

    // String literal: "string"
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        let content = &trimmed[1..trimmed.len() - 1];
        return Ok(RulePattern::StringLiteral(content.to_string()));
    }

    // Regular expression: /pattern/
    if trimmed.starts_with('/') && trimmed.ends_with('/') && trimmed.len() >= 2 {
        let content = &trimmed[1..trimmed.len() - 1];
        return Ok(RulePattern::Regex(content.to_string()));
    }

    // Character patterns: [0-9]+, [abc]+, [a-z]* etc.
    if trimmed.starts_with('[') && trimmed.contains(']') {
        // Parse bracket pattern
        // Check for simple range patterns like [0-9]+ or [a-z]*
        if let Some(closing_bracket) = trimmed.find(']') {
            let inside = &trimmed[1..closing_bracket];
            let quantifier = &trimmed[closing_bracket + 1..];
            
            // Helper function to parse a character or Unicode escape sequence
            let parse_char = |s: &str| -> Option<char> {
                if s.starts_with("\\u{") && s.ends_with('}') {
                    // Parse Unicode escape: \u{1F600}
                    let hex_str = &s[3..s.len()-1];
                    u32::from_str_radix(hex_str, 16)
                        .ok()
                        .and_then(|code| char::from_u32(code))
                } else if s.starts_with("\\x") && s.len() == 4 {
                    // Parse hex escape: \x41
                    let hex_str = &s[2..];
                    u8::from_str_radix(hex_str, 16)
                        .ok()
                        .map(|code| code as char)
                } else if s.len() == 1 {
                    s.chars().next()
                } else {
                    None
                }
            };
            
            // Check if it's a simple range like "0-9" or "a-z" or "\u{1F600}-\u{1F64F}"
            if let Some(dash_pos) = inside.find('-') {
                let start_str = &inside[..dash_pos];
                let end_str = &inside[dash_pos + 1..];
                
                if let (Some(start_char), Some(end_char)) = (parse_char(start_str), parse_char(end_str)) {
                    match quantifier {
                        "+" => return Ok(RulePattern::CharRangeMatch1(start_char, end_char)),
                        "*" => return Ok(RulePattern::CharRangeMatch0(start_char, end_char)),
                        _ => {} // Fall through to CharSet for other quantifiers
                    }
                }
            }
        }
        
        // For more complex patterns, use CharSet
        return Ok(RulePattern::CharSet(trimmed.to_string()));
    }

    // Choice: (pattern1 | pattern2)
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let content = &trimmed[1..trimmed.len() - 1];
        let parts: Vec<&str> = content.split('|').collect();
        if parts.len() > 1 {
            let mut patterns = Vec::new();
            for part in parts {
                patterns.push(parse_pattern(part.trim())?);
            }
            return Ok(RulePattern::Choice(patterns));
        }
    }

    // Default: treat as regex pattern for backward compatibility
    Ok(RulePattern::Regex(trimmed.to_string()))
}

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
    let mut token_names: HashMap<String, u32> = HashMap::new();

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

        // Check for %token directive
        if line.starts_with("%token") {
            // Extract custom token names: %token TOKEN1 TOKEN2 TOKEN3
            // or %token TOKEN1, TOKEN2, TOKEN3
            let tokens_part = line[6..].trim(); // Remove "%token"
            
            // Split by whitespace and/or commas
            let token_names_list: Vec<String> = tokens_part
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            
            spec.custom_tokens.extend(token_names_list);
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
                    if !token_names.contains_key(&context_token) {
                        return Err(Box::new(ParseError::new(format!(
                            "Unknown context token '{}' in rule: {}",
                            context_token, line
                        ))));
                    }
                    let pattern_str = parts[1].trim();
                    let pattern = parse_pattern(pattern_str)?;
                    spec.rules.push(LexerRule::new_with_context(
                        pattern,
                        kind_counter,
                        token_name,
                        context_token,
                    ));
                } else {
                    return Err(Box::new(ParseError::new(format!(
                        "Invalid context rule format: {}",
                        line
                    ))));
                }
            } else {
                return Err(Box::new(ParseError::new(format!(
                    "Context rule must have -> operator: {}",
                    line
                ))));
            }
        } else if let Some(arrow_pos) = line.find("->") {
            // Regular rule: pattern -> name or pattern -> { action_code }
            let pattern_str = line[..arrow_pos].trim();
            let pattern = parse_pattern(pattern_str)?;
            let right_part = line[arrow_pos + 2..].trim();

            if right_part.starts_with('{') && right_part.ends_with('}') {
                // Action rule: pattern -> { action_code }
                let action_code = right_part[1..right_part.len() - 1].trim().to_string();
                let mut rule = LexerRule::new_with_action(pattern, action_code);
                rule.kind = kind_counter; // Set the kind for action rules too
                spec.rules.push(rule);
            } else {
                // Token rule: pattern -> TOKEN_NAME
                let mut name = right_part.to_string();
                // Special case: _ is treated as Whitespace
                if name == "_" {
                    name = "Whitespace".to_string();
                }
                spec.rules.push(LexerRule::new(pattern, kind_counter, name));
            }
        } else {
            // Use the pattern as the name
            let pattern_str = line;
            let pattern = parse_pattern(pattern_str)?;
            let name = format!("TOKEN_{}", kind_counter);
            spec.rules.push(LexerRule::new(pattern, kind_counter, name));
        }

        if let Some(rule) = spec.rules.last() {
            if rule.action_code.is_none() && !rule.name.is_empty() {
                token_names.insert(rule.name.clone(), rule.kind);
            }
        }

        kind_counter += 1;
    }

    Ok(spec)
}
