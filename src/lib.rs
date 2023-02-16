use std::{fs, error::Error};

pub struct Data {
    pub query: String,
    pub file_path: String,
}

impl Data {
    pub fn get(args: &Vec<String>) -> Result<Data, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Data {
            query: query,
            file_path: file_path,
        })
    }
}

pub fn run(data: Data) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(data.file_path)?;

    for line in search(&data.query, &content) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
           results.push(line) 
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "thing";

        let contents = "Something something darkside";

        assert_eq!(vec!["Something something darkside"], search(query, contents))
    }
}