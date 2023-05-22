use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::Path;

fn main() {
    println!("Enter file path to compile:");
    let mut input_path_name = String::new();
    io::stdin().lock().read_line(&mut input_path_name)
        .expect("Failed to read the input");
    input_path_name.retain(|c| c != '\r' && c != '\n' && c != '"');
    let input_path = Path::new(&input_path_name);
    let input_file = OpenOptions::new().read(true).open(input_path)
        .expect("Failed to open the file");
    let output_path = input_path.parent().unwrap().join("out.html");
    let output_file = OpenOptions::new().create(true).truncate(true).write(true).open(output_path)
        .expect("Failed to create the output file");
    parse_and_write(read_all(input_file), output_file);
}

fn read_all(mut file: File) -> String {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read the file");
    input
}

fn write_all<S: AsRef<str>>(content: S, file: &mut File) {
    file.write_all(content.as_ref().as_bytes())
        .expect("Failed to write to the output file");
}

fn parse_and_write(source: String, mut out: File) {
    let mut source = source.as_str();
    loop {
        if let Some((before_lhc_comment, other)) = source.split_once("<!--?") {
            write_all(before_lhc_comment, &mut out);
            let (lhc_comment, after_lhc_comment) = other.split_once("-->")
                .expect("Syntax Error: expected -->, found eof");
            let processed = lhc_process(lhc_comment);
            write_all(&processed, &mut out);
            source = after_lhc_comment;
        } else {
            write_all(source, &mut out);
            break;
        }
    }
}

fn lhc_process(content: &str) -> String {
    if content.is_empty() {
        return String::from("<!--?-->");
    }
    let (key, args) = content.split_once(' ').unwrap_or((content, ""));
    let mut args_itr = args.split(' ');
    match key {
        "include" => {
            loop {
                if let Some(arg) = args_itr.next() {
                    if let Some((key, value)) = arg.split_once('=') {
                        match key {
                            "link" => {
                                todo!("open {} and read it", value);
                            }
                            _ => {
                                eprintln!("[WARN] Unknown property for \"include\": \"{}\"", key);
                            }
                        }
                    } else {
                        eprintln!("[ERROR] Illegal syntax for \"include\": \"{}\"", arg);
                    }
                } else {
                    break;
                }
            }
            todo!("feature: include")
        }
        _ => {
            eprintln!("Unknown lhc comment: {}", key);
            String::from("<!--?-->")
        }
    }
}
