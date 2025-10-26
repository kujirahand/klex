use crate::parser::LexerSpec;
use crate::template::*;

/// Generates Rust code for the lexer (optimized version with regex caching)
pub fn generate_lexer(spec: &LexerSpec) -> String {
    let mut output = String::new();

    // Add prefix code
    if !spec.prefix_code.is_empty() {
        output.push_str(&spec.prefix_code);
        output.push_str("\n\n");
    }

    // Add import statements for regex caching
    output.push_str(IMPORTS_TEMPLATE);

    // Add Token struct
    output.push_str(TOKEN_STRUCT_TEMPLATE);

    // Generate token kind constants
    output.push_str("// Token kind constants\n");
    for rule in &spec.rules {
        output.push_str(&format!("pub const {}: u32 = {};\n", rule.name, rule.kind));
    }
    output.push_str("pub const UNKNOWN_TOKEN: u32 = u32::MAX; // For unmatched characters\n");
    output.push_str("\n");

    // Generate the lexer struct (with regex cache)
    output.push_str(LEXER_STRUCT_TEMPLATE);

    // Generate the optimized constructor with pre-compiled regexes
    output.push_str(&lexer_constructor_template(&spec.rules));

    output.push_str(NEXT_TOKEN_START_TEMPLATE);

    // Generate pattern matching for each rule (using cached regexes)
    for rule in &spec.rules {
        output.push_str(&rule_match_template(&rule.pattern, &rule.name, rule.kind));
    }

    output.push_str(NEXT_TOKEN_END_TEMPLATE);

    output.push_str(HELPER_METHODS_TEMPLATE);

    // Add suffix code
    if !spec.suffix_code.is_empty() {
        output.push_str(&spec.suffix_code);
        output.push_str("\n");
    }

    output
}
