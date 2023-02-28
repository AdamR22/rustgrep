use std::{env, error::Error, fs};

pub struct Data {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Data {
    pub fn get(mut args: impl Iterator<Item = String>) -> Result<Data, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query strng"),
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Data {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(data: Data) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(data.file_path)?;

    let results = if data.ignore_case {
        search_case_insensitive(&data.query, &content)
    } else {
        search(&data.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "thing";

        let contents = "Something something darkside";

        assert_eq!(
            vec!["Something something darkside"],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "CoMe On";

        let contents = "Come on man. This isn't it";

        assert_eq!(
            vec!["Come on man. This isn't it"],
            search_case_insensitive(query, contents)
        )
    }
}
