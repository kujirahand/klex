mod generator;
mod parser;
mod token;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [output_file]", args[0]);
        eprintln!("  Generates a Rust lexer from a specification file");
        eprintln!();
        eprintln!("Input file format:");
        eprintln!("  (Rust code)");
        eprintln!("  %%");
        eprintln!("  (Lexer rules - one per line: pattern -> name)");
        eprintln!("  %%");
        eprintln!("  (Rust code)");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() >= 3 {
        args[2].clone()
    } else {
        "lexer.rs".to_string()
    };

    // Read input file
    let input = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };

    // Parse specification
    let spec = match parser::parse_spec(&input) {
        Ok(spec) => spec,
        Err(e) => {
            eprintln!("Error parsing specification: {}", e);
            process::exit(1);
        }
    };

    // Generate lexer code
    let generated_code = generator::generate_lexer(&spec);

    // Write output file
    match fs::write(&output_file, generated_code) {
        Ok(_) => {
            println!("Lexer generated successfully: {}", output_file);
        }
        Err(e) => {
            eprintln!("Error writing output file '{}': {}", output_file, e);
            process::exit(1);
        }
    }
}
