use bzip2::read::BzDecoder;
use clap::Clap;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use xml::reader::{EventReader, XmlEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Titles(t) => {
            println!("{}", t.filename);
            let reader = BufReader::new(File::open(t.filename)?);
            read_mediawiki_doc(BzDecoder::new(reader))?;
            Ok(())
        }
    }
}

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Titles(Titles),
}

#[derive(Clap)]
struct Titles {
    filename: String,
}

#[derive(PartialEq)]
enum ReaderState {
    Base,
    Title,
    Body,
}

fn lowercase_first_character(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn detect_title(title: &str, text: &str) -> String {
    lazy_static! {
        static ref LC: Regex =
            Regex::new(r"\{\{(小文字|lowercase|lowercase title)(\|[^\}\n]*)?\}\}").unwrap();
    }
    if LC.is_match(text) {
        lowercase_first_character(title)
    } else {
        title.to_string()
    }
}

fn read_mediawiki_doc<R: Read>(reader: R) -> Result<(), Box<dyn Error>> {
    let parser = EventReader::new(reader);

    let mut state = ReaderState::Base;
    let mut is_redirect = false;
    let mut cur_title = String::from("");
    for ev in parser {
        match ev? {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "page" => is_redirect = false,
                "title" => state = ReaderState::Title,
                "text" => state = ReaderState::Body,
                "redirect" => is_redirect = true,
                _ => {}
            },
            XmlEvent::EndElement { name, .. } => match name.local_name.as_str() {
                "title" | "text" => state = ReaderState::Base,
                _ => {}
            },
            XmlEvent::Characters(chars) => match state {
                ReaderState::Title => cur_title = chars,
                ReaderState::Body => {
                    if is_redirect {
                        continue;
                    }
                    println!("{}", detect_title(&cur_title, &chars))
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
