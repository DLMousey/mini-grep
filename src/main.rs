use std::env;
use std::process;

use mini_grep;
use mini_grep::Config;

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

    /*
        Start the run method, if it's return value is of a type that implements the
        Error interface the closure will run gracefully informing the user the program
        fell over and exiting with a non zero error code.

        Using an if statement for comparison instead of unwrap_or_else because run
        will not return a value that we can unwrap
    */
    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}