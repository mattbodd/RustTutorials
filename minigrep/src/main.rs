// Bring in the standard library module to accesss `args`
// std::env::args_os could be used to recognize non-ASCII values
use std::env;
// Bring in the standard library module to handle exiting process
use std::process;

use minigrep;
use minigrep::Config;

// The majority of the functionality in a Rust program should belong in
// lib.rs while the usage of functions defined in lib.rs go in main.rs

fn main() {
    // Result<T, E>.upwrap_or_else() allows a user to define non-panic! error handling
    // if the inner value Result<T, E> is Ok() then it behaves like a normal Ok()
    // otherwise the code in the closure is run
    // we pass env::args() directly into Config::new as env::args() is
    // an iterator which will allow Config::new to have ownership
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // We don't benefit from using unwrap_or_else here as the Ok() result is always the
    // unit type so we only care about detecting errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
