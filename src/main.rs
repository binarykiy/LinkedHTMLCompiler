mod config;
mod custom;
mod parse;
mod util;

use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use crate::config::Config;

fn main() {
    println!("Enter file path to compile:");
    let mut name = String::new();
    io::stdin().lock().read_line(&mut name)
        .expect("Failed to read the input");
    name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let mut cfg = Config::init(name.clone());
    let source = cfg.read_absolute(PathBuf::from(name))
        .expect("Failed to open the file to compile.");
    let doc = parse::parse(source, &mut cfg);
    cfg.write_all(format!("{}", doc))
}
