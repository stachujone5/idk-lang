#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

pub enum Procedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn to_string(&self) -> String {
        let mut content = String::new();

        for statement in &self.statements {
            match statement {
                Statement::LetStatement(identifier, expression) => {
                    content.push_str(&format!("let {} = {:?};", identifier.0, expression));
                }
                _ => (),
            }
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_program() {
        let program = Program {
            statements: vec![Statement::LetStatement(
                Identifier(String::from("myVar")),
                Expression::Identifier(String::from("X")),
            )],
        };

        assert_eq!(program.to_string(), "let myVar = X;")
    }
}
