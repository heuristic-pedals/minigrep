use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // collect command line arguments - ignore idx 0 (binary name)
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    // run application
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use Box<dyn Error> to allow all error types to propagate
    let contents: String = fs::read_to_string(&config.file_path)?;
    println!("With contents: {}", contents);
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(parsed_args: &Vec<String>) -> Result<Config, &'static str> {
        if parsed_args.len() < 3 {
            return Err("Too few arguments provided.");
        }
        let query = parsed_args[1].clone();
        let file_path = parsed_args[2].clone();
        Ok(Config { query, file_path })
    }
}
