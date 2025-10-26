//! Code generation module for klex.
//!
//! This module contains the functionality to generate Rust lexer code
//! from a parsed lexer specification.

use crate::parser::{LexerSpec, RulePattern};

// Include the auto-generated template
include!(concat!(env!("OUT_DIR"), "/template.rs"));

/// Converts a RulePattern to a regular expression string.
fn pattern_to_regex(pattern: &RulePattern) -> String {
    match pattern {
        RulePattern::CharLiteral(ch) => {
            // Escape special regex characters
            regex::escape(&ch.to_string())
        }
        RulePattern::StringLiteral(s) => {
            // Escape the entire string for literal matching
            regex::escape(s)
        }
        RulePattern::Regex(regex_str) => {
            // Use the regex pattern as-is
            regex_str.clone()
        }
        RulePattern::CharRange(start_char, end_char) => {
            // Convert character range to regex pattern: [a-z]+
            format!("[{}-{}]+", start_char, end_char)
        }
        RulePattern::CharSet(char_set_pattern) => {
            // Use character set pattern as-is (it's already a valid regex)
            char_set_pattern.clone()
        }
        RulePattern::Choice(patterns) => {
            // Create alternation: (pattern1|pattern2|...)
            let alternatives: Vec<String> = patterns.iter()
                .map(|p| pattern_to_regex(p))
                .collect();
            format!("({})", alternatives.join("|"))
        }
        RulePattern::EscapedChar(ch) => {
            // Escape the character for regex matching
            regex::escape(&ch.to_string())
        }
        RulePattern::AnyChar => {
            // Match any single character (except newline)
            ".".to_string()
        }
        RulePattern::AnyCharPlus => {
            // Match one or more of any character (except newline)
            ".+".to_string()
        }
    }
}

