use crate::utils::structs::tokens::Token;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    Missing(Token, String),
    Error(String)
}