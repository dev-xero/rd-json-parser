use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParsedValue {
    Token(Token),
    Object(HashMap<String, ParsedValue>),
}

pub struct Parser {
    pub pos: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    // Initialize a Parser
    pub fn new(pos: usize, tokens: Vec<Token>) -> Parser {
        let new_parser = Parser { pos, tokens };
        new_parser
    }

    // Advance a character ahead
    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;

            return Some(token);
        }
        return None;
    }

    // Expect a certain kind of token, panic otherwise
    fn expect(&mut self, expected: TokenKind) -> Option<String> {
        let token: Token = self.advance()?;

        if token.kind != expected {
            panic!("Expected: {:#?}, but found: {:#?}", expected, token.kind);
        }

        return token.value;
    }

    // Return the next character without advancing
    fn look_ahead(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Parse valid tokens from here
    pub fn parse(&mut self) -> HashMap<String, ParsedValue> {
        self.parse_object()
    }

    // Parse valid objects from here
    fn parse_object(&mut self) -> HashMap<String, ParsedValue> {
        self.expect(TokenKind::LBRACE); // Expecting '{'
        let members = self.parse_members();
        self.expect(TokenKind::RBRACE); // Expecting '}'

        members
    }

    // Parse valid members from here
    fn parse_members(&mut self) -> HashMap<String, ParsedValue> {
        let mut members = HashMap::new();

        if let Some(token) = self.look_ahead() {
            if token.kind != TokenKind::RBRACE {
                let (key, value) = self.parse_pair().expect("Failed to parse pair.");

                members.insert(key, value);

                while let Some(next_token) = self.look_ahead() {
                    if next_token.kind == TokenKind::COMMA {
                        self.advance();
                        let (key, value) = self.parse_pair().expect("Failed to parse pair");
                        members.insert(key, value);
                    } else {
                        break;
                    }
                }
            }
        }

        members
    }

    // Parse valid pairs
    fn parse_pair(&mut self) -> Option<(String, ParsedValue)> {
        let key = self
            .expect(TokenKind::STRING)
            .expect("Failed to parse key.");

        self.expect(TokenKind::COLON); // consume colon

        let value = self.parse_value().expect("Failed to parse value.");

        Some((key, value))
    }

    // Parse valid JSON values
    fn parse_value(&mut self) -> Option<ParsedValue> {
        if let Some(token) = self.look_ahead() {
            match token.kind {
                TokenKind::STRING | TokenKind::NUMBER | TokenKind::BOOLEAN | TokenKind::NULL => {
                    return Some(ParsedValue::Token(
                        self.advance().expect("Unexpected end of input"),
                    ));
                }
                TokenKind::LBRACE => Some(ParsedValue::Object(self.parse_object())),
                _ => panic!("Unexpected token encountered: {:?}", token.kind),
            }
        } else {
            None
        }
    }
}
