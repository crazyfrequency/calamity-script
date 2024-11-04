use std::env;

mod tests;
mod utils;

use utils::{elf::Elf, parser::Parser, semantic::{error::SemanticError, Semantic}, structs::types::LexerDigitalData};

fn main() {
    let (mut lexer_only, mut syntax, mut sem) = (false, false, false);
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut error = false;
    let mut path = String::new();
    let mut out_path = String::new();
    let mut lex_objects = false;
    let mut compact_mode = false;
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
            },
            "-lo" => {
                lex_objects = true;
                i+=1;
            },
            "-c" => {
                compact_mode = true;
                i+=1;
            },
            "-o" => match args.get(i+1) {
                Some(v) => {
                    out_path = v.clone();
                    i+=2;
                },
                None => {
                    println!("Не указан выходной файл");
                    return;
                }
            },
            "-h"|"--help" => {
                println!("Компилятор принимает следующие аргументы, последний аргумент имя файла");
                println!("\t-h | --help - отобразит текущее сообщение");
                println!("\t-l - остановиться после лексического анализа");
                println!("\t-lo - Выводит лексемы в виде объектов");
                println!("\t-st - остановиться после синтаксического анализа");
                println!("\t-sem - остановиться после семантического анализа");
                println!("\t-c - Компактный режим");
                println!("\t-o - имя выходного объектного файла");
                return;
            }
            _ => {
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
    if out_path.is_empty() {
        out_path = path.clone() + ".o";
    }
    if error {
        println!("Неверные аргументы, доступные аргументы можно увидеть введя: --help");
        return;
    }
    let mut parser_structure = Parser::new(path);
    let res = parser_structure.run_lexer();
    if res.is_ok() {
        println!("Лексический анализ успешно выполнен.");
    } else {
        return;
    };
    let tokens = parser_structure.tokens.clone();
    if !compact_mode {
        for i in tokens {
            if lex_objects {
                println!("{:?}", i.token);
            } else {
                print!("{} ", i.token);
            }
        }
        println!();
        let mut identifiers = parser_structure.ident_map.clone().into_iter()
            .map(|(name, id)| (id, name)).collect::<Vec<(u64, String)>>();
        identifiers.sort_by(|a, b| a.0.cmp(&b.0));
        let mut variables = parser_structure.vars.clone()
            .into_iter().collect::<Vec<(u64, LexerDigitalData)>>();
        variables.sort_by(|a, b| a.0.cmp(&b.0));
        println!("|        Переменные        |           Числа          |");
        println!("|---|----------------------|----|---------------------|");
        for i in 0..identifiers.len().max(variables.len()) {
            println!("|{} | {}|", match identifiers.get(i) {
                Some((id, name)) => format!("{0: <2} | {1: <20}", id, name),
                None => format!("{0: <2} | {1: <20}", "", "")
            }, match variables.get(i) {
                Some((id, value)) => format!("{0: <2} | {1: <20}", id, value.to_string()),
                None => format!("{0: <2} | {1: <20}", "", "")
            });
        }
    }
    if lexer_only { return }

    let res = parser_structure.run_syntax(compact_mode);
    if res.is_ok() {
        println!("Синтаксический анализ успешно выполнен.");
    } else {
        return;
    };
    if syntax { return }
    let (inner_structure, vars, idents) = (parser_structure.program, parser_structure.vars, parser_structure.ident_map);
    let mut semantic = Semantic::new(inner_structure.clone(), vars.clone(), idents.len() as u64);
    let res = semantic.run_process();
    if let Err(e) = res {
        match e {
            SemanticError::AssignError(from, to) =>
                println!("Не удалось присвоить тип {} к {}", from, to),
            SemanticError::InvalidOperation(t, o) =>
                println!("Невозможно выполнить операцию {} над типом {}", o, t),
            SemanticError::NotDefined(id) =>
                println!("Переменная {} ещё не объявлена или не инициализирована", idents.iter().find(|(_, v)| **v==id).unwrap().0),
            SemanticError::TypeError(s, f) =>
                println!("Ошибка типов: невозможно выполнить операцию с {} и {}", f, s),
            SemanticError::NotBoolean(t) =>
                println!("В условии обнаружен недопустимый тип {}", t),
            SemanticError::IdentifierAlreadyDeclared(id) =>
                println!("Переменная {} уже объявлена", idents.iter().find(|(_, v)| **v==id).unwrap().0)
        }
        return
    }
    if !compact_mode {
        println!("{:?}", semantic.asm);
    };
    if res.is_ok() {
        println!("Семантический анализ успешно выполнен.");
    };
    if sem { return }
    let mut elf = Elf::new(out_path, idents.len() as u16, semantic.asm, semantic.asm_idents);
    let res = elf.process();
    if res.is_ok() {
        println!("Создание объектного файла успешно выполнено.");
    };
}
