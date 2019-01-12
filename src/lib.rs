use std::error::Error;
use std::fs;

/*
    A struct for holding the Config values, AFAIK a "struct" in this world
    is what i'd call an "Object" or "Model" in JS Land
*/
pub struct Config {
    pub query: String,
    pub filename: String
}

/*
    An implementation for the Config struct, Creates a new method on the
    Struct that we can use as its constructor.

    Instead of returning a Config struct directly we return a Result
    object which will contain either the created Struct or a string
    explaining what went wrong.

    This originally called a panic! macro on error but this throws
    out a lot of extra noise like a stack trace that we don't need for
    a simple problem like not providing enough arguments.
*/
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient number of arguments passed");
        }

        /*
            Clone the values instead of passing them by reference
            because we're operating inside a library file now
        */
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

/*
    Attempt to run the business logic, This method will return a Result
    object with either an empty success value or a Box object containing something
    that implements the Error interface
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*
        Previous called expect which triggered a panic on fail, the question mark
        operator at the end signals that this will _return_ any errors it throws,
        which will be automatically added to the Result object and returned to main
    */
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // We won't have any values to return, so we'll just signal everything is ok
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/*
    A unit test to ensure that stuff we write is working.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        );
    }
}
