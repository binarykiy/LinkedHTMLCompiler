mod config;
mod custom;
mod parse;
mod util;
mod source;

use std::time::Instant;
use crate::config::Config;
use crate::util::read_from_stdin;

fn main() {
    println!("Enter file path to compile:");
    let mut name = read_from_stdin().expect("Failed to read the input");
    println!("[INFO] Compilation started.");
    let timer = Instant::now();
    name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let (mut cfg, Ok(source)) = Config::new(name.clone())
        else { panic!("Failed to open the file to compile.") };
    let doc = parse::into_doc(source, &mut cfg);
    cfg.write_all(format!("{}", doc));
    println!("[INFO] Compilation finished. Time = {:?}", timer.elapsed());
}
