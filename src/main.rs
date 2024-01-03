use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    // run application
    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    };
}
