use std::error::Error; // handling Result<T, E> errors
use std::fs; // doing stuff with the file system

// couple things together so demonstrate association.
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // constructor that takes the passed in reference to a vec of String
    // returns a Result<Config, &str> where the Config is OK and Err is the error message as &str
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // since the program name is args[0], we start at 1
        // we clone so we don't transfer ownership.
        // the struct is looking for String so we give them that.
        // to use str, we would have to use lifetimes and this is just easier.

        if args.len() < 3 {
            // the &str in the Result<> return
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // the Config in the Result<> return
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

/*
takes in a Config
returns a Result<(), Box<dyn Error>>
- we were returning a unit type which is what we were returning before...which is basically nothing....
- or we are returning an error...this fancy looking thing basically says:
-- the function will return a type that implements the Error trait, but we dont have to specify what type the return value will be.

we are using ? after read_to_string to have any error value returned

Ok(()): using () like this indicates we are calling run for its side effects only. it's returning nothing.

we need to handle the returned Result<> in main.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("file content: {contents}");
    Ok(())
}
