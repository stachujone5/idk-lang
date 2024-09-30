use crate::{
    ast::{Expression, Identifier, Procedence, Statement},
    lexer::{Lexer, Token},
};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
    errors: Vec<String>,
}

struct Program {
    statements: Vec<Statement>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let first_token = lexer.next_token();
        let second_token = lexer.next_token();

        Parser {
            lexer,
            current_token: first_token,
            next_token: second_token,
            errors: Vec::new(),
        }
    }

    fn parse_prefix(&mut self, token: &Token) -> Option<Expression> {
        match token {
            Token::Bang => self.parse_prefix(&Token::Minus),
            _ => self.parse_infix(&token, Expression::Identifier(String::from("IDK YET"))),
        }
    }

    fn parse_infix(&mut self, token: &Token, left_exp: Expression) -> Option<Expression> {
        match token {
            Token::Plus => self.parse_infix(
                &Token::Plus,
                Expression::Identifier(String::from("IDK YET")),
            ),
            _ => None,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression(&mut self, procedence: Procedence) -> Option<Expression> {
        let prefix = self.parse_prefix(&self.current_token);

        if let Some(prefix) = prefix {
            return Some(prefix);
        }

        None
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if self.next_token == Token::Semicolon {
            self.next_token()
        }

        let expression = self.parse_expression(Procedence::Lowest);

        if let Some(expression) = expression {
            return Some(Statement::ExpressionStatement(expression));
        }

        None
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let identifier = match self.next_token.clone() {
            Token::Ident(identifier) => {
                self.next_token();
                identifier
            }
            _ => {
                self.errors.push(format!(
                    "Expected identifier after 'let', got {:?} instead",
                    self.next_token
                ));
                return None;
            }
        };

        Some(Statement::LetStatement(
            Identifier(identifier),
            Expression::Identifier(String::from("IDK YET")),
        ))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let expression = match self.next_token.clone() {
            Token::Ident(identifier) => {
                self.next_token();
                identifier
            }
            Token::Int(integer) => {
                self.next_token();
                integer
            }
            Token::Semicolon => return None,
            _ => {
                self.errors.push(format!(
                    "Expected identifier after 'return', got {:?} instead",
                    self.next_token
                ));
                return None;
            }
        };

        Some(Statement::ReturnStatement(Expression::Identifier(
            expression,
        )))
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
    fn new_parser() {
        let input = String::from("let foobar = 5;");

        let lexer = Lexer::new(input);

        let parser = Parser::new(lexer);

        assert_eq!(parser.current_token, Token::Let);
        assert_eq!(parser.next_token, Token::Ident(String::from("foobar")));
    }

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

        let expected_statement_identifiers = vec![
            Identifier(String::from("x")),
            Identifier(String::from("y")),
            Identifier(String::from("foobar")),
        ];

        for (i, statement) in program.statements.iter().enumerate() {
            match statement {
                Statement::LetStatement(identifier, _) => {
                    assert_eq!(identifier, &expected_statement_identifiers[i]);
                }
                _ => panic!("Expected a let statement"),
            }
        }
    }

    #[test]
    fn return_statements() {
        let input = String::from(
            r#"
            return 5;
return 10;
return 993322;
          "#,
        );

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(return_parser_errors(&parser), 0);

        assert_eq!(program.statements.len(), 3);

        let expected_expressions = vec![
            Expression::Identifier(String::from("5")),
            Expression::Identifier(String::from("10")),
            Expression::Identifier(String::from("993322")),
        ];

        for (i, statement) in program.statements.iter().enumerate() {
            match statement {
                Statement::ReturnStatement(expression) => {
                    assert_eq!(expression, &expected_expressions[i]);
                }
                _ => panic!("Expected a returm statement"),
            }
        }
    }

    #[test]
    fn parser_errors() {
        let input = String::from(
            r#"
            let x = 5;
let = 10;
let foobar = 838383;
          "#,
        );

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        parser.parse_program();

        assert_eq!(return_parser_errors(&parser), 1)
    }

    fn return_parser_errors(parser: &Parser) -> usize {
        for error in &parser.errors {
            println!("{}", error)
        }

        parser.errors.len()
    }

    #[test]
    fn parse_identifier_expression() {
        let input = String::from("foobar;");

        let lexer = Lexer::new(input);

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(return_parser_errors(&parser), 0);

        assert_eq!(
            program.statements[0],
            Statement::ExpressionStatement(Expression::Identifier(String::from("foobar")))
        )
    }
}
