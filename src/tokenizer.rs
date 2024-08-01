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

    // Peak one character forward
    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.source.len() {
            return self.source.chars().nth(self.pos + 1);
        }
        return None;
    }

    // Tokenize string values
    fn scan_str(&mut self) -> Option<String> {
        let start_pos: usize = self.pos;

        // Iterate till closing quote or EOF
        while (self.pos + 1) < self.source.len() {
            if self.peek().unwrap() == '\0' {
                panic!("Unexpected EOF at pos: {}", self.pos);
            }

            if self.peek().unwrap() == '"' {
                self.advance();
                return Some(self.source[start_pos..self.pos].to_string());
            }

            self.advance();
        }

        return None;
    }

    // Tokenize integer values
    fn scan_num(&mut self) -> Option<String> {
        let start_pos: usize = self.pos;
        let mut has_decimal: bool = false;

        while (self.pos + 1) < self.source.len() {
            let next: char = self.peek().unwrap();
            if !next.is_numeric() && next == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else if next.is_numeric() {
                self.advance();
            } else {
                self.advance();
                return Some(self.source[start_pos..self.pos].to_string());
            }
        }
        return None;
    }

    // Tokenize identifiers (true/false/null)
    fn scan_key(&mut self) -> Option<String> {
        return None;
    }

    // Tokenize a valid JSON string
    pub fn scan(&mut self) -> Vec<Token> {
        println!("{}", self.source);
        let mut tokens: Vec<Token> = Vec::new();
        let char_vect: Vec<char> = self.source.chars().collect();

        // Panic on trailing chars
        if !char_vect[char_vect.len() - 2].is_alphanumeric() {
            panic!(
                "Unexpected trailing character: '{}'",
                char_vect[char_vect.len() - 2]
            );
        }

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
                    self.advance(); // consume '"'
                    if let Some(res) = self.scan_str() {
                        tokens.push(Token {
                            kind: TokenKind::STRING,
                            value: Some(res),
                        })
                    }
                }
                '0'..='9' | '-' => {
                    let mut num: String = String::new();
                    if lexeme == '-' {
                        num.push('-');
                        self.advance(); // consume "-"
                    }
                    if let Some(res) = self.scan_num() {
                        num += &res;
                        tokens.push(Token {
                            kind: TokenKind::NUMBER,
                            value: Some(num),
                        })
                    }
                    self.pos -= 1; // reset cursor
                }
                _ => {
                    panic!(
                        "Unexpected character '{}' encountered at pos: {}",
                        lexeme, self.pos
                    );
                }
            }

            self.advance();
        }

        return tokens;
    }
}
