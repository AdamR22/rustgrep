use std::env;
use std::process;

use rustgrep::Data;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data: Data = Data::get(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {} in {}\n", data.query, data.file_path);

    if let Err(e) = rustgrep::run(data) {
        println!("Application error: {e}");
        process::exit(1);
    }
}