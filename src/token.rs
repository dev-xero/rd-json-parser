// Token kind
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    LBRACE,
    RBRACE,
    COLON,
    COMMA,
    STRING,
    NUMBER,
    BOOLEAN,
    NULL,
}

// Formatting for display: token kind
impl TokenKind {
    pub fn debug(&self) {
        println!("{:?}", self);
    }
}

// Token struct, each token has a 'kind' and value
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

// Formatting for display: token
impl Token {
    pub fn debug(&self) {
        println!("{:?}", self.value);
    }
}
