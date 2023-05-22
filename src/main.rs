use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::Path;

fn main() {
    println!("Enter file path to compile:");
    let mut file_name = String::new();
    io::stdin().lock().read_line(&mut file_name)
        .expect("Failed to read the input");
    file_name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let source = read_all(&file_name)
        .expect("Failed to open the file to compile.");
    let mut out = OpenOptions::new().create(true).truncate(true).write(true)
        .open(Path::new(&file_name).parent().unwrap().join("out.html"))
        .expect("Failed to create the output file");
    parse_and_write(source, &mut out);
}

fn read_all<S: AsRef<str>>(name: S) -> io::Result<String> {
    let path = Path::new(name.as_ref());
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn write_all<S: AsRef<str>>(content: S, file: &mut File) {
    file.write_all(content.as_ref().as_bytes())
        .expect("Failed to write to the output file");
}

fn parse_and_write(source: String, out: &mut File) {
    let mut source = source.as_str();
    loop {
        if let Some((before_lhc_comment, other)) = source.split_once("<!--?") {
            write_all(before_lhc_comment, out);
            let (lhc_comment, after_lhc_comment) = other.split_once("-->")
                .expect("Syntax Error: expected -->, found eof");
            lhc_process(lhc_comment, out);
            source = after_lhc_comment;
        } else {
            write_all(source, out);
            break;
        }
    }
}

fn lhc_process(content: &str, out: &mut File) {
    if content.is_empty() {
        write_all("<!--?-->", out);
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
            include(args_itr, out);
        }
        _ => {
            eprintln!("Unknown lhc comment: {}", key);
            write_all("<!--?-->", out);
        }
    }
}

fn include<'a, F: Iterator<Item=(&'a str, &'a str)>>(iter: F, out: &mut File) {
    for (key, value) in iter {
        match key {
            "link" => {
                let link = value.trim_matches('"');
                let mut source = read_all(link)
                    .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
                if let Some((_, body_from_prefix)) = source.split_once("<body") {
                    let (_, body_after_prefix) = body_from_prefix.split_once('>')
                        .expect("[ERROR] Expected '>', not found");
                    let (_, body) = body_after_prefix.split_once("</body>")
                        .expect("[ERROR] Expected '</body>', not found");
                    source = body.to_string();
                }
                parse_and_write(source, out);
            }
            _ => {
                eprintln!("[WARN] Unknown property for \"include\": \"{}={}\"", key, value);
            }
        }
    }
}
