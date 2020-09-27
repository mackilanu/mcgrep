//! # mcgrep
//! `mcgrep` is a clone of the popular `grep` program found in unix-like environments.
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
   pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
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

/// Searches a file for matches.
/// # Examples
///
/// ```
///     let search_data = "\
/// Foo
/// Bar
/// Foobar";
///
///     let lines = mcgrep::search("Foo", &search_data);
///     assert_eq!(lines[0], "Foo");
///     assert_eq!(lines[1], "Foobar");
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


/// Searches a file for matches, but case insensitive.
/// # Examples
///
/// ```
///     let search_data = "\
/// foo
/// Bar
/// Foobar";
///
///     let lines = mcgrep::search_case_insensitive("foo", &search_data);
///     assert_eq!(lines[0], "foo");
///     assert_eq!(lines[1], "Foobar");
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}
