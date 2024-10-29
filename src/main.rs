use std::env;

mod tests;
mod utils;

use utils::{elf::Elf, parser::Parser, semantic::{self, error::SemanticError, Semantic}};

fn main() {
    let (mut lexer_only, mut syntax, mut sem) = (false, false, false);
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut error = false;
    let mut path = String::new();
    let last = args.len() - 1;

    while i < args.len() {
        match args[i].as_str() {
            "-l" => {
                lexer_only = true;
                i+=1;
            },
            "-st" => {
                syntax = true;
                i+=1;
            },
            "-sem" => {
                sem = true;
                i+=1;
            }
            _    => {
                if i == last {
                    path = args[i].clone();
                    break;
                } else {
                    error = true;
                    break;
                }
            },
        }
    };
    if path.is_empty() {
        println!("No file specified");
        return;
        // path = String::from("test.cm");
    }
    if error {
        println!("Неверные аргументы, доступные аргументы: -l, -st, -sem");
        return;
    }
    let mut parser_structure = Parser::new(path.clone());
    let res = parser_structure.run_lexer();
    if let Err(_) = res { return }
    let tokens = parser_structure.tokens.clone();
    for i in tokens {
        print!("{} ", i.token);
    }
    println!();
    println!("{:?}", parser_structure.ident_map);
    println!("{:?}", parser_structure.var_map);
    println!("{:?}", parser_structure.vars);
    if lexer_only { return }

    let res = parser_structure.run_syntax();
    if let Err(_) = res { return }
    if syntax { return }
    let (inner_structure, vars, idents) = (parser_structure.program, parser_structure.vars, parser_structure.ident_map);
    let mut semantic = Semantic::new(inner_structure.clone(), vars.clone(), idents.len() as u64);
    let res = semantic.run_process();
    if let Err(e) = res {
        match e {
            SemanticError::AssignError(from, to) =>
                println!("Не удалось присвоить тип {} к {}", from, to),
            SemanticError::InvalidOperation(t, o) =>
                println!("Невозможно выполнить операцию {} на типом {}", o, t),
            SemanticError::NotDefined(id) =>
                println!("Переменная {} ещё не объявлена или не инициализирована", idents.iter().find(|(_, v)| **v==id).unwrap().0),
            SemanticError::TypeError(s, f) =>
                println!("Ошибка типов: невозможно выполнить операцию с {} и {}", f, s),
            SemanticError::Error(e) =>
                println!("{}", e)
        }
        return
    }
    println!("{:?}", semantic.asm);
    if sem { return }
    let mut elf = Elf::new(path+".o", idents.len() as u16, semantic.asm, semantic.asm_idents);
    let res = elf.process();
    println!("{:?}", res);
}
