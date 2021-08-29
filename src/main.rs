use bzip2::read::BzDecoder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;
    println!("{}", config.filename);

    let reader = BufReader::new(File::open(config.filename)?);
    let reader = BzDecoder::new(reader);
    let parser = EventReader::new(reader);
    let mut title_mode = false;
    for ev in parser {
        match ev? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "title" {
                    title_mode = true
                }
            }
            XmlEvent::EndElement { name, .. } => {
                if name.local_name == "title" {
                    title_mode = false
                }
            }
            XmlEvent::Characters(text) => {
                if title_mode {
                    println!("{}", text)
                }
            }
            _ => {}
        }
    }
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
