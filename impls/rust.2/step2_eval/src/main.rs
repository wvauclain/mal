use std::collections::HashMap;
use std::error::Error;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use libmal::Object;

mod builtins;
mod eval;
mod print;
mod read;

type Environment = HashMap<String, Object>;

fn main() {
    let mut rl = Editor::<()>::new();

    let mut environment: HashMap<String, Object> = vec![
        ("+", builtins::binary_operation(|x, y| x + y)),
        ("-", builtins::binary_operation(|x, y| x - y)),
        ("*", builtins::binary_operation(|x, y| x * y)),
        ("/", builtins::binary_operation(|x, y| x / y)),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.into()))
    .collect();

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rep(line, &mut environment);
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

fn rep(input: String, environment: &mut Environment) {
    if let Err(e) = try_rep(input, environment) {
        println!("\x1b[1;31merror:\x1b[0m {}", e);
    }
}

fn try_rep(input: String, environment: &mut Environment) -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        print::print(eval::eval(read::read(input)?, environment)?)
    );
    Ok(())
}
