mod parser;
mod reader;
mod token;
mod tokenizer;
mod util;

use std::io::{self, Write};

use parser::Parser;
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
    let mut content: String = reader::read_json(json_path);
    println!("\n{content}");

    // Remove whitespace and lex string
    util::remove_whitespace(&mut content);
    let mut lexer: Tokenizer = tokenizer::Tokenizer {
        pos: 0,
        source: content.trim().to_string(),
    };

    let tokens: Vec<Token> = lexer.scan();
    let mut parser: Parser = Parser::new(0, tokens);
    let res = parser.parse();

    println!("Parsed: {:#?}", res);
}
