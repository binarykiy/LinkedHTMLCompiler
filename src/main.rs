use std::fs::OpenOptions;
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
    let mut input_file = OpenOptions::new().read(true).open(input_path)
        .expect("Failed to open the file");
    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("Failed to read the file");
    let output_path = input_path.parent().unwrap().join("out.html");
    let mut output_file = OpenOptions::new().create(true).truncate(true).write(true).open(output_path)
        .expect("Failed to create the output file");
    let mut source = input.as_str();
    loop {
        if let Some((before_lhc_comment, other)) = source.split_once("<!--?") {
            output_file.write_all(before_lhc_comment.as_bytes())
                .expect("Failed to write to the output file");
            let (lhc_comment, after_lhc_comment) = other.split_once("-->")
                .expect("Syntax Error: expected -->, found eof");
            let processed = lhc_process(lhc_comment);
            output_file.write_all(processed.as_bytes())
                .expect("Failed to write to the output file");
            source = after_lhc_comment;
        } else {
            output_file.write_all(source.as_bytes())
                .expect("Failed to write to the output file");
            break;
        }
    }
}

fn lhc_process(content: &str) -> String {
    if content.starts_with("include") {
        todo!("feature: include")
    } else {
        eprintln!("Unknown lhc comment: {}", content);
        String::new()
    }
}
