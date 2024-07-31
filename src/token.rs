// Token kind
#[derive(Debug)]
pub enum TokenKind {
    LBRACE,
    RBRACE,
    COLON,
    COMMA,
    STRING,
    NUMBER,
    OBJECT,
    BOOLEAN,
    NULL
}

// Formatting for display: token kind
impl TokenKind {
    pub fn debug(&self) {
        println!("{:?}", self);
    }
}

// Token struct, each token has a 'kind' and value
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>
}

// Formatting for display: token
impl Token {
    pub fn debug(&self) {
        println!("kind: {:?}, value: {:?}", self.kind, self.value);
    }
}