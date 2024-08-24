#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String),
    Return(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