/// Generates Rust code for the lexer (optimized version with regex caching).
///
/// This function takes a parsed lexer specification and generates complete
/// Rust source code that includes:
/// - Token kind constants
/// - A Lexer struct with caching for compiled regex patterns
/// - Token generation logic
/// - User-defined prefix and suffix code
///
/// # Arguments
///
/// * `spec` - The parsed lexer specification containing rules and code sections
/// * `source_file` - The name of the source file (used for comments)
///
/// # Returns
///
/// A String containing the complete generated Rust code for the lexer.
///
/// # Example
///
/// ```rust
/// use klex::{parse_spec, generate_lexer};
///
/// let input = r#"
/// use std::collections::HashMap;
/// %%
/// [0-9]+ -> NUMBER
/// [a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
/// %%
/// fn main() { println!("Generated lexer"); }
/// "#;
///
/// let spec = parse_spec(input).unwrap();
/// let code = generate_lexer(&spec, "example.klex");
/// // code now contains complete Rust lexer implementation
/// ```
pub fn generate_lexer(spec: &LexerSpec, source_file: &str) -> String {
    // Use the embedded template
    let template = LEXER_TEMPLATE;

    let mut output = template.to_string();

    // Add prefix code at the beginning
    if !spec.prefix_code.is_empty() {
        let prefix_with_newlines = format!("{}\n\n", spec.prefix_code);
        output = output.replace("// This file is auto-generated.", &format!("// This file is auto-generated.\n{}", prefix_with_newlines));
    }

    // Generate token kind constants
    let mut constants = String::new();
    constants.push_str("// Token kind constants\n");
    for rule in &spec.rules {
        if rule.action_code.is_none() && !rule.name.is_empty() {
            constants.push_str(&format!("pub const {}: u32 = {};\n", rule.name, rule.kind));
        }
    }
    constants.push_str("\n");

    // Generate regex cache code
    let mut regex_code = String::new();
    regex_code.push_str("        // Pre-compile all patterns and store them in cache\n");
    for rule in &spec.rules {
        // Convert pattern to regex and escape for string literal
        let regex_pattern = pattern_to_regex(&rule.pattern);
        let escaped_pattern = regex_pattern.replace("\\", "\\\\").replace("\"", "\\\"");
        regex_code.push_str(&format!(
            "        regex_cache.insert({}, Regex::new(\"^{}\").unwrap());\n",
            rule.kind, escaped_pattern
        ));
    }
    regex_code.push_str("        ");

    // Generate rule matching code
    let mut rule_match_code = String::new();
    
    // First, generate context-dependent rules (higher priority)
    for rule in &spec.rules {
        if let Some(context_token) = &rule.context_token {
            // Find the context token kind by name
            let context_kind = spec.rules.iter()
                .find(|r| r.name == *context_token)
                .map(|r| r.kind)
                .unwrap_or(0); // Default to 0 if not found
            
            let pattern_desc = pattern_to_regex(&rule.pattern).replace('\n', "\\n").replace('\t', "\\t").replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Context-dependent rule: {} -> {} (after {})
        if self.last_token_kind == Some({}) {{
            if let Some(matched) = self.match_cached_pattern(remaining, {}) {{
                let token = Token::new(
                    {},
                    matched.clone(),
                    start_row,
                    start_col,
                    matched.len(),
                    indent,
                );
                self.advance(&matched);
                self.last_token_kind = Some(token.kind);
                return Some(token);
            }}
        }}

"#,
                pattern_desc, rule.name, context_token, context_kind, rule.kind, rule.kind
            ));
        }
    }
    
    // Second, generate action rules (higher priority than regular token rules)
    for rule in &spec.rules {
        if rule.context_token.is_none() && rule.action_code.is_some() {
            let action_code = rule.action_code.as_ref().unwrap();
            let pattern_desc = pattern_to_regex(&rule.pattern).replace('\n', "\\n").replace('\t', "\\t").replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Action rule: {} -> {{ {} }}
        if let Some(matched) = self.match_cached_pattern(remaining, {}) {{
            let matched_str = matched.clone();
            // Create token for action code to use
            let test_t = Token::new(
                {},
                matched_str.clone(),
                start_row,
                start_col,
                matched_str.len(),
                indent,
            );
            self.advance(&matched_str);
            // Execute action code with available variables
            let action_result: Option<Token> = {{
                {}
            }};
            if let Some(token) = action_result {{
                self.last_token_kind = Some(token.kind);
                return Some(token);
            }} else {{
                // Continue to next iteration if no token was returned from action
                return self.next_token();
            }}
        }}

"#,
                pattern_desc, action_code, rule.kind, rule.kind, action_code
            ));
        }
    }
    
    // Finally, generate regular token rules
    for rule in &spec.rules {
        if rule.context_token.is_none() && rule.action_code.is_none() {
            let update_context = if rule.name == "WHITESPACE" || rule.name == "NEWLINE" {
                "// Whitespace tokens don't update context"
            } else {
                "self.last_token_kind = Some(token.kind)"
            };
            
            let pattern_desc = pattern_to_regex(&rule.pattern).replace('\n', "\\n").replace('\t', "\\t").replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Rule: {} -> {}
        if let Some(matched) = self.match_cached_pattern(remaining, {}) {{
            let token = Token::new(
                {},
                matched.clone(),
                start_row,
                start_col,
                matched.len(),
                indent,
            );
            self.advance(&matched);
            {};
            return Some(token);
        }}

"#,
                pattern_desc, rule.name, rule.kind, rule.kind, update_context
            ));
        }
    }

    // Replace markers with generated code
    output = output.replace("///GENERATED_BY", &format!("// Generated from: {}", source_file));
    output = output.replace("////REG_EX_CODE", &regex_code);
    output = output.replace("////RULE_MATCH_CODE", &rule_match_code);
    
    // Insert constants before the Token struct
    output = output.replace("use regex::Regex;\nuse std::collections::HashMap;\n", &format!("use regex::Regex;\nuse std::collections::HashMap;\n\n{}", constants));

    // Add suffix code
    if !spec.suffix_code.is_empty() {
        output.push_str(&format!("\n{}\n", spec.suffix_code));
    }

    output
}
