/// Code generation templates for the lexer

/// Import statements template
pub const IMPORTS_TEMPLATE: &str = r#"use regex::Regex;
use std::collections::HashMap;

"#;

/// Token struct template
pub const TOKEN_STRUCT_TEMPLATE: &str = r#"/// Token structure that represents a lexical token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: u32,
    pub value: String,
    pub row: usize,
    pub col: usize,
    pub length: usize,
    pub indent: usize,
    pub tag: isize,
}

impl Token {
    pub fn new(kind: u32, value: String, row: usize, col: usize, length: usize, indent: usize) -> Self {
        Token {
            kind,
            value,
            row,
            col,
            length,
            indent,
            tag: 0,
        }
    }
}

"#;

/// Lexer struct template
pub const LEXER_STRUCT_TEMPLATE: &str = r#"pub struct Lexer {
    input: String,
    pos: usize,
    row: usize,
    col: usize,
    regex_cache: HashMap<u32, Regex>,
}

"#;

/// Lexer constructor template - this will be customized per lexer
pub fn lexer_constructor_template(rules: &[crate::parser::LexerRule]) -> String {
    let mut constructor = String::from(
        r#"impl Lexer {
    pub fn new(input: String) -> Self {
        let mut regex_cache = HashMap::new();
        
        // Pre-compile all patterns and store them in cache
"#,
    );

    for rule in rules {
        constructor.push_str(&format!(
            "        regex_cache.insert({}, Regex::new(r\"^{}\").unwrap());\n",
            rule.kind, rule.pattern
        ));
    }

    constructor.push_str(
        r#"        
        Lexer {
            input,
            pos: 0,
            row: 1,
            col: 1,
            regex_cache,
        }
    }

"#,
    );

    constructor
}

/// next_token method start template
pub const NEXT_TOKEN_START_TEMPLATE: &str = r#"    pub fn next_token(&mut self) -> Option<Token> {
        if self.pos >= self.input.len() {
            return None;
        }

        let remaining = &self.input[self.pos..];
        let start_row = self.row;
        let start_col = self.col;

        // Calculate indent (spaces at the start of line)
        let indent = if self.col == 1 {
            remaining.chars().take_while(|&c| c == ' ').count()
        } else {
            0
        };

"#;

/// next_token method end template (fallback for unmatched characters)
pub const NEXT_TOKEN_END_TEMPLATE: &str = r#"        // No pattern matched, consume one character
        let ch = remaining.chars().next().unwrap();
        let matched = ch.to_string();
        self.advance(&matched);
        Some(Token::new(UNKNOWN_TOKEN, matched, start_row, start_col, 1, indent))
    }

"#;

/// Helper methods template (optimized version with regex caching)
pub const HELPER_METHODS_TEMPLATE: &str = r#"    fn match_cached_pattern(&self, input: &str, token_kind: u32) -> Option<String> {
        if let Some(regex) = self.regex_cache.get(&token_kind) {
            if let Some(mat) = regex.find(input) {
                return Some(mat.as_str().to_string());
            }
        }
        None
    }

    fn advance(&mut self, matched: &str) {
        for ch in matched.chars() {
            self.pos += ch.len_utf8();
            if ch == '\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }
}

"#;

/// Template for generating a rule match pattern (optimized version)
pub fn rule_match_template(rule_pattern: &str, rule_name: &str, rule_kind: u32) -> String {
    format!(
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
            return Some(token);
        }}

"#,
        rule_pattern, rule_name, rule_kind, rule_kind
    )
}
