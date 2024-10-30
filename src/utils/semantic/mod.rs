use std::collections::HashMap;

use error::{SemanticError, SemanticResult};

use crate::utils::structs::types::AdditionOperations;

use super::structs::{program::{Expression, MainOperation, Multiplier, Operand, Operator, Term}, types::{LexerDigitalData, MultiplicationOperations, ProgramTypes, RelationOperations}};

pub mod error;

pub struct Semantic {
    program: Vec<MainOperation>,
    identifiers: HashMap<u64, ProgramTypes>,
    pub reserve: u64,
    vars: HashMap<u64, LexerDigitalData>,
    pub asm: Vec<u8>,
    position: u64,
    pub asm_idents: Vec<(u64, u64, bool)>,
}

impl Semantic {
    pub fn new(program: Vec<MainOperation>, vars: HashMap<u64, LexerDigitalData>, reserv: u64) -> Self {
        Self {
            program,
            identifiers: HashMap::new(),
            vars,
            asm: Vec::new(),
            position: 0,
            reserve: reserv,
            asm_idents: Vec::new()
        }
    }

    pub fn run_process(&mut self) -> SemanticResult<()> {
        self.asm.append(&mut vec![0x48, 0x83, 0xec, 0x08]); // sub rsp, 8
        self.cur_pos();

        for main_operation in self.program.clone() {
            match main_operation {
                MainOperation::CreateVariable(ident_vec) =>
                    for (identifiers, identifiers_type) in ident_vec {
                        for id in identifiers {
                            match self.identifiers.get(&id) {
                                None => self.identifiers.insert(id, identifiers_type.clone()),
                                Some(_) => return Err(SemanticError::IdentifierAlreadyDeclared(id))
                            };
                        }
                    },
                MainOperation::Operator(operator) => {
                    match self.test_operator(operator) {
                        Ok(_) => (),
                        Err(e) => return Err(e.clone())
                    }
                }
            }
        }

        self.asm.append(&mut vec![0xb8, 0x3c, 0x00, 0x00, 0x00, 0x48, 0x31, 0xff, 0x0f, 0x05]);
        self.cur_pos();

        Ok(())
    }

