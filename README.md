# klex (kujira-lexer)

A simple lexer (tokenizer) generator for Rust.

English | [日本語はこちら](README-ja.md)

## Overview

klex generates Rust lexer code from a single definition file. You describe token patterns with regular expressions, and it outputs Rust source that includes a `Token` struct and a `Lexer` struct.

## Installation

```bash
cargo build --release
```

## Usage

### Basic

```bash
cargo run -- <INPUT_FILE> [OUTPUT_FILE]
```

### Input file format

An input file consists of three sections separated by `%%`:

```
(Rust code here – e.g. use statements)
%%
(Rules here – token patterns written as regular expressions)
%%
(Rust code here – e.g. main function or tests)
```

### Writing rules

Write one rule per line in the following form:

```
<regex pattern> -> <TOKEN_NAME>
```

Examples:
```
[0-9]+ -> NUMBER
[a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
\+ -> PLUS
\- -> MINUS
```

### Generated Token struct

The generated lexer produces tokens with the following shape:

```rust
struct Token {
    kind: u32,      // token kind (defined as constants)
    value: String,  // matched text
    row: usize,     // 1-based line number
    col: usize,     // 1-based column number
    length: usize,  // token length
    indent: usize,  // indentation width at line start (spaces)
    tag: isize,     // custom tag (defaults to 0)
}
```

## Examples

See `example.klex` for a minimal definition file.

### Generate a lexer

```bash
cargo run -- example.klex generated_lexer.rs
```

### Use the generated lexer

The generated file exports a `Lexer` struct and related constants:

```rust
let input = "123 + abc".to_string();
let mut lexer = Lexer::new(input);

while let Some(token) = lexer.next_token() {
    println!("{:?}", token);
}
```

## Tests

```bash
cargo test
```

## License

MIT License

