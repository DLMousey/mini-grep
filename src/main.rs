use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // Collect the command line arguments the user passed in
    let args: Vec<String> = env::args().collect();

    // Create a new Config struct from these arguments or exit out with
    // a non zero error code and a helpful-ish message
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

/*
    A struct for holding the Config values, AFAIK a "struct" in this world
    is what i'd call an "Object" or "Model" in JS Land
*/
struct Config {
    query: String,
    filename: String
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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient number of arguments passed");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
