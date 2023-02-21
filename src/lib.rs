use std::error::Error;
use std::fs;
//process help us handle withotu panicking
use std::env;
//figure out which to use

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //We conventionally use new as constructor functions.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("Oh No!")
            // panic is better when we face "programming error" instead of usage error.
            return Err("Not enough arguments!!!!> <");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        //Use clone = Do not take ownership of the string.
        // we gotta use lifetime to handle this more efficiently....

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //If we set the env to be case_sensitive

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename)
    //     .expect("Error reading file! Ouch!");

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("the lines u r looking for are:");
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //We set the lifetime of the return value = lifetime of contents.
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //We set the lifetime of the return value = lifetime of contents.
    let query = query.to_lowercase();
    //Notice that to_lowercase creates a new String!!
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// tells Rust to compile and run the test code only when you run cargo test
#[cfg(test)]
mod tests {
    use super::*;
    //import everything from parent module.
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast ,productive
Pick three";
        //assert_eq is a useful tool for writing tests and verifying that code works as expected.
        assert_eq!(vec!["safe, fast ,productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast ,productive
Pick three
Trust is everything.";
        assert_eq!(
            vec!["Rust:", "Trust is everything."],
            search_case_insensitive(query, contents)
        )
    }
}
