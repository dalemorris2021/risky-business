use std::process;

use clap::Parser;
use risky_business::{Args, Config};

fn main() {
    let args = Args::parse();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = risky_business::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
