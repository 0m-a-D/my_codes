use std::{error::Error, fs};
pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}
impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS!");
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Config { query, file_path })
    }
}
#[allow(unused)]
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "dyn" is keyword that's short for "dynamic"
    let contents = fs::read_to_string(config.file_path)?;
    // println!("with text:\n{}", contents);
    for line in search(config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
    // () is a unit type. unwrapping it in Ok tells we care about the side
    // effects only, it doesn't return a value.
}
#[allow(unused)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in contents.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
