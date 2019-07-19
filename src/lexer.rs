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
        self.skip_whitespace();

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
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    Token::lookup_ident(&literal)
                } else if self.ch.is_numeric() {
                    Token::Int(self.read_number())
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }

        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char()
        }

        self.input[position..self.position].to_owned()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
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
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);
        for e in expected.iter() {
            let token = lexer.next_token();
            println!("{:?}", token); 
            assert_eq!(e, &token, "Expected \"{:?}\" but got \"{:?}\"", e, token);
        }
    }
}