    fn test_operator(&mut self, operator: Operator) -> SemanticResult<()> {
        match operator {
            Operator::Assignment(id, expression) => match self.test_expression(expression) {
                Ok(t) => match self.identifiers.get(&id) {
                    Some(v) => match t.clone() & v.clone() {
                        true => {
                            match v {
                                ProgramTypes::Boolean(_) => {
                                    let i_type = ProgramTypes::Boolean(Some(false));
                                    self.identifiers.insert(id, i_type.clone());
                                },
                                ProgramTypes::Float(_) => {
                                    let i_type = ProgramTypes::Float(Some(0.));
                                    self.identifiers.insert(id, i_type.clone());
                                },
                                ProgramTypes::Integer(_) => {
                                    let i_type = ProgramTypes::Integer(Some(0));
                                    self.identifiers.insert(id, i_type.clone());
                                },
                            }
                            self.assign(id);
                            Ok(())
                        },
                        _ => return Err(SemanticError::AssignError(t, v.clone()))
                    },
                    None => return Err(SemanticError::NotDefined(id))
                },
                Err(e) => return Err(e)
            },
            Operator::Composite(operators) => {
                for operator in operators {
                    self.test_operator(operator)?;
                }
                return Ok(())
            },
            Operator::Output(expressions) => {
                for expression in expressions {
                    let p_type = self.test_expression(expression)?;
                    self.print(p_type);
                }
                return Ok(())
            },
            Operator::Input(ids) => {
                for id in ids {
                    match self.identifiers.get(&id) {
                        Some(v) => {
                            match v {
                                ProgramTypes::Boolean(_) => {
                                    let i_type = ProgramTypes::Boolean(Some(false));
                                    self.identifiers.insert(id, i_type.clone());
                                    self.input(id, i_type);
                                },
                                ProgramTypes::Float(_) => {
                                    let i_type = ProgramTypes::Float(Some(0.));
                                    self.identifiers.insert(id, i_type.clone());
                                    self.input(id, i_type);
                                },
                                ProgramTypes::Integer(_) => {
                                    let i_type = ProgramTypes::Integer(Some(0));
                                    self.identifiers.insert(id, i_type.clone());
                                    self.input(id, i_type);
                                },
                            }
                        },
                        None => return Err(SemanticError::NotDefined(id))
                    };
                };
                return Ok(())
            },
            Operator::If(expression, operator1, operator2) => {
                let p_type = self.test_expression(expression)?;
                match p_type {
                    ProgramTypes::Boolean(_) => (),
                    t => return Err(SemanticError::NotBoolean(t))
                }

                let jz_position = self.jz_default();
                self.test_operator(*operator1)?;

                match operator2 {
                    Some(operator2) => {
                        let jmp_position = self.jpm_default();
                        self.jz(jz_position);
                        self.test_operator(*operator2)?;
                        self.jmp(jmp_position);
                    },
                    None => {
                        self.jz(jz_position);
                    }
                }

                return Ok(())
            }
            Operator::For(expressions, operator) => {
                let start_position = self.position;
                let mut expressions = expressions.iter();

                if expressions.len() == 0 {
                    self.asm_bool(true);
                } else {
                    let p_type = self.test_expression(expressions.next().unwrap().clone())?;
                    match p_type {
                        ProgramTypes::Boolean(_) => (),
                        t => return Err(SemanticError::NotBoolean(t))
                    };

                    for expression in expressions {
                        self.push_rax();
                        let p_type = self.test_expression(expression.clone())?;

                        match p_type {
                            ProgramTypes::Boolean(_) => (),
                            t => return Err(SemanticError::NotBoolean(t))
                        };

                        self.pop_rbx();
                        self.cmp();
                        self.and();
                    }
                }
                let jz_position = self.jz_default();
                self.test_operator(*operator)?;
                self.jmp_cycle(start_position);
                self.jz(jz_position);
                return Ok(())
            },
            Operator::While(expression, operator) => {
                let start_position = self.position;
                let p_type = self.test_expression(expression.clone())?;
                match p_type {
                    ProgramTypes::Boolean(_) => (),
                    t => return Err(SemanticError::NotBoolean(t))
                };
                let jz_position = self.jz_default();
                self.test_operator(*operator)?;
                self.jmp_cycle(start_position);
                self.jz(jz_position);
                return Ok(())
            }
        }
    }

    fn test_expression(&mut self, expression: Expression) -> SemanticResult<ProgramTypes> {
        let Expression {operands, operations} = expression;
        let (mut operands, operations) = (operands.into_iter(), operations.into_iter());
        let op1 = self.test_operand(operands.next().unwrap())?;
        let mut current_type = op1;

        for (operand, operation) in operands.zip(operations) {
            self.push_rax();
            let op = self.test_operand(operand)?;

            match op {
                ProgramTypes::Boolean(_) => match operation {
                    RelationOperations::Equal|RelationOperations::NotEqual => (),
                    o => return Err(SemanticError::InvalidOperation(op, o.into()))
                },
                _ => ()
            }

            self.pop_rbx();
            
            if !(current_type.clone() & op.clone()) {
                return Err(SemanticError::TypeError(op, current_type));
            }

            current_type = ProgramTypes::Boolean(None);

            match current_type {
                ProgramTypes::Float(_) => {
                    self.init_fpu();
                    self.fcomi();
                },
                _ => self.cmp()
            }
            match operation {
                RelationOperations::Equal => self.eq(),
                RelationOperations::NotEqual => self.neq(),
                RelationOperations::Greater => self.more(),
                RelationOperations::GreaterEqual => self.more_eq(),
                RelationOperations::Less => self.less(),
                RelationOperations::LessEqual => self.less_eq()
            }
        }

        Ok(current_type)
    }

