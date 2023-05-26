mod config;
mod custom;
mod parse;

use std::fs::OpenOptions;
use std::{io, mem, process};
use std::io::{BufRead, Read};
use std::path::Path;
use parse::tag::Tag;
use crate::config::Config;
use parse::token::Token;
use crate::parse::doc::Doc;

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
    for i in 0..parsed.len() {
        config.write_all(format!("{}", parsed[i]));
    }
}

fn read_all<P: AsRef<Path>>(name: P) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse(source: &str, config: &mut Config) -> Doc {
    let mut parsed = Token::parse(source).unwrap_or_else(|| {
        process::exit(0);
    });
    let len = parsed.len();
    for i in 0..len {
        if let Token::CustomTag(_) = &parsed[i] {
            let mut swap_dest = Token::Null;
            mem::swap(&mut swap_dest, &mut parsed[i]);
            let Token::CustomTag(tag) = swap_dest else { unreachable!() };
            let ptr = convert_custom(tag, config);
            parsed[i] = ptr;
        }
    }
    Doc::from_vec(parsed)
}

fn convert_custom(source: Tag, config: &mut Config) -> Token {
    if let Some(v) = compile_custom(source, config) {
        Token::Pointer(v)
    } else {
        Token::Comment(String::from("?error"))
    }
}

fn compile_custom(source: Tag, config: &mut Config) -> Option<Doc> {
    match source.tag() {
        "include" => {
            custom::include(source, config)
        }
        _ => {
            None
        }
    }
}
