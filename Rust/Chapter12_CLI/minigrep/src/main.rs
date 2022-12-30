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

// we define explicit lifetimes 'a to match the lifetimes of contents and the result
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}

