extern crate core;

use std::{env, io};
use std::fs;

mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Welcome to Srack Lang!
        Please specify source filename, e.g. 'srack_lang main.srack'
        Specify -v after filename to get verbose stdout");
        return;
    }

    let file_name = &args[1].clone();

    let verbose = if args.len() >= 3 && &args[2].clone() == "-v" {
        print!("Parsing {}\n", file_name);
        true
    } else { false };


    let contents = fs::read_to_string(file_name)
        .expect("File not found");
    interpreter::run(contents, verbose, &mut io::stdout())
}


