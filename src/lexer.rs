use crate::token::Token;

pub struct Lexer<'a>  {
    input: &'a str,
    position: usize, // Current position in input.
    read_position: usize, // Current reading position in input (after current char).
    ch: char, // Current char being looked at.
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
           input: input,
           position: 0,
           read_position: 0,
           ch: '0',
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) { 
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' =>  Token::Assign,
            ';' =>  Token::Semicolon,
            '(' =>  Token::LParen,
            ')' =>  Token::RParen,
            ',' =>  Token::Comma,
            '+' =>  Token::Plus,
            '{' =>  Token::LBrace,
            '}' =>  Token::RBrace,
            '0' =>  Token::EOF,
            _ => Token::Illegal,
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);
        for e in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(e, &token);
        }
    }
}