    fn test_operand(&mut self, operand: Operand) -> SemanticResult<ProgramTypes> {
        let Operand { terms, operations } = operand;
        let (mut terms, operations) = (terms.into_iter(), operations.into_iter());
        let op1 = self.test_term(terms.next().unwrap())?;

        for (term, operation) in terms.zip(operations) {
            self.push_rax();
            let op = self.test_term(term)?;

            match op {
                ProgramTypes::Boolean(_) => match operation {
                    AdditionOperations::Or => (),
                    o => return Err(SemanticError::InvalidOperation(op, o.to_string())),
                },
                _ => match operation {
                    AdditionOperations::Or => return Err(SemanticError::InvalidOperation(op, AdditionOperations::Or.to_string())),
                    _ => ()
                }
            }

            self.pop_rbx();
            
            if !(op.clone() & op1.clone()) {
                return Err(SemanticError::TypeError(op, op1));
            }

            match op {
                ProgramTypes::Float(_) => {
                    self.init_fpu();
                    match operation {
                        AdditionOperations::Subtraction => self.sub_fpu(),
                        _ => self.add_fpu(),
                    }
                    self.save_fpu_rax();
                },
                _ => match operation {
                    AdditionOperations::Or => self.or(),
                    AdditionOperations::Addition => self.add_i64(),
                    AdditionOperations::Subtraction => self.sub_i64()
                }
            }
        }

        Ok(op1)
    }

    fn test_term(&mut self, term: Term) -> SemanticResult<ProgramTypes> {
        let Term { multipliers, operations } = term;
        let (mut multipliers, operations) = (multipliers.into_iter(), operations.into_iter());
        let op1  = self.test_multiplier(multipliers.next().unwrap())?;

        for (multiplier, operation) in multipliers.zip(operations) {
            self.push_rax();
            let op = self.test_multiplier(multiplier)?;

            match op {
                ProgramTypes::Boolean(_) => match operation {
                    MultiplicationOperations::And => (),
                    o => return Err(SemanticError::InvalidOperation(op, o.to_string())),
                },
                _ => match operation {
                    MultiplicationOperations::And => return Err(SemanticError::InvalidOperation(op, MultiplicationOperations::And.to_string())),
                    _ => ()
                }
            }

            self.pop_rbx();
            
            if !(op.clone() & op1.clone()) {
                return Err(SemanticError::TypeError(op, op1));
            }

            match op {
                ProgramTypes::Float(_) => {
                    self.init_fpu();
                    match operation {
                        MultiplicationOperations::Division => self.div_fpu(),
                        _ => self.mul_fpu(),
                    }
                    self.save_fpu_rax();
                },
                _ => match operation {
                    MultiplicationOperations::And => self.and(),
                    MultiplicationOperations::Multiplication => self.mul_i64(),
                    MultiplicationOperations::Division => self.div_i64()
                }
            }
        }

        Ok(op1)
    }

    fn test_multiplier(&mut self, multiplier: Multiplier) -> SemanticResult<ProgramTypes> {
        match multiplier  {
            Multiplier::Identifier(id) => {
                self.mov_rax_ident(id);
                match self.identifiers.get(&id) {
                    Some(v) => self.test_ident(id, v.clone()),
                    None => Err(SemanticError::NotDefined(id))
                }
            },
            Multiplier::Boolean(b) => {
                self.asm_bool(b);
                Ok(ProgramTypes::Boolean(None))
            },
            Multiplier::Variable(id) => {
                let var = self.vars.get(&id).unwrap().clone();
                match var {
                    LexerDigitalData::Float(v) => {
                        self.mov_rax_f64(v);
                        Ok(ProgramTypes::Float(None))
                    },
                    LexerDigitalData::Integer(v) => {
                        self.mov_rax_i64(v);
                        Ok(ProgramTypes::Integer(None))
                    }
                }
            }
            Multiplier::Expression(e) => self.test_expression(e),
            Multiplier::Not(m) => {
                let res = self.test_multiplier(*m);
                if let Ok(res) = res {
                    match res {
                        ProgramTypes::Boolean(_) => (),
                        t => return Err(SemanticError::InvalidOperation(t, "унарная".into()))
                    }
                }
                self.not_rax();
                Ok(ProgramTypes::Boolean(None))
            }
        }
    }

