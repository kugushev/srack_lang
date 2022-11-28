extern crate core;

mod token;

use crate::token::Token;
use std::env;
use std::fs;
use rand;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].clone();
    print!("Parsing {}\n", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("File not found");

    let tokens = lexer(contents);

    executor(tokens)
}

fn lexer(source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    for line in source_code.lines() {
        let left = &line[0..4];
        let right = if line.len() > 4 { &line[5..] } else { "" };

        let token = match left {
            "seed" => Token::Seed(right.parse().unwrap()),
            "push" => Token::Push(right.to_string()),
            "poop" => Token::Poop,
            "call" => Token::Call(right.to_string()),
            _ => panic!("Parsing error")
        };

        result.push(token)
    }

    return result;
}

fn executor(tokens: Vec<Token>) {
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        match token {
            Token::Seed(value) => {}
            Token::Push(variable) => { stack.push(variable.clone()) }
            Token::Poop => { stack.pop(); }
            Token::Call(function) => { exec_call(&function, &mut stack) }
        }
    }
}

fn exec_call(func_name: &String, stack: &mut Vec<String>) {
    match func_name.as_str() {
        "print" => {
            let val = stack[stack.len() - 1].clone();
            print!("{}", val)
        }
        // "summ" => {
        //     let left = &stack[stack.len() - 2];
        //     let right = &stack[stack.len() - 1];
        // }
        _ => { panic!("Func not found") }
    }
}


