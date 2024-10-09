use super::types::{AdditionOperations, MultiplicationOperations, ProgramTypes, RelationOperations};

#[derive(Debug, Clone)]
pub enum MainOperations {
    CreateVariable(Vec<u64>, ProgramTypes),
    Operator(Vec<Operator>)
}

#[derive(Debug, Clone)]
pub enum Operator {
    Composite(Vec<Operator>),
    Assignment(u64, Expression),
    If(Expression, Box<Self>, Option<Box<Self>>),
    For(Expression, Expression, Expression, Box<Self>),
    While(Expression, Box<Self>),
    Input(Vec<u64>),
    Output(Vec<u64>)
}

#[derive(Debug, Clone)]
pub struct Expression {
    operands: Vec<Operand>,
    operations: Vec<RelationOperations>
}

#[derive(Debug, Clone)]
pub struct Operand {
    terms: Vec<Operand>,
    operation: Vec<AdditionOperations>
}

#[derive(Debug, Clone)]
pub struct Term {
    multiplier: Vec<Multiplier>,
    operation: Vec<MultiplicationOperations>
}

#[derive(Debug, Clone)]
pub enum Multiplier {
    Identifier(u64),
    Variable(u64),
    Boolean(bool),
    Not(Box<Self>),
    Expression(Expression)
}
