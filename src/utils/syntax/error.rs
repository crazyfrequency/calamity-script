use crate::utils::structs::tokens::TokenGroup;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    Missing(TokenGroup, String),
    Error(String)
}