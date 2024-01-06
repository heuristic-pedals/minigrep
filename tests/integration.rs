use minigrep::Config;

struct IntegTestSetup;

impl IntegTestSetup {
    fn config() -> Config {
        let query = String::from("duct");
        let file_path = String::from("tests/data/dummy_input.txt");
        let parsed_args = vec!["".to_string(), query, file_path];
        Config::build(&parsed_args).unwrap()
    }
    fn contents(config: &Config) -> String {
        minigrep::read_file_contents(&config.file_path).unwrap_or("".to_string())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use temp_env;

    #[test]
    fn minigrep_on_pass_case_sensitive() {
        temp_env::with_var("IGNORE_CASE", None::<String>, || {
            let config = IntegTestSetup::config();
            let contents = IntegTestSetup::contents(&config);
            assert_eq!(
                vec![(2, "safe, fast, productive.")],
                minigrep::search(&config.query, &contents, &config.ignore_case),
                "Case sensitive results do not match expectations.",
            );
        });
    }

    #[test]
    fn minigrep_on_pass_case_insensitive() {
        temp_env::with_var("IGNORE_CASE", Some(""), || {
            let config = IntegTestSetup::config();
            let contents = IntegTestSetup::contents(&config);
            assert_eq!(
                vec![(2, "safe, fast, productive."), (4, "Duct tape.")],
                minigrep::search(&config.query, &contents, &config.ignore_case),
                "Case insensitive results do not match expectations.",
            );
        });
    }

    #[test]
    fn run_on_pass() {
        temp_env::with_var("IGNORE_CASE", None::<String>, || {
            let config = IntegTestSetup::config();
            let result = minigrep::run(config);
            assert!(result.is_ok());
        });
    }
}
