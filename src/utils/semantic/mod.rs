use std::collections::HashMap;

use error::SemanticResult;

use super::structs::{program::{Expression, MainOperation, Operator}, types::ProgramTypes};

pub mod error;

pub struct Semantic {
    program: Vec<MainOperation>,
    identifiers: HashMap<u64, ProgramTypes>
}

impl Semantic {
    pub fn new(program: Vec<MainOperation>) -> Self {
        Self {
            program,
            identifiers: HashMap::new()
        }
    }

    pub fn run_process(&mut self) -> SemanticResult<()> {
        for main_operation in self.program.clone() {
            match main_operation {
                MainOperation::CreateVariable(ident_vec) =>
                    for (identifiers, identifiers_type) in ident_vec {
                        for id in identifiers {
                            self.identifiers.insert(id, identifiers_type.clone());
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
        Ok(())
    }

    fn test_operator(&mut self, operator: Operator) -> SemanticResult<()> {
        match operator {
            Operator::Assignment(id, expression) => match self.test_expression(expression) {
                Ok(_) => todo!(),
                Err(e) => return Err(e)
            },
            _ => todo!()
        }
        Ok(())
    }

    fn test_expression(&mut self, expression: Expression) -> SemanticResult<ProgramTypes> {
        todo!()
    }
}