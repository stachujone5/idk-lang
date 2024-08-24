use crate::{
    ast::{Program, Statement},
    lexer::{Lexer, Token},
};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        // What to start width?
        let mut p = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let identifier = match self.peek_token.clone() {
            Token::Ident(identifier) => {
                self.next_token();
                identifier
            }
            Token::Int(int) => {
                self.next_token();
                int
            }
            _ => return None,
        };

        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Return(identifier))
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let identifier = match self.peek_token.clone() {
            Token::Ident(identifier) => {
                self.next_token();
                identifier
            }
            _ => {
                self.errors.push(format!(
                    "Expected identifier after 'let', got {:?} instead",
                    self.peek_token
                ));
                return None;
            }
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(identifier))
    }

    fn peek_error(&mut self, token: Token) {
        self.errors.push(format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        ))
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::Eof {
            let statement = self.parse_statement();

            if let Some(s) = statement {
                program.statements.push(s);
            }

            self.next_token();
        }

        program
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_statements() {
        let input = String::from(
            r#"
            let x = 5;
let y = 10;
let foobar = 838383;
                      "#,
        );

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);

        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (index, &identifier) in expected_identifiers.iter().enumerate() {
            let statement = &program.statements[index];

            assert_eq!(statement.clone(), Statement::Let(identifier.to_string()))
        }
    }

    #[test]
    fn return_statements() {
        let input = String::from(
            r#"
            return x;
return 10;
return 838383;
                      "#,
        );

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);

        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = vec!["x", "10", "838383"];

        for (index, &identifier) in expected_identifiers.iter().enumerate() {
            let statement = &program.statements[index];

            assert_eq!(statement.clone(), Statement::Return(identifier.to_string()))
        }
    }

    fn check_parser_errors(parser: Parser) {
        assert_eq!(parser.errors, Vec::new() as Vec<String>)
    }
}
