use std::{env, process};

use rustgrep::Data;

fn main() {
    let data: Data = Data::get(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {} in {}\n", data.query, data.file_path);

    if let Err(e) = rustgrep::run(data) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}