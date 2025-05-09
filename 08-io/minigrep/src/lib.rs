use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}


impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 去除第一个命令行参数(可执行文件地址)
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string.")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name.")
        };
        let sensitive = env::var("CASE_INSENSITIVE").is_err(); // is_err在该环境变量存在时返回false
        Ok(Config { query, filename, sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for result in results {
        println!("{}", result);
    }

    Ok(())
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "Rust:\nsafe, fast, hard.\nPick third one.\nTrust it.";

        assert_eq!(vec!["Rust:", "Trust it."], search_case_insensitive(query, content));
    }
}

/// 返回contents中所有包含query的行
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(| line | line.contains(query))
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