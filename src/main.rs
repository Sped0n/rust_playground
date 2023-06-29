use rust_playground::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(&cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
