mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use std::env;
use std::fs;
use std::process;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn run_source(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&statements)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Использование: cargo run -- <путь_к_файлу.ml>");
        process::exit(1);
    }

    let file_path = &args[1];
    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Не удалось прочитать файл '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    if let Err(err) = run_source(&source) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
