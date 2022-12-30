use std::env; // for cli args
use std::process; // for exiting 

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // parse args
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for `{}`", config.query);
    println!("In file `{}`", config.file_path);

    // run and check for errors
    if let Err(e) = minigrep::run(config) {
        println!("Application Logic Error: {e}");
        process::exit(1);
    }
}

