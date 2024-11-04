use std::{fmt::Display, ops::BitAnd};

#[derive(Debug, Clone)]
pub enum LexerDigitalData {
    Integer(i64),
    Float(f64)
}

impl Display for LexerDigitalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LexerDigitalData::Float(v) => format!("{}", v),
            LexerDigitalData::Integer(v) => format!("{}", v)
        })
    }
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
        let a = match self {
            ProgramTypes::Integer(_) => 0,
            ProgramTypes::Float(_) => 1,
            ProgramTypes::Boolean(_) => 2
        };

        let b = match rhs {
            ProgramTypes::Integer(_) => 0,
            ProgramTypes::Float(_) => 1,
            ProgramTypes::Boolean(_) => 2
        };

        a==b
    }
}

impl Display for ProgramTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ProgramTypes::Boolean(_) => "boolean",
            ProgramTypes::Float(_) => "real",
            ProgramTypes::Integer(_) => "integer"
        })
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

impl Into<String> for RelationOperations {
    fn into(self) -> String {
        match self {
            RelationOperations::Equal => "==",
            RelationOperations::Greater => ">",
            RelationOperations::GreaterEqual => ">=",
            RelationOperations::Less => "<",
            RelationOperations::LessEqual => "<=",
            RelationOperations::NotEqual => "!="
        }.to_string()
    }
}

impl Display for RelationOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RelationOperations::Equal => "не равно",
            RelationOperations::NotEqual => "равно",
            RelationOperations::Less => "меньше",
            RelationOperations::Greater => "больше",
            RelationOperations::LessEqual => "меньше или равно",
            RelationOperations::GreaterEqual => "больше или равно"
        })
    }
}

#[derive(Debug, Clone)]
pub enum AdditionOperations {
    Addition,
    Subtraction,
    Or
}

impl Display for AdditionOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AdditionOperations::Addition => "сложение",
            AdditionOperations::Subtraction => "вычитание",
            AdditionOperations::Or => "или"
        })
    }
}

#[derive(Debug, Clone)]
pub enum MultiplicationOperations {
    Multiplication,
    Division,
    And
}

impl Display for MultiplicationOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MultiplicationOperations::Multiplication => "умножение",
            MultiplicationOperations::Division => "деление",
            MultiplicationOperations::And => "и"
        })
    }
}
