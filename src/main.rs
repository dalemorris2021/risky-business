use std::process;

use clap::Parser;
use risky_business::Args;

fn main() {
    let _ = Args::parse();

    if let Err(e) = risky_business::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
