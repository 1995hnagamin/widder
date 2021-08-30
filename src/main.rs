use bzip2::read::BzDecoder;
use lazy_static::lazy_static;
use regex::Regex;
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

fn contains_lowercase_template(text: &str) -> bool {
    lazy_static! {
        static ref LC: Regex =
            Regex::new(r"(\{\{小文字(\|[^\}\n]*)?\}\}|\{\{lowercase title\}\})").unwrap();
    }
    LC.is_match(text)
}

fn lowercase_first_character(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn read_mediawiki_doc<R: Read>(reader: R) -> Result<(), Box<dyn Error>> {
    let parser = EventReader::new(reader);

    let mut state = ReaderState::Base;
    let mut cur_title = String::from("");
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
            XmlEvent::Characters(text) => match state {
                ReaderState::Title => cur_title = text,
                ReaderState::Body => {
                    if contains_lowercase_template(&text) {
                        println!("{}", lowercase_first_character(&cur_title))
                    } else {
                        println!("{}", cur_title)
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
