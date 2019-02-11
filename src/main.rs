use std::fs;
use std::env;
use std::process;
use std::error::Error;

// Sets up configuration then runs program
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    println!("{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query: query, filename: filename})
        }
    }
}
