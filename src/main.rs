mod config;
mod custom;
mod parse;
mod util;

use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, Read};
use std::path::Path;
use crate::config::Config;

fn main() {
    println!("Enter file path to compile:");
    let mut name = String::new();
    io::stdin().lock().read_line(&mut name)
        .expect("Failed to read the input");
    name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let source = read_all(&name)
        .expect("Failed to open the file to compile.");
    let mut cfg = Config::init(name);
    let doc = parse::parse(source, &mut cfg);
    cfg.write_all(format!("{}", doc))
}

fn read_all<P: AsRef<Path>>(name: P) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
