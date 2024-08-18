#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> Option<&Statement> {
        match self.statements.first() {
            Some(statement) => Some(statement),
            _ => None,
        }
    }
}
