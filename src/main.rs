mod lexer;
mod parser;
mod pair;

use std::*;
use lexer::{Token, Lexer, TokenType};
use parser::{Parser, AST};

fn print_command_usage(program: String) {
    eprintln!("Usage: {} <file>", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 {
        print_command_usage(program);
        return;
    }

    let file = fs::read_to_string(&args[1]).unwrap();
    //println!("File: {}", file);
    let tokens = Lexer::tokenize(file.to_string());
    //println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);
}
