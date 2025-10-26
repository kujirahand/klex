use std::error::Error;
use std::fmt;

/// Represents a lexer rule with a pattern and token kind
#[derive(Debug, Clone)]
pub struct LexerRule {
    pub pattern: String,
    pub kind: u32,
    pub name: String,
}

impl LexerRule {
    pub fn new(pattern: String, kind: u32, name: String) -> Self {
        LexerRule {
            pattern,
            kind,
            name,
        }
    }
}

/// Represents the parsed lexer specification
#[derive(Debug)]
pub struct LexerSpec {
    pub prefix_code: String,
    pub rules: Vec<LexerRule>,
    pub suffix_code: String,
}

impl LexerSpec {
    pub fn new() -> Self {
        LexerSpec {
            prefix_code: String::new(),
            rules: Vec::new(),
            suffix_code: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

/// Parses a lexer specification file
pub fn parse_spec(input: &str) -> Result<LexerSpec, Box<dyn Error>> {
    let mut spec = LexerSpec::new();

    // Split by %%
    let parts: Vec<&str> = input.split("%%").collect();

    if parts.len() != 3 {
        return Err(Box::new(ParseError::new(
            "Input must have exactly 3 sections separated by %%".to_string(),
        )));
    }

    spec.prefix_code = parts[0].trim().to_string();
    spec.suffix_code = parts[2].trim().to_string();

    // Parse rules section
    let rules_section = parts[1].trim();
    let mut kind_counter = 0u32;

    for line in rules_section.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Parse rule: pattern -> name or just pattern
        if let Some(arrow_pos) = line.find("->") {
            let pattern = line[..arrow_pos].trim().to_string();
            let name = line[arrow_pos + 2..].trim().to_string();
            spec.rules.push(LexerRule::new(pattern, kind_counter, name));
        } else {
            // Use the pattern as the name
            let pattern = line.to_string();
            let name = format!("TOKEN_{}", kind_counter);
            spec.rules.push(LexerRule::new(pattern, kind_counter, name));
        }

        kind_counter += 1;
    }

    Ok(spec)
}
