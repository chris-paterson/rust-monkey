use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // Current position in input.
    read_position: usize, // Current reading position in input (after current char).
    ch: Option<char>,     // Current char being looked at.
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(ch @ _) => {
                if ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    return Token::lookup_ident(&literal);
                } else if ch.is_numeric() {
                    return Token::Int(self.read_number());
                } else {
                    Token::Illegal
                }
            }
            None => Token::EOF,
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while match self.ch {
            Some(ch) => ch.is_whitespace(),
            _ => false,
        } {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while match self.ch {
            Some(ch) => ch.is_alphabetic(),
            _ => false,
        } {
            self.read_char();
        }

        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while match self.ch {
            Some(ch) => ch.is_numeric(),
            _ => false,
        } {
            self.read_char();
        }

        self.input[position..self.position].to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5; 
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
       return true;
   } else {
       return false;
}

10 == 10;
10 != 9;
";

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Gt,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEq,
            Token::Int("9".to_string()),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);
        for e in expected.iter() {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(e, &token, "Expected \"{:?}\" but got \"{:?}\"", e, token);
        }
    }
}
