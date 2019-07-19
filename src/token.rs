#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers and literals.
    Ident(String),
    Int(String),

    // Operators.
    Assign,
    Plus,

    // Delimeters.
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords.
    Function,
    Let,
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Ident(ident.to_string()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::Ident(literal) => write!(f, "Ident({})", literal),
            Token::Int(literal) => write!(f, "Int({})", literal),
            _ => write!(f, "\"{:?})\"", &self.to_string())
        }
    }
}
