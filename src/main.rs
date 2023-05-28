mod config;
mod custom;
mod parse;
mod util;

use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::Instant;
use crate::config::Config;

fn main() {
    println!("Enter file path to compile:");
    let mut name = String::new();
    io::stdin().lock().read_line(&mut name)
        .expect("Failed to read the input");
    println!("[INFO] Compilation started.");
    let timer = Instant::now();
    name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let (mut cfg, Ok(source)) = Config::new(name.clone())
        else { panic!("Failed to open the file to compile.") };
    let doc = parse::parse(source, &mut cfg);
    cfg.write_all(format!("{}", doc));
    println!("[INFO] Compilation finished. Time = {:?}", timer.elapsed());
}
