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
    assert_eq!(token.indent, 0); // not at start of line anymore
}
