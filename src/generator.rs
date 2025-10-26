use crate::parser::LexerSpec;

/// Generates Rust code for the lexer
pub fn generate_lexer(spec: &LexerSpec) -> String {
    let mut output = String::new();
    
    // Add prefix code
    if !spec.prefix_code.is_empty() {
        output.push_str(&spec.prefix_code);
        output.push_str("\n\n");
    }
    
    // Add Token struct
    output.push_str("/// Token structure that represents a lexical token\n");
    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str("pub struct Token {\n");
    output.push_str("    pub kind: u32,\n");
    output.push_str("    pub value: String,\n");
    output.push_str("    pub row: usize,\n");
    output.push_str("    pub col: usize,\n");
    output.push_str("    pub length: usize,\n");
    output.push_str("    pub indent: usize,\n");
    output.push_str("    pub tag: isize,\n");
    output.push_str("}\n\n");
    
    output.push_str("impl Token {\n");
    output.push_str("    pub fn new(kind: u32, value: String, row: usize, col: usize, length: usize, indent: usize) -> Self {\n");
    output.push_str("        Token {\n");
    output.push_str("            kind,\n");
    output.push_str("            value,\n");
    output.push_str("            row,\n");
    output.push_str("            col,\n");
    output.push_str("            length,\n");
    output.push_str("            indent,\n");
    output.push_str("            tag: 0,\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Generate token kind constants
    output.push_str("// Token kind constants\n");
    for rule in &spec.rules {
        output.push_str(&format!("pub const {}: u32 = {};\n", rule.name, rule.kind));
    }
    output.push_str("pub const UNKNOWN_TOKEN: u32 = u32::MAX; // For unmatched characters\n");
    output.push_str("\n");
    
    // Generate the lexer struct
    output.push_str("pub struct Lexer {\n");
    output.push_str("    input: String,\n");
    output.push_str("    pos: usize,\n");
    output.push_str("    row: usize,\n");
    output.push_str("    col: usize,\n");
    output.push_str("}\n\n");
    
    output.push_str("impl Lexer {\n");
    output.push_str("    pub fn new(input: String) -> Self {\n");
    output.push_str("        Lexer {\n");
    output.push_str("            input,\n");
    output.push_str("            pos: 0,\n");
    output.push_str("            row: 1,\n");
    output.push_str("            col: 1,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    output.push_str("    pub fn next_token(&mut self) -> Option<Token> {\n");
    output.push_str("        if self.pos >= self.input.len() {\n");
    output.push_str("            return None;\n");
    output.push_str("        }\n\n");
    
    output.push_str("        let remaining = &self.input[self.pos..];\n");
    output.push_str("        let start_row = self.row;\n");
    output.push_str("        let start_col = self.col;\n\n");
    
    output.push_str("        // Calculate indent (spaces at the start of line)\n");
    output.push_str("        let indent = if self.col == 1 {\n");
    output.push_str("            remaining.chars().take_while(|&c| c == ' ').count()\n");
    output.push_str("        } else {\n");
    output.push_str("            0\n");
    output.push_str("        };\n\n");
    
    // Generate pattern matching for each rule
    for rule in &spec.rules {
        output.push_str(&format!("        // Rule: {} -> {}\n", rule.pattern, rule.name));
        output.push_str(&format!("        if let Some(matched) = self.match_pattern(remaining, r\"{}\") {{\n", rule.pattern));
        output.push_str("            let token = Token::new(\n");
        output.push_str(&format!("                {},\n", rule.kind));
        output.push_str("                matched.clone(),\n");
        output.push_str("                start_row,\n");
        output.push_str("                start_col,\n");
        output.push_str("                matched.len(),\n");
        output.push_str("                indent,\n");
        output.push_str("            );\n");
        output.push_str("            self.advance(&matched);\n");
        output.push_str("            return Some(token);\n");
        output.push_str("        }\n\n");
    }
    
    output.push_str("        // No pattern matched, consume one character\n");
    output.push_str("        let ch = remaining.chars().next().unwrap();\n");
    output.push_str("        let matched = ch.to_string();\n");
    output.push_str("        self.advance(&matched);\n");
    output.push_str("        Some(Token::new(UNKNOWN_TOKEN, matched, start_row, start_col, 1, indent))\n");
    output.push_str("    }\n\n");
    
    output.push_str("    fn match_pattern(&self, input: &str, pattern: &str) -> Option<String> {\n");
    output.push_str("        use regex::Regex;\n");
    output.push_str("        let regex_pattern = format!(\"^{}\", pattern);\n");
    output.push_str("        if let Ok(re) = Regex::new(&regex_pattern) {\n");
    output.push_str("            if let Some(mat) = re.find(input) {\n");
    output.push_str("                return Some(mat.as_str().to_string());\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("        None\n");
    output.push_str("    }\n\n");
    
    output.push_str("    fn advance(&mut self, matched: &str) {\n");
    output.push_str("        for ch in matched.chars() {\n");
    output.push_str("            self.pos += ch.len_utf8();\n");
    output.push_str("            if ch == '\\n' {\n");
    output.push_str("                self.row += 1;\n");
    output.push_str("                self.col = 1;\n");
    output.push_str("            } else {\n");
    output.push_str("                self.col += 1;\n");
    output.push_str("            }\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    
    // Add suffix code
    if !spec.suffix_code.is_empty() {
        output.push_str(&spec.suffix_code);
        output.push_str("\n");
    }
    
    output
}
