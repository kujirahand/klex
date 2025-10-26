/// Token structure that represents a lexical token
#[allow(dead_code)]
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

#[allow(dead_code)]
impl Token {
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
