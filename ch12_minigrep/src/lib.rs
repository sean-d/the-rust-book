use std::env; // checking if an env variable is set or not via env::var
use std::error::Error; // handling Result<T, E> errors
use std::fs; // doing stuff with the file system //

// couple things together so demonstrate association.
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
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

        /*
        we check if the env variable "IGNORE_CASE is set or not. that presence determines case sensitivity in our minigrep runtime"

        depending on which is toggled, run will perform differently

        IGNORE_CASE=1 cargo run -- to poem.txt ... to ignore case
        */
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore case: {}", ignore_case);

        // the Config in the Result<> return
        Ok(Config {
            query: query,
            filename: filename,
            ignore_case,
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

depending on if we are searching with case sensitivity or not:
we iterate over the returned vector from search and print out the lines that have the query in them.

we print out the results

Ok(()): using () like this indicates we are calling run for its side effects only. it's returning nothing.

we need to handle the returned Result<> in main.

*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

// the returned Vec needs to life as long as the contents since that's what is being returned from. so we create a lifetime relationship.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

/*
tests section
*/

#[cfg(test)]
mod tests {
    use super::*;

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
        )
    }
}
