use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(ars: &[String]) -> Result<Config, &'static str> {
        if ars.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = ars[1].clone();
        let file_path = ars[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_path)?;
    let results = search(&config.query, &file_content);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
