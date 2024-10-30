use crate::utils::structs::types::ProgramTypes;

pub type SemanticResult<T> = Result<T, SemanticError>;

#[derive(Debug, Clone)]
pub enum SemanticError {
    NotDefined(u64),
    IdentifierAlreadyDeclared(u64),
    TypeError(ProgramTypes, ProgramTypes),
    AssignError(ProgramTypes, ProgramTypes),
    InvalidOperation(ProgramTypes, String),
    NotBoolean(ProgramTypes)
}