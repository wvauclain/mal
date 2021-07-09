use rustyline::error::ReadlineError;
use rustyline::Editor;

use libmal::printer::pr_str;
use libmal::reader::read_str;
use libmal::{Form, ParseError};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("\x1b[1;31merror:\x1b[0m {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), ParseError> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        // println!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let read_output = read(line);
                match read_output {
                    Ok(o) => println!("{}", print(eval(o))),
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();

    Ok(())
}

fn read(input: String) -> Result<Form, ParseError> {
    read_str(&input)
}

fn eval(input: Form) -> Form {
    input
}

fn print(input: Form) -> String {
    pr_str(&input, true)
}
