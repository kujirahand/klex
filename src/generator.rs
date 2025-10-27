//! Code generation module for klex.
//!
//! This module contains the functionality to generate Rust lexer code
//! from a parsed lexer specification.

use crate::parser::{LexerSpec, RulePattern};
use std::collections::HashSet;

// Include the auto-generated template
include!(concat!(env!("OUT_DIR"), "/template.rs"));

/// Extracts custom token names from action code.
/// Finds all occurrences of `TokenKind::Name` in the action code.
fn extract_custom_tokens(action_code: &str) -> HashSet<String> {
    let mut tokens = HashSet::new();
    let pattern = "TokenKind::";
    
    for (i, _) in action_code.match_indices(pattern) {
        let start = i + pattern.len();
        let remaining = &action_code[start..];
        
        // Extract the identifier after TokenKind::
        let end = remaining
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .count();
        
        if end > 0 {
            let token_name = &remaining[..end];
            // Skip common enum variants that are always present
            if token_name != "Unknown" && token_name != "Eof" {
                tokens.insert(token_name.to_string());
            }
        }
    }
    
    tokens
}

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
        RulePattern::CharSet(char_set_pattern) => {
            // Use character set pattern as-is (it's already a valid regex)
            char_set_pattern.clone()
        }
        RulePattern::CharRangeMatch1(start, end) => {
            // One or more character range: [start-end]+
            format!("[{}-{}]+", start, end)
        }
        RulePattern::CharRangeMatch0(start, end) => {
            // Zero or more character range: [start-end]*
            format!("[{}-{}]*", start, end)
        }
        RulePattern::Choice(patterns) => {
            // Create alternation: (pattern1|pattern2|...)
            let alternatives: Vec<String> = patterns.iter().map(|p| pattern_to_regex(p)).collect();
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

/// Generates optimized pattern matching code for a RulePattern.
/// This generates direct character/string comparison code instead of using regex when possible.
fn generate_pattern_match_code(pattern: &RulePattern, rule_name: &str) -> (String, bool) {
    match pattern {
        RulePattern::CharLiteral(ch) => {
            // Direct character comparison (most efficient)
            let escaped_ch = match ch {
                '\n' => "\\n".to_string(),
                '\t' => "\\t".to_string(),
                '\r' => "\\r".to_string(),
                '\\' => "\\\\".to_string(),
                '\'' => "\\'".to_string(),
                _ => ch.to_string(),
            };
            let code = format!(
                "if remaining.starts_with('{}') {{\n            Some(remaining.chars().next().unwrap().to_string())\n        }} else {{\n            None\n        }}",
                escaped_ch
            );
            (code, false) // false = doesn't need regex
        }
        RulePattern::StringLiteral(s) => {
            // Direct string comparison (very efficient)
            let escaped_s = s
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\t", "\\t")
                .replace("\r", "\\r");
            let code = format!(
                "if remaining.starts_with(\"{}\") {{\n            Some(\"{}\".to_string())\n        }} else {{\n            None\n        }}",
                escaped_s, escaped_s
            );
            (code, false) // false = doesn't need regex
        }
        RulePattern::EscapedChar(ch) => {
            // Direct character comparison for escaped chars
            let escaped_ch = match ch {
                '\n' => "\\n".to_string(),
                '\t' => "\\t".to_string(),
                '\r' => "\\r".to_string(),
                '\\' => "\\\\".to_string(),
                '\'' => "\\'".to_string(),
                _ => ch.to_string(),
            };
            let code = format!(
                "if remaining.starts_with('{}') {{\n            Some(remaining.chars().next().unwrap().to_string())\n        }} else {{\n            None\n        }}",
                escaped_ch
            );
            (code, false) // false = doesn't need regex
        }
        RulePattern::AnyChar => {
            // Match any single character (except newline)
            let code = "if let Some(ch) = remaining.chars().next() {\n            if ch != '\\n' {\n                Some(ch.to_string())\n            } else {\n                None\n            }\n        } else {\n            None\n        }".to_string();
            (code, false)
        }
        RulePattern::AnyCharPlus => {
            // Match one or more characters (except newline) - needs regex for simplicity
            (format!("self.match_cached_pattern(remaining, TokenKind::{})", rule_name), true)
        }
        RulePattern::CharRangeMatch1(start, end) => {
            // Character range with one or more matches - optimized direct matching
            let code = format!(
                "{{
            let mut matched = String::new();
            let mut chars = remaining.chars();
            while let Some(ch) = chars.next() {{
                if ch >= '{}' && ch <= '{}' {{
                    matched.push(ch);
                }} else {{
                    break;
                }}
            }}
            if !matched.is_empty() {{
                Some(matched)
            }} else {{
                None
            }}
        }}",
                start, end
            );
            (code, false) // false = doesn't need regex
        }
        RulePattern::CharRangeMatch0(_start, _end) => {
            // Character range with zero or more matches - needs regex for proper implementation
            (format!("self.match_cached_pattern(remaining, TokenKind::{})", rule_name), true)
        }
        RulePattern::Regex(_) | RulePattern::CharSet(_) | RulePattern::Choice(_) => {
            // Complex patterns need regex
            (format!("self.match_cached_pattern(remaining, TokenKind::{})", rule_name), true)
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
        output = output.replace(
            "// This file is auto-generated.",
            &format!("// This file is auto-generated.\n{}", prefix_with_newlines),
        );
    }

    // Generate TokenKind enum variants
    let mut token_kind_variants = String::new();
    let mut all_token_names = HashSet::new();
    
    // Collect token names from rules
    for rule in &spec.rules {
        if rule.action_code.is_none() && !rule.name.is_empty() {
            // Skip Unknown and Eof as they are always added automatically
            if rule.name != "Unknown" && rule.name != "Eof" {
                all_token_names.insert(rule.name.clone());
            }
        }
    }
    
    // Add explicitly declared custom tokens from %token directive
    for token_name in &spec.custom_tokens {
        if token_name != "Unknown" && token_name != "Eof" {
            all_token_names.insert(token_name.clone());
        }
    }
    
    // Collect custom token names from action code
    for rule in &spec.rules {
        if let Some(action_code) = &rule.action_code {
            let custom_tokens = extract_custom_tokens(action_code);
            all_token_names.extend(custom_tokens);
        }
    }
    
    // Generate variants for all collected tokens
    for token_name in &all_token_names {
        // Find the rule that defines this token to get pattern description
        if let Some(rule) = spec.rules.iter().find(|r| &r.name == token_name) {
            let pattern_desc = pattern_to_regex(&rule.pattern)
                .replace('\n', "\\n")
                .replace('\t', "\\t")
                .replace('\r', "\\r");
            token_kind_variants.push_str(&format!("\t{}, // {}\n", token_name, pattern_desc));
        } else {
            // Custom token without a pattern (used only in action code or %token directive)
            token_kind_variants.push_str(&format!("\t{}, // Custom token\n", token_name));
        }
    }

    // Generate regex cache code (only for patterns that need regex)
    let mut regex_code = String::new();
    regex_code.push_str("        // Pre-compile patterns that require regex\n");
    for rule in &spec.rules {
        let (_match_code, needs_regex) = generate_pattern_match_code(&rule.pattern, &rule.name);
        if needs_regex {
            // Convert pattern to regex and escape for string literal
            let regex_pattern = pattern_to_regex(&rule.pattern);
            let escaped_pattern = regex_pattern.replace("\\", "\\\\").replace("\"", "\\\"");
            regex_code.push_str(&format!(
                "        regex_cache.insert(TokenKind::{} as u32, Regex::new(\"^{}\").unwrap());\n",
                rule.name, escaped_pattern
            ));
        }
    }
    regex_code.push_str("        ");

    // Generate rule matching code
    let mut rule_match_code = String::new();

    // First, generate context-dependent rules (higher priority)
    for rule in &spec.rules {
        if let Some(context_token) = &rule.context_token {
            // Find the context token name
            let context_token_name = spec
                .rules
                .iter()
                .find(|r| r.name == *context_token)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| panic!("Context token '{}' not found", context_token));

            let (match_code, _needs_regex) = generate_pattern_match_code(&rule.pattern, &rule.name);
            let pattern_desc = pattern_to_regex(&rule.pattern)
                .replace('\n', "\\n")
                .replace('\t', "\\t")
                .replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Context-dependent rule: {} -> {} (after {})
        if self.last_token_kind == Some(TokenKind::{}) {{
            let matched_opt = {{{}}};
            if let Some(matched) = matched_opt {{
                let token = Token::new(
                    TokenKind::{},
                    matched.clone(),
                    self.pos,
                    start_row,
                    start_col,
                    matched.len(),
                    indent,
                );
                self.advance(&matched);
                self.last_token_kind = Some(token.kind.clone());
                return Some(token);
            }}
        }}

"#,
                pattern_desc, rule.name, context_token, context_token_name, match_code, rule.name
            ));
        }
    }

    // Second, generate action rules (higher priority than regular token rules)
    for rule in &spec.rules {
        if rule.context_token.is_none() && rule.action_code.is_some() {
            let action_code = rule.action_code.as_ref().unwrap();
            let (match_code, _needs_regex) = generate_pattern_match_code(&rule.pattern, &rule.name);
            let pattern_desc = pattern_to_regex(&rule.pattern)
                .replace('\n', "\\n")
                .replace('\t', "\\t")
                .replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Action rule: {} -> {{ {} }}
        {{
            let matched_opt = {{{}}};
            if let Some(matched) = matched_opt {{
                let matched_str = matched.clone();
                // Create token for action code to use
                let test_t = Token::new(
                    TokenKind::Unknown,
                    matched_str.clone(),
                    self.pos,
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
                    self.last_token_kind = Some(token.kind.clone());
                    return Some(token);
                }} else {{
                    // Continue to next iteration if no token was returned from action
                    return self.next_token();
                }}
            }}
        }}

"#,
                pattern_desc, action_code, match_code, action_code
            ));
        }
    }

    // Finally, generate regular token rules
    for rule in &spec.rules {
        if rule.context_token.is_none() && rule.action_code.is_none() {
            let update_context = if rule.name == "WHITESPACE" || rule.name == "Whitespace" || rule.name == "NEWLINE" || rule.name == "Newline" {
                "// Whitespace tokens don't update context"
            } else {
                "self.last_token_kind = Some(token.kind.clone())"
            };

            let (match_code, _needs_regex) = generate_pattern_match_code(&rule.pattern, &rule.name);
            let pattern_desc = pattern_to_regex(&rule.pattern)
                .replace('\n', "\\n")
                .replace('\t', "\\t")
                .replace('\r', "\\r");
            rule_match_code.push_str(&format!(
                r#"        // Rule: {} -> {}
        {{
            let matched_opt = {{{}}};
            if let Some(matched) = matched_opt {{
                let token = Token::new(
                    TokenKind::{},
                    matched.clone(),
                    self.pos,
                    start_row,
                    start_col,
                    matched.len(),
                    indent,
                );
                self.advance(&matched);
                {};
                return Some(token);
            }}
        }}

"#,
                pattern_desc, rule.name, match_code, rule.name, update_context
            ));
        }
    }

    // Generate to_string method
    let mut to_string_method = String::new();
    to_string_method.push_str("\t/// Returns a string representation of the token kind for debugging purposes.\n");
    to_string_method.push_str("\t///\n");
    to_string_method.push_str("\t/// # Returns\n");
    to_string_method.push_str("\t///\n");
    to_string_method.push_str("\t/// A human-readable string representation of the token kind\n");
    to_string_method.push_str("\tpub fn to_string(&self) -> String {\n");
    to_string_method.push_str("\t\tmatch self.kind {\n");
    
    // Add cases for all collected tokens (including custom tokens)
    for token_name in &all_token_names {
        to_string_method.push_str(&format!("\t\t\tTokenKind::{} => \"{}\".to_string(),\n", token_name, token_name));
    }
    
    // Add case for Unknown
    to_string_method.push_str("\t\t\tTokenKind::Unknown => \"UNKNOWN\".to_string(),\n");
    to_string_method.push_str("\t\t}\n");
    to_string_method.push_str("\t}");

    // Replace markers with generated code
    output = output.replace(
        "//----<GENERATED_BY>----",
        &format!("// Generated from: {}", source_file),
    );
    output = output.replace("//----<TOKEN_KIND>----", &token_kind_variants);
    output = output.replace("//----<REG_EX_CODE>----", &regex_code);
    output = output.replace("//----<RULE_MATCH_CODE>----", &rule_match_code);
    output = output.replace("//----<TO_STRING_METHOD>----", &to_string_method);

    // Add suffix code
    if !spec.suffix_code.is_empty() {
        output.push_str(&format!("\n{}\n", spec.suffix_code));
    }

    output
}
