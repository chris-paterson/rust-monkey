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
