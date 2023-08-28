use std::{
    fs,
    io::{self, Read},
};

use textcheck;

use clap::Parser;

/// Linter for text
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to lint
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let value = if let Some(file) = args.file {
        fs::read_to_string(file).unwrap()
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let errors = textcheck::check(&value);
    for error in errors {
        textcheck::display(error, value.lines())
    }
}
