use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::try_from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Run with `-h` or `--help` for usage");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
