use std::env;
use std::format;
use std::fs;
use std::io::{self, Write};
use std::process::{exit};

fn run(_script: &str) {

}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect(&format!("Could not read file: {}", path));

    run(&contents)
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
