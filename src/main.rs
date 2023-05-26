mod config;
mod custom;
mod parse;

use std::fs::OpenOptions;
use std::{io, process};
use std::io::{BufRead, Read};
use std::path::Path;
use parse::tag::Tag;
use crate::config::Config;
use parse::token::Token;
use crate::parse::doc::Doc;

fn main() {
    println!("Enter file path to compile:");
    let mut name = String::new();
    io::stdin().lock().read_line(&mut name)
        .expect("Failed to read the input");
    name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let source = read_all(&name)
        .expect("Failed to open the file to compile.");
    let mut cfg = Config::init(name);
    let doc = parse(source.as_str(), &mut cfg);
    cfg.write_all(format!("{}", doc))
}

fn read_all<P: AsRef<Path>>(name: P) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn parse(source: &str, cfg: &mut Config) -> Doc {
    let mut doc = Doc::parse(source).unwrap_or_else(|| {
        process::exit(0);
    });
    doc.reassign_custom(|tag| {
        if let Some(v) = compile_custom(tag, cfg) {
            Token::Pointer(v)
        } else {
            Token::Comment(String::from("?error"))
        }
    });
    doc
}

fn compile_custom(tag: Tag, cfg: &mut Config) -> Option<Doc> {
    match tag.tag() {
        "include" => {
            custom::include(tag, cfg)
        }
        _ => {
            None
        }
    }
}
