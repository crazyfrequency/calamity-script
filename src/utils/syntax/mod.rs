use std::collections::HashMap;

use crate::utils::structs::tokens::KeywordsGroup;

use super::structs::{program::{MainOperation, Operator}, tokens::{DelimitersGroup, TokenGroup}, types::{LexerDigitalData, ProgramTypes}};

mod error;

use error::{SyntaxResult, SyntaxError};

#[derive(Debug, Clone)]
pub struct Syntax {
    path: String,
    current_token: TokenGroup,
    position: usize,

    tokens: Vec<TokenGroup>,
    vars: HashMap<u64, LexerDigitalData>
}

impl Syntax {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            current_token: TokenGroup::Eof,
            position: 1,
            tokens: Vec::default(),
            vars: HashMap::default()
        }
    }

    pub fn run_process(
        &mut self,
        tokens: Vec<TokenGroup>,
        vars: HashMap<u64, LexerDigitalData>
    ) -> SyntaxResult<Vec<MainOperation>> {
        self.tokens = tokens;
        self.vars = vars;

        let mut main = Vec::new();

        match self.current_token {
            TokenGroup::Delimiters(DelimitersGroup::LeftCurlyBracket) => {
                self.next_token();
                while 
                    self.current_token != TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket)
                    ||
                    self.current_token != TokenGroup::Eof
                {
                    let res = self.get_main();
                    if let Err(e) = res {
                        return Err(e);
                    }
                    main.push(res.unwrap());
                    self.next_token();
                    match self.current_token {
                        TokenGroup::Delimiters(DelimitersGroup::Semicolon) => self.next_token(),
                        TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) => (),
                        t => return Err(SyntaxError::Missing(t, "Ожидалось '}' или ';'".to_string()))
                    }
                }
                if self.current_token == TokenGroup::Eof {
                    return Err(SyntaxError::Missing(self.current_token.clone(), "Ожидалось '}'".to_string()));
                } else {
                    self.next_token();
                    match self.current_token {
                        TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) => self.next_token(),
                        t => return Err(SyntaxError::Missing(t, "Ожидалось '}'".to_string()))
                    }
                    if self.current_token != TokenGroup::Eof {
                        return Err(SyntaxError::Missing(self.current_token.clone(), "Ожидался конец программы".to_string()));
                    }
                }
            },
            _ => return Err(SyntaxError::Missing(self.current_token.clone(), "Программа должна начинаться с '{'".to_string()))
        }
        return Ok(main);
    }

    fn get_main(&mut self) -> SyntaxResult<MainOperation> {
        match self.current_token {
            TokenGroup::Keywords(KeywordsGroup::Var) => match self.get_var() {
                Ok(vars) => return Ok(MainOperation::CreateVariable(vars)),
                Err(e) => return Err(e)
            },
            _ => match self.get_operator() {
                Ok(v) => Ok(MainOperation::Operator(v)),
                Err(e) => Err(e)
            }
        }
    }

    fn get_var(&mut self) -> SyntaxResult<Vec<(u64, ProgramTypes)>> {
        let mut vars = Vec::new();
        
        self.next_token();
        match self.current_token {
            TokenGroup::Identifier(id) => {
                self.next_token();
                let mut temp_vars = Vec::new();
                while  {
                    
                }
            },
            TokenGroup::Delimiters(DelimitersGroup::Semicolon) => self.next_token(),
            _ => return Err(SyntaxError::Missing(self.current_token.clone(), "Ожидался идентификатор".to_string()))
        }
    }

    fn get_operator(&mut self) -> SyntaxResult<Operator> {
        todo!()
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