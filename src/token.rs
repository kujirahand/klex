//! Token definitions for klex.
//!
//! This module provides the `Token` struct used by generated lexers.

/// Token structure that represents a lexical token.
///
/// Each token contains information about what was matched, where it was found,
/// and additional metadata like indentation level.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Token kind (numeric identifier)
    pub kind: u32,
    /// The matched text value
    pub value: String,
    /// 1-based line number where token was found
    pub row: usize,
    /// 1-based column number where token starts
    pub col: usize,
    /// Length of the token in characters
    pub length: usize,
    /// Indentation level (number of spaces at line start)
    pub indent: usize,
    /// Custom tag for additional metadata (defaults to 0)
    pub tag: isize,
}

#[allow(dead_code)]
impl Token {
    /// Creates a new token with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind (numeric identifier)
    /// * `value` - The matched text
    /// * `row` - 1-based line number
    /// * `col` - 1-based column number
    /// * `length` - Token length in characters
    /// * `indent` - Indentation level at line start
    ///
    /// # Returns
    ///
    /// A new `Token` instance with `tag` set to 0.
    pub fn new(
        kind: u32,
        value: String,
        row: usize,
        col: usize,
        length: usize,
        indent: usize,
    ) -> Self {
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
