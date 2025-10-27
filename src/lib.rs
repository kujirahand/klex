//! # klex (kujira-lexer)
//!
//! A simple lexer (tokenizer) generator for Rust.
//!
//! klex generates Rust lexer code from a single definition file. You describe token patterns
//! with regular expressions, and it outputs Rust source that includes a `Token` struct and a `Lexer` struct.
//!
//! ## Usage
//!
//! ### As a library
//!
//! ```rust
//! use klex::{generate_lexer, parse_spec};
//! use std::fs;
//!
//! // Read input file
//! let input = fs::read_to_string("tests/example.klex").expect("Failed to read input file");
//!
//! // Parse the input
//! let spec = parse_spec(&input).expect("Failed to parse input");
//!
//! // Generate Rust code
//! let output = generate_lexer(&spec, "tests/example.klex");
//!
//! // Write output
//! fs::write("output.rs", output).expect("Failed to write output");
//! ```
//!
//! ### Input file format
//!
//! An input file consists of three sections separated by `%%`:
//!
//! ```text
//! (Rust code here – e.g. use statements)
//! %%
//! (Rules here – token patterns written as regular expressions)
//! %%
//! (Rust code here – e.g. main function or tests)
//! ```
//!
//! ### Writing rules
//!
//! Write one rule per line in the following form:
//!
//! ```text
//! <regex pattern> -> <TOKEN_NAME>
//! ```
//!
//! Examples:
//! ```text
//! [0-9]+ -> NUMBER
//! [a-zA-Z_][a-zA-Z0-9_]* -> IDENTIFIER
//! \+ -> PLUS
//! \- -> MINUS
//! ```

pub mod parser;
pub mod generator;
pub mod token;
pub mod lexer;

pub use generator::generate_lexer;
pub use parser::{parse_spec, LexerRule, LexerSpec, ParseError};
pub use token::Token;
