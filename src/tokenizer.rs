use crate::token::{Token, TokenKind};

// Tokenizer struct
pub struct Tokenizer {
    pub pos: usize,
    pub source: String,
}

// Methods
impl Tokenizer {
    // Advance one character forward
    fn advance(&mut self) -> Option<char> {
        if self.pos < self.source.len() {
            self.pos += 1;
            return self.source.chars().nth(self.pos - 1);
        }
        return None;
    }

    // Tokenize string values
    fn scan_str(&mut self) -> Option<String> {
        let start_pos: usize = self.pos;

        // Continue iteration till ending quote or EOF
        while self.pos < self.source.len() {
            let next: char = self.advance().unwrap();

            // Handle EOF
            if next == '\0' {
                panic!("Unexpected end of file at pos: {}", self.pos);
            }

            // Handle closing quotes
            if next == '"' {
                return Some(self.source[start_pos..self.pos - 1].to_string());
            }
        }

        return None;
    }

    // Tokenize integer values
    fn scan_num(&mut self) -> Option<String> {
        let start_pos: usize = self.pos;
        let mut has_decimal = false;

        // Iterate till we're out of numbers
        while self.pos < self.source.len() {
            let curr_char: char = self.source.chars().nth(self.pos).unwrap();

            if curr_char.is_numeric() {
                self.advance();
            } else if curr_char == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else {
                return Some(self.source[start_pos..self.pos].to_string());
            }
        }

        return None;
    }

    // Tokenize a valid JSON string
    pub fn scan(&mut self) -> Vec<Token> {
        println!("{}", self.source);
        let mut tokens: Vec<Token> = Vec::new();
        let char_vect: Vec<char> = self.source.chars().collect();

        // Iterate till the end
        while self.pos < self.source.len() {
            let lexeme: char = char_vect[self.pos];
            println!("curr: {}, max: {}", self.pos, self.source.len());

            match lexeme {
                '{' => tokens.push(Token {
                    kind: TokenKind::LBRACE,
                    value: None,
                }),
                '}' => tokens.push(Token {
                    kind: TokenKind::RBRACE,
                    value: None,
                }),
                ':' => tokens.push(Token {
                    kind: TokenKind::COLON,
                    value: None,
                }),
                ',' => tokens.push(Token {
                    kind: TokenKind::COMMA,
                    value: None,
                }),
                '"' => {
                    self.advance(); // consume opening "
                    if let Some(res) = self.scan_str() {
                        // Pure strings
                        tokens.push(Token {
                            kind: TokenKind::STRING,
                            value: Some(res),
                        });
                    }
                    self.pos -= 1; // set cursor at closing quote
                }
                '0'..='9' | '-' => {
                    let mut num: String = String::new();
                    if lexeme == '-' {
                        num.push('-');
                        self.advance();
                    }
                    if let Some(res) = self.scan_num() {
                        num += &res;
                        tokens.push(Token {
                            kind: TokenKind::NUMBER,
                            value: Some(num),
                        });
                    }
                    self.pos -= 1;
                }
                _ => {
                    panic!("Unexpected character '{}' at pos: {}", lexeme, self.pos);
                }
            }

            self.advance();
        }

        return tokens;
    }
}
