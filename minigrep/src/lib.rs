use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_few_args() {
        let args = vec!["program".to_string()];
        let actual = Config::new(&args).unwrap_err();
        assert_eq!(actual, "not enough arguments");
    }

    #[test]
    fn enuff_args() {
        // let args = vec!["program", "needle", "haystack.txt"].iter().map(|s| s.to_string()).collect();
        let args = vec![
            "program".to_string(),
            "needle".to_string(),
            "haystack.txt".to_string(),
        ];
        let actual = Config::new(&args).unwrap();
        assert_eq!(
            actual,
            Config {
                query: "needle".to_string(),
                filename: "haystack.txt".to_string(),
                case_sensitive: true,
            }
        );
    }

    #[test]
    fn failed_run() {
        let cfg = Config {
            query: "needle".to_string(),
            filename: "not-exists.txt".to_string(),
            case_sensitive: true,
        };
        let err = run(cfg).unwrap_err();
        assert_eq!(err.to_string(), "No such file or directory (os error 2)");
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
