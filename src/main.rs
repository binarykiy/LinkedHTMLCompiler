mod config;
mod tag;
mod util;

use std::fs::OpenOptions;
use std::{io, mem, process};
use std::io::{BufRead, Read};
use std::path::Path;
use crate::config::Config;
use crate::util::{ParsedTag, ParsedText};

fn main() {
    println!("Enter file path to compile:");
    let mut file_name = String::new();
    io::stdin().lock().read_line(&mut file_name)
        .expect("Failed to read the input");
    file_name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let source = read_all(&file_name)
        .expect("Failed to open the file to compile.");
    let mut config = Config::init(file_name);
    parse_and_write(source, &mut config);
}

fn read_all<P: AsRef<Path>>(name: P) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn parse(source: String, config: &mut Config) {
    let mut parsed = ParsedText::parse(source.as_str()).unwrap_or_else(|| {
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
}

fn convert_custom<'a>(source: ParsedTag<'a>, config: &mut Config) -> ParsedText<'a> {
    if let Some(v) = compile_custom(source, config) {
        ParsedText::Pointer(v)
    } else {
        ParsedText::Comment("?error")
    }
}

fn compile_custom<'a>(source: ParsedTag<'a>, config: &mut Config) -> Option<Vec<ParsedText<'a>>> {
    None // todo
}

fn parse_and_write(source: String, config: &mut Config) {
    let mut source = source.as_str();
    loop {
        if let Some((before_lhc_comment, other)) = source.split_once("<!--?") {
            config.write_all(before_lhc_comment);
            let (lhc_comment, after_lhc_comment) = other.split_once("-->")
                .expect("Syntax Error: expected -->, found eof");
            lhc_process(lhc_comment, config);
            source = after_lhc_comment;
        } else {
            config.write_all(source);
            break;
        }
    }
}

fn lhc_process(content: &str, config: &mut Config) {
    if content.is_empty() {
        config.write_all("<!--?-->");
    }
    let (key, args): (& str, & str) = content.split_once(' ').unwrap_or((content, ""));
    let args_itr = args.split(' ').filter(|arg| {
        let res = arg.contains('=');
        if !res && !arg.is_empty() {
            eprintln!("[ERROR] Illegal property syntax: \"{}\"", arg);
        }
        res
    }).map(|arg| {
        arg.split_once('=').unwrap()
    });
    match key {
        "include" => {
            tag::include(args_itr, config);
        }
        _ => {
            eprintln!("Unknown lhc comment: {}", key);
            config.write_all("<!--?-->");
        }
    }
}
