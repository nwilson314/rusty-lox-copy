mod lox;
mod token_type;

use std::env;
use std::process;

use crate::lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() > 2 {
        eprintln!("Usage: cargo run [script]");
        process::exit(1);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}





