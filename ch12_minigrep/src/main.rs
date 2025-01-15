use ch12_minigrep::run;
use ch12_minigrep::Config;
use std::env; // reading values from commandline via env::args
use std::process; // used for exit codes // Config struct an associated functions from lib.rs
fn main() {
    // args returns an iterator of arguments
    // collect turns that into a collection, Vec<String> in this case
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            println!("Problem parsing arguments: {}", e);
            process::exit(1);
        }
    };

    /*
    the above can be written using unwrap_or_else.
    using _config2 to silence warnings


    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    */

    println!("searching for: {}", config.query);
    println!("in the file: {}", config.filename);

    /*
    we use if let rather than unwrap_or_else.

    the run function doesn't returna value that we wish to unwrap.
    because run returns () in the success case, we only care about the error.

    so we are not using unwrap_or_else to return the unwrapped value which would be ().
    */
    if let Err(e) = run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}
