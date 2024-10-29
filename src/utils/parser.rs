use std::{collections::HashMap, fs};

use crate::utils::{lexer::Lexer, structs::tokens::TokenGroup};

use super::{structs::{program::MainOperation, tokens::Token, types::LexerDigitalData}, syntax::Syntax};

#[derive(Debug, Clone)]
pub struct Parser {
    path: String,
    lexer: Lexer,
    syntax: Syntax,

    pub tokens: Vec<Token>,
    last_ident: u64,
    last_var: u64,

    pub program: Vec<MainOperation>,

    pub ident_map: HashMap<String, u64>,
    pub var_map: HashMap<String, u64>,

    pub vars: HashMap<u64, LexerDigitalData>,
}

impl Parser {
    pub fn new(path: impl Into<String>) -> Self {
        let path: String = path.into();
        let buf = fs::read_to_string(path.clone()).expect("Не удалось прочитать файл");
        let lexer = Lexer::new(buf.chars().collect::<Vec<char>>(), path.clone());
        let syntax = Syntax::new();
        Self {
            path,
            lexer,
            syntax,
            tokens: Vec::new(),
            last_ident: 0,
            last_var: 0,
            program: Vec::default(),
            ident_map: HashMap::new(),
            var_map: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn run_lexer(&mut self) -> Result<(), ()> {
        use crate::utils::structs::tokens::TokenGroupLexer;
        
        let mut has_illegals = false;

        loop {
            let token = self.lexer.next_token();
            match token {
                Ok((token, line, column)) => {
                    match token.clone() {
                        TokenGroupLexer::Illegal(i) => {
                            println!("Обнаружен непредвиденный символ: {} в {}:{}:{}", i.0, self.path, i.1, i.2);
                            has_illegals = true;
                        },
                        TokenGroupLexer::Keywords(v) => self.tokens.push(Token {
                            token: TokenGroup::Keywords(v),
                            line,
                            column
                        }),
                        TokenGroupLexer::Delimiters(v) => self.tokens.push(Token {
                            token: TokenGroup::Delimiters(v),
                            line,
                            column
                        }),
                        TokenGroupLexer::Identifier(v) => match self.ident_map.get(&v) {
                            Some(v) => self.tokens.push(Token {
                                token: TokenGroup::Identifier(*v),
                                line,
                                column
                            }),
                            None => {
                                self.ident_map.insert(v, self.last_ident);
                                self.tokens.push(Token {
                                    token: TokenGroup::Identifier(self.last_ident),
                                    line,
                                    column
                                });
                                self.last_ident += 1;
                            }
                        },
                        TokenGroupLexer::Variables(v) => match self.var_map.get(&v) {
                            Some(v) => self.tokens.push(Token {
                                token: TokenGroup::Variables(*v),
                                line,
                                column
                            }),
                            None => {
                                self.var_map.insert(v, self.last_var);
                                self.tokens.push(Token {
                                    token: TokenGroup::Variables(self.last_var),
                                    line,
                                    column
                                });
                                self.last_var += 1;
                            }
                        },
                        TokenGroupLexer::Eof => {
                            self.tokens.push(Token {
                                token: TokenGroup::Eof,
                                line,
                                column
                            });
                            break;
                        },
                    };
                },
                Err(e) => return Err(println!("{e}")),
            }
        }

        for var in self.var_map.clone() {
            let last_char = var.0.chars().last().unwrap();
            match last_char {
                'B'|'b' => {
                    let digit = &var.0[..var.0.len()-1];
                    let digit = i64::from_str_radix(digit, 2).unwrap();
                    self.vars.insert(var.1, LexerDigitalData::Integer(digit));
                },
                'O'|'o' => {
                    let digit = &var.0[..var.0.len()-1];
                    let digit = i64::from_str_radix(digit, 8).unwrap();
                    self.vars.insert(var.1, LexerDigitalData::Integer(digit));
                },
                'H'|'h' => {
                    let digit = &var.0[..var.0.len()-1];
                    let digit = i64::from_str_radix(digit, 16).unwrap();
                    self.vars.insert(var.1, LexerDigitalData::Integer(digit));
                },
                'D'|'d' => {
                    let digit = &var.0[..var.0.len()-1];
                    let digit = digit.parse().unwrap();
                    self.vars.insert(var.1, LexerDigitalData::Integer(digit));
                },
                _ => {
                    let digits = var.0;
                    if digits.contains('.') || digits.contains('e') || digits.contains('E') {
                        let digit = digits.parse().unwrap();
                        self.vars.insert(var.1, LexerDigitalData::Float(digit));
                    } else {
                        let digit = digits.parse().unwrap();
                        self.vars.insert(var.1, LexerDigitalData::Integer(digit));
                    }
                }
            }
        };
        if has_illegals {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn run_syntax(&mut self) -> Result<(), ()> {
        let res = self.syntax.run_process(
            self.tokens.clone(),
            self.vars.clone()
        );
        
        use crate::utils::syntax::error::SyntaxError;
        match res {
            Ok(data) => {
                self.program = data;
                println!("{:#?}", self.program);
                Ok(())
            },
            Err(e) => match e {
                SyntaxError::Missing(token, text) => {
                    let (line, column) = match self.tokens.get(self.syntax.position-1) {
                        Some(Token { line, column, .. }) => (*line, *column),
                        None => (0, 0)
                    };

                    return Err(println!(
                        "Синтаксический анализатор сообщает: {}, а встречена лексема: {:?} по адресу {}:{}:{}",
                        text,
                        token.token,
                        self.path,
                        line,
                        column
                    ))
                },
                SyntaxError::Error(text) => 
                    return Err(println!(
                        "Синтаксический анализатор сообщает: {}",
                        text
                    ))
            }
        }
    }

}