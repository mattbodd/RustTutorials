// Bring in the standard library module to handle Error types
use std::error::Error;
// Bring in the standard library module to handle files
use std::fs;
//
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// The error type for Result is &'static str as this is the type of string literals which is what
// is being returned in this program
impl Config {
    // Remember that a ' denotes a lifetime specifier and that static is the longest
    // lifetime annotation in Rust - the duration of an entire program!
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip the 0th index containing calling context
        args.next();

        // Returning an err here is more appropriate than invoking
        // panic! as this is a usage error rather than a programming
        // problem
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
// 'dyn Error' here stands for 'dynamic error' and allows the function to return any return value
// that is an Error type
// Box<dyn Error> is a trait object which allows us to return a type that implements the
// Error trait which is good for returning different error values that may be of
// different types in different error cases
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

    // Wrapping the unit type in the Ok() in this case signifies that we only care about
    // the side effects of the run() function
    Ok(())
}

// The actual searching for minigrep - the heart of the program
// There is an explicit lifetime which specifies that the returned vector should contain string
// slices that reference slices of the argument `contents` rather than `query`
// We're essentially telling the compiler the lifetime of the returned vector is the
// same as the lifetime of contents!  Query can be tossed after we finish searching
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Take advantage of iterator adapter methods like filter!1
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()

}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
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
