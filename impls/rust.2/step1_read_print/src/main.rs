use std::error::Error;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use libmal::printer::pr_str;
use libmal::reader::read_str;
use libmal::{Form, ParseError};

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rep(line);
            }
            Err(ReadlineError::Interrupted) => std::process::exit(1),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("\x1b[1;31merror:\x1b[0m {}", err);
                std::process::exit(1);
            }
        }
    }
}

fn rep(input: String) {
    if let Err(e) = try_rep(input) {
        println!("\x1b[1;31merror:\x1b[0m {}", e);
    }
}

fn try_rep(input: String) -> Result<(), Box<dyn Error>> {
    println!("{}", print(eval(read(input)?)?));
    Ok(())
}

fn read(input: String) -> Result<Form, ParseError> {
    read_str(&input)
}

fn eval(input: Form) -> Result<Form, Box<dyn Error>> {
    Ok(input)
}

fn print(input: Form) -> String {
    pr_str(&input, true)
}
