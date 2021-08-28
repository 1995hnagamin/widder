use bzip2::read::BzDecoder;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("{}", config.filename);
}

struct Config {
    filename: String,
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = args[1].clone();
    Ok(Config { filename })
}
