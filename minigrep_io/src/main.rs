use minigrep_io::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args); -> commenting this line as args moved into the dbg! macro
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments! {}", err);
        process::exit(1);
    });
    println!("Searching for '{}'", config.query);
    println!("in file '{}'", config.file_path);

    if let Err(e) = minigrep_io::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
