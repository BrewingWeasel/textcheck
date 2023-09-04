use std::{
    fs,
    io::{self, Read},
};

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
    let value = args.file.map_or_else(
        || {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        },
        |file| fs::read_to_string(file).unwrap(),
    );
    let errors = textcheck::check(&value);
    for error in errors {
        textcheck::display(&error, value.lines());
    }
}
