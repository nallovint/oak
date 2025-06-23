// REPL (Read-Eval-Print Loop)

use std::io::{self};

pub fn start_repl() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();
        if let Err(e) = stdin.read_line(input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        if input.trim() == "exit" {
            std::process::exit(0);
        } else {
            println!("{}", input);
        }
    }
}
