use expander::{BookAbbreviation, Reference};
use std::env;

fn main() {
    // Read the input from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <reference>", args[0]);
        return;
    }

    let input = &args[1];
    match input.reference() {
        Some(reference) => {
            let uri = reference.return_uri();
            println!("[{}]({})", input, uri);
        }
        None => {
            eprintln!("Invalid reference: {}", input);
        }
    }
}
