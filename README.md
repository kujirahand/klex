# klex (kujira-lexer)

A simple lexer (tokenizer) generator for Rust.

English | [日本語はこちら](README-ja.md)

## Overview

klex generates Rust lexer code from a single definition file. You describe token patterns with regular expressions, and it outputs Rust source that includes a `Token` struct and a `Lexer` struct.

## Installation

### From crates.io

```bash
cargo install klex
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
klex = "0.1.2"
```

### From source

```bash
git clone https://github.com/kujirahand/klex
cd klex
cargo build --release
```

## Usage

### As a library

```rust
use klex::{generate_lexer, parse_spec};
use std::fs;

// Read input file
let input = fs::read_to_string("example.klex").expect("Failed to read input file");

// Parse the input
let spec = parse_spec(&input).expect("Failed to parse input");

// Generate Rust code
let output = generate_lexer(&spec, "example.klex");

// Write output
fs::write("output.rs", output).expect("Failed to write output");
```

### Command line tool

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
<pattern> -> <TOKEN_NAME>
```

Supported pattern formats:

- `'c'` - Single character literal
- `"string"` - String literal
- `[0-9]+` - Character range with quantifier
- `[abc]+` - Character set with quantifier
- `/regex/` - Regular expression pattern
- `( pattern1 | pattern2 )` - Choice between patterns
- `\+` - Escaped special characters (`\+`, `\*`, `\n`, `\t`, etc.)
- `?` - Any single character
- `?+` - One or more any characters

Examples:

```text
[0-9]+ -> NUMBER
[a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
\+ -> PLUS
\- -> MINUS
\n -> NEWLINE
\t -> TAB
? -> ANY_CHAR
?+ -> ANY_CHAR_PLUS
"hello" -> HELLO
/[0-9]+\.[0-9]+/ -> FLOAT
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

## Advanced Features

### Escaped Characters

klex supports escaped special characters:

```text
\+ -> PLUS_ESCAPED    # Matches literal '+'
\* -> MULTIPLY        # Matches literal '*'
\n -> NEWLINE         # Matches newline character
\t -> TAB             # Matches tab character
```

### Wildcard Patterns

Use wildcard patterns for flexible matching:

```text
? -> ANY_CHAR         # Matches any single character
?+ -> ANY_CHAR_PLUS   # Matches one or more characters (i.e., captures to the end)
```

### Context-Dependent Rules

Rules can depend on the previous token:

```text
%IDENTIFIER [0-9]+ -> INDEXED_NUMBER   # Only after IDENTIFIER
```

### Action Code

Execute custom Rust code when a pattern matches:

```text
"debug" -> { println!("Debug mode!"); None }
```

## Examples

See `tests/*.klex` files for definition examples.

### Generate a lexer

```bash
cargo run -- tests/example.klex tests/example_lexer.rs
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

Run all tests:

```bash
make test
```

## License

MIT License

