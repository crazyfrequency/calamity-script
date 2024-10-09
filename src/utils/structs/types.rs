#[derive(Debug, Clone)]
pub enum LexerDigitalData {
    Integer(i64),
    Float(f64)
}

#[derive(Debug, Clone)]
pub enum ProgramTypes {
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

#[derive(Debug, Clone)]
pub enum RelationOperations {
    NotEqual,
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual
}

#[derive(Debug, Clone)]
pub enum AdditionOperations {
    Addition,
    Subtraction,
    Or
}

#[derive(Debug, Clone)]
pub enum MultiplicationOperations {
    Multiplication,
    Division,
    And
}
