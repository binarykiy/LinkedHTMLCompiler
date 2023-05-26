mod config;
mod custom;
mod parse;

use std::fs::OpenOptions;
use std::{io, mem, process};
use std::io::{BufRead, Read};
use std::path::Path;
use crate::config::Config;
use crate::parse::{ParsedTag, ParsedText};

fn main() {
    println!("Enter file path to compile:");
    let mut file_name = String::new();
    io::stdin().lock().read_line(&mut file_name)
        .expect("Failed to read the input");
    file_name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let source = read_all(&file_name)
        .expect("Failed to open the file to compile.");
    let mut config = Config::init(file_name);
    let parsed = parse(source.as_str(), &mut config);
    for text in parsed {
        config.write_all(format!("{}", text));
    }
}

fn read_all<P: AsRef<Path>>(name: P) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse(source: &str, config: &mut Config) -> Vec<ParsedText> {
    let mut parsed = ParsedText::parse(source).unwrap_or_else(|| {
        process::exit(0);
    });
    let len = parsed.len();
    for i in 0..len {
        if let ParsedText::CustomTag(_) = &parsed[i] {
            let mut swap_dest = ParsedText::Null;
            mem::swap(&mut swap_dest, &mut parsed[i]);
            let ParsedText::CustomTag(tag) = swap_dest else { unreachable!() };
            let ptr = convert_custom(tag, config);
            parsed[i] = ptr;
        }
    }
    parsed
}

fn convert_custom(source: ParsedTag, config: &mut Config) -> ParsedText {
    if let Some(v) = compile_custom(source, config) {
        ParsedText::Pointer(v)
    } else {
        ParsedText::Comment(String::from("?error"))
    }
}

fn compile_custom(source: ParsedTag, config: &mut Config) -> Option<Vec<ParsedText>> {
    match source.tag() {
        "include" => {
            custom::include(source, config)
        }
        _ => {
            None
        }
    }
}
