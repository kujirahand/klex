/// Code generation templates for the lexer

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
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            pos: 0,
            row: 1,
            col: 1,
        }
    }

"#;

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

/// Helper methods template
pub const HELPER_METHODS_TEMPLATE: &str = r#"    fn match_pattern(&self, input: &str, pattern: &str) -> Option<String> {
        use regex::Regex;
        let regex_pattern = format!("^{}", pattern);
        if let Ok(re) = Regex::new(&regex_pattern) {
            if let Some(mat) = re.find(input) {
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

/// Template for generating a rule match pattern
pub fn rule_match_template(rule_pattern: &str, rule_name: &str, rule_kind: u32) -> String {
    format!(r#"        // Rule: {} -> {}
        if let Some(matched) = self.match_pattern(remaining, r"{}") {{
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

"#, rule_pattern, rule_name, rule_pattern, rule_kind)
}