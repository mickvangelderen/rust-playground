extern crate minigrep;

use std::process;

fn main() {
    let config = minigrep::Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Input error: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err.description());
        process::exit(1);
    }
}