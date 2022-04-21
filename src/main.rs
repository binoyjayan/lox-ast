use std::io;
use std::fs;
use std::env;
use std::str;
use std::process;
use std::io::{Write, BufRead};

mod expr;
mod error;
mod token;
mod parser;
mod scanner;
mod ast_printer;
use error::LoxErr;
use parser::Parser;
use ast_printer::AstPrinter;

fn main() {
    run_main().expect("Failed to run program");
}

fn run_main() -> Result<(), LoxErr> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        run_file(&args[1]).unwrap();
    } else if args.len() == 1 {
        run_prompt()?;
    } else {
        println!("Usage: {} <lox-script>", args[0]);
        process::exit(1);
    }
    Ok(())
}

pub fn run_file(path: &String) -> io::Result<()> {
    let buf= fs::read_to_string(path)?;
    match run(buf.as_str()) {
        Ok(_) => {},
        Err(e) => {
            e.report("".to_string());
            process::exit(65);
        }
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), LoxErr> {
    let stdin = io::stdin();
    print!(">> ");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {        
        match line {
            Ok(line) => {
                match run(&line) {
                    Ok(_) => {},
                    Err(_e) => {} // already reported
                }
            },
            Err(_) => {}
        };
        print!(">> ");
        io::stdout().flush().unwrap();
    }
    println!("\nExiting...");
    Ok(())
}

fn run(source: &str) -> Result<(), LoxErr> {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Some(expr) => {
            let result = AstPrinter::default().print(&expr)?;
            println!("{}", result);
        },
        None => {}
    }
    Ok(())
}


