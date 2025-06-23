// Script Runner
use crate::parser::{ScriptError, parse_script};

pub fn run(source: String) -> Result<(), ScriptError> {
    println!("Running script with Oak version 0.1.0...");

    let parsed_script: Result<(), ScriptError> = parse_script(source);

    parsed_script
}
