//! # Minigrep
//! 
//! minigrep defines a series of functions to find a string
//! within a specified file and a Config struct designed to handle command line args

use std::{env, error::Error, fs};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Builds a Config structure off of an Iterable of Strings i.e. env::Args
    /// Minimum args is 3 and first argument is skipped
    /// # Examples
    /// 
    /// ```
    /// use std::env;
    /// use minigrep::Config;
    /// 
    /// let args = vec![String::from("PROGRAM_NAME"), String::from("QUERY"), String::from("FILE_PATH")].into_iter();
    /// let config = Config {
    ///     query: String::from("QUERY"),
    ///     file_path: String::from("FILE_PATH"),
    ///     ignore_case: env::var("IGNORE_CASE").is_ok(),
    /// };
    /// 
    /// assert_eq!(config, Config::build(args).unwrap());
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}