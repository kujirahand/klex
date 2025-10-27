//! Token definitions for klex.
//!
//! This module provides the `Token` struct used by generated lexers.


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // トークン種別の列挙型（例: IDENTIFIER, NUMBER, PLUS, etc.）
    // 生成時に自動的に定義される
    Unknown,
    // Basic literals
    Identifier,
    Number,
    Float,
    String,
    Char,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    
    // Keywords
    If,
    Else,
    While,
    For,
    Function,
    Return,
    Let,
    Const,
    
    // Special tokens
    Whitespace,
    Newline,
    Comment,
    Eof,
    
    // Context-dependent tokens
    IndexedNumber,
    PositiveNumber,
    
    // Custom tokens (for user-defined token types)
    Custom(u32),
}

/// Token structure that represents a lexical token.
///
/// Each token contains information about what was matched, where it was found,
/// and additional metadata like indentation level.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// トークン種別（列挙型として定義）
    pub kind: TokenKind,
    /// マッチしたテキスト
    pub value: String,
    /// 入力全体に対する0ベースの開始位置
    pub index: usize,
    /// 1ベース行番号
    pub row: usize,
    /// 1ベース列番号
    pub col: usize,
    /// トークン長
    pub length: usize,
    /// 行頭からのインデント（スペース数）
    pub indent: usize,
    /// カスタムタグ（デフォルト: 0）
    pub tag: isize,
}

#[allow(dead_code)]
impl Token {
    /// Creates a new token with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind
    /// * `value` - The matched text
    /// * `index` - 0-based start position in the entire input
    /// * `row` - 1-based line number
    /// * `col` - 1-based column number
    /// * `length` - Token length in characters
    /// * `indent` - Indentation level at line start
    ///
    /// # Returns
    ///
    /// A new `Token` instance with `tag` set to 0.
    pub fn new(
        kind: TokenKind,
        value: String,
        index: usize,
        row: usize,
        col: usize,
        length: usize,
        indent: usize,
    ) -> Self {
        Token {
            kind,
            value,
            index,
            row,
            col,
            length,
            indent,
            tag: 0,
        }
    }
}
