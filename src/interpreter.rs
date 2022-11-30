use std::io::Write;
use crate::interpreter::srack::Srack;
use crate::interpreter::srack_value::SrackValue;
use crate::interpreter::token::Token;

mod srack;
mod srack_value;
mod token;

pub fn run(source_code: String, verbose: bool, stdout: &mut dyn Write) {
    let tokens = lexer(source_code);
    executor(tokens, verbose, stdout)
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

fn executor(tokens: Vec<Token>, verbose: bool, stdout: &mut dyn Write) {
    let mut srack = Srack::new(verbose);

    for token in tokens {
        match token {
            Token::Seed(value) => { srack.init(value); }
            Token::Push(variable) => { srack.push(SrackValue(variable), stdout) }
            Token::Poop => { srack.poop(stdout); }
            Token::Call(function) => { exec_call(&function, &mut srack, stdout) }
        }
    }
}

fn exec_call(func_name: &String, srack: &mut Srack, stdout: &mut dyn Write) {
    match func_name.as_str() {
        "println" => {
            let to_print = srack.peek(0);

            match to_print {
                None => { panic!("Srack is empty on offset 0") }
                Some(val) => {
                    writeln!(stdout, "{}", &val).ok();
                }
            }
        }
        "+" => {
            do_binary_math(srack, |l, r| l + r, stdout);
        }
        "-" => {
            do_binary_math(srack, |l, r| l - r, stdout);
        }
        "*" => {
            do_binary_math(srack, |l, r| l * r, stdout);
        }
        "/" => {
            do_binary_math(srack, |l, r| l * r, stdout);
        }
        _ => { panic!("Func not found") }
    }
}

fn do_binary_math(srack: &mut Srack, f: fn(i64, i64) -> i64, stdout: &mut dyn Write) {
    let left = srack.peek(1);
    let right = srack.peek(0);

    let left_int = parse_int(left);
    let right_int = parse_int(right);

    let result = f(left_int, right_int);
    srack.push(SrackValue(result.to_string()), stdout)
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


