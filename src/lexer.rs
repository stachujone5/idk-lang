use crate::tokenizer::Token;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 'a',
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panicking::panic;

    use crate::tokenizer::TokenType;

    use super::*;

    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");

        let tests = vec![
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Plus,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Eof,
                literal: String::from(""),
            },
        ];

        let lexer = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = tt.next_token();

            if tok.token_type != tt.expect_type {

                //panic or smth
            }

            if tok.literal != tt.expected_literal {
                // panic or smth
            }
        }
    }
}
