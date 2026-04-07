use crate::cli::parse_args;

mod cli;
mod io;

use std::env;
use std::process;
use crate::io::process_csv;

fn main() {

    let config = match parse_args(env::args().skip(1)) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error in the arguments: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = process_csv(&config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
