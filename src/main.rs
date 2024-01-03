use std::env;
use std::fs;

fn main() {
    // collect command line arguments - ignore idx 0 (binary name)
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.file_path);

    let contents: String =
        fs::read_to_string(&config.file_path).expect("Unable to read provided file path");
    println!("With contents: {}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(parsed_args: &Vec<String>) -> Config {
        Config {
            query: parsed_args[1].clone(),
            file_path: parsed_args[2].clone(),
        }
    }
}
