use std::{io, io::Write};

use anyhow::{Context, Result};
use json_parser::parser::parse;

fn read_input() -> Result<String> {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .context("Failed to read line")?;

    Ok(raw_input)
}

pub fn repl() {
    loop {
        print!("json-parser> ");
        io::stdout().flush().unwrap();

        let expr = read_input().unwrap();

        match parse(expr.as_ref()) {
            Ok(val) => println!(" ==> {}", val),
            Err(error) => eprintln!("==> Error: {}", error),
        };
    }
}

fn main() {
    repl();
}
