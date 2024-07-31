mod reader;
mod tokenizer;
mod token;

use std::io::{self, Write};

use token::Token;
use tokenizer::Tokenizer;

fn main() {
    // Mutable path reference
    let mut json_path: String = String::new();
    
    // Clear input buffer
    print!("> JSON File Path: ");
    io::stdout().flush().unwrap();

    // Read file path
    io::stdin()
        .read_line(&mut json_path)
        .expect("Could not read file path.");

    // Trim the file path
    let json_path: String = json_path.trim().to_string();

    // Read file and return contents
    let content: String = reader::read_json(json_path);
    let lexer: Tokenizer = tokenizer::Tokenizer {
        pos: 0,
        line: 1,
        source: content
    };

    let tokens: Vec<Token> = lexer.tokenize();
    println!("{tokens:?}");
}
