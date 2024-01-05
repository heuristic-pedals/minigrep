#![doc = include_str!("../README.md")]
use std::env;
use std::error::Error;
use std::fs;
use std::io;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, &'static str> {
        if parsed_args.len() < 3 {
            return Err("Too few arguments provided.");
        }
        let query: String = parsed_args[1].clone();
        let file_path: String = parsed_args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use Box<dyn Error> to allow all error types to propagate
    let contents: String = read_file_contents(&config.file_path)?;
    for (i, line) in search(&config.query, &contents, &config.ignore_case) {
        println!("L{i}: {line}");
    }

    Ok(())
}

pub fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    // shallow wrapper for now - TODO improve to buffer read
    fs::read_to_string(file_path)
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: &bool) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| match *ignore_case {
            true => line.to_lowercase().contains(&query.to_lowercase()),
            _ => line.contains(query),
        })
        .map(|(i, line)| (i + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents, &false),
            "Case sensitive results do not match expectations.",
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust noone.";
        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust noone.")],
            search(query, contents, &true),
            "Case in-sensitive results do not match expectations.",
        );
    }

    #[test]
    fn config_build_on_pass() {
        let dummy_parsed_args = vec![
            "".to_string(),
            "dummy_query".to_string(),
            "dummy_file_path".to_string(),
        ];
        let config = Config::build(&dummy_parsed_args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.query, "dummy_query",
            "Unexpected `query` value {}",
            config.query
        );
        assert_eq!(
            config.file_path, "dummy_file_path",
            "Unexpected `file_path` value {}",
            config.query
        );
    }

    #[test]
    fn config_build_too_few_arguments() {
        let dummy_parsed_args = vec!["prog_name".to_string()];
        let config = Config::build(&dummy_parsed_args);
        // split out assertions to imporve test debug messages
        assert!(config.is_err(), "Too few arguments case was not detected.");
        assert!(
            // check equality since err message not expected to change
            config.is_err_and(|err| err == "Too few arguments provided."),
            "Unexpected error message when passing to few arguments."
        );
    }

    #[test]
    fn read_file_contents_on_pass() {
        let contents = read_file_contents("tests/data/dummy_input.txt");
        assert!(
            contents.is_ok(),
            "Did not read tests/data/dummy_input.txt: {:?}",
            contents.err()
        );
        assert_eq!(
            contents.unwrap(),
            "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.".to_string()
        );
    }

    #[test]
    fn read_file_contents_non_existant() {
        let contents = read_file_contents("does_not_exist.txt");
        assert!(contents.is_err_and(|err| err.kind() == ErrorKind::NotFound));
    }
}
