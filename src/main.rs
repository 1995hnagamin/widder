use bzip2::read::BzDecoder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;
    println!("{}", config.filename);

    let reader = BufReader::new(File::open(config.filename)?);
    let mut reader = BzDecoder::new(reader);
    Ok(())
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
