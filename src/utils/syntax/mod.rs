use std::collections::HashMap;

use super::structs::{tokens::{DelimitersGroup, TokenGroup}, types::LexerDigitalData};

pub struct Syntax {
    path: String,
    current_token: TokenGroup,
    position: usize,

    tokens: Vec<TokenGroup>,
    vars: HashMap<u64, LexerDigitalData>
}

impl Syntax {
    pub fn new(
        path: impl Into<String>,
        tokens: Vec<TokenGroup>,
        vars: HashMap<u64, LexerDigitalData>
    ) -> Self {
        Self {
            path: path.into(),
            current_token: tokens.first().unwrap().clone(),
            position: 1,
            tokens,
            vars
        }
    }

    pub fn run_process(&mut self) {
        match self.current_token {
            TokenGroup::Delimiters(DelimitersGroup::LeftCurlyBracket) => {
                while 
                    self.current_token != TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket)
                    ||
                    self.current_token != TokenGroup::Eof
                {
                    self.next_token();
                }
                if self.current_token != TokenGroup::Eof {
                    return;
                }
            },
            _ => {}
        }
    }

    fn next_token(&mut self) {
        if let Some(next) = self.tokens.get(self.position) {
            self.current_token = next.clone();
            self.position += 1;
        } else {
            panic!("Ошибка непредвиденный конец цепочки лексем")
        }
    }
}