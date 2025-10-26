use crate::parser::LexerSpec;
use std::fs;

/// Generates Rust code for the lexer (optimized version with regex caching)
pub fn generate_lexer(spec: &LexerSpec) -> String {
    // Read the lexer.rs template file
    let template = fs::read_to_string("src/lexer.rs")
        .expect("Failed to read src/lexer.rs template file");

    let mut output = template;

    // Add prefix code at the beginning
    if !spec.prefix_code.is_empty() {
        let prefix_with_newlines = format!("{}\n\n", spec.prefix_code);
        output = output.replace("// このファイルは自動生成されます。", &format!("// このファイルは自動生成されます。\n{}", prefix_with_newlines));
    }

    // Generate token kind constants
    let mut constants = String::new();
    constants.push_str("// Token kind constants\n");
    for rule in &spec.rules {
        constants.push_str(&format!("pub const {}: u32 = {};\n", rule.name, rule.kind));
    }
    constants.push_str("pub const UNKNOWN_TOKEN: u32 = u32::MAX; // For unmatched characters\n\n");

    // Generate regex cache code
    let mut regex_code = String::new();
    regex_code.push_str("        // Pre-compile all patterns and store them in cache\n");
    for rule in &spec.rules {
        regex_code.push_str(&format!(
            "        regex_cache.insert({}, Regex::new(r\"^{}\").unwrap());\n",
            rule.kind, rule.pattern
        ));
    }
    regex_code.push_str("        ");

    // Generate rule matching code
    let mut rule_match_code = String::new();
    for rule in &spec.rules {
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
            return Some(token);
        }}

"#,
            rule.pattern, rule.name, rule.kind, rule.kind
        ));
    }

    // Replace markers with generated code
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
