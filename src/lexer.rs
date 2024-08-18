#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == b'_'
    }

    fn is_int(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_int() {
            self.read_char()
        }

        String::from_utf8_lossy(&self.input[position..self.position]).into()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char()
        }

        String::from_utf8_lossy(&self.input[position..self.position]).into()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let t = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            0 => Token::Eof,
            _ => {
                if self.is_letter() {
                    let identifier = self.read_identifier();

                    match identifier.as_str() {
                        "fn" => return Token::Function,
                        "let" => return Token::Let,
                        _ => (),
                    };

                    return Token::Ident(identifier);
                }

                if self.is_int() {
                    return Token::Int(self.read_number());
                }

                Token::Illegal
            }
        };

        self.read_char();

        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for current_token in tokens {
            let next_token = lexer.next_token();

            assert_eq!(current_token, next_token);
        }
    }

    #[test]
    fn next_token_full() {
        let input = String::from(
            r#"
     let five = 5;
     let ten = 10;
    
     let add = fn(x, y) {
         x + y;
     };
    
     let result = add(five, ten);
         "#,
        );

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for current_token in tokens {
            let next_token = lexer.next_token();

            assert_eq!(current_token, next_token);
        }
    }
}
