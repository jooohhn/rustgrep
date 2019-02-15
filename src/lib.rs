use std::fs;
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query: query, filename: filename})
        }
    }
}
