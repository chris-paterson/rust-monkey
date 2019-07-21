use crate::lexer::Lexer;
use crate::token::Token;
use std::io::{self, BufRead, Write};

pub fn start() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        stdout.lock().write(b"> ")?;
        stdout.lock().flush()?;

        let mut buffer = String::new();
        stdin.lock().read_line(&mut buffer)?;

        let mut lexer = Lexer::new(&mut buffer);

        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
