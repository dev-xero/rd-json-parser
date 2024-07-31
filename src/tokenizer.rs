use crate::token::{Token, TokenKind};

pub struct Tokenizer {
    pub pos: u32,
    pub line: u32,
    pub source: String
}

impl Tokenizer {
    // Tokenize a valid JSON string
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut pos: u32 = 0;

        // Iterate through every character
        for lexeme in self.source.chars() {
            match lexeme {
                '{' => tokens.push(Token {
                    kind: TokenKind::LBRACE,
                    value: None
                }),
                '}' => tokens.push(Token {
                    kind: TokenKind::RBRACE,
                    value: None
                }),
                ':' => tokens.push(Token {
                    kind: TokenKind::COLON,
                    value: None
                }),
                ',' => tokens.push(Token {
                    kind: TokenKind::COMMA,
                    value: None
                }),
                '"' => {},
                _ => continue
            }

            pos += 1;
        }

        println!("{:?}", tokens);

        return tokens;
    }

}