use super::types::{AdditionOperations, MultiplicationOperations, ProgramTypes, RelationOperations};

#[derive(Debug, Clone)]
pub enum MainOperation {
    CreateVariable(Vec<(Vec<u64>, ProgramTypes)>),
    Operator(Operator)
}

#[derive(Debug, Clone)]
pub enum Operator {
    Composite(Vec<Operator>),
    Assignment(u64, Expression),
    If(Expression, Box<Self>, Option<Box<Self>>),
    For(Expression, Expression, Expression, Box<Self>),
    While(Expression, Box<Self>),
    Input(Vec<u64>),
    Output(Vec<Expression>)
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub operands: Vec<Operand>,
    pub operations: Vec<RelationOperations>
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub terms: Vec<Term>,
    pub operations: Vec<AdditionOperations>
}

#[derive(Debug, Clone)]
pub struct Term {
    pub multipliers: Vec<Multiplier>,
    pub operations: Vec<MultiplicationOperations>
}

#[derive(Debug, Clone)]
pub enum Multiplier {
    Identifier(u64),
    Variable(u64),
    Boolean(bool),
    Not(Box<Self>),
    Expression(Expression)
}
