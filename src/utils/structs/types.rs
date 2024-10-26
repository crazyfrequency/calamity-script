use std::ops::{BitAnd, Shr};

#[derive(Debug, Clone)]
pub enum LexerDigitalData {
    Integer(i64),
    Float(f64)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramTypes {
    Integer(Option<i64>),
    Float(Option<f64>),
    Boolean(Option<bool>)
}

impl BitAnd for ProgramTypes {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            ProgramTypes::Boolean(_) => match rhs {
                ProgramTypes::Boolean(_) => true,
                _ => false
            },
            _ => match rhs {
                ProgramTypes::Boolean(_) => false,
                _ => true
            }
        }
    }
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
