use std::collections::HashMap;

use crate::utils::structs::tokens::KeywordsGroup;

use super::structs::{program::{Expression, MainOperation, Multiplier, Operand, Operator, Term}, tokens::{DelimitersGroup, Token, TokenGroup}, types::{AdditionOperations, LexerDigitalData, MultiplicationOperations, ProgramTypes, RelationOperations}};

pub mod error;

use error::{SyntaxResult, SyntaxError};

#[derive(Debug, Clone)]
pub struct Syntax {
    current_token: Token,
    pub position: usize,

    tokens: Vec<Token>,
    vars: HashMap<u64, LexerDigitalData>
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            current_token: Token::eof(),
            position: 0,
            tokens: Vec::default(),
            vars: HashMap::default()
        }
    }

    pub fn run_process(
        &mut self,
        tokens: Vec<Token>,
        vars: HashMap<u64, LexerDigitalData>
    ) -> SyntaxResult<Vec<MainOperation>> {
        self.tokens = tokens;
        self.vars = vars;

        let mut main = Vec::new();
        self.read_token();

        match self.current_token.token {
            TokenGroup::Delimiters(DelimitersGroup::LeftCurlyBracket) => {
                self.read_token();
                while
                    self.current_token.token != TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket)
                    &&
                    self.current_token.token != TokenGroup::Eof
                {
                    let res = self.get_main();
                    if let Err(e) = res {
                        return Err(e);
                    }
                    main.push(res.unwrap());
                    match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::Semicolon) => self.read_token(),
                        _ => return Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидалось ';'".to_string()
                        ))
                    }
                }
                if self.current_token.token == TokenGroup::Eof {
                    return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалось '}'".to_string()
                    ));
                } else {
                    match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) =>
                            self.read_token(),
                        _ => return Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидалось '}'".to_string()
                        ))
                    }
                    if self.current_token.token != TokenGroup::Eof {
                        return Err(SyntaxError::Missing(self.current_token.clone(), "Ожидался конец программы".to_string()));
                    }
                }
            },
            _ => return Err(SyntaxError::Missing(self.current_token.clone(), "Программа должна начинаться с '{'".to_string()))
        }
        if main.len() == 0 {
            return Err(SyntaxError::Error("Ожидалось описание или оператор".to_string()));
        }
        return Ok(main);
    }

    fn get_main(&mut self) -> SyntaxResult<MainOperation> {
        match self.current_token.token {
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

    fn get_var(&mut self) -> SyntaxResult<Vec<(Vec<u64>, ProgramTypes)>> {
        let mut vars = Vec::new();
        self.read_token();
        
        if self.current_token.token == TokenGroup::Delimiters(DelimitersGroup::Semicolon) {
            return Ok(Vec::default());
        }

        let mut temp_vars = Vec::new();
        let mut comma = false;

        loop {
            match self.current_token.token {
                TokenGroup::Identifier(v) => {
                    temp_vars.push(v);
                    self.read_token();
                    match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::Comma) =>
                            comma = true,
                        TokenGroup::Delimiters(DelimitersGroup::Colon) => {
                            comma = false;
                            self.read_token();
                            match self.current_token.token {
                                TokenGroup::Keywords(KeywordsGroup::Integer) =>
                                    vars.push((temp_vars.clone(), ProgramTypes::Integer(None))),
                                TokenGroup::Keywords(KeywordsGroup::Real) =>
                                    vars.push((temp_vars.clone(), ProgramTypes::Float(None))),
                                TokenGroup::Keywords(KeywordsGroup::Boolean) =>
                                    vars.push((temp_vars.clone(), ProgramTypes::Boolean(None))),
                                _ => return Err(SyntaxError::Missing(
                                    self.current_token.clone(),
                                    "Ожидался тип данных".to_string()
                                ))
                            }
                            temp_vars.clear();
                            self.read_token();
                            match self.current_token.token {
                                TokenGroup::Delimiters(DelimitersGroup::Semicolon) => (),
                                _ => return Err(SyntaxError::Missing(
                                    self.current_token.clone(),
                                    "Ожидалась ';'".to_string()
                                ))
                            }
                        },
                        _ => return Err(SyntaxError::Error(
                            "Встречена неожиданная лексема".to_string()
                        ))
                    }
                },
                TokenGroup::Delimiters(DelimitersGroup::Semicolon)|
                TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) => match comma {
                    false => return Ok(vars),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидался идентификатор".to_string()
                    ))
                },
                _ => return Err(SyntaxError::Missing(self.current_token.clone(), "Ожидался идентификатор или ';'".to_string()))
            }
            self.read_token();
        }
    }

    fn get_operator(&mut self) -> SyntaxResult<Operator> {
        match self.current_token.token.clone() {
            TokenGroup::Delimiters(DelimitersGroup::LeftCurlyBracket) => {
                self.read_token();
                let mut operators = Vec::new();
                while
                    self.current_token.token != TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket)
                    &&
                    self.current_token.token != TokenGroup::Eof
                {
                    match self.get_operator() {
                        Ok(v) => operators.push(v),
                        Err(e) => return Err(e)
                    }
                    if self.current_token.token == TokenGroup::Delimiters(DelimitersGroup::Semicolon) {
                        match self.next_token().token {
                            TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) =>
                                return Err(SyntaxError::Error("Ожидался оператор".to_string())),
                            _ => self.read_token(),
                        }
                    } else if self.current_token.token != TokenGroup::Delimiters(DelimitersGroup::RightCurlyBracket) {
                        return Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидалось '}'".to_string()
                        ));
                    }
                }
                self.read_token();
                return  Ok(Operator::Composite(operators));
            },
            TokenGroup::Keywords(KeywordsGroup::Let) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Identifier(id) => {
                        self.read_token();
                        match self.current_token.token {
                            TokenGroup::Delimiters(DelimitersGroup::Equal) => {
                                self.read_token();
                                match self.get_expression() {
                                    Ok(v) => return Ok(Operator::Assignment(id, v)),
                                    Err(e) => return Err(e)
                                }
                            },
                            _ => return Err(SyntaxError::Missing(
                                self.current_token.clone(),
                                "Ожидался знак '='".to_string()
                            ))
                        }
                    },
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалось идентификатор".to_string()
                    ))
                }
            },
            TokenGroup::Identifier(id) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Delimiters(DelimitersGroup::Equal) => {
                        self.read_token();
                        match self.get_expression() {
                            Ok(v) => return Ok(Operator::Assignment(id, v)),
                            Err(e) => return Err(e)
                        }
                    },
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидался знак '='".to_string()
                    ))
                }
            },
            TokenGroup::Keywords(KeywordsGroup::If) => {
                self.read_token();
                let expression = match self.get_expression() {
                    Err(e) => return Err(e),
                    Ok(expression) => expression
                };
                match self.current_token.token {
                    TokenGroup::Keywords(KeywordsGroup::Then) => (),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалось 'then'".to_string()
                    ))
                }
                self.read_token();
                let operator1 = match self.get_operator() {
                    Ok(operator) => operator,
                    Err(e) => return Err(e),
                };
                let operator2 = match self.current_token.token {
                    TokenGroup::Keywords(KeywordsGroup::Else) => {
                        self.read_token();
                        match self.get_operator() {
                            Ok(operator2) => Some(Box::new(operator2)),
                            Err(e) => return Err(e)
                        }
                    },
                    _ => None
                };
                match self.current_token.token {
                    TokenGroup::Keywords(KeywordsGroup::EndElse) => (),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалось 'end_else'".to_string()
                    ))
                }
                self.read_token();
                Ok(Operator::If(
                    expression,
                    Box::new(operator1),
                    operator2
                ))
            }
            TokenGroup::Keywords(KeywordsGroup::For) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Delimiters(DelimitersGroup::LeftParenthesis) => self.read_token(),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалась '('".to_string()
                    ))
                };
                
                let mut expressions = Vec::with_capacity(3);
                match self.get_expression() {
                    Ok(v) => expressions.push(v),
                    Err(e) => return Err(e)
                };
                for _ in 1..3 {
                    match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::Semicolon) => self.read_token(),
                        _ => return Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидалась ';'".to_string()
                        ))
                    }
                    match self.get_expression() {
                        Ok(v) => expressions.push(v),
                        Err(e) => return Err(e)
                    };
                }
                
                match self.current_token.token {
                    TokenGroup::Delimiters(DelimitersGroup::RightParenthesis) =>
                        self.read_token(),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалась ')'".to_string()
                    ))
                };
                
                match self.get_operator() {
                    Ok(op) => Ok(Operator::For(
                        expressions[0].clone(),
                        expressions[1].clone(),
                        expressions[2].clone(),
                        Box::new(op)
                    )),
                    Err(e) => Err(e)
                }
            },
            TokenGroup::Keywords(KeywordsGroup::Do) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Keywords(KeywordsGroup::While) =>
                        self.read_token(),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалось ключевое слово 'while'".to_string()
                    ))
                };

                let expressions = match self.get_expression() {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };

                let operator = match self.get_operator() {
                    Ok(operator) => operator,
                    Err(e) => return Err(e)
                };
                
                match self.current_token.token {
                    TokenGroup::Keywords(KeywordsGroup::Loop) => {
                        self.read_token();
                        Ok(Operator::While(
                            expressions,
                            Box::new(operator)
                        ))
                    },
                    _ => Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалась 'loop'".to_string()
                    ))
                }
            },
            TokenGroup::Keywords(KeywordsGroup::Input) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Delimiters(DelimitersGroup::LeftParenthesis) =>
                        self.read_token(),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалась '('".to_string()
                    ))
                };
                let mut identifiers = Vec::new();

                loop {
                    match self.current_token.token.clone() {
                        TokenGroup::Delimiters(DelimitersGroup::RightParenthesis) => break,
                        TokenGroup::Identifier(id) => {
                            self.read_token();
                            identifiers.push(id);
                        },
                        _ => return Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидался идентификатор или ')'".to_string()
                        ))
                    };
                }

                if identifiers.len() == 0 {
                    return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидался идентификатор".to_string()
                    ));
                }

                self.read_token();
                Ok(Operator::Input(identifiers))
            },
            TokenGroup::Keywords(KeywordsGroup::Output) => {
                self.read_token();
                match self.current_token.token {
                    TokenGroup::Delimiters(DelimitersGroup::LeftParenthesis) =>
                        self.read_token(),
                    _ => return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидалась '('".to_string()
                    ))
                };

                let mut expressions = Vec::new();

                loop {
                    match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::RightParenthesis) =>
                            break,
                        _ => match self.get_expression() {
                            Ok(expression) => expressions.push(expression),
                            Err(e) => return  Err(e)
                        }
                    };
                }

                if expressions.len() == 0 {
                    return Err(SyntaxError::Missing(
                        self.current_token.clone(),
                        "Ожидался оператор".to_string()
                    ));
                }

                self.read_token();
                Ok(Operator::Output(expressions))
            },
            t => Err(SyntaxError::Error(format!("Встречена непредвиденная лексема {}", t)))
        }
    }

    fn get_expression(&mut self) -> SyntaxResult<Expression> {
        let mut operands = Vec::new();
        let mut operations = Vec::new();

        match self.get_operand() {
            Ok(operand) => operands.push(operand),
            Err(e) => return Err(e)
        }

        loop {
            match &self.current_token.token {
                TokenGroup::Delimiters(delimiter) => match delimiter {
                    DelimitersGroup::NotEqual =>
                        operations.push(RelationOperations::NotEqual),
                    DelimitersGroup::Identical =>
                        operations.push(RelationOperations::Equal),
                    DelimitersGroup::Less =>
                        operations.push(RelationOperations::Less),
                    DelimitersGroup::Greater =>
                        operations.push(RelationOperations::Greater),
                    DelimitersGroup::LessEqual =>
                        operations.push(RelationOperations::LessEqual),
                    DelimitersGroup::GreaterEqual =>
                        operations.push(RelationOperations::GreaterEqual),
                    _ => break
                },
                _ => break
            }
            self.read_token();
            match self.get_operand() {
                Ok(operand) => operands.push(operand),
                Err(e) => return Err(e)
            }
        }
        return Ok(Expression {
            operands,
            operations
        });
        
    }

    fn get_operand(&mut self) -> SyntaxResult<Operand> {
        let mut terms = Vec::new();
        let mut operations = Vec::new();

        match self.get_term() {
            Ok(operand) => terms.push(operand),
            Err(e) => return Err(e)
        }

        loop {
            match &self.current_token.token {
                TokenGroup::Delimiters(delimiter) => match delimiter {
                    DelimitersGroup::Plus =>
                        operations.push(AdditionOperations::Addition),
                    DelimitersGroup::Minus =>
                        operations.push(AdditionOperations::Subtraction),
                    DelimitersGroup::Or =>
                        operations.push(AdditionOperations::Or),
                    _ => break
                },
                _ => break
            }
            self.read_token();
            match self.get_term() {
                Ok(term) => terms.push(term),
                Err(e) => return Err(e)
            }
        }
        return Ok(Operand {
            terms,
            operations
        });
    }

    fn get_term(&mut self) -> SyntaxResult<Term> {
        let mut multipliers = Vec::new();
        let mut operations = Vec::new();

        match self.get_multiplier() {
            Ok(multiplier) => multipliers.push(multiplier),
            Err(e) => return Err(e)
        }

        loop {
            match &self.current_token.token {
                TokenGroup::Delimiters(delimiter) => match delimiter {
                    DelimitersGroup::Asterisk =>
                        operations.push(MultiplicationOperations::Multiplication),
                    DelimitersGroup::Slash =>
                        operations.push(MultiplicationOperations::Division),
                    DelimitersGroup::And =>
                        operations.push(MultiplicationOperations::And),
                    _ => break
                },
                _ => break
            }
            self.read_token();
            match self.get_multiplier() {
                Ok(multiplier) =>
                    multipliers.push(multiplier),
                Err(e) => return Err(e)
            }
        }
        return Ok(Term {
            multipliers,
            operations
        });
    }

    fn get_multiplier(&mut self) -> SyntaxResult<Multiplier> {
        let res = match &self.current_token.token {
            TokenGroup::Identifier(id) =>
                Ok(Multiplier::Identifier(*id)),
            TokenGroup::Variables(id) =>
                Ok(Multiplier::Variable(*id)),
            TokenGroup::Keywords(KeywordsGroup::True) =>
                Ok(Multiplier::Boolean(true)),
            TokenGroup::Keywords(KeywordsGroup::False) =>
                Ok(Multiplier::Boolean(false)),
            TokenGroup::Delimiters(DelimitersGroup::Not) => {
                self.read_token();
                match self.get_multiplier() {
                    Ok(multiplier) =>
                        return Ok(Multiplier::Not(Box::new(multiplier))),
                    Err(e) => Err(e)
                }
            },
            TokenGroup::Delimiters(DelimitersGroup::LeftParenthesis) => {
                self.read_token();
                match self.get_expression() {
                    Ok(expression) => match self.current_token.token {
                        TokenGroup::Delimiters(DelimitersGroup::RightParenthesis) => {
                            Ok(Multiplier::Expression(expression))
                        },
                        _ => Err(SyntaxError::Missing(
                            self.current_token.clone(),
                            "Ожидалась ')'".to_string()
                        ))
                    },
                    Err(e) => Err(e)
                }
            }
            _ => Err(SyntaxError::Missing(
                self.current_token.clone(),
                "Ожидался операнд".to_string()
            ))
        };
        if res.is_ok() {
            self.read_token();
        }

        res
    }

    fn read_token(&mut self) {
        if let Some(next) = self.tokens.get(self.position) {
            self.current_token = next.clone();
            self.position += 1;
        } else {
            self.current_token = self.tokens.last().unwrap_or_else(|| {
                &Token {
                    token: TokenGroup::Eof,
                    line: 0,
                    column: 0
                }
            }).clone();
            self.position += 1;
        }
    }

    fn next_token(&mut self) -> Token {
        match self.tokens.get(self.position) {
            Some(v) => v.clone(),
            None => self.tokens.last().unwrap_or_else(|| {
                &Token {
                    token: TokenGroup::Eof,
                    line: 0,
                    column: 0
                }
            }).clone()
        }
    }
}