// REPL (Read-Eval-Print Loop)

use std::io::{self};

pub fn start_repl() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.clear();
        let _ = stdin.read_line(input);

        if input.trim() == "exit" {
            std::process::exit(0);
        } else {
            println!("{}", input);
        }
    }
}
