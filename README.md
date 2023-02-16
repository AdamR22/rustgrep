# Rustgrep
A CLI tool built in rust that mimics the grep tool.

## Setting up the project
- Install [rust](https://www.rust-lang.org/tools/install).
- Clone the repo.
- cd to the cloned repo's location.
- run 
   ```bash 
   cargo build
   ```

## Running the project
- To use the tool, run
   ```bash
   cargo run -- <word to search> <path to desired file>

   #example
   cargo run -- rhetorical example_file.txt

   #searching for a sentence containing the word regardless of case
   IGNORE_CASE=1 cargo run -- RhEtOrIcAl example_file.txt
   ```

## Credits
[The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/)