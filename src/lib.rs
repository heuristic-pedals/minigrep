use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    contents: String, 
}

impl Config {
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
        if parsed_args.len() < 3 {
            return Err("Too few arguments provided.".into());
        }
        let query: String = parsed_args[1].clone();
        let file_path: String = parsed_args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        let contents: String = fs::read_to_string(&file_path)?;
        Ok(Config {
            query,
            file_path,
            ignore_case,
            contents
        })
    }
}

pub fn run(config: Config) {
    for (i, line) in search(&config.query, &config.contents, &config.ignore_case) {
        println!("L{i}: {line}");
    }
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: &bool) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| match *ignore_case {
            true => line.to_lowercase().contains(&query.to_lowercase()),
            _ => line.contains(&query),
        })
        .map(|(i, line)| (i + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "tests/data/dummy_input.txt".to_string(),
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
            config.file_path, "tests/data/dummy_input.txt",
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
            config.is_err_and(|err| err.to_string() == "Too few arguments provided."),
            "Unexpected error message when passing to few arguments."
        );
    }
}
