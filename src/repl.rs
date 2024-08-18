use std::io::{self};

use crate::lexer::{Lexer, Token};

pub fn start() {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        let mut lexer = Lexer::new(buffer.clone());

        loop {
            let token = lexer.next_token();

            println!("{:?}", token);

            if token == Token::Eof {
                break;
            }
        }
    }
}
