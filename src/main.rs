extern crate core;

mod token;
mod srack;
mod srack_item;

use crate::token::Token;
use std::env;
use std::fs;
use crate::srack::Srack;
use crate::srack_item::SrackItem;

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
    let mut srack = Srack::new();

    for token in tokens {
        match token {
            Token::Seed(value) => { srack.init(value); }
            Token::Push(variable) => {
                // todo: parse string or int
                srack.push(SrackItem::Str(variable))
            }
            Token::Poop => { srack.poop(); }
            Token::Call(function) => { exec_call(&function, &srack) }
        }
    }
}

fn exec_call(func_name: &String, srack: &Srack) {
    match func_name.as_str() {
        "println" => {
            let to_print = srack.peek(0);

            match to_print {
                None => { panic!("Srack is empty on offset 0") }
                Some(val) => { println!("{}", &val) }
            }
        }
        // "summ" => {
        //     let left = &stack[stack.len() - 2];
        //     let right = &stack[stack.len() - 1];
        // }
        _ => { panic!("Func not found") }
    }
}


