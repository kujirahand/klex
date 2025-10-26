// This test file includes the generated lexer and tests it

#[path = "example_lexer.rs"]
mod example_lexer;

use example_lexer::*;

#[test]
fn test_number_token() {
    let input = "123".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, NUMBER);
    assert_eq!(token.value, "123");
    assert_eq!(token.row, 1);
    assert_eq!(token.col, 1);
    assert_eq!(token.length, 3);
}

#[test]
fn test_identifier_token() {
    let input = "abc".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, IDENTIFIER);
    assert_eq!(token.value, "abc");
}

#[test]
fn test_indent_calculation() {
    let input = "no_indent\n  two_spaces\n    four_spaces".to_string();
    println!("Testing input: {:?}", input);
    let mut lexer = Lexer::new(input);
    
    // Get all tokens and print debug info
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        println!("Token: kind={}, value={:?}, row={}, col={}, indent={}", 
                 token.kind, token.value, token.row, token.col, token.indent);
        tokens.push(token);
    }

    // Filter out identifier tokens for testing
    let identifiers: Vec<_> = tokens.into_iter()
        .filter(|t| t.kind == IDENTIFIER)
        .collect();

    assert_eq!(identifiers.len(), 3, "Should have 3 identifier tokens");
    
    // First token: no indent
    assert_eq!(identifiers[0].value, "no_indent");
    assert_eq!(identifiers[0].indent, 0, "First token should have 0 indent");
    
    // Second token: 2 spaces indent
    assert_eq!(identifiers[1].value, "two_spaces");
    assert_eq!(identifiers[1].indent, 2, "Second token should have 2 spaces indent");
    
    // Third token: 4 spaces indent
    assert_eq!(identifiers[2].value, "four_spaces");
    assert_eq!(identifiers[2].indent, 4, "Third token should have 4 spaces indent");
}

#[test]
fn test_operator_tokens() {
    let input = "+ - * /".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, PLUS);

    lexer.next_token(); // skip whitespace

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, MINUS);

    lexer.next_token(); // skip whitespace

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, MULTIPLY);

    lexer.next_token(); // skip whitespace

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, DIVIDE);
}

#[test]
fn test_expression() {
    let input = "123 + abc".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, NUMBER);
    assert_eq!(token.value, "123");

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, WHITESPACE);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, PLUS);
    assert_eq!(token.value, "+");

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, WHITESPACE);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.kind, IDENTIFIER);
    assert_eq!(token.value, "abc");

    assert!(lexer.next_token().is_none());
}

#[test]
fn test_row_col_tracking() {
    let input = "123\nabc".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.row, 1);
    assert_eq!(token.col, 1);

    let token = lexer.next_token().unwrap(); // newline
    assert_eq!(token.row, 1);
    assert_eq!(token.col, 4);

    let token = lexer.next_token().unwrap();
    assert_eq!(token.row, 2);
    assert_eq!(token.col, 1);
}

#[test]
fn test_indent_tracking() {
    let input = "    abc".to_string();
    let mut lexer = Lexer::new(input);

    let token = lexer.next_token().unwrap(); // whitespace
    assert_eq!(token.indent, 4);

    let token = lexer.next_token().unwrap(); // abc
    assert_eq!(token.indent, 4); // should preserve line's indent level
}
