use std::env;

mod tests;
mod utils;

use utils::parser::Parser;

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
    }
    if error {
        return;
    }
    let mut parser_structure = Parser::new(path);
    let res = parser_structure.run_lexer();
    if let Err(_) = res { return }
    let tokens = parser_structure.tokens.clone();
    for i in tokens {
        print!("{:?} ", i);
    }
    println!();
    println!("{:?}", parser_structure.ident_map);
    println!("{:?}", parser_structure.var_map);
    println!("{:?}", parser_structure.vars);
    if lexer_only { return }

    let res = parser_structure.run_syntax();
    if let Err(_) = res { return }
    if syntax { return }
    if sem { return }
}
