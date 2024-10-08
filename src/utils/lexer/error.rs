use std::fmt::Display;

use crate::utils::structs::tokens::TokenGroupLexer;

pub type LexerResult<T> = Result<T, LexerError>;

pub struct LexerError {
    pub path: String,
    pub position: usize,
    pub line: usize,
    pub token: TokenGroupLexer,
    pub message: String
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Лексический анализатор сообщает, что при анализе токена '{:?}' по адресу {}:{}:{} произошла ошибка: {}",
            self.token,
            self.path,
            self.line,
            self.position,
            self.message
        )
    }
}