use std::env;
use std::error::Error;
use std::fmt;
use std::format;
use std::fs;
use std::io::{self, Write};
use std::process::{exit};

mod token;
mod scanner;
mod grammar;

use crate::scanner::Scanner;
use crate::grammar::Visitor;

#[derive(Debug)]
pub struct ExecutionError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

pub struct ExecutionSuccess {}

impl ExecutionError {
    fn report(&self) -> String {
        format!("[line {}] Error {}: {}", self.line, self.location, self.message)
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.report())
    }
}

impl Error for ExecutionError {
}

type ExecutionResult = Result<ExecutionSuccess, ExecutionError>;

fn run(script: &str) -> ExecutionResult {
    let mut scanner = Scanner::new(script.to_string());
    let tokens = scanner.scan();

    println!("tokens: {:?}", tokens);
    println!("errors: {:?}", scanner.errors);
    Ok(ExecutionSuccess {})
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect(&format!("Could not read file: {}", path));

    match run(&contents) {
        Ok(_) => exit(1),
        Err(err) => {
            eprintln!("{}", err);
            exit(65)
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        if line.trim().len() == 0  { break }
        run(&line.trim());
    }
}

fn main() {
    // let first: grammar::Expr = grammar::Expr::Unary {
    //     operator: &token::Token {
    //         token_type: token::TokenType::Minus,
    //         lexeme: "-".to_string(),
    //         literal: token::Literal::None,
    //         line: 1
    //     },
    //     right: &grammar::Expr::Literal {
    //         value: &token::Literal::Number(123.0)
    //     },
    // };

    // let expr: grammar::Expr = grammar::Expr::Binary {
    //     left: &first,
    //     operator: token::Token {
    //         token_type: token::TokenType::Star,
    //         lexeme: "*".to_string(),
    //         literal: token::Literal::None,
    //         line: 1
    //     },
    //     right: &grammar::Expr::Grouping {
    //         expression: &grammar::Expr::Literal {
    //             value: &token::Literal::Number(45.67)
    //         }
    //     }
    // };

    // let mut printer: grammar::AstPrinter = grammar::AstPrinter {};

    // println!("{}", printer.visit(&expr));

    let args: Vec<String> = env::args().collect();
    println!("args {:?}", args);

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: jlox_tree [script]");
            exit(64)
        }
    }
}
