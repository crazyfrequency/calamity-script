use crate::utils::structs::types::ProgramTypes;

pub type SemanticResult<T> = Result<T, SemanticError>;

#[derive(Debug, Clone)]
pub enum SemanticError {
    NotDefined(u64),
    TypeError(u64, ProgramTypes, ProgramTypes),
    AssignError(ProgramTypes, ProgramTypes),
    Error(String)
}