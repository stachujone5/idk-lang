use crate::{
    ast::{Program, Statement},
    lexer::{Lexer, Token},
};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        // What to start width?
        let mut p = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let identifier = match self.peek_token.clone() {
            Token::Ident(identifier) => {
                self.next_token();
                identifier
            }
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(identifier))
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
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

        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (index, &identifier) in expected_identifiers.iter().enumerate() {
            let statement = &program.statements[index];

            assert_eq!(statement.clone(), Statement::Let(identifier.to_string()))
        }
    }
}
