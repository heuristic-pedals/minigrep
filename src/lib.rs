use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(parsed_args: &Vec<String>) -> Result<Config, &'static str> {
        if parsed_args.len() < 3 {
            return Err("Too few arguments provided.");
        }
        let query: String = parsed_args[1].clone();
        let file_path: String = parsed_args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use Box<dyn Error> to allow all error types to propagate
    let contents: String = fs::read_to_string(&config.file_path)?;

    Ok(())
}
