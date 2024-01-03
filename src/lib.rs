use std::error::Error;
use std::fs;
use std::env;

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
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use Box<dyn Error> to allow all error types to propagate
    let contents: String = fs::read_to_string(&config.file_path)?;
    for line in search(&config.query, &contents, &config.ignore_case) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: &bool) -> Vec<&'a str> {
    if *ignore_case {
        contents.lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    } else {
        contents.lines()
            .filter(|line| line.contains(&query))
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, &false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust noone.";
        assert_eq!(vec!["Rust:", "Trust noone."], search(query, contents, &true));
    }

}