    fn test_ident(&mut self, id: u64, t: ProgramTypes) -> SemanticResult<ProgramTypes> {
        match t {
            ProgramTypes::Boolean(None) => Err(SemanticError::NotDefined(id)),
            ProgramTypes::Float(None) => Err(SemanticError::NotDefined(id)),
            ProgramTypes::Integer(None) => Err(SemanticError::NotDefined(id)),
            _ => Ok(t)
        }
    }
}

impl Semantic {
    fn cur_pos(&mut self) {
        self.position = self.asm.len() as u64;
    }

    fn input(&mut self, id: u64, i_type: ProgramTypes) {
        self.asm.append(&mut vec![0x48, 0xbf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        match i_type {
            ProgramTypes::Float(_) => self.asm_idents.push((self.reserve+2, self.position - 8, true)),
            _ => self.asm_idents.push((self.reserve+1, self.position - 8, true))
        }
        self.asm.append(&mut vec![0x48, 0xbe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((id, self.position - 8, true));
        self.asm.append(&mut vec![0xe8, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve+5, self.position - 4, false));
        if i_type & ProgramTypes::Boolean(None) {
            self.mov_rax_ident(id);
            self.asm.append(&mut vec![0x48, 0x85, 0xc0, 0x74, 12]);
            self.asm_bool(true);
            self.asm.append(&mut vec![0xeb, 10]);
            self.asm_bool(false);
            self.assign(id);
        }
    }

    fn assign(&mut self, id: u64) {
        self.asm.append(&mut vec![0x48, 0x89, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((id, self.position - 4, false));
    }

    fn print(&mut self, p_type: ProgramTypes) {
        self.asm.append(&mut vec![0x48, 0xbf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        match p_type {
            ProgramTypes::Float(_) => {
                self.asm_idents.push((self.reserve+4, self.position - 8, true));
                self.asm.append(&mut vec![0x48, 0xbe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
                self.assign(self.reserve);
                self.asm.append(&mut vec![0xb8, 0x01, 0x00, 0x00, 0x00, 0xf2, 0x0f, 0x10, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
                self.cur_pos();
                self.asm_idents.push((self.reserve, self.position - 4, false));
            },
            _ => {
                self.asm_idents.push((self.reserve+3, self.position - 8, true));
                self.asm.append(&mut vec![0x48, 0x89, 0xc6, 0x48, 0x31, 0xc0]);
            }
        }
        self.asm.append(&mut vec![0xe8, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve+6, self.position - 4, false));
    }

    fn mov_rax_f64(&mut self, num: f64) {
        self.asm.append(&mut vec![0x48, 0xb8]);
        self.asm.append(&mut num.to_le_bytes().to_vec());
        self.cur_pos();
    }

    fn mov_rax_i64(&mut self, num: i64) {
        self.asm.append(&mut vec![0x48, 0xb8]);
        self.asm.append(&mut num.to_le_bytes().to_vec());
        self.cur_pos();
    }

    fn not_rax(&mut self) {
        self.asm.append(&mut vec![0x48, 0xf7, 0xd0]);
        self.cur_pos();
    }

    fn push_rax(&mut self) {
        self.asm.push(0x50);
        self.cur_pos();
    }

    fn pop_rbx(&mut self) {
        self.asm.append(&mut vec![0x5b, 0x48, 0x93]);
        self.cur_pos();
    }

    fn and(&mut self) {
        self.asm.append(&mut vec![0x48, 0x21, 0xd8]);
        self.cur_pos();
    }

    fn or(&mut self) {
        self.asm.append(&mut vec![0x48, 0x09, 0xd8]);
        self.cur_pos();
    }

    fn init_fpu(&mut self) {
        self.asm.append(&mut vec![0x9b, 0xdb, 0xe3]);
        self.asm.append(&mut vec![0x48, 0x89, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
        self.asm.append(&mut vec![0xdd, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
        self.asm.append(&mut vec![0x48, 0x89, 0x1c, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn add_fpu(&mut self) {
        self.asm.append(&mut vec![0xdc, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn mul_fpu(&mut self) {
        self.asm.append(&mut vec![0xdc, 0x0c, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn div_fpu(&mut self) {
        self.asm.append(&mut vec![0xdc, 0x34, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn save_fpu_rax(&mut self) {
        self.asm.append(&mut vec![0xdd, 0x1c, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
        self.asm.append(&mut vec![0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn sub_fpu(&mut self) {
        self.asm.append(&mut vec![0xdc, 0x24, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
    }

    fn add_i64(&mut self) {
        self.asm.append(&mut vec![0x48, 0x01, 0xd8]);
        self.cur_pos();
    }

    fn sub_i64(&mut self) {
        self.asm.append(&mut vec![0x48, 0x29, 0xd8]);
        self.cur_pos();
    }

    fn mul_i64(&mut self) {
        self.asm.append(&mut vec![0x48, 0xf7, 0xeb]);
        self.cur_pos();
    }

    fn div_i64(&mut self) {
        self.asm.append(&mut vec![0x48, 0xf7, 0xfb]);
        self.cur_pos();
    }

    fn mov_rax_ident(&mut self, id: u64) {
        self.asm.append(&mut vec![0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((id, self.position - 4, false));
    }

    fn asm_bool(&mut self, b: bool) {
        self.mov_rax_i64(if b {-1} else {0});
    }

    fn jmp_cycle(&mut self, pos: u64) {
        self.asm.push(0xe9);
        let offset = (pos as i64 - self.position as i64 - 5) as i32;
        self.asm.append(&mut offset.to_le_bytes().to_vec());
        self.cur_pos();
    }

    fn jpm_default(&mut self) -> u64 {
        self.asm.append(&mut vec![0xe9, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.position - 4
    }

    fn jmp(&mut self, jmp_pos: u64) {
        let offset = (self.position - jmp_pos - 4) as i32;
        let bytes = offset.to_le_bytes();
        for i in 0..4 {
            self.asm[jmp_pos as usize + i] = bytes[i];
        }
    }

    fn jz_default(&mut self) -> u64 {
        self.asm.append(&mut vec![0x48, 0x85, 0xc0, 0x0f, 0x84, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.position - 4
    }

    fn jz(&mut self, jz_pos: u64) {
        let offset = (self.position - jz_pos - 4) as i32;
        let bytes = offset.to_le_bytes();
        for i in 0..4 {
            self.asm[jz_pos as usize + i] = bytes[i];
        }
    }

    fn cmp(&mut self) {
        self.asm.append(&mut vec![0x48, 0x39, 0xd8]);
        self.cur_pos();
    }

    fn fcomi(&mut self) {
        self.init_fpu();
        self.asm.append(&mut vec![0xdd, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00]);
        self.cur_pos();
        self.asm_idents.push((self.reserve, self.position - 4, false));
        self.asm.append(&mut vec![0xdb, 0xf1]);
        self.cur_pos();
    }

    fn eq(&mut self) {
        self.asm.append(&mut vec![0x74, 12]);
        self.jss_base();
    }

    fn neq(&mut self) {
        self.asm.append(&mut vec![0x75, 12]);
        self.jss_base();
    }

    fn more(&mut self) {
        self.asm.append(&mut vec![0x7f, 12]);
        self.jss_base();
    }

    fn less(&mut self) {
        self.asm.append(&mut vec![0x7c, 12]);
        self.jss_base();
    }

    fn more_eq(&mut self) {
        self.asm.append(&mut vec![0x7d, 12]);
        self.jss_base();
    }

    fn less_eq(&mut self) {
        self.asm.append(&mut vec![0x7e, 12]);
        self.jss_base();
    }

    fn jss_base(&mut self) {
        self.asm_bool(false);
        self.asm.append(&mut vec![0xeb, 10]);
        self.asm_bool(true);
        self.cur_pos();
    }

}
