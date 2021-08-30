use bzip2::read::BzDecoder;
use mediawiki_parser;
use serde_yaml;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args)?;
    println!("{}", config.filename);

    let reader = BufReader::new(File::open(config.filename)?);
    read_mediawiki_doc(BzDecoder::new(reader))?;
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

#[derive(PartialEq)]
enum ReaderState {
    Base,
    Title,
    Body,
}

fn read_mediawiki_doc<R: Read>(reader: R) -> Result<(), Box<dyn Error>> {
    let parser = EventReader::new(reader);

    let mut state = ReaderState::Base;
    for ev in parser {
        match ev? {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "title" => state = ReaderState::Title,
                "text" => state = ReaderState::Body,
                _ => {}
            },
            XmlEvent::EndElement { name, .. } => match name.local_name.as_str() {
                "title" | "text" => state = ReaderState::Base,
                _ => {}
            },
            XmlEvent::Characters(text) => {
                if state == ReaderState::Title {
                    println!("{}", text)
                } else if state == ReaderState::Body {
                    let body = mediawiki_parser::parse(&text)?;
                    println!("{}", &serde_yaml::to_string(&body)?)
                }
            }
            _ => {}
        }
    }
    Ok(())
}
