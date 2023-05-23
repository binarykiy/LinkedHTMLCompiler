use crate::config::Config;
use crate::{parse_and_write, read_all};
use crate::util::{extract, SyntaxError};

pub fn run<'a, F: Iterator<Item=(&'a str, &'a str)>>(iter: F, config: &mut Config) {
    for (key, value) in iter {
        match key {
            "link" => {
                let link = value.trim_matches('"');
                let source = read_all(config.relative_path(link))
                    .expect(format!("[ERROR] Failed to read the linked file: {}", value).as_str());
                match extract(source.as_str(), "body") {
                    Ok(body) => {
                        parse_and_write(body.to_string(), config);
                    }
                    Err(e) => {
                        if let SyntaxError::NoTagBegin(_) = e {
                            parse_and_write(source, config);
                        } else {
                            eprintln!("[FATAL] {}", e);
                            panic!()
                        }
                    }
                }
            }
            _ => {
                eprintln!("[WARN] Unknown property for \"include\": \"{}={}\"", key, value);
            }
        }
    }
}
