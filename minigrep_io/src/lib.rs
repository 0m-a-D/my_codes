use std::{env, error::Error, fs};
pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}
impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS!");
        }
        let query = &args[1];
        let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "dyn" is keyword that's short for "dynamic"
    let contents = fs::read_to_string(config.file_path)?;
    // println!("with text:\n{}", contents);
    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
    // () is a unit type. unwrapping it in Ok tells we care about the side
    // effects only, it doesn't return a value.
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for lines in contents.lines() {
    //     if lines.contains(query) {
    //         results.push(lines);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // using iterator adaptor methods like filter().
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
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
Duct tape";
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
