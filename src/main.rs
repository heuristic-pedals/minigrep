use std::env;

fn main() {
    // collect command line arguments - ignore idx 0 (binary name)
    let args: Vec<String> = env::args().collect();
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {} in {}", query, file_path);
}
