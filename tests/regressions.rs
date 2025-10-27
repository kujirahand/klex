use klex::{generate_lexer, parse_spec};

/// Regression harness for specifications that previously slipped through parsing.
#[test]
fn char_range_quantifier_is_preserved() {
    let spec_src = r#"
%%
[a-z]* -> OPTIONAL_ALPHA
%%
"#;
    let spec = parse_spec(spec_src).expect("spec parses");
    let generated = generate_lexer(&spec, "<inline>");

    assert!(
        generated.contains("Regex::new(\"^[a-z]*\")"),
        "expected generated lexer to keep the '*' quantifier when lowering [a-z]*, but got:\n{}",
        generated
    );
}

/// Regression harness to ensure context-dependent rules refer to known tokens.
#[test]
fn context_rule_requires_defined_token() {
    let spec_src = r#"
%%
%MISSING [0-9]+ -> NUMBER_AFTER_MISSING
%%
"#;

    assert!(
        parse_spec(spec_src).is_err(),
        "parser should surface an error when a context rule references an unknown token"
    );
}
