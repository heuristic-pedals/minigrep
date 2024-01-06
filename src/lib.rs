#![doc = include_str!("../README.md")]
use std::env;
use std::error::Error;
use std::fs;
use std::io;

/// Capture and collect the runtime configuration altogether
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Builds an instance of `Config` after parsing inputs
    ///
    /// # Arguments
    ///
    /// * `parsed_args` - A vector of strings denoting the parsed inputs.
    /// Expecting the format: [BINARY_NAME, SUB-STRING, PATH-TO-FILE] (this
    /// is the result of calling `std::env::args().collect()`)
    /// 
    /// > Note: the format of `parsed_args` is currently 'awkward' to use. A
    /// future feature will be to implement `Config::new()` such that use cases
    /// that don't require a cli can be catered for.
    ///
    /// # Examples
    ///
    /// ```
    /// use minigrep::Config;
    /// let dummy_parsed_args = vec![
    ///     "".to_string(),                 // empty dummy binary name (not used)
    ///     "test".to_string(),             // dummy sub-string query
    ///     "data/test.txt".to_string(),    // dummy file path
    /// ];
    /// let config = Config::build(&dummy_parsed_args).unwrap();
    /// assert_eq!(config.query, "test");
    /// assert_eq!(config.file_path, "data/test.txt");
    /// ```
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

/// A utility function to run `minigrep` using a parsed runtime config. Will
/// read the requested file's content (using `read_file_content`) and uses `search`
/// to find the requested sub-string patterns. Prints all results to stdout.
///
/// # Arguments
///
/// * `config` - An instance of the [`Config`] struct.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use Box<dyn Error> to allow all error types to propagate
    let contents: String = read_file_contents(&config.file_path)?;
    for (i, line) in search(&config.query, &contents, &config.ignore_case) {
        println!("L{i}: {line}");
    }

    Ok(())
}

/// A helper function to read a file's content into memeory. Currently, this is
/// just a wrapper around `std::fs::read_to_string`. It exists as a placeholder
/// such that improvements can be made (such as buffered file reading) without
/// impacting the API.
///
/// # Arguments
///
/// * `config` - An instance of the [`Config`] struct.
pub fn read_file_contents(file_path: &str) -> Result<String, io::Error> {
    // shallow wrapper for now - TODO improve to buffer read
    fs::read_to_string(file_path)
}

/// Search the file contents for a sub-string pattern.
///
/// # Arguments
///
/// * `query` - A string-slice denoting the query sub-string.
/// * `contents` - A string-slice denoting The file's contents.
/// * `ignore_case` - A boolean flagging whether case sensitivity should be
/// ignored (case insensitive when `ignore_case` is `true`).
/// 
/// > Note: the arguments correspond to fields of a [`Config`] instance and 
/// the contents of the input file.
///
/// # Examples
/// 1. Case sensitive:
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
/// assert_eq!(
///     vec![(2, "safe, fast, productive.")],
///     search(query, contents, &false),
/// );
/// ```
/// 2. Case insensitive:
/// ```
/// use minigrep::search;
/// let query = "rUsT";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust noone.";
/// assert_eq!(
///     vec![(1, "Rust:"), (4, "Trust noone.")],
///     search(query, contents, &true),
/// );
/// ```
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
