extern crate core;

mod token;
mod srack;
mod srack_value;

use crate::token::Token;
use std::env;
use std::fs;
use crate::srack::Srack;
use crate::srack_value::SrackValue;

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
        if line.len() == 0 { continue; }

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
            Token::Push(variable) => { srack.push(SrackValue(variable)) }
            Token::Poop => { srack.poop(); }
            Token::Call(function) => { exec_call(&function, &mut srack) }
        }
    }
}

fn exec_call(func_name: &String, srack: &mut Srack) {
    match func_name.as_str() {
        "println" => {
            let to_print = srack.peek(0);

            match to_print {
                None => { panic!("Srack is empty on offset 0") }
                Some(val) => { println!("{}", &val) }
            }
        }
        "+" => {
            do_binary_math(srack, |l, r| l + r);
        }
        "-" => {
            do_binary_math(srack, |l, r| l - r);
        }
        "*" => {
            do_binary_math(srack, |l, r| l * r);
        }
        "/" => {
            do_binary_math(srack, |l, r| l * r);
        }
        _ => { panic!("Func not found") }
    }
}

fn do_binary_math(srack: &mut Srack, f: fn(i64, i64) -> i64) {
    let left = srack.peek(1);
    let right = srack.peek(0);

    let left_int = parse_int(left);
    let right_int = parse_int(right);

    let result = f(left_int, right_int);
    srack.push(SrackValue(result.to_string()))
}

fn parse_int(value: Option<&SrackValue>) -> i64 {
    match value {
        None => { panic!("No value in stack for left") }
        Some(l) => {
            match l.0.parse::<i64>() {
                Ok(v) => { v }
                Err(_) => { panic!("Can't parse left value") }
            }
        }
    }
}


